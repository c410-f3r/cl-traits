/// Insert
pub trait Insert {
  /// Input type for the [`insert`](Insert::insert)` method.
  type Input;
  /// Output type for the [`insert`](Insert::insert)` method.
  type Output;

  /// Inserts a element provided by `Input` into this storage.
  fn insert(&mut self, input: Self::Input) -> Self::Output;
}

#[cfg(feature = "alloc")]
impl<T> Insert for alloc::vec::Vec<T> {
  type Input = (usize, T);
  type Output = ();

  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}

#[cfg(feature = "with_arrayvec")]
impl<A> Insert for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = (usize, A::Item);
  type Output = ();

  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}

#[cfg(feature = "with_smallvec")]
impl<A> Insert for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = (usize, A::Item);
  type Output = ();

  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}

#[cfg(feature = "with_staticvec")]
impl<T, const N: usize> Insert for staticvec::StaticVec<T, N> {
  type Input = (usize, T);
  type Output = ();

  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}
