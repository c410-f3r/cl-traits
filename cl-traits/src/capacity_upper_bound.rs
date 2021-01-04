#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::mem;

/// See [`capacity_upper_bound`](CapacityUpperBound::capacity_upper_bound) for more information.
pub trait CapacityUpperBound {
  /// The maximum theoretical number of elements the implementation is able to store.
  fn capacity_upper_bound(&self) -> usize;
}

/// ```rust
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&Some(0)), 1);
/// ```
impl<T> CapacityUpperBound for Option<T> {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    1
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 3);
/// ```
impl<T, const N: usize> CapacityUpperBound for [T; N] {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    N
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::slice();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 3);
/// ```
impl<T> CapacityUpperBound for &'_ [T] {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&&mut [1, 2, 3][..]), 3);
/// ```
impl<T> CapacityUpperBound for &'_ mut [T] {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
#[cfg(feature = "alloc")]
impl<T> CapacityUpperBound for Vec<T> {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
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
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
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
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    capacity_upper_bound_for_heap::<A::Item>()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// assert_eq!(cl_traits::CapacityUpperBound::capacity_upper_bound(&structure), 5);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> CapacityUpperBound for staticvec::StaticVec<T, N> {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
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
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
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
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    capacity_upper_bound_for_heap::<A::Item>()
  }
}

#[allow(
  // isize::MAX is smaller than usize::MAX
  clippy::as_conversions,
  // For convenience because of selected features
  unused
)]
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
