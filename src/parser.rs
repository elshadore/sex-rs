use crate::atom::{Atom, List, Number, Text, TextTy};
use std::io::{BufRead, BufReader, Read};

fn read_char(reader: &mut impl BufRead) -> Option<char> {
    loop {
        let buf = reader.fill_buf().ok()?;
        if buf.is_empty() {
            return None;
        }
        match std::str::from_utf8(buf) {
            Ok(s) => {
                let ch = s.chars().next().unwrap();
                let n = ch.len_utf8();
                reader.consume(n);
                return Some(ch);
            }
            Err(e) => {
                let valid = e.valid_up_to();
                if valid > 0 {
                    let ch = unsafe { std::str::from_utf8_unchecked(&buf[..valid]) }
                        .chars()
                        .next()
                        .unwrap();
                    let n = ch.len_utf8();
                    reader.consume(n);
                    return Some(ch);
                }
                reader.consume(1);
            }
        }
    }
}

#[derive(Debug)]
pub enum SexParserError {
    UnexpectedEof { pos: Position },
    UnexpectedChar { pos: Position, ch: char },
    UnterminatedList { pos: Position },
    UnterminatedString { pos: Position },
    InvalidEscape { pos: Position, ch: char },
    InvalidNumber { pos: Position, value: String },
    EmptyKeyword { pos: Position },
}

impl std::fmt::Display for SexParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SexParserError::UnexpectedEof { pos } => write!(f, "{}: unexpected EOF", pos),
            SexParserError::UnexpectedChar { pos, ch } => {
                write!(f, "{}: unexpected character '{}'", pos, ch)
            }
            SexParserError::UnterminatedList { pos } => {
                write!(f, "{}: unterminated list, expected ')'", pos)
            }
            SexParserError::UnterminatedString { pos } => write!(f, "{}: unterminated string", pos),
            SexParserError::InvalidEscape { pos, ch } => {
                write!(f, "{}: invalid escape sequence '\\{}'", pos, ch)
            }
            SexParserError::InvalidNumber { pos, value } => {
                write!(f, "{}: invalid number '{}'", pos, value)
            }
            SexParserError::EmptyKeyword { pos } => write!(f, "{}: empty keyword", pos),
        }
    }
}

impl std::error::Error for SexParserError {}

#[derive(Debug)]
pub enum SexParserAtomError {
    Generic(SexParserError),
    ExpectedSingleAtom { pos: Position },
}

impl std::fmt::Display for SexParserAtomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SexParserAtomError::Generic(generic) => generic.fmt(f),
            SexParserAtomError::ExpectedSingleAtom { pos } => {
                write!(f, "{}: expected single atom", pos)
            }
        }
    }
}

impl std::error::Error for SexParserAtomError {}

impl From<SexParserError> for SexParserAtomError {
    fn from(value: SexParserError) -> Self {
        Self::Generic(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub const fn new(line: usize, col: usize) -> Position {
        Position { line, col }
    }

    pub const fn start() -> Position {
        Position::new(1, 1)
    }

    pub const fn inc(self, c: char) -> Position {
        if c == '\n' {
            Position::new(self.line + 1, 1)
        } else {
            Position::new(self.line, self.col + 1)
        }
    }

    pub fn inc_mut(&mut self, c: char) {
        *self = self.inc(c)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

pub struct Parser<R: BufRead> {
    reader: R,
    pos: Position,
    buf: [Option<char>; 2],
}

impl<R: BufRead> Parser<R> {
    pub fn new(reader: R) -> Self {
        let mut result = Self {
            reader,
            pos: Position::start(),
            buf: [None, None],
        };
        result.buf[0] = read_char(&mut result.reader);
        result.buf[1] = read_char(&mut result.reader);
        result
    }

    fn is_finished(&self) -> bool {
        self.buf[0].is_none()
    }

    fn at(&self) -> Option<char> {
        self.buf[0]
    }

    fn peek(&self) -> Option<char> {
        self.buf[1]
    }

    fn inc(&mut self) -> Option<char> {
        let ch = self.buf[0].take();
        self.buf[0] = self.buf[1].take();
        self.buf[1] = read_char(&mut self.reader);
        if let Some(c) = ch {
            self.pos.inc_mut(c);
        }
        ch
    }

    fn inc_expect(&mut self, expected: char) -> Result<(), SexParserError> {
        match self.inc() {
            Some(ch) if ch == expected => Ok(()),
            Some(ch) => Err(self.unexpected_char(ch)),
            None => Err(self.unexpected_eof()),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.at() {
                None => return,
                Some(ch) if ch.is_whitespace() => {
                    self.inc();
                }
                _ => return,
            }
        }
    }

    fn exec_listed(&mut self) -> Result<List, SexParserError> {
        let mut atoms = Vec::new();
        loop {
            self.skip_whitespace();
            if self.is_finished() {
                break;
            }
            atoms.push(self.parse_atom()?);
        }
        Ok(atoms)
    }

    fn exec_atom(&mut self) -> Result<Atom, SexParserAtomError> {
        self.skip_whitespace();
        let result = self.parse_atom()?;
        self.skip_whitespace();
        if !self.is_finished() {
            return Err(self.expected_single_atom());
        }
        Ok(result)
    }

    fn parse_atom(&mut self) -> Result<Atom, SexParserError> {
        match self.at() {
            None => Err(self.unexpected_eof()),
            Some('(') => self.parse_list(),
            Some('"') => self.parse_string(),
            Some(':') => self.parse_keyword(),
            Some(ch) if ch == '-' && self.peek().map_or(false, |c| c.is_ascii_digit()) => {
                self.parse_number()
            }
            Some(ch) if ch.is_ascii_digit() => self.parse_number(),
            Some(ch) if is_symbol_char(ch) => self.parse_symbol(),
            Some(ch) => Err(self.unexpected_char(ch)),
        }
    }

    fn parse_list(&mut self) -> Result<Atom, SexParserError> {
        self.inc_expect('(')?;
        let mut list: List = Vec::new();
        loop {
            self.skip_whitespace();
            match self.at() {
                None => return Err(self.unterminated_list()),
                Some(')') => {
                    self.inc();
                    break;
                }
                _ => {
                    list.push(self.parse_atom()?);
                }
            }
        }
        Ok(Atom::List(list))
    }

    fn parse_string(&mut self) -> Result<Atom, SexParserError> {
        self.inc_expect('"')?;
        let mut s = String::new();
        loop {
            match self.inc() {
                None => return Err(self.unterminated_string()),
                Some('"') => break,
                Some('\\') => match self.inc() {
                    None => return Err(self.unterminated_string()),
                    Some('"') => s.push('"'),
                    Some('\\') => s.push('\\'),
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('r') => s.push('\r'),
                    Some(ch) => {
                        return Err(self.invalid_escape(ch));
                    }
                },
                Some(ch) => s.push(ch),
            }
        }
        Ok(Atom::Text(Text {
            ty: TextTy::String,
            contents: s,
        }))
    }

    fn parse_keyword(&mut self) -> Result<Atom, SexParserError> {
        self.inc_expect(':')?;
        let mut name = String::new();
        while let Some(ch) = self.at() {
            if !is_symbol_char(ch) {
                break;
            }
            name.push(ch);
            self.inc();
        }
        if name.is_empty() {
            return Err(self.empty_keyword());
        }
        Ok(Atom::Text(Text {
            ty: TextTy::Keyword,
            contents: name,
        }))
    }

    fn parse_symbol(&mut self) -> Result<Atom, SexParserError> {
        let mut name = String::new();
        while let Some(ch) = self.at() {
            if !is_symbol_char(ch) {
                break;
            }
            name.push(ch);
            self.inc();
        }
        if name == "nil" {
            return Ok(Atom::Nil);
        }
        Ok(Atom::Text(Text {
            ty: TextTy::Symbol,
            contents: name,
        }))
    }

    fn parse_number(&mut self) -> Result<Atom, SexParserError> {
        let start = self.pos;

        let mut buf = String::new();
        if self.at() == Some('-') {
            buf.push('-');
            self.inc();
        }

        let mut has_dot = false;
        while let Some(ch) = self.at() {
            if ch.is_ascii_digit() {
                buf.push(ch);
                self.inc();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                buf.push(ch);
                self.inc();
            } else {
                break;
            }
        }

        if has_dot {
            let n: f64 = buf.parse().map_err(|_| SexParserError::InvalidNumber {
                pos: start,
                value: buf.clone(),
            })?;
            Ok(Atom::Number(Number::Float(n)))
        } else {
            let n: i64 = buf.parse().map_err(|_| SexParserError::InvalidNumber {
                pos: start,
                value: buf.clone(),
            })?;
            Ok(Atom::Number(Number::Integer(n)))
        }
    }

    fn unexpected_eof(&self) -> SexParserError {
        SexParserError::UnexpectedEof { pos: self.pos }
    }

    fn unexpected_char(&self, ch: char) -> SexParserError {
        SexParserError::UnexpectedChar { pos: self.pos, ch }
    }

    fn unterminated_list(&self) -> SexParserError {
        SexParserError::UnterminatedList { pos: self.pos }
    }

    fn unterminated_string(&self) -> SexParserError {
        SexParserError::UnterminatedString { pos: self.pos }
    }

    fn invalid_escape(&self, ch: char) -> SexParserError {
        SexParserError::InvalidEscape { pos: self.pos, ch }
    }

    fn empty_keyword(&self) -> SexParserError {
        SexParserError::EmptyKeyword { pos: self.pos }
    }

    fn expected_single_atom(&self) -> SexParserAtomError {
        SexParserAtomError::ExpectedSingleAtom { pos: self.pos }
    }
}

fn is_symbol_char(ch: char) -> bool {
    ch.is_alphanumeric()
        || ch == '-'
        || ch == '_'
        || ch == '.'
        || ch == '/'
        || ch == '*'
        || ch == '+'
        || ch == '!'
        || ch == '?'
        || ch == '<'
        || ch == '>'
        || ch == '='
        || ch == '&'
        || ch == '%'
}

/// Parses multiple atoms/expressions from a string.
/// foo             => (foo)
/// (foo bar baz)   => ((foo bar bar))
/// (foo bar) (baz) => ((foo bar) (baz))
///                 => ()
pub fn parse_listed(input: impl AsRef<str>) -> Result<List, SexParserError> {
    let s = input.as_ref();
    let cursor = std::io::Cursor::new(s.as_bytes());
    let mut parser = Parser::new(cursor);
    parser.exec_listed()
}

/// Parses multiple atoms/expressions from a generic reader.
/// foo             => (foo)
/// (foo bar baz)   => ((foo bar bar))
/// (foo bar) (baz) => ((foo bar) (baz))
///                 => ()
pub fn parse_listed_reader(reader: impl Read) -> Result<List, SexParserError> {
    let reader = BufReader::new(reader);
    let mut parser = Parser::new(reader);
    parser.exec_listed()
}

/// Parses a single atom/expression from a string.
/// foo             => foo
/// (foo bar baz)   => (foo bar bar)
/// (foo bar) (baz) => X
///                 => X
pub fn parse_atom(input: impl AsRef<str>) -> Result<Atom, SexParserAtomError> {
    let s = input.as_ref();
    let cursor = std::io::Cursor::new(s.as_bytes());
    let mut parser = Parser::new(cursor);
    parser.exec_atom()
}

/// Parses a single atom/expression from a generic reader.
/// foo             => foo
/// (foo bar baz)   => (foo bar bar)
/// (foo bar) (baz) => X
///                 => X
pub fn parse_atom_reader(reader: impl Read) -> Result<Atom, SexParserAtomError> {
    let reader = BufReader::new(reader);
    let mut parser = Parser::new(reader);
    parser.exec_atom()
}
