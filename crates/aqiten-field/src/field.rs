//! The [`Field`] trait that downstream crates are generic over.

use core::fmt::Debug;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use aqiten_codec::{Decode, Encode};
use aqiten_error::Result;

/// A finite field with a canonical byte encoding.
///
/// The trait is intentionally small: it contains what the ring, transcript and
/// sum-check crates need and nothing about how a particular field is built.
pub trait Field:
    Copy
    + Eq
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + Encode
    + Decode
{
    /// The additive identity.
    const ZERO: Self;
    /// The multiplicative identity.
    const ONE: Self;
    /// The characteristic of the field (a prime).
    const CHARACTERISTIC: u64;

    /// The image of `x` under the canonical ring homomorphism `Z -> F`
    /// (reduction modulo the characteristic).
    fn from_u64(x: u64) -> Self;

    /// Whether this is the additive identity. Not constant time.
    fn is_zero(&self) -> bool;

    /// `self * self`.
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] // field multiplication is total; no overflow
    fn square(self) -> Self {
        self * self
    }

    /// `self^exp`, with the exponent treated as public.
    #[must_use]
    fn pow(self, exp: u64) -> Self;

    /// The multiplicative inverse.
    ///
    /// # Errors
    /// [`aqiten_error::Error::NotInvertible`] when `self` is zero.
    fn inv(self) -> Result<Self>;
}
