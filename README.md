# const\_hashmap

Compile-time hash map for any key type implementing `ConstHash + ConstEq`.

> **Work in Progress:** API may evolve. Nightly Rust only.

## Features

* **Const Construction**: Build fixed-size hash maps at compile time with `const fn`.
* **Standard Library Support**: Exposes both `no_std` and `std` APIs; can convert to a runtime `std::collections::HashMap` via `into_hashmap()`.
* **Optional Derive**: `#[derive(ConstHash, ConstEq)]` via feature flag.
* **Zero Dependencies**: Minimal, using only `const_trait_impl` on nightly.

## Installation

```toml
[dependencies]
const_hashmap = "0.1.1"
# To enable derive macros:
const_hashmap = { version = "0.1.1", features = ["derive"] }
```

Nightly is required:

```rust
#![feature(const_trait_impl)]
```

## Quick Start

```rust
#![feature(const_trait_impl)]
use const_hashmap::{ConstMap, ConstHash, ConstEq, build_map, Bucket};

// 1. Implement hashing & equality for your key type. &str by default

// 2. Build a const map:
const COLORS: ConstMap<&'static str, u8, 8> = build_map(&[
    ("red",   1),
    ("green", 2),
    ("blue",  3),
]);

// 3. Query at compile time:
const RED_ID: Option<&u8> = COLORS.get(&"red");

// 4. Convert to a runtime HashMap if needed:
let runtime_map: std::collections::HashMap<_, _> = COLORS.into_hashmap();

fn main() {
    assert_eq!(RED_ID, Some(&1));
    assert_eq!(runtime_map["green"], 2);
}
```

### Macro Form

```rust
#![feature(const_trait_impl)]
use const_hashmap::{ConstHash, ConstEq, const_hashmap};

#[derive(ConstHash, ConstEq)]
struct Point { x: i32, y: i32 }

const_hashmap! {
    const POINTS: Point => &str = {
        Point { x:0, y:0 } => "origin",
        Point { x:1, y:1 } => "diag",
    };
}

fn main() {
    assert_eq!(POINTS.get(&Point{x:1,y:1}), Some(&"diag"));
}
```

### Additional Example: Nested Lookups

```rust
#![feature(const_trait_impl)]
use const_hashmap::{ConstMap, ConstHash, ConstEq, build_map};

#[derive(Clone, Copy)]
enum Color { Red, Green, Blue }
impl const ConstHash for Color {
    fn const_hash(&self) -> u64 { *self as u64 }
}
impl const ConstEq for Color {
    fn const_eq(&self, other: &Self) -> bool { *self as u8 == *other as u8 }
}

// Map Color -> numeric code, then code -> name
const CODES: ConstMap<Color, u8, 4> = build_map(&[
    (Color::Red,   1),
    (Color::Green, 2),
    (Color::Blue,  3),
]);
const NAMES: ConstMap<u8, &'static str, 8> = build_map(&[
    (1, "Red"), (2, "Green"), (3, "Blue"),
]);

fn paint(c: Color) -> &'static str {
    let code = CODES.get(&c).unwrap();
    *NAMES.get(code).unwrap()
}

fn main() {
    assert_eq!(paint(Color::Green), "Green");
}
```

## Roadmap

* 📦 Publish stable version on stable Rust when `const_trait_impl` stabilizes.
* 🔧 More ergonomic `Index` support in const context.
* ⚙️ Support fallible construction and custom collision strategies.
* Generally make it more user friendly over time, and implement most of std/core API.&#x20;

## License

Dual-licensed under MIT OR Apache-2.0. See [LICENSE](LICENSE) for details.
