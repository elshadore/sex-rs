use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomTy {
    Symbol,
    Keyword,
    Text,
    Integer,
    Float,
    List,
    Nil,
    True,
    False,
    Logic,
}

impl fmt::Display for AtomTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomTy::Symbol => write!(f, "symbol"),
            AtomTy::Keyword => write!(f, "keyword"),
            AtomTy::Text => write!(f, "text (symbol || keyword || string)"),
            AtomTy::Integer => write!(f, "integer"),
            AtomTy::Float => write!(f, "float"),
            AtomTy::Nil => write!(f, "nil"),
            AtomTy::List => write!(f, "list"),
            AtomTy::True => write!(f, "true"),
            AtomTy::False => write!(f, "false"),
            AtomTy::Logic => write!(f, "logic (true || false || nil)"),
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
    True,
    False,
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
        matches!(self, Atom::True)
    }

    /// Is either `false` or `nil`.
    pub fn is_falsey(&self) -> bool {
        matches!(self, Atom::False | Atom::Nil)
    }

    /// Is literally the `false` value. So `nil` returns `false`, and `false` returns `true`. Easy!
    pub fn is_false_strict(&self) -> bool {
        matches!(self, Atom::False)
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Atom::Text(t) if t.ty == TextTy::Symbol)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self, Atom::Text(t) if t.ty == TextTy::Keyword)
    }

    /// Is the lisp atom a text type, i.e a string, symbol or a keyword.
    pub fn is_text(&self) -> bool {
        matches!(self, Atom::Text(_))
    }

    /// Is the lisp atom a number type, i.e an integer or a float.
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
