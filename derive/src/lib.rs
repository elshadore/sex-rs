use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Attribute, Lit, Meta, Expr};

/// Derives the [`FromSex`] trait for a struct or enum, enabling deserialization from
/// S-expression atoms.
///
/// Attribute syntax: `#[sex(...)]` on fields or enum variants.
///
///
/// # Structs
///
/// Only named fields are supported.
///
/// ## Positional fields (default)
///
/// Fields are matched positionally in declaration order:
///
/// ```ignore
/// #[derive(Sex)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// // Input: (10 20)  →  Point { x: 10, y: 20 }
/// ```
///
/// ## Keyword fields
///
/// Use `#[sex(keyword)]` to mark a field as keyword-driven. The keyword name defaults to the
/// field name with underscores replaced by hyphens.
///
/// Use `#[sex(keyword = "custom-name")]` to override the keyword.
///
/// ```ignore
/// #[derive(Sex)]
/// struct Config {
///     name: String,
///     #[sex(keyword)]
///     width: i32,
///     #[sex(keyword = "hgt")]
///     height: i32,
/// }
///
/// // Input: ("test" :width 800 :hgt 600)
/// //   →  Config { name: "test", width: 800, height: 600 }
/// ```
///
/// ## Optional / default fields
///
/// Combine with `#[sex(default)]` or `#[sex(default = expr)]` to make a field optional:
///
/// ```ignore
/// #[derive(Sex)]
/// struct Window {
///     title: String,
///     #[sex(keyword, default = 800)]
///     width: i32,
///     #[sex(keyword, default = 600)]
///     height: i32,
/// }
///
/// // Input: ("my window" :height 400)
/// //   →  Window { title: "my window", width: 800, height: 400 }
/// ```
///
/// `#[sex(default)]` (without an expression) uses `Default::default()` for the field type.
/// Keyword fields without `#[sex(default)]` are required and will error if missing.
///
///
/// # Enums
///
/// The first element of the input list must be a **symbol tag** that identifies which variant
/// to deserialize. The remaining elements are parsed as the variant's fields.
///
/// The tag defaults to the variant name in lowercase with underscores replaced by hyphens.
/// Override with `#[sex(tag = "custom-tag")]`.
///
/// ## Unit variants
///
/// ```ignore
/// #[derive(Sex)]
/// enum Command {
///     #[sex(tag = "noop")]
///     Noop,
///     #[sex(tag = "quit")]
///     Quit,
/// }
///
/// // Input: (noop)  →  Command::Noop
/// ```
///
/// ## Tuple variants (single field — primitive vs. complex)
///
/// A single tuple field containing a **primitive** type (`i32`, `f32`, `String`, `bool`, etc.)
/// reads one positional atom:
///
/// ```ignore
/// #[derive(Sex)]
/// enum Shape {
///     #[sex(tag = "circle")]
///     Circle(i32),
/// }
///
/// // Input: (circle 5)  →  Shape::Circle(5)
/// ```
///
/// A single tuple field containing a **complex** type (a struct with `FromSex`) reads the
/// *entire remaining list* as its S-expression body:
///
/// ```ignore
/// #[derive(Sex, PartialEq, Debug)]
/// struct Point { x: i32, y: i32 }
///
/// #[derive(Sex)]
/// enum Shape {
///     #[sex(tag = "point")]
///     Pt(Point),
/// }
///
/// // Input: (point 1 2)  →  Shape::Pt(Point { x: 1, y: 2 })
/// ```
///
/// ## Tuple variants (multiple fields)
///
/// Multiple positional atoms, one per field:
///
/// ```ignore
/// #[derive(Sex)]
/// enum Command {
///     #[sex(tag = "jump")]
///     Jump(i32, i32),
/// }
///
/// // Input: (jump 3 4)  →  Command::Jump(3, 4)
/// ```
///
/// ## Named-field variants
///
/// Support the same `#[sex(keyword)]` and `#[sex(default)]` attributes as struct fields:
///
/// ```ignore
/// #[derive(Sex, PartialEq, Debug)]
/// enum Shape {
///     #[sex(tag = "rect")]
///     Rect {
///         width: i32,
///         height: i32,
///         #[sex(keyword, default = 0)]
///         x: i32,
///         #[sex(keyword, default = 0)]
///         y: i32,
///     },
/// }
///
/// // Input: (rect 100 200 :x 10 :y 20)
/// //   →  Shape::Rect { width: 100, height: 200, x: 10, y: 20 }
/// ```
///
///
/// # Attribute reference summary
///
/// | Attribute | Scope | Description |
/// |---|---|
/// | `#[sex(keyword)]` | struct / enum-named field | Parse as keyword-value pair (`:name value`) |
/// | `#[sex(keyword = "str")]` | struct / enum-named field | Keyword with explicit name |
/// | `#[sex(default)]` | struct / enum-named field | Optional field, uses `Default::default()` |
/// | `#[sex(default = expr)]` | struct / enum-named field | Optional field, uses given expression |
/// | `#[sex(tag = "str")]` | enum variant | Override the symbol tag for dispatch |
///
/// See [`FromSex`], [`Atom`], and the `examples/` directory for more.
#[proc_macro_derive(Sex, attributes(sex))]
pub fn derive_sex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = match &input.data {
        Data::Struct(data_struct) => derive_struct(name, &data_struct.fields),
        Data::Enum(data_enum) => derive_enum(name, data_enum),
        Data::Union(_) => {
            return syn::Error::new_spanned(name, "Sex derive does not support unions")
                .to_compile_error()
                .into();
        }
    };
    
    TokenStream::from(expanded)
}

fn derive_struct(name: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let fields = match fields {
        Fields::Named(fields) => &fields.named,
        _ => {
            return syn::Error::new_spanned(name, "Sex derive only supports named fields")
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
            let keyword_name = sex_attrs.keyword_name
                .unwrap_or_else(|| field_name.to_string().replace('_', "-"));
            
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
                let #field_name: #field_ty = sex::FromSex::from_sex(view.try_next()?)?;
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
                
                let mut view = sex::AtomView::new(list);
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
        
        let tag = sex_attrs.keyword_name
            .unwrap_or_else(|| variant_name.to_string().to_lowercase().replace('_', "-"));
        
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
                                &sex::Atom::List(rest.to_vec())
                            )?;
                            Ok(#name::#variant_name(#field_name))
                        }
                    } else {
                        quote! {
                            let mut view = sex::AtomView::new(rest);
                            let #field_name: #field_ty = sex::FromSex::from_sex(view.try_next()?)?;
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
                            let #field_name: #field_ty = sex::FromSex::from_sex(view.try_next()?)?;
                        });
                        inits.push(quote! { #field_name });
                    }
                    quote! {
                        let mut view = sex::AtomView::new(rest);
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
                        let keyword_name = sex_attrs.keyword_name
                            .unwrap_or_else(|| field_name.to_string().replace('_', "-"));
                        
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
                            let #field_name: #field_ty = sex::FromSex::from_sex(view.try_next()?)?;
                        });
                    }
                    
                    inits.push(quote! { #field_name });
                }
                
                if keyword_parsers.is_empty() {
                    quote! {
                        let mut view = sex::AtomView::new(rest);
                        #(#positional_parsers)*
                        view.into_keywords()?;
                        Ok(#name::#variant_name { #(#inits),* })
                    }
                } else {
                    quote! {
                        let mut view = sex::AtomView::new(rest);
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
                        found: sex::Atom::List(vec![]),
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

struct SexAttrs {
    keyword: bool,
    keyword_name: Option<String>,
    strict: bool,
    default_expr: Option<Expr>,
}

fn parse_sex_attrs(attrs: &[Attribute]) -> SexAttrs {
    let mut result = SexAttrs {
        keyword: false,
        keyword_name: None,
        strict: true,
        default_expr: None,
    };
    
    for attr in attrs {
        if !attr.path().is_ident("sex") {
            continue;
        }
        
        if let Meta::List(meta_list) = &attr.meta {
            let tokens: Vec<_> = meta_list.tokens.clone().into_iter().collect();
            
            let mut i = 0;
            while i < tokens.len() {
                if let proc_macro2::TokenTree::Ident(ident) = &tokens[i] {
                    let ident_str = ident.to_string();
                    
                    if ident_str == "keyword" {
                        result.keyword = true;
                        
                        if i + 1 < tokens.len() {
                            if let proc_macro2::TokenTree::Punct(punct) = &tokens[i + 1] {
                                if punct.as_char() == '=' {
                                    if i + 2 < tokens.len() {
                                        if let proc_macro2::TokenTree::Literal(lit) = &tokens[i + 2] {
                                            let lit_tree: proc_macro2::TokenTree = lit.clone().into();
                                            if let Ok(Lit::Str(lit_str)) = syn::parse2(quote! { #lit_tree }) {
                                                result.keyword_name = Some(lit_str.value());
                                                i += 2;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if ident_str == "default" {
                        result.strict = false;
                        
                        if i + 1 < tokens.len() {
                            if let proc_macro2::TokenTree::Punct(punct) = &tokens[i + 1] {
                                if punct.as_char() == '=' {
                                    if i + 2 < tokens.len() {
                                        let expr_tokens: Vec<_> = tokens[i + 2..].iter()
                                            .take_while(|t| !matches!(t, proc_macro2::TokenTree::Punct(p) if p.as_char() == ','))
                                            .cloned()
                                            .collect();
                                        
                                        if let Ok(expr) = syn::parse2(quote! { #(#expr_tokens)* }) {
                                            result.default_expr = Some(expr);
                                        }
                                        
                                        i += 2 + expr_tokens.len();
                                        continue;
                                    }
                                }
                            }
                        }
                    } else if ident_str == "tag" {
                        if i + 1 < tokens.len() {
                            if let proc_macro2::TokenTree::Punct(punct) = &tokens[i + 1] {
                                if punct.as_char() == '=' {
                                    if i + 2 < tokens.len() {
                                        if let proc_macro2::TokenTree::Literal(lit) = &tokens[i + 2] {
                                            let lit_tree: proc_macro2::TokenTree = lit.clone().into();
                                            if let Ok(Lit::Str(lit_str)) = syn::parse2(quote! { #lit_tree }) {
                                                result.keyword_name = Some(lit_str.value());
                                                i += 2;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                i += 1;
            }
        }
    }
    
    result
}
