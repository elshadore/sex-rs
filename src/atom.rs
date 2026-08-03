use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomTy {
    Symbol,
    Keyword,
    Text,
    Integer,
    Float,
    Nil,
    List,
}

impl fmt::Display for AtomTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomTy::Symbol => write!(f, "symbol"),
            AtomTy::Keyword => write!(f, "keyword"),
            AtomTy::Text => write!(f, "text (symbol, keyword, or string)"),
            AtomTy::Integer => write!(f, "integer"),
            AtomTy::Float => write!(f, "float"),
            AtomTy::Nil => write!(f, "nil"),
            AtomTy::List => write!(f, "list"),
        }
    }
}

#[derive(Debug)]
pub enum SexError {
    TypeError {
        expected: AtomTy,
        found: Atom,
    },
    MissingField {
        name: String,
    },
    UnknownVariant {
        variant: String,
        expected: Vec<String>,
    },
    Overflow {
        expected: AtomTy,
        value: String,
    },
    ExpectedAtom,
    ExpectedFinished,
}

impl fmt::Display for SexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SexError::TypeError { expected, found } => {
                write!(f, "type error: expected {}, found {:?}", expected, found)
            }
            SexError::MissingField { name } => write!(f, "missing field: {}", name),
            SexError::UnknownVariant { variant, expected } => {
                write!(
                    f,
                    "unknown variant '{}', expected one of: {:?}",
                    variant, expected
                )
            }
            SexError::Overflow { expected, value } => {
                write!(f, "overflow: value '{}' does not fit in {}", value, expected)
            }
            SexError::ExpectedAtom => write!(f, "expected atom, reached end of input"),
            SexError::ExpectedFinished => {
                write!(f, "expected end of input, but more atoms remain")
            }
        }
    }
}

impl std::error::Error for SexError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTy {
    Symbol,
    Keyword,
    String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub ty: TextTy,
    pub contents: String,
}

pub type List = Vec<Atom>;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Nil,
    Number(Number),
    Text(Text),
    List(List),
}

impl Atom {
    /// Create a symbol atom. This does *not* parse the symbol, so anything goes.
    pub fn symbol(s: impl Into<String>) -> Self {
        Atom::Text(Text {
            ty: TextTy::Symbol,
            contents: s.into(),
        })
    }

    /// Create a keyword atom. Again this do *not* parse to the keyword format,
    /// you do not need to prefix with the `:` keyword character and anything else goes.
    pub fn keyword(s: impl Into<String>) -> Self {
        Atom::Text(Text {
            ty: TextTy::Keyword,
            contents: s.into(),
        })
    }

    /// Create a string literal atom.
    pub fn string(s: impl Into<String>) -> Self {
        Atom::Text(Text {
            ty: TextTy::String,
            contents: s.into(),
        })
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Atom::Nil)
    }

    pub fn is_true(&self) -> bool {
        !self.is_nil()
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Atom::Text(t) if t.ty == TextTy::Symbol)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self, Atom::Text(t) if t.ty == TextTy::Keyword)
    }

    // Is the lisp atom a text type, i.e a string, symbol or a keyword.
    pub fn is_text(&self) -> bool {
        matches!(self, Atom::Text(_))
    }

    // Is the lisp atom a number type, i.e an integer or a float.
    pub fn is_number(&self) -> bool {
        matches!(self, Atom::Number(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Atom::Number(Number::Integer(_)))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Atom::Number(Number::Float(_)))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Atom::List(_))
    }

    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Atom::Text(t) if t.ty == TextTy::Symbol => Some(&t.contents),
            _ => None,
        }
    }

    pub fn try_as_symbol(&self) -> Result<&str, SexError> {
        self.as_symbol().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Symbol,
            found: self.clone(),
        })
    }

    pub fn as_keyword(&self) -> Option<&str> {
        match self {
            Atom::Text(t) if t.ty == TextTy::Keyword => Some(&t.contents),
            _ => None,
        }
    }

    pub fn try_as_keyword(&self) -> Result<&str, SexError> {
        self.as_keyword().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Keyword,
            found: self.clone(),
        })
    }

    pub fn as_text(&self) -> Option<&Text> {
        match self {
            Atom::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn try_as_text(&self) -> Result<&Text, SexError> {
        self.as_text().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Text,
            found: self.clone(),
        })
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Atom::Number(Number::Integer(n)) => Some(*n),
            _ => None,
        }
    }

    pub fn try_as_integer(&self) -> Result<i64, SexError> {
        self.as_integer().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Integer,
            found: self.clone(),
        })
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Atom::Number(Number::Float(n)) => Some(*n),
            _ => None,
        }
    }

    pub fn try_as_float(&self) -> Result<f64, SexError> {
        self.as_float().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Float,
            found: self.clone(),
        })
    }

    pub fn as_list(&self) -> Option<&List> {
        match self {
            Atom::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn try_as_list(&self) -> Result<&List, SexError> {
        self.as_list().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::List,
            found: self.clone(),
        })
    }
}

pub trait FromSex: Sized {
    fn from_sex(atom: &Atom) -> Result<Self, SexError>;
}

impl FromSex for String {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Text(t) => Ok(t.contents.clone()),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Text,
                found: atom.clone(),
            }),
        }
    }
}

impl FromSex for i64 {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Number(Number::Integer(n)) => Ok(*n),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Integer,
                found: atom.clone(),
            }),
        }
    }
}

macro_rules! template_impl_from_sex_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl FromSex for $t {
                fn from_sex(atom: &Atom) -> Result<Self, SexError> {
                    match atom {
                        Atom::Number(Number::Integer(n)) => {
                            <$t>::try_from(*n).map_err(|_| SexError::Overflow {
                                expected: AtomTy::Integer,
                                value: n.to_string(),
                            })
                        }
                        _ => Err(SexError::TypeError {
                            expected: AtomTy::Integer,
                            found: atom.clone(),
                        }),
                    }
                }
            }
        )*
    };
}

template_impl_from_sex_int!(i8, i16, i32, isize, u8, u16, u32, u64, usize);

impl FromSex for f64 {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Number(Number::Float(n)) => Ok(*n),
            Atom::Number(Number::Integer(n)) => Ok(*n as f64),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Float,
                found: atom.clone(),
            }),
        }
    }
}

impl FromSex for f32 {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Number(Number::Float(n)) => {
                if (*n).is_nan() || (*n).abs() > f32::MAX as f64 {
                    Err(SexError::Overflow {
                        expected: AtomTy::Float,
                        value: n.to_string(),
                    })
                } else {
                    Ok(*n as f32)
                }
            }
            Atom::Number(Number::Integer(n)) => Ok(*n as f32),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Float,
                found: atom.clone(),
            }),
        }
    }
}

impl FromSex for bool {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Nil => Ok(false),
            _ => Ok(true),
        }
    }
}

impl FromSex for () {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Nil => Ok(()),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Nil,
                found: atom.clone(),
            }),
        }
    }
}

impl<T: FromSex> FromSex for Option<T> {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Nil => Ok(None),
            _ => Ok(Some(T::from_sex(atom)?)),
        }
    }
}

impl<T: FromSex> FromSex for Vec<T> {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::List(list) => list.iter().map(|a| T::from_sex(a)).collect(),
            _ => Err(SexError::TypeError {
                expected: AtomTy::List,
                found: atom.clone(),
            }),
        }
    }
}
