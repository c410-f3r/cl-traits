use crate::Array;
use core::{mem::MaybeUninit, ptr};

/// Creates an array `[T; N]` where each array element `T` is returned by the `cb` call.
#[inline]
pub fn create_array<A, F>(mut cb: F) -> A
where
  A: Array,
  F: FnMut(usize) -> A::Item,
{
  let mut array: MaybeUninit<A> = MaybeUninit::uninit();
  unsafe {
    for (idx, value_ptr) in (&mut *array.as_mut_ptr()).slice_mut().iter_mut().enumerate() {
      ptr::write(value_ptr, cb(idx));
    }
    array.assume_init()
  }
}
