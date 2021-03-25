#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [`with_capacity`](WithCapacity::with_capacity) for more information.
pub trait WithCapacity {
  /// Input
  type Input;

  /// Creates a new instance based on an initial holding capacity provided by `Input`.
  fn with_capacity(input: Self::Input) -> Self;
}

/// ```rust
/// use cl_traits::Capacity;
/// let structure: [i32; 5];
/// structure = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure.capacity(), 5);
/// ```
impl<T, const N: usize> WithCapacity for [T; N]
where
  T: Default,
{
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    crate::create_array(|_| T::default())
  }
}

/// ```rust
/// let structure: Option<i32> = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure, None);
/// ```
impl<T> WithCapacity for Option<T> {
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    None
  }
}

/// ```rust
/// let structure: Vec<i32> = cl_traits::WithCapacity::with_capacity(2);
/// assert_eq!(structure.capacity(), 2);
/// ```
#[cfg(feature = "alloc")]
impl<T> WithCapacity for Vec<T> {
  type Input = usize;

  #[inline]
  fn with_capacity(input: Self::Input) -> Self {
    Vec::with_capacity(input)
  }
}

/// ```rust
/// let structure: arrayvec::ArrayVec<i32, 5>;
/// structure = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<T, const N: usize> WithCapacity for arrayvec::ArrayVec<T, N> {
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    arrayvec::ArrayVec::new()
  }
}

/// ```rust
/// let structure: smallvec::SmallVec<[i32; 5]>;
/// structure = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> WithCapacity for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = usize;

  #[inline]
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

  #[inline]
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
impl<A> WithCapacity for tinyvec::ArrayVec<A>
where
  A: Default + tinyvec::Array,
  A::Item: Default,
{
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    tinyvec::ArrayVec::new()
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// structure = cl_traits::WithCapacity::with_capacity(Default::default());
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> WithCapacity for tinyvec::TinyVec<A>
where
  A: Default + tinyvec::Array,
  A::Item: Default,
{
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    tinyvec::TinyVec::new()
  }
}
