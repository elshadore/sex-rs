use std::fmt;

/// A source position (line and column) tracked during parsing.
///
/// Displayed as `line:col`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

/// The type of an [`Atom`], used in [`SexError::TypeError`] to describe what was expected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomTy {
    Symbol,
    Keyword,
    Text,
    Integer,
    Float,
    True,
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
            AtomTy::True => write!(f, "true"),
            AtomTy::Nil => write!(f, "nil"),
            AtomTy::List => write!(f, "list"),
        }
    }
}

/// Errors that can occur during S-expression parsing and deserialization.
#[derive(Debug)]
pub enum SexError {
    /// Reached end of input unexpectedly at the given position.
    UnexpectedEof { pos: Position },
    /// An unexpected character was encountered.
    UnexpectedChar { pos: Position, ch: char },
    /// A list opened with `(` was never closed with `)`.
    UnterminatedList { pos: Position },
    /// A string opened with `"` was never closed.
    UnterminatedString { pos: Position },
    /// An invalid escape sequence was found inside a string.
    InvalidEscape { pos: Position, ch: char },
    /// Text that looks like a number could not be parsed.
    InvalidNumber { pos: Position, value: String },
    /// A keyword started with `:` but had no name.
    EmptyKeyword { pos: Position },
    /// An atom of the wrong type was encountered (e.g. expected integer, got symbol).
    TypeError { expected: AtomTy, found: Atom },
    /// A required field was not present when deserializing with [`KeywordView`].
    MissingField { name: String },
    /// An unrecognized enum variant was encountered.
    UnknownVariant { variant: String, expected: Vec<String> },
    /// [`AtomView::try_next`] was called on an empty view.
    ExpectedAtom,
    /// [`AtomView::expect_finished`] was called but atoms remain unconsumed.
    ExpectedFinished,
}

impl fmt::Display for SexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SexError::UnexpectedEof { pos } => write!(f, "{}: unexpected EOF", pos),
            SexError::UnexpectedChar { pos, ch } => {
                write!(f, "{}: unexpected character '{}'", pos, ch)
            }
            SexError::UnterminatedList { pos } => {
                write!(f, "{}: unterminated list, expected ')'", pos)
            }
            SexError::UnterminatedString { pos } => write!(f, "{}: unterminated string", pos),
            SexError::InvalidEscape { pos, ch } => {
                write!(f, "{}: invalid escape sequence '\\{}'", pos, ch)
            }
            SexError::InvalidNumber { pos, value } => {
                write!(f, "{}: invalid number '{}'", pos, value)
            }
            SexError::EmptyKeyword { pos } => write!(f, "{}: empty keyword", pos),
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
            SexError::ExpectedAtom => write!(f, "expected atom, reached end of input"),
            SexError::ExpectedFinished => {
                write!(f, "expected end of input, but more atoms remain")
            }
        }
    }
}

impl std::error::Error for SexError {}

/// A numeric atom value, either integer or float.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

/// The variant of a [`Text`] atom — symbol, keyword, or string literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTy {
    /// A bare word (e.g. `hello`, `move`, `true`).
    Symbol,
    /// A keyword prefixed with `:` (e.g. `:name`, `:x`).
    Keyword,
    /// A quoted string literal (e.g. `"hello world"`).
    String,
}

/// A text atom with a type tag and string contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub ty: TextTy,
    pub contents: String,
}

/// A list of [`Atom`]s, representing a parenthesized S-expression.
pub type List = Vec<Atom>;

/// A parsed S-expression atom.
///
/// An atom is the fundamental unit of an S-expression. It may be [`Nil`](Atom::Nil),
/// [`True`](Atom::True), a [`Number`], a [`Text`] value (symbol, keyword, or string),
/// or a [`List`] of sub-atoms.
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    /// The `nil` value, representing nothingness or false.
    Nil,
    /// The `true` value.
    True,
    /// A numeric value (integer or float).
    Number(Number),
    /// A text value — symbol, keyword, or string literal.
    Text(Text),
    /// A parenthesized list of sub-atoms.
    List(List),
}

impl Atom {
    /// Create a symbol atom. This does *not* parse the symbol, so anything goes.
    pub fn symbol(s: impl Into<String>) -> Self {
        Atom::Text(Text { ty: TextTy::Symbol, contents: s.into() })
    }

    /// Create a keyword atom. Again this do *not* parse to the keyword format,
    /// you do not need to prefix with the `:` keyword character and anything else goes.
    pub fn keyword(s: impl Into<String>) -> Self {
        Atom::Text(Text { ty: TextTy::Keyword, contents: s.into() })
    }

    /// Create a string-literal atom.
    pub fn string(s: impl Into<String>) -> Self {
        Atom::Text(Text { ty: TextTy::String, contents: s.into() })
    }

    /// Returns `true` if this atom is `nil`.
    pub fn is_nil(&self) -> bool {
        matches!(self, Atom::Nil)
    }

    /// Returns `true` if this atom is `true`.
    pub fn is_true(&self) -> bool {
        matches!(self, Atom::True)
    }

    /// Returns `true` if this atom is a symbol.
    pub fn is_symbol(&self) -> bool {
        matches!(self, Atom::Text(t) if t.ty == TextTy::Symbol)
    }

    /// Returns `true` if this atom is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(self, Atom::Text(t) if t.ty == TextTy::Keyword)
    }

    /// Returns `true` if this atom is any text variant (symbol, keyword, or string).
    pub fn is_text(&self) -> bool {
        matches!(self, Atom::Text(_))
    }

    /// Returns `true` if this atom is a number (integer or float).
    pub fn is_number(&self) -> bool {
        matches!(self, Atom::Number(_))
    }

    /// Returns `true` if this atom is specifically an integer.
    pub fn is_integer(&self) -> bool {
        matches!(self, Atom::Number(Number::Integer(_)))
    }

    /// Returns `true` if this atom is specifically a float.
    pub fn is_float(&self) -> bool {
        matches!(self, Atom::Number(Number::Float(_)))
    }

    /// Returns `true` if this atom is a list.
    pub fn is_list(&self) -> bool {
        matches!(self, Atom::List(_))
    }

    /// Extract the symbol text, returning `None` if not a symbol.
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Atom::Text(t) if t.ty == TextTy::Symbol => Some(&t.contents),
            _ => None,
        }
    }

    /// Extract the symbol text, returning a [`TypeError`](SexError::TypeError) if not a symbol.
    pub fn try_as_symbol(&self) -> Result<&str, SexError> {
        self.as_symbol().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Symbol,
            found: self.clone(),
        })
    }

    /// Extract the keyword name returning `None` if not a keyword.
    /// This string does not contain the `:` keyword identifier.
    pub fn as_keyword(&self) -> Option<&str> {
        match self {
            Atom::Text(t) if t.ty == TextTy::Keyword => Some(&t.contents),
            _ => None,
        }
    }

    /// Extract the keyword name returning `None` if not a keyword.
    /// This string does not contain the `:` keyword identifier.
    /// This returns a [`TypeError`](SexError::TypeError) if not a keyword.
    pub fn try_as_keyword(&self) -> Result<&str, SexError> {
        self.as_keyword().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Keyword,
            found: self.clone(),
        })
    }

    /// Returns the [`Text`](Atom::Text) object type that encompasses [`Symbol`](TextTy::Symbol), [`Keyword`](TextTy::Keyword), and [`String`](TextTy::String), atom types.
    /// If the returning type if of type [`Keyword`](TextTy::Keyword), the string does not contain
    /// the `:` character identifier. This function returns `None` if not a [`Text`](Atom::Text) type.
    pub fn as_text(&self) -> Option<&Text> {
        match self {
            Atom::Text(t) => Some(t),
            _ => None,
        }
    }

    /// Returns the [`Text`](Atom::Text) object type that encompasses [`Symbol`](TextTy::Symbol), [`Keyword`](TextTy::Keyword), and [`String`](TextTy::String), atom types.
    /// If the returning type if of type [`Keyword`](TextTy::Keyword), the string does not contain the `:` character identifier.
    /// This function returns a [`TypeError`](SexError::TypeError) if not a [`Text`](Atom::Text) type.
    pub fn try_as_text(&self) -> Result<&Text, SexError> {
        self.as_text().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Text,
            found: self.clone(),
        })
    }

    /// Extract the integer value, returning `None` if not an integer.
    pub fn as_integer(&self) -> Option<i32> {
        match self {
            Atom::Number(Number::Integer(n)) => Some(*n),
            _ => None,
        }
    }

    /// Extract the integer value, returning a [`TypeError`](SexError::TypeError) if not an integer.
    pub fn try_as_integer(&self) -> Result<i32, SexError> {
        self.as_integer().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Integer,
            found: self.clone(),
        })
    }

    /// Extract the float value, returning `None` if not a float.
    pub fn as_float(&self) -> Option<f32> {
        match self {
            Atom::Number(Number::Float(n)) => Some(*n),
            _ => None,
        }
    }

    /// Extract the float value, returning a [`TypeError`](SexError::TypeError) if not a float.
    pub fn try_as_float(&self) -> Result<f32, SexError> {
        self.as_float().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::Float,
            found: self.clone(),
        })
    }

    /// Borrow the list elements, returning `None` if not a list.
    pub fn as_list(&self) -> Option<&List> {
        match self {
            Atom::List(l) => Some(l),
            _ => None,
        }
    }

    /// Borrow the list elements, returning a [`TypeError`](SexError::TypeError) if not a list.
    pub fn try_as_list(&self) -> Result<&List, SexError> {
        self.as_list().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::List,
            found: self.clone(),
        })
    }

    /// Extract the unit value, returning `Some(())` if this is `true`.
    pub fn as_true(&self) -> Option<()> {
        match self {
            Atom::True => Some(()),
            _ => None,
        }
    }

    /// Extract the unit value, returning a [`TypeError`](SexError::TypeError) if not `true`.
    pub fn try_as_true(&self) -> Result<(), SexError> {
        self.as_true().ok_or_else(|| SexError::TypeError {
            expected: AtomTy::True,
            found: self.clone(),
        })
    }
}

/// Deserialization trait for converting an [`Atom`] into a Rust value.
///
/// Analogous to `serde::Deserialize` — implementations should inspect the atom's
/// variant and extract the appropriate data, returning [`SexError::TypeError`] on
/// mismatch.
///
/// This trait can be derived with `#[derive(Sex)]` for structs and enums.
pub trait FromSex: Sized {
    fn from_sex(atom: &Atom) -> Result<Self, SexError>;
}

/// Deserialize a string from any text atom (symbol, keyword, or string literal).
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

/// Deserialize an `i32` from an integer atom.
///
/// Returns a [`SexError::TypeError`] for float or text atoms.
impl FromSex for i32 {
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

/// Deserialize an `f32` from a float or integer atom.
///
/// Integers are silently cast via `as f32`.
impl FromSex for f32 {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Number(Number::Float(n)) => Ok(*n),
            Atom::Number(Number::Integer(n)) => Ok(*n as f32),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Float,
                found: atom.clone(),
            }),
        }
    }
}

/// Deserialize a `bool` from `true`, `false`, or `nil`.
///
/// `true` atom → `true`, symbol `"true"` → `true`, `nil` → `false`, symbol `"false"` → `false`.
impl FromSex for bool {
    fn from_sex(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::True => Ok(true),
            Atom::Text(t) if t.ty == TextTy::Symbol && t.contents == "true" => Ok(true),
            Atom::Text(t) if t.ty == TextTy::Symbol && t.contents == "false" => Ok(false),
            Atom::Nil => Ok(false),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Symbol,
                found: atom.clone(),
            }),
        }
    }
}

/// Deserialize `()` (unit) from `nil`.
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

/// Deserialize a `Vec<T>` from a list atom by mapping each element.
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
