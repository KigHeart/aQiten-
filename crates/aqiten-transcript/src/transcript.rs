//! The Fiat-Shamir transcript.
//!
//! # Framing (normative; the Python model in the tests implements exactly this)
//!
//! `frame(tag, label, data) = tag(1 byte) || u32le(len label) || label || u64le(len data) || data`.
//! Because each field is length-prefixed, the concatenation of frames is
//! uniquely parseable, so distinct operation sequences give distinct sponge inputs.
//!
//! | op | tag | label | data |
//! |---|---|---|---|
//! | `new` | 1 | `"aqiten.transcript.v1"` | protocol label |
//! | `absorb_bytes` | 2 | caller label | message |
//! | challenge request (on a clone) | 3 | caller label | `u64le(index of this challenge)` |
//! | `begin_round` | 4 | caller label | `u64le(round index)` |
//! | ratchet (on the main state) | 5 | caller label | 32 bytes squeezed from the clone |
//!
//! A challenge: clone the main sponge, absorb frame 3, finalize, squeeze 32
//! ratchet bytes, absorb frame 5 into the main sponge, then hand the rest of
//! the clone's stream to the sampler.

use aqiten_codec::{encode_to_vec, Encode};
use aqiten_error::{arith, Error, Result};
use aqiten_field::Fp;

use crate::sample::{sample_below, sample_field};
use crate::{Ledger, Shake256, Shake256Reader};

const TAG_INIT: u8 = 1;
const TAG_ABSORB: u8 = 2;
const TAG_CHALLENGE: u8 = 3;
const TAG_ROUND: u8 = 4;
const TAG_RATCHET: u8 = 5;
const VERSION_LABEL: &[u8] = b"aqiten.transcript.v1";

/// Absorbs `frame(tag, label, data)` into `sponge`.
pub(crate) fn absorb_frame(
    sponge: &mut Shake256,
    tag: u8,
    label: &[u8],
    data: &[u8],
) -> Result<()> {
    let label_len = u32::try_from(label.len()).map_err(|_| Error::LengthTooLarge {
        declared: u64::try_from(label.len()).unwrap_or(u64::MAX),
        max: u64::from(u32::MAX),
    })?;
    let data_len = arith::u64_from_usize(data.len(), "transcript: data length")?;
    sponge.absorb(&[tag]);
    sponge.absorb(&label_len.to_le_bytes());
    sponge.absorb(label);
    sponge.absorb(&data_len.to_le_bytes());
    sponge.absorb(data);
    Ok(())
}

/// The running transcript of a protocol execution. Public data only.
#[derive(Clone)]
pub struct Transcript {
    sponge: Shake256,
    ledger: Ledger,
    /// Permutations spent inside challenge clones beyond the main sponge's own count.
    clone_permutations: u64,
}

impl Transcript {
    /// Starts a transcript bound to `protocol` (a label naming the protocol,
    /// parameter set and version).
    ///
    /// # Errors
    /// [`Error::LengthTooLarge`] if the label exceeds `u32::MAX` bytes.
    pub fn new(protocol: &[u8]) -> Result<Self> {
        let mut sponge = Shake256::new();
        absorb_frame(&mut sponge, TAG_INIT, VERSION_LABEL, protocol)?;
        Ok(Self {
            sponge,
            ledger: Ledger::default(),
            clone_permutations: 0,
        })
    }

    /// Absorbs a labelled byte string.
    ///
    /// # Errors
    /// [`Error::LengthTooLarge`] / [`Error::Overflow`] on absurd lengths or counters.
    pub fn absorb_bytes(&mut self, label: &[u8], data: &[u8]) -> Result<()> {
        absorb_frame(&mut self.sponge, TAG_ABSORB, label, data)?;
        self.ledger.absorbs = arith::add_u64(self.ledger.absorbs, 1, "ledger.absorbs")?;
        self.ledger.absorbed_bytes = arith::add_u64(
            self.ledger.absorbed_bytes,
            arith::u64_from_usize(data.len(), "ledger.absorbed_bytes")?,
            "ledger.absorbed_bytes",
        )?;
        Ok(())
    }

    /// Absorbs the canonical encoding of `value`.
    ///
    /// # Errors
    /// Any encoding error, plus those of [`Self::absorb_bytes`].
    pub fn absorb<T: Encode + ?Sized>(&mut self, label: &[u8], value: &T) -> Result<()> {
        let bytes = encode_to_vec(value)?;
        self.absorb_bytes(label, &bytes)
    }

    /// Marks the start of a protocol round (one fold level, one sum-check
    /// round, ...) and counts it in the ledger.
    ///
    /// # Errors
    /// As [`Self::absorb_bytes`].
    pub fn begin_round(&mut self, label: &[u8]) -> Result<()> {
        absorb_frame(
            &mut self.sponge,
            TAG_ROUND,
            label,
            &self.ledger.rounds.to_le_bytes(),
        )?;
        self.ledger.rounds = arith::add_u64(self.ledger.rounds, 1, "ledger.rounds")?;
        Ok(())
    }

    /// The operation counts so far (permutations include those in challenge clones).
    #[must_use]
    pub fn ledger(&self) -> Ledger {
        let mut l = self.ledger;
        l.permutations = self
            .sponge
            .permutations()
            .saturating_add(self.clone_permutations);
        l
    }

    /// Runs the challenge procedure and applies `f` to the clone's stream.
    fn with_stream<T>(
        &mut self,
        label: &[u8],
        f: impl FnOnce(&mut Shake256Reader) -> Result<T>,
    ) -> Result<T> {
        let index = self.ledger.challenges;
        let base = self.sponge.permutations();
        let mut clone = self.sponge.clone();
        absorb_frame(&mut clone, TAG_CHALLENGE, label, &index.to_le_bytes())?;
        let mut stream = clone.finalize();
        let mut ratchet = [0u8; 32];
        stream.squeeze(&mut ratchet);
        absorb_frame(&mut self.sponge, TAG_RATCHET, label, &ratchet)?;
        self.ledger.challenges = arith::add_u64(index, 1, "ledger.challenges")?;
        let out = f(&mut stream)?;
        self.clone_permutations = arith::add_u64(
            self.clone_permutations,
            stream.permutations().saturating_sub(base),
            "ledger.permutations",
        )?;
        Ok(out)
    }

    /// Fills `out` with challenge bytes.
    ///
    /// # Errors
    /// As [`Self::absorb_bytes`].
    pub fn challenge_bytes(&mut self, label: &[u8], out: &mut [u8]) -> Result<()> {
        self.with_stream(label, |s| {
            s.squeeze(out);
            Ok(())
        })
    }

    /// A uniform challenge in `[0, bound)`.
    ///
    /// # Errors
    /// [`Error::InvalidParameter`] if `bound == 0`; sampling exhaustion; counters.
    pub fn challenge_below(&mut self, label: &[u8], bound: u64) -> Result<u64> {
        self.with_stream(label, |s| sample_below(s, bound))
    }

    /// A uniform field element challenge.
    ///
    /// # Errors
    /// Sampling exhaustion; counters.
    pub fn challenge_field<const P: u64>(&mut self, label: &[u8]) -> Result<Fp<P>> {
        self.with_stream(label, sample_field::<P>)
    }

    /// Issues one challenge and hands its stream to `f`, for samplers that need
    /// many values from a single challenge (e.g. a vector of field elements or
    /// ring challenges in `aqiten-challenges`).
    ///
    /// # Errors
    /// Whatever `f` returns, plus counters.
    pub fn challenge_with<T>(
        &mut self,
        label: &[u8],
        f: impl FnOnce(&mut Shake256Reader) -> Result<T>,
    ) -> Result<T> {
        self.with_stream(label, f)
    }
}
