//! Exact integer norms on centered representatives.

use aqiten_error::{arith, Error, Result};

use crate::RingElem;

impl<const P: u64, const D: usize> RingElem<P, D> {
    /// The exact infinity norm: `max_i |c_i|` over centered representatives.
    /// At most `(P-1)/2 < 2^62`.
    #[must_use]
    pub fn linf_norm(&self) -> u64 {
        self.c
            .iter()
            .map(|x| x.to_centered_i64().unsigned_abs())
            .max()
            .unwrap_or(0)
    }

    /// Whether `linf_norm() <= bound`.
    #[must_use]
    pub fn within_linf(&self, bound: u64) -> bool {
        self.linf_norm() <= bound
    }

    /// The exact squared Euclidean norm `sum_i c_i^2` over centered representatives.
    ///
    /// # Errors
    /// [`Error::Overflow`] if the sum does not fit in `u128` (impossible for
    /// `D <= 2^16` and `P < 2^63`, but checked rather than assumed).
    pub fn l2_norm_squared(&self) -> Result<u128> {
        let mut acc: u128 = 0;
        for x in &self.c {
            let a = u128::from(x.to_centered_i64().unsigned_abs());
            let sq = a.checked_mul(a).ok_or(Error::Overflow {
                what: "l2_norm_squared: square",
            })?;
            acc = acc.checked_add(sq).ok_or(Error::Overflow {
                what: "l2_norm_squared: sum",
            })?;
        }
        Ok(acc)
    }

    /// Whether `l2_norm_squared() <= bound_sq`.
    ///
    /// # Errors
    /// As [`Self::l2_norm_squared`].
    pub fn within_l2_squared(&self, bound_sq: u128) -> Result<bool> {
        Ok(self.l2_norm_squared()? <= bound_sq)
    }

    /// The number of coefficients whose centered representative is non-zero.
    ///
    /// # Errors
    /// [`Error::Overflow`] cannot occur for valid `D`; the signature keeps the
    /// workspace rule that counters are checked.
    pub fn hamming_weight(&self) -> Result<usize> {
        let mut n = 0usize;
        for x in &self.c {
            if !aqiten_field::Field::is_zero(x) {
                n = arith::add_usize(n, 1, "hamming_weight")?;
            }
        }
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::RingElem;

    type R = RingElem<257, 4>;

    #[test]
    fn norms_known_answers() {
        let a = R::from_centered(&[3, -4, 0, 128]).unwrap_or(R::ZERO);
        assert_eq!(a.linf_norm(), 128);
        assert_eq!(a.l2_norm_squared(), Ok(9 + 16 + 128 * 128));
        assert_eq!(a.hamming_weight(), Ok(3));
        assert!(a.within_linf(128));
        assert!(!a.within_linf(127));
    }

    #[test]
    fn centered_boundary_is_exact() {
        // 129 = -128 mod 257, so its norm is 128, not 129.
        let a = R::from_centered(&[129, 0, 0, 0]).unwrap_or(R::ZERO);
        assert_eq!(a.linf_norm(), 128);
        assert_eq!(R::ZERO.linf_norm(), 0);
        assert_eq!(R::ZERO.l2_norm_squared(), Ok(0));
    }

    #[test]
    fn norm_is_of_the_residue_not_the_input_integer() {
        // 257 + 3 reduces to 3: no wrap-around is visible to the norm.
        let a = R::from_centered(&[260, 0, 0, 0]).unwrap_or(R::ZERO);
        assert_eq!(a.linf_norm(), 3);
    }
}
