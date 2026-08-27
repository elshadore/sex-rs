use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod from_sex;
mod into_sex;
mod shared;

/// Derive macro for declarively parsing sexpression data.
/// Examples:
///
/// #[derive(FromSex)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// #[derive(FromSex)]
/// enum Shape {
///     #[sex(tag = "circle")]
///     Circle(i32),
///
///     #[sex(tag = "point")]
///     Point(Point),
///
///     #[sex(tag = "rect")]
///     Rect {
///         width: i32,
///         height: i32,
///
///         #[sex(keyword = "z", default)]
///         x: i32,
///         #[sex(keyword, default = 0)]
///         y: i32,
///     },
/// }
#[proc_macro_derive(FromSex, attributes(sex))]
pub fn derive_from_sex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = from_sex::expand_from_sex(&input.ident, &input.data);
    TokenStream::from(expanded)
}

#[proc_macro_derive(IntoSex, attributes(sex))]
pub fn derive_into_sex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = into_sex::expand_into_sex(&input.ident, &input.data);
    TokenStream::from(expanded)
}
