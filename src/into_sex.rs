use crate::atom::{Atom, Number};
use crate::printer::print_atom;
use std::fmt;

pub trait IntoSex {
    fn into_sex(&self) -> Atom;
    fn sex_print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_atom(f, &self.into_sex())
    }
}

impl IntoSex for i64 {
    fn into_sex(&self) -> Atom {
        Atom::Number(Number::Integer(*self))
    }
}

macro_rules! impl_into_sex_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl IntoSex for $t {
                fn into_sex(&self) -> Atom {
                    Atom::Number(Number::Integer(*self as i64))
                }
            }
        )*
    };
}

impl_into_sex_int!(i8, i16, i32, isize, u8, u16, u32, u64, usize);

impl IntoSex for f64 {
    fn into_sex(&self) -> Atom {
        Atom::Number(Number::Float(*self))
    }
}

impl IntoSex for f32 {
    fn into_sex(&self) -> Atom {
        Atom::Number(Number::Float(*self as f64))
    }
}

impl IntoSex for bool {
    fn into_sex(&self) -> Atom {
        if *self { Atom::True } else { Atom::False }
    }
}

impl IntoSex for () {
    fn into_sex(&self) -> Atom {
        Atom::Nil
    }
}

impl IntoSex for String {
    fn into_sex(&self) -> Atom {
        Atom::string(self)
    }
}

impl IntoSex for &str {
    fn into_sex(&self) -> Atom {
        Atom::string(*self)
    }
}

impl<T: IntoSex> IntoSex for Option<T> {
    fn into_sex(&self) -> Atom {
        match self {
            Some(v) => v.into_sex(),
            None => Atom::Nil,
        }
    }
}

impl<T: IntoSex> IntoSex for Vec<T> {
    fn into_sex(&self) -> Atom {
        Atom::List(self.iter().map(IntoSex::into_sex).collect())
    }
}
