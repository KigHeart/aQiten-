# Crate 4 guide: aqiten-challenges

Rev. 0.1, 2026-11-27. Status: implemented; fmt, clippy -D warnings, debug+release tests, docs pass (`tools/ci-local.sh`).
Depends on: aqiten-error, aqiten-field, aqiten-ring, aqiten-transcript.

## The mathematics (proved in `split.rs` docs, verified exhaustively on small rings)
For a prime q = 5 mod 8 and D a power of two: 2 is a non-residue, i = 2^((q-1)/4) has order 4, X^m - i and X^m + i (m = D/2) are
irreducible (Lidl-Niederreiter Thm 3.75), so R_q = F_{q^m} x F_{q^m}, two fields. Hence:
1. **Exact unit test**: c is a unit iff NOT(c_j = +i c_{j+m} for all j) and NOT(c_j = -i c_{j+m} for all j).
2. **Lemma**: nonzero delta with ||delta||_inf < sqrt(q/2) is a unit (delta_j^2 + delta_{j+m}^2 = 0 mod q and < q forces 0).
3. **Certified difference invertibility**: two members of a sparse family differ by at most 2 c_max in infinity norm, so 2(2 c_max)^2 < q certifies that every nonzero challenge difference is invertible. This is the s_split = 2 case of the Lyubashevsky-Seiler bound Akita cites, proved here from scratch for this modulus class.

This is also *why* q = 5 mod 8: it is the class with exactly two factors, which makes the invertibility argument elementary and exact.
(Consequence for the ring crate: no full negacyclic NTT exists for these q, review point R1; reference multiplication is the backend.)

## Items
| Item | Meaning | Used by |
|---|---|---|
| `Split::<P>::sqrt_minus_one()` | canonical i (smaller of ±2^((P-1)/4)); compile error unless P = 5 mod 8 | tests, certificates |
| `Split::<P>::is_unit(&c)` | exact invertibility test | extraction harness (aqiten-sim), verifier sanity checks |
| `Split::<P>::small_elements_are_units(b)` | 2 b^2 < P | params I6 |
| `SparseFamily::new(D, &[(a, n_a)])` | validated Akita family; canonical order | params, schedule |
| `.c_max()`, `.omega()`, `.support_size()` | exact norms common to all members (linf = c_max, l1 = omega) | params (I1, I3), fold |
| `.cardinality_log2_floor()`, `.cardinality_at_least_two_pow(t)` | exact |C| = D!/(D-s)!/prod n_a! * 2^s by big-integer arithmetic, no floating point | schedule (error budget W/|C|) |
| `.differences_are_units::<P>()` | certificate 3 above | params I6 |
| `sample_sparse(stream, &family)` | uniform sampler: partial Fisher-Yates with bounded rejection, then sign bits | via transcript |
| `challenge_sparse(&mut transcript, label, &family)` | one transcript challenge -> one member | fold, sumcheck, verifier |
| `op_norm_sq_upper_bound(&c)` | exact integer B >= ||c||_mul,2^2 (Gershgorin on G = M^T M; all rows have the same absolute sum) | params I3/I4 |
| `op_norm_sq_certified_at_most(&c, gamma_sq)` | acceptance implies the true norm bound; rejection proves nothing | fold (filtered families) |

## Evidence
* Unit predicate vs "has a multiplicative inverse", every pair: (q,D) in {(5,2),(13,2),(29,2),(37,2),(5,4)}; for (13,4) and (5,8) the exact unit count (q^m-1)^2 for all elements (which also confirms irreducibility) plus inverse / zero-divisor checks on samples.
* Lemma tightness: for q in {5,13,29,37}, D=2, the smallest norm of a nonzero non-unit b* satisfies 2 b*^2 >= q and b*^2 <= q.
* Akita Table 3: exact floor(log2 |C|) = 128, 129, 131, 132, 131, 131 for d = 64..2048, matching the table (128.09, 129.62, 131.08, 132.76, 131.58, 131.59); big-integer, no floats.
* Sampler: every sample has exactly 31 magnitude-1, 10 magnitude-2 coefficients (300 draws); on the 48-element family (D=4) all 48 members occur with frequencies within 800..1200 of 1000 over 48 000 draws; deterministic for a fixed transcript; parameter checks.
* Operator norm: bound >= true max_k |c(zeta_k)|^2 (f64 DFT reference, 100 sparse challenges), ||c||_2^2 <= B <= omega^2, exact on constants and monomials, row-sum claim checked on full G.
* Mutation: 7 injected bugs; 6 caught; the 7th (`<` to `<=` in the threshold) is an equivalent mutant (2 b^2 = q is impossible for odd prime q).

## Tags
* ESTABLISHED: items 1-3 above (proof + exhaustive verification); cardinality exactness; sampler uniformity argument; soundness of the Gershgorin bound.
* CONJECTURE: none new.
* OPEN: (a) the tight certified operator-norm predicate of Akita Theorem D.1 (needs its fixed-point certificate); (b) a certified lower bound on |accepted subfamily|; (c) the packing-subring variant (challenge subring S embedded by Y -> X^(kh)); (d) sampler uniformity is argued and tested, not machine-verified. All belong to parameter work P1.
* LIMIT: the loose bound admits fewer challenges than Akita's tight predicate; using it costs challenge-set size, never soundness.
