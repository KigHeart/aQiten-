//! The uniform sampler for [`SparseFamily`].

use alloc::vec::Vec;

use aqiten_error::{Error, Result};
use aqiten_ring::RingElem;
use aqiten_transcript::{sample_below, Shake256Reader, Transcript};

use crate::SparseFamily;

/// Draws one member of `family`, uniformly, from `stream`.
///
/// Algorithm (Akita Section 3.2): a partial Fisher-Yates shuffle of `0..D` picks
/// `s` distinct positions in uniformly random order (each index drawn exactly
/// uniformly by bounded rejection sampling); the first `n_{a_1}` positions get
/// magnitude `a_1`, the next `n_{a_2}` get `a_2`, and so on; then `s` independent
/// sign bits are read. An ordered `s`-tuple of positions is uniform, and each
/// (subset, partition) pair has exactly `prod n_a!` ordered preimages, so the
/// result is uniform over the family.
///
/// # Errors
/// [`Error::InvalidParameter`] if `family.degree() != D` or `c_max > (P-1)/2`
/// (so that the centered lift is exact); [`Error::OutOfRange`] on rejection-sampling
/// exhaustion (probability `< 2^-128` per draw).
#[allow(clippy::arithmetic_side_effects)] // D <= 2^16 and s <= D: all index arithmetic is tiny
pub fn sample_sparse<const P: u64, const D: usize>(
    stream: &mut Shake256Reader,
    family: &SparseFamily,
) -> Result<RingElem<P, D>> {
    if family.degree() != D {
        return Err(Error::InvalidParameter {
            what: "sample_sparse: family degree differs from ring degree",
        });
    }
    if u64::from(family.c_max()) > (P - 1) / 2 {
        return Err(Error::InvalidParameter {
            what: "sample_sparse: magnitude exceeds (P-1)/2",
        });
    }
    let s = family.support_size();
    let mut order: Vec<usize> = (0..D).collect();
    for i in 0..s {
        let span = u64::try_from(D - i).unwrap_or(u64::MAX);
        let j = i + usize::try_from(sample_below(stream, span)?).unwrap_or(0);
        order.swap(i, j);
    }
    let mut sign_bytes = alloc::vec![0u8; s.div_ceil(8)];
    stream.squeeze(&mut sign_bytes);

    let mut coeffs = [0i64; D];
    let mut next = 0usize;
    for &(magnitude, count) in family.groups() {
        for _ in 0..count {
            let position = order.get(next).copied().unwrap_or(0);
            let byte = sign_bytes.get(next / 8).copied().unwrap_or(0);
            let negative = (byte >> (next % 8)) & 1 == 1;
            let value = i64::from(magnitude);
            if let Some(slot) = coeffs.get_mut(position) {
                *slot = if negative { -value } else { value };
            }
            next += 1;
        }
    }
    RingElem::from_centered(&coeffs)
}

/// Issues one transcript challenge labelled `label` and samples a member of `family`.
///
/// # Errors
/// As [`sample_sparse`], plus transcript errors.
pub fn challenge_sparse<const P: u64, const D: usize>(
    transcript: &mut Transcript,
    label: &[u8],
    family: &SparseFamily,
) -> Result<RingElem<P, D>> {
    transcript.challenge_with(label, |s| sample_sparse::<P, D>(s, family))
}
