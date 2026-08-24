# TODOLIST for Sex Version 1
Sex is aiming to be a generic lisp data format. As a result we want to make it compatibile enough with other lisps, but fundementally require each lisp to implement the format itself (as it differs from each lisp(s) print enough).

Both symbols and strings have to be valid unicode characters.

- [ ] Full symbol processing
  All characters are valid even `.` or `'`, that are used for things in other lisps.
  - [x] Whitespace symbols:
    `|bar symbols|`
  - [x] Escaping with \\ (same set as strings, plus `\|`)
    `|bar \| symbols \\|`
  - [x] Barred keywords: `:|foo bar|` (empty `:||` allowed)
  Keywords
  `:|keywords as well|`
- [ ] Ensure Full Number Parsing
  emulate rust or jsons number parsing rules
- [ ] Nice Parser Errors like *compilation* buffer format
  - Prints line number, char number, full line print, error print.
  - Parser `Metadata` for file name 
  - Seperate Logical Parser Error that are `Copy` and `Clone`
- [ ] Redo `example/example.rs`
  Use this add the base for the example in the README
- [ ] Redo README
- [ ] Serializer
- [ ] Redo Macro Derive
  derive changes rust symbols `foo_bar` to lisp symbols `foo-bar`
- [ ] Look into Serde and how it works
    wether we can integrate serde into sex
- [ ] default in the declarative macro should use the Default trait if no value is provided
- [ ] Look at *all* Error Messages to see if they make sense.
- [ ] Test check.
  - Ensure good coverage.
  - Remove useless tests.
- [ ] Quality Check
