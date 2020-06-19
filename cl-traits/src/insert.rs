#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

/// See [`insert`](Insert::insert) for more information.
pub trait Insert {
  /// Input
  type Input;
  /// Output
  type Output;

  /// Inserts an `Input` element.
  fn insert(&mut self, input: Self::Input) -> Self::Output;
}

/// ```rust
/// let mut opt = None;
/// cl_traits::Insert::insert(&mut opt, 3);
/// assert_eq!(opt, Some(3));
/// ```
impl<T> Insert for Option<T> {
  type Input = T;
  type Output = ();

  #[inline]
  fn insert(&mut self, input: Self::Input) {
    *self = Some(input);
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "alloc")]
impl<T> Insert for Vec<T> {
  type Input = (usize, T);
  type Output = ();

  #[inline]
  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Insert for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  type Input = (usize, A::Item);
  type Output = ();

  #[inline]
  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Insert for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = (usize, A::Item);
  type Output = ();

  #[inline]
  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Insert for staticvec::StaticVec<T, N> {
  type Input = (usize, T);
  type Output = ();

  #[inline]
  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Insert for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = (usize, A::Item);
  type Output = ();

  #[inline]
  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Insert for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = (usize, A::Item);
  type Output = ();

  #[inline]
  fn insert(&mut self, input: Self::Input) {
    self.insert(input.0, input.1)
  }
}
