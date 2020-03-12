/// Has some sort of storage that holds a maximum number of elements.
pub trait Capacity {
  /// Output type for the [`capacity`](Capacity::capacity)` method.
  type Output;

  /// The number of elements that can be holded.
  fn capacity(&self) -> Self::Output;
}

#[cfg(feature = "const_generics")]
impl<T, const N: usize> Capacity for [T; N] {
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    N
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
impl<T, const N: usize> Capacity for arrayvec::ArrayVec<crate::ArrayWrapper<T, N>> {
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

#[cfg(feature = "with_smallvec")]
impl<T, const N: usize> Capacity for smallvec::SmallVec<crate::ArrayWrapper<T, N>> {
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
