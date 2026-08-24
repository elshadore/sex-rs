# TODOLIST for Sex Version 1
Sex is aiming to be a generic lisp data format. As a result we want to make it compatibile enough with other lisps, but fundementally require each lisp to implement the format itself (as it differs from each lisp(s) print enough).

- [ ] `;` Comments
- [ ] `true` and `false`
- [ ] Full symbol processing
  `|bar symbols|`
  `|bar \| symbols \\|`
  Banned characters
- [ ] Readd `example/example.rs`
  Use this add the base for the example in the README
- [ ] Redo README
- [ ] Serializer
- [ ] Redo Macro Derive
- [ ] Look into Serde and how it works
    wether we can integrate serde into sex
- [ ] default in the declarative macro should use the Default trait if no value is provided
- [ ] Quality Check
