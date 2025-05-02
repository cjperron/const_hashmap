#![feature(const_trait_impl)] // impl de rasgos en const
//! Capa pública del crate `const_hashmap`.

mod const_hashmap_impl;
mod derive;
use proc_macro::TokenStream;

/// Macro that generates a `ConstMap` from a list of key-value pairs.
///
/// Example:
/// ```rust
/// #![feature(const_trait_impl)]
/// use const_hashmap_macros::const_hashmap;
/// const_hashmap! {
///      pub const COLORS: &str => u8 = {
///          "red"   => 0,
///          "green" => 1,
///          "blue"  => 2,
///      };
///  }
/// ```
///
/// Generates:
///
/// ```rust
///  #![feature(const_trait_impl)]
///  pub const COLORS: const_hashmap::ConstMap<&str, u8, 8> = {
///      const_hashmap::build_map::<&str, u8, 8>(&[
///          ("red",   0),
///          ("green", 1),
///          ("blue",  2),
///      ])
///  };
/// ```
#[proc_macro]
pub fn const_hashmap(input: TokenStream) -> TokenStream {
    const_hashmap_impl::const_hashmap_impl(input)
}

/// Derives a `ConstHash` implementation for a struct or enum.
///
/// Example:
/// ```rust
/// #![feature(const_trait_impl)]
/// use const_hashmap_macros::ConstHash;
///
/// #[derive(ConstHash)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// This generates a `ConstHash` implementation that can be used in constant contexts.
#[cfg(feature = "derive")]
#[proc_macro_derive(ConstHash)]
pub fn derive_const_hash(input: TokenStream) -> TokenStream {
    derive::derive_const_hash_impl(input)
}

/// Derives a `ConstEq` implementation for a struct or enum.
///
/// Example:
/// ```rust
/// #![feature(const_trait_impl)]
/// use const_hashmap_macros::ConstEq;
///
/// #[derive(ConstEq)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// This generates a `ConstEq` implementation that can be used in constant contexts.
#[cfg(feature = "derive")]
#[proc_macro_derive(ConstEq)]
pub fn derive_const_eq_impl(input: TokenStream) -> TokenStream {
    derive::derive_const_eq_impl(input)
}
