use crate::atom::{Atom, Number, Text, TextTy};
use crate::list::{List, ListBuilder};
use crate::parser_data::*;
use crate::{err, err_unicode};
use crate::{err_number, is_symbol_char};
use std::io::{BufRead, BufReader, Cursor, Read};

fn skip_whitespace<R: BufRead>(p: &mut Parser<R>) -> bool {
    let mut result: bool = false;
    loop {
        match p.at() {
            None => return result,
            Some(c) if c.is_whitespace() => {
                result = true;
                p.inc();
            }
            Some(';') => {
                result = true;
                while let Some(c) = p.at() {
                    p.inc();
                    if c == '\n' {
                        break;
                    }
                }
            }
            _ => return result,
        }
    }
}

fn read_list<R: BufRead>(p: &mut Parser<R>) -> Result<Atom, SexParserError> {
    p.try_inc('(')?;

    let mut list = ListBuilder::new();
    let mut first: bool = true;
    let mut whitespace = skip_whitespace(p);

    loop {
        match p.at() {
            None => return err!(p, UnterminatedList),
            Some(')') => {
                p.inc();
                break;
            }
            Some(c) => {
                if !first && !whitespace {
                    return err!(p, ExpectedWhitespace(c));
                }
                list.push(read_atom(p)?);
            }
        }
        whitespace = skip_whitespace(p);
        first = false;
    }
    Ok(Atom::List(list.build()))
}

fn read_hex_digit<R: BufRead>(p: &mut Parser<R>) -> Result<u8, HexError> {
    match p.inc() {
        Some(c) if c.is_ascii_hexdigit() => Ok(c.to_digit(16).unwrap_or(0) as u8),
        Some(c) => Err(HexError::Invalid(c)),
        None => Err(HexError::NoChar),
    }
}

fn read_hex_escape<R: BufRead>(p: &mut Parser<R>) -> Result<char, SexParserError> {
    match read_hex_digit(p) {
        Ok(hi) => match read_hex_digit(p) {
            Ok(lo) => Ok(char::from((hi << 4) | lo)),
            Err(HexError::Invalid(right)) => err!(
                p,
                MalformedHexEscape(MalformedHexCode::InvalidRight {
                    left: hi.into(),
                    right,
                })
            ),
            Err(HexError::NoChar) => err!(
                p,
                MalformedHexEscape(MalformedHexCode::MissingRight { left: hi.into() })
            ),
        },
        Err(HexError::Invalid(left)) => err!(
            p,
            MalformedHexEscape(MalformedHexCode::InvalidLeft { left })
        ),
        Err(HexError::NoChar) => err!(p, MalformedHexEscape(MalformedHexCode::MissingLeft)),
    }
}

fn read_unicode_escape<R: BufRead>(p: &mut Parser<R>) -> Result<char, SexParserError> {
    match p.inc() {
        Some(c) => {
            if c != '{' {
                return err_unicode!(p, EscapeOpeningBraceExpected(Some(c)));
            }
        }
        None => {
            return err_unicode!(p, EscapeOpeningBraceExpected(None));
        }
    }

    let mut value: u32 = 0;
    let mut digits: u32 = 0;
    loop {
        match p.inc() {
            Some('}') => break,
            Some(c) if c.is_ascii_hexdigit() => {
                if digits < 6 {
                    value = value * 16 + c.to_digit(16).unwrap_or(0);
                    digits += 1;
                } else {
                    return err_unicode!(p, HexMaximumOfSixReached);
                }
            }
            Some(c) => {
                return err_unicode!(p, ExpectedHexChar(c));
            }
            None => {
                return err_unicode!(p, EscapeUnterminated);
            }
        }
    }
    if digits == 0 {
        return err_unicode!(p, EmptyEscape);
    }
    match char::from_u32(value) {
        Some(c) => Ok(c),
        None => err_unicode!(p, InvalidUnicodeCodepoint(value)),
    }
}

fn read_string<R: BufRead>(p: &mut Parser<R>) -> Result<Atom, SexParserError> {
    p.try_inc('"')?;
    let mut s = String::new();
    loop {
        match p.inc() {
            None => return err!(p, UnterminatedString),
            Some('"') => {
                return Ok(Atom::Text(Text {
                    ty: TextTy::String,
                    contents: s,
                }));
            }
            Some('\\') => {
                let esc = match p.inc() {
                    None => return err!(p, UnterminatedString),
                    Some('"') => '"',
                    Some('\\') => '\\',
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('0') => '\0',
                    Some('x') => read_hex_escape(p)?,
                    Some('u') => read_unicode_escape(p)?,
                    Some(c) => {
                        return err!(p, MalformedStringEscape(c));
                    }
                };
                s.push(esc);
            }
            Some(c) => s.push(c),
        }
    }
}

fn read_barred<R: BufRead>(p: &mut Parser<R>, ty: BarredTy) -> Result<Atom, SexParserError> {
    p.try_inc('|')?;
    let mut name = String::new();
    loop {
        match p.inc() {
            None => return err!(p, UnterminatedBarSymbol),
            Some('|') => break,
            Some('\\') => {
                let esc = match p.inc() {
                    None => return err!(p, UnterminatedBarSymbol),
                    Some('"') => '"',
                    Some('|') => '|',
                    Some('\\') => '\\',
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('0') => '\0',
                    Some('x') => read_hex_escape(p)?,
                    Some('u') => read_unicode_escape(p)?,
                    Some(ch) => {
                        return err!(p, MalformedBarEscape(ch));
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

fn read_keyword<R: BufRead>(p: &mut Parser<R>) -> Result<Atom, SexParserError> {
    p.try_inc(':')?;
    if p.at() == Some('|') {
        return read_barred(p, BarredTy::Keyword);
    }
    let mut name = String::new();
    while let Some(ch) = p.at() {
        if !is_symbol_char(ch) {
            break;
        }
        name.push(ch);
        p.inc();
    }
    if name.is_empty() {
        return err!(p, EmptyKeyword);
    }
    Ok(Atom::Text(Text {
        ty: TextTy::Keyword,
        contents: name,
    }))
}

fn read_symbol<R: BufRead>(p: &mut Parser<R>) -> Result<Atom, SexParserError> {
    let mut name = String::new();
    while let Some(c) = p.at() {
        if !is_symbol_char(c) {
            break;
        }
        name.push(c);
        p.inc();
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

fn take_digits<R: BufRead>(p: &mut Parser<R>, s: &mut String) -> bool {
    let mut flag: bool = false;
    while let Some(c) = p.at() {
        if !c.is_ascii_digit() {
            break;
        }
        s.push(c);
        p.inc();
        flag = true;
    }
    flag
}

enum NumberLiteral {
    Integer,
    Float,
}

fn read_number_literal<R: BufRead>(
    p: &mut Parser<R>,
) -> Result<(NumberLiteral, String), SexParserError> {
    let mut result = String::new();
    let mut ty = NumberLiteral::Integer;

    if p.at() == Some('-') {
        result.push('-');
        p.inc();
    }

    match p.at() {
        Some('0') => {
            result.push('0');
            if let Some(c) = p.inc().filter(|c| c.is_ascii_digit()) {
                return err_number!(p, ZeroHasTrailingDigit(c));
            }
        }
        Some(c) if c.is_ascii_digit() => {
            _ = take_digits(p, &mut result);
        }
        _ => {
            return err_number!(p, NoDigits);
        }
    }

    if p.at() == Some('.') {
        result.push('.');
        p.inc();
        ty = NumberLiteral::Float;
        if !take_digits(p, &mut result) {
            return err_number!(p, NoDecimalDigits);
        }
    }

    if p.at().is_some_and(|c| c == 'e' || c == 'E') {
        let c = p.at().unwrap();
        result.push(c);
        p.inc();
        ty = NumberLiteral::Float;
        if p.at() == Some('-') {
            result.push('-');
            p.inc();
        } else if p.at() == Some('+') {
            result.push('+');
            p.inc();
        }
        if !take_digits(p, &mut result) {
            return err_number!(p, NoExponentDigits);
        }
    }

    Ok((ty, result))
}

fn read_number<R: BufRead>(p: &mut Parser<R>) -> Result<Atom, SexParserError> {
    let start = p.pos;
    let (ty, s) = read_number_literal(p)?;
    const ERROR: SexParserErrorKind =
        SexParserErrorKind::MalformedNumber(MalformedNumber::NumberTooBigForPrecision);
    let number = match ty {
        NumberLiteral::Float => s
            .parse::<f64>()
            .map(Number::Float)
            .map_err(|_| p.error(start, ERROR)),
        NumberLiteral::Integer => s
            .parse::<i64>()
            .map(Number::Integer)
            .map_err(|_| p.error(start, ERROR)),
    };
    number.map(Atom::Number)
}

fn read_atom<R: BufRead>(p: &mut Parser<R>) -> Result<Atom, SexParserError> {
    match p.at() {
        None => err!(p, UnexpectedEof),
        Some('(') => read_list(p),
        Some('"') => read_string(p),
        Some(':') => read_keyword(p),
        Some('|') => read_barred(p, BarredTy::Symbol),
        Some(c) if c == '-' && p.peek().is_some_and(|c| c.is_ascii_digit()) => read_number(p),
        Some(c) if c.is_ascii_digit() => read_number(p),
        Some(c) if is_symbol_char(c) => read_symbol(p),
        Some(c) => err!(p, UnexpectedChar(c)),
    }
}

fn parse_exprlist<R: BufRead>(p: &mut Parser<R>) -> Result<List, SexParserError> {
    let mut atoms = ListBuilder::new();
    let mut whitespace: bool = true;

    skip_whitespace(p);

    loop {
        if p.is_finished() {
            break;
        }
        if !whitespace {
            return err!(p, ExpectedWhitespace(p.at().unwrap_or('\0')));
        }
        atoms.push(read_atom(p)?);
        whitespace = skip_whitespace(p);
    }

    Ok(atoms.build())
}

fn parse_expression<R: BufRead>(p: &mut Parser<R>) -> Result<Atom, SexParserAtomError> {
    skip_whitespace(p);
    let result = read_atom(p)?;
    skip_whitespace(p);
    if !p.is_finished() {
        return Err(SexParserAtomError::new_expected_single_atom(
            p.pos,
            p.file.clone(),
        ));
    }
    Ok(result)
}

/// Parses a list of expressions from a string.
/// | Input         | Output |
/// | :------------ | -----: |
/// | foo           | (foo)  |
/// | (foo bar baz) | ((foo bar bar)) |
/// | (foo bar) baz | ((foo bar) baz) |
/// |               | () |
pub fn parse_exprlist_str(
    input: impl AsRef<str>,
    file: Option<String>,
) -> Result<List, SexParserError> {
    let s = input.as_ref();
    let cursor = Cursor::new(s.as_bytes());
    let mut parser = Parser::new(cursor, file);
    parse_exprlist(&mut parser)
}

/// Parses a list of expressions from a generic reader.
/// | Input         | Output |
/// | :------------ | -----: |
/// | foo           | (foo)  |
/// | (foo bar baz) | ((foo bar bar)) |
/// | (foo bar) baz | ((foo bar) baz) |
/// |               | () |
pub fn parse_exprlist_reader(
    reader: impl Read,
    file: Option<String>,
) -> Result<List, SexParserError> {
    let reader = BufReader::new(reader);
    let mut parser = Parser::new(reader, file);
    parse_exprlist(&mut parser)
}

/// Parses a single expression from a string.
/// | Input         | Output |
/// | :------------ | -----: |
/// | foo           | foo    |
/// | (foo bar baz) | (foo bar bar) |
/// | (foo bar) baz | `Error` |
/// |               | `Error` |
pub fn parse_expression_str(
    input: impl AsRef<str>,
    file: Option<String>,
) -> Result<Atom, SexParserAtomError> {
    let s = input.as_ref();
    let cursor = Cursor::new(s.as_bytes());
    let mut parser = Parser::new(cursor, file);
    parse_expression(&mut parser)
}

/// Parses a single expression from a generic reader.
/// | Input         | Output |
/// | :------------ | -----: |
/// | foo           | foo    |
/// | (foo bar baz) | (foo bar bar) |
/// | (foo bar) baz | `Error` |
/// |               | `Error` |
pub fn parse_expression_reader(
    reader: impl Read,
    file: Option<String>,
) -> Result<Atom, SexParserAtomError> {
    let reader = BufReader::new(reader);
    let mut parser = Parser::new(reader, file);
    parse_expression(&mut parser)
}
