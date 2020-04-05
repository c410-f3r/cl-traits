/// See [`Truncate`](Truncate::truncate) for more information.
pub trait Truncate {
  /// Input
  type Input;
  /// Output
  type Output;

  /// Truncates the storage, delimiting its length by `Input`.
  fn truncate(&mut self, input: Self::Input) -> Self::Output;
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "alloc")]
impl<T> Truncate for alloc::vec::Vec<T> {
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "with-arrayvec")]
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

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "with-smallvec")]
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

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Truncate for staticvec::StaticVec<T, N> {
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Truncate for tinyvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Truncate for tinyvec::TinyVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input)
  }
}
