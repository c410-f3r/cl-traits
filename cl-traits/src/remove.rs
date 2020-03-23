/// Remove
pub trait Remove {
  /// Input type for the [`remove`](Remove::remove)` method.
  type Input;
  /// Output type for the [`remove`](Remove::remove)` method.
  type Output;

  /// Removes an element referenced by `Input`
  fn remove(&mut self, input: Self::Input) -> Self::Output;
}

#[cfg(feature = "alloc")]
impl<T> Remove for alloc::vec::Vec<T> {
  type Input = usize;
  type Output = T;

  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}

#[cfg(feature = "with_arrayvec")]
impl<A> Remove for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;
  type Output = A::Item;

  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}

#[cfg(feature = "with_smallvec")]
impl<A> Remove for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;
  type Output = A::Item;

  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}

#[cfg(feature = "with_staticvec")]
impl<T, const N: usize> Remove for staticvec::StaticVec<T, N> {
  type Input = usize;
  type Output = T;

  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}
