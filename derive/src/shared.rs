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

pub fn parse_sex_attributes(attributes: &[Attribute]) -> syn::Result<SexAttributes> {
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
                            return Err(syn::Error::new(
                                ident.span(),
                                "`keyword` attribute already defined",
                            ));
                        }
                        result.keyword = Some(if maybe_punct(&mut iter, '=') {
                            match iter.pop_front() {
                                Some(proc_macro2::TokenTree::Literal(lit)) => {
                                    let s = lit.to_string();
                                    SexKeyword::Custom(s.trim_matches('"').to_string())
                                }
                                Some(proc_macro2::TokenTree::Ident(ident)) => {
                                    SexKeyword::Custom(ident.to_string())
                                }
                                other => {
                                    return Err(syn::Error::new(
                                        token_span(other),
                                        "expected a `Literal` or `Ident` type of token as `keyword` attribute",
                                    ));
                                }
                            }
                        } else {
                            SexKeyword::Keyword
                        });
                    }
                    "default" => {
                        if result.default.is_some() {
                            return Err(syn::Error::new(
                                ident.span(),
                                "`default` attribute already defined",
                            ));
                        }
                        result.default = Some(if maybe_punct(&mut iter, '=') {
                            match iter.pop_front() {
                                Some(proc_macro2::TokenTree::Ident(ident)) => {
                                    let expr = syn::parse2(quote! { #ident })?;
                                    SexDefault::Custom(expr)
                                }
                                Some(proc_macro2::TokenTree::Literal(lit)) => {
                                    let expr = syn::parse2(quote! { #lit })?;
                                    SexDefault::Custom(expr)
                                }
                                other => {
                                    return Err(syn::Error::new(
                                        token_span(other),
                                        "expected a `Literal` or `Ident` type of token as `default` attribute",
                                    ));
                                }
                            }
                        } else {
                            SexDefault::Default
                        });
                    }
                    "tag" => {
                        if result.tag.is_some() {
                            return Err(syn::Error::new(
                                ident.span(),
                                "`tag` attribute already defined",
                            ));
                        }
                        expect_punct(&mut iter, '=')?;
                        result.tag = Some(match iter.pop_front() {
                            Some(proc_macro2::TokenTree::Literal(lit)) => {
                                let s = lit.to_string();
                                s.trim_matches('"').to_string()
                            }
                            Some(proc_macro2::TokenTree::Ident(ident)) => ident.to_string(),
                            other => {
                                return Err(syn::Error::new(
                                    token_span(other),
                                    "expected a `Literal` or `Ident` type of token as `tag` attribute",
                                ));
                            }
                        });
                    }
                    _ => {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("unknown `FromSex` || `IntoSex` attribute identifier: '{power_word}'"),
                        ));
                    }
                }
            }
        }
    }

    Ok(result)
}

fn token_span(token: Option<proc_macro2::TokenTree>) -> proc_macro2::Span {
    token.map_or_else(proc_macro2::Span::call_site, |token| token.span())
}

fn expect_punct(
    iter: &mut VecDeque<proc_macro2::TokenTree>,
    expect: char,
) -> syn::Result<()> {
    match iter.pop_front() {
        Some(proc_macro2::TokenTree::Punct(punct)) => {
            let found = punct.as_char();
            if found != expect {
                Err(syn::Error::new(
                    punct.span(),
                    format!("expected optional punctuation: '{expect}', found: '{found}'"),
                ))
            } else {
                Ok(())
            }
        }
        other => Err(syn::Error::new(
            token_span(other),
            format!("expected punctuation: '{expect}'"),
        )),
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
