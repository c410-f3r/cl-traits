//! Instances for documentation tests

use crate::ArrayWrapper;

/// `ArrayWrapper` with three elements
pub fn array_wrapper() -> ArrayWrapper<[i32; 3]> {
    crate::ArrayWrapper::new([1, 2, 3])
}

/// Slice with three elements
pub fn slice() -> &'static [i32] {
    &[1, 2, 3]
}

#[cfg(feature = "alloc")]
/// `Vec` with three elements
pub fn vec() -> alloc::vec::Vec<i32> {
    let mut vec = alloc::vec::Vec::with_capacity(5);
    vec.extend([1, 2, 3].iter().copied());
    vec
}

#[cfg(feature = "with-arrayvec")]
/// `ArrayVec` with three elements
pub fn array_vec() -> arrayvec::ArrayVec<ArrayWrapper<[i32; 5]>> {
    let mut vec = arrayvec::ArrayVec::new();
    vec.extend([1, 2, 3].iter().copied());
    vec
}

#[cfg(feature = "with-smallvec")]
/// `SmallVec` with three elements
pub fn small_vec() -> smallvec::SmallVec<ArrayWrapper<[i32; 5]>> {
    let mut vec = smallvec::SmallVec::new();
    vec.extend([1, 2, 3].iter().copied());
    vec
}

#[cfg(feature = "with-staticvec")]
/// `StaticVec` with three elements
pub fn static_vec() -> staticvec::StaticVec<i32, 5> {
    let mut vec = staticvec::StaticVec::new();
    vec.extend([1, 2, 3].iter().copied());
    vec
}

#[cfg(feature = "with-tinyvec")]
/// `tinyvec::ArrayVec` with three elements
pub fn tiny_vec_array_vec() -> tinyvec::ArrayVec<ArrayWrapper<[i32; 5]>> {
    let mut vec = tinyvec::ArrayVec::new();
    vec.extend([1, 2, 3].iter().copied());
    vec
}

#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
/// `TinyVec` with three elements
pub fn tiny_vec() -> tinyvec::TinyVec<ArrayWrapper<[i32; 5]>> {
    let mut vec = tinyvec::TinyVec::new();
    vec.extend([1, 2, 3].iter().copied());
    vec
}