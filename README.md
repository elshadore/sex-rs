# 🍆 Sex: An S-Expression Parser, Serializer and Deserializer 🍆
Sex is a generic s-expression parser that can be used for serializing and deserializing rust data to the format. The format uses any lisp value for true, `nil` for nil/null values as well as false. It also has `:keywords`, primarily for optional key value pairs. The generic sexpression data will work in all lisps (Scheme, Common Lisp, Clojure, ...).

## Derive Macro Example

```rust
// Basic Struct Example
#[derive(Sex)]
struct Point {
    x: i32,
    y: i32,
}
// (10 20) => Point { x: 10, y: 20 }

// Enum Example, with tags, keywords and default values.
#[derive(Sex)]
enum Shape {
    #[sex(tag = "circle")]
    Circle(i32),

    #[sex(tag = "point")]
    Point(Point),

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
// (circle 5)            => Shape::Circle(5)
// (point 1 2)           => Shape::Point(Point { x: 1, y: 2 })
// (rect 100 200 :x 10)  => Shape::Rect { width: 100, height: 200, x: 10, y: 0 }
```
