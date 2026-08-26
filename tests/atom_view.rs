use sex::{Atom, AtomView, Number, SexError};

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
    assert_eq!(v.peek(), None);
    assert_eq!(v.pop(), Some(&Atom::symbol("hello")));
    assert!(v.is_finished());
    assert_eq!(v.pop(), None);
}

#[test]
fn multiple_atoms() {
    let atoms = [Atom::symbol("a"), Atom::symbol("b"), Atom::symbol("c")];
    let mut v = AtomView::new(&atoms);
    assert_eq!(v.remaining(), 3);
    assert_eq!(v.pop(), Some(&Atom::symbol("a")));
    assert_eq!(v.remaining(), 2);
    assert_eq!(v.pop(), Some(&Atom::symbol("b")));
    assert_eq!(v.remaining(), 1);
    assert_eq!(v.pop(), Some(&Atom::symbol("c")));
    assert_eq!(v.remaining(), 0);
    assert_eq!(v.pop(), None);
}


#[test]
fn at_does_not_advance() {
    let atoms = [Atom::symbol("a"), Atom::symbol("b")];
    let v = AtomView::new(&atoms);
    assert_eq!(v.at(), Some(&Atom::symbol("a")));
    assert_eq!(v.at(), Some(&Atom::symbol("a")));
    assert_eq!(v.remaining(), 2);
}

#[test]
fn peek_does_not_advance() {
    let atoms = [Atom::symbol("first"), Atom::symbol("second")];
    let v = AtomView::new(&atoms);
    assert_eq!(v.peek(), Some(&Atom::symbol("second")));
    assert_eq!(v.peek(), Some(&Atom::symbol("second")));
    assert_eq!(v.remaining(), 2);
}


#[test]
fn skip_partial() {
    let atoms = [Atom::symbol("a"), Atom::symbol("b"), Atom::symbol("c")];
    let mut v = AtomView::new(&atoms);
    v.skip_n(2);
    assert_eq!(v.pop(), Some(&Atom::symbol("c")));
}

#[test]
fn skip_past_end() {
    let atoms = [Atom::symbol("a")];
    let mut v = AtomView::new(&atoms);
    v.skip_n(10);
    assert!(v.is_finished());
    assert_eq!(v.pop(), None);
}

#[test]
fn skip_zero() {
    let atoms = [Atom::symbol("a")];
    let mut v = AtomView::new(&atoms);
    v.skip_n(0);
    assert_eq!(v.pop(), Some(&Atom::symbol("a")));
}

#[test]
fn remaining_slice_after_consumption() {
    let atoms = [Atom::symbol("a"), Atom::symbol("b"), Atom::symbol("c")];
    let mut v = AtomView::new(&atoms);
    v.pop();
    assert_eq!(v.remaining_slice(), &[Atom::symbol("b"), Atom::symbol("c")]);
}

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
    assert_eq!(inner.pop(), Some(&Atom::symbol("a")));
    assert_eq!(inner.pop(), Some(&Atom::symbol("b")));
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
    assert!(matches!(err, SexError::ExpectedAtom));
}


#[test]
fn parse_then_view() {
    let atoms = sex::parse_exprlist_str("(defexample foo :src \"bar.sex\")", None).unwrap();
    let mut v = AtomView::new(&atoms);
    let mut list = v.enter_list().unwrap();
    assert_eq!(list.pop().unwrap().as_text().unwrap().contents, "defexample");
    assert_eq!(list.pop().unwrap().as_text().unwrap().contents, "foo");
    let kw = list.into_keywords().unwrap();
    let src = kw.get("src").unwrap();
    assert_eq!(src, &Atom::string("bar.sex"));
}

#[test]
fn parse_multiple_toplevel_forms() {
    let atoms = sex::parse_exprlist_str("(a 1) (b 2)", None).unwrap();
    let mut v = AtomView::new(&atoms);

    let mut first = v.enter_list().unwrap();
    assert_eq!(first.pop().unwrap().as_text().unwrap().contents, "a");
    assert_eq!(first.pop().unwrap().as_integer().unwrap(), 1);

    let mut second = v.enter_list().unwrap();
    assert_eq!(second.pop().unwrap().as_text().unwrap().contents, "b");
    assert_eq!(second.pop().unwrap().as_integer().unwrap(), 2);

    assert!(v.is_finished());
}

#[test]
fn view_remaining_after_partial_consumption() {
    let atoms = sex::parse_exprlist_str("foo bar baz", None).unwrap();
    let mut v = AtomView::new(&atoms);
    v.pop();
    assert_eq!(v.remaining(), 2);
    v.pop();
    assert_eq!(v.remaining(), 1);
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


#[test]
fn try_at_returns_atom() {
    let atoms = [Atom::symbol("hello")];
    let mut v = AtomView::new(&atoms);
    assert_eq!(v.try_at().unwrap(), &Atom::symbol("hello"));
}

#[test]
fn try_at_at_end_errors() {
    let empty: &[Atom] = &[];
    let mut v = AtomView::new(empty);
    let err = v.try_at().unwrap_err();
    assert!(matches!(err, SexError::ExpectedAtom));
}

#[test]
fn try_pop_returns_atom() {
    let atoms = [Atom::symbol("hello")];
    let mut v = AtomView::new(&atoms);
    assert_eq!(v.try_pop().unwrap(), &Atom::symbol("hello"));
    assert!(v.is_finished());
}

#[test]
fn try_pop_at_end_errors() {
    let empty: &[Atom] = &[];
    let mut v = AtomView::new(empty);
    let err = v.try_pop().unwrap_err();
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
