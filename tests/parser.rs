use sex::{Atom, Number, SexParserError, parse_atom, parse_listed};

fn p1(input: &str) -> Atom {
    parse_listed(input).unwrap().remove(0)
}


#[test]
fn parse_bare_symbol() {
    assert_eq!(p1("hello"), Atom::symbol("hello"));
}

#[test]
fn parse_symbol_with_hyphen() {
    assert_eq!(p1("foo-bar"), Atom::symbol("foo-bar"));
}

#[test]
fn parse_symbol_with_underscore() {
    assert_eq!(p1("foo_bar"), Atom::symbol("foo_bar"));
}

#[test]
fn parse_symbol_with_slash() {
    assert_eq!(p1("foo/bar"), Atom::symbol("foo/bar"));
}

#[test]
fn parse_symbol_with_dot() {
    assert_eq!(p1("foo.bar"), Atom::symbol("foo.bar"));
}

#[test]
fn parse_symbol_with_star() {
    assert_eq!(p1("foo*bar"), Atom::symbol("foo*bar"));
}

#[test]
fn parse_symbol_with_plus() {
    assert_eq!(p1("foo+bar"), Atom::symbol("foo+bar"));
}

#[test]
fn parse_symbol_with_exclamation() {
    assert_eq!(p1("foo!"), Atom::symbol("foo!"));
}

#[test]
fn parse_symbol_with_question() {
    assert_eq!(p1("foo?"), Atom::symbol("foo?"));
}

#[test]
fn parse_symbol_with_angle_brackets() {
    assert_eq!(p1("foo<bar>"), Atom::symbol("foo<bar>"));
}

#[test]
fn parse_symbol_with_equals() {
    assert_eq!(p1("foo=bar"), Atom::symbol("foo=bar"));
}

#[test]
fn parse_symbol_with_ampersand() {
    assert_eq!(p1("foo&bar"), Atom::symbol("foo&bar"));
}

#[test]
fn parse_symbol_with_percent() {
    assert_eq!(p1("foo%bar"), Atom::symbol("foo%bar"));
}

#[test]
fn parse_symbol_mixed() {
    let s = p1("a-b/c.d_e+f!?");
    assert_eq!(s, Atom::symbol("a-b/c.d_e+f!?"));
}

#[test]
fn parse_symbol_starting_with_dot() {
    assert_eq!(p1(".foo"), Atom::symbol(".foo"));
}

#[test]
fn parse_symbol_single_dot() {
    assert_eq!(p1("."), Atom::symbol("."));
}

#[test]
fn parse_symbol_single_hyphen() {
    assert_eq!(p1("-"), Atom::symbol("-"));
}

#[test]
fn parse_true_is_a_symbol() {
    assert_eq!(p1("true"), Atom::symbol("true"));
}

#[test]
fn parse_t_is_a_symbol() {
    assert_eq!(p1("t"), Atom::symbol("t"));
}

#[test]
fn parse_nil() {
    assert_eq!(p1("nil"), Atom::Nil);
}

#[test]
fn parse_nil_in_list() {
    let atoms = parse_listed("(nil)").unwrap();
    assert_eq!(atoms, vec![Atom::List(vec![Atom::Nil])]);
}


#[test]
fn parse_integer_zero() {
    assert_eq!(p1("0"), Atom::Number(Number::Integer(0)));
}

#[test]
fn parse_integer_positive() {
    assert_eq!(p1("42"), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_integer_negative() {
    assert_eq!(p1("-42"), Atom::Number(Number::Integer(-42)));
}

#[test]
fn parse_integer_max() {
    assert_eq!(
        p1("9223372036854775807"),
        Atom::Number(Number::Integer(9223372036854775807))
    );
}

#[test]
fn parse_integer_min() {
    assert_eq!(
        p1("-9223372036854775808"),
        Atom::Number(Number::Integer(-9223372036854775808))
    );
}

#[test]
fn parse_negative_sign_not_number() {
    assert_eq!(p1("-"), Atom::symbol("-"));
}


#[test]
fn parse_float_simple() {
    assert_eq!(p1("3.14"), Atom::Number(Number::Float(3.14)));
}

#[test]
fn parse_float_negative() {
    assert_eq!(p1("-2.5"), Atom::Number(Number::Float(-2.5)));
}

#[test]
fn parse_float_zero() {
    assert_eq!(p1("0.0"), Atom::Number(Number::Float(0.0)));
}

#[test]
fn parse_trailing_dot_is_valid_float() {
    assert_eq!(p1("42."), Atom::Number(Number::Float(42.0)));
}

#[test]
fn parse_double_dot_stops_at_second_dot() {
    let atoms = parse_listed("1.2.3").unwrap();
    assert_eq!(atoms[0], Atom::Number(Number::Float(1.2)));
    assert_eq!(atoms[1], Atom::symbol(".3"));
}

#[test]
fn parse_number_with_letters_stops_at_letter() {
    let atoms = parse_listed("12a34").unwrap();
    assert_eq!(atoms[0], Atom::Number(Number::Integer(12)));
    assert_eq!(atoms[1], Atom::symbol("a34"));
}

#[test]
fn negative_number_vs_symbol() {
    assert!(p1("-42").is_number());
    assert!(p1("-").is_symbol());
}


#[test]
fn parse_string_empty() {
    assert_eq!(p1(r#""""#), Atom::string(""));
}

#[test]
fn parse_string_basic() {
    assert_eq!(p1(r#""hello""#), Atom::string("hello"));
}

#[test]
fn parse_string_with_escaped_quote() {
    assert_eq!(p1(r#""say \"hi\"""#), Atom::string("say \"hi\""));
}

#[test]
fn parse_string_with_escaped_backslash() {
    assert_eq!(p1(r#""a\\b""#), Atom::string("a\\b"));
}

#[test]
fn parse_string_with_newline() {
    assert_eq!(p1(r#""a\nb""#), Atom::string("a\nb"));
}

#[test]
fn parse_string_with_tab() {
    assert_eq!(p1(r#""a\tb""#), Atom::string("a\tb"));
}

#[test]
fn parse_string_with_carriage_return() {
    assert_eq!(p1(r#""a\rb""#), Atom::string("a\rb"));
}

#[test]
fn parse_string_with_spaces() {
    assert_eq!(p1(r#""hello world""#), Atom::string("hello world"));
}

#[test]
fn strings_are_text() {
    let s = p1(r#""hello""#);
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
    let r = parse_listed(r#""\x""#);
    assert!(matches!(
        r,
        Err(SexParserError::InvalidEscape { ch: 'x', .. })
    ));
}


#[test]
fn parse_keyword_basic() {
    assert_eq!(p1(":foo"), Atom::keyword("foo"));
}

#[test]
fn parse_keyword_with_hyphen() {
    assert_eq!(p1(":foo-bar"), Atom::keyword("foo-bar"));
}

#[test]
fn parse_empty_keyword() {
    let r = parse_listed(":");
    assert!(matches!(r, Err(SexParserError::EmptyKeyword { .. })));
}


#[test]
fn parse_empty_list() {
    assert_eq!(p1("()"), Atom::List(vec![]));
}

#[test]
fn parse_list_one_element() {
    assert_eq!(
        p1("(42)"),
        Atom::List(vec![Atom::Number(Number::Integer(42))])
    );
}

#[test]
fn parse_list_multiple_elements() {
    assert_eq!(
        p1("(a b c)"),
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
        p1("(a (b c))"),
        Atom::List(vec![
            Atom::symbol("a"),
            Atom::List(vec![Atom::symbol("b"), Atom::symbol("c")]),
        ])
    );
}

#[test]
fn parse_deeply_nested_list() {
    let result = p1("((((nil))))");
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
    assert_eq!(p1("  42"), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_with_trailing_whitespace() {
    assert_eq!(p1("42  "), Atom::Number(Number::Integer(42)));
}

#[test]
fn parse_with_tabs_and_newlines() {
    let input = "\n\t(a\n\tb\n)";
    assert_eq!(
        p1(input),
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
    let parsed = p1(input);
    assert!(parsed.is_list());
}

#[test]
fn parse_keyword_atom_near_list_boundary() {
    let parsed = p1("(:tag)");
    let list = parsed.as_list().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0], Atom::keyword("tag"));
}


#[test]
fn parse_deftexture_shape() {
    let input = r#"(deftexture cursor :src (path "./textures/cursor.png"))"#;
    let atom = p1(input);
    let list = atom.as_list().unwrap();
    assert_eq!(list[0], Atom::symbol("deftexture"));
    assert_eq!(list[1], Atom::symbol("cursor"));
    assert_eq!(list[2], Atom::keyword("src"));
    let src = list[3].as_list().unwrap();
    assert_eq!(src[0], Atom::symbol("path"));
    assert_eq!(src[1], Atom::string("./textures/cursor.png"));
}

#[test]
fn parse_deftexture_with_subrect() {
    let input = r#"(deftexture element-fire :src (texture element) :x 0 :y 32 :w 32 :h 32)"#;
    let atom = p1(input);
    let list = atom.as_list().unwrap();
    assert_eq!(list[0], Atom::symbol("deftexture"));
    assert_eq!(list[4], Atom::keyword("x"));
    assert_eq!(list[5], Atom::Number(Number::Integer(0)));
    assert_eq!(list[6], Atom::keyword("y"));
    assert_eq!(list[7], Atom::Number(Number::Integer(32)));
}

#[test]
fn parse_mixed_keywords_positional() {
    let input = "(deffont iosevka :type (bmfont \"./fonts/iosevka.fnt\" iosevka))";
    let atom = p1(input);
    let list = atom.as_list().unwrap();
    assert_eq!(list.len(), 4);
    assert_eq!(list[2], Atom::keyword("type"));
    let ty = list[3].as_list().unwrap();
    assert_eq!(ty[0], Atom::symbol("bmfont"));
    assert_eq!(ty[1], Atom::string("./fonts/iosevka.fnt"));
    assert_eq!(ty[2], Atom::symbol("iosevka"));
}


#[test]
fn nil_in_list() {
    let atom = p1("(nil)");
    let list = atom.as_list().unwrap();
    assert_eq!(list[0], Atom::Nil);
}
