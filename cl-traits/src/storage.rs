/// Storage is anything that can hold a collection of items
pub trait Storage {
  /// Storage item
  type Item;
}

impl<'a, T> Storage for &'a [T] {
  type Item = T;
}

impl<'a, T> Storage for &'a mut [T] {
  type Item = T;
}

#[cfg(feature = "alloc")]
impl<T> Storage for alloc::vec::Vec<T> {
  type Item = T;
}

#[cfg(feature = "const_generics")]
impl<T, const N: usize> Storage for [T; N] {
  type Item = T;
}

#[cfg(feature = "with_arrayvec")]
impl<T, const N: usize> Storage for arrayvec::ArrayVec<crate::ArrayWrapper<T, N>> {
  type Item = T;
}

#[cfg(feature = "with_smallvec")]
impl<T, const N: usize> Storage for smallvec::SmallVec<crate::ArrayWrapper<T, N>> {
  type Item = T;
}

#[cfg(feature = "with_staticvec")]
impl<T, const N: usize> Storage for staticvec::StaticVec<T, N> {
  type Item = T;
}
