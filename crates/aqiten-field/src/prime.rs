//! Compile-time primality test and modular helpers used to validate moduli.
//!
//! Everything here is `const fn` so that [`crate::Fp`] can reject a bad modulus
//! during compilation. None of it handles secrets: it only ever sees the public
//! modulus, so variable-time `%` is acceptable.

// These functions operate on public constants at compile time; wrapping and
// division behaviour is intended and bounded by the argument ranges documented.
#![allow(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::many_single_char_names
)]

/// `a * b mod m` using a 128-bit intermediate. Requires `m > 0`.
#[must_use]
#[allow(clippy::cast_possible_truncation)] // the result is < m <= u64::MAX
pub(crate) const fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    // (a as u128) * (b as u128) < 2^128 for any u64 a, b, so no overflow.
    ((a as u128 * b as u128) % m as u128) as u64
}

/// `base^exp mod m` by square-and-multiply. Requires `m > 0`.
#[must_use]
pub(crate) const fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut acc = 1 % m;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mul_mod(acc, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    acc
}

/// Deterministic Miller–Rabin primality test for every `u64`.
///
/// The twelve bases `2, 3, 5, ..., 37` are a proven deterministic witness set
/// for all `n < 3.3 * 10^24`, which covers the whole `u64` range (Sorenson and
/// Webster, 2015). Returns `false` for `0` and `1`.
#[must_use]
pub const fn is_prime(n: u64) -> bool {
    const BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    if n < 2 {
        return false;
    }
    let mut i = 0;
    while i < BASES.len() {
        let p = BASES[i];
        if n % p == 0 {
            return n == p;
        }
        i += 1;
    }
    // n is odd, > 37, with no factor among the bases. Write n - 1 = d * 2^s, d odd.
    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    let mut b = 0;
    while b < BASES.len() {
        let a = BASES[b];
        let mut x = pow_mod(a, d, n);
        if x != 1 && x != n - 1 {
            let mut witnessed_composite = true;
            let mut r = 1;
            while r < s {
                x = mul_mod(x, x, n);
                if x == n - 1 {
                    witnessed_composite = false;
                    break;
                }
                r += 1;
            }
            if witnessed_composite {
                return false;
            }
        }
        b += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trial_division(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let mut i = 2u64;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    #[test]
    fn agrees_with_trial_division_below_200_000() {
        for n in 0u64..200_000 {
            assert_eq!(is_prime(n), trial_division(n), "n = {n}");
        }
    }

    #[test]
    fn rejects_carmichael_and_strong_pseudoprimes() {
        // Carmichael numbers.
        for n in [
            561u64,
            41_041,
            825_265,
            321_197_185,
            5_394_826_801,
            232_250_619_601,
            9_746_347_772_161,
        ] {
            assert!(!is_prime(n), "{n} is composite");
        }
        // 3215031751 is a strong pseudoprime to bases 2, 3, 5 and 7.
        assert!(!is_prime(3_215_031_751));
        // 318665857834031151167461 is beyond u64; instead use a large semiprime below 2^63.
        let p: u64 = 2_147_483_647; // 2^31 - 1, prime
        let q: u64 = 2_147_483_629; // prime (2^31 - 19)
        assert!(is_prime(p) && is_prime(q));
        assert!(!is_prime(p * q));
    }

    #[test]
    fn accepts_known_large_primes() {
        assert!(is_prime(2_305_843_009_213_693_951)); // 2^61 - 1
        assert!(is_prime(9_223_372_036_854_775_783)); // 2^63 - 25, the largest prime below 2^63
        assert!(is_prime(4_294_967_197)); // 5 mod 8, near 2^32
        assert!(is_prime(2_147_483_629)); // 5 mod 8, near 2^31
        assert!(is_prime(18_446_744_073_709_551_557)); // 2^64 - 59, the largest u64 prime
        assert!(!is_prime(18_446_744_073_709_551_615)); // u64::MAX
    }

    #[test]
    fn pow_mod_matches_naive_for_small_inputs() {
        for m in 1u64..40 {
            for b in 0u64..40 {
                let mut naive = 1 % m;
                for e in 0u64..12 {
                    assert_eq!(pow_mod(b, e, m), naive, "b={b} e={e} m={m}");
                    naive = naive * (b % m) % m;
                }
            }
        }
    }
}
