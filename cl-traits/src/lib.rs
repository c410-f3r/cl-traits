//! Collection Traits (cl-traits)
//!
//! Many data structures have unique features that make it difficult or even impossible to create
//! a single `trait` that fits all. This crate tries to circumvent such behaviour by providing
//! a single method for each `trait` to achieve maximum flexibility and freedom.

#![cfg_attr(feature = "const-generics", allow(incomplete_features))]
#![cfg_attr(feature = "const-generics", feature(const_generics))]
#![forbid(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod array;
mod array_wrapper;
mod capacity;
mod clear;
pub mod doc_tests;
mod insert;
mod length;
mod macros;
mod push;
mod remove;
mod retain;
mod storage;
mod swap;
mod truncate;
mod utils;
mod with_capacity;

pub use array::*;
pub use array_wrapper::*;
pub use capacity::*;
#[cfg(feature = "with-derive")]
pub use cl_traits_derive::*;
pub use clear::*;
pub use insert::*;
pub use length::*;
pub use push::*;
pub use remove::*;
pub use retain::*;
pub use storage::*;
pub use swap::*;
pub use truncate::*;
pub use utils::*;
pub use with_capacity::*;
