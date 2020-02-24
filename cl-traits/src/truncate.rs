/// Has some way to truncate some sort of storage.
pub trait Truncate {
  /// Input type for the [`truncate`](Truncate::truncate)` method.
  type Input;
  /// Outputurn type for the [`truncate`](Truncate::truncate)` method.
  type Output;

  /// Truncates the storage.
  fn truncate(&mut self, input: Self::Input) -> Self::Output;
}

#[cfg(feature = "alloc")]
impl<T> Truncate for alloc::vec::Vec<T> {
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}

#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Truncate for arrayvec::ArrayVec<crate::ArrayWrapper<T, N>> {
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}

#[cfg(feature = "smallvec")]
impl<T, const N: usize> Truncate for smallvec::SmallVec<crate::ArrayWrapper<T, N>> {
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}

#[cfg(feature = "staticvec")]
impl<T, const N: usize> Truncate for staticvec::StaticVec<T, N> {
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}
