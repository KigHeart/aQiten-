# Crate 3 guide: aqiten-transcript

Rev. 0.1, 2026-11-27. Status: implemented; fmt, clippy -D warnings, debug+release tests, docs pass (`tools/ci-local.sh`).
Depends on: aqiten-error, aqiten-codec, aqiten-field. Independent model: `tools/transcript_model.py` (hashlib).

## Purpose
Fiat-Shamir for every aQiten round, and a coordinate-indexed random oracle for expanding public setup from a seed,
with an exact ledger of rounds, messages, challenges and Keccak permutations (the quantities the QROM loss terms depend on, work item Q0).

## Items
| Item | Meaning | Used by |
|---|---|---|
| `Shake256`, `Shake256Reader` | SHAKE256 written in full (Keccak-f[1600], FIPS 202), absorb-only and squeeze-only type-states, exact permutation counter | everything below; `aqiten-challenges` samplers |
| `Transcript::new(protocol)` | binds the run to a protocol/parameter-set label and version | prover, verifier |
| `absorb_bytes`, `absorb<T: Encode>` | labelled, length-framed absorption; `absorb` uses the canonical codec encoding | types, commit-*, sumcheck, fold |
| `begin_round(label)` | marks a round (fold level, sum-check round), counted in the ledger | schedule, sumcheck, fold |
| `challenge_bytes`, `challenge_below`, `challenge_field::<P>`, `challenge_with` | challenge derivation with ratchet; `challenge_with` gives a sampler one stream for many values | challenges (ring challenges), sumcheck, fold |
| `sample_below`, `sample_field` | uniform sampling by masking and bounded rejection (<= 128 draws, failure < 2^-128, never biased) | challenges, setup |
| `Oracle::new(seed, label)`, `field::<P>(coords)`, `bytes(coords, out)` | H(seed, label, i_1..i_n), order-independent, domain-separated; counts queries | setup (matrix expansion), commit-core |
| `Ledger` | rounds, absorbs, challenges, absorbed bytes, permutations | schedule (round ledger), Q0 measurement, bench |

## Normative framing
`frame(tag, label, data) = tag || u32le(|label|) || label || u64le(|data|) || data`; tags: 1 init, 2 absorb, 3 challenge request,
4 round, 5 ratchet, 6 oracle. A challenge runs on a clone of the state; 32 bytes of the clone are absorbed back (ratchet).
Because every field is length-prefixed the concatenation parses uniquely, so distinct operation sequences give distinct hash inputs.

## Evidence
* 3 unit + 8 integration tests. SHAKE256: NIST empty-message vector and six Python-computed vectors at lengths 1, 135, 136, 137, 272, 1000 (rate boundaries); chunked absorb and chunked squeeze equal one-shot.
* Transcript and oracle outputs equal the independent Python model (bytes, `below`, field challenge, oracle at several coordinates/seeds/labels).
* Behavioural: injective framing across label/data boundaries; order, label, protocol and round markers all change challenges; same label twice differs (ratchet); clone replays identically; shorter request is a prefix of a longer one; chi-square-style bounds on 20 000 field samples (P=5) and 12 000 samples below 3; ledger counts.
* Mutation check (6 injected bugs: ratchet removed, label length removed, sampler off-by-one, Keccak chi altered, wrong padding byte, oracle coordinate order reversed): each caught.

## Tags
* ESTABLISHED: SHAKE256 implementation correctness (byte-equal to hashlib on the vectors above); framing injectivity (argument above plus tests); transcript/oracle agree with the independent model.
* CONJECTURE: that the ratcheted-clone construction yields Fiat-Shamir challenges whose soundness reduces as intended in the QROM. This is the Q1/Q2 research item, not proved here.
* NOT claimed: constant time (transcripts are public data by design); no secret may be absorbed.
* LIMIT: the Keccak implementation is not independently audited and not tested beyond the vectors above.
