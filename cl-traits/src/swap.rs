/// Has some way to swap elements of some sort of storage.
pub trait Swap {
  /// Input type for the [`swap`](Swap::swap)` method.
  type Input;
  /// Outputurn type for the [`swap`](Swap::swap)` method.
  type Output;

  /// Swaps two elements
  fn swap(&mut self, input: Self::Input) -> Self::Output;
}

impl<'a, T> Swap for &'a mut [T] {
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.as_mut().swap(a, b);
  }
}

impl<T, const N: usize> Swap for [T; N] {
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

#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Swap for arrayvec::ArrayVec<crate::ArrayWrapper<T, N>> {
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.as_mut_slice().swap(a, b);
  }
}

#[cfg(feature = "smallvec")]
impl<T, const N: usize> Swap for smallvec::SmallVec<crate::ArrayWrapper<T, N>> {
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.as_mut_slice().swap(a, b);
  }
}

#[cfg(feature = "staticvec")]
impl<T, const N: usize> Swap for staticvec::StaticVec<T, N> {
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, (a, b): Self::Input) -> Self::Output {
    self.as_mut_slice().swap(a, b);
  }
}
