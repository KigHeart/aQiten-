//! A coordinate-indexed random oracle for expanding public setup from a seed.
//!
//! `H(seed, label, i_1, ..., i_n)`: the value at a coordinate depends only on
//! `(seed, label, coordinates)`, never on the order of queries, so a prover and
//! a verifier can regenerate any matrix entry independently.
//!
//! Framing: `frame(6, label, seed)` once at construction, then per query
//! `u32le(n) || u64le(i_1) || ... || u64le(i_n)` on a clone of that prefix.

use aqiten_error::{arith, Error, Result};
use aqiten_field::Fp;

use crate::sample::sample_field;
use crate::transcript::absorb_frame;
use crate::{Shake256, Shake256Reader};

const TAG_ORACLE: u8 = 6;

/// A random oracle bound to a seed and a domain label, with a query counter.
#[derive(Clone)]
pub struct Oracle {
    prefix: Shake256,
    queries: u64,
    permutations: u64,
}

impl Oracle {
    /// Creates the oracle for `seed` under the domain `label`.
    ///
    /// # Errors
    /// [`Error::LengthTooLarge`] on absurd lengths.
    pub fn new(seed: &[u8], label: &[u8]) -> Result<Self> {
        let mut prefix = Shake256::new();
        absorb_frame(&mut prefix, TAG_ORACLE, label, seed)?;
        Ok(Self {
            prefix,
            queries: 0,
            permutations: 0,
        })
    }

    fn stream(&mut self, coords: &[u64]) -> Result<Shake256Reader> {
        let n = u32::try_from(coords.len()).map_err(|_| Error::LengthTooLarge {
            declared: u64::try_from(coords.len()).unwrap_or(u64::MAX),
            max: u64::from(u32::MAX),
        })?;
        let mut s = self.prefix.clone();
        s.absorb(&n.to_le_bytes());
        for c in coords {
            s.absorb(&c.to_le_bytes());
        }
        self.queries = arith::add_u64(self.queries, 1, "oracle.queries")?;
        Ok(s.finalize())
    }

    /// Fills `out` with the oracle's output at `coords`.
    ///
    /// # Errors
    /// [`Error::LengthTooLarge`] / [`Error::Overflow`].
    pub fn bytes(&mut self, coords: &[u64], out: &mut [u8]) -> Result<()> {
        let mut r = self.stream(coords)?;
        r.squeeze(out);
        self.permutations = arith::add_u64(
            self.permutations,
            r.permutations().saturating_sub(self.prefix.permutations()),
            "oracle.permutations",
        )?;
        Ok(())
    }

    /// The uniform field element at `coords`.
    ///
    /// # Errors
    /// As [`Self::bytes`], plus sampling exhaustion.
    pub fn field<const P: u64>(&mut self, coords: &[u64]) -> Result<Fp<P>> {
        let mut r = self.stream(coords)?;
        let v = sample_field::<P>(&mut r)?;
        self.permutations = arith::add_u64(
            self.permutations,
            r.permutations().saturating_sub(self.prefix.permutations()),
            "oracle.permutations",
        )?;
        Ok(v)
    }

    /// Number of oracle queries made.
    #[must_use]
    pub fn queries(&self) -> u64 {
        self.queries
    }

    /// Keccak permutations spent on queries (excluding the one-time prefix).
    #[must_use]
    pub fn permutations(&self) -> u64 {
        self.permutations
    }
}
