#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [`length`](Length::length) for more information.
pub trait Length {
  /// Holds a certain number of elements.
  fn length(&self) -> usize;
}

/// ```rust
/// let mut opt = Some(0);
/// assert_eq!(cl_traits::Length::length(&opt), 1);
/// opt.take();
/// assert_eq!(cl_traits::Length::length(&opt), 0);
/// ```
impl<T> Length for Option<T> {
  #[inline]
  fn length(&self) -> usize {
    if self.is_some() {
      1
    } else {
      0
    }
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::slice();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
impl<T> Length for &'_ [T] {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// assert_eq!(cl_traits::Length::length(&&mut [1, 2, 3][..]), 3);
/// ```
impl<T> Length for &'_ mut [T] {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::array();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
impl<T, const N: usize> Length for [T; N] {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "alloc")]
impl<T> Length for Vec<T> {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::array_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Length for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::small_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Length for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::static_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Length for staticvec::StaticVec<T, N> {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Length for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_traits::doc_tests::tiny_vec();
/// assert_eq!(cl_traits::Length::length(&structure), 3);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Length for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}
