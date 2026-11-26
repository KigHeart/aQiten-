//! `Fp<P>`: arithmetic modulo a compile-time-checked prime `P < 2^63`.
//!
//! # Representation
//!
//! A value `x` is stored as `x * R mod P` with `R = 2^64`, fully reduced into
//! `[0, P)` ("Montgomery form"). Because the stored word is always fully
//! reduced, it is a unique representative, so equality of stored words is
//! equality of field elements.
//!
//! # Bounds used by the arithmetic (all rely on `P < 2^63`)
//!
//! * `a, b < P  =>  a + b < 2P < 2^64`: addition never overflows a `u64`.
//! * `a, b < P  =>  a * b < P^2 < P * 2^64`: the product satisfies the input
//!   requirement `T < P * R` of Montgomery reduction.
//! * For `T < P * R`, `m = (T mod R) * (-P^{-1}) mod R`, the value
//!   `T + m * P` is divisible by `R`, is below `2 * P * R < 2^128`, and
//!   `(T + m * P) / R < 2P`; one conditional subtraction finishes the reduction.

// The proofs of non-overflow are the comments above; each operation below states
// which bound it uses. Lints about arithmetic would flag every line otherwise.
#![allow(
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use core::fmt;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use aqiten_codec::{Decode, Encode, Reader, Writer};
use aqiten_error::{Error, Result};

use crate::prime::is_prime;
use crate::Field;

/// An element of the prime field `Z / P`, stored in Montgomery form.
///
/// ```
/// use aqiten_field::{Field, Fp};
/// type F = Fp<4_294_967_197>;
/// let a = F::from_u64(5);
/// let b = F::from_u64(7);
/// assert_eq!((a * b).to_canonical(), 35);
/// assert_eq!(a.inv().map(|x| x * a), Ok(F::ONE));
/// ```
///
/// A composite or even modulus does not compile:
///
/// ```compile_fail
/// use aqiten_field::{Field, Fp};
/// let _ = Fp::<15>::ONE;
/// ```
///
/// Nor does a modulus of 63 bits or more:
///
/// ```compile_fail
/// use aqiten_field::{Field, Fp};
/// let _ = Fp::<9_223_372_036_854_775_837>::ONE; // prime above 2^63
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fp<const P: u64>(u64);

/// Computes `-p^{-1} mod 2^64` for odd `p` by Newton iteration.
///
/// Starting from `inv = p` (correct to 3 bits because `p*p = 1 mod 8` for odd
/// `p`), each step `inv <- inv * (2 - p * inv)` doubles the number of correct
/// low bits: 3, 6, 12, 24, 48, 96 >= 64.
const fn neg_inv_mod_2_64(p: u64) -> u64 {
    let mut inv = p;
    let mut i = 0;
    while i < 6 {
        inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv)));
        i += 1;
    }
    inv.wrapping_neg()
}

impl<const P: u64> Fp<P> {
    /// Compile-time validity of the modulus. Evaluated whenever any associated
    /// constant below is used, so every use of `Fp<P>` checks it.
    const VALID: () = assert!(
        P >= 3 && P < (1u64 << 63) && is_prime(P),
        "Fp<P>: P must be a prime with 3 <= P < 2^63"
    );

    /// `-P^{-1} mod 2^64`.
    const NEG_P_INV: u64 = {
        let () = Self::VALID;
        neg_inv_mod_2_64(P)
    };

    /// `R mod P` where `R = 2^64`: the Montgomery form of one.
    const R_MOD_P: u64 = {
        let () = Self::VALID;
        ((1u128 << 64) % P as u128) as u64
    };

    /// `R^2 mod P`: multiplying by it converts into Montgomery form.
    const R2_MOD_P: u64 = {
        let () = Self::VALID;
        ((Self::R_MOD_P as u128 * Self::R_MOD_P as u128) % P as u128) as u64
    };

    /// Number of bytes in the canonical encoding: `ceil(bit_length(P - 1) / 8)`.
    pub const ENCODED_BYTES: usize = {
        let () = Self::VALID;
        ((u64::BITS - (P - 1).leading_zeros()) as usize).div_ceil(8)
    };

    /// The modulus.
    pub const MODULUS: u64 = {
        let () = Self::VALID;
        P
    };

    /// Reduces `x < 2P` into `[0, P)` without branching on `x`.
    #[inline]
    #[allow(clippy::cast_lossless)] // `u64::from(bool)` is not available in const fn
    const fn reduce_once(x: u64) -> u64 {
        let (diff, borrow) = x.overflowing_sub(P);
        // borrow == true  <=>  x < P  <=>  keep x, i.e. add P back to the wrapped difference.
        let mask = 0u64.wrapping_sub(borrow as u64);
        diff.wrapping_add(P & mask)
    }

    /// Montgomery reduction: for `t < P * 2^64` returns `t * 2^{-64} mod P`, fully reduced.
    #[inline]
    const fn redc(t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(Self::NEG_P_INV);
        // t < P*2^64 < 2^127 and m*P < 2^64 * 2^63 = 2^127, so the sum is below 2^128.
        let u = t + (m as u128) * (P as u128);
        Self::reduce_once((u >> 64) as u64)
    }

    /// Selects `a` when `mask == u64::MAX` and `b` when `mask == 0`.
    #[inline]
    #[allow(clippy::cast_lossless)]
    const fn select(mask: u64, a: u64, b: u64) -> u64 {
        (a & mask) | (b & !mask)
    }

    /// Builds an element from its canonical representative.
    ///
    /// # Errors
    /// [`Error::NonCanonical`] when `x >= P`. The decision is on the validity of
    /// an input and is treated as public.
    pub fn from_canonical(x: u64) -> Result<Self> {
        if x >= P {
            return Err(Error::NonCanonical {
                what: "field element",
            });
        }
        // x < P and R2 < P, so x * R2 < P * 2^64 as redc requires.
        Ok(Self(Self::redc(u128::from(x) * u128::from(Self::R2_MOD_P))))
    }

    /// The canonical representative in `[0, P)`.
    #[must_use]
    pub fn to_canonical(self) -> u64 {
        // Montgomery reduction of the stored word multiplies by R^{-1}.
        Self::redc(u128::from(self.0))
    }

    /// Reduces an arbitrary `u64` modulo `P`.
    #[must_use]
    pub fn from_u64_reduced(x: u64) -> Self {
        // x < 2^64 and R2 < P, so x * R2 < P * 2^64 as redc requires.
        Self(Self::redc(u128::from(x) * u128::from(Self::R2_MOD_P)))
    }

    /// Maps a signed integer to its residue, without branching on the sign.
    #[must_use]
    pub fn from_i64(x: i64) -> Self {
        let magnitude = Self::from_u64_reduced(x.unsigned_abs());
        let negative_mask = 0u64.wrapping_sub((x as u64) >> 63);
        let negated = -magnitude;
        Self(Self::select(negative_mask, negated.0, magnitude.0))
    }

    /// The centred representative in `[-(P-1)/2, (P-1)/2]`, without branching on the value.
    #[must_use]
    pub fn to_centered_i64(self) -> i64 {
        let c = self.to_canonical();
        let half = (P - 1) / 2;
        // c > half  <=>  (half - c) borrows.
        let (_, borrow) = half.overflowing_sub(c);
        let mask = 0u64.wrapping_sub(u64::from(borrow));
        // For c > half subtract P (wrapping); |result| <= (P-1)/2 < 2^62 so it fits i64.
        c.wrapping_sub(P & mask) as i64
    }

    /// All-ones if `self == other`, zero otherwise; branch-free.
    #[must_use]
    pub fn ct_eq_mask(self, other: Self) -> u64 {
        let d = self.0 ^ other.0;
        // d == 0  <=>  (d | -d) has top bit clear.
        let nonzero = (d | d.wrapping_neg()) >> 63;
        nonzero.wrapping_sub(1)
    }

    /// Returns `a` when `mask == u64::MAX` and `b` when `mask == 0`. Any other
    /// mask value is a caller bug and yields an unspecified (but defined) element.
    #[must_use]
    pub fn ct_select(mask: u64, a: Self, b: Self) -> Self {
        Self(Self::select(mask, a.0, b.0))
    }
}

impl<const P: u64> Add for Fp<P> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        // Both words are < P, so the sum is < 2P < 2^64.
        Self(Self::reduce_once(self.0 + rhs.0))
    }
}

impl<const P: u64> Sub for Fp<P> {
    type Output = Self;
    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)] // masked add-back of P is the intended borrow fix-up
    fn sub(self, rhs: Self) -> Self {
        let (diff, borrow) = self.0.overflowing_sub(rhs.0);
        let mask = 0u64.wrapping_sub(u64::from(borrow));
        Self(diff.wrapping_add(P & mask))
    }
}

impl<const P: u64> Neg for Fp<P> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(0) - self
    }
}

impl<const P: u64> Mul for Fp<P> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        // Both words are < P, so the product is < P^2 < P * 2^64.
        Self(Self::redc(u128::from(self.0) * u128::from(rhs.0)))
    }
}

impl<const P: u64> AddAssign for Fp<P> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<const P: u64> SubAssign for Fp<P> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<const P: u64> MulAssign for Fp<P> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const P: u64> fmt::Debug for Fp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Prints the canonical value. Do not log secret field elements.
        write!(f, "Fp<{P}>({})", self.to_canonical())
    }
}

impl<const P: u64> Field for Fp<P> {
    const ZERO: Self = {
        let () = Self::VALID;
        Self(0)
    };
    const ONE: Self = {
        let () = Self::VALID;
        Self(Self::R_MOD_P)
    };
    const CHARACTERISTIC: u64 = P;

    fn from_u64(x: u64) -> Self {
        Self::from_u64_reduced(x)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn pow(self, exp: u64) -> Self {
        // Fixed 64-iteration left-to-right ladder: the sequence of operations does
        // not depend on the exponent bits (a mask selects whether the multiply counts).
        let mut acc = Self::ONE;
        let mut i = 64;
        while i > 0 {
            i -= 1;
            acc = acc * acc;
            let times_base = acc * self;
            let mask = 0u64.wrapping_sub((exp >> i) & 1);
            acc = Self::ct_select(mask, times_base, acc);
        }
        acc
    }

    fn inv(self) -> Result<Self> {
        if self.is_zero() {
            return Err(Error::NotInvertible);
        }
        // Fermat: x^{P-2} = x^{-1} for x != 0 in a prime field.
        Ok(self.pow(P - 2))
    }
}

impl<const P: u64> Encode for Fp<P> {
    /// Canonical little-endian encoding in [`Fp::ENCODED_BYTES`] bytes.
    fn encode(&self, w: &mut Writer) -> Result<()> {
        let bytes = self.to_canonical().to_le_bytes();
        let used = bytes
            .get(..Self::ENCODED_BYTES)
            .ok_or(Error::InvalidParameter {
                what: "field element encoding width",
            })?;
        w.put_raw(used);
        Ok(())
    }
}

impl<const P: u64> Decode for Fp<P> {
    /// Reads [`Fp::ENCODED_BYTES`] bytes and rejects any value `>= P`.
    fn decode(r: &mut Reader<'_>) -> Result<Self> {
        let bytes = r.take(Self::ENCODED_BYTES)?;
        let mut x = 0u64;
        for &b in bytes.iter().rev() {
            x = (x << 8) | u64::from(b);
        }
        Self::from_canonical(x)
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    fn montgomery_constants_hold<const P: u64>() {
        // -P^{-1} * P = -1 mod 2^64.
        assert_eq!(P.wrapping_mul(Fp::<P>::NEG_P_INV), u64::MAX, "P = {P}");
        // R mod P and R^2 mod P against the definition.
        assert_eq!(u128::from(Fp::<P>::R_MOD_P), (1u128 << 64) % u128::from(P));
        assert_eq!(
            u128::from(Fp::<P>::R2_MOD_P),
            (u128::from(Fp::<P>::R_MOD_P) * u128::from(Fp::<P>::R_MOD_P)) % u128::from(P)
        );
        // Stored words are always fully reduced.
        assert!(Fp::<P>::R_MOD_P < P && Fp::<P>::R2_MOD_P < P);
    }

    #[test]
    fn constants_for_representative_moduli() {
        montgomery_constants_hold::<3>();
        montgomery_constants_hold::<257>();
        montgomery_constants_hold::<65_537>();
        montgomery_constants_hold::<2_147_483_629>();
        montgomery_constants_hold::<4_294_967_197>();
        montgomery_constants_hold::<2_305_843_009_213_693_951>();
        montgomery_constants_hold::<9_223_372_036_854_775_783>();
    }

    #[test]
    fn reduce_once_boundaries() {
        type F = Fp<4_294_967_197>;
        assert_eq!(F::reduce_once(0), 0);
        assert_eq!(F::reduce_once(4_294_967_196), 4_294_967_196);
        assert_eq!(F::reduce_once(4_294_967_197), 0);
        assert_eq!(F::reduce_once(2 * 4_294_967_197 - 1), 4_294_967_196);
    }
}
