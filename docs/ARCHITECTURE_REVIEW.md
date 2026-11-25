# Architecture review: crate tree and dependency graph

**Review date:** 2026-11-25
**Reviewed inputs:** docs/TREE.md, docs/crate_dag.png, current main, TR-TEG-2026-AQT-001/-002, and the ring/NTT compatibility review on PR #1.

## 1. Current repository state

The latest main commit adds the crate tree, build order, property tree, and rendered dependency graph. The repository remains a specification/research repository: there is no Rust workspace or cryptographic implementation yet.

The diagram at docs/crate_dag.png is already present on main. It is a dependency map, not a runtime sequence diagram:
- Solid arrows represent crate dependency direction (consumer to dependency).
- Dotted arrows represent test harnesses targeting components.
- pcs is the orchestration boundary; prover, verifier, and setup are separate roles.
- The low-level dependency spine is ring -> field/codec/error, with transcript -> field/error.

The tree document is the normative textual explanation; the PNG is its visual index. Keep both in sync.

## 2. Required corrections

### R1 — Ring/NTT contradiction (blocking)

The tree currently describes aqiten-ring as providing an NTT while fixing prime q = 5 mod 8. A full negacyclic NTT of degree d over F_q requires a primitive 2d-th root, hence 2d divides q-1. For power-of-two d >= 4, this is impossible when q = 5 mod 8.

**Correction:** the ring crate must provide exact reference multiplication first. An NTT backend is optional and admitted only for a selected modulus/degree pair with a verified root certificate. Do not change the challenge modulus class without re-proving the challenge-set/invertibility arguments. See spec/aqt003_ring_architecture_gate.md on PR #1.

### R2 — Verification-only commitment boundary (blocking architecture decision)

The verifier must not pull witness-generation, randomness-sampling, masking, or secret-bearing code merely to verify a commitment. A comment saying commit(verify-only) is not sufficient if the crate dependency graph still compiles prover functionality.

Preferred split:
- aqiten-commit-core: public commitment parameters, public commitment encodings, canonical public validation types.
- aqiten-commit-prover: randomness generation and commitment creation; depends on core and ring.
- aqiten-commit-verifier: public opening/verification equations only; depends on core and ring.

If the project chooses a single aqiten-commit crate with feature gating instead, CI must prove that the verifier build has default features disabled and that no prover-only module or dependency is compiled. Separate crates are the safer default.

### R3 — Setup is not a prover dependency

Setup is a parameter/public-structure generation role. Its outputs should be public setup artifacts consumed by prover and verifier. Avoid a dependency from verifier into setup or from setup into prover. If setup uses secret trapdoors or toxic waste, their lifecycle and erasure must be specified; if setup is transparent, document reproducibility and domain separation.

### R4 — Test harnesses are non-production dependencies

aqiten-sim, aqiten-attacks, aqiten-akita-oracle, and aqiten-bench must remain dev/test-only. Dotted graph edges must never become production dependencies. The oracle must remain feature-gated and disabled by default.

### R5 — Estimator and parameter validator are distinct

The estimator is an offline model producing cost/security estimates. The parameter validator is a deterministic verifier of explicit inequalities and certificates. A parameter set must not be accepted merely because the estimator reports a target security level. The certificate should state assumptions, model version, parameter values, and each checked inequality separately.

### R6 — Types and codec boundaries

Wire types must be canonical and strict-decoded. The verifier should reject non-canonical encodings before algebraic verification. Keep proof wire types independent of prover internals; do not let aqiten-types import prover/fold/simulator code.

## 3. Proposed production dependency DAG

Arrows mean "depends on"; they do not mean runtime call order.

    aqiten-pcs
      |-- aqiten-prover --> aqiten-fold --> aqiten-commit-prover
      |        |                 |          aqiten-sumcheck
      |        |                 |          aqiten-challenges
      |        |                 |          aqiten-transcript
      |        |                 |          aqiten-schedule
      |        |                 |          aqiten-params
      |        |                 |          aqiten-types
      |        |-- aqiten-types
      |        |-- aqiten-schedule
      |        '-- aqiten-params
      |
      |-- aqiten-verifier --> aqiten-commit-verifier
      |        |               aqiten-sumcheck
      |        |               aqiten-transcript
      |        |               aqiten-types
      |        '--              aqiten-params
      |
      '-- aqiten-setup --> aqiten-commit-prover (only if setup creates commitments)
               |          aqiten-types
               '--        aqiten-params

    aqiten-commit-{prover,verifier} --> aqiten-commit-core --> aqiten-ring
    aqiten-sumcheck --> aqiten-ring, aqiten-field, aqiten-transcript
    aqiten-challenges --> aqiten-ring, aqiten-transcript
    aqiten-schedule --> aqiten-params, aqiten-challenges
    aqiten-params --> aqiten-estimator, aqiten-ring, aqiten-challenges
    aqiten-transcript --> aqiten-field, aqiten-error
    aqiten-ring --> aqiten-field, aqiten-codec, aqiten-error
    aqiten-field --> aqiten-error

TEST ONLY (never production dependencies):
  aqiten-sim, aqiten-attacks, aqiten-akita-oracle, aqiten-bench

This is a target graph, not a claim that the current PNG contains every edge. In particular, the commitment split is a proposed strengthening of the existing commit(verify-only) annotation.

## 4. Test obligations attached to the graph

| Harness | Must test | Must not imply |
|---|---|---|
| Akita oracle | differential correctness in transparent mode | ZK or QROM security |
| attacks | hiding/binding failures on deliberately weakened parameters; attack resistance is empirical evidence only | a proof of MLWE/MSIS security |
| simulator | transcript distribution against a specified simulator and adversarial distinguisher tests | a theorem without the reduction |
| schedule/parameter validator | reject malformed dimensions, violated inequalities, invalid root certificates | security from estimator output alone |
| verifier fuzzing | malformed, oversized, non-canonical and adversarial proof inputs | completeness of the proof system |

## 5. Build order adjustment

Retain the spec-first build order, with these gates:

1. error, codec, field: checked arithmetic, canonical decoding, field known-answer tests.
2. ring: exact negacyclic reference multiplication, balanced-digit round trips, norm bounds; optional certified NTT only.
3. estimator: reproduce reference instances and document model assumptions.
4. transcript, challenges: coordinate-indexed domain separation, query ledger, challenge-before-commit tests.
5. params, schedule: deterministic certificate generation and independent rejection tests.
6. commitment core + prover/verifier split: formal equations and adversarial harness before implementation.
7. sum-check and fold only after the augmented-vector maps and masking proofs are dimension-complete.
8. wire types, setup, prover, verifier, then PCS orchestration.
9. simulator, attacks, oracle and benchmarks remain test/dev packages.

Do not start masked fold or masked sum-check implementation merely because their crate boxes appear in the graph. The algebraic compatibility and transcript-level simulator obligations remain open.

## 6. Status

- docs/crate_dag.png is already committed on main; no duplicate image is needed.
- docs/TREE.md is the current architecture baseline, but needs the corrections above.
- The ring/NTT correction is established algebraically; commitment split and setup boundaries are architecture decisions to enforce in Cargo/CI.
- No Rust code exists yet. No ZK, MLWE/MSIS parameter, classical-ROM composition, or QROM security claim is established.
