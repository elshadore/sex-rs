use std::io::{BufRead, BufReader, Read};

use crate::atom::{Atom, List, Number, SexError, Text, TextTy};

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

pub struct Parser<R: BufRead> {
    reader: R,
    line: usize,
    col: usize,
    buf: [Option<char>; 2],
}

impl<R: BufRead> Parser<R> {
    pub fn new(reader: R) -> Self {
        let mut p = Parser {
            reader,
            line: 1,
            col: 1,
            buf: [None, None],
        };
        p.refill();
        p.refill();
        p
    }

    fn refill(&mut self) {
        self.buf[0] = self.buf[1].take();
        self.buf[1] = read_char(&mut self.reader);
    }

    fn peek(&self) -> Option<char> {
        self.buf[0]
    }

    fn peek_next(&self) -> Option<char> {
        self.buf[1]
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.buf[0].take();
        self.refill();
        if let Some(c) = ch {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        ch
    }

    fn is_eof(&self) -> bool {
        self.buf[0].is_none()
    }

    fn pos(&self) -> crate::atom::Position {
        crate::atom::Position {
            line: self.line,
            col: self.col,
        }
    }

    fn expect_char(&mut self, expected: char) -> Result<(), SexError> {
        match self.advance() {
            Some(ch) if ch == expected => Ok(()),
            Some(ch) => Err(self.unexpected_char(ch)),
            None => Err(self.unexpected_eof()),
        }
    }

    fn unexpected_eof(&self) -> SexError {
        SexError::UnexpectedEof { pos: self.pos() }
    }

    fn unexpected_char(&self, ch: char) -> SexError {
        SexError::UnexpectedChar {
            pos: self.pos(),
            ch,
        }
    }

    fn unterminated_list(&self) -> SexError {
        SexError::UnterminatedList { pos: self.pos() }
    }

    fn unterminated_string(&self) -> SexError {
        SexError::UnterminatedString { pos: self.pos() }
    }

    fn invalid_escape(&self, ch: char) -> SexError {
        SexError::InvalidEscape {
            pos: self.pos(),
            ch,
        }
    }

    fn empty_keyword(&self) -> SexError {
        SexError::EmptyKeyword { pos: self.pos() }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                None => return,
                Some(ch) if ch.is_whitespace() => {
                    self.advance();
                }
                Some(';') => {
                    while let Some(ch) = self.peek() {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn parse_all(&mut self) -> Result<Vec<Atom>, SexError> {
        let mut atoms = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            if self.is_eof() {
                break;
            }
            atoms.push(self.parse_atom()?);
        }
        Ok(atoms)
    }

    fn parse_atom(&mut self) -> Result<Atom, SexError> {
        self.skip_whitespace_and_comments();
        match self.peek() {
            None => Err(self.unexpected_eof()),
            Some('(') => self.parse_list(),
            Some('"') => self.parse_string(),
            Some(':') => self.parse_keyword(),
            Some(ch) if ch == '-' && self.peek_next().map_or(false, |c| c.is_ascii_digit()) => {
                self.parse_number()
            }
            Some(ch) if ch.is_ascii_digit() => self.parse_number(),
            Some(ch) if is_symbol_char(ch) => self.parse_symbol(),
            Some(ch) => Err(self.unexpected_char(ch)),
        }
    }

    fn parse_list(&mut self) -> Result<Atom, SexError> {
        self.expect_char('(')?;
        let mut list: List = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            match self.peek() {
                None => return Err(self.unterminated_list()),
                Some(')') => {
                    self.advance();
                    break;
                }
                _ => {
                    list.push(self.parse_atom()?);
                }
            }
        }
        Ok(Atom::List(list))
    }

    fn parse_string(&mut self) -> Result<Atom, SexError> {
        self.expect_char('"')?;
        let mut s = String::new();
        loop {
            match self.advance() {
                None => return Err(self.unterminated_string()),
                Some('"') => break,
                Some('\\') => match self.advance() {
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
        Ok(Atom::Text(Text { ty: TextTy::String, contents: s }))
    }

    fn parse_keyword(&mut self) -> Result<Atom, SexError> {
        self.expect_char(':')?;
        let mut name = String::new();
        while let Some(ch) = self.peek() {
            if !is_symbol_char(ch) {
                break;
            }
            name.push(ch);
            self.advance();
        }
        if name.is_empty() {
            return Err(self.empty_keyword());
        }
        Ok(Atom::Text(Text { ty: TextTy::Keyword, contents: name }))
    }

    fn parse_symbol(&mut self) -> Result<Atom, SexError> {
        let mut name = String::new();
        while let Some(ch) = self.peek() {
            if !is_symbol_char(ch) {
                break;
            }
            name.push(ch);
            self.advance();
        }
        if name == "nil" {
            return Ok(Atom::Nil);
        }
        if name == "true" || name == "t" {
            return Ok(Atom::True);
        }
        Ok(Atom::Text(Text { ty: TextTy::Symbol, contents: name }))
    }

    fn parse_number(&mut self) -> Result<Atom, SexError> {
        let start_pos = self.pos();
        let mut buf = String::new();
        if self.peek() == Some('-') {
            buf.push('-');
            self.advance();
        }
        let mut has_dot = false;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                buf.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                buf.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if has_dot {
            let n: f32 = buf.parse().map_err(|_| SexError::InvalidNumber {
                pos: start_pos,
                value: buf.clone(),
            })?;
            Ok(Atom::Number(Number::Float(n)))
        } else {
            let n: i32 = buf.parse().map_err(|_| SexError::InvalidNumber {
                pos: start_pos,
                value: buf.clone(),
            })?;
            Ok(Atom::Number(Number::Integer(n)))
        }
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

pub fn parse(input: impl AsRef<str>) -> Result<Vec<Atom>, SexError> {
    let s = input.as_ref();
    let cursor = std::io::Cursor::new(s.as_bytes());
    let mut parser = Parser::new(cursor);
    parser.parse_all()
}

pub fn parse_reader(reader: impl Read) -> Result<Vec<Atom>, SexError> {
    let reader = BufReader::new(reader);
    let mut parser = Parser::new(reader);
    parser.parse_all()
}
