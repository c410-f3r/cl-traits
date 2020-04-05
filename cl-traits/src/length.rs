use crate::{Array, ArrayWrapper};

/// See [`length`](Length::length) for more information.
pub trait Length {
  /// Output
  type Output;

  /// Holds a certain number of elements.
  fn length(&self) -> Self::Output;
}

/// ```rust
/// let structure = cl_traits::doc_tests::slice();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
impl<'a, T> Length for &'a [T] {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// assert_eq!(cl_traits::Length::length(&&mut [1, 2, 3][..]), 3);
/// ```
impl<'a, T> Length for &'a mut [T] {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::array_wrapper();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
impl<A> Length for ArrayWrapper<A>
where
  A: Array,
{
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.array.slice().len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "alloc")]
impl<T> Length for alloc::vec::Vec<T> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::array_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Length for arrayvec::ArrayVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::small_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Length for smallvec::SmallVec<ArrayWrapper<A>>
where
  A: Array,
{
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::static_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Length for staticvec::StaticVec<T, N> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Length for tinyvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::tiny_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Length for tinyvec::TinyVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.len()
  }
}
