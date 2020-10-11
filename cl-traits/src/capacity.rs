#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [`capacity`](Capacity::capacity) for more information.
pub trait Capacity {
  /// The number of elements the implementation has pre-allocated as an internal buffer. Not
  /// necessarily the current number of inserted elements.
  fn capacity(&self) -> usize;
}

/// ```rust
/// assert_eq!(cl_traits::Capacity::capacity(&Some(0)), 1);
/// ```
impl<T> Capacity for Option<T> {
  #[inline]
  fn capacity(&self) -> usize {
    1
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 3);
/// ```
impl<T, const N: usize> Capacity for [T; N] {
  #[inline]
  fn capacity(&self) -> usize {
    N
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::slice();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
impl<'a, T> Capacity for &'a [T] {
  #[inline]
  fn capacity(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// assert_eq!(cl_traits::Length::length(&&mut [1, 2, 3][..]), 3);
/// ```
impl<'a, T> Capacity for &'a mut [T] {
  #[inline]
  fn capacity(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "alloc")]
impl<T> Capacity for Vec<T> {
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Capacity for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Capacity for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Capacity for staticvec::StaticVec<T, N> {
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Capacity for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Capacity for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}
