//! The ring `R_q = Z_q[X]/(X^D + 1)` for aQiten.
//!
//! [`RingElem<P, D>`] is a polynomial of degree `< D` with coefficients in
//! [`aqiten_field::Fp<P>`], multiplied modulo `X^D + 1` (negacyclic convolution).
//!
//! # Design decisions
//!
//! * **Exact reference multiplication only.** Multiplication is the textbook
//!   `D^2` schoolbook product, built as `sum_i a_i * (b * X^i)` with a negacyclic
//!   shift. It is correct for *every* prime `q` and every power-of-two `D`.
//!   No NTT is provided: the architecture review (R1) showed that a full
//!   negacyclic NTT needs `2D | q - 1`, which is impossible for `q = 5 mod 8`
//!   and `D >= 4`. An NTT backend may be added later **only** for a modulus and
//!   degree pair that carries a verified root certificate, and it must be
//!   differentially tested against this reference.
//! * **Exact integer norms.** Norms are computed on centered integer
//!   representatives in `[-(q-1)/2, (q-1)/2]` and never reduced, so a bound
//!   check cannot wrap around modulo `q`. Overflow is an error, not a wrap.
//! * **Balanced power-of-two digits.** Decomposition uses base `2^k` (`2 <= k <= 32`), digits in
//!   `[-2^(k-1), 2^(k-1) - 1]`, by shifts and masks only (no division), with a
//!   fixed number of iterations.
//! * **Degree is checked at compile time**: `D` must be a power of two with
//!   `2 <= D <= 2^16`; anything else fails to compile.
//!
//! # What is and is not constant time
//!
//! Addition, subtraction, negation, multiplication, monomial multiplication and
//! digit extraction have data-independent control flow (they branch only on
//! `D`, `k` and `len`, all public). The norm predicates ([`RingElem::linf_norm`],
//! [`RingElem::within_linf`], [`RingElem::l2_norm_squared`]) return values that
//! the protocol publishes (accept/reject) but are **not** written to be
//! constant time; the masked-fold rejection step that must hide the exact norm
//! will need its own branch-free comparison (tag: OPEN, belongs to `aqiten-fold`).
//! Hardware constant time of the arithmetic is a CONJECTURE, as in `aqiten-field`.
//!
//! # Cross-crate role
//!
//! * consumes [`aqiten_field`] for coefficients and the centered lift,
//!   [`aqiten_codec`] for the wire format, [`aqiten_error`] for failures;
//! * is consumed by `aqiten-challenges` (operator norms, invertibility),
//!   `aqiten-commit-*` (Ajtai/BDLOP matrix-vector products), `aqiten-fold`
//!   (masked responses, exact norm bounds) and `aqiten-estimator`'s parameter
//!   validation (digit counts for the no-wrap-around inequality I3).

#![no_std]

extern crate alloc;

mod digits;
mod element;
mod norm;
mod wire;

pub use digits::{decompose_i64, digits_needed, recompose_i64};
pub use element::RingElem;
