#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Storage is anything that can hold a collection of items
pub trait Storage {
  /// Storage item
  type Item;
}

impl<T> Storage for Option<T> {
  type Item = T;
}

impl<T, const N: usize> Storage for [T; N] {
  type Item = T;
}

impl<'a, T> Storage for &'a [T] {
  type Item = T;
}

impl<'a, T> Storage for &'a mut [T] {
  type Item = T;
}

#[cfg(feature = "alloc")]
impl<T> Storage for Vec<T> {
  type Item = T;
}

#[cfg(feature = "with-arrayvec")]
impl<A> Storage for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  type Item = A::Item;
}

#[cfg(feature = "with-smallvec")]
impl<A> Storage for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Item = A::Item;
}

#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Storage for staticvec::StaticVec<T, N> {
  type Item = T;
}

#[cfg(feature = "with-tinyvec")]
impl<A> Storage for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Item = A::Item;
}

#[cfg(feature = "with-tinyvec")]
impl<A> Storage for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Item = A::Item;
}
