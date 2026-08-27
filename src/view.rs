use crate::FromSex;
use crate::atom::{Atom, AtomTy, SexError, Text, TextTy};
use crate::list::List;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ListView<'a> {
    atoms: &'a [Atom],
    curr: usize,
}

impl<'a> ListView<'a> {
    pub fn new(list: &'a List) -> Self {
        ListView { atoms: list.slice(), curr: 0 }
    }

    pub fn new_slice(atoms: &'a [Atom]) -> Self {
        ListView { atoms, curr: 0 }
    }

    /// Returns the currently selected atom.
    pub fn at(&self) -> Option<&'a Atom> {
        self.atoms.get(self.curr)
    }

    /// Increments the cursor, and returns the next atom.
    pub fn inc(&mut self) -> Option<&'a Atom> {
        self.curr += 1;
        if self.is_finished() {
            self.curr = self.atoms.len();
            return None;
        }
        Some(&self.atoms[self.curr])
    }

    /// Returns the currently selected atom, then increments the cursor.
    pub fn pop(&mut self) -> Option<&'a Atom> {
        let atom = self.atoms.get(self.curr)?;
        self.curr = self.curr.saturating_add(1).min(self.atoms.len());
        Some(atom)
    }

    /// Returns the next atom, but does not increment the cursor.
    pub fn peek(&self) -> Option<&'a Atom> {
        self.atoms.get(self.curr + 1)
    }

    /// Returns the currently selected atom.
    pub fn try_at(&mut self) -> Result<&'a Atom, SexError> {
        self.at().ok_or(SexError::ExpectedAtom)
    }

    /// Increments the cursor, and returns the next atom.
    pub fn try_inc(&mut self) -> Result<&'a Atom, SexError> {
        self.inc().ok_or(SexError::ExpectedAtom)
    }

    /// Returns the currently selected atom, then increments the cursor.
    pub fn try_pop(&mut self) -> Result<&'a Atom, SexError> {
        self.pop().ok_or(SexError::ExpectedAtom)
    }

    pub fn expect_last(&mut self) -> Result<&'a Atom, SexError> {
        let atom = self.try_at()?;
        _ = self.inc();
        self.expect_finished()?;
        Ok(atom)
    }

    pub fn skip(&mut self) {
        self.curr = self.curr.saturating_add(1).min(self.atoms.len());
    }
    
    pub fn skip_n(&mut self, n: usize) {
        self.curr = self.curr.saturating_add(n).min(self.atoms.len());
    }

    pub fn is_finished(&self) -> bool {
        self.curr >= self.atoms.len()
    }

    pub fn expect_finished(&self) -> Result<(), SexError> {
        if self.is_finished() {
            Ok(())
        } else {
            Err(SexError::ExpectedFinished)
        }
    }

    pub fn remaining(&self) -> usize {
        self.atoms.len().saturating_sub(self.curr)
    }

    pub fn remaining_slice(&self) -> &'a [Atom] {
        &self.atoms[self.curr..]
    }

    pub fn enter_list(&mut self) -> Result<ListView<'a>, SexError> {
        let atom = self.try_pop()?;
        match atom {
            Atom::List(elements) => Ok(ListView::new(elements)),
            other => Err(SexError::TypeError {
                expected: AtomTy::List,
                found: other.clone(),
            }),
        }
    }

    /// Returns a `KeywordView`, this essentially asserts that all remaining
    /// elements of the list are `:keyword` value pairs.
    pub fn into_keywords(&mut self) -> Result<KeywordView<'a>, SexError> {
        let mut result = KeywordView {
            map: HashMap::new(),
        };
        while let Some(atom) = self.at() {
            match atom {
                Atom::Text(Text {
                    ty: TextTy::Keyword,
                    contents,
                }) => {
                    let name: &str = contents;
                    let value = self.try_inc()?;
                    result.map.insert(name, value);
                    _ = self.inc();
                }
                other => {
                    return Err(SexError::TypeError {
                        expected: AtomTy::Keyword,
                        found: other.clone(),
                    });
                }
            }
        }
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct KeywordView<'a> {
    map: HashMap<&'a str, &'a Atom>,
}

impl<'a> KeywordView<'a> {
    pub fn from_slice(atoms: &'a [Atom]) -> Result<Self, SexError> {
        let mut view = ListView::new_slice(atoms);
        view.into_keywords()
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&'a Atom> {
        self.map.get(name).copied()
    }

    pub fn required<T: FromSex>(&self, name: &str) -> Result<T, SexError> {
        match self.map.get(name) {
            Some(atom) => {
                let mut view = ListView::new_slice(std::slice::from_ref(atom));
                T::from_sex(&mut view)
            }
            None => Err(SexError::MissingField {
                name: name.to_string(),
            }),
        }
    }

    pub fn optional<T: FromSex>(&self, name: &str) -> Result<Option<T>, SexError> {
        match self.map.get(name) {
            Some(atom) => {
                let mut view = ListView::new_slice(std::slice::from_ref(atom));
                T::from_sex(&mut view).map(Some)
            }
            None => Ok(None),
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'a str, &'a Atom)> + '_ {
        self.map.iter().map(|(k, v)| (*k, *v))
    }
}
