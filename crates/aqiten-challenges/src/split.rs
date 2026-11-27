//! The two-factor splitting of `X^D + 1` for `q = 5 (mod 8)` and the exact unit test.

use aqiten_error::{Error, Result};
use aqiten_field::{Field, Fp};
use aqiten_ring::RingElem;

/// The splitting structure of `Z_P[X]/(X^D + 1)` for `P = 5 (mod 8)`.
///
/// Using `Split<P>` with `P % 8 != 5` is a compile error:
///
/// ```compile_fail
/// use aqiten_challenges::Split;
/// let _ = Split::<17>::sqrt_minus_one(); // 17 = 1 (mod 8)
/// ```
///
/// # Mathematics
///
/// Let `q = P = 5 (mod 8)`, so `q - 1 = 4u` with `u` odd.
///
/// * `2` is a quadratic non-residue (`q = 5 (mod 8)`), hence `i = 2^((q-1)/4)`
///   satisfies `i^2 = 2^((q-1)/2) = -1` and has order exactly `4`.
/// * For `m = 2^k`, `X^m - a` is irreducible over `F_q` when `q = 1 (mod 4)` iff
///   every prime divisor of `m` divides `ord(a)`, `gcd(m, (q-1)/ord(a)) = 1`, and
///   `4 | q - 1` when `4 | m` (Lidl-Niederreiter, Thm 3.75). For `a = +-i`:
///   `ord(a) = 4`, `(q-1)/4 = u` is odd, so all conditions hold: both
///   `X^m - i` and `X^m + i` are irreducible.
/// * `X^D + 1 = (X^m - i)(X^m + i)` with `m = D/2`, so by CRT
///   `R_q = F_q[X]/(X^m - i) x F_q[X]/(X^m + i)`, a product of two fields.
///   An element is a unit iff both components are non-zero.
/// * Writing `c = c_lo + X^m c_hi`, the component modulo `X^m - eps*i` is
///   `c_lo + eps*i*c_hi`, which vanishes iff `c_j = -eps*i*c_{j+m}` for all `j`.
///
/// **Lemma (small elements are units).** If `delta != 0` and
/// `||delta||_inf < sqrt(q/2)` then `delta` is a unit.
/// *Proof.* Otherwise some sign `eps` has `delta_j = eps*i*delta_{j+m}` for all
/// `j`, hence `delta_j^2 = -delta_{j+m}^2 (mod q)`, i.e. `q | delta_j^2 +
/// delta_{j+m}^2`. But `0 <= delta_j^2 + delta_{j+m}^2 < q/2 + q/2 = q`, so the
/// sum is `0` and `delta_j = delta_{j+m} = 0` for every `j`; so `delta = 0`. QED.
///
/// The bound is essentially tight: `delta_0^2 + delta_m^2 = q` is attainable
/// (tested exhaustively for small `q`).
#[derive(Clone, Copy, Debug)]
pub struct Split<const P: u64>;

impl<const P: u64> Split<P> {
    const VALID: () = assert!(P % 8 == 5, "Split<P>: P must be 5 (mod 8)");

    /// The canonical square root of `-1` in `F_P`: the smaller of
    /// `2^((P-1)/4)` and `P - 2^((P-1)/4)` (as canonical residues).
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] // P >= 5 so P - 1 cannot underflow; field negation is total
    pub fn sqrt_minus_one() -> Fp<P> {
        let () = Self::VALID;
        let i = <Fp<P> as Field>::from_u64(2).pow((P - 1) / 4);
        let neg = -i;
        if i.to_canonical() <= neg.to_canonical() {
            i
        } else {
            neg
        }
    }

    /// Whether `c` is invertible in `Z_P[X]/(X^D + 1)`. Exact. Not constant time
    /// (challenges are public).
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] // field operations are total; D / 2 with D >= 2
    pub fn is_unit<const D: usize>(c: &RingElem<P, D>) -> bool {
        let i = Self::sqrt_minus_one();
        let Some((lo, hi)) = c.coeffs().split_at_checked(D / 2) else {
            return false;
        };
        let plus_vanishes = lo.iter().zip(hi).all(|(a, b)| (*a + i * *b).is_zero());
        let minus_vanishes = lo.iter().zip(hi).all(|(a, b)| (*a - i * *b).is_zero());
        !plus_vanishes && !minus_vanishes
    }

    /// The certified invertibility threshold: `true` iff
    /// `2 * bound^2 < P`, i.e. every non-zero element with `||.||_inf <= bound`
    /// is a unit (lemma above).
    ///
    /// # Errors
    /// [`Error::Overflow`] if `2 * bound^2` overflows `u128` (impossible for `bound < 2^63`).
    pub fn small_elements_are_units(bound: u64) -> Result<bool> {
        let () = Self::VALID;
        let sq = u128::from(bound)
            .checked_mul(u128::from(bound))
            .and_then(|v| v.checked_mul(2))
            .ok_or(Error::Overflow {
                what: "small_elements_are_units",
            })?;
        Ok(sq < u128::from(P))
    }
}
