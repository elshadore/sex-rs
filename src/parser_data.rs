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
    MalformedStringEscape {
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
        value: String,
    },
    EmptyKeyword {
        pos: Position,
    },
    ExpectedWhitespace {
        pos: Position,
        ch: char,
    },
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
            SexParserError::MalformedStringEscape { pos, ch } => {
                write!(f, "{}: malformed string escape sequence '\\{}'", pos, ch)
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
            SexParserError::InvalidNumber { pos, value } => {
                write!(f, "{}: invalid number '{}'", pos, value)
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
