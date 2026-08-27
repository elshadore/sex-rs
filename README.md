# 🍆 Sex: An S-Expression Parser
Sex is an sexpression data format designed to be a JSON of sexpression data, to be used between different programming languages and applications. Sex can also be used as a declarive scripting language for configs as generic sexpression data is perfect for custom declarive languages.

## Format Desciption
Below is a desciption of the formats data types.

### `nil`
The `nil` have represents `nil`.

### `Booleans` (`true`/`false`)
Boolean values respresenting the formats logical true and false operations. Note whilst `true` is true, and `false` is false, `nil` is also false as well.

### `Numbers` (Integer/Float)
`100`, `-1.03`, `120e+10`.
Basic number like anything else. These follow the JSON format for number parsing so refer the that SPEC for the details on the way numbers work.

### `Lists`
The core structure of sexpressions. They start with the character `(` and end with the character `)`. Elements in a list have be seperated with whitespace. Here are some examples:
- `(foo bar baz)`
- `(foo bar (bar "foo" 100 :key value))`
- `()`, the empty list. Note for users coming from LISP languages, the empty list does not equal `nil`.

### `Symbols`
`foo`, `bar`, `|foo bar`.
Basic symbol parsing follows some easy rules.

- The sequence cannot start with the character `:` (this is a keyword)
- The sequence cannot have whitespace (whitespace will be interpetted as a symbol end)
- The sequence cannot start with a number (this will be interpetted as number parsing)
- The sequence cannot contain the graphic characters `(`, `)`, `"`, `|`
- The sequence cannot be the following symbols `true`, `false`, `nil`

Other than that any printable unicode character is allowed.
Advanced symbol parsing allows you to use the `|` character like you would use the `"` in a string literal. This allows for any symbol with any set of characters. Example:

- `|true|` The `Symbol` true, not the value `true`.
- `|foo bar baz` Whitespace in a symbol.
- `|1000|` This is a `Symbol` not a `Number`.
- `|\xFF  \u{03BB} \| \t\n|` Escape sequences like strings with the addition of the `\|` escape for the `|` literal character.

### `Keywords`
`:foo`, `:bar`, `:|foo bar|`.
Keyword are a lot like symbols. Except start with the character `:`. For this reason `:100` is a valid `Keyword`. Keywords also have `|` bar character advanced parsing as well such as `:|foo bar|`. Keywords are used of optional key value pairs in a `List`. They are better understood by demonstration rather than explaination, the declarative macro section later on will show there usecases.

### `Strings`
`"hello"`, `"world"`, `"cool\nbeans"`
Strings are pretty basic like every other string implementation. Escape codes are allowed, for reference on the default escape codes also used in the `Symbol` processing.
- `\"` the `"` literal.
- `\\` the `\` literal.
- `\n` a newline character.
- `\t` a tab character.
- `\r` carriage return.
- `\0` null character.
- `\xFF` a hex escape, featuring two custom hex characters. As strings are all valid Unicode, this essentially is read as extended ascii and is converted into whatever unicode format that is being used, so this does not refer to the actual character byte being used.
- `\u{FFFFFF}` a unicode escape, this features up to six custom hex codes are refers to a valid Unicode codepoint character. Example `\u{03BB}` => `λ`.

<!-- ## The Library -->
<!-- The Sex library comes with a bunch of tools for working with the format. -->

<!-- - Basic Lisp Data -->
<!--     Using the `Atom` type primitive as a code Sex data can be respresented in a LISP like form. -->
<!-- - Iterative Views -->
<!--     The `ListView` and `KeywordView` are custom iterative views that make procedural parsing easy to accomplish. -->
<!-- - `FromSex` and `IntoSex` Traits -->
<!--     The Rust Traits `FromSex` and `IntoSex` allow you to use the Rust type system to  -->
<!-- - Declarive Macros (like serde) -->
<!--     Using the `#[derive(FromSex)]` and `#[derive(IntoSex)]` forms to automate the construction of the `FromSex` and `IntoSex` traits to create declarative serialization and deserialization. -->

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
