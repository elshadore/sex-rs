use quote::{format_ident, quote};
use syn::{Attribute, Data, Expr, Fields, Lit, Meta};

pub fn expand_from_sex(name: &syn::Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data_struct) => derive_struct(name, &data_struct.fields),
        Data::Enum(data_enum) => derive_enum(name, data_enum),
        Data::Union(_) => syn::Error::new_spanned(name, "FromSex derive does not support unions")
            .to_compile_error(),
    }
}

fn derive_struct(name: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let fields = match fields {
        Fields::Named(fields) => &fields.named,
        _ => {
            return syn::Error::new_spanned(name, "FromSex derive only supports named fields")
                .to_compile_error();
        }
    };

    let mut positional_parsers = Vec::new();
    let mut keyword_parsers = Vec::new();
    let mut field_inits = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        let sex_attrs = parse_sex_attrs(&field.attrs);

        if sex_attrs.keyword {
            let keyword_name = sex_attrs
                .keyword_name
                .unwrap_or_else(|| sex_util::sex_name(field_name.to_string()));

            let parser = if sex_attrs.strict {
                quote! {
                    let #field_name: #field_ty = keyword_map.required(#keyword_name)?;
                }
            } else {
                let default_value = if let Some(expr) = sex_attrs.default_expr {
                    quote! { #expr }
                } else {
                    quote! { Default::default() }
                };

                quote! {
                    let #field_name: #field_ty = keyword_map.optional(#keyword_name)?.unwrap_or(#default_value);
                }
            };

            keyword_parsers.push(parser);
        } else {
            positional_parsers.push(quote! {
                let #field_name: #field_ty = sex::FromSex::from_sex(view.try_pop()?)?;
            });
        }

        field_inits.push(quote! { #field_name });
    }

    quote! {
        impl sex::FromSex for #name {
            fn from_sex(atom: &sex::Atom) -> Result<Self, sex::SexError> {
                let list = match atom {
                    sex::Atom::List(list) => list,
                    _ => return Err(sex::SexError::TypeError {
                        expected: sex::AtomTy::List,
                        found: atom.clone(),
                    }),
                };

                let mut view = sex::ListView::new(list);
                #(#positional_parsers)*
                let keyword_map = view.into_keywords()?;
                #(#keyword_parsers)*

                Ok(#name {
                    #(#field_inits),*
                })
            }
        }
    }
}

fn derive_enum(name: &syn::Ident, data_enum: &syn::DataEnum) -> proc_macro2::TokenStream {
    let mut variant_arms = Vec::new();
    let mut variant_names = Vec::new();

    for variant in &data_enum.variants {
        let variant_name = &variant.ident;
        let sex_attrs = parse_sex_attrs(&variant.attrs);

        let tag = sex_attrs
            .keyword_name
            .unwrap_or_else(|| sex_util::sex_name(variant_name.to_string()));

        variant_names.push(tag.clone());

        let fields = &variant.fields;
        let arm_body = match fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    let field_ty = &fields.unnamed.first().unwrap().ty;
                    let field_name = format_ident!("field_0");

                    let is_complex_type = matches!(field_ty, syn::Type::Path(type_path)
                        if type_path.path.segments.len() > 1
                        || type_path.path.segments.first().map_or(false, |seg| {
                            let name = seg.ident.to_string();
                            name.chars().next().map_or(false, |c| c.is_uppercase())
                            && name.len() > 1
                            && !matches!(name.as_str(), "String" | "Vec" | "Option" | "Box")
                        })
                    );

                    if is_complex_type {
                        quote! {
                            let #field_name: #field_ty = sex::FromSex::from_sex(
                                &sex::Atom::List(sex::List::from_slice(rest))
                            )?;
                            Ok(#name::#variant_name(#field_name))
                        }
                    } else {
                        quote! {
                            let mut view = sex::ListView::new_slice(rest);
                            let #field_name: #field_ty = sex::FromSex::from_sex(view.try_pop()?)?;
                            view.into_keywords()?;
                            Ok(#name::#variant_name(#field_name))
                        }
                    }
                } else {
                    let mut parsers = Vec::new();
                    let mut inits = Vec::new();
                    for (i, field) in fields.unnamed.iter().enumerate() {
                        let field_ty = &field.ty;
                        let field_name = format_ident!("field_{}", i);
                        parsers.push(quote! {
                            let #field_name: #field_ty = sex::FromSex::from_sex(view.try_pop()?)?;
                        });
                        inits.push(quote! { #field_name });
                    }
                    quote! {
                        let mut view = sex::ListView::new_slice(rest);
                        #(#parsers)*
                        view.into_keywords()?;
                        Ok(#name::#variant_name(#(#inits),*))
                    }
                }
            }
            Fields::Named(fields) => {
                let mut positional_parsers = Vec::new();
                let mut keyword_parsers = Vec::new();
                let mut inits = Vec::new();

                for field in &fields.named {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_ty = &field.ty;
                    let sex_attrs = parse_sex_attrs(&field.attrs);

                    if sex_attrs.keyword {
                        let keyword_name = sex_attrs
                            .keyword_name
                            .unwrap_or_else(|| sex_util::sex_name(field_name.to_string()));

                        let parser = if sex_attrs.strict {
                            quote! {
                                let #field_name: #field_ty = keyword_map.required(#keyword_name)?;
                            }
                        } else {
                            let default_value = if let Some(expr) = sex_attrs.default_expr {
                                quote! { #expr }
                            } else {
                                quote! { Default::default() }
                            };

                            quote! {
                                let #field_name: #field_ty = keyword_map.optional(#keyword_name)?.unwrap_or(#default_value);
                            }
                        };

                        keyword_parsers.push(parser);
                    } else {
                        positional_parsers.push(quote! {
                            let #field_name: #field_ty = sex::FromSex::from_sex(view.try_pop()?)?;
                        });
                    }

                    inits.push(quote! { #field_name });
                }

                if keyword_parsers.is_empty() {
                    quote! {
                        let mut view = sex::ListView::new_slice(rest);
                        #(#positional_parsers)*
                        view.into_keywords()?;
                        Ok(#name::#variant_name { #(#inits),* })
                    }
                } else {
                    quote! {
                        let mut view = sex::ListView::new_slice(rest);
                        #(#positional_parsers)*
                        let keyword_map = view.into_keywords()?;
                        #(#keyword_parsers)*
                        Ok(#name::#variant_name { #(#inits),* })
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    Ok(#name::#variant_name)
                }
            }
        };

        variant_arms.push(quote! {
            #tag => { #arm_body }
        });
    }

    let _expected_variants = format!("{:?}", variant_names);

    quote! {
        impl sex::FromSex for #name {
            fn from_sex(atom: &sex::Atom) -> Result<Self, sex::SexError> {
                let list = match atom {
                    sex::Atom::List(list) => list,
                    _ => return Err(sex::SexError::TypeError {
                        expected: sex::AtomTy::List,
                        found: atom.clone(),
                    }),
                };

                if list.is_empty() {
                    return Err(sex::SexError::TypeError {
                        expected: sex::AtomTy::List,
                        found: sex::Atom::List(sex::List::from(vec![])),
                    });
                }

                let tag = match &list[0] {
                    sex::Atom::Text(t) if t.ty == sex::TextTy::Symbol => t.contents.as_str(),
                    _ => return Err(sex::SexError::TypeError {
                        expected: sex::AtomTy::Symbol,
                        found: list[0].clone(),
                    }),
                };

                let rest = &list[1..];

                match tag {
                    #(#variant_arms)*
                    _ => Err(sex::SexError::UnknownVariant {
                        variant: tag.to_string(),
                        expected: vec![#(#variant_names.to_string()),*],
                    }),
                }
            }
        }
    }
}

enum SexKeyword {
    Strict,
    Custom(String),
}

enum SexDefault {
    Default,
    Custom(Expr),
}

struct SexAttributes {
    pub tag: Option<String>,
    pub keyword: Option<SexKeyword>,
    pub default: Option<SexDefault>,
}

impl SexAttributes {
    pub fn verify(&self) {
        if self.tag.is_some() {
            if self.keyword.is_some() {
                panic!("")
            }
            if self.default.is_some() {
                panic!("")
            }
        } else if self.default.is_some() {
            if self.keyword.is_none() {
                panic!("")
            }
        }
    }
}

fn parse_sex_attribute(attributes: &[Attribute]) -> SexAttributes {
    let mut result = SexAttributes {
        tag: None,
        keyword: None,
        default: None
    };
    
    for attribute in attributes.iter().filter(|attrib| attrib.path().is_ident("sex")) {
        if let Meta::List(meta) = &attribute.meta {
            let mut iter = meta.tokens.clone().into_iter();
            if let Some(proc_macro2::TokenTree::Ident(ident)) = iter.next() {
                match ident.to_string().as_str() {
                    "keyword" => {
                        if result.keyword.is_some() {
                            panic!("keyword attribute already defined");
                        }
                        if maybe_punct(&mut iter, '=') {
                            match iter.next() {
                                Some(proc_macro2::TokenTree::Literal(lit)) => {
                                    result.keyword = Some(SexKeyword::Custom(lit.to_string()));
                                }
                                Some(proc_macro2::TokenTree::Ident(ident)) => {
                                    result.keyword = Some(SexKeyword::Custom(ident.to_string()));
                                }
                                _ => {
                                    panic!("expected");
                                }
                            }
                        } else {
                            result.keyword = Some(SexKeyword::Strict);
                        }
                    },
                    "default" => {
                        if result.default.is_some() {
                            panic!("default attribute already defined");
                        }
                        if maybe_punct(&mut iter, '=') {
                            match iter.next() {
                                Some(proc_macro2::TokenTree::Literal(lit)) => {
                                    let expr = syn::parse2(quote! { #lit }).unwrap();
                                    result.default = Some(SexDefault::Custom(expr));
                                }
                                Some(proc_macro2::TokenTree::Ident(ident)) => {
                                    let expr = syn::parse2(quote! { #ident }).unwrap();
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
                        if let Some(tag) = result.tag {
                            panic!("tag attribute already defined: {tag}");
                        }
                        expect_punct(&mut iter, '=');
                        match iter.next() {
                            Some(proc_macro2::TokenTree::Literal(lit)) => {
                                result.tag = Some(lit.to_string());
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
                        panic!("unknown attribute identifier");
                    }
                }
            }
        }
    }

    result

}

fn expect_punct(iter: &mut impl Iterator<Item = proc_macro2::TokenTree>, expect: char) {
    if let Some(proc_macro2::TokenTree::Punct(punct)) = iter.next() {
        let found = punct.as_char();
        if found != expect {
            panic!("expected: '{expect}', found: '{found}'");
        }
    } else {
        panic!("expected: '{expect}'")
    }
}

fn maybe_punct(iter: &mut impl Iterator<Item = proc_macro2::TokenTree>, maybe: char) -> bool {
    if let Some(proc_macro2::TokenTree::Punct(punct)) = iter.next() {
        let found = punct.as_char();
        if found != maybe {
            panic!("expected: '{maybe}', found: '{found}'");
        }
        true
    } else {
        false
    }
}
