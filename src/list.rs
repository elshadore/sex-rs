use crate::Atom;
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
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Atom> {
        self.vec.iter()
    }

    pub fn get(&self, index: usize) -> Option<&Atom> {
        self.vec.get(index)
    }

    pub fn from_slice(slice: &[Atom]) -> Self {
        List {
            vec: slice.to_vec(),
        }
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

impl From<Vec<Atom>> for List {
    fn from(vec: Vec<Atom>) -> Self {
        List { vec }
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

impl From<Vec<Atom>> for ListBuilder {
    fn from(vec: Vec<Atom>) -> Self {
        ListBuilder { vec }
    }
}
