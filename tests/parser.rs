use sex::{Atom, Number, SexParserError, parse_atom, parse_listed};

#[test]
fn parse_bare_symbol() {
    assert_eq!(parse_atom("hello").unwrap(), Atom::symbol("hello"));
}

#[test]
fn parse_symbol_with_hyphen() {
    assert_eq!(parse_atom("foo-bar").unwrap(), Atom::symbol("foo-bar"));
}

#[test]
fn parse_symbol_with_underscore() {
    assert_eq!(parse_atom("foo_bar").unwrap(), Atom::symbol("foo_bar"));
}

#[test]
fn parse_symbol_with_slash() {
    assert_eq!(parse_atom("foo/bar").unwrap(), Atom::symbol("foo/bar"));
}

#[test]
fn parse_symbol_with_dot() {
    assert_eq!(parse_atom("foo.bar").unwrap(), Atom::symbol("foo.bar"));
}

#[test]
fn parse_symbol_with_star() {
    assert_eq!(parse_atom("foo*bar").unwrap(), Atom::symbol("foo*bar"));
}

#[test]
fn parse_symbol_with_plus() {
    assert_eq!(parse_atom("foo+bar").unwrap(), Atom::symbol("foo+bar"));
}

#[test]
fn parse_symbol_with_exclamation() {
    assert_eq!(parse_atom("foo!").unwrap(), Atom::symbol("foo!"));
}

#[test]
fn parse_symbol_with_question() {
    assert_eq!(parse_atom("foo?").unwrap(), Atom::symbol("foo?"));
}

#[test]
fn parse_symbol_with_angle_brackets() {
    assert_eq!(parse_atom("foo<bar>").unwrap(), Atom::symbol("foo<bar>"));
}

#[test]
fn parse_symbol_with_equals() {
    assert_eq!(parse_atom("foo=bar").unwrap(), Atom::symbol("foo=bar"));
}

#[test]
fn parse_symbol_with_ampersand() {
    assert_eq!(parse_atom("foo&bar").unwrap(), Atom::symbol("foo&bar"));
}

#[test]
fn parse_symbol_with_percent() {
    assert_eq!(parse_atom("foo%bar").unwrap(), Atom::symbol("foo%bar"));
}

#[test]
fn parse_symbol_mixed() {
    let s = parse_atom("a-b/c.d_e+f!?").unwrap();
    assert_eq!(s, Atom::symbol("a-b/c.d_e+f!?"));
}

#[test]
fn parse_symbol_starting_with_dot() {
    assert_eq!(parse_atom(".foo").unwrap(), Atom::symbol(".foo"));
}

#[test]
fn parse_symbol_single_dot() {
    assert_eq!(parse_atom(".").unwrap(), Atom::symbol("."));
}

#[test]
fn parse_symbol_single_hyphen() {
    assert_eq!(parse_atom("-").unwrap(), Atom::symbol("-"));
}

#[test]
fn parse_true_is_a_symbol() {
    assert_eq!(parse_atom("true").unwrap(), Atom::symbol("true"));
}

#[test]
fn parse_t_is_a_symbol() {
    assert_eq!(parse_atom("t").unwrap(), Atom::symbol("t"));
}

#[test]
fn parse_nil() {
    assert_eq!(parse_atom("nil").unwrap(), Atom::Nil);
}

#[test]
fn parse_nil_in_list() {
    let atoms = parse_listed("(nil)").unwrap();
    assert_eq!(atoms, vec![Atom::List(vec![Atom::Nil])]);
}


#[test]
fn parse_integer_zero() {
    assert_eq!(parse_atom("0").unwrap(), Atom::Number(Number::Integer(0)));
}

#[test]
fn parse_integer_positive() {
    assert_eq!(parse_atom("42").unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_integer_negative() {
    assert_eq!(parse_atom("-42").unwrap(), Atom::Number(Number::Integer(-42)));
}

#[test]
fn parse_integer_max() {
    assert_eq!(
        parse_atom("9223372036854775807").unwrap(),
        Atom::Number(Number::Integer(9223372036854775807))
    );
}

#[test]
fn parse_integer_min() {
    assert_eq!(
        parse_atom("-9223372036854775808").unwrap(),
        Atom::Number(Number::Integer(-9223372036854775808))
    );
}

#[test]
fn parse_negative_sign_not_number() {
    assert_eq!(parse_atom("-").unwrap(), Atom::symbol("-"));
}


#[test]
fn parse_float_simple() {
    assert_eq!(parse_atom("3.14").unwrap(), Atom::Number(Number::Float(3.14)));
}

#[test]
fn parse_float_negative() {
    assert_eq!(parse_atom("-2.5").unwrap(), Atom::Number(Number::Float(-2.5)));
}

#[test]
fn parse_float_zero() {
    assert_eq!(parse_atom("0.0").unwrap(), Atom::Number(Number::Float(0.0)));
}

#[test]
fn parse_trailing_dot_is_valid_float() {
    assert_eq!(parse_atom("42.").unwrap(), Atom::Number(Number::Float(42.0)));
}

#[test]
fn parse_double_dot_requires_whitespace() {
    let err = parse_listed("1.2.3").unwrap_err();
    assert!(matches!(
        err,
        SexParserError::ExpectedWhitespace { ch: '.', .. }
    ));
}

#[test]
fn parse_number_with_letters_requires_whitespace() {
    let err = parse_listed("12a34").unwrap_err();
    assert!(matches!(
        err,
        SexParserError::ExpectedWhitespace { ch: 'a', .. }
    ));
}

#[test]
fn parse_adjacent_strings_requires_whitespace() {
    let err = parse_listed(r#""a""b""#).unwrap_err();
    assert!(matches!(
        err,
        SexParserError::ExpectedWhitespace { ch: '"', .. }
    ));
}

#[test]
fn parse_symbol_followed_by_list_requires_whitespace() {
    let err = parse_listed("foo(bar)").unwrap_err();
    assert!(matches!(
        err,
        SexParserError::ExpectedWhitespace { ch: '(', .. }
    ));
}

#[test]
fn parse_adjacent_lists_require_whitespace() {
    let err = parse_listed("(a)(b)").unwrap_err();
    assert!(matches!(
        err,
        SexParserError::ExpectedWhitespace { ch: '(', .. }
    ));
}

#[test]
fn negative_number_vs_symbol() {
    assert!(parse_atom("-42").unwrap().is_number());
    assert!(parse_atom("-").unwrap().is_symbol());
}


#[test]
fn parse_string_empty() {
    assert_eq!(parse_atom(r#""""#).unwrap(), Atom::string(""));
}

#[test]
fn parse_string_basic() {
    assert_eq!(parse_atom(r#""hello""#).unwrap(), Atom::string("hello"));
}

#[test]
fn parse_string_with_escaped_quote() {
    assert_eq!(parse_atom(r#""say \"hi\"""#).unwrap(), Atom::string("say \"hi\""));
}

#[test]
fn parse_string_with_escaped_backslash() {
    assert_eq!(parse_atom(r#""a\\b""#).unwrap(), Atom::string("a\\b"));
}

#[test]
fn parse_string_with_newline() {
    assert_eq!(parse_atom(r#""a\nb""#).unwrap(), Atom::string("a\nb"));
}

#[test]
fn parse_string_with_tab() {
    assert_eq!(parse_atom(r#""a\tb""#).unwrap(), Atom::string("a\tb"));
}

#[test]
fn parse_string_with_carriage_return() {
    assert_eq!(parse_atom(r#""a\rb""#).unwrap(), Atom::string("a\rb"));
}

#[test]
fn parse_string_with_spaces() {
    assert_eq!(parse_atom(r#""hello world""#).unwrap(), Atom::string("hello world"));
}

#[test]
fn strings_are_text() {
    let s = parse_atom(r#""hello""#).unwrap();
    assert!(s.is_text());
    assert!(!s.is_symbol());
    assert!(!s.is_keyword());
    assert_eq!(s.as_text().unwrap().contents, "hello");
}

#[test]
fn parse_unterminated_string() {
    let r = parse_listed(r#""hello"#);
    assert!(matches!(r, Err(SexParserError::UnterminatedString { .. })));
}

#[test]
fn parse_unterminated_string_after_escape() {
    let r = parse_listed(r#""hello\"#);
    assert!(matches!(r, Err(SexParserError::UnterminatedString { .. })));
}

#[test]
fn parse_invalid_escape() {
    let r = parse_listed(r#""\q""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidEscape { ch: 'q', .. })
    ));
}

#[test]
fn parse_escape_null() {
    assert_eq!(parse_atom(r#""a\0b""#).unwrap(), Atom::string("a\0b"));
}

#[test]
fn parse_escape_hex_ascii() {
    assert_eq!(parse_atom(r#""\x41""#).unwrap(), Atom::string("A"));
}

#[test]
fn parse_escape_hex_lowercase() {
    assert_eq!(parse_atom(r#""\x7f""#).unwrap(), Atom::string("\u{7f}"));
}

#[test]
fn parse_escape_hex_too_large() {
    let r = parse_listed(r#""\x80""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidHexEscape { .. })
    ));
}

#[test]
fn parse_escape_hex_missing_digit() {
    let r = parse_listed(r#""\x4""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidHexEscape { .. })
    ));
}

#[test]
fn parse_escape_hex_invalid_char() {
    let r = parse_listed(r#""\xzz""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidHexEscape { .. })
    ));
}

#[test]
fn parse_escape_unicode() {
    assert_eq!(parse_atom(r#""\u{7FFF}""#).unwrap(), Atom::string("\u{7FFF}"));
}

#[test]
fn parse_escape_unicode_empty() {
    let r = parse_listed(r#""\u{}""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidUnicodeEscape { .. })
    ));
}

#[test]
fn parse_escape_unicode_missing_brace() {
    let r = parse_listed(r#""\u{41""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidUnicodeEscape { .. })
    ));
}

#[test]
fn parse_escape_unicode_surrogate() {
    let r = parse_listed(r#""\u{D800}""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidUnicodeEscape { .. })
    ));
}

#[test]
fn parse_escape_unicode_too_large() {
    let r = parse_listed(r#""\u{110000}""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidUnicodeEscape { .. })
    ));
}

#[test]
fn parse_escape_unicode_no_brace() {
    let r = parse_listed(r#""\u41""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidUnicodeEscape { .. })
    ));
}


#[test]
fn parse_keyword_basic() {
    assert_eq!(parse_atom(":foo").unwrap(), Atom::keyword("foo"));
}

#[test]
fn parse_keyword_with_hyphen() {
    assert_eq!(parse_atom(":foo-bar").unwrap(), Atom::keyword("foo-bar"));
}

#[test]
fn parse_empty_keyword() {
    let r = parse_listed(":");
    assert!(matches!(r, Err(SexParserError::EmptyKeyword { .. })));
}


#[test]
fn parse_empty_list() {
    assert_eq!(parse_atom("()").unwrap(), Atom::List(vec![]));
}

#[test]
fn parse_list_one_element() {
    assert_eq!(
        parse_atom("(42)").unwrap(),
        Atom::List(vec![Atom::Number(Number::Integer(42))])
    );
}

#[test]
fn parse_list_multiple_elements() {
    assert_eq!(
        parse_atom("(a b c)").unwrap(),
        Atom::List(vec![
            Atom::symbol("a"),
            Atom::symbol("b"),
            Atom::symbol("c"),
        ])
    );
}

#[test]
fn parse_nested_list() {
    assert_eq!(
        parse_atom("(a (b c))").unwrap(),
        Atom::List(vec![
            Atom::symbol("a"),
            Atom::List(vec![Atom::symbol("b"), Atom::symbol("c")]),
        ])
    );
}

#[test]
fn parse_deeply_nested_list() {
    let result = parse_atom("((((nil))))").unwrap();
    assert_eq!(
        result,
        Atom::List(vec![Atom::List(vec![Atom::List(vec![Atom::List(vec![
            Atom::Nil
        ])])])])
    );
}

#[test]
fn parse_unterminated_list() {
    let r = parse_listed("(a b");
    assert!(matches!(r, Err(SexParserError::UnterminatedList { .. })));
}

#[test]
fn parse_unterminated_list_empty() {
    let r = parse_listed("(");
    assert!(matches!(r, Err(SexParserError::UnterminatedList { .. })));
}


#[test]
fn parse_with_leading_whitespace() {
    assert_eq!(parse_atom("  42").unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_with_trailing_whitespace() {
    assert_eq!(parse_atom("42  ").unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_with_tabs_and_newlines() {
    let input = "\n\t(a\n\tb\n)";
    assert_eq!(
        parse_atom(input).unwrap(),
        Atom::List(vec![Atom::symbol("a"), Atom::symbol("b")])
    );
}


#[test]
fn parse_multiple_atoms() {
    let atoms = parse_listed("a b c").unwrap();
    assert_eq!(
        atoms,
        vec![
            Atom::symbol("a"),
            Atom::symbol("b"),
            Atom::symbol("c"),
        ]
    );
}

#[test]
fn parse_multiple_mixed() {
    let atoms = parse_listed("42 :key \"str\"").unwrap();
    assert_eq!(
        atoms,
        vec![
            Atom::Number(Number::Integer(42)),
            Atom::keyword("key"),
            Atom::string("str"),
        ]
    );
}

#[test]
fn parse_empty_input() {
    let atoms = parse_listed("").unwrap();
    assert!(atoms.is_empty());
}

#[test]
fn parse_whitespace_only() {
    let atoms = parse_listed("  \n\t  ").unwrap();
    assert!(atoms.is_empty());
}


#[test]
fn parse_atom_single() {
    assert_eq!(parse_atom("42").unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_atom_errors_on_trailing() {
    assert!(parse_atom("42 foo").is_err());
}


#[test]
fn error_position_tracked() {
    let err = parse_listed("(\n :\n)").unwrap_err();
    match err {
        SexParserError::EmptyKeyword { pos } => {
            assert_eq!(pos.line, 2);
            assert_eq!(pos.col, 3);
        }
        _ => panic!("expected EmptyKeyword, got {err}"),
    }
}

#[test]
fn error_position_in_nested_list() {
    let err = parse_listed("(a (b :c)").unwrap_err();
    match err {
        SexParserError::UnterminatedList { pos } => {
            assert_eq!(pos.line, 1);
            assert_eq!(pos.col, 10);
        }
        _ => panic!("expected UnterminatedList, got {err}"),
    }
}

#[test]
fn error_unexpected_eof() {
    let err = parse_listed("(").unwrap_err();
    assert!(matches!(err, SexParserError::UnterminatedList { .. }));
}

#[test]
fn error_unexpected_char() {
    let err = parse_listed("@").unwrap_err();
    assert!(matches!(
        err,
        SexParserError::UnexpectedChar { ch: '@', .. }
    ));
}


#[test]
fn parse_mixed_keywords_and_symbols_in_list() {
    let input = "(deftexture foo :src (path \"bar.png\") :x 0 :y 0)";
    let parsed = parse_atom(input).unwrap();
    assert!(parsed.is_list());
}

#[test]
fn parse_keyword_atom_near_list_boundary() {
    let parsed = parse_atom("(:tag)").unwrap();
    let list = parsed.as_list().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0], Atom::keyword("tag"));
}

#[test]
fn nil_in_list() {
    let atom = parse_atom("(nil)").unwrap();
    let list = atom.as_list().unwrap();
    assert_eq!(list[0], Atom::Nil);
}
