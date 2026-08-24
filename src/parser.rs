use crate::{
    MalformedHexCode, Position, SexParserAtomError, SexParserError,
    atom::{Atom, List, Number, Text, TextTy},
    parser_data::{BarredTy, HexError},
};
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
            Some(ch) => Err(SexParserError::new_unexpected_char(self.pos, ch)),
            None => Err(SexParserError::new_unexpected_eof(self.pos)),
        }
    }

    fn skip_whitespace(&mut self) -> bool {
        let mut result: bool = false;
        loop {
            match self.at() {
                None => return result,
                Some(ch) if ch.is_whitespace() => {
                    result = true;
                    self.inc();
                }
                Some(';') => {
                    result = true;
                    while let Some(ch) = self.at() {
                        self.inc();
                        if ch == '\n' {
                            break;
                        }
                    }
                }
                _ => return result,
            }
        }
    }

    fn exec_listed(&mut self) -> Result<List, SexParserError> {
        let mut atoms = Vec::new();
        let mut whitespace: bool = true;

        self.skip_whitespace();

        loop {
            if self.is_finished() {
                break;
            }
            if !whitespace {
                return Err(SexParserError::new_expected_whitespace(
                    self.pos,
                    self.at().unwrap_or('\0'),
                ));
            }
            atoms.push(self.parse_atom()?);
            whitespace = self.skip_whitespace();
        }

        Ok(atoms)
    }

    fn exec_atom(&mut self) -> Result<Atom, SexParserAtomError> {
        self.skip_whitespace();
        let result = self.parse_atom()?;
        self.skip_whitespace();
        if !self.is_finished() {
            return Err(SexParserAtomError::new_expected_single_atom(self.pos));
        }
        Ok(result)
    }

    fn parse_atom(&mut self) -> Result<Atom, SexParserError> {
        match self.at() {
            None => Err(SexParserError::new_unexpected_eof(self.pos)),
            Some('(') => self.parse_list(),
            Some('"') => self.parse_string(),
            Some(':') => self.parse_keyword(),
            Some('|') => self.parse_barred(BarredTy::Symbol),
            Some(ch) if ch == '-' && self.peek().is_some_and(|c| c.is_ascii_digit()) => {
                self.parse_number()
            }
            Some(ch) if ch.is_ascii_digit() => self.parse_number(),
            Some(ch) if is_symbol_char(ch) => self.parse_symbol(),
            Some(ch) => Err(SexParserError::new_unexpected_char(self.pos, ch)),
        }
    }

    fn parse_list(&mut self) -> Result<Atom, SexParserError> {
        self.inc_expect('(')?;

        let mut list: List = Vec::new();
        let mut first: bool = true;
        let mut whitespace = self.skip_whitespace();

        loop {
            match self.at() {
                None => return Err(SexParserError::new_unterminated_list(self.pos)),
                Some(')') => {
                    self.inc();
                    break;
                }
                Some(c) => {
                    if !first && !whitespace {
                        return Err(SexParserError::new_expected_whitespace(self.pos, c));
                    }
                    list.push(self.parse_atom()?);
                }
            }
            whitespace = self.skip_whitespace();
            first = false;
        }
        Ok(Atom::List(list))
    }

    fn parse_string(&mut self) -> Result<Atom, SexParserError> {
        self.inc_expect('"')?;
        let mut s = String::new();
        loop {
            match self.inc() {
                None => return Err(SexParserError::new_unterminated_string(self.pos)),
                Some('"') => {
                    return Ok(Atom::Text(Text {
                        ty: TextTy::String,
                        contents: s,
                    }));
                }
                Some('\\') => {
                    let esc = match self.inc() {
                        None => return Err(SexParserError::new_unterminated_string(self.pos)),
                        Some('"') => '"',
                        Some('\\') => '\\',
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('r') => '\r',
                        Some('0') => '\0',
                        Some('x') => self.read_hex_escape()?,
                        Some('u') => self.read_unicode_escape()?,
                        Some(ch) => {
                            return Err(SexParserError::new_malformed_string_escape(self.pos, ch));
                        }
                    };
                    s.push(esc);
                }
                Some(ch) => s.push(ch),
            }
        }
    }

    fn parse_barred(&mut self, ty: BarredTy) -> Result<Atom, SexParserError> {
        self.inc_expect('|')?;
        let mut name = String::new();
        loop {
            match self.inc() {
                None => return Err(SexParserError::new_unterminated_bar_symbol(self.pos)),
                Some('|') => break,
                Some('\\') => {
                    let esc = match self.inc() {
                        None => return Err(SexParserError::new_unterminated_bar_symbol(self.pos)),
                        Some('"') => '"',
                        Some('|') => '|',
                        Some('\\') => '\\',
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('r') => '\r',
                        Some('0') => '\0',
                        Some('x') => self.read_hex_escape()?,
                        Some('u') => self.read_unicode_escape()?,
                        Some(ch) => {
                            return Err(SexParserError::new_malformed_bar_escape(self.pos, ch));
                        }
                    };
                    name.push(esc);
                }
                Some(ch) => name.push(ch),
            }
        }
        Ok(Atom::Text(Text {
            ty: match ty {
                BarredTy::Symbol => TextTy::Symbol,
                BarredTy::Keyword => TextTy::Keyword,
            },
            contents: name,
        }))
    }

    fn parse_keyword(&mut self) -> Result<Atom, SexParserError> {
        self.inc_expect(':')?;
        if self.at() == Some('|') {
            return self.parse_barred(BarredTy::Keyword);
        }
        let mut name = String::new();
        while let Some(ch) = self.at() {
            if !is_symbol_char(ch) {
                break;
            }
            name.push(ch);
            self.inc();
        }
        if name.is_empty() {
            return Err(SexParserError::new_empty_keyword(self.pos));
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
        match name.as_str() {
            "nil" => Ok(Atom::Nil),
            "true" => Ok(Atom::True),
            "false" => Ok(Atom::False),
            _ => Ok(Atom::Text(Text {
                ty: TextTy::Symbol,
                contents: name,
            })),
        }
    }

    fn read_hex_digit(&mut self) -> Result<u8, HexError> {
        match self.inc() {
            Some(ch) if ch.is_ascii_hexdigit() => Ok(ch.to_digit(16).unwrap_or(0) as u8),
            Some(ch) => Err(HexError::Invalid(ch)),
            None => Err(HexError::NoChar),
        }
    }

    fn read_hex_escape(&mut self) -> Result<char, SexParserError> {
        match self.read_hex_digit() {
            Ok(hi) => match self.read_hex_digit() {
                Ok(lo) => Ok(char::from((hi << 4) | lo)),
                Err(HexError::Invalid(right)) => Err(SexParserError::new_malformed_hex_escape(
                    self.pos,
                    MalformedHexCode::InvalidRight {
                        left: hi.into(),
                        right,
                    },
                )),
                Err(HexError::NoChar) => Err(SexParserError::new_malformed_hex_escape(
                    self.pos,
                    MalformedHexCode::MissingRight { left: hi.into() },
                )),
            },
            Err(HexError::Invalid(left)) => Err(SexParserError::new_malformed_hex_escape(
                self.pos,
                MalformedHexCode::InvalidLeft { left },
            )),
            Err(HexError::NoChar) => Err(SexParserError::new_malformed_hex_escape(
                self.pos,
                MalformedHexCode::MissingLeft,
            )),
        }
    }

    fn read_unicode_escape(&mut self) -> Result<char, SexParserError> {
        let mut value: u32 = 0;
        let mut digits: u32 = 0;
        match self.inc() {
            Some(c) => {
                if c != '{' {
                    return Err(SexParserError::new_malformed_unicode_escape(self.pos, c));
                }
            }
            None => {
                return Err(SexParserError::new_malformed_unicode_escape(self.pos, '\0'));
            }
        }
        loop {
            match self.inc() {
                Some('}') => break,
                Some(ch) if ch.is_ascii_hexdigit() && digits < 6 => {
                    value = value * 16 + ch.to_digit(16).unwrap_or(0);
                    digits += 1;
                }
                Some(ch) if ch.is_ascii_hexdigit() => {
                    return Err(SexParserError::new_malformed_unicode_escape(self.pos, ch));
                }
                Some(ch) => {
                    return Err(SexParserError::new_malformed_unicode_escape(self.pos, ch));
                }
                None => {
                    return Err(SexParserError::new_malformed_unicode_escape(self.pos, '\0'));
                }
            }
        }
        if digits == 0 {
            return Err(SexParserError::new_malformed_unicode_escape(self.pos, '\0'));
        }
        match char::from_u32(value) {
            Some(ch) => Ok(ch),
            None => Err(SexParserError::new_invalid_unicode_char(self.pos, value)),
        }
    }

    fn parse_number(&mut self) -> Result<Atom, SexParserError> {
        let start = self.pos;
        let negative = if self.at() == Some('-') {
            self.inc();
            true
        } else {
            false
        };

        let mut mantissa: u64 = 0;
        match self.at() {
            Some('0') => {
                self.inc();
                if self.at().is_some_and(|ch| ch.is_ascii_digit()) {
                    return Err(SexParserError::new_invalid_number(start));
                }
            }
            _ => {
                self.take_digits(&mut mantissa, start)?;
            }
        }

        let mut is_float = false;
        let mut frac_digits: u32 = 0;
        if self.at() == Some('.') {
            self.inc();
            is_float = true;
            frac_digits = self.take_digits(&mut mantissa, start)?;
            if frac_digits == 0 {
                return Err(SexParserError::new_invalid_number(start));
            }
        }

        let mut exp: i32 = 0;
        if self.at() == Some('e') || self.at() == Some('E') {
            self.inc();
            is_float = true;
            let exp_negative = if self.at() == Some('-') {
                self.inc();
                true
            } else {
                if self.at() == Some('+') {
                    self.inc();
                }
                false
            };
            let mut magnitude: u64 = 0;
            if self.take_digits(&mut magnitude, start)? == 0 {
                return Err(SexParserError::new_invalid_number(start));
            }
            let magnitude =
                i32::try_from(magnitude).map_err(|_| SexParserError::new_invalid_number(start))?;
            exp = if exp_negative { -magnitude } else { magnitude };
        }

        if !is_float {
            let n: i64 = if negative {
                if mantissa > (i64::MAX as u64) + 1 {
                    return Err(SexParserError::new_invalid_number(start));
                }
                if mantissa == (i64::MAX as u64) + 1 {
                    i64::MIN
                } else {
                    -(mantissa as i64)
                }
            } else {
                i64::try_from(mantissa).map_err(|_| SexParserError::new_invalid_number(start))?
            };
            Ok(Atom::Number(Number::Integer(n)))
        } else {
            let sign = if negative { -1.0 } else { 1.0 };
            let scale = 10f64.powi(exp - (frac_digits as i32));
            Ok(Atom::Number(Number::Float(
                sign * (mantissa as f64) * scale,
            )))
        }
    }

    fn take_digits(&mut self, mantissa: &mut u64, start: Position) -> Result<u32, SexParserError> {
        let mut count: u32 = 0;
        while let Some(digit) = self.at().and_then(|ch| ch.to_digit(10)) {
            *mantissa = (*mantissa)
                .checked_mul(10)
                .and_then(|m| m.checked_add(digit as u64))
                .ok_or_else(|| SexParserError::new_invalid_number(start))?;
            self.inc();
            count += 1;
        }
        Ok(count)
    }
}

fn is_symbol_char(ch: char) -> bool {
    (ch.is_alphanumeric() || ch.is_ascii_graphic())
        && (ch != '(' && ch != ')' && ch != ';' && ch != '"' && ch != '|')
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
