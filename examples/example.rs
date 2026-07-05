use sex::{parse, FromSex, Sex};

#[derive(Debug, Sex)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Sex)]
struct Config {
    name: String,
    #[sex(keyword)]
    width: i32,
    #[sex(keyword, default = 100)]
    height: i32,
}

#[derive(Debug, Sex)]
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
}

fn main() {
    fn p1(input: &str) -> sex::Atom {
        parse(input).unwrap().pop().unwrap()
    }

    // Test struct with positional args
    let point_atom = p1("(point 10 20)");
    if let sex::Atom::List(list) = &point_atom {
        let point: Point = Point::from_sex(&sex::Atom::List(list[1..].to_vec())).unwrap();
        println!("Point: {:?}", point);
        assert_eq!(point.x, 10);
        assert_eq!(point.y, 20);
    }

    // Test struct with keywords
    let config_atom = p1("(config \"test\" :width 800)");
    if let sex::Atom::List(list) = &config_atom {
        let config: Config = Config::from_sex(&sex::Atom::List(list[1..].to_vec())).unwrap();
        println!("Config: {:?}", config);
        assert_eq!(config.name, "test");
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 100); // default
    }

    // Test enum - pass the full list including the tag
    let circle_atom = p1("(circle 5)");
    let shape: Shape = Shape::from_sex(&circle_atom).unwrap();
    println!("Shape: {:?}", shape);
    match shape {
        Shape::Circle(radius) => assert_eq!(radius, 5),
        _ => panic!("Expected Circle"),
    }

    let rect_atom = p1("(rect 100 200 :x 10 :y 20)");
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

    println!("All tests passed!");
}
