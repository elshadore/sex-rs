use quote::quote;
use std::collections::VecDeque;
use syn::{Attribute, Expr, Meta};

pub enum SexKeyword {
    Keyword,
    Custom(String),
}

pub enum SexDefault {
    Default,
    Custom(Expr),
}

pub struct SexAttributes {
    pub tag: Option<String>,
    pub keyword: Option<SexKeyword>,
    pub default: Option<SexDefault>,
}

pub fn keyword_name(field: &syn::Ident, keyword: SexKeyword) -> String {
    match keyword {
        SexKeyword::Custom(string) => string,
        SexKeyword::Keyword => sex_util::sex_name(field.to_string()),
    }
}

pub fn parse_sex_attributes(attributes: &[Attribute]) -> SexAttributes {
    let mut result = SexAttributes {
        tag: None,
        keyword: None,
        default: None,
    };

    for attribute in attributes
        .iter()
        .filter(|attrib| attrib.path().is_ident("sex"))
    {
        if let Meta::List(meta) = &attribute.meta {
            let mut iter: VecDeque<_> = meta.tokens.clone().into_iter().collect();
            while let Some(token) = iter.pop_front() {
                let ident = match token {
                    proc_macro2::TokenTree::Ident(ident) => ident,
                    _ => continue,
                };
                let power_word = ident.to_string();
                match power_word.as_str() {
                    "keyword" => {
                        if result.keyword.is_some() {
                            panic!("keyword attribute already defined");
                        }
                        if maybe_punct(&mut iter, '=') {
                            match iter.pop_front() {
                                Some(proc_macro2::TokenTree::Literal(lit)) => {
                                    let s = lit.to_string();
                                    result.keyword = Some(SexKeyword::Custom(s.trim_matches('"').to_string()));
                                }
                                Some(proc_macro2::TokenTree::Ident(ident)) => {
                                    result.keyword = Some(SexKeyword::Custom(ident.to_string()));
                                }
                                _ => {
                                    panic!("expected");
                                }
                            }
                        } else {
                            result.keyword = Some(SexKeyword::Keyword);
                        }
                    }
                    "default" => {
                        if result.default.is_some() {
                            panic!("default attribute already defined");
                        }
                        if maybe_punct(&mut iter, '=') {
                            match iter.pop_front() {
                                Some(proc_macro2::TokenTree::Ident(ident)) => {
                                    let expr = syn::parse2(quote! { #ident }).unwrap();
                                    result.default = Some(SexDefault::Custom(expr));
                                }
                                Some(proc_macro2::TokenTree::Literal(lit)) => {
                                    let expr = syn::parse2(quote! { #lit }).unwrap();
                                    result.default = Some(SexDefault::Custom(expr));
                                }
                                _ => {
                                    panic!("expected default");
                                }
                            }
                        } else {
                            result.default = Some(SexDefault::Default);
                        }
                    }
                    "tag" => {
                        if result.tag.is_some() {
                            panic!("tag attribute already defined");
                        }
                        expect_punct(&mut iter, '=');
                        match iter.pop_front() {
                            Some(proc_macro2::TokenTree::Literal(lit)) => {
                                let s = lit.to_string();
                                result.tag = Some(s.trim_matches('"').to_string());
                            }
                            Some(proc_macro2::TokenTree::Ident(ident)) => {
                                result.tag = Some(ident.to_string());
                            }
                            _ => {
                                panic!("expected");
                            }
                        }
                    }
                    _ => {
                        panic!("unknown FromSex || IntoSex attribute identifier: {power_word}");
                    }
                }
            }
        }
    }

    result
}

fn expect_punct(iter: &mut VecDeque<proc_macro2::TokenTree>, expect: char) {
    if let Some(proc_macro2::TokenTree::Punct(punct)) = iter.pop_front() {
        let found = punct.as_char();
        if found != expect {
            panic!("expected: '{expect}', found: '{found}'");
        }
    } else {
        panic!("expected: '{expect}'")
    }
}

fn maybe_punct(iter: &mut VecDeque<proc_macro2::TokenTree>, maybe: char) -> bool {
    match iter.front() {
        Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == maybe => {
            iter.pop_front();
            true
        }
        _ => false,
    }
}