//! Collection Traits (cl-traits)

#![allow(incomplete_features)]
#![feature(const_generics)]
#![forbid(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![no_std]

mod array_wrapper;
mod capacity;
mod clear;
mod length;
mod macros;
mod push;
mod swap;
mod truncate;
mod utils;

#[cfg(feature = "alloc")]
extern crate alloc;

pub use array_wrapper::*;
pub use capacity::*;
#[cfg(feature = "derive")]
pub use cl_traits_derive::*;
pub use clear::*;
pub use length::*;
pub use push::*;
pub use swap::*;
pub use truncate::*;
pub use utils::*;
