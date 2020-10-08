#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::mem;

/// See [`capacity_upper_bound`](CapacityUpperBound::capacity_upper_bound) for more information.
pub trait CapacityUpperBound {
  /// Output
  type Output;

  /// The maximum theoretical number of elements the implementation is able to store.
  fn capacity_upper_bound(&self) -> Self::Output;
}

/// ```rust
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&Some(0)), 1);
/// ```
impl<T> CapacityUpperBound for Option<T> {
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    1
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 3);
/// ```
impl<T, const N: usize> CapacityUpperBound for [T; N] {
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    N
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::slice();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 3);
/// ```
impl<'a, T> CapacityUpperBound for &'a [T] {
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&&mut [1, 2, 3][..]), 3);
/// ```
impl<'a, T> CapacityUpperBound for &'a mut [T] {
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
#[cfg(feature = "alloc")]
impl<T> CapacityUpperBound for Vec<T> {
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    capacity_upper_bound_for_heap::<T>()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 5);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> CapacityUpperBound for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    A::CAPACITY
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> CapacityUpperBound for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    capacity_upper_bound_for_heap::<A::Item>()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 5);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> CapacityUpperBound for staticvec::StaticVec<T, N> {
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    N
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 5);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> CapacityUpperBound for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    A::CAPACITY
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> CapacityUpperBound for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Output = usize;

  #[inline]
  fn capacity_upper_bound(&self) -> Self::Output {
    capacity_upper_bound_for_heap::<A::Item>()
  }
}

#[allow(clippy::as_conversions, unused)]
#[inline]
fn capacity_upper_bound_for_heap<T>() -> usize {
  let size_of_t = mem::size_of::<T>();
  let isize_max_usize = isize::MAX as usize;
  if size_of_t > isize_max_usize {
    0
  } else {
    isize_max_usize.wrapping_div(size_of_t)
  }
}
