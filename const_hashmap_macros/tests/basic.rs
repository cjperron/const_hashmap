// integration-test: es *otro* crate, así que puede depender del macro
#![feature(const_trait_impl)]
use const_hashmap::{ConstEq, ConstHash};
use const_hashmap_macros::const_hashmap; // ← basta con el nombre del paquete // tu runtime-crate “normal”
use const_hashmap_macros::{ConstEq, ConstHash};

#[test]
fn custom_type_manual_impl() {
    #[derive(Clone, Copy)]
    struct Point {
        x: u32,
        y: u32,
    }
    impl Point {
        const fn new(x: u32, y: u32) -> Self {
            Point { x, y }
        }
    }

    impl const ConstHash for Point {
        fn const_hash(&self) -> u64 {
            self.x as u64 ^ ((self.y as u64) << 32)
        }
    }

    impl const ConstEq for Point {
        fn const_eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    const_hashmap! {
        const PAIRS: Point => u8 = {
        Point::new(1, 2) => 1,
        Point::new(3, 4) => 2,
        };
    }

    assert_eq!(PAIRS.get(&Point::new(1, 2)), Some(&1));
    assert_eq!(PAIRS.get(&Point::new(3, 4)), Some(&2));
}

#[test]
fn auto_lookupworks1() {
    const_hashmap! {
        const LETTERS: &str => u8 = {
            "a" => 1,
            "b" => 2,
        };
    }

    assert_eq!(LETTERS.get(&"a"), Some(&1))
}

#[test]
fn complicated1() {
    const_hashmap! {
        const COLORS: &str => u8 = {
            "red"   => 0,
            "green" => 1,
            "blue"  => 2,
        };
    }
    eprintln!("COLORS: {COLORS:?}");
}

#[test]
fn custom_struct_derived() {
    use const_hashmap_macros::{ConstEq, ConstHash};
    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
    impl Color {
        const fn new(r: u8, g: u8, b: u8) -> Self {
            Color { r, g, b }
        }
    }
    const_hashmap! {
        const COLORS: Color => u8 = {
            Color::new(255, 0, 0) => 1,
            Color::new(0, 255, 0) => 2,
            Color::new(0, 0, 255) => 3,
        };
    }

    assert_eq!(COLORS.get(&Color::new(255, 0, 0)), Some(&1));
}

#[test]
fn custom_enum_derived() {
    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    const_hashmap! {
        const COLORS: Color => u8 = {
            Color::Red   => 1,
            Color::Green => 2,
            Color::Blue  => 3,
        };
    }

    assert_eq!(COLORS.get(&Color::Red), Some(&1));
}

#[test]
fn custom_tuple_struct_derived() {
    use const_hashmap_macros::{ConstEq, ConstHash};
    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    struct Color(u8, u8, u8);
    impl Color {
        const fn new(r: u8, g: u8, b: u8) -> Self {
            Color(r, g, b)
        }
    }
    const_hashmap! {
        const COLORS: Color => u8 = {
            Color::new(255, 0, 0) => 1,
            Color::new(0, 255, 0) => 2,
            Color::new(0, 0, 255) => 3,
        };
    }

    assert_eq!(COLORS.get(&Color::new(255, 0, 0)), Some(&1));
}

#[test]
fn custom_struct_with_enum_derived() {
    use const_hashmap_macros::{ConstEq, ConstHash};
    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    struct Point {
        x: u32,
        y: u32,
        color: Color,
    }
    impl Point {
        const fn new(x: u32, y: u32, color: Color) -> Self {
            Point { x, y, color }
        }
    }
    const_hashmap! {
        const POINTS: Point => u8 = {
            Point::new(1, 2, Color::Red)   => 1,
            Point::new(3, 4, Color::Green) => 2,
            Point::new(5, 6, Color::Blue)  => 3,
        };
    }

    assert_eq!(POINTS.get(&Point::new(1, 2, Color::Red)), Some(&1));
}

#[test]
fn custom_struct_with_inner_struct() {
    use const_hashmap_macros::{ConstEq, ConstHash};
    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    struct Point {
        x: u32,
        y: u32,
    }
    impl Point {
        const fn new(x: u32, y: u32) -> Self {
            Point { x, y }
        }
    }
    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    struct PointPair {
        p1: Point,
        p2: Point,
    }
    impl PointPair {
        const fn new(p1: Point, p2: Point) -> Self {
            PointPair { p1, p2 }
        }
    }

    const_hashmap! {
        const POINTS: PointPair => u8 = {
            PointPair::new(Point::new(1, 2), Point::new(3, 4)) => 1,
            PointPair::new(Point::new(5, 6), Point::new(7, 8)) => 2,
        };
    }
    assert_eq!(
        POINTS.get(&PointPair::new(Point::new(1, 2), Point::new(3, 4))),
        Some(&1)
    );
}

#[test]
fn custom_struct_with_everything() {
    use const_hashmap_macros::{ConstEq, ConstHash};

    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    struct MyStruct {
        a: i8,
        b: u16,
        c: u32,
    }

    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    enum MyEnum {
        Variant1(MyStruct),
        Variant2(u8, bool),
        Variant3(Option<u64>),
    }

    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    struct MyTupleStruct(u64, MyEnum);

    impl MyStruct {
        const fn new(a: i8, b: u16, c: u32) -> Self {
            MyStruct { a, b, c }
        }
    }

    impl MyTupleStruct {
        const fn new(f: u64, e: MyEnum) -> Self {
            MyTupleStruct(f, e)
        }
    }

    const_hashmap! {
        const MY_MAP: MyTupleStruct => u16 = {
            MyTupleStruct::new(1, MyEnum::Variant1(MyStruct::new(1, 2, 3))) => 1,
            MyTupleStruct::new(2, MyEnum::Variant2(4, true)) => 2,
            MyTupleStruct::new(3, MyEnum::Variant3(Some(5))) => 377,
        };
    }

    assert_eq!(
        MY_MAP.get(&MyTupleStruct::new(
            1,
            MyEnum::Variant1(MyStruct::new(1, 2, 3))
        )),
        Some(&1)
    );
    assert_eq!(
        MY_MAP.get(&MyTupleStruct::new(2, MyEnum::Variant2(4, true))),
        Some(&2)
    );
    assert_eq!(
        MY_MAP.get(&MyTupleStruct::new(3, MyEnum::Variant3(Some(5)))),
        Some(&377)
    );
}

#[test]
fn some_constants() {
    #[derive(Clone, Copy, PartialEq, ConstHash, ConstEq)]
    struct Foo {
        a: u8,
        b: u16,
        c: u32,
    }

    const FOO_1: Foo = Foo { a: 1, b: 2, c: 3 };
    const FOO_2: Foo = Foo { a: 4, b: 5, c: 6 };
    const FOO_3: Foo = Foo { a: 7, b: 8, c: 9 };
    const FOO_4: Foo = Foo {
        a: 10,
        b: 11,
        c: 12,
    };

    const_hashmap! {
        const FOO_MAP: Foo => u8 = {
            FOO_1 => 1,
            FOO_2 => 2,
            FOO_3 => 3,
            FOO_4 => 4,
        };
    }

    assert_eq!(FOO_MAP.get(&FOO_1), Some(&1));
}
