# aQiten tree (rev. 0.1, 2026-11-25)

Status: **plan, no code yet.** Derived from the public Akita repository
(LayerZero-Labs/akita, MIT OR Apache-2.0, read-only reference) and from
TR-TEG-2026-AQT-001/-002. Akita structure below is taken from its
`docs/crate-graph.md` and the crates' `Cargo.toml` path dependencies.

## 1. What Akita's repository looks like (reference)

17 crates, ~296k lines of Rust, Rust 1.99.1. No `akita-scheme` crate; orchestration lives in `akita-pcs`.
Verifier is kept apart from the prover so verifier consumers pull no prover code.

```
Layer 0  akita-error  akita-serialization  (jolt-field: external)
Layer 1  akita-algebra            -> error, serialization, field
         akita-transcript         -> error, field
Layer 2  akita-challenges         -> error, field, transcript
         akita-sumcheck           -> error, algebra, field, serialization, transcript
Layer 3  akita-params             -> error, algebra, challenges, serialization, sumcheck, transcript
Layer 4  akita-types              -> params, error, algebra, challenges, serialization, sumcheck, transcript
         akita-sis-estimator      -> params           (offline)
Layer 5  akita-schedules          -> error, challenges, params
         akita-planner            -> challenges, config, schedules, sis-estimator, params   (offline)
         akita-config             -> challenges, schedules, serialization, transcript, types, params
Layer 6  akita-verifier           -> algebra, challenges, config, sumcheck, transcript, types, params   (no prover code)
         akita-prover             -> algebra, challenges, config, sumcheck, transcript, types, params   (backend contracts)
         akita-cpu-backend        -> prover, verifier, config, types, params, ...   (88k lines, owns commitments/witness arithmetic)
         akita-setup              -> config, cpu-backend, types, params, challenges
Layer 7  akita-pcs                -> everything above (orchestration, examples, integration tests)
```
Also in the repo: `specs/` (design specs, one per feature), `book/` (mdBook incl.
`roadmap/zero-knowledge.md`), `docs/` (security posture, soundness audit,
verifier contract, panic audit), `fuzz/`, `fixtures/`, `deny.toml`.

Lessons we adopt: verifier isolated from prover; offline planner separate from
runtime; independent schedule validator; written verifier contract and panic
audit; spec per feature; fuzz targets; no compatibility shims.

## 2. aQiten crate tree

```
aqiten-            (workspace)
|- crates/
|  |- aqiten-error            L0  shared errors, checked integer formulas
|  |- aqiten-codec            L0  canonical serialization, strict decoding (reject non-canonical)
|  |- aqiten-field            L1  prime field q (q = 5 mod 8), extension E, constant-time ops
|  |- aqiten-ring             L1  R_q = F_q[X]/(X^d+1), NTT, balanced digits, exact integer norms
|  |- aqiten-estimator        L1  MSIS + MLWE cost models (offline, no protocol deps)
|  |- aqiten-transcript       L2  domain-separated, coordinate-indexed oracle queries, round ledger, query counter
|  |- aqiten-challenges       L2  challenge families, certified operator-norm predicate, bounded samplers
|  |- aqiten-params           L3  solves inequalities I1-I6; emits certificate; independent validator
|  |- aqiten-commit           L3  hiding two-tier commitment (A,B,D,F,H with randomness columns)
|  |- aqiten-sumcheck         L3  plain + masked sum-check (degree <= 3), fused row support
|  |- aqiten-fold             L4  masked fold: mask commitment, box rejection, extraction harness hooks
|  |- aqiten-schedule         L4  schedule artifacts, round ledger, Tree(sch) computation, validator
|  |- aqiten-types            L5  proof / setup / claim wire types
|  |- aqiten-verifier         L6  the single hidden-statement verifier (no prover deps)
|  |- aqiten-prover           L6  sequencing: fold levels, terminal, retry-restart rule
|  |- aqiten-setup            L6  setup expansion, setup-prefix commitments (offloading)
|  |- aqiten-pcs              L7  orchestration, one acceptance rule, examples
|  |- aqiten-sim              T   simulator + witness-extraction harness (test-only)
|  |- aqiten-attacks          T   lattice-reduction hiding/binding attack harness (test-only)
|  |- aqiten-akita-oracle     T   differential tests against Akita transparent mode (feature-gated, off by default)
|  `- aqiten-bench            T   sizes, times, security-bit report
|- spec/        TR-TEG-2026-AQT-nnn LaTeX reports (spec first, code second)
|- docs/        TREE.md, P0_PLAN.md, verifier contract, panic audit, validation log
|- papers/      reference papers (akita.pdf)
|- fuzz/        decode and verifier fuzz targets
`- tools/       ci-local.sh, estimator reproduction scripts
```

Dependency DAG (arrows point to what a crate depends on; the rendered dependency map is docs/crate_dag.png):

```
pcs --> prover --> fold --> {commit, sumcheck, challenges, transcript, schedule, params}
pcs --> verifier --> {types, sumcheck, transcript, commit-verifier, params}
pcs --> setup  --> {commit-prover (only if setup creates commitments), types, params}
types --> {params, ring, codec}
schedule --> {params, challenges}
params --> {estimator, ring, challenges}
commit-core --> {ring, params}   commit-prover --> commit-core   commit-verifier --> commit-core\nsumcheck --> {ring, field, transcript}
challenges --> {ring, transcript}   transcript --> {field, error}
ring --> {field, codec, error}
sim, attacks, akita-oracle, bench : test/dev-only, never in verifier path
```
Rule: `aqiten-verifier` must not depend on `aqiten-prover`, `aqiten-fold`, `aqiten-setup` or `aqiten-sim`
(enforced by a cargo-deny/CI check, as Akita does).

**Commitment boundary:** the preferred implementation splits `aqiten-commit` into `aqiten-commit-core`, `aqiten-commit-prover`, and `aqiten-commit-verifier`. The verifier must not compile randomness generation or witness-side commitment creation. If feature-gating is used instead, CI must build the verifier with default features disabled and inspect the dependency tree. See `docs/ARCHITECTURE_REVIEW.md`.\n\n**Ring constraint:** a full negacyclic NTT of degree `d` over `F_q` requires `2d | (q-1)`. The current `q = 5 mod 8` condition is incompatible with a full base-field NTT for power-of-two `d >= 4`. Therefore the ring's required backend is exact reference multiplication; an NTT is optional only with a verified root certificate and a separately approved parameter profile. Do not change `q` without re-proving the challenge-set arguments.\n\n## 3. Build order (one crate at a time, each complete and runnable)

| # | Crate(s) | Why this position | Gate before the next |
|---|----------|-------------------|----------------------|
| 1 | error, codec, field | no protocol knowledge | known-answer vectors, canonical-decoding tests |
| 2 | ring | everything lattice sits on it | slow-reference property tests, digit round trips |
| 3 | estimator | needed before any parameter is trusted | reproduces Akita's published instances; reports its model |
| 4 | transcript, challenges | D1/D6 live here | challenge-before-commit negative tests, certified predicate |
| 5 | params + schedule(validator part) | solves I1-I6 | validator rejects every adversarial parameter set |
| 6 | commit | hiding and binding together | attack harness fails at certified params, succeeds on weakened set |
| 7 | sumcheck (masked) | Lemma zksc | round-message distribution test |
| 8 | fold | box rejection, extraction harness | z' independent of witness; extractor recovers weak openings |
| 9 | types, verifier, prover, setup | assembly | one acceptance rule, no partial mode |
| 10 | pcs, sim, bench, akita-oracle | end to end | transparent-vs-hidden comparison; differential check vs Akita |

This refines the ten-crate list of TR-TEG-2026-AQT-002 Sec. 6 (adds error/codec/field and
splits types/verifier/prover/setup out of the single `aqiten-pcs`, following Akita's
verifier/prover separation). Open decision D-F: write `aqiten-field` ourselves
(recommended for a small, auditable prime field) or depend on an external field crate
as Akita does with `jolt-field`.

## 4. Property tree (what each security claim rests on)

Tags: E established, C conjecture, O open.

```
G3 evaluation zero knowledge (classical ROM)                            [O: needs Z6 hybrid proof]
|- G2 commitment hiding (MLWE at certified params)                      [E structure / C parameters]
|- box rejection => z' independent of witness                           [E  Lemma box]
|- every revealed functional masked                                     [C  Z3, vacuous under plain Schwartz-Zippel]
|- masked sum-check rounds uniform                                      [E rounds / C cubic fused row, Z4]
|- retry restarts mask, commitment, challenges                          [E  design rule D3]
`- simulator programs oracle after t_y                                  [C]
G1 knowledge soundness (classical ROM)                                  [E for Akita's own relation; C for masked variant]
|- extraction survives committed mask                                   [E  Lemma extract]
|- MSIS binding of A,B,D,F,H with randomness columns, masked radius     [C parameters]
|- extraction tree size polynomial for the schedule                     [E (Akita Thm 10.29)]
G1-Q knowledge soundness in the QROM                                    [O]
|- collapse-binding of the commitment maps                              [O]
|- quantum extraction for the multi-round tree (loss vs |Rounds|)       [O]
`- round reduction (fewer, larger rounds)                               [C]
G3-Q zero knowledge in the QROM                                         [C: adaptive reprogramming, min-entropy of t_y]
Parameters: I1 binding, I2 hiding, I3 no wrap-around, I4 rejection, I5 masking entropy, I6 challenge set
|- estimator model choice (quantum ADPS16 + stated alternatives)        [E as a model, not a proof]
```
