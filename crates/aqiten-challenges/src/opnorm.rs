//! A sound, integer-exact bound on the multiplication operator norm.
//!
//! `||c||_mul,2` is the spectral norm of the `D x D` integer matrix `M_c` of
//! multiplication by `c` (centered lift). `G = M_c^T M_c` is symmetric, and its
//! largest eigenvalue is `||c||_mul,2^2`. Row `a` of `G` is, up to signs, a
//! cyclic shift of row `0`, so every row has the same absolute sum
//! `B = sum_b |G_{0,b}|`, and `G_{0,b} = <c, X^b c>` (inner product of the
//! centered integer coefficient vectors). By Gershgorin,
//! `||c||_mul,2^2 <= B`. Everything is exact integer arithmetic.
//!
//! The bound is **sound and loose**: for `c = a` (a constant) it is exact
//! (`B = a^2`); in general `||c||_2^2 <= B <= ||c||_1^2`. The tight predicate of
//! Akita Theorem D.1 is OPEN here.

use aqiten_error::{Error, Result};
use aqiten_ring::RingElem;

/// `B >= ||c||_mul,2^2`, computed exactly.
///
/// # Errors
/// [`Error::Overflow`] if an intermediate value exceeds `i128`/`u128` (impossible
/// for sparse small-norm challenges).
pub fn op_norm_sq_upper_bound<const P: u64, const D: usize>(c: &RingElem<P, D>) -> Result<u128> {
    let base = c.to_centered();
    let mut total: u128 = 0;
    for b in 0..D {
        let shifted = c.mul_monomial(b).to_centered();
        let mut inner: i128 = 0;
        for (x, y) in base.iter().zip(shifted.iter()) {
            let prod = i128::from(*x)
                .checked_mul(i128::from(*y))
                .ok_or(Error::Overflow {
                    what: "op_norm_sq_upper_bound: product",
                })?;
            inner = inner.checked_add(prod).ok_or(Error::Overflow {
                what: "op_norm_sq_upper_bound: inner product",
            })?;
        }
        total = total
            .checked_add(inner.unsigned_abs())
            .ok_or(Error::Overflow {
                what: "op_norm_sq_upper_bound: row sum",
            })?;
    }
    Ok(total)
}

/// Whether the certified bound proves `||c||_mul,2^2 <= gamma_sq`
/// (acceptance implies the true bound; rejection proves nothing).
///
/// # Errors
/// As [`op_norm_sq_upper_bound`].
pub fn op_norm_sq_certified_at_most<const P: u64, const D: usize>(
    c: &RingElem<P, D>,
    gamma_sq: u128,
) -> Result<bool> {
    Ok(op_norm_sq_upper_bound(c)? <= gamma_sq)
}
