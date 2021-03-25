#[cfg(feature = "alloc")]
use alloc::{
  collections::{BTreeMap, BTreeSet},
  vec::Vec,
};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[allow(
  // For convenience because of selected features
  unused
)]
// Manages vectors that don't perform bound checks
macro_rules! manage_vec {
  ($v:expr, $idx:expr, $elem:expr) => {{
    if $idx > $v.len() {
      return Err($elem);
    }
    Ok($v.insert($idx, $elem))
  }};
}

/// See [`insert`](Insert::insert) for more information.
pub trait Insert {
  /// Error
  type Error;
  /// Input
  type Input;
  /// Result
  type Ok;

  /// Inserts an `Input` element.
  fn insert(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error>;
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::b_tree_map();
/// cl_traits::Insert::insert(&mut structure, (10, 100));
/// assert_eq!(structure.iter().find(|(k, v)| **k == 10), Some((&10, &100)));
/// ```
#[cfg(feature = "alloc")]
impl<K, V> Insert for BTreeMap<K, V>
where
  K: Ord,
{
  type Error = core::convert::Infallible;
  type Input = (K, V);
  type Ok = Option<V>;

  #[inline]
  fn insert(&mut self, (k, v): Self::Input) -> Result<Self::Ok, Self::Error> {
    Ok(self.insert(k, v))
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::b_tree_set();
/// cl_traits::Insert::insert(&mut structure, 10);
/// assert_eq!(structure.iter().find(|&&e| e == 10), Some(&10));
/// ```
#[cfg(feature = "alloc")]
impl<V> Insert for BTreeSet<V>
where
  V: Ord,
{
  type Error = V;
  type Input = V;
  type Ok = ();

  #[inline]
  fn insert(&mut self, v: Self::Input) -> Result<Self::Ok, Self::Error> {
    if self.contains(&v) {
      Err(v)
    } else {
      let _ = self.insert(v);
      Ok(())
    }
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::hash_map();
/// cl_traits::Insert::insert(&mut structure, (10, 100));
/// assert_eq!(structure.iter().find(|(k, v)| **k == 10), Some((&10, &100)));
/// ```
#[cfg(feature = "std")]
impl<K, V> Insert for HashMap<K, V>
where
  K: Eq + core::hash::Hash,
{
  type Error = core::convert::Infallible;
  type Input = (K, V);
  type Ok = Option<V>;

  #[inline]
  fn insert(&mut self, (k, v): Self::Input) -> Result<Self::Ok, Self::Error> {
    Ok(self.insert(k, v))
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::hash_set();
/// cl_traits::Insert::insert(&mut structure, 10);
/// assert_eq!(structure.iter().find(|&&e| e == 10), Some(&10));
/// ```
#[cfg(feature = "std")]
impl<V> Insert for HashSet<V>
where
  V: core::hash::Hash + Eq,
{
  type Error = V;
  type Input = V;
  type Ok = ();

  #[inline]
  fn insert(&mut self, v: Self::Input) -> Result<Self::Ok, Self::Error> {
    if self.contains(&v) {
      Err(v)
    } else {
      let _ = self.insert(v);
      Ok(())
    }
  }
}

/// ```rust
/// let mut opt = None;
/// cl_traits::Insert::insert(&mut opt, 3);
/// assert_eq!(opt, Some(3));
/// ```
impl<T> Insert for Option<T> {
  type Error = T;
  type Input = T;
  type Ok = ();

  #[inline]
  fn insert(&mut self, input: Self::Input) -> Result<Self::Ok, Self::Error> {
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
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "alloc")]
impl<T> Insert for Vec<T> {
  type Error = T;
  type Input = (usize, T);
  type Ok = ();

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx, elem)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::array_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "with-arrayvec")]
impl<T, const N: usize> Insert for arrayvec::ArrayVec<T, N> {
  type Error = T;
  type Input = (usize, T);
  type Ok = ();

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx, elem)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::small_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "with-smallvec")]
impl<A> Insert for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Error = A::Item;
  type Input = (usize, A::Item);
  type Ok = ();

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx, elem)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::static_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "with-staticvec")]
impl<T, const N: usize> Insert for staticvec::StaticVec<T, N> {
  type Error = T;
  type Input = (usize, T);
  type Ok = ();

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx, elem)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec_array_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Insert for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = A::Item;
  type Input = (usize, A::Item);
  type Ok = ();

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx, elem)
  }
}

/// ```rust
/// let mut structure = cl_traits::doc_tests::tiny_vec();
/// cl_traits::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "with-tinyvec")]
impl<A> Insert for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = A::Item;
  type Input = (usize, A::Item);
  type Ok = ();

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<Self::Ok, Self::Error> {
    manage_vec!(self, idx, elem)
  }
}
