# Crate group 1 guide: aqiten-error, aqiten-codec, aqiten-field

Rev. 0.1, 2026-11-26. Status: implemented, tested, locally validated (`tools/ci-local.sh`).
Rule applied to all three: no `unsafe`, no `unwrap`/`expect`/`panic!`/indexing in library code,
every public item documented, `#![no_std]` declared (bare-metal build not verified: target unavailable here).

## Dependency order
```
aqiten-error  <-  aqiten-codec  <-  aqiten-field
```
`field` uses `codec` (encoding) and `error`; `codec` uses `error`; `error` uses nothing.

## aqiten-error

| Item | What it does | Who will use it |
|---|---|---|
| `Error` (non_exhaustive enum) | The one failure type of the workspace: `Overflow`, `InvalidParameter`, `NonCanonical`, `Truncated`, `TrailingBytes`, `LengthTooLarge`, `OutOfRange`, `NotInvertible`. Carries only static labels and lengths, never secrets. | every crate returns `Result<_, Error>`; `verifier` turns any `Err` into "reject" |
| `Result<T>` | alias | all |
| `arith::add_u64 / mul_u64 / sub_u64 / add_usize / mul_usize` | checked arithmetic; overflow is an error, never a wrap | `params` (sizes, norm bounds, ledger), `schedule` (`Tree(sch)` product), `types` (proof sizes) |
| `arith::usize_from_u64 / u64_from_usize` | explicit checked conversions | `codec` (length prefixes), `types` |
| `arith::ceil_div_u64` | `ceil(a/b)` without the overflowing `(a+b-1)/b` | `params` (block counts, digit counts) |
| `arith::pow2_u64 / next_pow2_u64` | `2^k`, next power of two | `ring` (degree checks), `sumcheck` (hypercube sizes), `schedule` |
| `arith::ceil_log2_u64` | `ceil(log2 x)` | `params` (digits needed for a bound, number of sum-check variables) |

## aqiten-codec

Contract: one canonical encoding per value; `decode(b)=Ok(v)` implies `encode(v)=b`; decoders never panic and never
allocate more than the input justifies.

| Item | What it does | Who will use it |
|---|---|---|
| `Reader::new/remaining/take` | bounds-checked cursor (`split_at_checked`); a failed `take` consumes nothing | `types`, `transcript`, all `decode` impls |
| `Reader::read_u8/u16/u32/u64` | little-endian fixed-width integers | wire types |
| `Reader::read_bool` | accepts only 0 or 1 | wire types |
| `Reader::read_array::<N>` | fixed-size byte arrays (digests, seeds) | `transcript`, `types` |
| `Reader::read_len(max)` | reads a `u32` length and checks it against `max` **before** any allocation | `decode_vec` and every variable-length field |
| `Reader::finish` | rejects trailing bytes | `decode_exact` |
| `Writer::put_*`, `put_len`, `put_bytes`, `into_bytes` | canonical output; length prefix is checked to fit `u32` | `types`, `transcript` |
| `Encode`, `Decode` traits | the contract above | `Fp` now; `Ring`, proof types, setup artifacts later |
| `encode_to_vec`, `decode_exact` | whole-value helpers; `decode_exact` is the verifier's entry point for untrusted bytes | `verifier`, tests, `fuzz/` |
| `encode_slice`, `decode_vec(r, max_len)` | length-prefixed sequences with a mandatory bound; initial capacity capped by unread bytes so a lying prefix cannot force a large allocation | `types` (proof vectors), `schedule` artifacts |
| impls for `u8,u16,u32,u64,bool,[u8;N]` | primitives | everything |

## aqiten-field

| Item | What it does | Who will use it |
|---|---|---|
| `is_prime(n)` (`const fn`) | deterministic Miller-Rabin for every `u64` (12 bases) | `Fp` at compile time; `params` may re-use it to validate moduli |
| `Fp<const P: u64>` | element of Z/P, stored in Montgomery form (R = 2^64), always fully reduced so equality of words is equality of elements. **A non-prime, even, <3 or >= 2^63 modulus fails to compile.** | `ring` (coefficients), `transcript` (challenges), `sumcheck` (evaluations), `commit-*` (matrix entries) |
| `Fp::from_canonical(x)` | `x` in `[0,P)` -> element; rejects `x >= P` | `Decode` |
| `Fp::to_canonical()` | element -> representative in `[0,P)` | `Encode`, hashing into the transcript, tests |
| `Fp::from_u64_reduced`, `Field::from_u64` | any `u64` -> residue (no division) | `ring` (small constants), `challenges` |
| `Fp::from_i64` | signed -> residue, no branch on the sign | `ring` (balanced digits are signed) |
| `Fp::to_centered_i64()` | element -> representative in `[-(P-1)/2, (P-1)/2]`, no branch on the value | `ring` (exact integer norms), `fold`/`params` (response bounds, box rejection) |
| `+ - * -x` and `*Assign` | branch-free modular arithmetic (see bounds in `fp.rs`) | all numeric crates |
| `Field::pow(exp)` | fixed 64-step ladder; sequence independent of exponent bits | `inv`, `sumcheck` |
| `Field::inv()` | Fermat inverse; `NotInvertible` on zero | `sumcheck` (Lagrange interpolation), `challenges` (invertibility of differences, later in the ring) |
| `Fp::ct_eq_mask`, `ct_select` | mask-based equality and selection | `fold` (rejection decision logic), `ring` |
| `Encode`/`Decode for Fp` | `ceil(bitlen(P-1)/8)` little-endian bytes; rejects `>= P` | `transcript` (absorb), `types` (proof elements) |
| `Field` trait | the interface `ring`, `transcript`, `sumcheck` are written against: `ZERO`, `ONE`, `CHARACTERISTIC`, `from_u64`, `is_zero`, `square`, `pow`, `inv`, plus ops and codec bounds | the whole numeric stack |

## What is NOT here, and why
* **Extension field**: its degree and base field come from the parameter work (P1); adding one now would fix an unmade choice.
* **NTT**: belongs to `aqiten-ring` and is optional; the architecture review (R1) shows a full negacyclic NTT is impossible for q = 5 mod 8 and d >= 4.
* **Zeroization of secrets**: not yet; decision pending (own `Drop` wipe vs external `zeroize`).

## Evidence
* `aqiten-error`: 6 unit tests (checked arithmetic, exhaustive `ceil_log2` for x < 4096).
* `aqiten-codec`: 9 unit tests (known answers, exhaustive 2-byte canonicality for `u16`, lying-length attack, no-panic prefix sweep).
* `aqiten-field`: 6 unit tests (constants, `is_prime` vs trial division below 200k, Carmichael numbers, 2^63-25, 2^64-59), 11 property/known-answer tests
  (exhaustive over all operand pairs for P in {3,5,7,13,97,257}; 20k random samples for five primes up to the largest allowed 2^63-25; independent Python known answers; signed and centered conversions; canonical encoding), 3 doc-tests (including two `compile_fail` for bad moduli).
* Mutation check (manual, 25 Nov): four deliberately injected bugs (unreduced add, wrong centered boundary, `pow` skipping the top bit, decode accepting `>= P`) were each caught by at least one test.

## Tags
* ESTABLISHED: functional correctness against references, as listed above.
* CONJECTURE: constant-time behaviour on real hardware (source is branch-free; compiled code and CPU not verified); `no_std` bare-metal build (not run).
* OPEN: none for this group; external review not done.
