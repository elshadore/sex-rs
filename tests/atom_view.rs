use sex::{Atom, AtomView, Number, SexError};

// -----------------------------------------------------------------------
// AtomView – Construction and basic iteration
// -----------------------------------------------------------------------

#[test]
fn empty_view() {
    let v = AtomView::new(&[]);
    assert!(v.is_finished());
    assert_eq!(v.remaining(), 0);
    assert_eq!(v.peek(), None);
    assert_eq!(v.remaining_slice(), &[]);
}

#[test]
fn single_atom() {
    let atoms = [Atom::symbol("hello")];
    let mut v = AtomView::new(&atoms);
    assert!(!v.is_finished());
    assert_eq!(v.peek(), Some(&Atom::symbol("hello")));
    assert_eq!(v.next(), Some(&Atom::symbol("hello")));
    assert!(v.is_finished());
    assert_eq!(v.next(), None);
}

#[test]
fn multiple_atoms() {
    let atoms = [Atom::symbol("a"), Atom::symbol("b"), Atom::symbol("c")];
    let mut v = AtomView::new(&atoms);
    assert_eq!(v.remaining(), 3);
    assert_eq!(v.next(), Some(&Atom::symbol("a")));
    assert_eq!(v.remaining(), 2);
    assert_eq!(v.next(), Some(&Atom::symbol("b")));
    assert_eq!(v.remaining(), 1);
    assert_eq!(v.next(), Some(&Atom::symbol("c")));
    assert_eq!(v.remaining(), 0);
    assert_eq!(v.next(), None);
}

// -----------------------------------------------------------------------
// AtomView – skip
// -----------------------------------------------------------------------

#[test]
fn skip_partial() {
    let atoms = [Atom::symbol("a"), Atom::symbol("b"), Atom::symbol("c")];
    let mut v = AtomView::new(&atoms);
    v.skip(2);
    assert_eq!(v.next(), Some(&Atom::symbol("c")));
}

#[test]
fn skip_past_end() {
    let atoms = [Atom::symbol("a")];
    let mut v = AtomView::new(&atoms);
    v.skip(10);
    assert!(v.is_finished());
    assert_eq!(v.next(), None);
}

#[test]
fn skip_zero() {
    let atoms = [Atom::symbol("a")];
    let mut v = AtomView::new(&atoms);
    v.skip(0);
    assert_eq!(v.next(), Some(&Atom::symbol("a")));
}

// -----------------------------------------------------------------------
// AtomView – remaining_slice
// -----------------------------------------------------------------------

#[test]
fn remaining_slice_after_consumption() {
    let atoms = [Atom::symbol("a"), Atom::symbol("b"), Atom::symbol("c")];
    let mut v = AtomView::new(&atoms);
    v.next();
    assert_eq!(v.remaining_slice(), &[Atom::symbol("b"), Atom::symbol("c")]);
}

// -----------------------------------------------------------------------
// AtomView – enter_list
// -----------------------------------------------------------------------

#[test]
fn enter_list_empty() {
    let atoms = [Atom::List(vec![])];
    let mut v = AtomView::new(&atoms);
    let inner = v.enter_list().unwrap();
    assert!(inner.is_finished());
    assert!(v.is_finished());
}

#[test]
fn enter_list_with_elements() {
    let atoms = [Atom::List(vec![Atom::symbol("a"), Atom::symbol("b")])];
    let mut v = AtomView::new(&atoms);
    let mut inner = v.enter_list().unwrap();
    assert_eq!(inner.next(), Some(&Atom::symbol("a")));
    assert_eq!(inner.next(), Some(&Atom::symbol("b")));
    assert!(inner.is_finished());
    assert!(v.is_finished());
}

#[test]
fn enter_list_not_a_list() {
    let atoms = [Atom::symbol("foo")];
    let mut v = AtomView::new(&atoms);
    let err = v.enter_list().unwrap_err();
    assert!(matches!(err, SexError::TypeError { .. }));
}

#[test]
fn enter_list_eof() {
    let mut v = AtomView::new(&[]);
    let err = v.enter_list().unwrap_err();
    assert!(matches!(err, SexError::UnexpectedEof { .. }));
}

// -----------------------------------------------------------------------
// AtomView – Integration with parsing
// -----------------------------------------------------------------------

#[test]
fn parse_then_view() {
    let atoms = sex::parse("(defexample foo :src \"bar.sex\")").unwrap();
    let mut v = AtomView::new(&atoms);
    let mut list = v.enter_list().unwrap();
    assert_eq!(list.next().unwrap().as_text().unwrap().contents, "defexample");
    assert_eq!(list.next().unwrap().as_text().unwrap().contents, "foo");
    let kw = list.into_keywords().unwrap();
    let src = kw.get("src").unwrap();
    assert_eq!(src, &Atom::string("bar.sex"));
}

#[test]
fn parse_multiple_toplevel_forms() {
    let atoms = sex::parse("(a 1) (b 2)").unwrap();
    let mut v = AtomView::new(&atoms);

    let mut first = v.enter_list().unwrap();
    assert_eq!(first.next().unwrap().as_text().unwrap().contents, "a");
    assert_eq!(first.next().unwrap().as_integer().unwrap(), 1);

    let mut second = v.enter_list().unwrap();
    assert_eq!(second.next().unwrap().as_text().unwrap().contents, "b");
    assert_eq!(second.next().unwrap().as_integer().unwrap(), 2);

    assert!(v.is_finished());
}

#[test]
fn view_remaining_after_partial_consumption() {
    let atoms = sex::parse("foo bar baz").unwrap();
    let mut v = AtomView::new(&atoms);
    v.next();
    assert_eq!(v.remaining(), 2);
    v.next();
    assert_eq!(v.remaining(), 1);
}

#[test]
fn peek_does_not_advance() {
    let atoms = [Atom::symbol("first"), Atom::symbol("second")];
    let v = AtomView::new(&atoms);
    assert_eq!(v.peek(), Some(&Atom::symbol("first")));
    assert_eq!(v.peek(), Some(&Atom::symbol("first")));
    assert_eq!(v.remaining(), 2);
}

#[test]
fn keyword_then_value_pattern() {
    let atoms = [
        Atom::keyword("width"),
        Atom::Number(Number::Integer(800)),
        Atom::keyword("height"),
        Atom::Number(Number::Integer(600)),
    ];
    let v = AtomView::new(&atoms);
    let kw = v.into_keywords().unwrap();
    assert_eq!(kw.get("width"), Some(&Atom::Number(Number::Integer(800))));
    assert_eq!(kw.get("height"), Some(&Atom::Number(Number::Integer(600))));
}

// -----------------------------------------------------------------------
// AtomView – try_peek / try_next / expect_finished
// -----------------------------------------------------------------------

#[test]
fn try_peek_returns_atom() {
    let atoms = [Atom::symbol("hello")];
    let mut v = AtomView::new(&atoms);
    assert_eq!(v.try_peek().unwrap(), &Atom::symbol("hello"));
}

#[test]
fn try_peek_at_end_errors() {
    let empty: &[Atom] = &[];
    let mut v = AtomView::new(empty);
    let err = v.try_peek().unwrap_err();
    assert!(matches!(err, SexError::ExpectedAtom));
}

#[test]
fn try_next_returns_atom() {
    let atoms = [Atom::symbol("hello")];
    let mut v = AtomView::new(&atoms);
    assert_eq!(v.try_next().unwrap(), &Atom::symbol("hello"));
    assert!(v.is_finished());
}

#[test]
fn try_next_at_end_errors() {
    let empty: &[Atom] = &[];
    let mut v = AtomView::new(empty);
    let err = v.try_next().unwrap_err();
    assert!(matches!(err, SexError::ExpectedAtom));
}

#[test]
fn expect_finished_ok() {
    let empty: &[Atom] = &[];
    let v = AtomView::new(empty);
    v.expect_finished().unwrap();
}

#[test]
fn expect_finished_errors_on_remaining() {
    let atoms = [Atom::symbol("x")];
    let v = AtomView::new(&atoms);
    let err = v.expect_finished().unwrap_err();
    assert!(matches!(err, SexError::ExpectedFinished));
}

#[test]
fn expect_last_returns_atom() {
    let atoms = [Atom::symbol("x")];
    let mut v = AtomView::new(&atoms);
    assert_eq!(v.expect_last().unwrap(), &Atom::symbol("x"));
    assert!(v.is_finished());
}

#[test]
fn expect_last_errors_on_empty() {
    let empty: &[Atom] = &[];
    let mut v = AtomView::new(empty);
    let err = v.expect_last().unwrap_err();
    assert!(matches!(err, SexError::ExpectedAtom));
}

#[test]
fn expect_last_errors_on_more_remaining() {
    let atoms = [Atom::symbol("a"), Atom::symbol("b")];
    let mut v = AtomView::new(&atoms);
    let err = v.expect_last().unwrap_err();
    assert!(matches!(err, SexError::ExpectedFinished));
}
