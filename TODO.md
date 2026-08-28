# TODOLIST for Sex Version 1
Sex is aiming to be a generic lisp data format. As a result we want to make it compatibile enough with other lisps, but fundementally require each lisp to implement the format itself (as it differs from each lisp(s) print enough).

Both symbols and strings have to be valid unicode characters.

- [ ] Check Macro output in example (Macroexpand)
- [ ] Test check.
  - Ensure good coverage.
  - Remove useless tests.
- [ ] Quality Check
- [ ] Pretty Printing
  - [ ] Method on `IntoSex`


## Keyword Details
if the struct has the specifier `keyword` the `default` modifier can be applied to it. By default the default modifier means the keyword is no longer `strict`, as in it doesn't have to be supplied by the sex data. Its default value is determined by the Default trait, if there is not Default trait for the type, a compilation error is thrown. An optional override for the default modifer can be specified. `default = value` specifies to use that value as the default instead (no Default trait is needed).
