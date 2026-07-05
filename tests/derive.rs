use sex::{parse, Atom, AtomTy, FromSex, Number, Sex, SexError};

fn p1(input: &str) -> Atom {
    parse(input).unwrap().remove(0)
}

#[derive(Debug, PartialEq, Sex)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Sex)]
struct Config {
    name: String,
    #[sex(keyword)]
    width: i32,
    #[sex(keyword, default = 100)]
    height: i32,
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
    Circle(i32),

    #[sex(tag = "rect")]
    Rect {
        width: i32,
        height: i32,
        #[sex(keyword, default = 0)]
        x: i32,
        #[sex(keyword, default = 0)]
        y: i32,
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
        dx: i32,
        #[sex(keyword)]
        dy: i32,
    },

    #[sex(tag = "jump")]
    Jump(i32, i32),
}

// -----------------------------------------------------------------------
// Derive – Struct positional fields
// -----------------------------------------------------------------------

#[test]
fn struct_positional() {
    let atom = p1("(10 20)");
    let p: Point = Point::from_sex(&atom).unwrap();
    assert_eq!(p, Point { x: 10, y: 20 });
}

#[test]
fn struct_positional_single() {
    let atom = p1("(99)");
    let err = Point::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::ExpectedAtom));
}

// -----------------------------------------------------------------------
// Derive – Struct keyword fields
// -----------------------------------------------------------------------

#[test]
fn struct_keyword() {
    let atom = p1("(\"test\" :width 800)");
    let c: Config = Config::from_sex(&atom).unwrap();
    assert_eq!(c.name, "test");
    assert_eq!(c.width, 800);
    assert_eq!(c.height, 100); // default
}

#[test]
fn struct_keyword_default_used() {
    let atom = p1("(\"test\" :width 800 :height 200)");
    let c: Config = Config::from_sex(&atom).unwrap();
    assert_eq!(c.name, "test");
    assert_eq!(c.width, 800);
    assert_eq!(c.height, 200);
}

#[test]
fn struct_keyword_missing_optional() {
    let atom = p1("(\"test\")");
    let err = Config::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::MissingField { .. }));
}

// -----------------------------------------------------------------------
// Derive – Struct optional keyword
// -----------------------------------------------------------------------

#[test]
fn struct_optional_keyword_present() {
    let atom = p1("(\"hello\" :label \"world\")");
    let o: OptionalFields = OptionalFields::from_sex(&atom).unwrap();
    assert_eq!(o.name, "hello");
    assert_eq!(o.label, Some("world".into()));
}

#[test]
fn struct_optional_keyword_absent() {
    let atom = p1("(\"hello\")");
    let o: OptionalFields = OptionalFields::from_sex(&atom).unwrap();
    assert_eq!(o.name, "hello");
    assert_eq!(o.label, None);
}

// -----------------------------------------------------------------------
// Derive – Enum tuple variant (single field, primitive)
// -----------------------------------------------------------------------

#[test]
fn enum_tuple_primitive() {
    let atom = p1("(circle 5)");
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Circle(5));
}

// -----------------------------------------------------------------------
// Derive – Enum tuple variant (single field, complex/composite)
// -----------------------------------------------------------------------

#[test]
fn enum_tuple_complex() {
    let atom = p1("(point 1 2)");
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Pt(Point { x: 1, y: 2 }));
}

// -----------------------------------------------------------------------
// Derive – Enum named variant with positional + keyword fields
// -----------------------------------------------------------------------

#[test]
fn enum_named_positional_only() {
    let atom = p1("(rect 100 200)");
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Rect { width: 100, height: 200, x: 0, y: 0 });
}

#[test]
fn enum_named_with_keywords() {
    let atom = p1("(rect 100 200 :x 10 :y 20)");
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Rect { width: 100, height: 200, x: 10, y: 20 });
}

#[test]
fn enum_named_partial_keywords() {
    let atom = p1("(rect 100 200 :x 5)");
    let s: Shape = Shape::from_sex(&atom).unwrap();
    assert_eq!(s, Shape::Rect { width: 100, height: 200, x: 5, y: 0 });
}

// -----------------------------------------------------------------------
// Derive – Enum unit variant
// -----------------------------------------------------------------------

#[test]
fn enum_unit_variant() {
    let atom = p1("(noop)");
    let c: Command = Command::from_sex(&atom).unwrap();
    assert_eq!(c, Command::Noop);
}

// -----------------------------------------------------------------------
// Derive – Enum multiple tuple fields
// -----------------------------------------------------------------------

#[test]
fn enum_tuple_multiple() {
    let atom = p1("(jump 3 4)");
    let c: Command = Command::from_sex(&atom).unwrap();
    assert_eq!(c, Command::Jump(3, 4));
}

// -----------------------------------------------------------------------
// Derive – Enum named variant without positional
// -----------------------------------------------------------------------

#[test]
fn enum_named_move() {
    let atom = p1("(move :dx 1 :dy 2)");
    let c: Command = Command::from_sex(&atom).unwrap();
    assert_eq!(c, Command::Move { dx: 1, dy: 2 });
}

// -----------------------------------------------------------------------
// Derive – Enum error cases
// -----------------------------------------------------------------------

#[test]
fn enum_unknown_variant() {
    let atom = p1("(triangle 5)");
    let err = Shape::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::UnknownVariant { .. }));
}

#[test]
fn enum_empty_list() {
    let atom = p1("()");
    let err = Shape::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::TypeError { .. }));
}

#[test]
fn enum_first_element_not_symbol() {
    let atom = p1("(42)");
    let err = Shape::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::TypeError { .. }));
}

// -----------------------------------------------------------------------
// Derive – Type mismatch error cases
// -----------------------------------------------------------------------

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

// -----------------------------------------------------------------------
// Derive – Positional-before-keyword strict ordering
// -----------------------------------------------------------------------

#[test]
fn enum_rejects_positional_after_keyword() {
    let atom = p1("(rect 100 :width 200 300)");
    let err = Shape::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::TypeError { expected: AtomTy::Integer, .. }));
}

#[test]
fn struct_rejects_positional_after_keyword() {
    let atom = p1("(\"test\" :width 800 100)");
    let err = Config::from_sex(&atom).unwrap_err();
    assert!(matches!(err, SexError::TypeError { expected: AtomTy::Keyword, .. }));
}
