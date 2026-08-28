use sex::{List, Atom, AtomTy, FromSex, ListView, Number, SexError};

fn view_from(atoms: &[Atom]) -> ListView<'_> {
    ListView::new_slice(atoms)
}


#[test]
fn from_sex_string_from_symbol() {
    assert_eq!(String::from_atom(&Atom::symbol("hello")).unwrap(), "hello");
}

#[test]
fn from_sex_string_from_keyword() {
    assert_eq!(String::from_atom(&Atom::keyword("k")).unwrap(), "k");
}

#[test]
fn from_sex_string_from_string_literal() {
    assert_eq!(String::from_atom(&Atom::string("raw")).unwrap(), "raw");
}

#[test]
fn from_sex_string_err_on_number() {
    assert!(String::from_atom(&Atom::Number(Number::Integer(5))).is_err());
}

#[test]
fn from_sex_string_err_on_list() {
    assert!(String::from_atom(&Atom::List(List::from(vec![]))).is_err());
}

#[test]
fn from_sex_string_err_on_nil() {
    assert!(String::from_atom(&Atom::Nil).is_err());
}


#[test]
fn from_sex_i64_ok() {
    assert_eq!(i64::from_atom(&Atom::Number(Number::Integer(42))).unwrap(), 42);
}

#[test]
fn from_sex_i64_negative() {
    assert_eq!(i64::from_atom(&Atom::Number(Number::Integer(-7))).unwrap(), -7);
}

#[test]
fn from_sex_i64_err_on_float() {
    assert!(i64::from_atom(&Atom::Number(Number::Float(3.0))).is_err());
}

#[test]
fn from_sex_i64_err_on_text() {
    assert!(i64::from_atom(&Atom::symbol("x")).is_err());
}

#[test]
fn from_sex_i32_ok() {
    assert_eq!(i32::from_atom(&Atom::Number(Number::Integer(42))).unwrap(), 42);
}

#[test]
fn from_sex_u8_ok() {
    assert_eq!(u8::from_atom(&Atom::Number(Number::Integer(255))).unwrap(), 255);
}

#[test]
fn from_sex_i8_range() {
    assert_eq!(i8::from_atom(&Atom::Number(Number::Integer(127))).unwrap(), 127);
    assert_eq!(i8::from_atom(&Atom::Number(Number::Integer(-128))).unwrap(), -128);
}

#[test]
fn from_sex_int_overflow() {
    let err = u8::from_atom(&Atom::Number(Number::Integer(256))).unwrap_err();
    assert!(matches!(err, SexError::IntegerOverflow { .. }));
}

#[test]
fn from_sex_int_underflow() {
    let err = u32::from_atom(&Atom::Number(Number::Integer(-1))).unwrap_err();
    assert!(matches!(err, SexError::IntegerOverflow { .. }));
}


#[test]
fn from_sex_f64_from_float() {
    assert_eq!(f64::from_atom(&Atom::Number(Number::Float(2.5))).unwrap(), 2.5);
}

#[test]
fn from_sex_f64_from_integer() {
    assert_eq!(f64::from_atom(&Atom::Number(Number::Integer(7))).unwrap(), 7.0);
}

#[test]
fn from_sex_f32_from_float() {
    assert_eq!(f32::from_atom(&Atom::Number(Number::Float(2.5))).unwrap(), 2.5);
}

#[test]
fn from_sex_f32_from_integer() {
    assert_eq!(f32::from_atom(&Atom::Number(Number::Integer(7))).unwrap(), 7.0);
}

#[test]
fn from_sex_float_err_on_text() {
    assert!(f64::from_atom(&Atom::symbol("x")).is_err());
}

#[test]
fn from_sex_f32_overflow() {
    let err = f32::from_atom(&Atom::Number(Number::Float(1e39))).unwrap_err();
    assert!(matches!(err, SexError::FloatOverflow { .. }));
}


#[test]
fn from_sex_bool_logic_values() {
    assert!(bool::from_atom(&Atom::True).unwrap());
    assert!(!bool::from_atom(&Atom::False).unwrap());
    assert!(!bool::from_atom(&Atom::Nil).unwrap());
}

#[test]
fn from_sex_bool_errs_on_non_logic() {
    for atom in [
        Atom::symbol("true"),
        Atom::keyword("x"),
        Atom::string("x"),
        Atom::Number(Number::Integer(0)),
        Atom::List(List::from(vec![])),
    ] {
        let err = bool::from_atom(&atom).unwrap_err();
        match err {
            SexError::TypeError { expected, .. } => {
                assert_eq!(expected, AtomTy::Logic);
            }
            other => panic!("expected TypeError, got {other}"),
        }
    }
}


#[test]
fn from_sex_unit_from_nil() {
    <()>::from_atom(&Atom::Nil).unwrap();
}

#[test]
fn from_sex_unit_err_on_text() {
    assert!(<()>::from_atom(&Atom::symbol("x")).is_err());
}


#[test]
fn from_sex_option_none() {
    let mut view = view_from(&[Atom::Nil]);
    let r: Option<i64> = FromSex::from_list(&mut view).unwrap();
    assert_eq!(r, None);
}

#[test]
fn from_sex_option_some() {
    let mut view = view_from(&[Atom::Number(Number::Integer(99))]);
    let r: Option<i64> = FromSex::from_list(&mut view).unwrap();
    assert_eq!(r, Some(99));
}

#[test]
fn from_sex_option_inside_list() {
    let list = Atom::List(List::from(vec![Atom::Number(Number::Integer(1)), Atom::Nil]));
    let r: Vec<Option<i64>> = Vec::from_atom(&list).unwrap();
    assert_eq!(r, vec![Some(1), None]);
}


#[test]
fn from_sex_vec_empty() {
    let r: Vec<i64> = Vec::from_atom(&Atom::List(List::from(vec![]))).unwrap();
    assert!(r.is_empty());
}

#[test]
fn from_sex_vec_integers() {
    let list = Atom::List(List::from(vec![
        Atom::Number(Number::Integer(10)),
        Atom::Number(Number::Integer(20)),
        Atom::Number(Number::Integer(30)),
    ]));
    let r: Vec<i64> = Vec::from_atom(&list).unwrap();
    assert_eq!(r, vec![10, 20, 30]);
}

#[test]
fn from_sex_vec_err_on_non_list() {
    let r: Result<Vec<i64>, _> = Vec::from_atom(&Atom::symbol("x"));
    assert!(r.is_err());
}

#[test]
fn from_sex_vec_type_error_inside() {
    let list = Atom::List(List::from(vec![Atom::Number(Number::Integer(1)), Atom::symbol("bad")]));
    let r: Result<Vec<i64>, _> = Vec::from_atom(&list);
    assert!(r.is_err());
}


#[test]
fn from_sex_user_error_from_str() {
    let err = SexError::user("position must be positive");
    assert_eq!(err.to_string(), "position must be positive");
}

#[test]
fn from_sex_user_error_source() {
    use std::error::Error;

    let err = SexError::user("custom cause");
    assert!(err.source().is_some());
    assert_eq!(err.source().unwrap().to_string(), "custom cause");

    let err: SexError = SexError::ExpectedAtom;
    assert!(err.source().is_none());
}
