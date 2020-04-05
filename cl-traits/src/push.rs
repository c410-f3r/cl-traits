/// See [`push`](Push::push) for more information.
pub trait Push {
  /// Input
  type Input;
  /// Output
  type Output;

  /// Pushes an element, increasing the storage length.
  fn push(&mut self, input: Self::Input) -> Self::Output;
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "alloc")]
impl<T> Push for alloc::vec::Vec<T> {
  type Input = T;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Push for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Push for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Push for staticvec::StaticVec<T, N> {
  type Input = T;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Push for tinyvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Push for tinyvec::TinyVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) -> Self::Output {
    self.push(input)
  }
}
