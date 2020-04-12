/// See [`retain`](Retain::retain) for more information.
pub trait Retain {
  /// Function
  type Fn;
  /// Output
  type Output;

  /// Retains only the elements specified by the `F` predicate.
  fn retain(&mut self, input: Self::Fn) -> Self::Output;
}

/// ```rust
/// let mut opt = Some(1);
/// cl_traits::Retain::retain(&mut opt, |n| n % 2 == 0);
/// assert_eq!(opt, None);
/// ```
impl<T> Retain for Option<T> {
  type Fn = fn(&T) -> bool;
  type Output = ();

  fn retain(&mut self, input: Self::Fn) {
    if let Some(elem) = self {
      if !input(elem) {
        *self = None;
      }
    }
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure, &[2]);
/// ```
#[cfg(feature = "alloc")]
impl<T> Retain for alloc::vec::Vec<T> {
  type Fn = fn(&T) -> bool;
  type Output = ();

  fn retain(&mut self, input: Self::Fn) {
    self.retain(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "with-arrayvec")]
impl<A> Retain for arrayvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Fn = fn(&A::Item) -> bool;
  type Output = ();

  fn retain(&mut self, input: Self::Fn) {
    self.retain(|i| input(i))
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Retain for smallvec::SmallVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
{
  type Fn = fn(&A::Item) -> bool;
  type Output = ();

  fn retain(&mut self, input: Self::Fn) {
    self.retain(|i| input(i))
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Retain for staticvec::StaticVec<T, N> {
  type Fn = fn(&T) -> bool;
  type Output = ();

  fn retain(&mut self, input: Self::Fn) {
    self.retain(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Retain for tinyvec::ArrayVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Fn = fn(&A::Item) -> bool;
  type Output = ();

  fn retain(&mut self, input: Self::Fn) {
    self.retain(input)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(all(feature = "alloc", feature = "with-tinyvec"))]
impl<A> Retain for tinyvec::TinyVec<crate::ArrayWrapper<A>>
where
  A: crate::Array,
  A::Item: Default,
{
  type Fn = fn(&A::Item) -> bool;
  type Output = ();

  fn retain(&mut self, input: Self::Fn) {
    self.retain(input)
  }
}
