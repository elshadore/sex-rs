use sex::{Atom, Number, Position};

// -----------------------------------------------------------------------
// Atom accessors
// -----------------------------------------------------------------------

#[test]
fn nil_is_nil() {
    let a = Atom::Nil;
    assert!(a.is_nil());
    assert!(!a.is_symbol());
    assert!(!a.is_keyword());
    assert!(!a.is_text());
    assert!(!a.is_number());
    assert!(!a.is_list());
}

#[test]
fn symbol_accessors() {
    let a = Atom::symbol("hello");
    assert!(a.is_symbol());
    assert!(a.is_text());
    assert_eq!(a.as_symbol(), Some("hello"));
    assert_eq!(a.as_text().unwrap().contents, "hello");
    assert!(!a.is_nil());
    assert!(!a.is_keyword());
    assert!(!a.is_number());
    assert!(!a.is_list());
    assert!(a.as_keyword().is_none());
    assert!(a.as_integer().is_none());
    assert!(a.as_float().is_none());
    assert!(a.as_list().is_none());
}

#[test]
fn keyword_accessors() {
    let a = Atom::keyword("foo");
    assert!(a.is_keyword());
    assert!(a.is_text());
    assert_eq!(a.as_keyword(), Some("foo"));
    assert_eq!(a.as_text().unwrap().contents, "foo");
    assert!(!a.is_symbol());
    assert!(a.as_symbol().is_none());
}

#[test]
fn string_text_accessors() {
    let a = Atom::string("hello");
    assert!(a.is_text());
    assert_eq!(a.as_text().unwrap().contents, "hello");
    assert!(!a.is_symbol());
    assert!(!a.is_keyword());
    assert!(a.as_symbol().is_none());
    assert!(a.as_keyword().is_none());
}

#[test]
fn integer_accessors() {
    let a = Atom::Number(Number::Integer(42));
    assert!(a.is_number());
    assert!(a.is_integer());
    assert!(!a.is_float());
    assert_eq!(a.as_integer(), Some(42));
    assert!(a.as_float().is_none());
}

#[test]
fn float_accessors() {
    let a = Atom::Number(Number::Float(3.14));
    assert!(a.is_number());
    assert!(a.is_float());
    assert!(!a.is_integer());
    assert_eq!(a.as_float(), Some(3.14));
    assert!(a.as_integer().is_none());
}

#[test]
fn list_accessors() {
    let inner = vec![Atom::symbol("a")];
    let a = Atom::List(inner.clone());
    assert!(a.is_list());
    assert_eq!(a.as_list(), Some(&inner));
}

#[test]
fn list_is_empty_for_empty_list() {
    let a = Atom::List(vec![]);
    assert!(a.is_list());
    assert_eq!(a.as_list().map(|l| l.len()), Some(0));
}

// -----------------------------------------------------------------------
// Atom – True variant
// -----------------------------------------------------------------------

#[test]
fn is_true_returns_true_for_true() {
    assert!(Atom::True.is_true());
}

#[test]
fn is_true_returns_false_for_nil() {
    assert!(!Atom::Nil.is_true());
}

// -----------------------------------------------------------------------
// TextTy
// -----------------------------------------------------------------------

#[test]
fn text_ty_constructors() {
    let s = Atom::symbol("x");
    let k = Atom::keyword("x");
    let t = Atom::string("x");

    assert!(s.is_symbol());
    assert!(k.is_keyword());
    assert!(t.is_text());
    assert!(!t.is_symbol());
    assert!(!t.is_keyword());
}

// -----------------------------------------------------------------------
// Position Display
// -----------------------------------------------------------------------

#[test]
fn position_display() {
    let p = Position { line: 3, col: 14 };
    assert_eq!(p.to_string(), "3:14");
}

#[test]
fn position_display_zero() {
    let p = Position { line: 0, col: 0 };
    assert_eq!(p.to_string(), "0:0");
}
