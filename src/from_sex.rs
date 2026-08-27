use super::{Atom, AtomTy, Number, SexError};
use super::view::ListView;

pub trait FromAtom: Sized {
    fn from_atom(atom: &Atom) -> Result<Self, SexError>;
}

pub trait FromSex: Sized {
    fn from_sex(view: &mut ListView) -> Result<Self, SexError>;
}

macro_rules! impl_from_atom_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl FromAtom for $t {
                fn from_atom(atom: &Atom) -> Result<Self, SexError> {
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

            impl FromSex for $t {
                fn from_sex(view: &mut ListView) -> Result<Self, SexError> {
                    Self::from_atom(view.try_pop()?)
                }
            }
        )*
    };
}

impl_from_atom_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl FromAtom for String {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Text(t) => Ok(t.contents.clone()),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Text,
                found: atom.clone(),
            }),
        }
    }
}

impl FromSex for String {
    fn from_sex(view: &mut ListView) -> Result<Self, SexError> {
        Self::from_atom(view.try_pop()?)
    }
}

impl FromAtom for f64 {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
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

impl FromSex for f64 {
    fn from_sex(view: &mut ListView) -> Result<Self, SexError> {
        Self::from_atom(view.try_pop()?)
    }
}

impl FromAtom for f32 {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
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

impl FromSex for f32 {
    fn from_sex(view: &mut ListView) -> Result<Self, SexError> {
        Self::from_atom(view.try_pop()?)
    }
}

impl FromAtom for bool {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::True => Ok(true),
            Atom::False => Ok(false),
            Atom::Nil => Ok(false),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Logic,
                found: atom.clone(),
            }),
        }
    }
}

impl FromSex for bool {
    fn from_sex(view: &mut ListView) -> Result<Self, SexError> {
        Self::from_atom(view.try_pop()?)
    }
}

impl FromAtom for () {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Nil => Ok(()),
            _ => Err(SexError::TypeError {
                expected: AtomTy::Nil,
                found: atom.clone(),
            }),
        }
    }
}

impl FromSex for () {
    fn from_sex(view: &mut ListView) -> Result<Self, SexError> {
        Self::from_atom(view.try_pop()?)
    }
}

impl<T: FromAtom> FromAtom for Option<T> {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Nil => Ok(None),
            _ => Ok(Some(T::from_atom(atom)?)),
        }
    }
}

impl<T: FromSex> FromSex for Option<T> {
    fn from_sex(view: &mut ListView) -> Result<Self, SexError> {
        match view.at() {
            Some(Atom::Nil) => {
                view.skip();
                Ok(None)
            }
            _ => Ok(Some(T::from_sex(view)?)),
        }
    }
}

impl<T: FromAtom> FromAtom for Vec<T> {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::List(list) => list.iter().map(|a| T::from_atom(a)).collect(),
            _ => Err(SexError::TypeError {
                expected: AtomTy::List,
                found: atom.clone(),
            }),
        }
    }
}

impl<T: FromAtom> FromSex for Vec<T> {
    fn from_sex(view: &mut ListView) -> Result<Self, SexError> {
        view.remaining_slice().iter().map(|a| T::from_atom(a)).collect()
    }
}
