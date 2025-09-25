//! A set of hash-based collection definitions, all in one location

use std::hash::{Hash, BuildHasher as _};

pub use rustc_hash::{FxBuildHasher, FxHasher, FxHashMap, FxHashSet};


pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, FxBuildHasher>;
pub type FxIndexSet<T> = indexmap::IndexSet<T, FxBuildHasher>;

pub type FxDashMap<K, V> = dashmap::DashMap<K, V, FxBuildHasher>;
pub type FxDashSet<K> = dashmap::DashSet<K, FxBuildHasher>;

pub fn hash_one<T: Hash>(x: T) -> u64 {
    FxBuildHasher::default().hash_one(x)
}

pub fn fxhashmap_with_capacity<K: Hash, V>(n: usize) -> FxHashMap<K, V> {
    FxHashMap::with_capacity_and_hasher(n, FxBuildHasher)
}

pub fn fxindexmap_with_capacity<K: Hash, V>(n: usize) -> FxIndexMap<K, V> {
    FxIndexMap::with_capacity_and_hasher(n, FxBuildHasher)
}

pub const fn fxindexmap<K: Hash, V>() -> FxIndexMap<K, V> {
    FxIndexMap::with_hasher(FxBuildHasher)
}

// FIXME: Allow for FxIndexSet::with_capacity so the hasher does not have to be specified (base-db/src/input.rs)