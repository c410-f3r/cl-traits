#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [`push`](Push::push) for more information.
pub trait Push {
  /// Error
  type Error;
  /// Input
  type Input;
  /// Output
  type Ok;

  /// Pushes an element, increasing the storage length.
  fn push(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error>;
}

/// ```rust
/// let mut opt = None;
/// cl_traits::Push::push(&mut opt, 3);
/// assert_eq!(opt, Some(3));
/// ```
impl<T> Push for Option<T> {
  type Error = T;
  type Input = T;
  type Ok = ();

  #[inline]
  fn push(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error> {
    if self.is_some() {
      Err(input)
    } else {
      *self = Some(input);
      Ok(())
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
  type Error = core::convert::Infallible;
  type Input = T;
  type Ok = ();

  #[inline]
  fn push(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error> {
    self.push(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-arrayvec")]
impl<T, const N: usize> Push for arrayvec::ArrayVec<T, N> {
  type Error = T;
  type Input = T;
  type Ok = ();

  #[inline]
  fn push(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error> {
    self.try_push(input).map_err(|e| e.element())
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
  type Error = A::Item;
  type Input = A::Item;
  type Ok = ();

  #[inline]
  fn push(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error> {
    self.push(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Push for staticvec::StaticVec<T, N> {
  type Error = T;
  type Input = T;
  type Ok = ();

  #[inline]
  fn push(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error> {
    self.try_push(input).map_err(|e| e.into_value())
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
  type Error = A::Item;
  type Input = A::Item;
  type Ok = ();

  #[inline]
  fn push(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error> {
    match self.try_push(input) {
      None => Ok(()),
      Some(rslt) => Err(rslt),
    }
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Push for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = A::Item;
  type Input = A::Item;
  type Ok = ();

  #[inline]
  fn push(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error> {
    self.push(input);
    Ok(())
  }
}
