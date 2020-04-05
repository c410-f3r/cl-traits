use crate::{Array, ArrayWrapper};

/// See [`capacity`](Capacity::capacity) for more information.
pub trait Capacity {
  /// Output
  type Output;
  
  /// The number of elements that the implementation is able to store, not necessary equal
  /// to its length.
  fn capacity(&self) -> Self::Output;
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_wrapper();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 3);
/// ```
impl<A> Capacity for ArrayWrapper<A>
where
  A: Array,
{
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    A::CAPACITY
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::slice();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
impl<'a, T> Capacity for &'a [T] {
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// assert_eq!(cl_traits::Length::length(&&mut [1, 2, 3][..]), 3);
/// ```
impl<'a, T> Capacity for &'a mut [T] {
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "alloc")]
impl<T> Capacity for alloc::vec::Vec<T> {
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Capacity for arrayvec::ArrayVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Capacity for smallvec::SmallVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Capacity for staticvec::StaticVec<T, N> {
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Capacity for tinyvec::ArrayVec<ArrayWrapper<A>>
where
  A: Array,
  A::Item: Default,
{
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// assert_eq!(cl_traits::Capacity::capacity(&structure), 5);
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Capacity for tinyvec::TinyVec<ArrayWrapper<A>>
where
  A: Array,
  A::Item: Default,
{
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.capacity()
  }
}

