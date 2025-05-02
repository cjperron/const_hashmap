//! # Const HashMap
//!
//! A small, fixed-size hash map you can build at compile time and use in `const`
//! contexts. Under the hood it stores an array of [`Bucket<K, V>`] of power-of-two
//! size, uses FNV-1a hashing and linear probing, and requires `K: ConstHash + ConstEq`.
//!
//! Unlike `std::collections::HashMap`, this map can be constructed and queried
//! at compile time via `const fn`.  It’s intended for _compile-time_ lookup
//! tables (e.g. embedded constants), not for dynamic resizing.
//!
//! ## Feature flags
//!
//! - **Nightly only**: this crate depends on `#![feature(const_trait_impl)]`
//!   and other `const fn` nightly features.
//! - No additional features in v0.1: everything is available in the core crate.
//!
//! > **Work in progress**: this is a proof-of-concept. API may change.
//!
//! ## Quickstart
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! const_hashmap = "0.1.0"
//! ```
//!
//! In your crate root (or lib):
//! ```rust
//! #![feature(const_trait_impl)]  // nightly only
//! use const_hashmap::{ConstMap, ConstHash, ConstEq, build_map, Bucket};
//!
//! // 1. Implement ConstHash + ConstEq for your key type. &str is implemented from the get go
//!
//!
//! // 2. Build a small const map from a slice of (key, value) pairs.
//! //    `N` is a power of two ≥ 2×pairs.len().
//! const COLORS: ConstMap<&'static str, u8, 8> = build_map(&[
//!     ("red",   1),
//!     ("green", 2),
//!     ("blue",  3),
//! ]);
//!
//! // 3. Query at compile time or runtime.
//! const RED_ID: Option<&u8> = COLORS.get(&"red");
//! const MISSING: Option<&u8> = COLORS.get(&"purple");
//!
//! fn main() {
//!     assert_eq!(RED_ID, Some(&1));
//!     assert_eq!(MISSING, None);
//! }
//! ```
//!
//! For more advanced keys (structs, enums), implement `ConstHash` and `ConstEq`

#![feature(const_trait_impl)]
mod core;

//pub use const_hashmap_macros::const_hashmap;
pub use core::{Bucket, ConstEq, ConstHash, ConstMap, build_map};

//#[cfg(feature = "derive")]
//pub use const_hashmap_macros::{ConstEq, ConstHash};
