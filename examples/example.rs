use sex::{Atom, FromSex, Sex, parse_atom};

#[derive(Debug, Sex)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Sex)]
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
}

fn point_example() {
    let point_atom: Atom = parse_atom("(point 10 20)").unwrap();
    
    if let Atom::List(list) = &point_atom {
        let point: Point = Point::from_sex(&Atom::List(list[1..].to_vec())).unwrap();
        println!("Point: {:?}", point);
        assert_eq!(point.x, 10);
        assert_eq!(point.y, 20);
    }
}

fn shape_example() {
    let circle_atom: Atom = parse_atom("(circle 5)").unwrap();
    
    let shape: Shape = Shape::from_sex(&circle_atom).unwrap();
    println!("Shape: {:?}", shape);
    match shape {
        Shape::Circle(radius) => assert_eq!(radius, 5),
        _ => panic!("Expected Circle"),
    }

    let rect_atom: Atom = parse_atom("(rect 100 200 :x 10 :y 20)").unwrap();
    let shape: Shape = Shape::from_sex(&rect_atom).unwrap();
    println!("Shape: {:?}", shape);
    match shape {
        Shape::Rect { width, height, x, y } => {
            assert_eq!(width, 100);
            assert_eq!(height, 200);
            assert_eq!(x, 10);
            assert_eq!(y, 20);
        }
        _ => panic!("Expected Rect"),
    }
}
    

fn main() {
    point_example();
    shape_example();
}
