use crate::{Array, ArrayWrapper};

/// Storage is anything that can hold a collection of items
pub trait Storage {
  /// Storage item
  type Item;
}

impl<T> Storage for Option<T> {
  type Item = T;
}

impl<A> Storage for ArrayWrapper<A>
where
  A: Array,
{
  type Item = A::Item;
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

#[cfg(feature = "with-arrayvec")]
impl<A> Storage for arrayvec::ArrayVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Item = A::Item;
}

#[cfg(feature = "with-smallvec")]
impl<A> Storage for smallvec::SmallVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Item = A::Item;
}

#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Storage for staticvec::StaticVec<T, N> {
  type Item = T;
}

#[cfg(feature = "with-tinyvec")]
impl<A> Storage for tinyvec::ArrayVec<ArrayWrapper<A>>
where
  A: Array,
  A::Item: Default,
{
  type Item = A::Item;
}

#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Storage for tinyvec::TinyVec<ArrayWrapper<A>>
where
  A: Array,
  A::Item: Default,
{
  type Item = A::Item;
}
