# TODOLIST for Sex Version 1
Sex is aiming to be a generic lisp data format. As a result we want to make it compatibile enough with other lisps, but fundementally require each lisp to implement the format itself (as it differs from each lisp(s) print enough).

Both symbols and strings have to be valid unicode characters.

- [ ] List Type
  - Immutable
  - [ ] List Builder Type
    - Mutable
  - Review List View and Keyword View
- [ ] Printer
  - Normal
  - Pretty Printing
  Implement Display
- [ ] Serializer
  - IntoSex Trait
- [ ] Redo Macro Derive
  derive changes rust symbols `foo_bar` to lisp symbols `foo-bar`
- [ ] default in the declarative macro should use the Default trait if no value is provided
- [ ] Look at *all* Error Messages to see if they make sense.
  - Unicode has bad errors
- [ ] Test check.
  - Ensure good coverage.
  - Remove useless tests.
- [ ] Quality Check
- [ ] Redo `example/example.rs`
  Use this add the base for the example in the README
- [ ] Redo README
- [ ] Floating Point Parsing Precision
- [ ] Write Format Spec
