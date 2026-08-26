use sex::{Atom, Number, SexParserAtomError, SexParserError, SexParserErrorKind, parse_expression_str, parse_exprlist_str};

#[test]
fn parse_bare_symbol() {
    assert_eq!(parse_expression_str("hello", None).unwrap(), Atom::symbol("hello"));
}

#[test]
fn parse_symbol_with_hyphen() {
    assert_eq!(parse_expression_str("foo-bar", None).unwrap(), Atom::symbol("foo-bar"));
}

#[test]
fn parse_symbol_with_underscore() {
    assert_eq!(parse_expression_str("foo_bar", None).unwrap(), Atom::symbol("foo_bar"));
}

#[test]
fn parse_symbol_with_slash() {
    assert_eq!(parse_expression_str("foo/bar", None).unwrap(), Atom::symbol("foo/bar"));
}

#[test]
fn parse_symbol_with_dot() {
    assert_eq!(parse_expression_str("foo.bar", None).unwrap(), Atom::symbol("foo.bar"));
}

#[test]
fn parse_symbol_with_star() {
    assert_eq!(parse_expression_str("foo*bar", None).unwrap(), Atom::symbol("foo*bar"));
}

#[test]
fn parse_symbol_with_plus() {
    assert_eq!(parse_expression_str("foo+bar", None).unwrap(), Atom::symbol("foo+bar"));
}

#[test]
fn parse_symbol_with_exclamation() {
    assert_eq!(parse_expression_str("foo!", None).unwrap(), Atom::symbol("foo!"));
}

#[test]
fn parse_symbol_with_question() {
    assert_eq!(parse_expression_str("foo?", None).unwrap(), Atom::symbol("foo?"));
}

#[test]
fn parse_symbol_with_angle_brackets() {
    assert_eq!(parse_expression_str("foo<bar>", None).unwrap(), Atom::symbol("foo<bar>"));
}

#[test]
fn parse_symbol_with_equals() {
    assert_eq!(parse_expression_str("foo=bar", None).unwrap(), Atom::symbol("foo=bar"));
}

#[test]
fn parse_symbol_with_ampersand() {
    assert_eq!(parse_expression_str("foo&bar", None).unwrap(), Atom::symbol("foo&bar"));
}

#[test]
fn parse_symbol_with_percent() {
    assert_eq!(parse_expression_str("foo%bar", None).unwrap(), Atom::symbol("foo%bar"));
}

#[test]
fn parse_symbol_mixed() {
    let s = parse_expression_str("a-b/c.d_e+f!?", None).unwrap();
    assert_eq!(s, Atom::symbol("a-b/c.d_e+f!?"));
}

#[test]
fn parse_symbol_starting_with_dot() {
    assert_eq!(parse_expression_str(".foo", None).unwrap(), Atom::symbol(".foo"));
}

#[test]
fn parse_symbol_single_dot() {
    assert_eq!(parse_expression_str(".", None).unwrap(), Atom::symbol("."));
}

#[test]
fn parse_symbol_single_hyphen() {
    assert_eq!(parse_expression_str("-", None).unwrap(), Atom::symbol("-"));
}

#[test]
fn parse_t_is_a_symbol() {
    assert_eq!(parse_expression_str("t", None).unwrap(), Atom::symbol("t"));
}

#[test]
fn parse_true() {
    assert_eq!(parse_expression_str("true", None).unwrap(), Atom::True);
}

#[test]
fn parse_false() {
    assert_eq!(parse_expression_str("false", None).unwrap(), Atom::False);
}

#[test]
fn parse_true_in_list() {
    let atoms = parse_exprlist_str("(true)", None).unwrap();
    assert_eq!(atoms, vec![Atom::List(vec![Atom::True])]);
}

#[test]
fn parse_false_in_list() {
    let atoms = parse_exprlist_str("(false)", None).unwrap();
    assert_eq!(atoms, vec![Atom::List(vec![Atom::False])]);
}

#[test]
fn parse_logic_values_listed() {
    let atoms = parse_exprlist_str("true false nil", None).unwrap();
    assert_eq!(atoms, vec![Atom::True, Atom::False, Atom::Nil]);
}

#[test]
fn parse_logic_values_in_list() {
    let atoms = parse_exprlist_str("(true false nil)", None).unwrap();
    assert_eq!(
        atoms,
        vec![Atom::List(vec![Atom::True, Atom::False, Atom::Nil])]
    );
}

#[test]
fn parse_logic_case_sensitive() {
    assert_eq!(parse_expression_str("True", None).unwrap(), Atom::symbol("True"));
    assert_eq!(parse_expression_str("TRUE", None).unwrap(), Atom::symbol("TRUE"));
    assert_eq!(parse_expression_str("False", None).unwrap(), Atom::symbol("False"));
    assert_eq!(parse_expression_str("FALSE", None).unwrap(), Atom::symbol("FALSE"));
}

#[test]
fn parse_logic_prefixes_are_symbols() {
    assert_eq!(parse_expression_str("truest", None).unwrap(), Atom::symbol("truest"));
    assert_eq!(parse_expression_str("falsey", None).unwrap(), Atom::symbol("falsey"));
}

#[test]
fn parse_nil() {
    assert_eq!(parse_expression_str("nil", None).unwrap(), Atom::Nil);
}

#[test]
fn parse_nil_in_list() {
    let atoms = parse_exprlist_str("(nil)", None).unwrap();
    assert_eq!(atoms, vec![Atom::List(vec![Atom::Nil])]);
}


#[test]
fn parse_integer_zero() {
    assert_eq!(parse_expression_str("0", None).unwrap(), Atom::Number(Number::Integer(0)));
}

#[test]
fn parse_integer_positive() {
    assert_eq!(parse_expression_str("42", None).unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_integer_negative() {
    assert_eq!(parse_expression_str("-42", None).unwrap(), Atom::Number(Number::Integer(-42)));
}

#[test]
fn parse_integer_max() {
    assert_eq!(
        parse_expression_str("9223372036854775807", None).unwrap(),
        Atom::Number(Number::Integer(9223372036854775807))
    );
}

#[test]
fn parse_integer_min() {
    assert_eq!(
        parse_expression_str("-9223372036854775808", None).unwrap(),
        Atom::Number(Number::Integer(-9223372036854775808))
    );
}

#[test]
fn parse_negative_sign_not_number() {
    assert_eq!(parse_expression_str("-", None).unwrap(), Atom::symbol("-"));
}


#[test]
fn parse_float_simple() {
    assert_eq!(parse_expression_str("3.14", None).unwrap(), Atom::Number(Number::Float(3.14)));
}

#[test]
fn parse_float_negative() {
    assert_eq!(parse_expression_str("-2.5", None).unwrap(), Atom::Number(Number::Float(-2.5)));
}

#[test]
fn parse_float_zero() {
    assert_eq!(parse_expression_str("0.0", None).unwrap(), Atom::Number(Number::Float(0.0)));
}

#[test]
fn parse_trailing_dot_is_invalid() {
    let err = parse_expression_str("42.", None).unwrap_err();
    assert!(matches!(
        err,
        SexParserAtomError::Generic(SexParserError { kind: SexParserErrorKind::InvalidNumber, .. })
    ));
}

#[test]
fn parse_double_dot_requires_whitespace() {
    let err = parse_exprlist_str("1.2.3", None).unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace('.'), .. }
    ));
}

#[test]
fn parse_number_with_letters_requires_whitespace() {
    let err = parse_exprlist_str("12a34", None).unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace('a'), .. }
    ));
}

#[test]
fn parse_leading_zeros_invalid() {
    assert!(parse_expression_str("007", None).is_err());
    assert!(parse_expression_str("-01", None).is_err());
    assert!(parse_expression_str("00", None).is_err());
}

#[test]
fn parse_zero_forms() {
    assert_eq!(parse_expression_str("0", None).unwrap(), Atom::Number(Number::Integer(0)));
    assert_eq!(parse_expression_str("-0", None).unwrap(), Atom::Number(Number::Integer(0)));
    assert_eq!(
        parse_expression_str("0.5", None).unwrap(),
        Atom::Number(Number::Float(0.5))
    );
    assert_eq!(
        parse_expression_str("-0.5", None).unwrap(),
        Atom::Number(Number::Float(-0.5))
    );
}

#[test]
fn parse_exponent_basic() {
    assert_eq!(
        parse_expression_str("1e5", None).unwrap(),
        Atom::Number(Number::Float(100000.0))
    );
    assert_eq!(
        parse_expression_str("1E5", None).unwrap(),
        Atom::Number(Number::Float(100000.0))
    );
    assert_eq!(
        parse_expression_str("1e+5", None).unwrap(),
        Atom::Number(Number::Float(100000.0))
    );
    assert_eq!(
        parse_expression_str("1e-5", None).unwrap(),
        Atom::Number(Number::Float(0.00001))
    );
    assert_eq!(
        parse_expression_str("-2.5e2", None).unwrap(),
        Atom::Number(Number::Float(-250.0))
    );
}

#[test]
fn parse_exponent_is_float_even_if_integral() {
    assert_eq!(
        parse_expression_str("1e0", None).unwrap(),
        Atom::Number(Number::Float(1.0))
    );
}

#[test]
fn parse_exponent_malformed() {
    for input in ["1e", "1e+", "1e-", "1.e5", "42."] {
        let err = parse_expression_str(input, None).unwrap_err();
        assert!(
            matches!(err, SexParserAtomError::Generic(SexParserError { kind: SexParserErrorKind::InvalidNumber, .. })),
            "expected InvalidNumber for {input}, got {err:?}"
        );
    }
}

#[test]
fn parse_integer_overflow_is_invalid_number() {
    let err = parse_expression_str("99999999999999999999", None).unwrap_err();
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
    let err = parse_exprlist_str(r#""a""b""#, None).unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace('"'), .. }
    ));
}

#[test]
fn parse_symbol_followed_by_list_requires_whitespace() {
    let err = parse_exprlist_str("foo(bar)", None).unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace('('), .. }
    ));
}

#[test]
fn parse_adjacent_lists_require_whitespace() {
    let err = parse_exprlist_str("(a)(b)", None).unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::ExpectedWhitespace('('), .. }
    ));
}

#[test]
fn negative_number_vs_symbol() {
    assert!(parse_expression_str("-42", None).unwrap().is_number());
    assert!(parse_expression_str("-", None).unwrap().is_symbol());
}


#[test]
fn parse_string_empty() {
    assert_eq!(parse_expression_str(r#""""#, None).unwrap(), Atom::string(""));
}

#[test]
fn parse_string_basic() {
    assert_eq!(parse_expression_str(r#""hello""#, None).unwrap(), Atom::string("hello"));
}

#[test]
fn parse_string_with_escaped_quote() {
    assert_eq!(parse_expression_str(r#""say \"hi\"""#, None).unwrap(), Atom::string("say \"hi\""));
}

#[test]
fn parse_string_with_escaped_backslash() {
    assert_eq!(parse_expression_str(r#""a\\b""#, None).unwrap(), Atom::string("a\\b"));
}

#[test]
fn parse_string_with_newline() {
    assert_eq!(parse_expression_str(r#""a\nb""#, None).unwrap(), Atom::string("a\nb"));
}

#[test]
fn parse_string_with_tab() {
    assert_eq!(parse_expression_str(r#""a\tb""#, None).unwrap(), Atom::string("a\tb"));
}

#[test]
fn parse_string_with_carriage_return() {
    assert_eq!(parse_expression_str(r#""a\rb""#, None).unwrap(), Atom::string("a\rb"));
}

#[test]
fn parse_string_with_spaces() {
    assert_eq!(parse_expression_str(r#""hello world""#, None).unwrap(), Atom::string("hello world"));
}

#[test]
fn strings_are_text() {
    let s = parse_expression_str(r#""hello""#, None).unwrap();
    assert!(s.is_text());
    assert!(!s.is_symbol());
    assert!(!s.is_keyword());
    assert_eq!(s.as_text().unwrap().contents, "hello");
}

#[test]
fn parse_unterminated_string() {
    let r = parse_exprlist_str(r#""hello"#, None);
    assert!(matches!(r,         Err(SexParserError { kind: SexParserErrorKind::UnterminatedString, .. })));
}

#[test]
fn parse_unterminated_string_after_escape() {
    let r = parse_exprlist_str(r#""hello\"#, None);
    assert!(matches!(r,         Err(SexParserError { kind: SexParserErrorKind::UnterminatedString, .. })));
}

#[test]
fn parse_invalid_escape() {
    let r = parse_exprlist_str(r#""\q""#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedStringEscape('q'), .. })
    ));
}

#[test]
fn parse_escape_null() {
    assert_eq!(parse_expression_str(r#""a\0b""#, None).unwrap(), Atom::string("a\0b"));
}

#[test]
fn parse_escape_hex_ascii() {
    assert_eq!(parse_expression_str(r#""\x41""#, None).unwrap(), Atom::string("A"));
}

#[test]
fn parse_escape_hex_lowercase() {
    assert_eq!(parse_expression_str(r#""\x7f""#, None).unwrap(), Atom::string("\u{7f}"));
}

#[test]
fn parse_escape_hex_full_byte_range() {
    assert_eq!(parse_expression_str(r#""\x80""#, None).unwrap(), Atom::string("\u{80}"));
    assert_eq!(parse_expression_str(r#""\xFF""#, None).unwrap(), Atom::string("ÿ"));
    assert_eq!(parse_expression_str(r#""\xff""#, None).unwrap(), Atom::string("\u{ff}"));
    assert_eq!(
        parse_expression_str(r#""\xC3\xBF""#, None).unwrap(),
        Atom::string("Ã¿")
    );
}

#[test]
fn parse_escape_hex_missing_digit() {
    let r = parse_exprlist_str(r#""\x4""#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedHexEscape(_), .. })
    ));
}

#[test]
fn parse_escape_hex_invalid_char() {
    let r = parse_exprlist_str(r#""\xzz""#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedHexEscape(_), .. })
    ));
}

#[test]
fn parse_escape_unicode() {
    assert_eq!(parse_expression_str(r#""\u{7FFF}""#, None).unwrap(), Atom::string("\u{7FFF}"));
}

#[test]
fn parse_escape_unicode_empty() {
    let r = parse_exprlist_str(r#""\u{}""#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedUnicodeEscape(_), .. })
    ));
}

#[test]
fn parse_escape_unicode_missing_brace() {
    let r = parse_exprlist_str(r#""\u{41""#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedUnicodeEscape(_), .. })
    ));
}

#[test]
fn parse_escape_unicode_surrogate() {
    let r = parse_exprlist_str(r#""\u{D800}""#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::InvalidUnicodeChar(0xD800), .. })
    ));
}

#[test]
fn parse_escape_unicode_too_large() {
    let r = parse_exprlist_str(r#""\u{110000}""#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::InvalidUnicodeChar(0x110000), .. })
    ));
}

#[test]
fn parse_escape_unicode_boundary_scalars() {
    assert_eq!(
        parse_expression_str(r#""\u{D7FF}""#, None).unwrap(),
        Atom::string("\u{D7FF}")
    );
    assert_eq!(
        parse_expression_str(r#""\u{E000}""#, None).unwrap(),
        Atom::string("\u{E000}")
    );
    assert_eq!(
        parse_expression_str(r#""\u{10FFFF}""#, None).unwrap(),
        Atom::string("\u{10FFFF}")
    );
}

#[test]
fn parse_escape_unicode_no_brace() {
    let r = parse_exprlist_str(r#""\u41""#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedUnicodeEscape(_), .. })
    ));
}


#[test]
fn parse_barred_symbol_basic() {
    assert_eq!(
        parse_expression_str("|hello world|", None).unwrap(),
        Atom::symbol("hello world")
    );
}

#[test]
fn parse_barred_symbol_empty() {
    assert_eq!(parse_expression_str("||", None).unwrap(), Atom::symbol(""));
}

#[test]
fn parse_barred_symbol_unicode() {
    assert_eq!(parse_expression_str("|日本語|", None).unwrap(), Atom::symbol("日本語"));
}

#[test]
fn parse_barred_symbol_multiline() {
    assert_eq!(
        parse_expression_str("|foo\nbar|", None).unwrap(),
        Atom::symbol("foo\nbar")
    );
}

#[test]
fn parse_barred_symbol_contains_delimiters() {
    assert_eq!(parse_expression_str("|(foo)|", None).unwrap(), Atom::symbol("(foo)"));
    assert_eq!(
        parse_expression_str(r#"|"quoted"|"#, None).unwrap(),
        Atom::symbol("\"quoted\"")
    );
    assert_eq!(parse_expression_str("|;comment|", None).unwrap(), Atom::symbol(";comment"));
    assert_eq!(parse_expression_str("|:kw|", None).unwrap(), Atom::symbol(":kw"));
}

#[test]
fn parse_barred_symbol_is_literal() {
    assert_eq!(parse_expression_str("|nil|", None).unwrap(), Atom::symbol("nil"));
    assert_ne!(parse_expression_str("|nil|", None).unwrap(), Atom::Nil);
    assert_eq!(parse_expression_str("|true|", None).unwrap(), Atom::symbol("true"));
    assert_ne!(parse_expression_str("|true|", None).unwrap(), Atom::True);
    assert_eq!(parse_expression_str("|123|", None).unwrap(), Atom::symbol("123"));
}

#[test]
fn parse_barred_symbol_in_list() {
    let atoms = parse_exprlist_str("(a |b c| d)", None).unwrap();
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
    assert_eq!(parse_expression_str("|a\\|b|", None).unwrap(), Atom::symbol("a|b"));
    assert_eq!(parse_expression_str("|a\\\\b|", None).unwrap(), Atom::symbol("a\\b"));
    assert_eq!(parse_expression_str(r#"|a\"b|"#, None).unwrap(), Atom::symbol("a\"b"));
    assert_eq!(parse_expression_str("|a\\nb|", None).unwrap(), Atom::symbol("a\nb"));
    assert_eq!(parse_expression_str("|a\\tb|", None).unwrap(), Atom::symbol("a\tb"));
    assert_eq!(parse_expression_str("|a\\rb|", None).unwrap(), Atom::symbol("a\rb"));
    assert_eq!(parse_expression_str("|a\\0b|", None).unwrap(), Atom::symbol("a\u{0}b"));
    assert_eq!(parse_expression_str("|\\x41|", None).unwrap(), Atom::symbol("A"));
    assert_eq!(parse_expression_str("|\\u{1F600}|", None).unwrap(), Atom::symbol("\u{1F600}"));
}

#[test]
fn parse_barred_symbol_unknown_escape() {
    let r = parse_exprlist_str("|ab\\qc|", None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedBarEscape('q'), .. })
    ));
}

#[test]
fn parse_barred_symbol_bad_hex_shares_error() {
    let r = parse_exprlist_str("|\\xzz|", None);
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::MalformedHexEscape(_), .. })));
}

#[test]
fn parse_barred_symbol_surrogate_shares_error() {
    let r = parse_exprlist_str("|\\u{D800}|", None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::InvalidUnicodeChar(0xD800), .. })
    ));
}

#[test]
fn parse_barred_symbol_unterminated() {
    let r = parse_exprlist_str("|abc", None);
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedBarSymbol, .. })));
}

#[test]
fn parse_barred_symbol_unterminated_after_escape() {
    let r = parse_exprlist_str("|abc\\", None);
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedBarSymbol, .. })));
}

#[test]
fn bare_pipe_in_symbol_rejected() {
    assert!(parse_expression_str("foo|bar", None).is_err());
}

#[test]
fn parse_barred_keyword_basic() {
    assert_eq!(
        parse_expression_str(":|foo bar|", None).unwrap(),
        Atom::keyword("foo bar")
    );
}

#[test]
fn parse_barred_keyword_empty() {
    assert_eq!(parse_expression_str(":||", None).unwrap(), Atom::keyword(""));
}

#[test]
fn parse_barred_keyword_unicode_and_multiline() {
    assert_eq!(parse_expression_str(":|日本 語|", None).unwrap(), Atom::keyword("日本 語"));
    assert_eq!(parse_expression_str(":|a\nb|", None).unwrap(), Atom::keyword("a\nb"));
}

#[test]
fn parse_barred_keyword_escapes() {
    assert_eq!(
        parse_expression_str(r#" :|a\|b| "#, None).unwrap(),
        Atom::keyword("a|b")
    );
    assert_eq!(parse_expression_str(r#" :|\x41| "#, None).unwrap(), Atom::keyword("A"));
    assert_eq!(
        parse_expression_str(r#" :|\u{1F600}| "#, None).unwrap(),
        Atom::keyword("\u{1F600}")
    );
}

#[test]
fn parse_barred_keyword_is_literal() {
    assert_eq!(parse_expression_str(":|nil|", None).unwrap(), Atom::keyword("nil"));
    assert_ne!(parse_expression_str(":|nil|", None).unwrap(), Atom::Nil);
    assert_eq!(parse_expression_str(":|true|", None).unwrap(), Atom::keyword("true"));
    assert_ne!(parse_expression_str(":|true|", None).unwrap(), Atom::True);
    assert_eq!(parse_expression_str(":|123|", None).unwrap(), Atom::keyword("123"));
}

#[test]
fn parse_barred_keyword_contains_delimiters() {
    assert_eq!(parse_expression_str(":|(x)|", None).unwrap(), Atom::keyword("(x)"));
    assert_eq!(parse_expression_str(r#" :|"s"| "#, None).unwrap(), Atom::keyword("\"s\""));
}

#[test]
fn parse_barred_keyword_unknown_escape() {
    let r = parse_exprlist_str(r#":|ab\qc|"#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::MalformedBarEscape('q'), .. })
    ));
}

#[test]
fn parse_barred_keyword_shares_hex_and_surrogate_errors() {
    let r = parse_exprlist_str(r#":|\xzz|"#, None);
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::MalformedHexEscape(_), .. })));
    let r = parse_exprlist_str(r#":|\u{D800}|"#, None);
    assert!(matches!(
        r,
        Err(SexParserError { kind: SexParserErrorKind::InvalidUnicodeChar(0xD800), .. })
    ));
}

#[test]
fn parse_barred_keyword_unterminated() {
    let r = parse_exprlist_str(":|abc", None);
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedBarSymbol, .. })));
}

#[test]
fn bare_pipe_after_keyword_rejected() {
    assert!(parse_expression_str(":foo|bar|", None).is_err());
}


#[test]
fn parse_keyword_basic() {
    assert_eq!(parse_expression_str(":foo", None).unwrap(), Atom::keyword("foo"));
}

#[test]
fn parse_keyword_with_hyphen() {
    assert_eq!(parse_expression_str(":foo-bar", None).unwrap(), Atom::keyword("foo-bar"));
}

#[test]
fn parse_empty_keyword() {
    let r = parse_exprlist_str(":", None);
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::EmptyKeyword, .. })));
}


#[test]
fn parse_empty_list() {
    assert_eq!(parse_expression_str("()", None).unwrap(), Atom::List(vec![]));
}

#[test]
fn parse_list_one_element() {
    assert_eq!(
        parse_expression_str("(42)", None).unwrap(),
        Atom::List(vec![Atom::Number(Number::Integer(42))])
    );
}

#[test]
fn parse_list_multiple_elements() {
    assert_eq!(
        parse_expression_str("(a b c)", None).unwrap(),
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
        parse_expression_str("(a (b c))", None).unwrap(),
        Atom::List(vec![
            Atom::symbol("a"),
            Atom::List(vec![Atom::symbol("b"), Atom::symbol("c")]),
        ])
    );
}

#[test]
fn parse_deeply_nested_list() {
    let result = parse_expression_str("((((nil))))", None).unwrap();
    assert_eq!(
        result,
        Atom::List(vec![Atom::List(vec![Atom::List(vec![Atom::List(vec![
            Atom::Nil
        ])])])])
    );
}

#[test]
fn parse_unterminated_list() {
    let r = parse_exprlist_str("(a b", None);
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedList, .. })));
}

#[test]
fn parse_unterminated_list_empty() {
    let r = parse_exprlist_str("(", None);
    assert!(matches!(r, Err(SexParserError { kind: SexParserErrorKind::UnterminatedList, .. })));
}


#[test]
fn parse_with_leading_whitespace() {
    assert_eq!(parse_expression_str("  42", None).unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_with_trailing_whitespace() {
    assert_eq!(parse_expression_str("42  ", None).unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_with_tabs_and_newlines() {
    let input = "\n\t(a\n\tb\n)";
    assert_eq!(
        parse_expression_str(input, None).unwrap(),
        Atom::List(vec![Atom::symbol("a"), Atom::symbol("b")])
    );
}


#[test]
fn parse_multiple_atoms() {
    let atoms = parse_exprlist_str("a b c", None).unwrap();
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
    let atoms = parse_exprlist_str("42 :key \"str\"", None).unwrap();
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
    let atoms = parse_exprlist_str("", None).unwrap();
    assert!(atoms.is_empty());
}

#[test]
fn parse_whitespace_only() {
    let atoms = parse_exprlist_str("  \n\t  ", None).unwrap();
    assert!(atoms.is_empty());
}


#[test]
fn parse_atom_single() {
    assert_eq!(parse_expression_str("42", None).unwrap(), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_atom_errors_on_trailing() {
    assert!(parse_expression_str("42 foo", None).is_err());
}


#[test]
fn error_position_tracked() {
    let err = parse_exprlist_str("(\n :\n)", None).unwrap_err();
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
    let err = parse_exprlist_str("(a (b :c)", None).unwrap_err();
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
    let err = parse_exprlist_str("(", None).unwrap_err();
    assert!(matches!(err, SexParserError { kind: SexParserErrorKind::UnterminatedList, .. }));
}

#[test]
fn error_unexpected_char() {
    let err = parse_exprlist_str(")", None).unwrap_err();
    assert!(matches!(
        err,
        SexParserError { kind: SexParserErrorKind::UnexpectedChar(')'), .. }
    ));
}


#[test]
fn parse_mixed_keywords_and_symbols_in_list() {
    let input = "(deftexture foo :src (path \"bar.png\") :x 0 :y 0)";
    let parsed = parse_expression_str(input, None).unwrap();
    assert!(parsed.is_list());
}

#[test]
fn parse_keyword_atom_near_list_boundary() {
    let parsed = parse_expression_str("(:tag)", None).unwrap();
    let list = parsed.as_list().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0], Atom::keyword("tag"));
}

#[test]
fn nil_in_list() {
    let atom = parse_expression_str("(nil)", None).unwrap();
    let list = atom.as_list().unwrap();
    assert_eq!(list[0], Atom::Nil);
}

fn assert_float_eq(input: &str, expected: f64) {
    let atom = parse_expression_str(input, None).unwrap();
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
    let atom = parse_expression_str("-0.0", None).unwrap();
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