use crate::{Array, ArrayWrapper};

/// Has some way to swap elements of some sort of storage.
pub trait Swap {
  /// Input type for the [`swap`](Swap::swap)` method.
  type Input;
  /// Output type for the [`swap`](Swap::swap)` method.
  type Output;

  /// Swaps two elements
  fn swap(&mut self, input: Self::Input) -> Self::Output;
}

impl<A> Swap for ArrayWrapper<A>
where
  A: Array,
{
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.array.slice_mut().swap(a, b);
  }
}

impl<'a, T> Swap for &'a mut [T] {
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.as_mut().swap(a, b);
  }
}

#[cfg(feature = "alloc")]
impl<T> Swap for alloc::vec::Vec<T> {
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.as_mut_slice().swap(a, b);
  }
}

#[cfg(feature = "with_arrayvec")]
impl<A> Swap for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: Array,
{
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.as_mut_slice().swap(a, b);
  }
}

#[cfg(feature = "with_smallvec")]
impl<A> Swap for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: Array,
{
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.as_mut_slice().swap(a, b);
  }
}

#[cfg(feature = "with_staticvec")]
impl<T, const N: usize> Swap for staticvec::StaticVec<T, N> {
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.as_mut_slice().swap(a, b);
  }
}
