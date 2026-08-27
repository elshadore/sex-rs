use std::io::BufRead;

#[macro_export]
macro_rules! err {
    ($p:expr, $kind:ident) => {
        Err($p.error($p.pos, SexParserErrorKind::$kind))
    };
    ($p:expr, $kind:ident($($arg:tt)*)) => {
        Err($p.error($p.pos, SexParserErrorKind::$kind($($arg)*)))
    };
    ($p:expr, $kind:ident { $($arg:tt)* }) => {
        Err($p.error($p.pos, SexParserErrorKind::$kind { $($arg)* }))
    };
}

#[macro_export]
macro_rules! err_unicode {
    ($p:expr, $kind:ident) => {
        Err($p.error($p.pos, SexParserErrorKind::MalformedUnicodeEscape(MalformedUnicodeEscape::$kind)))
    };
    ($p:expr, $kind:ident($($arg:tt)*)) => {
        Err($p.error($p.pos, SexParserErrorKind::MalformedUnicodeEscape(MalformedUnicodeEscape::$kind($($arg)*))))
    };
}

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
    pub fn new(reader: R, file: Option<String>) -> Self {
        let mut result = Self {
            reader,
            pos: Position::start(),
            file,
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
            Some(found) => err!(self, ExpectedChar { expected, found }),
            None => err!(self, UnexpectedEof),
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
pub enum MalformedUnicodeEscape {
    EscapeOpeningBraceExpected(Option<char>),
    HexMaximumOfSixReached,
    ExpectedHexChar(char),
    EscapeUnterminated,
    EmptyEscape,
    InvalidUnicodeCodepoint(u32),
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
    MalformedUnicodeEscape(MalformedUnicodeEscape),
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
        write!(f, "{}: ", pos)?;
        match kind {
            SexParserErrorKind::UnexpectedEof => write!(f, "unexpected EOF"),
            SexParserErrorKind::UnexpectedChar(c) => {
                write!(f, "unexpected character '{}'", c)
            }
            SexParserErrorKind::ExpectedChar { expected, found } => {
                write!(
                    f,
                    "expected the character: '{}', found: '{}'",
                    expected, found
                )
            }
            SexParserErrorKind::UnterminatedList => {
                write!(f, "unterminated list, expected ')'")
            }
            SexParserErrorKind::UnterminatedString => write!(f, "unterminated string"),
            SexParserErrorKind::UnterminatedBarSymbol => {
                write!(f, "unterminated barred symbol, expected '|'")
            }
            SexParserErrorKind::MalformedStringEscape(c) => {
                write!(f, "malformed string escape sequence '\\{}'", c)
            }
            SexParserErrorKind::MalformedBarEscape(c) => {
                write!(f, "malformed barred symbol escape sequence '\\{}'", c)
            }
            SexParserErrorKind::MalformedHexEscape(value) => match value {
                MalformedHexCode::InvalidLeft { left } => {
                    write!(
                        f,
                        "malformed hex escape sequence in the first position: '\\x{}?'",
                        left
                    )
                }
                MalformedHexCode::InvalidRight { left, right } => {
                    write!(
                        f,
                        "malformed hex escape sequence in the second position: '\\x{}{}'",
                        left, right
                    )
                }
                MalformedHexCode::MissingLeft => {
                    write!(
                        f,
                        "malformed hex escape sequence, string ended before any hexcodes could be read!"
                    )
                }
                MalformedHexCode::MissingRight { left } => {
                    write!(
                        f,
                        "malformed hex escape sequence, string ended before second hexcode could be read: '\\x{}_'",
                        left
                    )
                }
            },
            SexParserErrorKind::InvalidNumber => {
                write!(f, "invalid number")
            }
            SexParserErrorKind::EmptyKeyword => write!(f, "empty keyword"),
            SexParserErrorKind::ExpectedWhitespace(c) => {
                write!(f, "expected whitespace before '{}'", c)
            }
            SexParserErrorKind::MalformedUnicodeEscape(err) => match err {
                MalformedUnicodeEscape::EscapeOpeningBraceExpected(c) => {
                    if let Some(c) = c {
                        write!(f, "expected opening brace found: '{c}'")
                    } else {
                        write!(f, "expected opening brace found: EOF")
                    }
                }
                MalformedUnicodeEscape::EmptyEscape => {
                    write!(f, "empty unicode escape sequence")
                }
                MalformedUnicodeEscape::EscapeUnterminated => {
                    write!(f, "unicode escape sequence unterminated missing '}}'")
                }
                MalformedUnicodeEscape::ExpectedHexChar(c) => {
                    write!(f, "expected hex char, found: '{c}'")
                }
                MalformedUnicodeEscape::HexMaximumOfSixReached => {
                    write!(f, "a maximum of 6 unicode hex values has been reached")
                }
                MalformedUnicodeEscape::InvalidUnicodeCodepoint(code) => {
                    write!(f, "invalid unicode codepoint: '{code}'")
                }
            },
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
