#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[allow(
  // For convenience because of selected features
  unused
)]
// Manages vectors that don't perform bound checks
macro_rules! manage_vec {
  ($v:expr, $idx:expr) => {{
    if $idx >= $v.len() {
      return Err(());
    }
    Ok($v.remove($idx))
  }};
}

/// See [`remove`](Remove::remove) for more information.
pub trait Remove {
  /// Error
  type Error;
  /// Input
  type Input;
  /// Ok
  type Ok;

  /// Removes an element referenced by `Input`.
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Ok, Self::Error>;
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "alloc")]
impl<T> Remove for Vec<T> {
  type Error = ();
  type Input = usize;
  type Ok = T;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "with-arrayvec")]
impl<T, const N: usize> Remove for arrayvec::ArrayVec<T, N> {
  type Error = ();
  type Input = usize;
  type Ok = T;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx)
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
  type Error = ();
  type Input = usize;
  type Ok = A::Item;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Remove for staticvec::StaticVec<T, N> {
  type Error = ();
  type Input = usize;
  type Ok = T;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx)
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
  type Error = ();
  type Input = usize;
  type Ok = A::Item;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Remove for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = ();
  type Input = usize;
  type Ok = A::Item;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx)
  }
}
