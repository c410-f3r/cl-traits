use crate::{create_array, Array};
use core::{
  borrow::{Borrow, BorrowMut},
  cmp::Ordering,
  fmt,
  iter::IntoIterator,
  ops::{Deref, DerefMut, Index, IndexMut},
  slice::{Iter, IterMut, SliceIndex},
};
#[cfg(feature = "with-serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// With `const-generics` feature, wraps an arbitrary length array. Otherwise, wraps an
/// array with a selected number of elements. Necessary for third-party and std implementations.
///
/// This structure will be removed once `const-generics` is stabilized.
pub struct ArrayWrapper<A> {
  pub(crate) array: A,
}

impl<A> ArrayWrapper<A>
where
  A: Array,
{
  /// Wraps `array` into this structure.
  pub fn new(array: A) -> Self {
    Self { array }
  }
}

#[cfg(feature = "with-arrayvec")]
unsafe impl<A> arrayvec::Array for ArrayWrapper<A>
where
  A: Array,
{
  type Index = usize;
  type Item = A::Item;

  const CAPACITY: usize = A::CAPACITY;

  #[inline]
  fn as_slice(&self) -> &[Self::Item] {
    self.array.slice()
  }

  #[inline]
  fn as_mut_slice(&mut self) -> &mut [Self::Item] {
    self.array.slice_mut()
  }
}

#[cfg(feature = "with-smallvec")]
unsafe impl<A> smallvec::Array for ArrayWrapper<A>
where
  A: Array,
{
  type Item = A::Item;

  #[inline]
  fn size() -> usize {
    A::CAPACITY
  }
}

#[cfg(feature = "with-serde")]
impl<'de, A, T> Deserialize<'de> for ArrayWrapper<A>
where
  A: Array<Item = T>,
  T: Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    use core::marker::PhantomData;
    use serde::de::{Error, SeqAccess, Visitor};

    struct ArrayWrapperVisitor<'de, A, T>(PhantomData<(&'de (), A, T)>);

    impl<'de, A, T> Visitor<'de> for ArrayWrapperVisitor<'de, A, T>
    where
      A: Array<Item = T>,
      T: Deserialize<'de>,
    {
      type Value = ArrayWrapper<A>;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "an array with no more than {} items", A::CAPACITY)
      }

      fn visit_seq<SA>(self, mut seq: SA) -> Result<Self::Value, SA::Error>
      where
        SA: SeqAccess<'de>,
      {
        let array = crate::create_array_rslt(|idx| {
          if let Some(value) = seq.next_element()? {
            Ok(value)
          } else {
            Err(SA::Error::invalid_length(idx, &self))
          }
        })?;
        Ok(ArrayWrapper { array })
      }
    }

    deserializer.deserialize_seq(ArrayWrapperVisitor::<A, T>(PhantomData))
  }
}

#[cfg(feature = "with-serde")]
impl<A> Serialize for ArrayWrapper<A>
where
  A: Array,
  A::Item: Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_seq(self)
  }
}

#[cfg(feature = "with-tinyvec")]
impl<A> tinyvec::Array for ArrayWrapper<A>
where
  A: Array,
  A::Item: Default,
{
  const CAPACITY: usize = A::CAPACITY;

  type Item = A::Item;

  #[inline]
  fn as_slice(&self) -> &[Self::Item] {
    self.array.slice()
  }

  #[inline]
  fn as_slice_mut(&mut self) -> &mut [Self::Item] {
    self.array.slice_mut()
  }
}

impl<A> AsRef<A> for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn as_ref(&self) -> &A {
    &self.array
  }
}

impl<A> AsRef<[A::Item]> for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn as_ref(&self) -> &[A::Item] {
    &self.array.slice()
  }
}

impl<A> AsMut<A> for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn as_mut(&mut self) -> &mut A {
    &mut self.array
  }
}

impl<A> AsMut<[A::Item]> for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn as_mut(&mut self) -> &mut [A::Item] {
    self.array.slice_mut()
  }
}

impl<A> Borrow<A> for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn borrow(&self) -> &A {
    &self.array
  }
}

impl<A> Borrow<[A::Item]> for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn borrow(&self) -> &[A::Item] {
    self.array.slice()
  }
}

impl<A> BorrowMut<A> for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn borrow_mut(&mut self) -> &mut A {
    &mut self.array
  }
}

impl<A> BorrowMut<[A::Item]> for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn borrow_mut(&mut self) -> &mut [A::Item] {
    self.array.slice_mut()
  }
}

impl<A> Clone for ArrayWrapper<A>
where
  A: Array + Clone,
  A::Item: Clone,
{
  #[inline]
  fn clone(&self) -> Self {
    Self { array: self.array.clone() }
  }
}

impl<A> Copy for ArrayWrapper<A>
where
  A: Array + Copy,
  A::Item: Copy,
{
}

impl<A> Default for ArrayWrapper<A>
where
  A: Array,
  A::Item: Default,
{
  #[inline]
  fn default() -> Self {
    ArrayWrapper { array: create_array(|_| A::Item::default()) }
  }
}

impl<A> Deref for ArrayWrapper<A>
where
  A: Array,
{
  type Target = A;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.array
  }
}

impl<A> DerefMut for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.array
  }
}

impl<A> Eq for ArrayWrapper<A>
where
  A: Array,
  A::Item: Eq,
{
}

impl<A> From<A> for ArrayWrapper<A>
where
  A: Array,
{
  #[inline]
  fn from(from: A) -> Self {
    Self { array: from }
  }
}

impl<I, A> Index<I> for ArrayWrapper<A>
where
  A: Array,
  I: SliceIndex<[A::Item]>,
{
  type Output = <I as SliceIndex<[A::Item]>>::Output;

  #[inline]
  fn index(&self, idx: I) -> &Self::Output {
    if let Some(r) = self.array.slice().get(idx) {
      r
    } else {
      panic!("Index out of bounds");
    }
  }
}

impl<I, A> IndexMut<I> for ArrayWrapper<A>
where
  A: Array,
  I: SliceIndex<[A::Item]>,
{
  #[inline]
  fn index_mut(&mut self, idx: I) -> &mut Self::Output {
    if let Some(r) = self.array.slice_mut().get_mut(idx) {
      r
    } else {
      panic!("Index out of bounds");
    }
  }
}

impl<'a, A> IntoIterator for &'a ArrayWrapper<A>
where
  A: Array,
{
  type Item = &'a A::Item;
  type IntoIter = Iter<'a, A::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.array.slice().iter()
  }
}

impl<'a, A> IntoIterator for &'a mut ArrayWrapper<A>
where
  A: Array,
{
  type Item = &'a mut A::Item;
  type IntoIter = IterMut<'a, A::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.array.slice_mut().iter_mut()
  }
}

impl<A> Ord for ArrayWrapper<A>
where
  A: Array,
  A::Item: Ord,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.array.slice().cmp(&other.array.slice())
  }
}

impl<A> PartialEq for ArrayWrapper<A>
where
  A: Array,
  A::Item: PartialEq,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.array.slice() == other.array.slice()
  }
}

impl<A> PartialOrd for ArrayWrapper<A>
where
  A: Array,
  A::Item: PartialOrd,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.array.slice().partial_cmp(&other.array.slice())
  }
}

impl<A> fmt::Debug for ArrayWrapper<A>
where
  A: Array,
  A::Item: fmt::Debug,
{
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_list().entries(self.array.slice()).finish()
  }
}
