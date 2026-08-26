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
            Some(ch) if ch == expected => Ok(()),
            Some(ch) => Err(SexParserError::new_unexpected_char(self.pos, ch)),
            None => Err(SexParserError::new_unexpected_eof(self.pos)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum HexError {
    Invalid(char),
    NoChar,
}

#[derive(Debug, Clone, Copy)]
pub enum BarredTy {
    Symbol,
    Keyword,
}

#[derive(Debug)]
pub enum SexParserError {
    UnexpectedEof {
        pos: Position,
    },
    UnexpectedChar {
        pos: Position,
        ch: char,
    },
    UnterminatedList {
        pos: Position,
    },
    UnterminatedString {
        pos: Position,
    },
    UnterminatedBarSymbol {
        pos: Position,
    },
    MalformedStringEscape {
        pos: Position,
        ch: char,
    },
    MalformedBarEscape {
        pos: Position,
        ch: char,
    },
    MalformedHexEscape {
        pos: Position,
        value: MalformedHexCode,
    },
    MalformedUnicodeEscape {
        pos: Position,
        value: char,
    },
    InvalidUnicodeChar {
        pos: Position,
        value: u32,
    },
    InvalidNumber {
        pos: Position,
    },
    EmptyKeyword {
        pos: Position,
    },
    ExpectedWhitespace {
        pos: Position,
        ch: char,
    },
}

impl SexParserError {
    pub fn new_unexpected_eof(pos: Position) -> Self {
        SexParserError::UnexpectedEof { pos }
    }

    pub fn new_unexpected_char(pos: Position, ch: char) -> Self {
        SexParserError::UnexpectedChar { pos, ch }
    }

    pub fn new_unterminated_list(pos: Position) -> Self {
        SexParserError::UnterminatedList { pos }
    }

    pub fn new_unterminated_string(pos: Position) -> Self {
        SexParserError::UnterminatedString { pos }
    }

    pub fn new_unterminated_bar_symbol(pos: Position) -> Self {
        SexParserError::UnterminatedBarSymbol { pos }
    }

    pub fn new_malformed_string_escape(pos: Position, ch: char) -> Self {
        SexParserError::MalformedStringEscape { pos, ch }
    }

    pub fn new_malformed_bar_escape(pos: Position, ch: char) -> Self {
        SexParserError::MalformedBarEscape { pos, ch }
    }

    pub fn new_malformed_hex_escape(pos: Position, value: MalformedHexCode) -> Self {
        SexParserError::MalformedHexEscape { pos, value }
    }

    pub fn new_malformed_unicode_escape(pos: Position, value: char) -> Self {
        SexParserError::MalformedUnicodeEscape { pos, value }
    }

    pub fn new_invalid_unicode_char(pos: Position, value: u32) -> Self {
        SexParserError::InvalidUnicodeChar { pos, value }
    }

    pub fn new_invalid_number(pos: Position) -> Self {
        SexParserError::InvalidNumber { pos }
    }

    pub fn new_empty_keyword(pos: Position) -> Self {
        SexParserError::EmptyKeyword { pos }
    }

    pub fn new_expected_whitespace(pos: Position, ch: char) -> Self {
        SexParserError::ExpectedWhitespace { pos, ch }
    }
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
            SexParserError::UnterminatedBarSymbol { pos } => {
                write!(f, "{}: unterminated barred symbol, expected '|'", pos)
            }
            SexParserError::MalformedStringEscape { pos, ch } => {
                write!(f, "{}: malformed string escape sequence '\\{}'", pos, ch)
            }
            SexParserError::MalformedBarEscape { pos, ch } => {
                write!(f, "{}: malformed barred symbol escape sequence '\\{}'", pos, ch)
            }
            SexParserError::MalformedHexEscape { pos, value } => match value {
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
            SexParserError::MalformedUnicodeEscape { pos, value } => {
                write!(f, "{}: malformed unicode escape sequence: {}", pos, value)
            }
            SexParserError::InvalidNumber { pos } => {
                write!(f, "{}: invalid number", pos)
            }
            SexParserError::EmptyKeyword { pos } => write!(f, "{}: empty keyword", pos),
            SexParserError::ExpectedWhitespace { pos, ch } => {
                write!(f, "{}: expected whitespace before '{}'", pos, ch)
            }
            SexParserError::InvalidUnicodeChar { pos, value } => {
                write!(f, "{}, invalid unicode character '\\u{{{:x}}}'", pos, value)
            }
        }
    }
}

impl std::error::Error for SexParserError {}

#[derive(Debug)]
pub enum SexParserAtomError {
    Generic(SexParserError),
    ExpectedSingleAtom {
        pos: Position,
    },
}

impl SexParserAtomError {
    pub fn new_expected_single_atom(pos: Position) -> Self {
        SexParserAtomError::ExpectedSingleAtom { pos }
    }
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
