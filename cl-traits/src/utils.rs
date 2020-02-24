use core::{mem::MaybeUninit, ptr};

/// Creates an array `[T; N]` where each array element `T` is returned by the `cb` call.
#[inline]
pub fn create_array<F, T, const N: usize>(mut cb: F) -> [T; N]
where
  F: FnMut(usize) -> T,
{
  unsafe {
    let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
    for (idx, value_ptr) in (&mut *array.as_mut_ptr()).iter_mut().enumerate() {
      ptr::write(value_ptr, cb(idx));
    }
    array.assume_init()
  }
}
