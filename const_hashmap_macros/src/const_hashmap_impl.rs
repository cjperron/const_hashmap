use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, ItemConst, Token, Type, Visibility, braced,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::Brace,
};

/// Representa un par `key => value` dentro del macro.
struct Entry {
    key: Expr,
    _arrow_token: Token![=>],
    value: Expr,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _arrow_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

/// Entrada del macro: una lista separada por comas de `Entry`.
struct Entries(Vec<Entry>);

impl Parse for Entries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entries = Punctuated::<Entry, Token![,]>::parse_terminated(input)?;
        Ok(Entries(entries.into_iter().collect()))
    }
}

struct ConstMapInput {
    vis: Visibility,
    _const_token: Token![const],
    name: Ident,
    _colon: Token![:],
    key_ty: Type,
    _arrow: Token![=>],
    val_ty: Type,
    _eq: Token![=],
    _brace: Brace,    // ← envoltorio de las llaves
    entries: Entries, // los pares dentro
    _semi: Token![;],
}

impl Parse for ConstMapInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis: Visibility = input.parse()?;
        let _const_token: Token![const] = input.parse()?;
        let name: Ident = input.parse()?;
        let _colon: Token![:] = input.parse()?;
        let key_ty: Type = input.parse()?;
        let _arrow: Token![=>] = input.parse()?;
        let val_ty: Type = input.parse()?;
        let _eq: Token![=] = input.parse()?;

        // --- aquí capturamos el bloque { … } -----------------------------
        let content;
        let _brace = braced!(content in input); // ← macro de syn
        let entries = content.parse()?; // parsea Entries

        let _semi: Token![;] = input.parse()?;

        Ok(Self {
            vis,
            _const_token,
            name,
            _colon,
            key_ty,
            _arrow,
            val_ty,
            _eq,
            _brace,
            entries,
            _semi,
        })
    }
}

pub fn const_hashmap_impl(input: TokenStream) -> TokenStream {
    let ConstMapInput {
        vis,
        name,
        key_ty,
        val_ty,
        entries,
        ..
    } = parse_macro_input!(input as ConstMapInput);

    /* 1.  Re-utilizamos la lógica de const_hashmap! para producir la expresión
    (build_map::<_,_,N>(…)) pero guardamos los pares y calculamos N. */
    let Entries(pairs) = entries;
    let n_pairs = pairs.len();
    let buckets_cap = (n_pairs * 2).next_power_of_two().max(4);
    let buckets_cap_lit = syn::Index::from(buckets_cap);

    let pair_tokens = pairs.iter().map(|e| {
        let k = &e.key;
        let v = &e.value;
        quote! { (#k, #v) }
    });

    /* 2.  Montamos directamente el ItemConst que verá el compilador */
    let generated: ItemConst = parse_quote! {
        #vis const #name: const_hashmap::ConstMap<#key_ty, #val_ty, #buckets_cap_lit> = {
            const_hashmap::build_map::<#key_ty, #val_ty, #buckets_cap_lit>(&[
                #(#pair_tokens),*
            ])
        };
    };

    quote!( #generated ).into()
}
