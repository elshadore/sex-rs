use sex::{Atom, AtomTy, FromSex, Number, Sex, SexError, parse_atom};

#[derive(Debug, PartialEq, Sex)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Sex)]
struct Config {
    name: String,
    #[sex(keyword)]
    width: i64,
    #[sex(keyword, default = 100)]
    height: i64,
}

#[derive(Debug, PartialEq, Sex)]
struct OptionalFields {
    name: String,
    #[sex(keyword, default)]
    label: Option<String>,
}

#[derive(Debug, PartialEq, Sex)]
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

#[derive(Debug, PartialEq, Sex)]
enum Command {
    #[sex(tag = "noop")]
    Noop,

    #[sex(tag = "move")]
    Move {
        #[sex(keyword)]
        dx: i64,
        #[sex(keyword)]
        dy: i64,
    },

    #[sex(tag = "jump")]
    Jump(i64, i64),
}


#[test]
fn struct_positional() {
    let atom = parse_atom("(10 20)").unwrap();
    let p: Point = Point::from_sex(&atom).unwrap();
    assert_eq!(p, Point { x: 10, y: 20 });
}

#[test]
fn struct_positional_single() {
    let atom = parse_atom("(99)").unwrap();
    let err = Point::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::ExpectedAtom));
}


#[test]
fn struct_keyword() {
    let atom = parse_atom("(\"test\" :width 800)").unwrap();
    let c: Config = Config::from_sex(&atom).unwrap();
    assert_eq!(c.name, "test");
    assert_eq!(c.width, 800);
    assert_eq!(c.height, 100); 
}

#[test]
fn struct_keyword_default_used() {
    let atom = parse_atom("(\"test\" :width 800 :height 200)").unwrap();
    let c: Config = Config::from_sex(&atom).unwrap();
    assert_eq!(c.name, "test");
    assert_eq!(c.width, 800);
    assert_eq!(c.height, 200);
}

#[test]
fn struct_keyword_missing_optional() {
    let atom = parse_atom("(\"test\")").unwrap();
    let err = Config::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::MissingField { .. }));
}


#[test]
fn struct_optional_keyword_present() {
    let atom = parse_atom("(\"hello\" :label \"world\")").unwrap();
    let o: OptionalFields = OptionalFields::from_sex(&atom).unwrap();
    assert_eq!(o.name, "hello");
    assert_eq!(o.label, Some("world".into()));
}

#[test]
fn struct_optional_keyword_absent() {
    let atom = parse_atom("(\"hello\")").unwrap();
    let o: OptionalFields = OptionalFields::from_sex(&atom).unwrap();
    assert_eq!(o.name, "hello");
    assert_eq!(o.label, None);
}


#[test]
fn enum_tuple_primitive() {
    let atom = parse_atom("(circle 5)").unwrap();
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Circle(5));
}


#[test]
fn enum_tuple_complex() {
    let atom = parse_atom("(point 1 2)").unwrap();
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Pt(Point { x: 1, y: 2 }));
}


#[test]
fn enum_named_positional_only() {
    let atom = parse_atom("(rect 100 200)").unwrap();
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Rect {
        width: 100,
        height: 200,
        x: 0,
        y: 0
    });
}

#[test]
fn enum_named_with_keywords() {
    let atom = parse_atom("(rect 100 200 :x 10 :y 20)").unwrap();
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Rect {
        width: 100,
        height: 200,
        x: 10,
        y: 20
    });
}

#[test]
fn enum_named_partial_keywords() {
    let atom = parse_atom("(rect 100 200 :x 5)").unwrap();
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Rect {
        width: 100,
        height: 200,
        x: 5,
        y: 0
    });
}


#[test]
fn enum_unit_variant() {
    let atom = parse_atom("(noop)").unwrap();
    let c: Command = Command::from_sex(&atom).unwrap();
    assert_eq!(c, Command::Noop);
}


#[test]
fn enum_tuple_multiple() {
    let atom = parse_atom("(jump 3 4)").unwrap();
    let c: Command = Command::from_sex(&atom).unwrap();
    assert_eq!(c, Command::Jump(3, 4));
}


#[test]
fn enum_named_move() {
    let atom = parse_atom("(move :dx 1 :dy 2)").unwrap();
    let c: Command = Command::from_sex(&atom).unwrap();
    assert_eq!(c, Command::Move { dx: 1, dy: 2 });
}


#[test]
fn enum_unknown_variant() {
    let atom = parse_atom("(triangle 5)").unwrap();
    let err = Shape::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::UnknownVariant { .. }));
}

#[test]
fn enum_empty_list() {
    let atom = parse_atom("()").unwrap();
    let err = Shape::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::TypeError { .. }));
}

#[test]
fn enum_first_element_not_symbol() {
    let atom = parse_atom("(42)").unwrap();
    let err = Shape::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::TypeError { .. }));
}

#[test]
fn struct_from_non_list() {
    let atom = Atom::symbol("oops");
    let err = Point::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::TypeError { .. }));
}

#[test]
fn enum_from_non_list() {
    let atom = Atom::Number(Number::Integer(0));
    let err = Shape::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::TypeError { .. }));
}


#[test]
fn enum_rejects_positional_after_keyword() {
    let atom = parse_atom("(rect 100 :width 200 300)").unwrap();
    let err = Shape::from_sex(&atom).unwrap_err();
    assert!(matches!(
        err,
        SexError::TypeError {
            expected: AtomTy::Integer,
            ..
        }
    ));
}

#[test]
fn struct_rejects_positional_after_keyword() {
    let atom = parse_atom("(\"test\" :width 800 100)").unwrap();
    let err = Config::from_sex(&atom).unwrap_err();
    assert!(matches!(
        err,
        SexError::TypeError {
            expected: AtomTy::Keyword,
            ..
        }
    ));
}
