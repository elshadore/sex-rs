use super::{Atom, AtomTy, Number, SexError};
use super::view::ListView;

pub trait FromSex: Sized {
    fn from_list(view: &mut ListView) -> Result<Self, SexError> {
        Self::from_atom(view.try_pop()?)
    }

    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
        let list = atom.try_as_list()?;
        let mut view = ListView::new(list);
        Self::from_list(&mut view)
    }
}

macro_rules! impl_from_sex_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl FromSex for $t {
                fn from_atom(atom: &Atom) -> Result<Self, SexError> {
                    match atom {
                        Atom::Number(Number::Integer(n)) => {
                            <$t>::try_from(*n).map_err(|_| SexError::IntegerOverflow {
                                expected_size: std::mem::size_of::<$t>() * 8,
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

impl_from_sex_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl FromSex for String {
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

impl FromSex for f64 {
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

impl FromSex for f32 {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Number(Number::Float(n)) => {
                if (*n).is_nan() || (*n).abs() > f32::MAX as f64 {
                    Err(SexError::FloatOverflow {
                        expected_size: std::mem::size_of::<f32>() * 8,
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

impl FromSex for () {
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

impl<T: FromSex> FromSex for Option<T> {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::Nil => Ok(None),
            _ => Ok(Some(T::from_atom(atom)?)),
        }
    }
}

impl<T: FromSex> FromSex for Vec<T> {
    fn from_atom(atom: &Atom) -> Result<Self, SexError> {
        match atom {
            Atom::List(list) => {
                let mut view = ListView::new(list);
                let mut result = Vec::new();
                while !view.is_finished() {
                    result.push(T::from_atom(view.try_pop()?)?);
                }
                Ok(result)
            }
            _ => Err(SexError::TypeError {
                expected: AtomTy::List,
                found: atom.clone(),
            }),
        }
    }
}