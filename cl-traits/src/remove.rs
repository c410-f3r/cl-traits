#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

/// See [`remove`](Remove::remove) for more information.
pub trait Remove {
  /// Input
  type Input;
  /// Output
  type Output;

  /// Removes an element referenced by `Input`.
  fn remove(&mut self, input: Self::Input) -> Self::Output;
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "alloc")]
impl<T> Remove for Vec<T> {
  type Input = usize;
  type Output = T;

  #[inline]
  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Remove for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  type Input = usize;
  type Output = A::Item;

  #[inline]
  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Remove for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = usize;
  type Output = A::Item;

  #[inline]
  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Remove for staticvec::StaticVec<T, N> {
  type Input = usize;
  type Output = T;

  #[inline]
  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Remove for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = usize;
  type Output = A::Item;

  #[inline]
  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Remove for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = usize;
  type Output = A::Item;

  #[inline]
  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}
