use sex::{Atom, AtomTy, KeywordView, Number, SexError, parse_atom};

#[test]
fn keyword_view_barred_keyword_key() {
    let atom = parse_atom("(name :|my key| 5)").unwrap();
    let list = atom.try_as_list().unwrap();
    assert_eq!(list[0], Atom::symbol("name"));
    let kv = KeywordView::from_slice(&list[1..]).unwrap();
    assert!(kv.contains_key("my key"));
    assert_eq!(kv.required::<i64>("my key").unwrap(), 5);
}

#[test]
fn keyword_view_empty() {
    let kv = KeywordView::from_slice(&[]).unwrap();
    assert!(kv.is_empty());
    assert_eq!(kv.len(), 0);
}

#[test]
fn keyword_view_single_pair() {
    let atoms = [Atom::keyword("x"), Atom::Number(Number::Integer(42))];
    let kv = KeywordView::from_slice(&atoms).unwrap();
    assert_eq!(kv.len(), 1);
    assert!(kv.contains_key("x"));
    assert_eq!(kv.get("x"), Some(&Atom::Number(Number::Integer(42))));
}

#[test]
fn keyword_view_multiple_pairs() {
    let atoms = [
        Atom::keyword("width"),
        Atom::Number(Number::Integer(800)),
        Atom::keyword("height"),
        Atom::Number(Number::Integer(600)),
        Atom::keyword("title"),
        Atom::string("hello"),
    ];
    let kv = KeywordView::from_slice(&atoms).unwrap();
    assert_eq!(kv.len(), 3);
    assert_eq!(kv.required::<i64>("width").unwrap(), 800);
    assert_eq!(kv.required::<i64>("height").unwrap(), 600);
    assert_eq!(kv.required::<String>("title").unwrap(), "hello");
}

#[test]
fn keyword_view_from_atom_view() {
    let atoms = [
        Atom::keyword("a"),
        Atom::Number(Number::Integer(1)),
        Atom::keyword("b"),
        Atom::Number(Number::Integer(2)),
    ];
    let view = sex::AtomView::new(&atoms);
    let kv = view.into_keywords().unwrap();
    assert_eq!(kv.required::<i64>("a").unwrap(), 1);
    assert_eq!(kv.required::<i64>("b").unwrap(), 2);
}


#[test]
fn keyword_view_required_ok() {
    let atoms = [Atom::keyword("x"), Atom::Number(Number::Integer(99))];
    let kv = KeywordView::from_slice(&atoms).unwrap();
    assert_eq!(kv.required::<i64>("x").unwrap(), 99);
}

#[test]
fn keyword_view_required_missing() {
    let kv = KeywordView::from_slice(&[]).unwrap();
    let err = kv.required::<i64>("x").unwrap_err();
    assert!(matches!(err, SexError::MissingField { .. }));
}

#[test]
fn keyword_view_optional_present() {
    let atoms = [Atom::keyword("x"), Atom::Number(Number::Integer(99))];
    let kv = KeywordView::from_slice(&atoms).unwrap();
    assert_eq!(kv.optional::<i64>("x").unwrap(), Some(99));
}

#[test]
fn keyword_view_optional_absent() {
    let kv = KeywordView::from_slice(&[]).unwrap();
    assert_eq!(kv.optional::<i64>("x").unwrap(), None);
}

#[test]
fn keyword_view_required_type_error() {
    let atoms = [Atom::keyword("x"), Atom::symbol("hello")];
    let kv = KeywordView::from_slice(&atoms).unwrap();
    let err = kv.required::<i64>("x").unwrap_err();
    assert!(matches!(err, SexError::TypeError { .. }));
}


#[test]
fn keyword_view_errors_on_non_keyword() {
    let atoms = [Atom::symbol("foo")];
    let err = KeywordView::from_slice(&atoms).unwrap_err();
    assert!(matches!(
        err,
        SexError::TypeError {
            expected: AtomTy::Keyword,
            ..
        }
    ));
}

#[test]
fn keyword_view_errors_on_positional_between_keywords() {
    let atoms = [
        Atom::keyword("x"),
        Atom::Number(Number::Integer(1)),
        Atom::symbol("bad"),
        Atom::keyword("y"),
        Atom::Number(Number::Integer(2)),
    ];
    let err = KeywordView::from_slice(&atoms).unwrap_err();
    assert!(matches!(
        err,
        SexError::TypeError {
            expected: AtomTy::Keyword,
            ..
        }
    ));
}

#[test]
fn keyword_view_errors_on_keyword_without_value() {
    let atoms = [Atom::keyword("x")];
    let err = KeywordView::from_slice(&atoms).unwrap_err();
    assert!(matches!(err, SexError::ExpectedAtom));
}

#[test]
fn keyword_view_errors_on_keyword_without_value_among_others() {
    let atoms = [
        Atom::keyword("x"),
        Atom::Number(Number::Integer(1)),
        Atom::keyword("y"),
    ];
    let err = KeywordView::from_slice(&atoms).unwrap_err();
    assert!(matches!(err, SexError::ExpectedAtom));
}


#[test]
fn keyword_view_iter() {
    let atoms = [
        Atom::keyword("a"),
        Atom::Number(Number::Integer(1)),
        Atom::keyword("b"),
        Atom::Number(Number::Integer(2)),
    ];
    let kv = KeywordView::from_slice(&atoms).unwrap();
    let pairs: Vec<(&str, &Atom)> = kv.iter().collect();
    assert_eq!(pairs.len(), 2);
}


#[test]
fn keyword_view_from_parsed_list() {
    let atom = sex::parse_atom("(:x 10 :y 20)").unwrap();
    let list = atom.as_list().unwrap();
    let kv = KeywordView::from_slice(list).unwrap();
    assert_eq!(kv.required::<i64>("x").unwrap(), 10);
    assert_eq!(kv.required::<i64>("y").unwrap(), 20);
}
