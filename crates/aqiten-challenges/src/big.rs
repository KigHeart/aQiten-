//! A tiny unsigned big integer: just enough for exact challenge-set cardinalities.

//! All arithmetic is on `u64` holding `u32 * u32 + carry` (at most `2^64 - 2^33 + 1`), so
//! it cannot overflow; the module opts out of the workspace arithmetic lint for that reason.
#![allow(clippy::arithmetic_side_effects)]

use alloc::vec::Vec;

use aqiten_error::{Error, Result};

/// Little-endian base-2^32 limbs, no trailing zero limb (zero is the empty vector).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Big {
    limbs: Vec<u32>,
}

impl Big {
    pub(crate) fn from_u32(x: u32) -> Self {
        let mut limbs = Vec::new();
        if x != 0 {
            limbs.push(x);
        }
        Self { limbs }
    }

    /// `self * m`.
    pub(crate) fn mul_small(&mut self, m: u32) {
        let mut carry: u64 = 0;
        for limb in &mut self.limbs {
            let v = u64::from(*limb) * u64::from(m) + carry;
            *limb = u32::try_from(v & 0xFFFF_FFFF).unwrap_or(0);
            carry = v >> 32;
        }
        if carry != 0 {
            self.limbs.push(u32::try_from(carry).unwrap_or(u32::MAX));
        }
        while self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    /// `self / m`, required to be exact.
    pub(crate) fn div_small_exact(&mut self, m: u32) -> Result<()> {
        if m == 0 {
            return Err(Error::InvalidParameter {
                what: "div_small_exact: division by zero",
            });
        }
        let mut rem: u64 = 0;
        for limb in self.limbs.iter_mut().rev() {
            let cur = (rem << 32) | u64::from(*limb);
            *limb = u32::try_from(cur / u64::from(m)).unwrap_or(0);
            rem = cur % u64::from(m);
        }
        while self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
        if rem == 0 {
            Ok(())
        } else {
            Err(Error::InvalidParameter {
                what: "div_small_exact: division was not exact",
            })
        }
    }

    /// `self * 2^k`.
    pub(crate) fn shl(&mut self, k: u32) {
        for _ in 0..k {
            self.mul_small(2);
        }
    }

    /// Number of bits of the value (`0` for zero), i.e. `floor(log2) + 1`.
    pub(crate) fn bit_length(&self) -> u64 {
        match self.limbs.last() {
            None => 0,
            Some(top) => {
                let full = (self.limbs.len() as u64 - 1) * 32;
                full + u64::from(32 - top.leading_zeros())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_values() {
        let mut b = Big::from_u32(1);
        assert_eq!(b.bit_length(), 1);
        b.shl(10);
        assert_eq!(b.bit_length(), 11);
        b.mul_small(3);
        assert_eq!(b.bit_length(), 12); // 3072
        assert_eq!(b.div_small_exact(1024), Ok(()));
        assert_eq!(b, Big::from_u32(3));
        assert!(b.div_small_exact(2).is_err());
        assert!(b.div_small_exact(0).is_err());
        assert_eq!(Big::from_u32(0).bit_length(), 0);
    }

    #[test]
    fn factorial_25_exact() {
        // 25! = 15511210043330985984000000, 84 bits
        let mut b = Big::from_u32(1);
        for k in 2..=25u32 {
            b.mul_small(k);
        }
        assert_eq!(b.bit_length(), 84);
        for k in (2..=25u32).rev() {
            assert_eq!(b.div_small_exact(k), Ok(()));
        }
        assert_eq!(b, Big::from_u32(1));
    }
}
