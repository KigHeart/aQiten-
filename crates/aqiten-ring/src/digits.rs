//! Balanced power-of-two digit decomposition.
//!
//! Base `B = 2^k` with digits in `[-B/2, B/2 - 1]`. Every integer in
//! `[-(B/2)·S, (B/2 - 1)·S]`, `S = (B^len - 1)/(B - 1)`, has exactly one
//! representation with `len` digits.

use alloc::vec::Vec;

use aqiten_error::{arith, Error, Result};
use aqiten_field::{Field, Fp};

use crate::RingElem;

/// Largest supported `k` (digits fit comfortably in `i64` arithmetic).
const MAX_LOG_BASE: u32 = 32;
/// Smallest supported `k`: for `k = 1` the digit set is `{-1, 0}`, which cannot
/// represent any positive integer, so no symmetric bound is coverable.
const MIN_LOG_BASE: u32 = 2;

fn check_log_base(k: u32) -> Result<()> {
    if (MIN_LOG_BASE..=MAX_LOG_BASE).contains(&k) {
        Ok(())
    } else {
        Err(Error::InvalidParameter {
            what: "log_base must be in 2..=32",
        })
    }
}

/// Decomposes `x` into exactly `len` balanced base-`2^k` digits, lowest first,
/// so that `x = sum_i d_i * 2^(k*i)` with each `d_i` in `[-2^(k-1), 2^(k-1) - 1]`.
///
/// Branch-free per digit: `d = ((x + 2^(k-1)) & (2^k - 1)) - 2^(k-1)`,
/// then `x <- (x - d) >> k` (exact, since `x - d` is divisible by `2^k`).
///
/// # Errors
/// [`Error::InvalidParameter`] if `k` is outside `2..=32`;
/// [`Error::OutOfRange`] if `x` needs more than `len` digits.
#[allow(clippy::arithmetic_side_effects)] // k in 2..=32 is checked above: shifts and k-1 cannot overflow
pub fn decompose_i64(x: i64, k: u32, len: usize) -> Result<Vec<i64>> {
    check_log_base(k)?;
    let half = 1i64 << (k - 1);
    let mask = (1i64 << k) - 1;
    let mut rest = x;
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        let d = (rest.wrapping_add(half) & mask).wrapping_sub(half);
        rest = rest.wrapping_sub(d) >> k;
        out.push(d);
    }
    if rest == 0 {
        Ok(out)
    } else {
        Err(Error::OutOfRange {
            what: "decompose_i64: value needs more digits",
        })
    }
}

/// Recombines digits: `sum_i d_i * 2^(k*i)`, with overflow checked.
///
/// # Errors
/// [`Error::InvalidParameter`] if `k` is outside `2..=32`;
/// [`Error::Overflow`] if the value does not fit in `i128`.
pub fn recompose_i64(digits: &[i64], k: u32) -> Result<i128> {
    check_log_base(k)?;
    let mut acc: i128 = 0;
    for &d in digits.iter().rev() {
        acc = acc
            .checked_mul(1i128 << k)
            .and_then(|v| v.checked_add(i128::from(d)))
            .ok_or(Error::Overflow {
                what: "recompose_i64",
            })?;
    }
    Ok(acc)
}

/// The smallest `len` such that every integer `x` with `|x| <= bound` has a
/// `len`-digit balanced base-`2^k` representation. Because the digit range is
/// asymmetric, the negative side `-bound` is never the binding one.
///
/// # Errors
/// [`Error::InvalidParameter`] if `k` is outside `2..=32`;
/// [`Error::Overflow`] if more than 64 digits would be needed.
#[allow(clippy::arithmetic_side_effects)] // k in 2..=32 is checked above: shifts and k-1 cannot overflow
pub fn digits_needed(bound: u64, k: u32) -> Result<usize> {
    check_log_base(k)?;
    let half_minus_one = (1u128 << (k - 1)) - 1; // largest positive digit
    let mut max_pos: u128 = 0; // largest representable with `len` digits
    let mut weight: u128 = 1; // 2^(k*len)
    let mut len = 0usize;
    while max_pos < u128::from(bound) {
        max_pos = half_minus_one
            .checked_mul(weight)
            .and_then(|t| t.checked_add(max_pos))
            .ok_or(Error::Overflow {
                what: "digits_needed",
            })?;
        weight = weight.checked_shl(k).ok_or(Error::Overflow {
            what: "digits_needed",
        })?;
        len = arith::add_usize(len, 1, "digits_needed: digit count")?;
        if len > 64 {
            return Err(Error::Overflow {
                what: "digits_needed: more than 64 digits",
            });
        }
    }
    Ok(len)
}

impl<const P: u64, const D: usize> RingElem<P, D> {
    /// Decomposes every coefficient into `len` balanced base-`2^k` digits and
    /// returns `len` ring elements `e_0, ..., e_{len-1}` (lowest first) with
    /// `self = sum_j 2^(k*j) * e_j` and `e_j` having `linf_norm() <= 2^(k-1)`.
    ///
    /// # Errors
    /// As [`decompose_i64`].
    pub fn decompose_balanced(&self, k: u32, len: usize) -> Result<Vec<Self>> {
        check_log_base(k)?;
        let mut parts = alloc::vec![Self::ZERO; len];
        for (i, x) in self.to_centered().iter().enumerate() {
            let digits = decompose_i64(*x, k, len)?;
            for (part, d) in parts.iter_mut().zip(digits) {
                if let Some(slot) = part.c.get_mut(i) {
                    *slot = Fp::<P>::from_i64(d);
                }
            }
        }
        Ok(parts)
    }

    /// Inverse of [`Self::decompose_balanced`]: `sum_j 2^(k*j) * parts[j]`, evaluated
    /// modulo `P` (a ring identity, so there is no integer overflow to check).
    ///
    /// # Errors
    /// [`Error::InvalidParameter`] if `k` is outside `2..=32`.
    #[allow(clippy::arithmetic_side_effects)] // field multiplication is total
    pub fn recompose_balanced(parts: &[Self], k: u32) -> Result<Self> {
        check_log_base(k)?;
        let base = <Fp<P> as Field>::from_u64(1u64 << k);
        let mut acc = Self::ZERO;
        for p in parts.iter().rev() {
            acc = acc.scale(base) + *p;
        }
        Ok(acc)
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation,
        clippy::arithmetic_side_effects
    )]
    use super::*;

    #[test]
    fn digits_in_range_and_recompose_exhaustively() {
        for k in 2..=4u32 {
            let half = 1i64 << (k - 1);
            for len in 1..=4usize {
                let s: i64 = (0..len).map(|i| 1i64 << (k as usize * i)).sum();
                let (lo, hi) = (-half * s, (half - 1) * s);
                for x in (lo - 3)..=(hi + 3) {
                    let r = decompose_i64(x, k, len);
                    if (lo..=hi).contains(&x) {
                        let ds = r.unwrap_or_default();
                        assert_eq!(ds.len(), len);
                        assert!(ds.iter().all(|d| (-half..half).contains(d)), "k={k} x={x}");
                        assert_eq!(recompose_i64(&ds, k), Ok(i128::from(x)));
                    } else {
                        assert!(r.is_err(), "k={k} len={len} x={x} should not fit");
                    }
                }
            }
        }
    }

    #[test]
    fn digits_needed_is_minimal_against_brute_force() {
        for k in 2..=4u32 {
            for bound in 0u64..=300 {
                let n = digits_needed(bound, k).unwrap_or(usize::MAX);
                let fits = |len: usize| {
                    (-(bound as i64)..=bound as i64).all(|x| decompose_i64(x, k, len).is_ok())
                };
                assert!(fits(n), "k={k} bound={bound} n={n}");
                if n > 0 {
                    assert!(!fits(n - 1), "k={k} bound={bound}: {n} is not minimal");
                }
            }
        }
    }

    #[test]
    fn extremes_and_parameter_errors() {
        assert_eq!(decompose_i64(0, 8, 0), Ok(Vec::new()));
        assert!(decompose_i64(1, 8, 0).is_err());
        assert!(decompose_i64(0, 0, 1).is_err());
        assert!(decompose_i64(0, 1, 1).is_err());
        assert!(decompose_i64(0, 33, 1).is_err());
        assert_eq!(digits_needed(0, 4), Ok(0));
        // i64 extremes with 32-bit digits: 2 digits cover up to ~2^62, 3 are enough for centered 2^62.
        let big = (1i64 << 62) - 1;
        let ds = decompose_i64(big, 32, 3).unwrap_or_default();
        assert_eq!(recompose_i64(&ds, 32), Ok(i128::from(big)));
        let ds = decompose_i64(-big, 32, 3).unwrap_or_default();
        assert_eq!(recompose_i64(&ds, 32), Ok(-i128::from(big)));
    }

    #[test]
    fn ring_decompose_round_trip_and_digit_norm() {
        type R = RingElem<2_147_483_629, 16>;
        let mut s: u64 = 0x1234_5678_9ABC_DEF1;
        for k in [2u32, 3, 8, 16] {
            let len = digits_needed(1_073_741_814, k).unwrap_or(0);
            for _ in 0..50 {
                let xs: alloc::vec::Vec<i64> = (0..16)
                    .map(|_| {
                        s ^= s << 13;
                        s ^= s >> 7;
                        s ^= s << 17;
                        (s % 2_147_483_629) as i64 - 1_073_741_814
                    })
                    .collect();
                let a = R::from_centered(&xs).unwrap_or(R::ZERO);
                let parts = a.decompose_balanced(k, len).unwrap_or_default();
                assert_eq!(parts.len(), len);
                assert!(parts.iter().all(|p| p.linf_norm() <= 1u64 << (k - 1)));
                assert_eq!(R::recompose_balanced(&parts, k), Ok(a));
            }
        }
    }
}
