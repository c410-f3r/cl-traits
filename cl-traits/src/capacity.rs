use crate::{Array, ArrayWrapper};

/// Has some sort of storage that holds a maximum number of elements.
pub trait Capacity {
  /// Output type for the [`capacity`](Capacity::capacity)` method.
  type Output;

  /// The number of elements that can be holded.
  fn capacity(&self) -> Self::Output;
}

impl<A> Capacity for ArrayWrapper<A>
where
  A: Array,
{
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    A::CAPACITY
  }
}

#[cfg(feature = "alloc")]
impl<T> Capacity for alloc::vec::Vec<T> {
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

#[cfg(feature = "with_arrayvec")]
impl<A> Capacity for arrayvec::ArrayVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

#[cfg(feature = "with_smallvec")]
impl<A> Capacity for smallvec::SmallVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

#[cfg(feature = "with_staticvec")]
impl<T, const N: usize> Capacity for staticvec::StaticVec<T, N> {
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}
