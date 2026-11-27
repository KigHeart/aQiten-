# Crate 2 guide: aqiten-ring

Rev. 0.1, 2026-11-27. Status: implemented; fmt, clippy -D warnings, debug+release tests, docs pass (`tools/ci-local.sh`).
Depends on: aqiten-error, aqiten-codec, aqiten-field. No other dependency.

## Correction note
The earlier tree (TREE.md, before review R1) said this crate provides an NTT for q = 5 mod 8. That was wrong: a full
negacyclic NTT of degree D over F_q needs 2D | q-1, impossible for q = 5 mod 8 and power-of-two D >= 4.
This crate therefore provides exact reference multiplication only. An NTT may be added later for a (q, D) pair
with a verified root certificate, differentially tested against this reference.

## Items
| Item | Meaning | Used by |
|---|---|---|
| `RingElem<P, D>` | element of Z_P[X]/(X^D+1); D power of two in 2..=2^16 and P prime < 2^63 checked at compile time | challenges, commit-*, fold, sumcheck |
| `ZERO`, `one`, `constant`, `from_coeffs`, `from_centered`, `coeffs`, `to_centered`, `is_zero` | construction and views (modular view and centered-integer view) | all |
| `+ - neg` | coefficient-wise, via `Fp` | all |
| `*` | negacyclic product `sum_i a_i (b X^i)`; exact for every prime and power-of-two D; D^2 field mults | commit (A*s), fold (c_i*s_i), challenges |
| `scale` | multiply by a field scalar | sumcheck-style combinations, tests |
| `mul_x`, `mul_monomial(k)` | signed rotation; X^(2D)=1; O(D) | challenges (monomial challenge sets), tests |
| `linf_norm`, `within_linf`, `l2_norm_squared`, `within_l2_squared`, `hamming_weight` | exact norms on centered representatives; never reduced, overflow is an error | params (I1, I3, I4), fold (box bounds), challenges |
| `decompose_i64`, `recompose_i64`, `digits_needed` | balanced base-2^k digits, 2 <= k <= 32, digits in [-2^(k-1), 2^(k-1)-1]; minimal digit count for a bound | params, commit |
| `decompose_balanced`, `recompose_balanced` | the same, coefficient-wise on ring elements; parts have linf <= 2^(k-1) | commit (Ajtai gadget), fold |
| `Encode`/`Decode`, `ENCODED_BYTES` | D coefficients, each canonical `Fp` encoding, no length prefix; rejects any coefficient >= P | types, transcript |

`k = 1` is rejected: its digit set {-1, 0} cannot represent any positive integer (found by the brute-force test).

## Evidence
* 15 unit tests (x^D = -1, monomial vs repeated shift for 40 exponents, known products, exhaustive digit ranges and
  exact digit counts for k = 2..4, ring round trip at P = 2147483629, codec canonicality).
* 6 integration tests against an independent i128 convolution reference: exhaustive all pairs for (P=5, D=2) and (P=3, D=4);
  randomized ring axioms (commutative, associative, distributive, identity) for P in {5, 257, 4294967197 (= 5 mod 8), 2^61-1, 2^63-25}
  and D up to 64; zero divisors present as expected.
* 2 compile_fail doctests (bad degree, non-prime modulus).
* A first run of the largest-modulus test failed because the *test reference* overflowed i128; fixed in the test, not the crate.

## Tags
* ESTABLISHED: arithmetic correctness as tested above.
* CONJECTURE: hardware constant time (control flow depends only on public D, k, len); not measured.
* OPEN: the norm predicates are not constant time; the fold rejection step needs its own branch-free comparison (aqiten-fold).
* NOT claimed: no security statement. Binding/hiding/extraction arguments live in later crates and in the specs.
