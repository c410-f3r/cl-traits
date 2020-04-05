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
impl<T> Remove for alloc::vec::Vec<T> {
  type Input = usize;
  type Output = T;

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
impl<A> Remove for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;
  type Output = A::Item;

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
impl<A> Remove for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;
  type Output = A::Item;

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
impl<A> Remove for tinyvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default
{
  type Input = usize;
  type Output = A::Item;

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
impl<A> Remove for tinyvec::TinyVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default
{
  type Input = usize;
  type Output = A::Item;

  fn remove(&mut self, input: Self::Input) -> Self::Output {
    self.remove(input)
  }
}