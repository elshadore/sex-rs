# sex — S-Expression Parser & Deserialization

A generic S-expression parser with a cursor-based view API and a `#[derive(Sex)]` macro for declarative deserialization into Rust structs and enums.

## Parsing

```rust
let atoms = sex::parse("(defexample foo :src (path \"bar.sex\") :x 0 :y 0)")?;
```

Returns `Vec<Atom>`. Each top-level form is one element. Parses symbols, keywords (`:name`), quoted strings, integers, floats, lists, `nil`, and `true`/`t`.

## The `Atom` type

```rust
pub enum Atom {
    Nil,
    True,
    Number(Number),     // Number::Integer(i32) or Number::Float(f32)
    Text(Text),         // Text { ty: TextTy, contents: String }
    List(Vec<Atom>),
}
```

Accessor methods chain from `try_as_*` (returns `Result`) or `as_*` (returns `Option`):

- `try_as_symbol()`, `try_as_keyword()`, `try_as_text()`, `try_as_integer()`, `try_as_float()`, `try_as_list()`, `try_as_true()`

`TextTy` distinguishes the three text variants: `Symbol`, `Keyword`, `String`.

## `AtomView` — procedural cursor

A cursor over `&[Atom]` for reading atoms in sequence:

```rust
let atoms = sex::parse("(defexample foo :src \"bar.sex\")")?;
let mut view = sex::AtomView::new(&atoms);
let mut list = view.enter_list()?;             // enter nested list

let name = list.next().unwrap();               // "defexample"
let foo = list.next().unwrap();                // "foo"

let kw = list.into_keywords()?;                // strict key-value remainder
let src = kw.get("src").unwrap();              // Atom::string("bar.sex")
```

| Method | Behaviour |
|---|---|
| `peek()` / `next()` | `Option<&Atom>` — non-fallible access |
| `try_peek()` / `try_next()` | `Result<&Atom, SexError>` — errors with `ExpectedAtom` at end |
| `skip(n)` | Advance by `n`, saturating |
| `remaining()` / `remaining_slice()` | Unconsumed count / slice |
| `is_finished()` / `expect_finished()` | End-of-input check |
| `expect_last()` | `try_next()` + `expect_finished()` in one call |
| `enter_list()` | Consume next atom as list, return new `AtomView` over its elements |
| `into_keywords(self)` | Consume remainder as strict `:key value` pairs → `KeywordView` |

## `KeywordView` — strict key-value parsing

Every remaining atom must be a keyword (`:name`) followed by a value, alternating strictly.

```rust
let kw: KeywordView<'_> = view.into_keywords()?;

let width: i32 = kw.required("width")?;              // error if missing
let title: Option<String> = kw.optional("title")?;   // None if absent
```

| Method | Behaviour |
|---|---|
| `required::<T>(name)` | Look up + deserialize via `FromSex`; `MissingField` if absent |
| `optional::<T>(name)` | Look up or `Ok(None)`; `TypeError` if present but wrong type |
| `get(name)` | `Option<&Atom>` — raw value |
| `contains_key(name)` | Presence check |
| `iter()` | `(name, &Atom)` pairs |
| `len()` / `is_empty()` | Pair count |

Construction errors on: non-keyword atom (`TypeError`), keyword without value (`UnexpectedEof`).

## `FromSex` trait

```rust
pub trait FromSex: Sized {
    fn from_sex(atom: &Atom) -> Result<Self, SexError>;
}
```

Built-in implementations:

| Rust type | Accepts |
|---|---|
| `String` | any `Text` variant (symbol, keyword, string) |
| `i32` | `Number::Integer` |
| `f32` | `Number::Float` or `Number::Integer` |
| `bool` | `true`, symbol `"true"`, symbol `"false"`, `nil` |
| `()` | `nil` |
| `Option<T>` | `nil` → `None`, otherwise delegates to `T::from_sex` |
| `Vec<T>` | `Atom::List`, mapping each element |

## `#[derive(Sex)]` — declarative deserialization

Works on structs (named fields only) and enums.

### Structs

**Positional fields** (default) — matched in declaration order:

```rust
#[derive(Sex)]
struct Point {
    x: i32,
    y: i32,
}
// (10 20)  →  Point { x: 10, y: 20 }
```

**Keyword fields** with `#[sex(keyword)]`:

```rust
#[derive(Sex)]
struct Config {
    name: String,
    #[sex(keyword)]
    width: i32,
    #[sex(keyword, default = 100)]
    height: i32,
}
// ("test" :width 800)  →  Config { name: "test", width: 800, height: 100 }
```

### Enums

The first element of the list is a **symbol tag** that selects the variant. Remaining elements become the variant's fields.

```rust
#[derive(Sex)]
enum Shape {
    #[sex(tag = "circle")]
    Circle(i32),                          // single primitive → one positional atom

    #[sex(tag = "point")]
    Pt(Point),                            // single complex type → entire rest body

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
// (circle 5)           →  Shape::Circle(5)
// (point 1 2)           →  Shape::Pt(Point { x: 1, y: 2 })
// (rect 100 200 :x 10)  →  Shape::Rect { width: 100, height: 200, x: 10, y: 0 }
```

**Positional-before-keyword ordering** is enforced: all positional atoms must come before the first keyword, or parsing fails with `TypeError`.

### Attribute reference

| Attribute | Scope | Description |
|---|---|---|
| `#[sex(keyword)]` | struct / enum-named field | Parse as `:name value` pair |
| `#[sex(keyword = "str")]` | struct / enum-named field | Keyword with explicit name |
| `#[sex(default)]` | struct / enum-named field | Optional field, uses `Default::default()` |
| `#[sex(default = expr)]` | struct / enum-named field | Optional field with custom default |
| `#[sex(tag = "str")]` | enum variant | Symbol tag for dispatch |

## Error types

```rust
pub enum SexError {
    UnexpectedEof { pos: Position },
    UnexpectedChar { pos: Position, ch: char },
    UnterminatedList { pos: Position },
    UnterminatedString { pos: Position },
    InvalidEscape { pos: Position, ch: char },
    InvalidNumber { pos: Position, value: String },
    EmptyKeyword { pos: Position },
    TypeError { expected: AtomTy, found: Atom },
    MissingField { name: String },
    UnknownVariant { variant: String, expected: Vec<String> },
    ExpectedAtom,
    ExpectedFinished,
}
```

`AtomTy` describes what was expected: `Symbol`, `Keyword`, `Text`, `Integer`, `Float`, `True`, `Nil`, or `List`.
