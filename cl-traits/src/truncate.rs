/// Has some way to truncate some sort of storage.
pub trait Truncate {
  /// Input type for the [`truncate`](Truncate::truncate)` method.
  type Input;
  /// Output type for the [`truncate`](Truncate::truncate)` method.
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

#[cfg(feature = "with_arrayvec")]
impl<A> Truncate for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}

#[cfg(feature = "with_smallvec")]
impl<A> Truncate for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}

#[cfg(feature = "with_staticvec")]
impl<T, const N: usize> Truncate for staticvec::StaticVec<T, N> {
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}
