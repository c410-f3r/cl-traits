#[cfg(feature = "alloc")]
use alloc::vec::Vec;

macro_rules! vec_swap {
  ($v:expr, $a:expr, $b:expr) => {{
    if $a >= $v.len() || $b >= $v.len() {
      return Err(());
    }
    let slice: &mut [_] = $v.as_mut();
    slice.swap($a, $b);
    Ok(())
  }};
}

/// See [`Swap`](Swap::swap) for more information.
pub trait Swap {
  /// Input
  type Input;
  /// Output
  type Output;

  /// Swaps two elements referencied by `Input`.
  fn swap(&mut self, input: Self::Input) -> Self::Output;
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
impl<T, const N: usize> Swap for [T; N] {
  type Input = [usize; 2];
  type Output = Result<(), ()>;

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Self::Output {
    vec_swap!(self, a, b)
  }
}

/// ```rust
/// let mut structure = &mut [1, 2, 3][..];
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
impl<T> Swap for &'_ mut [T] {
  type Input = [usize; 2];
  type Output = Result<(), ()>;

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Self::Output {
    vec_swap!(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "alloc")]
impl<T> Swap for Vec<T> {
  type Input = [usize; 2];
  type Output = Result<(), ()>;

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Self::Output {
    vec_swap!(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Swap for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  type Input = [usize; 2];
  type Output = Result<(), ()>;

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Self::Output {
    vec_swap!(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Swap for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = [usize; 2];
  type Output = Result<(), ()>;

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Self::Output {
    vec_swap!(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Swap for staticvec::StaticVec<T, N> {
  type Input = [usize; 2];
  type Output = Result<(), ()>;

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Self::Output {
    vec_swap!(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Swap for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = [usize; 2];
  type Output = Result<(), ()>;

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Self::Output {
    vec_swap!(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Swap for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = [usize; 2];
  type Output = Result<(), ()>;

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Self::Output {
    vec_swap!(self, a, b)
  }
}
