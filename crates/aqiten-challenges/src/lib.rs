//! Folding challenges for aQiten: sparse small-norm elements of `R_q = Z_q[X]/(X^D+1)`.
//!
//! # The algebra this crate rests on (proved in the documentation of [`Split`])
//!
//! For a prime `q = 5 (mod 8)` and `D` a power of two, `-1` is a square but
//! `i = sqrt(-1)` is **not** a square. Then `X^D + 1 = (X^m - i)(X^m + i)` with
//! `m = D/2`, both factors are irreducible, and
//! `R_q = F_{q^m} x F_{q^m}` (two fields). Consequences used here:
//!
//! 1. **Exact unit test.** `c` is invertible iff it is non-zero modulo both
//!    factors, i.e. iff NOT (`c_j = +i c_{j+m}` for all `j`) and NOT
//!    (`c_j = -i c_{j+m}` for all `j`) ([`Split::is_unit`]).
//! 2. **Certified invertible differences.** If `||delta||_inf < sqrt(q/2)` and
//!    `delta != 0` then `delta` is a unit (two-line proof in [`Split`]). For a
//!    sparse family with maximum magnitude `c_max`, every challenge difference
//!    has `||.||_inf <= 2 c_max`, so `8 c_max^2 < q` certifies that every
//!    non-zero difference is invertible ([`SparseFamily::differences_are_units`]).
//!    This is the case `s_split = 2` of the Lyubashevsky-Seiler bound that Akita
//!    cites, proved here from scratch for this modulus class.
//!
//! # What is here
//!
//! * [`SparseFamily`]: the family `C(D; (a, n_a))` of Akita Section 3.2: `s = sum n_a`
//!   support positions, `n_a` of them with magnitude `a`, independent signs.
//!   `||c||_inf = c_max`, `||c||_1 = omega = sum a n_a` for every member. Its
//!   exact cardinality is computed with big-integer arithmetic (no floating point).
//! * [`sample_sparse`] / [`challenge_sparse`]: the uniform sampler (partial
//!   Fisher-Yates with bounded rejection sampling, then sign bits) driven by a
//!   SHAKE256 stream from `aqiten-transcript`.
//! * [`op_norm_sq_upper_bound`]: a **sound but loose**, integer-exact upper bound
//!   on the squared multiplication operator norm.
//!
//! # What is NOT here (OPEN)
//!
//! The *tight* certified operator-norm predicate of Akita Theorem D.1, and the
//! certified lower bound on the cardinality of the *accepted* subfamily, are not
//! implemented. They need the fixed-point certificate of that theorem and a
//! parameter choice (work item P1); the loose bound below is safe to use
//! (acceptance by it implies the true norm bound) but admits fewer challenges.
//!
//! # Cross-crate role
//!
//! Consumes `aqiten-ring` (elements, exact centered lift), `aqiten-field` (`Fp`,
//! square roots of -1), `aqiten-transcript` (challenge streams, uniform
//! sampling), `aqiten-error`. Consumed by `aqiten-params` (inequality I6),
//! `aqiten-schedule` (challenge-set size per fold), `aqiten-fold`,
//! `aqiten-sumcheck` and `aqiten-verifier`.

#![no_std]

extern crate alloc;

mod big;
mod family;
mod opnorm;
mod sampler;
mod split;

pub use family::SparseFamily;
pub use opnorm::{op_norm_sq_certified_at_most, op_norm_sq_upper_bound};
pub use sampler::{challenge_sparse, sample_sparse};
pub use split::Split;
