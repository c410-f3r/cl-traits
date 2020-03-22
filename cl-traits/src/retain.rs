/// Retain
pub trait Retain<I> {
  /// Output type for the [`insert`](Insert::insert)` method.
  type Output;

  /// Retains only the elements specified by the `I` predicate.
  fn retain(&mut self, input: I) -> Self::Output;
}

#[cfg(feature = "alloc")]
impl<F, T> Retain<F> for alloc::vec::Vec<T>
where
  F: FnMut(&T) -> bool,
{
  type Output = ();

  fn retain(&mut self, input: F) {
    self.retain(input)
  }
}

#[cfg(feature = "with_arrayvec")]
impl<A, F> Retain<F> for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  F: FnMut(&A::Item) -> bool,
{
  type Output = ();

  fn retain(&mut self, mut input: F) {
    self.retain(|i| input(i))
  }
}

#[cfg(feature = "with_smallvec")]
impl<A, F> Retain<F> for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  F: FnMut(&A::Item) -> bool,
{
  type Output = ();

  fn retain(&mut self, mut input: F) {
    self.retain(|i| input(i))
  }
}

#[cfg(feature = "with_staticvec")]
impl<F, T, const N: usize> Retain<F> for staticvec::StaticVec<T, N>
where
  F: FnMut(&T) -> bool,
{
  type Output = ();

  fn retain(&mut self, input: F) {
    self.retain(input)
  }
}
