use crate::{Array, ArrayWrapper};
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

/// See [`Swap`](Swap::swap) for more information.
pub trait Swap {
  /// Input
  type Input;
  /// Output
  type Output;

  /// Swaps two elements referencied by `Input`.
  fn swap(&mut self, input: Self::Input) -> Self::Output;
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_wrapper();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
impl<A> Swap for ArrayWrapper<A>
where
  A: Array,
{
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) {
    self.array.slice_mut().swap(a, b)
  }
}

/// ```rust
/// let mut structure = &mut [1, 2, 3][..];
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
impl<'a, T> Swap for &'a mut [T] {
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) {
    self.as_mut().swap(a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "alloc")]
impl<T> Swap for Vec<T> {
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) {
    self.as_mut_slice().swap(a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Swap for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) {
    self.as_mut_slice().swap(a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Swap for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) {
    self.as_mut_slice().swap(a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Swap for staticvec::StaticVec<T, N> {
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) {
    self.as_mut_slice().swap(a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Swap for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) {
    self.as_mut_slice().swap(a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Swap for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) {
    self.as_mut_slice().swap(a, b)
  }
}
