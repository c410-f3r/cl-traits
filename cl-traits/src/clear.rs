/// See [`clear`](Clear::clear) for more information.
pub trait Clear {
  /// Output
  type Output;

  /// Clears the internal buffer, erasing all elements.
  fn clear(&mut self) -> Self::Output;
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "alloc")]
impl<T> Clear for alloc::vec::Vec<T> {
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Clear for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Clear for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Clear for staticvec::StaticVec<T, N> {
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Clear for tinyvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default
{
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Clear for tinyvec::TinyVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default
{
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear()
  }
}