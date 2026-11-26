//! Checked integer formulas shared by sizing, parameter and codec code.
//!
//! All functions return [`Error::Overflow`] instead of wrapping. They are the
//! only place workspace code is allowed to do size arithmetic on untrusted or
//! derived quantities.

use crate::{Error, Result};

/// `a + b` on `u64`, or an overflow error naming `what`.
///
/// # Errors
/// [`Error::Overflow`] when the sum exceeds `u64::MAX`.
pub fn add_u64(a: u64, b: u64, what: &'static str) -> Result<u64> {
    a.checked_add(b).ok_or(Error::Overflow { what })
}

/// `a * b` on `u64`, or an overflow error naming `what`.
///
/// # Errors
/// [`Error::Overflow`] when the product exceeds `u64::MAX`.
pub fn mul_u64(a: u64, b: u64, what: &'static str) -> Result<u64> {
    a.checked_mul(b).ok_or(Error::Overflow { what })
}

/// `a - b` on `u64`, or an overflow (underflow) error naming `what`.
///
/// # Errors
/// [`Error::Overflow`] when `b > a`.
pub fn sub_u64(a: u64, b: u64, what: &'static str) -> Result<u64> {
    a.checked_sub(b).ok_or(Error::Overflow { what })
}

/// `a + b` on `usize`, or an overflow error naming `what`.
///
/// # Errors
/// [`Error::Overflow`] when the sum exceeds `usize::MAX`.
pub fn add_usize(a: usize, b: usize, what: &'static str) -> Result<usize> {
    a.checked_add(b).ok_or(Error::Overflow { what })
}

/// `a * b` on `usize`, or an overflow error naming `what`.
///
/// # Errors
/// [`Error::Overflow`] when the product exceeds `usize::MAX`.
pub fn mul_usize(a: usize, b: usize, what: &'static str) -> Result<usize> {
    a.checked_mul(b).ok_or(Error::Overflow { what })
}

/// Converts `u64` to `usize`, failing on 32-bit targets when the value does not fit.
///
/// # Errors
/// [`Error::Overflow`] when `x` does not fit in `usize`.
pub fn usize_from_u64(x: u64, what: &'static str) -> Result<usize> {
    usize::try_from(x).map_err(|_| Error::Overflow { what })
}

/// Converts `usize` to `u64`. Always succeeds on supported targets (`usize` is at
/// most 64 bits) but is checked so that the assumption is explicit.
///
/// # Errors
/// [`Error::Overflow`] when `x` does not fit in `u64`.
pub fn u64_from_usize(x: usize, what: &'static str) -> Result<u64> {
    u64::try_from(x).map_err(|_| Error::Overflow { what })
}

/// `ceil(a / b)` for `b > 0`.
///
/// # Errors
/// [`Error::InvalidParameter`] when `b == 0`.
pub fn ceil_div_u64(a: u64, b: u64, what: &'static str) -> Result<u64> {
    if b == 0 {
        return Err(Error::InvalidParameter { what });
    }
    // `div_ceil` cannot overflow, unlike the textbook `(a + b - 1) / b`.
    Ok(a.div_ceil(b))
}

/// `2^k` as `u64`, failing when `k >= 64`.
///
/// # Errors
/// [`Error::Overflow`] when `k >= 64`.
pub fn pow2_u64(k: u32, what: &'static str) -> Result<u64> {
    1u64.checked_shl(k).ok_or(Error::Overflow { what })
}

/// The smallest power of two that is `>= x` (with `next_pow2(0) = 1`), failing
/// when the result does not fit in `u64`.
///
/// # Errors
/// [`Error::Overflow`] when the result exceeds `u64::MAX`.
pub fn next_pow2_u64(x: u64, what: &'static str) -> Result<u64> {
    x.checked_next_power_of_two()
        .ok_or(Error::Overflow { what })
}

/// `ceil(log2(x))` for `x >= 1`; `ceil_log2(1) = 0`.
///
/// # Errors
/// [`Error::InvalidParameter`] when `x == 0`.
pub fn ceil_log2_u64(x: u64, what: &'static str) -> Result<u32> {
    if x == 0 {
        return Err(Error::InvalidParameter { what });
    }
    // For x >= 1: ceil(log2 x) = 64 - leading_zeros(x - 1), and 0 when x == 1.
    // `x - 1` cannot underflow (x >= 1) and `leading_zeros <= 64`, so neither
    // saturating step below ever saturates; they only avoid raw arithmetic.
    Ok(u64::BITS.saturating_sub((x.saturating_sub(1)).leading_zeros()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_mul_sub_detect_overflow() {
        assert_eq!(add_u64(1, 2, "t"), Ok(3));
        assert_eq!(
            add_u64(u64::MAX, 1, "t"),
            Err(Error::Overflow { what: "t" })
        );
        assert_eq!(mul_u64(1 << 32, 1 << 31, "t"), Ok(1 << 63));
        assert_eq!(
            mul_u64(1 << 32, 1 << 32, "t"),
            Err(Error::Overflow { what: "t" })
        );
        assert_eq!(sub_u64(5, 5, "t"), Ok(0));
        assert_eq!(sub_u64(4, 5, "t"), Err(Error::Overflow { what: "t" }));
        assert_eq!(
            add_usize(usize::MAX, 1, "t"),
            Err(Error::Overflow { what: "t" })
        );
        assert_eq!(
            mul_usize(usize::MAX, 2, "t"),
            Err(Error::Overflow { what: "t" })
        );
    }

    #[test]
    fn conversions_round_trip_on_this_target() {
        assert_eq!(usize_from_u64(7, "t"), Ok(7usize));
        assert_eq!(u64_from_usize(7, "t"), Ok(7u64));
    }

    #[test]
    fn ceil_div_matches_definition() {
        for a in 0u64..200 {
            for b in 1u64..20 {
                let expected = a.div_ceil(b);
                assert_eq!(ceil_div_u64(a, b, "t"), Ok(expected), "a={a} b={b}");
            }
        }
        assert_eq!(ceil_div_u64(u64::MAX, 1, "t"), Ok(u64::MAX));
        assert_eq!(ceil_div_u64(u64::MAX, 2, "t"), Ok(1 << 63));
        assert_eq!(
            ceil_div_u64(1, 0, "t"),
            Err(Error::InvalidParameter { what: "t" })
        );
    }

    #[test]
    fn pow2_and_next_pow2() {
        assert_eq!(pow2_u64(0, "t"), Ok(1));
        assert_eq!(pow2_u64(63, "t"), Ok(1 << 63));
        assert_eq!(pow2_u64(64, "t"), Err(Error::Overflow { what: "t" }));
        assert_eq!(next_pow2_u64(0, "t"), Ok(1));
        assert_eq!(next_pow2_u64(1, "t"), Ok(1));
        assert_eq!(next_pow2_u64(3, "t"), Ok(4));
        assert_eq!(next_pow2_u64(1 << 63, "t"), Ok(1 << 63));
        assert_eq!(
            next_pow2_u64((1 << 63) + 1, "t"),
            Err(Error::Overflow { what: "t" })
        );
    }

    #[test]
    fn ceil_log2_matches_definition() {
        assert_eq!(
            ceil_log2_u64(0, "t"),
            Err(Error::InvalidParameter { what: "t" })
        );
        assert_eq!(ceil_log2_u64(1, "t"), Ok(0));
        assert_eq!(ceil_log2_u64(2, "t"), Ok(1));
        assert_eq!(ceil_log2_u64(3, "t"), Ok(2));
        assert_eq!(ceil_log2_u64(4, "t"), Ok(2));
        assert_eq!(ceil_log2_u64(5, "t"), Ok(3));
        assert_eq!(ceil_log2_u64(u64::MAX, "t"), Ok(64));
        // Exhaustive against the definition for small x.
        for x in 1u64..4096 {
            let k = ceil_log2_u64(x, "t").unwrap_or(u32::MAX);
            assert!((1u64 << k) >= x, "2^k >= x failed for x={x}");
            if k > 0 {
                assert!((1u64 << (k - 1)) < x, "minimality failed for x={x}");
            }
        }
    }

    #[test]
    fn error_display_has_no_panics_and_names_the_object() {
        extern crate alloc;
        use alloc::string::ToString;
        let e = Error::Overflow {
            what: "matrix rows",
        };
        assert!(e.to_string().contains("matrix rows"));
        let e = Error::LengthTooLarge {
            declared: 9,
            max: 4,
        };
        assert!(e.to_string().contains('9') && e.to_string().contains('4'));
    }
}
