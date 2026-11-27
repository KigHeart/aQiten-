//! Fiat-Shamir transcript and coordinate-indexed random oracle for aQiten.
//!
//! Everything is built on SHAKE256 ([`Shake256`]), implemented in this crate and
//! checked against an independent implementation.
//!
//! # What is here
//!
//! * [`Transcript`]: the verifier-visible record of a protocol run. The prover
//!   and verifier absorb the same public messages in the same order and obtain
//!   the same challenges. Every operation is length-framed and carries a label,
//!   so two different sequences of operations can never produce the same hash
//!   input (injective framing; see `transcript.rs`).
//! * [`Oracle`]: a random oracle indexed by coordinates `(i, j, ...)`, used to
//!   expand public setup (matrix entries) from a short seed without storing it.
//! * [`Ledger`]: exact counts of rounds, absorbed messages, challenges and
//!   Keccak permutations. The permutation count is the number of random-oracle
//!   queries an honest run makes; the QROM argument's loss terms depend on such
//!   counts and on the number of rounds (work item Q0), so they are recorded
//!   rather than estimated.
//!
//! # Design decisions
//!
//! * **Public data only.** A transcript must never absorb secret values; its
//!   state is public by construction and is cloneable. (Nothing here is
//!   constant time, deliberately.)
//! * **Every challenge ratchets the state.** A challenge is derived from a clone
//!   of the state, and a 32-byte digest of that clone is then absorbed into the
//!   main state, so later challenges depend on earlier ones.
//! * **Rejection sampling is bounded.** Uniform sampling below a bound draws at
//!   most 128 candidates (each accepted with probability > 1/2); exhaustion
//!   (probability < 2^-128) is an error, never an infinite loop or a biased value.
//! * **A prover retry must start a fresh transcript.** There is no rewind API.
//!
//! # Cross-crate role
//!
//! Consumes [`aqiten_field`] (`Fp` challenges), [`aqiten_codec`] (absorbing any
//! [`aqiten_codec::Encode`] value in its canonical form) and [`aqiten_error`].
//! Consumed by `aqiten-challenges` (samplers draw from a challenge stream),
//! `aqiten-sumcheck` (round challenges), `aqiten-fold` (mask/fold challenges,
//! rejection restarts), `aqiten-setup` (matrix expansion through [`Oracle`]),
//! `aqiten-schedule` (round ledger) and `aqiten-verifier`.

#![no_std]

extern crate alloc;

mod keccak;
mod ledger;
mod oracle;
mod sample;
mod transcript;

pub use keccak::{Shake256, Shake256Reader};
pub use ledger::Ledger;
pub use oracle::Oracle;
pub use sample::{sample_below, sample_field};
pub use transcript::Transcript;
