/// Has some way to push an element into some sort of storage.
pub trait Push {
  /// Input type for the [`push`](Push::push)` method.
  type Input;
  /// Output type for the [`push`](Push::push)` method.
  type Output;

  /// Pushes an element
  fn push(&mut self, input: Self::Input) -> Self::Output;
}

#[cfg(feature = "alloc")]
impl<T> Push for alloc::vec::Vec<T> {
  type Input = T;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input);
  }
}

#[cfg(feature = "with_arrayvec")]
impl<A> Push for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input);
  }
}

#[cfg(feature = "with_smallvec")]
impl<A> Push for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input);
  }
}

#[cfg(feature = "with_staticvec")]
impl<T, const N: usize> Push for staticvec::StaticVec<T, N> {
  type Input = T;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input);
  }
}
