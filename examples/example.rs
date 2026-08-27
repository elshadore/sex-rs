use sex::{Atom, FromSex, IntoSex, List, list, parse_expression_str};

#[derive(FromSex, IntoSex)]
struct Point {
    #[sex(keyword = "x")]
    point_x: i64,
    #[sex(keyword = "y")]
    point_y: i64,
}

#[derive(FromSex, IntoSex)]
enum Shape {
    #[sex(tag = "circle")]
    Circle(i32),

    #[sex(tag = "point")]
    Point(Point),

    #[sex(tag = "rect")]
    Rect {
        width: i64,
        height: i64,
        #[sex(keyword, default)]
        x: i64,
        #[sex(keyword, default = 0)]
        y: i64,
    }
}

/// Declarative derive example
fn example1() {
    let shape_atom: Atom = parse_expression_str("(rect 200 100 :y 10)", None).unwrap();
    // let shape: Shape = Shape::from_sex(view)
}

/// `Sex` data example.
fn example2() {
    let data: List = list![Atom::symbol("+"), Atom::integer(1), Atom::integer(2)];
    println!("{data}")
}

/// `ListView` and `KeywordView` example
fn example3() {}

/// Read from file example using the `parse*` functions.
fn example4() {
    let name = String::from("examples/example.sex");
    let file = std::fs::File::open(&name).unwrap();
    match sex::parse_exprlist_reader(file, Some(name)) {
        Ok(atom) => {
            println!("{atom}");
        }
        Err(err) => {
            eprintln!("{err}");
        }
    }
}

fn main() {
    example1();
    example2();
    example3();
}
