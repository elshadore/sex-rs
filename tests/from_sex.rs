use sex::{Atom, FromSex, Number, Text, TextTy};

// -----------------------------------------------------------------------
// FromSex – String (accepts any text variant)
// -----------------------------------------------------------------------

#[test]
fn from_sex_string_from_symbol() {
    assert_eq!(String::from_sex(&Atom::symbol("hello")).unwrap(), "hello");
}

#[test]
fn from_sex_string_from_keyword() {
    assert_eq!(String::from_sex(&Atom::keyword("k")).unwrap(), "k");
}

#[test]
fn from_sex_string_from_string_literal() {
    assert_eq!(String::from_sex(&Atom::string("raw")).unwrap(), "raw");
}

#[test]
fn from_sex_string_err_on_number() {
    assert!(String::from_sex(&Atom::Number(Number::Integer(5))).is_err());
}

#[test]
fn from_sex_string_err_on_list() {
    assert!(String::from_sex(&Atom::List(vec![])).is_err());
}

#[test]
fn from_sex_string_err_on_nil() {
    assert!(String::from_sex(&Atom::Nil).is_err());
}

// -----------------------------------------------------------------------
// FromSex – i32
// -----------------------------------------------------------------------

#[test]
fn from_sex_i32_ok() {
    assert_eq!(i32::from_sex(&Atom::Number(Number::Integer(42))).unwrap(), 42);
}

#[test]
fn from_sex_i32_negative() {
    assert_eq!(i32::from_sex(&Atom::Number(Number::Integer(-7))).unwrap(), -7);
}

#[test]
fn from_sex_i32_err_on_float() {
    assert!(i32::from_sex(&Atom::Number(Number::Float(3.0))).is_err());
}

#[test]
fn from_sex_i32_err_on_text() {
    assert!(i32::from_sex(&Atom::symbol("x")).is_err());
}

// -----------------------------------------------------------------------
// FromSex – f32
// -----------------------------------------------------------------------

#[test]
fn from_sex_f32_from_float() {
    assert_eq!(f32::from_sex(&Atom::Number(Number::Float(2.5))).unwrap(), 2.5);
}

#[test]
fn from_sex_f32_from_integer() {
    assert_eq!(f32::from_sex(&Atom::Number(Number::Integer(7))).unwrap(), 7.0);
}

#[test]
fn from_sex_f32_err_on_text() {
    assert!(f32::from_sex(&Atom::symbol("x")).is_err());
}

// -----------------------------------------------------------------------
// FromSex – bool
// -----------------------------------------------------------------------

#[test]
fn from_sex_bool_true() {
    assert!(bool::from_sex(&Atom::symbol("true")).unwrap());
}

#[test]
fn from_sex_bool_false() {
    assert!(!bool::from_sex(&Atom::symbol("false")).unwrap());
}

#[test]
fn from_sex_bool_nil_is_false() {
    assert!(!bool::from_sex(&Atom::Nil).unwrap());
}

#[test]
fn from_sex_bool_from_true_atom() {
    assert!(bool::from_sex(&Atom::True).unwrap());
}

#[test]
fn from_sex_bool_err_on_keyword() {
    assert!(bool::from_sex(&Atom::keyword("true")).is_err());
}

#[test]
fn from_sex_bool_err_on_string() {
    assert!(bool::from_sex(&Atom::string("true")).is_err());
}

#[test]
fn from_sex_bool_err_on_other_symbol() {
    assert!(bool::from_sex(&Atom::symbol("yes")).is_err());
}

#[test]
fn from_sex_bool_err_on_number() {
    assert!(bool::from_sex(&Atom::Number(Number::Integer(1))).is_err());
}

// -----------------------------------------------------------------------
// FromSex – ()
// -----------------------------------------------------------------------

#[test]
fn from_sex_unit_from_nil() {
    <()>::from_sex(&Atom::Nil).unwrap();
}

#[test]
fn from_sex_unit_err_on_text() {
    assert!(<()>::from_sex(&Atom::symbol("x")).is_err());
}

// -----------------------------------------------------------------------
// FromSex – Option<T>
// -----------------------------------------------------------------------

#[test]
fn from_sex_option_none() {
    let r: Option<i32> = Option::from_sex(&Atom::Nil).unwrap();
    assert_eq!(r, None);
}

#[test]
fn from_sex_option_some() {
    let r: Option<i32> = Option::from_sex(&Atom::Number(Number::Integer(99))).unwrap();
    assert_eq!(r, Some(99));
}

#[test]
fn from_sex_option_inside_list() {
    let list = Atom::List(vec![Atom::Number(Number::Integer(1)), Atom::Nil]);
    let r: Vec<Option<i32>> = Vec::from_sex(&list).unwrap();
    assert_eq!(r, vec![Some(1), None]);
}

// -----------------------------------------------------------------------
// FromSex – Vec<T>
// -----------------------------------------------------------------------

#[test]
fn from_sex_vec_empty() {
    let r: Vec<i32> = Vec::from_sex(&Atom::List(vec![])).unwrap();
    assert!(r.is_empty());
}

#[test]
fn from_sex_vec_integers() {
    let list = Atom::List(vec![
        Atom::Number(Number::Integer(10)),
        Atom::Number(Number::Integer(20)),
        Atom::Number(Number::Integer(30)),
    ]);
    let r: Vec<i32> = Vec::from_sex(&list).unwrap();
    assert_eq!(r, vec![10, 20, 30]);
}

#[test]
fn from_sex_vec_err_on_non_list() {
    let r: Result<Vec<i32>, _> = Vec::from_sex(&Atom::symbol("x"));
    assert!(r.is_err());
}

#[test]
fn from_sex_vec_type_error_inside() {
    let list = Atom::List(vec![
        Atom::Number(Number::Integer(1)),
        Atom::symbol("bad"),
    ]);
    let r: Result<Vec<i32>, _> = Vec::from_sex(&list);
    assert!(r.is_err());
}

// -----------------------------------------------------------------------
// Text struct
// -----------------------------------------------------------------------

#[test]
fn text_struct_fields_accessible() {
    let t = Text { ty: TextTy::Symbol, contents: "hello".into() };
    assert_eq!(t.ty, TextTy::Symbol);
    assert_eq!(t.contents, "hello");
}

#[test]
fn text_equality() {
    let a = Text { ty: TextTy::Symbol, contents: "x".into() };
    let b = Text { ty: TextTy::Symbol, contents: "x".into() };
    let c = Text { ty: TextTy::Keyword, contents: "x".into() };
    assert_eq!(a, b);
    assert_ne!(a, c);
}
