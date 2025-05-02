//! # Const HashMap
//!
//! A hash map that can be used in `const` context!
//! It is an abstraction over `&[T]`, and generates an Array of Buckets, where order is determined by the hash of the key. (to support collisions, also need Eq as [`std::collections::HashMap`] requires as well).
//!
//! Ordinarily, you would use `std::collections::HashMap`, but it is not usable in `const` context.
//! Also, it would seem useless to use a hash map in `const` context, but sometimes certain modeling is easier with a hash map than with any other thing, the use of the Key-Value pair is very common in programming, and this library allows you to use it in `const` context.
//!
//! ## Note
//! This library is a work in progress and is not yet ready for production use.
//! It is intended for educational purposes and to demonstrate the use of `const fn`.
//!
//! And for that reason, it is only available as a proof of concept of what future stable rust will be able to do, this crate only compiles in **latest nightly**, with the use of `const_trait_impl`.
//!
//! ## Example
//! Let's say you're modelling a card game, and you want to use a hash map to store the cards in a meaningful way, and conventional arrays are not a viable fit, since indexation only supports usize, and you're **not** a genius, but you care about runtime performance to the max.
//!
//! Since you could have a very big number of cards, and you want to use a hash map to store them, you could use this library to create a hash map of cards, and have them all available at the start of the program, as it is embedded in the binary.
//!
//! ```rust
//! #![feature(const_trait_impl)] // <- only for nightly
//! use const_hashmap::{const_hashmap, ConstEq, ConstHash, ConstMap};
//! use const_hashmap_macros::{ConstEq, ConstHash};
//! #[derive(Debug, Clone, Copy, ConstEq, ConstHash)]
//! struct Card {
//!     name: &'static str,
//!     value: u8,
//!     // Maybe other useful data
//! }
//! #[derive(Debug, Copy, Clone, PartialEq)]
//! struct SpecialProperty {
//!     name: &'static str,
//!     flag1: bool,
//!     flag2: bool,
//!     relevant_number: u32,
//! }
//!
//! impl Card {
//!     const fn new(name: &'static str, value: u8) -> Self {
//!         Card { name, value }
//!     }
//! }
//!
//! impl SpecialProperty {
//!     const fn lazy_new(name: &'static str) -> Self {
//!         SpecialProperty {
//!             name,
//!             flag1: true,
//!             flag2: false,
//!             relevant_number: 0,
//!         }
//!     }
//! }
//! const_hashmap! {
//!     const CARDS: Card => SpecialProperty = {
//!     Card::new("Ace of Spades", 1) => SpecialProperty::lazy_new("Ace of Spades"),
//!     Card::new("Two of Spades", 2) => SpecialProperty::lazy_new("Two of Spades"),
//!     //...and so on
//!     };
//! }
//!
//! assert_eq!(CARDS.get(&Card::new("Ace of Spades", 1)),
//!            Some(&SpecialProperty::lazy_new("Ace of Spades"))); // works!
//! ```

#![feature(const_trait_impl)]
mod core;

pub use const_hashmap_macros::const_hashmap;
pub use core::{build_map, Bucket, ConstEq, ConstHash, ConstMap};

#[cfg(feature = "derive")]
pub use const_hashmap_macros::{ConstEq, ConstHash};
