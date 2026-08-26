use crate::atom::{Atom, Number, TextTy};
use std::fmt;

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
            TextTy::Symbol => write!(f, "{}", t.contents),
            TextTy::Keyword => write!(f, ":{}", t.contents),
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

