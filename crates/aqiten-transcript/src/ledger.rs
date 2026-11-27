//! The exact operation ledger of a transcript.

/// Counts of everything a transcript did.
///
/// `permutations` is the number of `Keccak-f[1600]` evaluations, i.e. the number
/// of random-oracle queries (at the permutation level) an honest run performs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ledger {
    /// Calls to `begin_round`.
    pub rounds: u64,
    /// Calls to `absorb_bytes` / `absorb` (not counting framing or round markers).
    pub absorbs: u64,
    /// Challenges issued.
    pub challenges: u64,
    /// Payload bytes absorbed through `absorb_bytes` / `absorb`.
    pub absorbed_bytes: u64,
    /// Keccak-f permutations executed, including those inside challenge clones.
    pub permutations: u64,
}
