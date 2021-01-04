use core::{
  mem::{self, MaybeUninit},
  ptr,
};

struct ArrayGuard<T, const N: usize> {
  dst: *mut T,
  initialized: usize,
}

impl<T, const N: usize> Drop for ArrayGuard<T, N> {
  #[inline]
  fn drop(&mut self) {
    let initialized_part = ptr::slice_from_raw_parts_mut(self.dst, self.initialized);
    #[allow(
      // Waiting for std method to remove `ArrayGuard`
      unsafe_code
    )]
    unsafe {
      ptr::drop_in_place(initialized_part);
    }
  }
}

/// Creates an array `[T; N]` where each array element `T` is returned by the `cb` call.
///
/// * Example
///
/// ```rust
/// use cl_traits::create_array;
/// let array: [usize; 4] = create_array(|idx| idx);
/// assert_eq!(array, [0, 1, 2, 3]);
/// ```
#[allow(
  // First array pointer
  clippy::as_conversions,
  // Should only drop the array when `cb` panics
  clippy::mem_forget
)]
#[inline]
pub fn create_array<F, T, const N: usize>(mut cb: F) -> [T; N]
where
  F: FnMut(usize) -> T,
{
  let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
  let mut guard: ArrayGuard<T, N> = ArrayGuard { dst: array.as_mut_ptr() as _, initialized: 0 };
  #[allow(
    // Waiting for std method to remove `ArrayGuard`
    unsafe_code
  )]
  unsafe {
    for (idx, value_ptr) in (&mut *array.as_mut_ptr()).iter_mut().enumerate() {
      ptr::write(value_ptr, cb(idx));
      guard.initialized = guard.initialized.saturating_add(1);
    }
    mem::forget(guard);
    array.assume_init()
  }
}

/// Creates an array `[T; N]` where each array element is `T::default()`
///
/// * Example
///
/// ```rust
/// use cl_traits::default_array;
/// let array: [usize; 4] = default_array();
/// assert_eq!(array, [0, 0, 0, 0]);
/// ```
#[inline]
pub fn default_array<T, const N: usize>() -> [T; N]
where
  T: Default,
{
  create_array(|_| T::default())
}

/// Creates a fallible array `[T; N]` where each array element `T` is returned by the `cb` call.
///
/// * Example
///
/// ```rust
/// use cl_traits::try_create_array;
///
/// #[derive(Debug, PartialEq)]
/// enum SomeError {
///   Foo
/// }
///
/// let array: Result<[usize; 5], SomeError> = try_create_array(|i| Ok(i));
/// assert_eq!(array, Ok([0, 1, 2, 3, 4]));
///
/// let another_array: Result<[usize; 5], SomeError> = try_create_array(|_| Err(SomeError::Foo));
/// assert_eq!(another_array, Err(SomeError::Foo));
/// ```
#[allow(
  // First array pointer
  clippy::as_conversions,
  // Should only drop the array when `cb` panics
  clippy::mem_forget
)]
#[inline]
pub fn try_create_array<E, F, T, const N: usize>(mut cb: F) -> Result<[T; N], E>
where
  F: FnMut(usize) -> Result<T, E>,
{
  let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
  let mut guard: ArrayGuard<T, N> = ArrayGuard { dst: array.as_mut_ptr() as _, initialized: 0 };
  #[allow(
    // Waiting for std method to remove `ArrayGuard`
    unsafe_code
  )]
  unsafe {
    for (idx, value_ptr) in (&mut *array.as_mut_ptr()).iter_mut().enumerate() {
      core::ptr::write(value_ptr, cb(idx)?);
      guard.initialized = guard.initialized.saturating_add(1);
    }
    mem::forget(guard);
    Ok(array.assume_init())
  }
}
