/// Has some sort of storage that holds a certain number of elements.
pub trait Length {
  /// Outputurn type for the [`length`](Length::length)` method.
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

impl<T, const N: usize> Length for [T; N] {
  type Output = usize;

  fn length(&self) -> Self::Output {
    N
  }
}

#[cfg(feature = "alloc")]
impl<T> Length for alloc::vec::Vec<T> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Length for arrayvec::ArrayVec<crate::ArrayWrapper<T, N>> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

#[cfg(feature = "smallvec")]
impl<T, const N: usize> Length for smallvec::SmallVec<crate::ArrayWrapper<T, N>> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

#[cfg(feature = "staticvec")]
impl<T, const N: usize> Length for staticvec::StaticVec<T, N> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}
