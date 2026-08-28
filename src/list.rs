use crate::Atom;
use crate::printer::print_list;
use std::fmt;
use std::ops::Deref;

#[macro_export]
macro_rules! list {
    ($($elem:expr),* $(,)?) => {
        $crate::List::from(vec![$($elem),*])
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    vec: Vec<Atom>,
}

impl List {
    pub fn get(&self, index: usize) -> Option<&Atom> {
        self.vec.get(index)
    }

    pub fn from_slice(slice: &[Atom]) -> Self {
        List {
            vec: slice.to_vec(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Atom> {
        self.vec.iter()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn slice(&self) -> &[Atom] {
        &self.vec
    }
}

impl AsRef<[Atom]> for List {
    fn as_ref(&self) -> &[Atom] {
        &self.vec
    }
}

impl Deref for List {
    type Target = [Atom];

    fn deref(&self) -> &[Atom] {
        &self.vec
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = &'a Atom;
    type IntoIter = std::slice::Iter<'a, Atom>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

impl FromIterator<Atom> for List {
    fn from_iter<I: IntoIterator<Item = Atom>>(iter: I) -> Self {
        List {
            vec: iter.into_iter().collect(),
        }
    }
}

impl From<&[Atom]> for List {
    fn from(value: &[Atom]) -> Self {
        List::from_slice(value)
    }
}

impl From<Vec<Atom>> for List {
    fn from(vec: Vec<Atom>) -> Self {
        List { vec }
    }
}

impl From<ListBuilder> for List {
    fn from(value: ListBuilder) -> Self {
        value.build()
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_list(f, &self.vec)
    }
}

pub struct ListBuilder {
    vec: Vec<Atom>,
}

impl ListBuilder {
    pub fn new() -> Self {
        ListBuilder { vec: Vec::new() }
    }

    pub fn push(&mut self, atom: Atom) {
        self.vec.push(atom);
    }

    pub fn append<'a>(&mut self, other: impl Iterator<Item = &'a Atom>) {
        for atom in other {
            self.push(atom.clone());
        }
    }

    pub fn pop(&mut self) -> Option<Atom> {
        self.vec.pop()
    }

    pub fn build(self) -> List {
        List { vec: self.vec }
    }
}

impl From<List> for ListBuilder {
    fn from(value: List) -> Self {
        ListBuilder { vec: value.vec }
    }
}

impl From<&[Atom]> for ListBuilder {
    fn from(value: &[Atom]) -> Self {
        ListBuilder {
            vec: Vec::from(value),
        }
    }
}

impl From<Vec<Atom>> for ListBuilder {
    fn from(vec: Vec<Atom>) -> Self {
        ListBuilder { vec }
    }
}

impl FromIterator<Atom> for ListBuilder {
    fn from_iter<I: IntoIterator<Item = Atom>>(iter: I) -> Self {
        ListBuilder {
            vec: iter.into_iter().collect(),
        }
    }
}
