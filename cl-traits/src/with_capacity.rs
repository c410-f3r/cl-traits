/// Creates a new instance based on an initial holding capacity.
pub trait WithCapacity {
  /// Input type for the [`with_capacity`](WithCapacity::with_capacity)` method.
  type Input;

  /// New instance with capacity
  fn with_capacity(input: Self::Input) -> Self;
}

#[cfg(feature = "alloc")]
impl<T> WithCapacity for alloc::vec::Vec<T> {
  type Input = usize;

  fn with_capacity(input: Self::Input) -> Self {
    alloc::vec::Vec::with_capacity(input)
  }
}

#[cfg(feature = "with_arrayvec")]
impl<A> WithCapacity for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;

  fn with_capacity(_: Self::Input) -> Self {
    arrayvec::ArrayVec::new()
  }
}

#[cfg(feature = "with_smallvec")]
impl<A> WithCapacity for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;

  fn with_capacity(input: Self::Input) -> Self {
    smallvec::SmallVec::with_capacity(input)
  }
}

#[cfg(feature = "with_staticvec")]
impl<T, const N: usize> WithCapacity for staticvec::StaticVec<T, N> {
  type Input = usize;

  fn with_capacity(_: Self::Input) -> Self {
    staticvec::StaticVec::new()
  }
}
