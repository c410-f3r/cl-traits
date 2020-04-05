/// This is my array trait. There are many like it, but this one is mine.
///
/// With `const-generics` feature, generalizes arbitrary length arrays. Otherwise, generalizes
/// arrays with up to 32 elements.
///
/// This trait will be removed once `const-generics` is stabilized.
pub trait Array {
  /// How many elements the array holds
  const CAPACITY: usize;

  /// The item being stored
  type Item;

  /// Immutable slice reference of the array.
  fn slice(&self) -> &[Self::Item];

  /// Mutable slice reference of the array.
  fn slice_mut(&mut self) -> &mut [Self::Item];
}

#[cfg(feature = "const-generics")]
impl<T, const N: usize> Array for [T; N] {
  const CAPACITY: usize = N;
  type Item = T;

  fn slice(&self) -> &[Self::Item] {
    &self[..]
  }

  fn slice_mut(&mut self) -> &mut [Self::Item] {
    &mut self[..]
  }
}

#[cfg(not(feature = "const-generics"))]
macro_rules! array_impls {
  ($($N:expr),+) => {
    $(
      impl<T> Array for [T; $N] {
        const CAPACITY: usize = $N;
        type Item = T;

        fn slice(&self) -> &[Self::Item] {
          &self[..]
        }
        fn slice_mut(&mut self) -> &mut [Self::Item] {
          &mut self[..]
        }
      }
    )+
  }
}

#[cfg(not(feature = "const-generics"))]
array_impls!(
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
  27, 28, 29, 30, 31, 32
);
