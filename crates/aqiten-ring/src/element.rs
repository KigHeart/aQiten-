//! The ring element type and its exact arithmetic.

use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use aqiten_error::{Error, Result};
use aqiten_field::{Field, Fp};

/// An element of `Z_P[X]/(X^D + 1)`, stored as its `D` coefficients,
/// lowest degree first.
///
/// Equality is coefficient-wise and exact (each [`Fp`] is fully reduced).
///
/// A degree that is not a power of two in `2..=2^16` is a compile error:
///
/// ```compile_fail
/// use aqiten_ring::RingElem;
/// let _ = RingElem::<17, 3>::ZERO;
/// ```
///
/// and so is a modulus that is not prime:
///
/// ```compile_fail
/// use aqiten_ring::RingElem;
/// let _ = RingElem::<15, 4>::ZERO;
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RingElem<const P: u64, const D: usize> {
    pub(crate) c: [Fp<P>; D],
}

impl<const P: u64, const D: usize> RingElem<P, D> {
    /// Compile-time validity of the degree (the modulus is checked by [`Fp`]).
    const VALID: () = assert!(
        D >= 2 && D <= (1 << 16) && D.is_power_of_two(),
        "RingElem<P, D>: D must be a power of two with 2 <= D <= 2^16"
    );

    /// The degree `D` (number of coefficients).
    pub const DEGREE: usize = {
        let () = Self::VALID;
        D
    };

    /// The zero polynomial.
    pub const ZERO: Self = {
        let () = Self::VALID;
        Self {
            c: [<Fp<P> as Field>::ZERO; D],
        }
    };

    /// The constant polynomial `1`.
    #[must_use]
    pub fn one() -> Self {
        Self::constant(<Fp<P> as Field>::ONE)
    }

    /// The constant polynomial `a`.
    #[must_use]
    pub fn constant(a: Fp<P>) -> Self {
        let mut r = Self::ZERO;
        if let Some(first) = r.c.first_mut() {
            *first = a;
        }
        r
    }

    /// Builds an element from its coefficients, lowest degree first.
    #[must_use]
    pub fn from_coeffs(c: [Fp<P>; D]) -> Self {
        let () = Self::VALID;
        Self { c }
    }

    /// Builds an element from signed integer coefficients, reducing each modulo `P`.
    ///
    /// # Errors
    /// [`Error::InvalidParameter`] if `xs.len() != D`.
    pub fn from_centered(xs: &[i64]) -> Result<Self> {
        if xs.len() != D {
            return Err(Error::InvalidParameter {
                what: "from_centered: slice length must equal D",
            });
        }
        let mut r = Self::ZERO;
        for (dst, &x) in r.c.iter_mut().zip(xs) {
            *dst = Fp::<P>::from_i64(x);
        }
        Ok(r)
    }

    /// The coefficients, lowest degree first.
    #[must_use]
    pub fn coeffs(&self) -> &[Fp<P>; D] {
        &self.c
    }

    /// The centered integer representatives of the coefficients, each in
    /// `[-(P-1)/2, (P-1)/2]`.
    #[must_use]
    pub fn to_centered(&self) -> [i64; D] {
        let mut out = [0i64; D];
        for (dst, src) in out.iter_mut().zip(self.c.iter()) {
            *dst = src.to_centered_i64();
        }
        out
    }

    /// Whether every coefficient is zero. Not constant time.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.c.iter().all(Field::is_zero)
    }

    /// Multiplies every coefficient by the scalar `a`.
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] // field multiplication is total
    pub fn scale(&self, a: Fp<P>) -> Self {
        let mut r = *self;
        for x in &mut r.c {
            *x *= a;
        }
        r
    }

    /// Multiplication by `X`: `sum a_i X^i -> sum a_i X^(i+1)` with `X^D = -1`.
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] // field negation is total
    pub fn mul_x(&self) -> Self {
        let mut r = *self;
        r.c.rotate_right(1);
        if let Some(first) = r.c.first_mut() {
            *first = -*first;
        }
        r
    }

    /// Multiplication by the monomial `X^k` for any `k` (`X^(2D) = 1`, so only
    /// `k mod 2D` matters). Runs in `O(D)`, independent of the coefficient values.
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] // k % (2D) and k % D are total (D >= 2); negation is total
    pub fn mul_monomial(&self, k: usize) -> Self {
        let k = k % (2 * D);
        let (wrap, shift) = (k / D, k % D);
        let mut r = *self;
        r.c.rotate_right(shift);
        for x in r.c.iter_mut().take(shift) {
            *x = -*x;
        }
        if wrap == 1 {
            for x in &mut r.c {
                *x = -*x;
            }
        }
        r
    }
}

impl<const P: u64, const D: usize> Add for RingElem<P, D> {
    type Output = Self;
    #[allow(clippy::arithmetic_side_effects)] // field addition is total
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<const P: u64, const D: usize> AddAssign for RingElem<P, D> {
    #[allow(clippy::arithmetic_side_effects)]
    fn add_assign(&mut self, rhs: Self) {
        for (a, b) in self.c.iter_mut().zip(rhs.c.iter()) {
            *a += *b;
        }
    }
}

impl<const P: u64, const D: usize> Sub for RingElem<P, D> {
    type Output = Self;
    #[allow(clippy::arithmetic_side_effects)]
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl<const P: u64, const D: usize> SubAssign for RingElem<P, D> {
    #[allow(clippy::arithmetic_side_effects)]
    fn sub_assign(&mut self, rhs: Self) {
        for (a, b) in self.c.iter_mut().zip(rhs.c.iter()) {
            *a -= *b;
        }
    }
}

impl<const P: u64, const D: usize> Neg for RingElem<P, D> {
    type Output = Self;
    #[allow(clippy::arithmetic_side_effects)] // field negation is total
    fn neg(self) -> Self {
        let mut r = self;
        for x in &mut r.c {
            *x = -*x;
        }
        r
    }
}

impl<const P: u64, const D: usize> Mul for RingElem<P, D> {
    type Output = Self;
    /// Negacyclic product `self * rhs mod (X^D + 1)`, computed as
    /// `sum_i self_i * (rhs * X^i)`: `D` scaled additions and `D` negacyclic shifts.
    #[allow(clippy::arithmetic_side_effects)] // field operations are total
    fn mul(self, rhs: Self) -> Self {
        let mut acc = Self::ZERO;
        let mut shifted = rhs;
        for a in &self.c {
            for (t, s) in acc.c.iter_mut().zip(shifted.c.iter()) {
                *t += *a * *s;
            }
            shifted = shifted.mul_x();
        }
        acc
    }
}

impl<const P: u64, const D: usize> MulAssign for RingElem<P, D> {
    #[allow(clippy::arithmetic_side_effects)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type R8 = RingElem<257, 8>;

    fn from_ints<const P: u64, const D: usize>(v: &[i64]) -> RingElem<P, D> {
        RingElem::from_centered(v).unwrap_or(RingElem::ZERO)
    }

    #[test]
    fn x_to_the_d_is_minus_one() {
        let mut x = R8::ZERO;
        x.c[1] = Fp::from_u64(1);
        let mut p = R8::one();
        for _ in 0..8 {
            p *= x;
        }
        assert_eq!(p, -R8::one());
        p *= p;
        assert_eq!(p, R8::one());
    }

    #[test]
    fn mul_x_matches_definition() {
        let a: R8 = from_ints(&[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(a.mul_x(), from_ints(&[-8, 1, 2, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn monomial_matches_repeated_mul_x_for_all_k() {
        let a: R8 = from_ints(&[1, -2, 3, -4, 5, -6, 7, -8]);
        let mut expect = a;
        for k in 0..40usize {
            assert_eq!(a.mul_monomial(k), expect, "k = {k}");
            expect = expect.mul_x();
        }
    }

    #[test]
    fn centered_round_trip_and_length_check() {
        let a: R8 = from_ints(&[0, 1, -1, 128, -128, 5, -5, 100]);
        assert_eq!(a.to_centered(), [0, 1, -1, 128, -128, 5, -5, 100]);
        assert!(R8::from_centered(&[1, 2, 3]).is_err());
    }

    #[test]
    fn small_known_product() {
        // (1 + X)(1 - X) = 1 - X^2 in Z[X]/(X^8+1)
        let a: R8 = from_ints(&[1, 1, 0, 0, 0, 0, 0, 0]);
        let b: R8 = from_ints(&[1, -1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(a * b, from_ints(&[1, 0, -1, 0, 0, 0, 0, 0]));
        // X^7 * X^7 = X^14 = -X^6
        let x7 = R8::one().mul_monomial(7);
        assert_eq!(x7 * x7, R8::one().mul_monomial(14));
        assert_eq!(x7 * x7, from_ints(&[0, 0, 0, 0, 0, 0, -1, 0]));
    }
}
