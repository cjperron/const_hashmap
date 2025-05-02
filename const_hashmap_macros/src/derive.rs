//! Procedural helpers for `ConstHash` / `ConstEq` derives.

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as Ts};
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields, Ident, Index, parse_macro_input};

/*────────────────────────────  #[derive(ConstHash)] ───────────────────────*/
pub fn derive_const_hash_impl(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let name = &ast.ident;

    let body = match &ast.data {
        Data::Struct(s) => struct_hash(&s.fields, quote! { self }),
        Data::Enum(e) => enum_hash(name, e),
        Data::Union(_) => unimplemented!("ConstHash not supported for unions"),
    };

    quote! {
        impl const const_hashmap::ConstHash for #name {
            fn const_hash(&self) -> u64 { #body }
        }
    }
    .into()
}

/*──────────────────────────────  #[derive(ConstEq)] ───────────────────────*/
pub fn derive_const_eq_impl(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let name = &ast.ident;

    let body = match &ast.data {
        Data::Struct(s) => struct_eq(&s.fields, quote! { self }, quote! { other }),
        Data::Enum(e) => enum_eq(name, e),
        Data::Union(_) => unimplemented!("ConstEq not supported for unions"),
    };

    quote! {
        impl const const_hashmap::ConstEq for #name {
            fn const_eq(&self, other: &Self) -> bool { #body }
        }
    }
    .into()
}

/*──────────────────────── helpers: structs ───────────────────────────────*/
fn struct_hash(fields: &Fields, base: Ts) -> Ts {
    let parts = match fields {
        Fields::Unit => return quote! { 0u64 },

        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| {
                let id = f.ident.as_ref().unwrap();
                quote! { const_hashmap::ConstHash::const_hash(&#base.#id) }
            })
            .collect::<Vec<_>>(),

        Fields::Unnamed(un) => (0..un.unnamed.len())
            .map(|i| {
                let idx = Index::from(i);
                quote! { const_hashmap::ConstHash::const_hash(&#base.#idx) }
            })
            .collect::<Vec<_>>(),
    };

    quote! {{
        let mut h = 0u64;
        #( h = h.rotate_left(5) ^ #parts; )*
        h
    }}
}

fn struct_eq(fields: &Fields, lhs: Ts, rhs: Ts) -> Ts {
    let parts = match fields {
        Fields::Unit => return quote! { true },

        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| {
                let id = f.ident.as_ref().unwrap();
                quote! { const_hashmap::ConstEq::const_eq(&#lhs.#id, &#rhs.#id) }
            })
            .collect::<Vec<_>>(),

        Fields::Unnamed(un) => (0..un.unnamed.len())
            .map(|i| {
                let idx = Index::from(i);
                quote! { const_hashmap::ConstEq::const_eq(&#lhs.#idx, &#rhs.#idx) }
            })
            .collect::<Vec<_>>(),
    };

    quote! { #( #parts )&&* }
}

/*──────────────────────── helpers: enums ─────────────────────────────────*/
fn enum_hash(enm: &Ident, data: &DataEnum) -> Ts {
    let arms = data.variants.iter().enumerate().map(|(idx, var)| {
        let disc = Index::from(idx);
        let v = &var.ident;

        match &var.fields {
            /* unit ---------------------------------------------------------- */
            Fields::Unit => quote! { #enm::#v => (#disc as u64) },

            /* tuple-like ----------------------------------------------------- */
            Fields::Unnamed(un) => {
                let ids: Vec<_> = (0..un.unnamed.len())
                    .map(|i| Ident::new(&format!("f{idx}_{i}"), Span::call_site()))
                    .collect();
                let pat = quote! { #( #ids ),* };
                let parts = ids.iter().map(|id| {
                    quote! { const_hashmap::ConstHash::const_hash(&#id) }
                });
                quote! {
                    #enm::#v( #pat ) => {
                        let mut h = (#disc as u64);
                        #( h = h.rotate_left(5) ^ #parts; )*
                        h
                    }
                }
            }

            /* struct-like ---------------------------------------------------- */
            Fields::Named(named) => {
                let field_names: Vec<_> = named
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();
                let ids: Vec<_> = field_names
                    .iter()
                    .map(|n| Ident::new(&format!("f{idx}_{n}"), Span::call_site()))
                    .collect();

                let binds = field_names.iter().zip(&ids).map(|(n, i)| quote! { #n: #i });
                let parts = ids.iter().map(|id| {
                    quote! { const_hashmap::ConstHash::const_hash(&#id) }
                });

                quote! {
                    #enm::#v { #( #binds ),* } => {
                        let mut h = (#disc as u64);
                        #( h = h.rotate_left(5) ^ #parts; )*
                        h
                    }
                }
            }
        }
    });

    quote! { match self { #( #arms ),* } }
}

fn enum_eq(enm: &Ident, data: &DataEnum) -> Ts {
    let arms = data.variants.iter().enumerate().map(|(idx, var)| {
        let v = &var.ident;

        match &var.fields {
            Fields::Unit => quote! { (#enm::#v, #enm::#v) => true },

            Fields::Unnamed(un) => {
                let l_ids: Vec<_> = (0..un.unnamed.len())
                    .map(|i| Ident::new(&format!("a{idx}_{i}"), Span::call_site()))
                    .collect();
                let r_ids: Vec<_> = (0..un.unnamed.len())
                    .map(|i| Ident::new(&format!("b{idx}_{i}"), Span::call_site()))
                    .collect();

                let cmp = l_ids.iter().zip(&r_ids).map(|(a, b)| {
                    quote! { const_hashmap::ConstEq::const_eq(&#a, &#b) }
                });

                quote! {
                    ( #enm::#v( #( #l_ids ),* ), #enm::#v( #( #r_ids ),* ) ) => { #( #cmp )&&* }
                }
            }

            Fields::Named(named) => {
                let fns: Vec<_> = named
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();
                let l_ids: Vec<_> = fns
                    .iter()
                    .map(|n| Ident::new(&format!("la{idx}_{n}"), Span::call_site()))
                    .collect();
                let r_ids: Vec<_> = fns
                    .iter()
                    .map(|n| Ident::new(&format!("rb{idx}_{n}"), Span::call_site()))
                    .collect();

                let bl = fns.iter().zip(&l_ids).map(|(n, l)| quote! { #n: #l });
                let br = fns.iter().zip(&r_ids).map(|(n, r)| quote! { #n: #r });

                let cmp = l_ids.iter().zip(&r_ids).map(|(a, b)| {
                    quote! { const_hashmap::ConstEq::const_eq(&#a, &#b) }
                });

                quote! {
                    ( #enm::#v { #( #bl ),* }, #enm::#v { #( #br ),* } ) => { #( #cmp )&&* }
                }
            }
        }
    });

    quote! { match (self, other) { #( #arms , )* _ => false } }
}
