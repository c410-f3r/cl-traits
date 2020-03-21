//! Collection Traits (cl-traits)

#![cfg_attr(feature = "const_generics", allow(incomplete_features))]
#![cfg_attr(feature = "const_generics", feature(const_generics))]
#![forbid(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod array;
mod array_wrapper;
mod capacity;
mod clear;
mod insert;
mod length;
mod macros;
mod push;
mod storage;
mod swap;
mod truncate;
mod utils;
mod with_capacity;

pub use array::*;
pub use array_wrapper::*;
pub use capacity::*;
#[cfg(feature = "with_derive")]
pub use cl_traits_derive::*;
pub use clear::*;
pub use insert::*;
pub use length::*;
pub use push::*;
pub use storage::*;
pub use swap::*;
pub use truncate::*;
pub use utils::*;
pub use with_capacity::*;
