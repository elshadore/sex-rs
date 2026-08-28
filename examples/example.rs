use sex::{Atom, FromSex, IntoSex, List, ListBuilder, ListView, list, parse_expression_str};

#[derive(Debug, FromSex, IntoSex)]
struct Point {
    #[sex(keyword = "x")]
    point_x: i64,
    #[sex(keyword = "y")]
    point_y: i64,
}

#[derive(Debug, FromSex, IntoSex)]
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
    let shape: Shape = Shape::from_atom(&shape_atom).unwrap();
    println!("{shape:?}")
}

/// `Sex` data example.
fn example2() {
    let data: List = list![Atom::symbol("+"), Atom::integer(1), Atom::integer(2)];
    println!("{data}");
    
}

/// `ListBuilder` example
fn example3() {
    let mut builder = ListBuilder::new();
    builder.push(Atom::symbol("foo"));
    builder.push(Atom::symbol("bar"));
    builder.push(Atom::symbol("baz"));
    let data = builder.build();
    println!("{data}");
}

/// `ListView` and `KeywordView` example
fn example4() {
    let atoms: Atom = parse_expression_str("(foo (bar 1 2 3) baz :foo 10 :bar 40)", None).unwrap();
    let mut view = ListView::new(atoms.try_as_list().unwrap());
    let foo = view.pop().unwrap();
    let mut view2 = view.enter_list().unwrap();
    {
        let bar = view2.pop().unwrap();
        let at1 = view2.pop().unwrap();
        let at2 = view2.pop().unwrap();
        let at3 = view2.pop().unwrap();
        view2.expect_finished().unwrap();
        println!("{bar}, {at1}, {at2}, {at3}");
    }
    let baz = view.pop().unwrap();
    let kw = view.into_keywords().unwrap();
    let kw_foo = kw.get("foo").unwrap();
    let kw_bar = kw.get("bar").unwrap();
    println!("{foo}, {baz}, foo: {kw_foo}, bar: {kw_bar}");
}

/// Read from file.
fn example5() {
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
    example4();
    example5();
}
