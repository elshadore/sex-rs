use crate::FromSex;
use crate::atom::{Atom, AtomTy, Position, SexError, Text, TextTy};
use std::collections::HashMap;

/// A cursor over a borrowed slice of [`Atom`]s.
///
/// Provides sequential access methods for consuming and inspecting atoms in order,
/// typed deserialization via [`FromSex`], and conveniences for keyword-value patterns
/// and nested list traversal.
#[derive(Debug, Clone)]
pub struct AtomView<'a> {
    atoms: &'a [Atom],
    pos: usize,
}

impl<'a> AtomView<'a> {
    /// Create a new view over the given atom slice.
    pub fn new(atoms: &'a [Atom]) -> Self {
        AtomView { atoms, pos: 0 }
    }

    /// Look at the current atom without consuming it.
    pub fn peek(&self) -> Option<&'a Atom> {
        self.atoms.get(self.pos)
    }

    /// Consume and return the current atom, advancing the cursor.
    ///
    /// Returns `None` if all atoms have been consumed.
    pub fn next(&mut self) -> Option<&'a Atom> {
        let atom = self.atoms.get(self.pos)?;
        self.pos += 1;
        Some(atom)
    }

    /// Like [`peek`](Self::peek) but returns an error instead of `None`.
    pub fn try_peek(&mut self) -> Result<&'a Atom, SexError> {
        self.peek().ok_or(SexError::ExpectedAtom)
    }

    /// Like [`next`](Self::next) but returns an error instead of `None`.
    pub fn try_next(&mut self) -> Result<&'a Atom, SexError> {
        self.next().ok_or(SexError::ExpectedAtom)
    }

    /// Consume the next atom and verify it is the last one.
    ///
    /// Equivalent to `try_next()` followed by `expect_finished()`.
    pub fn expect_last(&mut self) -> Result<&'a Atom, SexError> {
        let atom = self.try_next()?;
        self.expect_finished()?;
        Ok(atom)
    }

    /// Advance the cursor by `n` atoms, saturating at the end.
    pub fn skip(&mut self, n: usize) {
        self.pos = self.pos.saturating_add(n).min(self.atoms.len());
    }

    /// Returns `true` if all atoms have been consumed.
    pub fn is_finished(&self) -> bool {
        self.pos >= self.atoms.len()
    }

    /// Error if any atoms remain unconsumed.
    pub fn expect_finished(&self) -> Result<(), SexError> {
        if self.is_finished() {
            Ok(())
        } else {
            Err(SexError::ExpectedFinished)
        }
    }

    /// Number of unconsumed atoms.
    pub fn remaining(&self) -> usize {
        self.atoms.len().saturating_sub(self.pos)
    }

    /// The unconsumed portion of the atom slice.
    pub fn remaining_slice(&self) -> &'a [Atom] {
        &self.atoms[self.pos..]
    }

    /// Consume the next atom as a list and return a new [`AtomView`] over its elements.
    pub fn enter_list(&mut self) -> Result<AtomView<'a>, SexError> {
        let atom = self.next().ok_or_else(|| SexError::UnexpectedEof {
            pos: Position { line: 0, col: 0 },
        })?;
        match atom {
            Atom::List(elements) => Ok(AtomView::new(elements)),
            other => Err(SexError::TypeError {
                expected: AtomTy::List,
                found: other.clone(),
            }),
        }
    }

    /// Consume remaining atoms as strict keyword-value pairs.
    ///
    /// Every atom in the remaining slice must be a keyword (`:name`) followed by a value.
    /// Returns a [`KeywordView`] mapping keyword names to their value atoms.
    ///
    /// # Errors
    ///
    /// - [`SexError::TypeError`] if a non-keyword atom is encountered.
    /// - [`SexError::UnexpectedEof`] if a keyword has no following value.
    pub fn into_keywords(self) -> Result<KeywordView<'a>, SexError> {
        let mut map = HashMap::new();
        let mut peekable = self;
        while let Some(atom) = peekable.peek() {
            match atom {
                Atom::Text(Text {
                    ty: TextTy::Keyword,
                    contents,
                }) => {
                    let name: &str = contents;
                    peekable.next();
                    match peekable.next() {
                        Some(value) => {
                            map.insert(name, value);
                        }
                        None => {
                            return Err(SexError::UnexpectedEof {
                                pos: Position { line: 0, col: 0 },
                            });
                        }
                    }
                }
                other => {
                    return Err(SexError::TypeError {
                        expected: AtomTy::Keyword,
                        found: other.clone(),
                    });
                }
            }
        }
        Ok(KeywordView { map })
    }
}

/// A view over parsed keyword-value pairs.
///
/// Created by calling [`AtomView::into_keywords`] on an `AtomView` whose remaining
/// atoms are expected to be strictly `:key value :key value ...` pairs.
///
/// Provides HashMap-style lookup with `required` / `optional` typed accessors.
#[derive(Debug, Clone)]
pub struct KeywordView<'a> {
    map: HashMap<&'a str, &'a Atom>,
}

impl<'a> KeywordView<'a> {
    /// Build a `KeywordView` by parsing a slice as strict keyword-value pairs.
    ///
    /// Every atom must be a keyword (`:name`) followed by a value.
    pub fn from_slice(atoms: &'a [Atom]) -> Result<Self, SexError> {
        AtomView::new(atoms).into_keywords()
    }

    /// Returns `true` if the given keyword is present.
    pub fn contains_key(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }

    /// Look up a keyword's value atom.
    pub fn get(&self, name: &str) -> Option<&'a Atom> {
        self.map.get(name).copied()
    }

    /// Look up and deserialize a required keyword.
    ///
    /// Returns an error if the keyword is missing or the value cannot be deserialized.
    pub fn required<T: FromSex>(&self, name: &str) -> Result<T, SexError> {
        match self.map.get(name) {
            Some(atom) => T::from_sex(atom),
            None => Err(SexError::MissingField {
                name: name.to_string(),
            }),
        }
    }

    /// Look up and deserialize an optional keyword.
    ///
    /// Returns `Ok(None)` if the keyword is absent. Also returns an error if the value
    /// cannot be deserialized.
    pub fn optional<T: FromSex>(&self, name: &str) -> Result<Option<T>, SexError> {
        match self.map.get(name) {
            Some(atom) => T::from_sex(atom).map(Some),
            None => Ok(None),
        }
    }

    /// Number of keyword-value pairs.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Whether there are zero pairs.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Iterate over all `(keyword_name, value_atom)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&'a str, &'a Atom)> + '_ {
        self.map.iter().map(|(k, v)| (*k, *v))
    }
}
