use sex::{Atom, IntoSex, List, Number};

#[test]
fn into_atom_i64() {
    assert_eq!(42i64.into_atom(), Atom::Number(Number::Integer(42)));
    assert_eq!((-7i64).into_atom(), Atom::Number(Number::Integer(-7)));
}

#[test]
fn into_atom_i32() {
    assert_eq!(42i32.into_atom(), Atom::Number(Number::Integer(42)));
}

#[test]
fn into_atom_u8() {
    assert_eq!(255u8.into_atom(), Atom::Number(Number::Integer(255)));
}

#[test]
fn into_atom_usize() {
    assert_eq!(100usize.into_atom(), Atom::Number(Number::Integer(100)));
}

#[test]
fn into_atom_f64() {
    assert_eq!(2.5f64.into_atom(), Atom::Number(Number::Float(2.5)));
}

#[test]
fn into_atom_f32() {
    assert_eq!(2.5f32.into_atom(), Atom::Number(Number::Float(2.5)));
}

#[test]
fn into_atom_bool() {
    assert_eq!(true.into_atom(), Atom::True);
    assert_eq!(false.into_atom(), Atom::False);
}

#[test]
fn into_atom_unit() {
    assert_eq!(().into_atom(), Atom::Nil);
}

#[test]
fn into_atom_string() {
    assert_eq!("hello".into_atom(), Atom::string("hello"));
}

#[test]
fn into_atom_string_owned() {
    let s = String::from("world");
    assert_eq!(s.into_atom(), Atom::string("world"));
}

#[test]
fn into_atom_option_some() {
    let v: Option<i64> = Some(42);
    assert_eq!(v.into_atom(), Atom::Number(Number::Integer(42)));
}

#[test]
fn into_atom_option_none() {
    let v: Option<i64> = None;
    assert_eq!(v.into_atom(), Atom::Nil);
}

#[test]
fn into_atom_vec() {
    let v = vec![1i64, 2, 3];
    assert_eq!(
        v.into_atom(),
        Atom::List(List::from(vec![
            Atom::Number(Number::Integer(1)),
            Atom::Number(Number::Integer(2)),
            Atom::Number(Number::Integer(3)),
        ]))
    );
}

#[test]
fn into_atom_vec_empty() {
    let v: Vec<i64> = vec![];
    assert_eq!(v.into_atom(), Atom::List(List::from(vec![])));
}

#[test]
fn into_atom_atom_identity() {
    let a = Atom::symbol("hello");
    assert_eq!(a.clone().into_atom(), a);
}

#[test]
fn into_atom_list() {
    let list = List::from(vec![Atom::True, Atom::Nil]);
    assert_eq!(
        list.into_atom(),
        Atom::List(List::from(vec![Atom::True, Atom::Nil]))
    );
}

