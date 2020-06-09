#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

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
/// let mut opt = None;
/// cl_traits::Push::push(&mut opt, 3);
/// assert_eq!(opt, Some(3));
/// ```
impl<T> Push for Option<T> {
  type Input = T;
  type Output = ();

  fn push(&mut self, input: Self::Input) {
    if self.is_some() {
      panic!("Exceeded capacity for Option")
    } else {
      *self = Some(input);
    }
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "alloc")]
impl<T> Push for Vec<T> {
  type Input = T;
  type Output = ();

  fn push(&mut self, input: Self::Input) {
    self.push(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Push for arrayvec::ArrayVec<A>
where
  A: arrayvec::Array,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) {
    self.push(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Push for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) {
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

  fn push(&mut self, input: Self::Input) {
    self.push(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Push for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) {
    self.push(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Push for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = A::Item;
  type Output = ();

  fn push(&mut self, input: Self::Input) {
    self.push(input)
  }
}
