use std::fmt::Debug;

use sex::{Atom, FromSex, IntoSex, Number, parse_expression_str};

#[derive(Debug, PartialEq, FromSex, IntoSex)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, FromSex, IntoSex)]
struct Config {
    name: String,
    #[sex(keyword)]
    width: i64,
    #[sex(keyword, default = 100)]
    height: i64,
}

#[derive(Debug, PartialEq, FromSex, IntoSex)]
struct OptionalFields {
    name: String,
    #[sex(keyword, default)]
    label: Option<String>,
}

#[derive(Debug, PartialEq, FromSex, IntoSex)]
struct Outer {
    a: i64,
    p: Point,
    c: i64,
}

#[derive(Debug, PartialEq, FromSex, IntoSex)]
enum Shape {
    #[sex(tag = "circle")]
    Circle(i64),

    #[sex(tag = "rect")]
    Rect {
        width: i64,
        height: i64,
        #[sex(keyword, default = 0)]
        x: i64,
        #[sex(keyword, default = 0)]
        y: i64,
    },

    #[sex(tag = "point")]
    Pt(Point),
}

#[derive(Debug, PartialEq, FromSex, IntoSex)]
enum Command {
    #[sex(tag = "noop")]
    Noop,

    #[sex(tag = "move")]
    Move {
        #[sex(keyword = "dx")]
        dx: i64,
        #[sex(keyword = "dy")]
        dy: i64,
    },

    #[sex(tag = "jump")]
    Jump(i64, i64),
}

#[test]
fn struct_positional() {
    let p = Point { x: 10, y: 20 };
    assert_eq!(p.into_atom().to_string(), "(10 20)");
}

#[test]
fn struct_keyword() {
    let c = Config {
        name: "test".into(),
        width: 800,
        height: 100,
    };
    assert_eq!(c.into_atom().to_string(), "(\"test\" :width 800 :height 100)");
}

#[test]
fn struct_keyword_default_still_serialized() {
    let c = Config {
        name: "test".into(),
        width: 800,
        height: 200,
    };
    assert_eq!(c.into_atom().to_string(), "(\"test\" :width 800 :height 200)");
}

#[test]
fn struct_optional_keyword_none() {
    let o = OptionalFields {
        name: "hello".into(),
        label: None,
    };
    assert_eq!(o.into_atom().to_string(), "(\"hello\" :label nil)");
}

#[test]
fn struct_optional_keyword_some() {
    let o = OptionalFields {
        name: "hello".into(),
        label: Some("world".into()),
    };
    assert_eq!(o.into_atom().to_string(), "(\"hello\" :label \"world\")");
}

#[test]
fn enum_tuple_single() {
    let s = Shape::Circle(5);
    assert_eq!(s.into_atom().to_string(), "(circle 5)");
}

#[test]
fn enum_tuple_complex() {
    let s = Shape::Pt(Point { x: 1, y: 2 });
    assert_eq!(s.into_atom().to_string(), "(point 1 2)");
}

#[test]
fn enum_named() {
    let s = Shape::Rect {
        width: 100,
        height: 200,
        x: 10,
        y: 20,
    };
    assert_eq!(s.into_atom().to_string(), "(rect 100 200 :x 10 :y 20)");
}

#[test]
fn enum_unit() {
    let c = Command::Noop;
    assert_eq!(c.into_atom().to_string(), "(noop)");
}

#[test]
fn enum_named_keyword_out_of_order_atoms() {
    let c = Command::Move { dx: 1, dy: 2 };
    let atom = c.into_atom();
    assert_eq!(atom.to_string(), "(move :dx 1 :dy 2)");
}

#[test]
fn enum_tuple_multiple() {
    let c = Command::Jump(3, 4);
    assert_eq!(c.into_atom().to_string(), "(jump 3 4)");
}

#[test]
fn struct_nested_field() {
    let o = Outer {
        a: 1,
        p: Point { x: 2, y: 3 },
        c: 4,
    };
    assert_eq!(o.into_atom().to_string(), "(1 (2 3) 4)");
}

fn roundtrip<T: FromSex + IntoSex + PartialEq + Debug>(value: T, input: &str) {
    let atom = parse_expression_str(input, None).unwrap();
    let parsed: T = T::from_atom(&atom).unwrap();
    assert_eq!(parsed, value);
    assert_eq!(parsed.into_atom().to_string(), input);
}

#[test]
fn roundtrip_struct() {
    roundtrip(
        Config {
            name: "test".into(),
            width: 800,
            height: 100,
        },
        "(\"test\" :width 800 :height 100)",
    );
}

#[test]
fn roundtrip_optional_none() {
    roundtrip(
        OptionalFields {
            name: "hello".into(),
            label: None,
        },
        "(\"hello\" :label nil)",
    );
}

#[test]
fn roundtrip_enum_named() {
    roundtrip(
        Shape::Rect {
            width: 100,
            height: 200,
            x: 10,
            y: 20,
        },
        "(rect 100 200 :x 10 :y 20)",
    );
}

#[test]
fn roundtrip_enum_unit() {
    roundtrip(Command::Noop, "(noop)");
}

#[test]
fn roundtrip_enum_tuple_payload() {
    roundtrip(Shape::Pt(Point { x: 1, y: 2 }), "(point 1 2)");
}

#[test]
fn roundtrip_struct_nested() {
    roundtrip(
        Outer {
            a: 1,
            p: Point { x: 2, y: 3 },
            c: 4,
        },
        "(1 (2 3) 4)",
    );
}

#[test]
fn into_atom_values() {
    let c = Config {
        name: "test".into(),
        width: 800,
        height: 100,
    };
    let atom = c.into_atom();
    let list = atom.as_list().unwrap();
    assert_eq!(list.get(0), Some(&Atom::string("test")));
    assert_eq!(list.get(2), Some(&Atom::Number(Number::Integer(800))));
}