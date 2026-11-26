//! Shared error type and checked integer formulas for the aQiten workspace.
//!
//! Every fallible operation in aQiten returns [`Result`]. Two rules follow from
//! the charter:
//!
//! 1. Decoding and verification never panic on adversarial input; they return an
//!    [`Error`]. The workspace denies `unwrap`, `expect`, `panic!` and slice
//!    indexing in library code to enforce this.
//! 2. Every size, count and bound computation goes through the checked
//!    functions in [`arith`], so an overflow is an error and never a silent wrap.
//!
//! The error type carries a `&'static str` naming the offending object. It never
//! carries secret data, so errors can be logged and returned to untrusted callers.

#![no_std]

pub mod arith;
mod error;

pub use error::{Error, Result};
