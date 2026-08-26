use sex::{Atom, Number, SexParserAtomError, SexParserError, SexParserErrorKind, parse_expression_str, parse_exprlist_str};

#[test]
fn parse_bare_symbol() {
    assert_eq!(parse_expression_str("hello").unwrap(), Atom::symbol("hello"));
}

#[test]
fn parse_symbol_with_hyphen() {
    assert_eq!(parse_expression_str("foo-bar").unwrap(), Atom::symbol("foo-bar"));
}

#[test]
fn parse_symbol_with_underscore() {
    assert_eq!(parse_expression_str("foo_bar").unwrap(), Atom::symbol("foo_bar"));
}

#[test]
fn parse_symbol_with_slash() {
    assert_eq!(parse_expression_str("foo/bar").unwrap(), Atom::symbol("foo/bar"));
}

#[test]
fn parse_symbol_with_dot() {
    assert_eq!(parse_expression_str("foo.bar").unwrap(), Atom::symbol("foo.bar"));
}

#[test]
fn parse_symbol_with_star() {
    assert_eq!(parse_expression_str("foo*bar").unwrap(), Atom::symbol("foo*bar"));
}

#[test]
fn parse_symbol_with_plus() {
    assert_eq!(parse_expression_str("foo+bar").unwrap(), Atom::symbol("foo+bar"));
}

#[test]
fn parse_symbol_with_exclamation() {
    assert_eq!(parse_expression_str("foo!").unwrap(), Atom::symbol("foo!"));
}

#[test]
fn parse_symbol_with_question() {
    assert_eq!(parse_expression_str("foo?").unwrap(), Atom::symbol("foo?"));
}

#[test]
fn parse_symbol_with_angle_brackets() {
    assert_eq!(parse_expression_str("foo<bar>").unwrap(), Atom::symbol("foo<bar>"));
}

#[test]
fn parse_symbol_with_equals() {
    assert_eq!(parse_expression_str("foo=bar").unwrap(), Atom::symbol("foo=bar"));
}

#[test]
fn parse_symbol_with_ampersand() {
    assert_eq!(parse_expression_str("foo&bar").unwrap(), Atom::symbol("foo&bar"));
}

#[test]
fn parse_symbol_with_percent() {
    assert_eq!(parse_expression_str("foo%bar").unwrap(), Atom::symbol("foo%bar"));
}

#[test]
fn parse_symbol_mixed() {
    let s = parse_expression_str("a-b/c.d_e+f!?").unwrap();
    assert_eq!(s, Atom::symbol("a-b/c.d_e+f!?"));
}

#[test]
fn parse_symbol_starting_with_dot() {
    assert_eq!(parse_expression_str(".foo").unwrap(), Atom::symbol(".foo"));
}

#[test]
fn parse_symbol_single_dot() {
    assert_eq!(parse_expression_str(".").unwrap(), Atom::symbol("."));
}

#[test]
fn parse_symbol_single_hyphen() {
    assert_eq!(parse_expression_str("-").unwrap(), Atom::symbol("-"));
}

#[test]
fn parse_t_is_a_symbol() {
    assert_eq!(parse_expression_str("t").unwrap(), Atom::symbol("t"));
}

#[test]
fn parse_true() {
    assert_eq!(parse_expression_str("true").unwrap(), Atom::True);
}

#[test]
fn parse_false() {
    assert_eq!(parse_expression_str("false").unwrap(), Atom::False);
}

#[test]
fn parse_true_in_list() {
    let atoms = parse_exprlist_str("(true)").unwrap();
    assert_eq!(atoms, vec![Atom::List(vec![Atom::True])]);
}

#[test]
fn parse_false_in_list() {
    let atoms = parse_exprlist_str("(false)").unwrap();
    assert_eq!(atoms, vec![Atom::List(vec![Atom::False])]);
}

#[test]
fn parse_logic_values_listed() {
    let atoms = parse_exprlist_str("true false nil").unwrap();
    assert_eq!(atoms, vec![Atom::True, Atom::False, Atom::Nil]);
}

#[test]
fn parse_logic_values_in_list() {
    let atoms = parse_exprlist_str("(true false nil)").unwrap();
    assert_eq!(
        atoms,
        vec![Atom::List(vec![Atom::True, Atom::False, Atom::Nil])]
    );
}

#[test]
fn parse_logic_case_sensitive() {
    assert_eq!(parse_expression_str("True").unwrap(), Atom::symbol("True"));
    assert_eq!(parse_expression_str("TRUE").unwrap(), Atom::symbol("TRUE"));
    assert_eq!(parse_expression_str("False").unwrap(), Atom::symbol("False"));
    assert_eq!(parse_expression_str("FALSE").unwrap(), Atom::symbol("FALSE"));
}

#[test]
fn parse_logic_prefixes_are_symbols() {
    assert_eq!(parse_expression_str("truest").unwrap(), Atom::symbol("truest"));
    assert_eq!(parse_expression_str("falsey").unwrap(), Atom::symbol("falsey"));
}

#[test]
fn parse_nil() {
    assert_eq!(parse_expression_str("nil").unwrap(), Atom::Nil);
}

#[test]
fn parse_nil_in_list() {
    let atoms = parse_exprlist_str("(nil)").unwrap();
    assert_eq!(atoms, vec![Atom::List(vec![Atom::Nil])]);
}


#[test]
fn parse_integer_zero() {
    assert_eq!(parse_expression_str("0").unwrap(), Atom::Number(Number::Integer(0)));
}

#[test]
fn parse_integer_positive() {
    assert_eq!(parse_expression_str("42").unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_integer_negative() {
    assert_eq!(parse_expression_str("-42").unwrap(), Atom::Number(Number::Integer(-42)));
}

#[test]
fn parse_integer_max() {
    assert_eq!(
        parse_expression_str("9223372036854775807").unwrap(),
        Atom::Number(Number::Integer(9223372036854775807))
    );
}

#[test]
fn parse_integer_min() {
    assert_eq!(
        parse_expression_str("-9223372036854775808").unwrap(),
        Atom::Number(Number::Integer(-9223372036854775808))
    );
}

#[test]
fn parse_negative_sign_not_number() {
    assert_eq!(parse_expression_str("-").unwrap(), Atom::symbol("-"));
}


#[test]
fn parse_float_simple() {
    assert_eq!(parse_expression_str("3.14").unwrap(), Atom::Number(Number::Float(3.14)));
}

#[test]
fn parse_float_negative() {
    assert_eq!(parse_expression_str("-2.5").unwrap(), Atom::Number(Number::Float(-2.5)));
}

#[test]
fn parse_float_zero() {
    assert_eq!(parse_expression_str("0.0").unwrap(), Atom::Number(Number::Float(0.0)));
}

#[test]
fn parse_trailing_dot_is_invalid() {
    let err = parse_expression_str("42.").unwrap_err();
    assert!(matches!(
        err,
        SexParserAtomError::Generic(SexParserError { kind: SexParserErrorKind::InvalidNumber, .. })
    ));
}

#[test]
fn parse_double_dot_requires_whitespace() {
    let err = parse_exprlist_str("1.2.3").unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace { ch: '.' }, .. }
    ));
}

#[test]
fn parse_number_with_letters_requires_whitespace() {
    let err = parse_exprlist_str("12a34").unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace { ch: 'a' }, .. }
    ));
}

#[test]
fn parse_leading_zeros_invalid() {
    assert!(parse_expression_str("007").is_err());
    assert!(parse_expression_str("-01").is_err());
    assert!(parse_expression_str("00").is_err());
}

#[test]
fn parse_zero_forms() {
    assert_eq!(parse_expression_str("0").unwrap(), Atom::Number(Number::Integer(0)));
    assert_eq!(parse_expression_str("-0").unwrap(), Atom::Number(Number::Integer(0)));
    assert_eq!(
        parse_expression_str("0.5").unwrap(),
        Atom::Number(Number::Float(0.5))
    );
    assert_eq!(
        parse_expression_str("-0.5").unwrap(),
        Atom::Number(Number::Float(-0.5))
    );
}

#[test]
fn parse_exponent_basic() {
    assert_eq!(
        parse_expression_str("1e5").unwrap(),
        Atom::Number(Number::Float(100000.0))
    );
    assert_eq!(
        parse_expression_str("1E5").unwrap(),
        Atom::Number(Number::Float(100000.0))
    );
    assert_eq!(
        parse_expression_str("1e+5").unwrap(),
        Atom::Number(Number::Float(100000.0))
    );
    assert_eq!(
        parse_expression_str("1e-5").unwrap(),
        Atom::Number(Number::Float(0.00001))
    );
    assert_eq!(
        parse_expression_str("-2.5e2").unwrap(),
        Atom::Number(Number::Float(-250.0))
    );
}

#[test]
fn parse_exponent_is_float_even_if_integral() {
    assert_eq!(
        parse_expression_str("1e0").unwrap(),
        Atom::Number(Number::Float(1.0))
    );
}

#[test]
fn parse_exponent_malformed() {
    for input in ["1e", "1e+", "1e-", "1.e5", "42."] {
        let err = parse_expression_str(input).unwrap_err();
        assert!(
            matches!(err, SexParserAtomError::Generic(SexParserError { kind: SexParserErrorKind::InvalidNumber, .. })),
            "expected InvalidNumber for {input}, got {err:?}"
        );
    }
}

#[test]
fn parse_integer_overflow_is_invalid_number() {
    let err = parse_expression_str("99999999999999999999").unwrap_err();
    match err {
        SexParserAtomError::Generic(SexParserError { kind: SexParserErrorKind::InvalidNumber, pos, .. }) => {
            assert_eq!(pos.line, 1);
            assert_eq!(pos.col, 1);
        }
        other => panic!("expected InvalidNumber, got {other:?}"),
    }
}

#[test]
fn parse_adjacent_strings_requires_whitespace() {
    let err = parse_exprlist_str(r#""a""b""#).unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace { ch: '"' }, .. }
    ));
}

#[test]
fn parse_symbol_followed_by_list_requires_whitespace() {
    let err = parse_exprlist_str("foo(bar)").unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace { ch: '(' }, .. }
    ));
}

#[test]
fn parse_adjacent_lists_require_whitespace() {
    let err = parse_exprlist_str("(a)(b)").unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace { ch: '(' }, .. }
    ));
}

#[test]
fn negative_number_vs_symbol() {
    assert!(parse_expression_str("-42").unwrap().is_number());
    assert!(parse_expression_str("-").unwrap().is_symbol());
}


#[test]
fn parse_string_empty() {
    assert_eq!(parse_expression_str(r#""""#).unwrap(), Atom::string(""));
}

#[test]
fn parse_string_basic() {
    assert_eq!(parse_expression_str(r#""hello""#).unwrap(), Atom::string("hello"));
}

#[test]
fn parse_string_with_escaped_quote() {
    assert_eq!(parse_expression_str(r#""say \"hi\"""#).unwrap(), Atom::string("say \"hi\""));
}

#[test]
fn parse_string_with_escaped_backslash() {
    assert_eq!(parse_expression_str(r#""a\\b""#).unwrap(), Atom::string("a\\b"));
}

#[test]
fn parse_string_with_newline() {
    assert_eq!(parse_expression_str(r#""a\nb""#).unwrap(), Atom::string("a\nb"));
}

#[test]
fn parse_string_with_tab() {
    assert_eq!(parse_expression_str(r#""a\tb""#).unwrap(), Atom::string("a\tb"));
}

#[test]
fn parse_string_with_carriage_return() {
    assert_eq!(parse_expression_str(r#""a\rb""#).unwrap(), Atom::string("a\rb"));
}

#[test]
fn parse_string_with_spaces() {
    assert_eq!(parse_expression_str(r#""hello world""#).unwrap(), Atom::string("hello world"));
}

#[test]
fn strings_are_text() {
    let s = parse_expression_str(r#""hello""#).unwrap();
    assert!(s.is_text());
    assert!(!s.is_symbol());
    assert!(!s.is_keyword());
    assert_eq!(s.as_text().unwrap().contents, "hello");
}

#[test]
fn parse_unterminated_string() {
    let r = parse_exprlist_str(r#""hello"#);
    assert!(matches!(r,         Err(SexParserError { kind: SexParserErrorKind::UnterminatedString, .. })));
}

#[test]
fn parse_unterminated_string_after_escape() {
    let r = parse_exprlist_str(r#""hello\"#);
    assert!(matches!(r,         Err(SexParserError { kind: SexParserErrorKind::UnterminatedString, .. })));
}

#[test]
fn parse_invalid_escape() {
    let r = parse_exprlist_str(r#""\q""#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedStringEscape { ch: 'q' }, .. })
    ));
}

#[test]
fn parse_escape_null() {
    assert_eq!(parse_expression_str(r#""a\0b""#).unwrap(), Atom::string("a\0b"));
}

#[test]
fn parse_escape_hex_ascii() {
    assert_eq!(parse_expression_str(r#""\x41""#).unwrap(), Atom::string("A"));
}

#[test]
fn parse_escape_hex_lowercase() {
    assert_eq!(parse_expression_str(r#""\x7f""#).unwrap(), Atom::string("\u{7f}"));
}

#[test]
fn parse_escape_hex_full_byte_range() {
    assert_eq!(parse_expression_str(r#""\x80""#).unwrap(), Atom::string("\u{80}"));
    assert_eq!(parse_expression_str(r#""\xFF""#).unwrap(), Atom::string("ÿ"));
    assert_eq!(parse_expression_str(r#""\xff""#).unwrap(), Atom::string("\u{ff}"));
    assert_eq!(
        parse_expression_str(r#""\xC3\xBF""#).unwrap(),
        Atom::string("Ã¿")
    );
}

#[test]
fn parse_escape_hex_missing_digit() {
    let r = parse_exprlist_str(r#""\x4""#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedHexEscape { value: _ }, .. })
    ));
}

#[test]
fn parse_escape_hex_invalid_char() {
    let r = parse_exprlist_str(r#""\xzz""#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedHexEscape { value: _ }, .. })
    ));
}

#[test]
fn parse_escape_unicode() {
    assert_eq!(parse_expression_str(r#""\u{7FFF}""#).unwrap(), Atom::string("\u{7FFF}"));
}

#[test]
fn parse_escape_unicode_empty() {
    let r = parse_exprlist_str(r#""\u{}""#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedUnicodeEscape { value: _ }, .. })
    ));
}

#[test]
fn parse_escape_unicode_missing_brace() {
    let r = parse_exprlist_str(r#""\u{41""#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedUnicodeEscape { value: _ }, .. })
    ));
}

#[test]
fn parse_escape_unicode_surrogate() {
    let r = parse_exprlist_str(r#""\u{D800}""#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::InvalidUnicodeChar {
            value: 0xD800,
            ..
        }, .. })
    ));
}

#[test]
fn parse_escape_unicode_too_large() {
    let r = parse_exprlist_str(r#""\u{110000}""#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::InvalidUnicodeChar {
            value: 0x110000,
            ..
        }, .. })
    ));
}

#[test]
fn parse_escape_unicode_boundary_scalars() {
    assert_eq!(
        parse_expression_str(r#""\u{D7FF}""#).unwrap(),
        Atom::string("\u{D7FF}")
    );
    assert_eq!(
        parse_expression_str(r#""\u{E000}""#).unwrap(),
        Atom::string("\u{E000}")
    );
    assert_eq!(
        parse_expression_str(r#""\u{10FFFF}""#).unwrap(),
        Atom::string("\u{10FFFF}")
    );
}

#[test]
fn parse_escape_unicode_no_brace() {
    let r = parse_exprlist_str(r#""\u41""#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedUnicodeEscape { value: _ }, .. })
    ));
}


#[test]
fn parse_barred_symbol_basic() {
    assert_eq!(
        parse_expression_str("|hello world|").unwrap(),
        Atom::symbol("hello world")
    );
}

#[test]
fn parse_barred_symbol_empty() {
    assert_eq!(parse_expression_str("||").unwrap(), Atom::symbol(""));
}

#[test]
fn parse_barred_symbol_unicode() {
    assert_eq!(parse_expression_str("|日本語|").unwrap(), Atom::symbol("日本語"));
}

#[test]
fn parse_barred_symbol_multiline() {
    assert_eq!(
        parse_expression_str("|foo\nbar|").unwrap(),
        Atom::symbol("foo\nbar")
    );
}

#[test]
fn parse_barred_symbol_contains_delimiters() {
    assert_eq!(parse_expression_str("|(foo)|").unwrap(), Atom::symbol("(foo)"));
    assert_eq!(
        parse_expression_str(r#"|"quoted"|"#).unwrap(),
        Atom::symbol("\"quoted\"")
    );
    assert_eq!(parse_expression_str("|;comment|").unwrap(), Atom::symbol(";comment"));
    assert_eq!(parse_expression_str("|:kw|").unwrap(), Atom::symbol(":kw"));
}

#[test]
fn parse_barred_symbol_is_literal() {
    assert_eq!(parse_expression_str("|nil|").unwrap(), Atom::symbol("nil"));
    assert_ne!(parse_expression_str("|nil|").unwrap(), Atom::Nil);
    assert_eq!(parse_expression_str("|true|").unwrap(), Atom::symbol("true"));
    assert_ne!(parse_expression_str("|true|").unwrap(), Atom::True);
    assert_eq!(parse_expression_str("|123|").unwrap(), Atom::symbol("123"));
}

#[test]
fn parse_barred_symbol_in_list() {
    let atoms = parse_exprlist_str("(a |b c| d)").unwrap();
    assert_eq!(
        atoms,
        vec![Atom::List(vec![
            Atom::symbol("a"),
            Atom::symbol("b c"),
            Atom::symbol("d"),
        ])]
    );
}

#[test]
fn parse_barred_symbol_escapes() {
    assert_eq!(parse_expression_str("|a\\|b|").unwrap(), Atom::symbol("a|b"));
    assert_eq!(parse_expression_str("|a\\\\b|").unwrap(), Atom::symbol("a\\b"));
    assert_eq!(parse_expression_str(r#"|a\"b|"#).unwrap(), Atom::symbol("a\"b"));
    assert_eq!(parse_expression_str("|a\\nb|").unwrap(), Atom::symbol("a\nb"));
    assert_eq!(parse_expression_str("|a\\tb|").unwrap(), Atom::symbol("a\tb"));
    assert_eq!(parse_expression_str("|a\\rb|").unwrap(), Atom::symbol("a\rb"));
    assert_eq!(parse_expression_str("|a\\0b|").unwrap(), Atom::symbol("a\u{0}b"));
    assert_eq!(parse_expression_str("|\\x41|").unwrap(), Atom::symbol("A"));
    assert_eq!(parse_expression_str("|\\u{1F600}|").unwrap(), Atom::symbol("\u{1F600}"));
}

#[test]
fn parse_barred_symbol_unknown_escape() {
    let r = parse_exprlist_str("|ab\\qc|");
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedBarEscape { ch: 'q' }, .. })
    ));
}

#[test]
fn parse_barred_symbol_bad_hex_shares_error() {
    let r = parse_exprlist_str("|\\xzz|");
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::MalformedHexEscape { value: _ }, .. })));
}

#[test]
fn parse_barred_symbol_surrogate_shares_error() {
    let r = parse_exprlist_str("|\\u{D800}|");
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::InvalidUnicodeChar { value: 0xD800, .. }, .. })
    ));
}

#[test]
fn parse_barred_symbol_unterminated() {
    let r = parse_exprlist_str("|abc");
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedBarSymbol, .. })));
}

#[test]
fn parse_barred_symbol_unterminated_after_escape() {
    let r = parse_exprlist_str("|abc\\");
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedBarSymbol, .. })));
}

#[test]
fn bare_pipe_in_symbol_rejected() {
    assert!(parse_expression_str("foo|bar").is_err());
}

#[test]
fn parse_barred_keyword_basic() {
    assert_eq!(
        parse_expression_str(":|foo bar|").unwrap(),
        Atom::keyword("foo bar")
    );
}

#[test]
fn parse_barred_keyword_empty() {
    assert_eq!(parse_expression_str(":||").unwrap(), Atom::keyword(""));
}

#[test]
fn parse_barred_keyword_unicode_and_multiline() {
    assert_eq!(parse_expression_str(":|日本 語|").unwrap(), Atom::keyword("日本 語"));
    assert_eq!(parse_expression_str(":|a\nb|").unwrap(), Atom::keyword("a\nb"));
}

#[test]
fn parse_barred_keyword_escapes() {
    assert_eq!(
        parse_expression_str(r#" :|a\|b| "#).unwrap(),
        Atom::keyword("a|b")
    );
    assert_eq!(parse_expression_str(r#" :|\x41| "#).unwrap(), Atom::keyword("A"));
    assert_eq!(
        parse_expression_str(r#" :|\u{1F600}| "#).unwrap(),
        Atom::keyword("\u{1F600}")
    );
}

#[test]
fn parse_barred_keyword_is_literal() {
    assert_eq!(parse_expression_str(":|nil|").unwrap(), Atom::keyword("nil"));
    assert_ne!(parse_expression_str(":|nil|").unwrap(), Atom::Nil);
    assert_eq!(parse_expression_str(":|true|").unwrap(), Atom::keyword("true"));
    assert_ne!(parse_expression_str(":|true|").unwrap(), Atom::True);
    assert_eq!(parse_expression_str(":|123|").unwrap(), Atom::keyword("123"));
}

#[test]
fn parse_barred_keyword_contains_delimiters() {
    assert_eq!(parse_expression_str(":|(x)|").unwrap(), Atom::keyword("(x)"));
    assert_eq!(parse_expression_str(r#" :|"s"| "#).unwrap(), Atom::keyword("\"s\""));
}

#[test]
fn parse_barred_keyword_unknown_escape() {
    let r = parse_exprlist_str(r#":|ab\qc|"#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedBarEscape { ch: 'q' }, .. })
    ));
}

#[test]
fn parse_barred_keyword_shares_hex_and_surrogate_errors() {
    let r = parse_exprlist_str(r#":|\xzz|"#);
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::MalformedHexEscape { value: _ }, .. })));
    let r = parse_exprlist_str(r#":|\u{D800}|"#);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::InvalidUnicodeChar { value: 0xD800, .. }, .. })
    ));
}

#[test]
fn parse_barred_keyword_unterminated() {
    let r = parse_exprlist_str(":|abc");
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedBarSymbol, .. })));
}

#[test]
fn bare_pipe_after_keyword_rejected() {
    assert!(parse_expression_str(":foo|bar|").is_err());
}


#[test]
fn parse_keyword_basic() {
    assert_eq!(parse_expression_str(":foo").unwrap(), Atom::keyword("foo"));
}

#[test]
fn parse_keyword_with_hyphen() {
    assert_eq!(parse_expression_str(":foo-bar").unwrap(), Atom::keyword("foo-bar"));
}

#[test]
fn parse_empty_keyword() {
    let r = parse_exprlist_str(":");
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::EmptyKeyword, .. })));
}


#[test]
fn parse_empty_list() {
    assert_eq!(parse_expression_str("()").unwrap(), Atom::List(vec![]));
}

#[test]
fn parse_list_one_element() {
    assert_eq!(
        parse_expression_str("(42)").unwrap(),
        Atom::List(vec![Atom::Number(Number::Integer(42))])
    );
}

#[test]
fn parse_list_multiple_elements() {
    assert_eq!(
        parse_expression_str("(a b c)").unwrap(),
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
        parse_expression_str("(a (b c))").unwrap(),
        Atom::List(vec![
            Atom::symbol("a"),
            Atom::List(vec![Atom::symbol("b"), Atom::symbol("c")]),
        ])
    );
}

#[test]
fn parse_deeply_nested_list() {
    let result = parse_expression_str("((((nil))))").unwrap();
    assert_eq!(
        result,
        Atom::List(vec![Atom::List(vec![Atom::List(vec![Atom::List(vec![
            Atom::Nil
        ])])])])
    );
}

#[test]
fn parse_unterminated_list() {
    let r = parse_exprlist_str("(a b");
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedList, .. })));
}

#[test]
fn parse_unterminated_list_empty() {
    let r = parse_exprlist_str("(");
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedList, .. })));
}


#[test]
fn parse_with_leading_whitespace() {
    assert_eq!(parse_expression_str("  42").unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_with_trailing_whitespace() {
    assert_eq!(parse_expression_str("42  ").unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_with_tabs_and_newlines() {
    let input = "\n\t(a\n\tb\n)";
    assert_eq!(
        parse_expression_str(input).unwrap(),
        Atom::List(vec![Atom::symbol("a"), Atom::symbol("b")])
    );
}


#[test]
fn parse_multiple_atoms() {
    let atoms = parse_exprlist_str("a b c").unwrap();
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
    let atoms = parse_exprlist_str("42 :key \"str\"").unwrap();
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
    let atoms = parse_exprlist_str("").unwrap();
    assert!(atoms.is_empty());
}

#[test]
fn parse_whitespace_only() {
    let atoms = parse_exprlist_str("  \n\t  ").unwrap();
    assert!(atoms.is_empty());
}


#[test]
fn parse_atom_single() {
    assert_eq!(parse_expression_str("42").unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_atom_errors_on_trailing() {
    assert!(parse_expression_str("42 foo").is_err());
}


#[test]
fn error_position_tracked() {
    let err = parse_exprlist_str("(\n :\n)").unwrap_err();
    match err {
        SexParserError { pos, kind: SexParserErrorKind::EmptyKeyword, .. } => {
            assert_eq!(pos.line, 2);
            assert_eq!(pos.col, 3);
        }
        _ => panic!("expected EmptyKeyword, got {err}"),
    }
}

#[test]
fn error_position_in_nested_list() {
    let err = parse_exprlist_str("(a (b :c)").unwrap_err();
    match err {
        SexParserError { pos, kind: SexParserErrorKind::UnterminatedList, .. } => {
            assert_eq!(pos.line, 1);
            assert_eq!(pos.col, 10);
        }
        _ => panic!("expected UnterminatedList, got {err}"),
    }
}

#[test]
fn error_unexpected_eof() {
    let err = parse_exprlist_str("(").unwrap_err();
    assert!(matches!(err, SexParserError { kind: SexParserErrorKind::UnterminatedList, .. }));
}

#[test]
fn error_unexpected_char() {
    let err = parse_exprlist_str(")").unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::UnexpectedChar { ch: ')' }, .. }
    ));
}


#[test]
fn parse_mixed_keywords_and_symbols_in_list() {
    let input = "(deftexture foo :src (path \"bar.png\") :x 0 :y 0)";
    let parsed = parse_expression_str(input).unwrap();
    assert!(parsed.is_list());
}

#[test]
fn parse_keyword_atom_near_list_boundary() {
    let parsed = parse_expression_str("(:tag)").unwrap();
    let list = parsed.as_list().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0], Atom::keyword("tag"));
}

#[test]
fn nil_in_list() {
    let atom = parse_expression_str("(nil)").unwrap();
    let list = atom.as_list().unwrap();
    assert_eq!(list[0], Atom::Nil);
}

fn assert_float_eq(input: &str, expected: f64) {
    let atom = parse_expression_str(input).unwrap();
    match atom {
        Atom::Number(Number::Float(f)) => {
            assert_eq!(f.to_bits(), expected.to_bits(),
                "parse_float({input}): got {f}, expected {expected}");
        }
        other => panic!("expected Float, got {other:?}"),
    }
}

#[test]
fn parse_float_precision_zero_point_three() {
    assert_float_eq("0.3", 0.3_f64);
}

#[test]
fn parse_float_precision_many_fractional_digits() {
    assert_float_eq("0.123456789012345678", "0.123456789012345678".parse::<f64>().unwrap());
}

#[test]
fn parse_float_precision_pi() {
    assert_float_eq("3.141592653589793238", "3.141592653589793238".parse::<f64>().unwrap());
}

#[test]
fn parse_float_precision_subnormal() {
    assert_float_eq("5e-324", 5e-324_f64);
}

#[test]
fn parse_float_precision_near_f64_max() {
    assert_float_eq("1.7976931348623157e308", "1.7976931348623157e308".parse::<f64>().unwrap());
}

#[test]
fn parse_float_precision_many_nines() {
    assert_float_eq("0.9999999999999999", "0.9999999999999999".parse::<f64>().unwrap());
}

#[test]
fn parse_float_precision_negative_zero() {
    let atom = parse_expression_str("-0.0").unwrap();
    match atom {
        Atom::Number(Number::Float(f)) => {
            assert_eq!(f, -0.0_f64);
            assert!(f.is_sign_negative());
        }
        other => panic!("expected Float, got {other:?}"),
    }
}

#[test]
fn parse_float_very_small_exponent() {
    assert_float_eq("1e-308", "1e-308".parse::<f64>().unwrap());
}
