/// Has some sort of storage that can be cleared.
pub trait Clear {
  /// Outputurn type for the [`clear`](Clear::clear)` method.
  type Output;

  /// Clears the storage.
  fn clear(&mut self) -> Self::Output;
}

#[cfg(feature = "alloc")]
impl<T> Clear for alloc::vec::Vec<T> {
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear();
  }
}

#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Clear for arrayvec::ArrayVec<crate::ArrayWrapper<T, N>> {
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear()
  }
}

#[cfg(feature = "smallvec")]
impl<T, const N: usize> Clear for smallvec::SmallVec<crate::ArrayWrapper<T, N>> {
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear()
  }
}

#[cfg(feature = "staticvec")]
impl<T, const N: usize> Clear for staticvec::StaticVec<T, N> {
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.clear()
  }
}
