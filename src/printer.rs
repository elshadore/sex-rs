use crate::atom::{Atom, Number, TextTy};
use crate::parser_data::is_symbol_char;
use std::fmt;

fn needs_bar(s: &str) -> bool {
    s.chars().any(|c| !is_symbol_char(c))
}

fn print_barred(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
    write!(f, "|")?;
    for c in s.chars() {
        match c {
            '\\' => write!(f, "\\\\")?,
            '|' => write!(f, "\\|")?,
            '"' => write!(f, "\\\"")?,
            '\n' => write!(f, "\\n")?,
            '\t' => write!(f, "\\t")?,
            '\r' => write!(f, "\\r")?,
            '\0' => write!(f, "\\0")?,
            c => write!(f, "{}", c)?,
        }
    }
    write!(f, "|")
}

fn print_name(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
    if needs_bar(s) {
        print_barred(f, s)
    } else {
        write!(f, "{}", s)
    }
}

pub fn print_number(f: &mut fmt::Formatter<'_>, num: &Number) -> fmt::Result {
    match num {
        Number::Integer(n) => write!(f, "{}", n),
        Number::Float(n) => write!(f, "{}", n),
    }
}

pub fn print_atom(f: &mut fmt::Formatter<'_>, atom: &Atom) -> fmt::Result {
    match atom {
        Atom::Nil => write!(f, "nil"),
        Atom::True => write!(f, "true"),
        Atom::False => write!(f, "false"),
        Atom::Number(num) => print_number(f, num),
        Atom::Text(t) => match t.ty {
            TextTy::Symbol => print_name(f, &t.contents),
            TextTy::Keyword => {
                write!(f, ":")?;
                print_name(f, &t.contents)
            }
            TextTy::String => {
                write!(f, "\"")?;
                for c in t.contents.chars() {
                    match c {
                        '\\' => write!(f, "\\\\")?,
                        '"' => write!(f, "\\\"")?,
                        '\n' => write!(f, "\\n")?,
                        '\t' => write!(f, "\\t")?,
                        '\r' => write!(f, "\\r")?,
                        '\0' => write!(f, "\\0")?,
                        c => write!(f, "{}", c)?,
                    }
                }
                write!(f, "\"")
            }
        },
        Atom::List(elems) => {
            write!(f, "(")?;
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", elem)?;
            }
            write!(f, ")")
        }
    }
}
