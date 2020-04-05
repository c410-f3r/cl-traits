/// See [`with_capacity`](WithCapacity::with_capacity) for more information.
pub trait WithCapacity {
  /// Input
  type Input;

  /// Creates a new instance based on an initial holding capacity provided by `Input`.
  fn with_capacity(input: Self::Input) -> Self;
}

/// ```rust
/// let structure: Vec<i32> = cl_traits::WithCapacity::with_capacity(2);
/// assert_eq!(structure.capacity(), 2);
/// ```
#[cfg(feature = "alloc")]
impl<T> WithCapacity for alloc::vec::Vec<T> {
  type Input = usize;

  fn with_capacity(input: Self::Input) -> Self {
    alloc::vec::Vec::with_capacity(input)
  }
}

/// ```rust
/// let structure: arrayvec::ArrayVec<cl_traits::ArrayWrapper<[i32; 5]>>;
/// structure = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> WithCapacity for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;

  fn with_capacity(_: Self::Input) -> Self {
    arrayvec::ArrayVec::new()
  }
}

/// ```rust
/// let structure: smallvec::SmallVec<cl_traits::ArrayWrapper<[i32; 5]>>;
/// structure = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> WithCapacity for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Input = usize;

  fn with_capacity(input: Self::Input) -> Self {
    smallvec::SmallVec::with_capacity(input)
  }
}

/// ```rust
/// let structure: staticvec::StaticVec<i32, 5>;
/// structure = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> WithCapacity for staticvec::StaticVec<T, N> {
  type Input = usize;

  fn with_capacity(_: Self::Input) -> Self {
    staticvec::StaticVec::new()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// structure = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> WithCapacity for tinyvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Input = usize;

  fn with_capacity(_: Self::Input) -> Self {
    tinyvec::ArrayVec::new()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// structure = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> WithCapacity for tinyvec::TinyVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Input = usize;

  fn with_capacity(_: Self::Input) -> Self {
    tinyvec::TinyVec::new()
  }
}
