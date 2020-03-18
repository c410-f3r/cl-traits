use crate::{Array, ArrayWrapper};

/// Has some sort of storage that holds a certain number of elements.
pub trait Length {
  /// Output type for the [`length`](Length::length)` method.
  type Output;

  /// The number of elements.
  fn length(&self) -> Self::Output;
}

impl<'a, T> Length for &'a [T] {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

impl<'a, T> Length for &'a mut [T] {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

impl<A> Length for ArrayWrapper<A>
where
  A: Array,
{
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.array.slice().len()
  }
}

#[cfg(feature = "alloc")]
impl<T> Length for alloc::vec::Vec<T> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

#[cfg(feature = "with_arrayvec")]
impl<A> Length for arrayvec::ArrayVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

#[cfg(feature = "with_smallvec")]
impl<A> Length for smallvec::SmallVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

#[cfg(feature = "with_staticvec")]
impl<T, const N: usize> Length for staticvec::StaticVec<T, N> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}
