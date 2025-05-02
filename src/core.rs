use core::marker::PhantomData;
use std::ops::Index;

#[const_trait]
/// Minimalist substitute of `PartialEq` / `Eq` that we **can** call in `const fn`.
pub trait ConstEq {
    fn const_eq(&self, other: &Self) -> bool;
}

/* impls básicos --------------------------------------------------------- */
macro_rules! impl_eq_ints {
    ($($t:ty),*) => {$(
        impl const ConstEq for $t {
            #[inline(always)]
            fn const_eq(&self, other: &Self) -> bool { *self == *other }  // ← operador builtin (sí es const)
        }
    )*};
}
impl_eq_ints!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool
);

impl const ConstEq for char {
    fn const_eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl const ConstEq for &str {
    fn const_eq(&self, other: &&str) -> bool {
        let b1 = self.as_bytes();
        let b2 = other.as_bytes();

        if b1.len() != b2.len() {
            return false;
        }
        let mut i = 0;
        while i < b1.len() {
            // la comparación es entre u8, que sí es const-safe
            if b1[i] != b2[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

/// Minimalist substitute of `Hash` that we **can** call in `const fn`.
#[const_trait]
pub trait ConstHash {
    fn const_hash(&self) -> u64;
}

/* ► impls básicos --------------------------------------------------------- */
macro_rules! impl_hash_int {
    ($($t:ty),*) => {$(
        impl const ConstHash for $t {
            #[inline(always)]
            fn const_hash(&self) -> u64 { *self as u64 }
        }
    )*};
}

impl_hash_int!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool
);

impl const ConstHash for char {
    #[inline(always)]
    fn const_hash(&self) -> u64 {
        *self as u32 as u64
    }
}

impl const ConstHash for &str {
    fn const_hash(&self) -> u64 {
        /* FNV-1a 64 bit ─ totalmente *const* */
        const FNV_PRIME: u64 = 0x0000_0100_0000_01B3;
        const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

        let mut hash = OFFSET;
        let mut i = 0;
        let bytes = self.as_bytes();
        while i < bytes.len() {
            hash ^= bytes[i] as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
            i += 1;
        }
        hash
    }
}

/*  en src/lib.rs de const_map  ----------------------------------------- */
impl<T: ~const ConstHash + ?Sized> const ConstHash for &T {
    #[inline(always)]
    fn const_hash(&self) -> u64 {
        (*self).const_hash()
    }
}

impl<T: ~const ConstEq + ?Sized> const ConstEq for &T {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> bool {
        (*self).const_eq(*other)
    }
}

impl<T: ~const ConstHash + Sized> const ConstHash for Option<T> {
    #[inline(always)]
    fn const_hash(&self) -> u64 {
        match self {
            Some(v) => v.const_hash().saturating_add(1),
            None => 0,
        }
    }
}

impl<T: ~const ConstEq + Sized> const ConstEq for Option<T> {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(v1), Some(v2)) => v1.const_eq(v2),
            (None, None) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Represents a single bucket in the hash map.
pub enum Bucket<K, V> {
    Empty,
    Filled { key: K, val: V },
}

#[derive(Debug, Clone, Copy)]
/// Data structure that represents a hash map with a fixed size determined at compile time.
/// Use the macro `const_map!` to create a `ConstMap` in a very friendly manner.
pub struct ConstMap<K, V, const N: usize>
where
    K: ConstHash + ConstEq,
{
    pub buckets: [Bucket<K, V>; N],
    len: usize,
    pub _phantom: PhantomData<fn() -> (K, V)>, // ← para evitar warnings (?)
}

impl<K, V, const N: usize> ConstMap<K, V, N>
where
    K: ConstHash + ConstEq,
{
    /// Creates a map from the already constructed `buckets` table.
    ///
    /// *Safety*: The *proc-macro* must ensure that there are no duplicate keys
    /// or irresolvable collisions.
    pub const fn new(buckets: [Bucket<K, V>; N]) -> Self {
        let mut len = 0;
        let mut i = 0;
        while i < N {
            if let Bucket::Filled { .. } = buckets[i] {
                len += 1;
            }
            i += 1;
        }
        Self {
            len,
            buckets,
            _phantom: PhantomData,
        }
    }

    /// Amortized O(1) lookup using linear probing.
    pub const fn get(&self, key: &K) -> Option<&V>
    where
        K: ~const ConstHash + ~const ConstEq, // ← aquí está la clave
    {
        let mut idx = (key.const_hash() as usize) & (N - 1);
        let mut probes = 0;

        while probes < N {
            match &self.buckets[idx] {
                Bucket::Empty => return None,
                Bucket::Filled { key: k, val: v } if k.const_eq(key) => return Some(v),
                _ => {
                    idx = (idx + 1) & (N - 1); // siguiente bucket
                    probes += 1;
                }
            }
        }
        None
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn items(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets.iter().filter_map(|bucket| {
            if let Bucket::Filled { key, val } = bucket {
                Some((key, val))
            } else {
                None
            }
        })
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.buckets.iter().filter_map(|bucket| {
            if let Bucket::Filled { key, .. } = bucket {
                Some(key)
            } else {
                None
            }
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.buckets.iter().filter_map(|bucket| {
            if let Bucket::Filled { val, .. } = bucket {
                Some(val)
            } else {
                None
            }
        })
    }
    /// Converts the `ConstMap` into a `HashMap`. both `K` and `V` must implement `Clone`, as we are copying them onto the heap.
    pub fn to_hashmap(&self) -> std::collections::HashMap<K, V>
    where
        K: std::hash::Hash + Eq + Clone,
        V: Clone,
    {
        let mut map = std::collections::HashMap::new();
        for bucket in &self.buckets {
            if let Bucket::Filled { key, val } = bucket {
                map.insert(key.clone(), val.clone());
            }
        }
        map
    }
}

impl<K, V, const N: usize> Index<K> for ConstMap<K, V, N>
where
    K: ConstHash + ConstEq + Copy,
{
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        self.get(&key).unwrap()
    }
}

/// El caller debe de asegurarse de que el tamaño del array es potencia de 2 y que pairs esta armado correctamente.
pub const fn build_map<K, V, const N: usize>(pairs: &[(K, V)]) -> ConstMap<K, V, N>
where
    K: ~const ConstHash + ~const ConstEq + Copy,
    V: Copy,
{
    // 1. Array de buckets vacío
    let mut buckets: [Bucket<K, V>; N] = [Bucket::Empty; N];

    // 2. Insertamos uno a uno
    let mut p = 0;
    while p < pairs.len() {
        let (ref key, ref val) = pairs[p];
        let mut idx = (key.const_hash() as usize) & (N - 1);
        loop {
            match &buckets[idx] {
                Bucket::Empty => {
                    buckets[idx] = Bucket::Filled {
                        key: *key,
                        val: *val,
                    };
                    break;
                }
                _ => idx = (idx + 1) & (N - 1),
            }
        }
        p += 1;
    }

    ConstMap::new(buckets)
}
