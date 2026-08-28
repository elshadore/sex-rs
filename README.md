# 🍆 Sex: An S-Expression Parser
Sex is an sexpression data format designed to be a JSON of sexpression data. This is format is primarily designed to be used between different programming languages and applications. Another great usecase for Sex can be as a declarive scripting language for application configs, as generic sexpression data is perfect for custom declarive languages.

## Example
```rust
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
```

## The Library
The Sex library comes with a bunch of tools for working with the format.

- Basic Lisp Data

    Using the `Atom` type primitive as a code Sex data can be respresented in a LISP like form.

- Iterative Views

    The `ListView` and `KeywordView` are custom iterative views that make procedural parsing easy to accomplish.

- `FromSex` and `IntoSex` Traits

    The Rust Traits `FromSex` and `IntoSex` allow you to use the Rust type system to

- Declarive Macros (like serde)

    Using the `#[derive(FromSex)]` and `#[derive(IntoSex)]` forms to automate the construction of the `FromSex` and `IntoSex` traits to create declarative serialization and deserialization.

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
`(foo)`, `(foo bar baz)`, `()`.

The core structure of sexpressions. They start with the character `(` and end with the character `)`. Elements in a list have be seperated with whitespace. Here are some examples:
- `(foo bar baz)`
- `(foo bar (bar "foo" 100 :key value))`
- `()`, the empty list. Note for users coming from LISP languages, the empty list does not equal `nil`.

### `Symbols`
`foo`, `bar`, `|foo bar|`.

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
`"hello"`, `"world"`, `"cool\nbeans"`.

Strings are pretty basic like every other string implementation. Escape codes are allowed, for reference on the default escape codes also used in the `Symbol` processing.
- `\"` the `"` literal.
- `\\` the `\` literal.
- `\n` a newline character.
- `\t` a tab character.
- `\r` carriage return.
- `\0` null character.
- `\xFF` a hex escape, featuring two custom hex characters. As strings are all valid Unicode, this essentially is read as extended ascii and is converted into whatever unicode format that is being used, so this does not refer to the actual character byte being used.
- `\u{FFFFFF}` a unicode escape, this features up to six custom hex codes are refers to a valid Unicode codepoint character. Example `\u{03BB}` => `λ`.
