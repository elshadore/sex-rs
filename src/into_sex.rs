use crate::atom::{Atom, Number};
use crate::list::{List, ListBuilder};
use crate::printer::print_atom;
use std::fmt;

pub trait IntoSex {
    #[allow(clippy::wrong_self_convention)]
    fn into_atom(&self) -> Atom {
        let mut builder = ListBuilder::new();
        Self::into_list(self, &mut builder);
        Atom::List(builder.build())
    }

    #[allow(clippy::wrong_self_convention)]
    fn into_list(&self, builder: &mut ListBuilder) {
        builder.push(Self::into_atom(self));
    }

    #[deprecated = "use `into_atom` instead"]
    fn into_sex(&self) -> Atom {
        self.into_atom()
    }

    fn sex_print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_atom(f, &self.into_atom())
    }
}

impl IntoSex for i64 {
    fn into_atom(&self) -> Atom {
        Atom::Number(Number::Integer(*self))
    }
}

macro_rules! impl_into_sex_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl IntoSex for $t {
                fn into_atom(&self) -> Atom {
                    Atom::Number(Number::Integer(*self as i64))
                }
            }
        )*
    };
}

impl_into_sex_int!(i8, i16, i32, isize, u8, u16, u32, u64, usize);

impl IntoSex for f64 {
    fn into_atom(&self) -> Atom {
        Atom::Number(Number::Float(*self))
    }
}

impl IntoSex for f32 {
    fn into_atom(&self) -> Atom {
        Atom::Number(Number::Float(*self as f64))
    }
}

impl IntoSex for bool {
    fn into_atom(&self) -> Atom {
        if *self { Atom::True } else { Atom::False }
    }
}

impl IntoSex for () {
    fn into_atom(&self) -> Atom {
        Atom::Nil
    }
}

impl IntoSex for String {
    fn into_atom(&self) -> Atom {
        Atom::string(self)
    }
}

impl IntoSex for &str {
    fn into_atom(&self) -> Atom {
        Atom::string(*self)
    }
}

impl<T: IntoSex> IntoSex for Option<T> {
    fn into_atom(&self) -> Atom {
        match self {
            Some(v) => v.into_atom(),
            None => Atom::Nil,
        }
    }
}

impl<T: IntoSex> IntoSex for Vec<T> {
    fn into_atom(&self) -> Atom {
        Atom::List(self.iter().map(IntoSex::into_atom).collect())
    }
}

impl IntoSex for Atom {
    fn into_atom(&self) -> Atom {
        self.clone()
    }
}

impl IntoSex for List {
    fn into_atom(&self) -> Atom {
        Atom::List(self.clone())
    }
}