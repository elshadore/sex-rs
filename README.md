# 🍆 Sex: An S-Expression Parser
TODO: redo this entire documentation.
<!-- Sex is a parser for transforming generic lisp data (s-expressions) into rust data. Sex has four main modes of use. -->

<!-- - Basic Lisp Data Reading -->
<!--     Using the `Atom` type enum to procedurally inspect the data parsed. -->
<!-- - Iterative Views (`AtomView`, `KeywordView`) -->
<!--     For easier iteration and extraction of data (particulary `:kewords`) -->
<!-- - A `FromSex` Trait -->
<!--     This can be attached to types for typed declarative parsing. -->
<!-- - A Declarative Macro (Like Serde) -->
<!--     This auto implements the `FromSex` trait. -->
  
<!-- Because of the wide variety of lisps and the nature of the rust data it is mapping to, Sex makes the following choices for maximum compatibility. -->

<!-- - `;` Is used for comments. This will comment out the rest of the line. -->
<!-- - `true` are used for rust `true`. `t`, `#t` will fail. -->
<!-- - (`false` && `nil`) are used for rust `false`. `f`, `#f` will fail. -->
<!-- - `nil` is used for rust `None`. -->
<!-- - `:keyword` is use for keywords, `#:keyword` and `keyword:` will fail to parse. -->
<!-- - `|` braces: `|foo bar|`, are used for strictly creating a symbol (it can contain any character). -->

<!-- ## Derive Macro Example -->

<!-- ```rust -->
<!-- // Simple Struct Example. -->
<!-- #[derive(Sex)] -->
<!-- struct Point { -->
<!--     #[sex(keyword, default = 0)] -->
<!--     x: i64, -->
<!--     #[sex(keyword, default = 0)] -->
<!--     y: i64, -->
<!-- } -->

<!-- fn example1() { -->
<!--     let point_atom: Atom = parse_atom("(10 -5)").unwrap(); -->
<!--     let point = Point::from_sex(&point_atom).unwrap(); -->
<!--     assert_eq!(point.x, 10); -->
<!--     assert_eq!(point.y, -5); -->
<!-- } -->

<!-- // Enum Example. -->
<!-- // This creates a tagged expression. -->
<!-- // This also shows the use of the keyword tag and the default values that can be assigned. -->
<!-- #[derive(Sex)] -->
<!-- enum Shape { -->
<!--     #[sex(tag = "circle")] -->
<!--     Circle(i32), -->

<!--     #[sex(tag = "point")] -->
<!--     Point(Point), -->

<!--     #[sex(tag = "rect")] -->
<!--     Rect { -->
<!--         width: i64, -->
<!--         height: i64, -->
<!--         #[sex(keyword, default = 0)] -->
<!--         x: i64, -->
<!--         #[sex(keyword, default = 0)] -->
<!--         y: i64, -->
<!--     } -->
<!-- } -->

<!-- fn example2() { -->
<!--     let circle_atom: Atom = parse_atom("(circle 5)").unwrap(); -->
    
<!--     let shape: Shape = Shape::from_sex(&circle_atom).unwrap(); -->
<!--     match shape { -->
<!--         Shape::Circle(radius) => assert_eq!(radius, 5), -->
<!--         _ => panic!("expected circle"), -->
<!--     } -->

<!--     let rect_atom: Atom = parse_atom("(rect 100 200 :y 20)").unwrap(); -->
<!--     let shape: Shape = Shape::from_sex(&rect_atom).unwrap(); -->
<!--     match shape { -->
<!--         Shape::Rect { width, height, x, y } => { -->
<!--             assert_eq!(width, 100); -->
<!--             assert_eq!(height, 200); -->
<!--             assert_eq!(x, 0); -->
<!--             assert_eq!(y, 20); -->
<!--         } -->
<!--         _ => panic!("expected rect"), -->
<!--     } -->
<!-- } -->
<!-- ``` -->
