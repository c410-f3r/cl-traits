//! Instances for documentation tests

#[cfg(feature = "alloc")]
use alloc::{
  collections::{BTreeMap, BTreeSet},
  vec::Vec,
};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

/// Array with three elements
#[inline]
pub fn array() -> [i32; 3] {
  [1, 2, 3]
}

/// `ArrayVec` with three elements
#[cfg(feature = "with-arrayvec")]
#[inline]
pub fn array_vec() -> arrayvec::ArrayVec<i32, 5> {
  let mut vec = arrayvec::ArrayVec::new();
  vec.extend([1, 2, 3].iter().copied());
  vec
}

/// `BTreeMap` with three elements
#[cfg(feature = "alloc")]
#[inline]
pub fn b_tree_map() -> BTreeMap<i32, i32> {
  [(0, 1), (1, 2), (2, 3)].iter().copied().collect()
}

/// `BTreeSet` with three elements
#[cfg(feature = "alloc")]
#[inline]
pub fn b_tree_set() -> BTreeSet<i32> {
  [1, 2, 3].iter().copied().collect()
}

/// `HashMap` with three elements
#[cfg(feature = "std")]
#[inline]
pub fn hash_map() -> HashMap<i32, i32> {
  [(0, 1), (1, 2), (2, 3)].iter().copied().collect()
}

/// `HashSet` with three elements
#[cfg(feature = "std")]
#[inline]
pub fn hash_set() -> HashSet<i32> {
  [1, 2, 3].iter().copied().collect()
}

/// Slice with three elements
#[inline]
pub fn slice() -> &'static [i32] {
  &[1, 2, 3]
}

#[cfg(feature = "with-smallvec")]
#[inline]
/// `SmallVec` with three elements
pub fn small_vec() -> smallvec::SmallVec<[i32; 5]> {
  let mut vec = smallvec::SmallVec::new();
  vec.extend([1, 2, 3].iter().copied());
  vec
}

#[cfg(feature = "with-staticvec")]
#[inline]
/// `StaticVec` with three elements
pub fn static_vec() -> staticvec::StaticVec<i32, 5> {
  let mut vec = staticvec::StaticVec::new();
  vec.extend([1, 2, 3].iter().copied());
  vec
}

#[cfg(feature = "with-tinyvec")]
#[inline]
/// `TinyVec` with three elements
pub fn tiny_vec() -> tinyvec::TinyVec<[i32; 5]> {
  let mut vec = tinyvec::TinyVec::new();
  vec.extend([1, 2, 3].iter().copied());
  vec
}

#[cfg(feature = "with-tinyvec")]
#[inline]
/// `tinyvec::ArrayVec` with three elements
pub fn tiny_vec_array_vec() -> tinyvec::ArrayVec<[i32; 5]> {
  let mut vec = tinyvec::ArrayVec::new();
  vec.extend([1, 2, 3].iter().copied());
  vec
}

#[cfg(feature = "alloc")]
#[inline]
/// `Vec` with three elements
pub fn vec() -> Vec<i32> {
  let mut vec = Vec::with_capacity(5);
  vec.extend([1, 2, 3].iter().copied());
  vec
}
