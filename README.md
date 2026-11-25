# aQiten

**Verifiable Agentic Computation Runtime**

aQiten is a research and engineering project for systems in which autonomous agents perform bounded work, exchange typed results, invoke formally specified computations, and return evidence that independent systems can check.

Its architecture separates four planes:

1. **Intelligence** — model-driven agents, planning, delegation and tool orchestration.
2. **Authority** — identities, scoped capabilities, deterministic policy decisions, approval and revocation.
3. **Computation and proof** — deterministic relations, lattice-based polynomial commitments, zero-knowledge proofs and independent verification.
4. **Evidence** — portable proof envelopes, provenance records, signed events and reproducible verification.

The runtime is designed around typed agent-result artifacts and deterministic BSP super-steps. Models may propose plans and actions; they do not grant themselves authority, decide whether a proof is valid, or directly execute protected side effects.

## What aQiten is building

- A durable, cost-aware, model-agnostic runtime for heterogeneous agents and deterministic workers.
- Typed, provenance-bearing artifacts that make one agent's result an explicit input to another.
- A BSP kernel with frozen frontiers, immutable node inputs, deterministic reducers, journals and checkpoints.
- A capability/policy authority layer and a sole protected-effect gateway.
- A formal computation interface for versioned relations.
- A post-quantum-oriented polynomial commitment and zero-knowledge research core, with a QROM security target.
- Independent verification and portable evidence services.

## Current status — research/specification stage

**There is no Rust workspace, agent runtime, SDK, proof verifier or production cryptographic implementation in this repository yet.** The architecture and specifications are target designs, not delivered functionality.

The proposed lattice-based ZK construction has unresolved mathematical obligations. Commitment augmentation, masked sum-check, fold masking, transcript-level simulation and QROM extraction are not established. Do not use this project to protect secrets, authorize real transactions or make production security claims.

Akita is a read-only mathematical and differential-testing reference, not a codebase being forked. The aQiten implementation is intended to be independent.

## Architecture and specifications

- [Normative agentic system specification](docs/AGENTIC_SYSTEM_SPEC.md) — actors, data model, BSP runtime, authority, effects, proof, evidence, crates and security invariants.
- [Complete compute lifecycle](docs/COMPUTE_LIFECYCLE.md) — request-to-evidence lifecycle, QALABASH example, sequence and recovery matrix.
- [AJENTIR runtime adaptation](docs/AJENTIR_INTEGRATION.md) — BSP, task artifacts, checkpoints, cost ledger and effect-journal principles.
- [Agentic product landscape](docs/PRODUCT_LANDSCAPE.md) — category-by-category product review and aQiten positioning.
- [Product architecture](docs/PLATFORM_ARCHITECTURE.md) — four planes and trust boundaries.
- [Platform trust/dependency diagram](docs/PLATFORM_DAG.md) — agent, authority, proof and evidence flow.
- [Rust workspace tree](docs/TREE.md) — target crates, dependency rules and build sequence.
- [Agent security model](docs/AGENTIC_SECURITY_MODEL.md) — identity, capabilities, policy, delegation and side-effect controls.
- [Competitive positioning](docs/COMPETITIVE_POSITIONING.md) — PCS, zkVM, folding, agent-runtime and authenticated-workflow comparisons.
- [Product roadmap](docs/ROADMAP.md) — staged research, engineering and integration gates.
- [Project charter](CHARTER.md) — security goals and epistemic rules.
- [P0 audit plan](docs/P0_PLAN.md) — outstanding Akita audit and construction questions.
- [Architecture review](docs/ARCHITECTURE_REVIEW.md) — prior cryptographic crate-boundary decisions.
- [Original cryptographic dependency diagram](docs/crate_dag.png) — crypto-core DAG only, not the full platform.
- [Akita research reference](papers/akita.pdf) — retained for research comparison.

## Non-negotiable trust boundaries

- Agent/model code does not depend into the cryptographic verifier.
- The verifier does not import LLMs, agent runtimes, memory stores, tool adapters or orchestrators.
- All protected side effects pass through a deterministic effect gateway.
- Capability checks are independent of model judgement and fail closed.
- Delegation may attenuate authority; it may not expand authority.
- Proof validity, execution attestation, identity, provenance and policy approval are distinct claims.
- A proof establishes only the specified relation over its public inputs; it does not establish source-data truth or business-policy correctness.
- Runtime state determinism is not cryptographic proof soundness.
- Research harnesses, attack tools, simulators and benchmarks never enter production dependency paths.

## Initial demonstrator

The first end-to-end target is a private financial computation based on the QALABASH R_spend relation, using synthetic data and a mock settlement connector:

1. Authenticate caller and create a durable run.
2. Planner proposes a typed graph; deterministic validation admits it.
3. Authority grants bounded capabilities to specific nodes.
4. Specialist agents emit typed artifacts; downstream agents consume and challenge them.
5. A deterministic evaluator executes the approved relation.
6. Prover generates an experimental proof; an independent verifier checks it.
7. Effect gateway separately authorizes and journals the mock settlement.
8. Runtime commits a checkpoint and exports portable evidence.

False statements, wrong relation versions, unauthorized effects, replay and altered evidence must be rejected.

## Competitive position

aQiten is not currently a peer of production zkVMs, deployed agent frameworks or established cryptographic libraries. It currently has research documents and architecture specifications, not a working runtime.

The intended differentiation is the integration of typed multi-agent computation, durable deterministic execution, scoped agent authority, proof-carrying computational claims and independently verifiable evidence. The product landscape explains where aQiten should integrate rather than rebuild.

## Development discipline

Specification before implementation. Mathematical proof before security claims. Adversarial tests before security-sensitive code. One crate at a time. No placeholder APIs presented as working features. Every security claim is labelled **ESTABLISHED**, **CONJECTURE** or **OPEN PROBLEM**, with evidence and failure conditions.

Maintained by **Tilil Engineering Group, the aQiten Team**.

Licensed under Apache-2.0 OR MIT.
