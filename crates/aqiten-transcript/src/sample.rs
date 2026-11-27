//! Bounded rejection sampling from a SHAKE256 stream.

use aqiten_error::{Error, Result};
use aqiten_field::Fp;

use crate::Shake256Reader;

/// Maximum candidates drawn per sample. Each is accepted with probability
/// greater than 1/2, so exhaustion has probability below `2^-128`.
const MAX_ATTEMPTS: usize = 128;

/// A uniform value in `[0, bound)`, by masking to the bit length of `bound - 1`
/// and rejecting values `>= bound`.
///
/// # Errors
/// [`Error::InvalidParameter`] if `bound == 0`;
/// [`Error::OutOfRange`] if 128 consecutive candidates were rejected.
pub fn sample_below(stream: &mut Shake256Reader, bound: u64) -> Result<u64> {
    if bound == 0 {
        return Err(Error::InvalidParameter {
            what: "sample_below: bound must be positive",
        });
    }
    let top = bound.wrapping_sub(1);
    let bits = u64::BITS.wrapping_sub(top.leading_zeros());
    let mask = if bits >= u64::BITS {
        u64::MAX
    } else {
        (1u64 << bits).wrapping_sub(1)
    };
    for _ in 0..MAX_ATTEMPTS {
        let mut buf = [0u8; 8];
        stream.squeeze(&mut buf);
        let candidate = u64::from_le_bytes(buf) & mask;
        if candidate < bound {
            return Ok(candidate);
        }
    }
    Err(Error::OutOfRange {
        what: "sample_below: rejection sampling exhausted",
    })
}

/// A uniform element of `Fp<P>`.
///
/// # Errors
/// As [`sample_below`] (never `InvalidParameter`, since `P >= 3`).
pub fn sample_field<const P: u64>(stream: &mut Shake256Reader) -> Result<Fp<P>> {
    Fp::<P>::from_canonical(sample_below(stream, Fp::<P>::MODULUS)?)
}
