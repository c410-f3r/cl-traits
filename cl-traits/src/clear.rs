#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

/// See [`clear`](Clear::clear) for more information.
pub trait Clear {
  /// Output
  type Output;

  /// Clears the internal buffer, erasing all elements.
  fn clear(&mut self) -> Self::Output;
}

/// ```rust
/// let mut opt = Some(0);
/// cl_traits::Clear::clear(&mut opt);
/// assert_eq!(opt, None);
/// ```
impl<T> Clear for Option<T> {
  type Output = ();

  #[inline]
  fn clear(&mut self) {
    self.take();
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "alloc")]
impl<T> Clear for Vec<T> {
  type Output = ();

  #[inline]
  fn clear(&mut self) {
    self.clear()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Clear for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  type Output = ();

  #[inline]
  fn clear(&mut self) {
    self.clear()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Clear for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Output = ();

  #[inline]
  fn clear(&mut self) {
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

  #[inline]
  fn clear(&mut self) {
    self.clear()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Clear for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Output = ();

  #[inline]
  fn clear(&mut self) {
    self.clear()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Clear for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Output = ();

  #[inline]
  fn clear(&mut self) {
    self.clear()
  }
}
