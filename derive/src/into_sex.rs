use crate::shared::*;
use quote::{format_ident, quote};
use syn::{Data, Fields, FieldsNamed};

enum Accessor {
    SelfAccess,
    Local,
}

pub fn expand_into_sex(name: &syn::Ident, data: &Data) -> proc_macro2::TokenStream {
    try_expand_into_sex(name, data).unwrap_or_else(syn::Error::into_compile_error)
}

fn try_expand_into_sex(name: &syn::Ident, data: &Data) -> syn::Result<proc_macro2::TokenStream> {
    match data {
        Data::Struct(data_struct) => derive_struct(name, &data_struct.fields),
        Data::Enum(data_enum) => derive_enum(name, data_enum),
        Data::Union(_) => Err(syn::Error::new_spanned(
            name,
            "`IntoSex` derive does not support unions",
        )),
    }
}

fn derive_struct(name: &syn::Ident, fields: &Fields) -> syn::Result<proc_macro2::TokenStream> {
    match fields {
        Fields::Named(fields) => {
            let (elems, keywords) = named_field_list(fields, Accessor::SelfAccess)?;
            Ok(quote! {
                impl sex::IntoSex for #name {
                    fn into_sex(&self) -> sex::Atom {
                        sex::Atom::List(sex::List::from(vec![
                            #(#elems)*
                            #(#keywords)*
                        ]))
                    }
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            name,
            "`IntoSex` derive only supports named fields",
        )),
    }
}

fn derive_enum(name: &syn::Ident, denum: &syn::DataEnum) -> syn::Result<proc_macro2::TokenStream> {
    let mut arms = Vec::new();

    for variant in &denum.variants {
        let variant_name = &variant.ident;
        let attribs = parse_sex_attributes(&variant.attrs)?;
        if attribs.default.is_some() || attribs.keyword.is_some() {
            return Err(syn::Error::new_spanned(
                variant_name,
                "`default` and `keyword` attributes cannot be used on an enum identifier",
            ));
        }

        let tag = attribs
            .tag
            .unwrap_or_else(|| sex_util::sex_name(variant_name.to_string()));
        let tag_lit = syn::LitStr::new(&tag, proc_macro2::Span::call_site());

        let arm = match &variant.fields {
            Fields::Unit => {
                quote! {
                    #name::#variant_name => {
                        sex::Atom::List(sex::List::from(vec![sex::Atom::symbol(#tag_lit)]))
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let mut values = Vec::new();
                let mut names = Vec::new();
                for (i, _) in fields.unnamed.iter().enumerate() {
                    let field_name = format_ident!("field_{}", i);
                    names.push(field_name.clone());
                    values.push(quote! { #field_name.into_sex() });
                }
                quote! {
                    #name::#variant_name(#(#names),*) => {
                        sex::Atom::List(sex::List::from(vec![
                            sex::Atom::symbol(#tag_lit),
                            #(#values),*
                        ]))
                    }
                }
            }
            Fields::Named(fields) => {
                let (elems, keywords) = named_field_list(fields, Accessor::Local)?;
                let destructure = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
                quote! {
                    #name::#variant_name { #(#destructure),* } => {
                        sex::Atom::List(sex::List::from(vec![
                            sex::Atom::symbol(#tag_lit),
                            #(#elems)*
                            #(#keywords)*
                        ]))
                    }
                }
            }
        };
        arms.push(arm);
    }

    Ok(quote! {
        impl sex::IntoSex for #name {
            fn into_sex(&self) -> sex::Atom {
                match self {
                    #(#arms)*
                }
            }
        }
    })
}

fn named_field_list(
    fields: &FieldsNamed,
    accessor: Accessor,
) -> syn::Result<(Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>)> {
    let mut elems = Vec::new();
    let mut keywords = Vec::new();

    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let value = match accessor {
            Accessor::SelfAccess => quote! { self.#field_name },
            Accessor::Local => quote! { #field_name },
        };

        let attribs = parse_sex_attributes(&field.attrs)?;
        if attribs.tag.is_some() {
            return Err(syn::Error::new_spanned(
                field_name,
                "struct identifiers cannot have the `tag` attribute",
            ));
        }

        if let Some(keyword) = attribs.keyword {
            let keyword_name = keyword_name(field_name, keyword);
            let keyword_lit = syn::LitStr::new(&keyword_name, proc_macro2::Span::call_site());
            keywords.push(quote! {
                sex::Atom::keyword(#keyword_lit), #value.into_sex(),
            });
        } else {
            if attribs.default.is_some() {
                return Err(syn::Error::new_spanned(
                    field_name,
                    "struct attribute `default`, cannot be used without a `keyword` attribute",
                ));
            }
            elems.push(quote! { #value.into_sex(), });
        }
    }

    Ok((elems, keywords))
}
