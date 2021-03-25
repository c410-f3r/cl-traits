#[cfg(feature = "alloc")]
use alloc::vec::Vec;

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
/// let structure: Option<i32> = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure, None);
/// ```
impl<T> Truncate for Option<T> {
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    if input == 0 {
      *self = None;
    }
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "alloc")]
impl<T> Truncate for Vec<T> {
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<T, const N: usize> Truncate for arrayvec::ArrayVec<T, N> {
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Truncate for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input);
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

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Truncate for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Truncate for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input);
  }
}
