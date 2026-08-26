use sex::{Atom, IntoSex, List, Number};

#[test]
fn into_sex_atom_identity() {
    let a = Atom::symbol("hello");
    assert_eq!(a.clone().into_sex(), a);
}

#[test]
fn into_sex_i64() {
    assert_eq!(42i64.into_sex(), Atom::Number(Number::Integer(42)));
    assert_eq!((-7i64).into_sex(), Atom::Number(Number::Integer(-7)));
}

#[test]
fn into_sex_i32() {
    assert_eq!(42i32.into_sex(), Atom::Number(Number::Integer(42)));
}

#[test]
fn into_sex_u8() {
    assert_eq!(255u8.into_sex(), Atom::Number(Number::Integer(255)));
}

#[test]
fn into_sex_usize() {
    assert_eq!(100usize.into_sex(), Atom::Number(Number::Integer(100)));
}

#[test]
fn into_sex_f64() {
    assert_eq!(2.5f64.into_sex(), Atom::Number(Number::Float(2.5)));
}

#[test]
fn into_sex_f32() {
    assert_eq!(2.5f32.into_sex(), Atom::Number(Number::Float(2.5)));
}

#[test]
fn into_sex_bool() {
    assert_eq!(true.into_sex(), Atom::True);
    assert_eq!(false.into_sex(), Atom::False);
}

#[test]
fn into_sex_unit() {
    assert_eq!(().into_sex(), Atom::Nil);
}

#[test]
fn into_sex_string() {
    assert_eq!("hello".into_sex(), Atom::string("hello"));
}

#[test]
fn into_sex_string_owned() {
    let s = String::from("world");
    assert_eq!(s.into_sex(), Atom::string("world"));
}

#[test]
fn into_sex_option_some() {
    let v: Option<i64> = Some(42);
    assert_eq!(v.into_sex(), Atom::Number(Number::Integer(42)));
}

#[test]
fn into_sex_option_none() {
    let v: Option<i64> = None;
    assert_eq!(v.into_sex(), Atom::Nil);
}

#[test]
fn into_sex_vec() {
    let v = vec![1i64, 2, 3];
    assert_eq!(
        v.into_sex(),
        Atom::List(List::from(vec![
            Atom::Number(Number::Integer(1)),
            Atom::Number(Number::Integer(2)),
            Atom::Number(Number::Integer(3)),
        ]))
    );
}

#[test]
fn into_sex_vec_empty() {
    let v: Vec<i64> = vec![];
    assert_eq!(v.into_sex(), Atom::List(List::from(vec![])));
}

#[test]
fn into_sex_list() {
    let list = List::from(vec![Atom::True, Atom::Nil]);
    assert_eq!(
        list.into_sex(),
        Atom::List(List::from(vec![Atom::True, Atom::Nil]))
    );
}
