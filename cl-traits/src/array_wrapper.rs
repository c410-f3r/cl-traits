use crate::create_array;
use core::{
  borrow::{Borrow, BorrowMut},
  cmp::Ordering,
  fmt,
  iter::IntoIterator,
  ops::{Deref, DerefMut, Index, IndexMut},
  slice::{Iter, IterMut, SliceIndex},
};
#[cfg(feature = "with_serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "with_arrayvec")]
/// A shortcut for `ArrayVec<ArrayWrapper<T, N>>`
pub type ArrayVecArrayWrapper<T, const N: usize> = arrayvec::ArrayVec<ArrayWrapper<T, N>>;
#[cfg(feature = "with_smallvec")]
/// A shortcut for `SmallVec<ArrayWrapper<T, N>>`
pub type SmallVecArrayWrapper<T, const N: usize> = smallvec::SmallVec<ArrayWrapper<T, N>>;

/// Arbitrary length array wrapper. This structure is necessary for third-party
/// and std implementations.
pub struct ArrayWrapper<T, const N: usize> {
  array: [T; N],
}

impl<T, const N: usize> ArrayWrapper<T, N> {
  /// Wraps `array` into this structure.
  pub fn new(array: [T; N]) -> Self {
    Self { array }
  }
}

#[cfg(feature = "with_arrayvec")]
unsafe impl<T, const N: usize> arrayvec::Array for ArrayWrapper<T, N> {
  type Index = usize;
  type Item = T;

  const CAPACITY: usize = N;

  fn as_slice(&self) -> &[Self::Item] {
    &self.array
  }

  fn as_mut_slice(&mut self) -> &mut [Self::Item] {
    &mut self.array
  }
}

#[cfg(feature = "with_smallvec")]
unsafe impl<T, const N: usize> smallvec::Array for ArrayWrapper<T, N> {
  type Item = T;

  fn size() -> usize {
    N
  }
}

#[cfg(feature = "with_serde")]
impl<'de, T, const N: usize> Deserialize<'de> for ArrayWrapper<T, N>
where
  T: Deserialize<'de> + 'de,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    use core::marker::PhantomData;
    use serde::de::{Error, SeqAccess, Visitor};

    struct ArrayWrapperVisitor<'de, T, const N: usize>(PhantomData<&'de [T; N]>);

    impl<'de, T, const N: usize> Visitor<'de> for ArrayWrapperVisitor<'de, T, N>
    where
      T: Deserialize<'de>,
    {
      type Value = ArrayWrapper<T, N>;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "an array with no more than {} items", N)
      }

      fn visit_seq<SA>(self, mut seq: SA) -> Result<Self::Value, SA::Error>
      where
        SA: SeqAccess<'de>,
      {
        let array = unsafe {
          let mut array: core::mem::MaybeUninit<[T; N]> = core::mem::MaybeUninit::uninit();
          for (idx, value_ptr) in (&mut *array.as_mut_ptr()).iter_mut().enumerate() {
            if let Some(value) = seq.next_element()? {
              core::ptr::write(value_ptr, value);
            } else {
              return Err(SA::Error::invalid_length(idx, &self));
            }
          }
          array.assume_init()
        };
        Ok(ArrayWrapper { array })
      }
    }

    deserializer.deserialize_seq(ArrayWrapperVisitor::<T, N>(PhantomData))
  }
}

#[cfg(feature = "with_serde")]
impl<T, const N: usize> Serialize for ArrayWrapper<T, N>
where
  T: Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_seq(self)
  }
}

impl<T, const N: usize> AsRef<[T]> for ArrayWrapper<T, N> {
  #[inline]
  fn as_ref(&self) -> &[T] {
    &self.array
  }
}

impl<T, const N: usize> AsRef<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn as_ref(&self) -> &[T; N] {
    &self.array
  }
}

impl<T, const N: usize> AsMut<[T]> for ArrayWrapper<T, N> {
  fn as_mut(&mut self) -> &mut [T] {
    &mut self.array
  }
}

impl<T, const N: usize> AsMut<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn as_mut(&mut self) -> &mut [T; N] {
    &mut self.array
  }
}

impl<T, const N: usize> Borrow<[T]> for ArrayWrapper<T, N> {
  #[inline]
  fn borrow(&self) -> &[T] {
    &self.array
  }
}

impl<T, const N: usize> Borrow<[T]> for &'_ ArrayWrapper<T, N> {
  #[inline]
  fn borrow(&self) -> &[T] {
    &self.array
  }
}

impl<T, const N: usize> Borrow<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn borrow(&self) -> &[T; N] {
    &self.array
  }
}

impl<T, const N: usize> BorrowMut<[T]> for ArrayWrapper<T, N> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut [T] {
    &mut self.array
  }
}

impl<T, const N: usize> BorrowMut<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut [T; N] {
    &mut self.array
  }
}

impl<T, const N: usize> Clone for ArrayWrapper<T, N>
where
  T: Clone,
{
  #[inline]
  fn clone(&self) -> Self {
    Self { array: self.array.clone() }
  }
}

impl<T, const N: usize> Copy for ArrayWrapper<T, N> where T: Copy {}

impl<T, const N: usize> Default for ArrayWrapper<T, N>
where
  T: Default,
{
  #[inline]
  fn default() -> Self {
    ArrayWrapper { array: create_array(|_| T::default()) }
  }
}

impl<T, const N: usize> Deref for ArrayWrapper<T, N> {
  type Target = [T; N];
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.array
  }
}

impl<T, const N: usize> DerefMut for ArrayWrapper<T, N> {
  #[inline]
  fn deref_mut(&mut self) -> &mut [T; N] {
    &mut self.array
  }
}

impl<T, const N: usize> Eq for ArrayWrapper<T, N> where T: Eq {}

impl<T, const N: usize> From<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn from(from: [T; N]) -> Self {
    Self { array: from }
  }
}

impl<T, const N: usize> From<ArrayWrapper<T, N>> for [T; N] {
  #[inline]
  fn from(from: ArrayWrapper<T, N>) -> Self {
    from.array
  }
}

impl<I, T, const N: usize> Index<I> for ArrayWrapper<T, N>
where
  I: SliceIndex<[T]>,
{
  type Output = <I as SliceIndex<[T]>>::Output;

  #[inline]
  fn index(&self, idx: I) -> &Self::Output {
    &self.array[idx]
  }
}

impl<I, T, const N: usize> IndexMut<I> for ArrayWrapper<T, N>
where
  I: SliceIndex<[T]>,
{
  #[inline]
  fn index_mut(&mut self, idx: I) -> &mut Self::Output {
    &mut self.array[idx]
  }
}

impl<'a, T, const N: usize> IntoIterator for &'a ArrayWrapper<T, N> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.array.iter()
  }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut ArrayWrapper<T, N> {
  type Item = &'a mut T;
  type IntoIter = IterMut<'a, T>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.array.iter_mut()
  }
}

impl<T, const N: usize> Ord for ArrayWrapper<T, N>
where
  T: Ord,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.array[..].cmp(&other.array[..])
  }
}

impl<T, const N: usize> PartialEq for ArrayWrapper<T, N>
where
  T: PartialEq,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.array[..] == other.array[..]
  }
}

impl<T, const N: usize> PartialOrd for ArrayWrapper<T, N>
where
  T: PartialOrd,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.array[..].partial_cmp(&other.array[..])
  }
}

impl<T, const N: usize> fmt::Debug for ArrayWrapper<T, N>
where
  T: fmt::Debug,
{
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_list().entries(&self.array[..]).finish()
  }
}
