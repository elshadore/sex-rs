use crate::shared::*;
use quote::{format_ident, quote};
use syn::{Data, Fields, FieldsNamed};

pub fn expand_from_sex(name: &syn::Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data_struct) => derive_struct(name, &data_struct.fields),
        Data::Enum(data_enum) => derive_enum(name, data_enum),
        Data::Union(_) => syn::Error::new_spanned(name, "FromSex derive does not support unions")
            .to_compile_error(),
    }
}

fn keyword_name(field: &syn::Ident, keyword: SexKeyword) -> String {
    match keyword {
        SexKeyword::Custom(string) => string,
        SexKeyword::Keyword => sex_util::sex_name(field.to_string()),
    }
}

fn derive_struct(name: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => derive_struct_named(StructIdent::Struct(name), fields),
        _ => {
            return syn::Error::new_spanned(name, "FromSex derive only supports named fields")
                .to_compile_error();
        }
    }
}

enum StructIdent<'a> {
    Struct(&'a syn::Ident),
    Enum {
        name: &'a syn::Ident,
        variant: &'a syn::Ident,
    },
}

fn derive_struct_named(name: StructIdent, fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let mut elements = Vec::new();
    let mut keywords = Vec::new();
    let mut names = Vec::new();

    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        let attribs = parse_sex_attributes(&field.attrs);
        if attribs.tag.is_some() {
            panic!("struct identifiers cannot have the `tag` attribute");
        }

        if let Some(keyword) = attribs.keyword {
            let keyword_name = keyword_name(field_name, keyword);
            let field_code = match attribs.default {
                Some(SexDefault::Custom(expr)) => {
                    quote! {
                        let #field_name: #field_ty = keyword_map.optional(#keyword_name)?.unwrap_or(#expr);
                    }
                }
                Some(SexDefault::Default) => {
                    quote! {
                        let #field_name: #field_ty = keyword_map.optional(#keyword_name)?.unwrap_or(Default::default());
                    }
                }
                None => {
                    quote! {
                        let #field_name: #field_ty = keyword_map.required(#keyword_name)?;
                    }
                }
            };
            keywords.push(field_code);
        } else {
            if attribs.default.is_some() {
                panic!("struct attribute `default`, cannot be used without a `keyword` attribute");
            }
            elements.push(quote! {
                let #field_name: #field_ty = sex::FromSex::from_sex(view)?;
            });
        }

        names.push(quote! { #field_name });
    }
    match name {
        StructIdent::Struct(name) => {
            quote! {
                impl sex::FromSex for #name {
                    fn from_sex(view: &mut sex::ListView) -> Result<Self, sex::SexError> {
                        #(#elements)*
                        let keyword_map = view.into_keywords()?;
                        #(#keywords)*

                        Ok(#name {
                            #(#names),*
                        })
                    }
                }
            }
        }
        StructIdent::Enum { name, variant } => {
            quote! {
                #(#elements)*
                let keyword_map = view.into_keywords()?;
                #(#keywords)*
                Ok(#name::#variant { #(#names),* })
            }
        }
    }
}

fn derive_enum(name: &syn::Ident, denum: &syn::DataEnum) -> proc_macro2::TokenStream {
    let mut variant_arms = Vec::new();
    let mut variant_names = Vec::new();

    for variant in &denum.variants {
        let variant_name = &variant.ident;
        let attribs = parse_sex_attributes(&variant.attrs);
        if attribs.default.is_some() || attribs.keyword.is_some() {
            panic!("`default` and `keyword` attributes cannot be used on an enum identifier");
        }

        let tag = attribs
            .tag
            .unwrap_or_else(|| sex_util::sex_name(variant_name.to_string()));
        variant_names.push(tag.clone());

        let fields = &variant.fields;
        let arm_body = match fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    let field_ty = &fields.unnamed.first().unwrap().ty;
                    let field_name = format_ident!("field_0");

                    quote! {
                        let #field_name: #field_ty = sex::FromSex::from_sex(view)?;
                        Ok(#name::#variant_name(#field_name))
                    }
                } else {
                    let mut parsers = Vec::new();
                    let mut inits = Vec::new();
                    for (i, field) in fields.unnamed.iter().enumerate() {
                        let field_ty = &field.ty;
                        let field_name = format_ident!("field_{}", i);
                        parsers.push(quote! {
                            let #field_name: #field_ty = sex::FromSex::from_sex(view)?;
                        });
                        inits.push(quote! { #field_name });
                    }
                    quote! {
                        #(#parsers)*
                        view.into_keywords()?;
                        Ok(#name::#variant_name(#(#inits),*))
                    }
                }
            }
            Fields::Named(fields) => derive_struct_named(
                StructIdent::Enum {
                    name,
                    variant: variant_name,
                },
                fields,
            ),
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

    quote! {
        impl sex::FromSex for #name {
            fn from_sex(view: &mut sex::ListView) -> Result<Self, sex::SexError> {
                let tag_atom = view.try_pop()?;
                let tag = match tag_atom {
                    sex::Atom::Text(t) if t.ty == sex::TextTy::Symbol => t.contents.as_str(),
                    _ => return Err(sex::SexError::TypeError {
                        expected: sex::AtomTy::Symbol,
                        found: tag_atom.clone(),
                    }),
                };

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


