# aQiten

**Verifiable Agentic Infrastructure**

aQiten is a research and engineering project for systems in which autonomous agents can perform bounded work, invoke verifiable computations, and return evidence that independent systems can check.

Its long-term architecture combines four planes:

1. **Intelligence** — model-driven agents, planning, delegation and tool orchestration.
2. **Authority** — identities, scoped capabilities, deterministic policy decisions, approval and revocation.
3. **Computation and proof** — deterministic relations, lattice-based polynomial commitments, zero-knowledge proofs and independent verification.
4. **Evidence** — portable proof envelopes, provenance records, signed events and reproducible verification.

The cryptographic layer is deliberately independent of the agent layer. Models may propose plans and actions; they do not grant themselves authority, decide whether a proof is valid, or directly execute protected side effects.

## What aQiten is building

- A post-quantum-oriented polynomial commitment and zero-knowledge research core, with a QROM security target.
- A computation interface that turns explicitly specified relations into proof-system inputs.
- An agent runtime that coordinates specialised agents without making model output an authority mechanism.
- A capability and policy system that constrains tools, delegation, budgets, resources and side effects.
- Proof-carrying results that bind a public claim to a relation identifier, public inputs, proof, parameter profile and provenance.
- Independent verification and evidence services for organisations that do not want to trust the proof producer.

## Current status — research/specification stage

**There is no Rust workspace, agent runtime, SDK, proof verifier or production cryptographic implementation in this repository yet.** The architecture is a target design, not a delivered platform.

The proposed lattice-based ZK construction has unresolved mathematical obligations. In particular, commitment augmentation, masked sum-check, fold masking, transcript-level simulation and QROM extraction are not established. Do not use this project to protect secrets, authorise real transactions or make production security claims.

Akita is a read-only mathematical and differential-testing reference, not a codebase being forked. The aQiten implementation is intended to be independent.

## Architecture and design documents

- [Product architecture](docs/PLATFORM_ARCHITECTURE.md) — four planes, product boundaries and end-to-end workflow.
- [Platform trust/dependency diagram](docs/PLATFORM_DAG.md) — agent, authority, proof and evidence flow.
- [Rust workspace tree](docs/TREE.md) — target crates, dependency rules and build sequence.
- [Agent security model](docs/AGENTIC_SECURITY_MODEL.md) — identities, capabilities, policy, delegation and side-effect controls.
- [Competitive positioning](docs/COMPETITIVE_POSITIONING.md) — comparison with PCS, zkVM, folding and agent-runtime projects.
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
- A proof establishes only the specified relation over its public inputs; it does not establish that source data is truthful or that the relation expresses the right business policy.
- Research harnesses, attack tools, simulators and benchmarks never enter production dependency paths.

## Initial demonstrator

The first end-to-end target is a private financial computation based on the QALABASH `R_spend` relation:

1. An agent receives a bounded task and a scoped capability.
2. It constructs or selects a formally specified relation.
3. A deterministic computation/prover path produces a proof.
4. An independent verifier checks the proof and public statement.
5. A separate policy/effect gateway decides whether any requested action is authorised.
6. A proof/evidence envelope records exactly what was established and what was merely attested.

The first demonstrator must use test data and a non-production proof profile until the construction and implementation have passed independent review.

## Competitive position

aQiten is not currently a peer of production zkVMs, deployed agent frameworks or established cryptographic libraries. Those projects have working implementations and integrations; aQiten currently has research documents and an architectural proposal.

The intended differentiation is the integration of:
- lattice-oriented post-quantum proof research;
- zero-knowledge computational claims;
- cryptographically scoped agent authority;
- proof-carrying agent results; and
- independently verifiable operational evidence.

See [the competitive analysis](docs/COMPETITIVE_POSITIONING.md) for the comparison and limitations.

## Development discipline

Specification before implementation. Mathematical proof before security claims. Adversarial tests before security-sensitive code. One crate at a time. No placeholder APIs presented as working features. Every security claim is labelled **ESTABLISHED**, **CONJECTURE** or **OPEN PROBLEM**, with its evidence and failure conditions.

Maintained by **Tilil Engineering Group, the aQiten Team**.

Licensed under Apache-2.0 OR MIT.
