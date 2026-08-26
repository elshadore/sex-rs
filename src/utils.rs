use crate::is_symbol_char;

/// A function used in the `FromSex` and `IntoSex` macros for determining the symbol names of things.
pub fn sex_name(string: impl AsRef<str>) -> String {
    let string = string.as_ref();
    for c in string.chars() {
        if !is_symbol_char(c) {
            return string.to_string();
        }
    }
    string.to_lowercase().replace('_', "-")
}
