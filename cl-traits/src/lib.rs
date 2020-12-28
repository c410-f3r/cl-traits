//! Collection Traits (cl-traits)
//!
//! Many data structures have unique features that make it difficult or even impossible to create
//! a single `trait` that fits all. This crate tries to circumvent such behaviour by providing
//! a single method for each `trait` to achieve maximum flexibility and freedom.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod capacity;
mod capacity_upper_bound;
mod clear;
pub mod doc_tests;
mod insert;
mod length;
mod push;
mod remove;
mod retain;
mod storage;
mod swap;
mod truncate;
mod utils;
mod with_capacity;

pub use capacity::*;
pub use capacity_upper_bound::*;
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
