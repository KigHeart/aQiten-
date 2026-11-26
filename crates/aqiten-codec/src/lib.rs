//! Canonical, strictly-decoded binary encoding for aQiten wire types.
//!
//! # Contract
//!
//! * **One encoding per value.** `encode` is a function; `decode` is its partial
//!   inverse. A decoder must reject every byte string that `encode` cannot
//!   produce (non-canonical integers, `bool` bytes other than 0 or 1, trailing
//!   bytes, oversized lengths). Therefore `decode(b) = Ok(v)` implies
//!   `encode(v) = b`.
//! * **Total on adversarial input.** No decoder in this crate panics, indexes out
//!   of bounds, or allocates more than the input could justify; every failure is
//!   an [`aqiten_error::Error`].
//! * **Little-endian, fixed width** for integers; **`u32` little-endian length
//!   prefix** for variable-length data, always checked against a caller-supplied
//!   maximum *before* any allocation.
//!
//! The verifier relies on this contract: it rejects non-canonical encodings
//! before any algebraic verification (architecture review R6).

#![no_std]

extern crate alloc;

mod reader;
mod traits;
mod writer;

pub use reader::Reader;
pub use traits::{decode_exact, decode_vec, encode_slice, encode_to_vec, Decode, Encode};
pub use writer::Writer;

/// Width in bytes of the length prefix used for variable-length data.
pub const LEN_PREFIX_BYTES: usize = 4;
