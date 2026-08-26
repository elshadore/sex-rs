use std::io::BufRead;

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
    pub pos: Position,
    pub file: Option<String>,
    buf: [Option<char>; 2],
}

impl<R: BufRead> Parser<R> {
    pub fn new(reader: R) -> Self {
        let mut result = Self {
            reader,
            pos: Position::start(),
            file: None,
            buf: [None, None],
        };
        result.buf[0] = read_char(&mut result.reader);
        result.buf[1] = read_char(&mut result.reader);
        result
    }

    pub fn is_finished(&self) -> bool {
        self.buf[0].is_none()
    }

    pub fn at(&self) -> Option<char> {
        self.buf[0]
    }

    pub fn peek(&self) -> Option<char> {
        self.buf[1]
    }

    pub fn inc(&mut self) -> Option<char> {
        let ch = self.buf[0].take();
        self.buf[0] = self.buf[1].take();
        self.buf[1] = read_char(&mut self.reader);
        if let Some(c) = ch {
            self.pos.inc_mut(c);
        }
        ch
    }

    pub fn try_inc(&mut self, expected: char) -> Result<(), SexParserError> {
        match self.inc() {
            Some(c) if c == expected => Ok(()),
            Some(found) => Err(SexParserError {
                pos: self.pos,
                file: self.file.clone(),
                kind: SexParserErrorKind::ExpectedChar { expected, found },
            }),
            None => Err(SexParserError {
                pos: self.pos,
                file: self.file.clone(),
                kind: SexParserErrorKind::UnexpectedEof,
            }),
        }
    }

    pub fn error(&self, pos: Position, kind: SexParserErrorKind) -> SexParserError {
        SexParserError {
            pos,
            file: self.file.clone(),
            kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MalformedHexCode {
    InvalidLeft { left: char },
    InvalidRight { left: char, right: char },
    MissingLeft,
    MissingRight { left: char },
}

impl MalformedHexCode {
    pub fn invalid_left(left: char) -> Self {
        Self::InvalidLeft { left }
    }

    pub fn invalid_right(left: char, right: char) -> Self {
        Self::InvalidRight { left, right }
    }

    pub fn missing_left() -> Self {
        Self::MissingLeft
    }

    pub fn missing_right(left: char) -> Self {
        Self::MissingRight { left }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexError {
    Invalid(char),
    NoChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BarredTy {
    Symbol,
    Keyword,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SexParserErrorKind {
    UnexpectedEof,
    UnexpectedChar(char),
    ExpectedChar { expected: char, found: char },
    UnterminatedList,
    UnterminatedString,
    UnterminatedBarSymbol,
    MalformedStringEscape(char),
    MalformedBarEscape(char),
    MalformedHexEscape(MalformedHexCode),
    MalformedUnicodeEscape(char),
    InvalidUnicodeChar(u32),
    InvalidNumber,
    EmptyKeyword,
    ExpectedWhitespace(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SexParserError {
    pub pos: Position,
    pub file: Option<String>,
    pub kind: SexParserErrorKind,
}

impl std::fmt::Display for SexParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let SexParserError { pos, file, kind } = self;
        if let Some(file) = file {
            write!(f, "{}:", file)?;
        }
        match kind {
            SexParserErrorKind::UnexpectedEof => write!(f, "{}: unexpected EOF", pos),
            SexParserErrorKind::UnexpectedChar(ch) => {
                write!(f, "{}: unexpected character '{}'", pos, ch)
            }
            SexParserErrorKind::ExpectedChar { expected, found } => {
                write!(
                    f,
                    "{}: expected the character: '{}', found: '{}'",
                    pos, expected, found
                )
            }
            SexParserErrorKind::UnterminatedList => {
                write!(f, "{}: unterminated list, expected ')'", pos)
            }
            SexParserErrorKind::UnterminatedString => write!(f, "{}: unterminated string", pos),
            SexParserErrorKind::UnterminatedBarSymbol => {
                write!(f, "{}: unterminated barred symbol, expected '|'", pos)
            }
            SexParserErrorKind::MalformedStringEscape(ch) => {
                write!(f, "{}: malformed string escape sequence '\\{}'", pos, ch)
            }
            SexParserErrorKind::MalformedBarEscape(ch) => {
                write!(
                    f,
                    "{}: malformed barred symbol escape sequence '\\{}'",
                    pos, ch
                )
            }
            SexParserErrorKind::MalformedHexEscape(value) => match value {
                MalformedHexCode::InvalidLeft { left } => {
                    write!(
                        f,
                        "{}: malformed hex escape sequence in the first position: '\\x{}?'",
                        pos, left
                    )
                }
                MalformedHexCode::InvalidRight { left, right } => {
                    write!(
                        f,
                        "{}: malformed hex escape sequence in the second position: '\\x{}{}'",
                        pos, left, right
                    )
                }
                MalformedHexCode::MissingLeft => {
                    write!(
                        f,
                        "{}: malformed hex escape sequence, string ended before any hexcodes could be read!",
                        pos
                    )
                }
                MalformedHexCode::MissingRight { left } => {
                    write!(
                        f,
                        "{}: malformed hex escape sequence, string ended before second hexcode could be read: '\\x{}_'",
                        pos, left
                    )
                }
            },
            SexParserErrorKind::MalformedUnicodeEscape(value) => {
                write!(f, "{}: malformed unicode escape sequence: {}", pos, value)
            }
            SexParserErrorKind::InvalidNumber => {
                write!(f, "{}: invalid number", pos)
            }
            SexParserErrorKind::EmptyKeyword => write!(f, "{}: empty keyword", pos),
            SexParserErrorKind::ExpectedWhitespace(ch) => {
                write!(f, "{}: expected whitespace before '{}'", pos, ch)
            }
            SexParserErrorKind::InvalidUnicodeChar(value) => {
                write!(f, "{}, invalid unicode character '\\u{{{:x}}}'", pos, value)
            }
        }
    }
}

impl std::error::Error for SexParserError {}

#[derive(Debug)]
pub enum SexParserAtomError {
    Generic(SexParserError),
    ExpectedSingleAtom { pos: Position, file: Option<String> },
}

impl SexParserAtomError {
    pub fn new_expected_single_atom(pos: Position, file: Option<String>) -> Self {
        SexParserAtomError::ExpectedSingleAtom { pos, file }
    }
}

impl std::fmt::Display for SexParserAtomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SexParserAtomError::Generic(generic) => generic.fmt(f),
            SexParserAtomError::ExpectedSingleAtom { pos, file } => {
                if let Some(file) = file {
                    write!(f, "{}:{}: expected single atom", file, pos)
                } else {
                    write!(f, "{}: expected single atom", pos)
                }
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
