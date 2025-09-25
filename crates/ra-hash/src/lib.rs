//! A set of hash-related definitions.
//!
//! This crate is essentially a wrapper around some out-of-tree library crates.
//! It will be one of the first crates built and should not depend on any crates
//! local to rust-analzyzer.

use std::hash::{BuildHasher as _, Hash};

pub use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet, FxHasher};

pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, FxBuildHasher>;
pub type FxIndexSet<T> = indexmap::IndexSet<T, FxBuildHasher>;
pub type IndexEntry<'a, K, V> = indexmap::map::Entry<'a, K, V>;

pub type FxDashMap<K, V> = dashmap::DashMap<K, V, FxBuildHasher>;
pub type FxDashSet<K> = dashmap::DashSet<K, FxBuildHasher>;
pub type DashEntry<'a, K, V> = dashmap::Entry<'a, K, V>;

pub fn fxhashmap_with_capacity<K, V>(n: usize) -> FxHashMap<K, V> {
    FxHashMap::with_capacity_and_hasher(n, FxBuildHasher)
}

pub const fn fxindexmap<K, V>() -> FxIndexMap<K, V> {
    FxIndexMap::with_hasher(FxBuildHasher)
}

pub fn fxindexmap_with_capacity<K, V>(n: usize) -> FxIndexMap<K, V> {
    FxIndexMap::with_capacity_and_hasher(n, FxBuildHasher)
}

pub fn fxindexset_with_capacity<K>(n: usize) -> FxIndexSet<K> {
    FxIndexSet::with_capacity_and_hasher(n, FxBuildHasher)
}

/// Hash a single value using the FxHash algorithm
pub fn fxhash_one<T: Hash>(x: T) -> u64 {
    FxBuildHasher::default().hash_one(x)
}
