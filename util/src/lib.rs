pub fn is_symbol_char(c: char) -> bool {
    (c.is_alphabetic() || c.is_ascii_graphic()) && !matches!(c, '(' | ')' | ';' | '"' | '|')
}

pub fn sex_name(string: impl AsRef<str>) -> String {
    let string = string.as_ref();
    for c in string.chars() {
        if !is_symbol_char(c) {
            return string.to_string();
        }
    }
    string.to_lowercase().replace('_', "-")
}
