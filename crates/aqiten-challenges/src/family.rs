//! Sparse challenge families (Akita Section 3.2).

use alloc::vec::Vec;

use aqiten_error::{arith, Error, Result};

use crate::big::Big;
use crate::Split;

/// The family `C(D; (a, n_a))`: elements with exactly `s = sum n_a` non-zero
/// coefficients at uniformly random positions, `n_a` of them of absolute value
/// `a`, each with an independent uniform sign.
///
/// For every member `||c||_inf = c_max` and `||c||_1 = omega = sum a n_a`.
/// For two members, `||c - c'||_inf <= 2 c_max` and `||c - c'||_1 <= 2 omega`.
/// The exact cardinality is
/// `|C| = D!/(D-s)! / prod_a n_a! * 2^s`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SparseFamily {
    degree: usize,
    /// `(magnitude, count)`, magnitudes strictly increasing, counts >= 1.
    groups: Vec<(u32, u32)>,
}

impl SparseFamily {
    /// Builds and validates a family.
    ///
    /// Requirements: `degree` a power of two in `2..=2^16`; at least one group;
    /// magnitudes `>= 1` and pairwise distinct; counts `>= 1`; `s = sum n_a <= degree`.
    /// Groups are stored sorted by magnitude (a canonical order, used by the sampler).
    ///
    /// # Errors
    /// [`Error::InvalidParameter`] on any violated requirement.
    pub fn new(degree: usize, groups: &[(u32, u32)]) -> Result<Self> {
        if !(2..=(1 << 16)).contains(&degree) || !degree.is_power_of_two() {
            return Err(Error::InvalidParameter {
                what: "SparseFamily: degree must be a power of two in 2..=2^16",
            });
        }
        if groups.is_empty() {
            return Err(Error::InvalidParameter {
                what: "SparseFamily: at least one magnitude group is required",
            });
        }
        let mut sorted: Vec<(u32, u32)> = groups.to_vec();
        sorted.sort_unstable();
        let mut support: usize = 0;
        let mut previous: Option<u32> = None;
        for &(a, n) in &sorted {
            if a == 0 || n == 0 {
                return Err(Error::InvalidParameter {
                    what: "SparseFamily: magnitudes and counts must be >= 1",
                });
            }
            if previous == Some(a) {
                return Err(Error::InvalidParameter {
                    what: "SparseFamily: magnitudes must be distinct",
                });
            }
            previous = Some(a);
            support = arith::add_usize(
                support,
                arith::usize_from_u64(u64::from(n), "SparseFamily: count")?,
                "SparseFamily: support size",
            )?;
        }
        if support > degree {
            return Err(Error::InvalidParameter {
                what: "SparseFamily: support size exceeds the degree",
            });
        }
        Ok(Self {
            degree,
            groups: sorted,
        })
    }

    /// The ring degree `D`.
    #[must_use]
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// The `(magnitude, count)` groups, magnitudes increasing.
    #[must_use]
    pub fn groups(&self) -> &[(u32, u32)] {
        &self.groups
    }

    /// The support size `s = sum n_a`.
    #[must_use]
    pub fn support_size(&self) -> usize {
        self.groups
            .iter()
            .map(|&(_, n)| usize::try_from(n).unwrap_or(usize::MAX))
            .fold(0usize, usize::saturating_add)
    }

    /// `c_max`, the common infinity norm of every member (largest magnitude).
    #[must_use]
    pub fn c_max(&self) -> u32 {
        self.groups.last().map_or(0, |&(a, _)| a)
    }

    /// `omega = sum a n_a`, the common `l1` norm of every member.
    ///
    /// # Errors
    /// [`Error::Overflow`] if the sum exceeds `u64` (impossible for validated families).
    pub fn omega(&self) -> Result<u64> {
        let mut w: u64 = 0;
        for &(a, n) in &self.groups {
            w = arith::add_u64(
                w,
                arith::mul_u64(u64::from(a), u64::from(n), "omega: a * n_a")?,
                "omega",
            )?;
        }
        Ok(w)
    }

    /// `floor(log2 |C|)`, exactly (big-integer arithmetic).
    ///
    /// # Errors
    /// Only on internal inexact division (cannot happen for validated families).
    #[allow(clippy::arithmetic_side_effects)] // `degree - i` with i < s <= degree cannot underflow
    pub fn cardinality_log2_floor(&self) -> Result<u64> {
        let mut v = Big::from_u32(1);
        let degree = u32::try_from(self.degree).map_err(|_| Error::Overflow {
            what: "cardinality: degree",
        })?;
        let s = u32::try_from(self.support_size()).map_err(|_| Error::Overflow {
            what: "cardinality: support",
        })?;
        for i in 0..s {
            v.mul_small(degree - i); // i < s <= degree, so no underflow
        }
        for &(_, n) in &self.groups {
            for k in 2..=n {
                v.div_small_exact(k)?;
            }
        }
        v.shl(s);
        Ok(v.bit_length().saturating_sub(1))
    }

    /// Whether `|C| >= 2^bits`, exactly.
    ///
    /// # Errors
    /// As [`Self::cardinality_log2_floor`].
    pub fn cardinality_at_least_two_pow(&self, bits: u64) -> Result<bool> {
        Ok(self.cardinality_log2_floor()? >= bits)
    }

    /// Certifies that **every non-zero difference of two members is invertible**
    /// in `Z_P[X]/(X^D + 1)`: differences have `||.||_inf <= 2 c_max`, and
    /// `2 (2 c_max)^2 < P` implies invertibility ([`Split`], lemma). Requires `P = 5 (mod 8)`.
    ///
    /// # Errors
    /// [`Error::Overflow`] as in [`Split::small_elements_are_units`].
    pub fn differences_are_units<const P: u64>(&self) -> Result<bool> {
        Split::<P>::small_elements_are_units(2u64.saturating_mul(u64::from(self.c_max())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation() {
        assert!(SparseFamily::new(64, &[(1, 31), (2, 10)]).is_ok());
        assert!(SparseFamily::new(63, &[(1, 3)]).is_err());
        assert!(SparseFamily::new(1, &[(1, 1)]).is_err());
        assert!(SparseFamily::new(64, &[]).is_err());
        assert!(SparseFamily::new(64, &[(0, 3)]).is_err());
        assert!(SparseFamily::new(64, &[(1, 0)]).is_err());
        assert!(SparseFamily::new(64, &[(1, 3), (1, 4)]).is_err());
        assert!(SparseFamily::new(8, &[(1, 5), (2, 4)]).is_err()); // s = 9 > 8
        assert!(SparseFamily::new(8, &[(1, 5), (2, 3)]).is_ok());
    }

    #[test]
    fn small_cardinalities_by_hand() {
        // D=4, one magnitude, s=2: C(4,2)*4 = 24
        let f = SparseFamily::new(4, &[(1, 2)]).unwrap_or_else(|_| unreachable_family());
        assert_eq!(f.cardinality_log2_floor(), Ok(4)); // 24 -> floor(log2) = 4
                                                       // D=4, (1,1)+(2,1): 4*3 positions * 4 signs = 48 -> 5
        let g = SparseFamily::new(4, &[(2, 1), (1, 1)]).unwrap_or_else(|_| unreachable_family());
        assert_eq!(g.cardinality_log2_floor(), Ok(5));
        assert_eq!(g.groups(), &[(1, 1), (2, 1)]);
        assert_eq!(g.c_max(), 2);
        assert_eq!(g.omega(), Ok(3));
        assert_eq!(g.support_size(), 2);
    }

    fn unreachable_family() -> SparseFamily {
        SparseFamily {
            degree: 2,
            groups: alloc::vec![(1, 1)],
        }
    }
}
