# aQiten charter (rev. 0.2, 2026-11-25)

## 1. Product purpose

aQiten is being designed as **verifiable agentic infrastructure**: a system in which autonomous agents can perform bounded work, invoke formally specified computations, and produce evidence that independent parties can verify.

The product vision combines four planes:

1. Intelligence: model-driven planning, orchestration and delegation.
2. Authority: identity, scoped capabilities, deterministic policy, approval and revocation.
3. Computation and proof: deterministic relations, lattice-based polynomial commitments, zero-knowledge proofs and independent verification.
4. Evidence: proof envelopes, provenance, signed events and reproducible verification.

The cryptographic core must remain independently usable. Agentic features must never become a dependency of the cryptographic verifier.

## 2. Cryptographic research objective

Develop an independent multilinear polynomial commitment and sum-check framework targeting:
- post-quantum security under explicitly stated lattice assumptions;
- commitment hiding;
- binding and knowledge soundness;
- zero knowledge for the complete evaluation transcript; and
- a rigorous quantum random-oracle model (QROM) security argument.

The first application relation is the QALABASH payment relation `R_spend`. It is a research target, not an implemented or validated product.

## 3. Relation to Akita

Akita (LayerZero Labs) is a read-only mathematical and implementation reference. aQiten studies its lattice PCS and transparent-mode behaviour, while independently specifying and implementing any chosen construction. This repository is not a fork.

No claim that aQiten improves on Akita is justified until aQiten has a complete construction, independent implementation, reproducible benchmarks and external review.

## 4. Agentic-system objective

The agent layer coordinates work; it is not the root of trust.

- Models may propose tasks, plans, code, relation descriptions and actions.
- A deterministic policy engine decides whether a requested operation is permitted.
- A capability issuer grants bounded, expiring authority.
- Delegation attenuates scope, budget and lifetime; it never increases them.
- A deterministic effect gateway is the only route to protected side effects.
- Proof verification is performed by an independent verifier, never by the agent that produced the proof.
- Evidence records distinguish model assertions, signed attestations, execution records and cryptographic proofs.

The system must support human approval for configured high-impact actions. It must fail closed when authority, policy, proof validation or required evidence is unavailable.

## 5. Security goals

### Cryptographic goals

- **G1 — Binding / knowledge soundness in the QROM.** Extraction against quantum provers with quantum oracle access.
- **G2 — Hiding.** Commitments to distinct messages are computationally indistinguishable under a stated and parameterised lattice assumption.
- **G3 — Evaluation zero knowledge.** A simulator, given only public parameters, the statement and claimed evaluation, produces a view indistinguishable from the real complete proof transcript. This includes every commitment, sum-check message, fold message, retry and terminal check.

No partial mode may be advertised as zero knowledge. Component experiments must be labelled as such.

### Agentic-system goals

- **A1 — Authority integrity.** No model output can create, expand or self-approve a capability.
- **A2 — Action mediation.** Protected side effects pass through an independently enforced policy and capability check.
- **A3 — Delegation safety.** Child capabilities are no more powerful than their parents and are bounded by depth, time, resources and action scope.
- **A4 — Evidence integrity.** Events and proof envelopes are canonically encoded, authenticated where appropriate, replay-resistant and independently checkable.
- **A5 — Claim precision.** A proof, signature, execution attestation, policy decision and source-data assertion are never conflated.

## 6. Epistemic discipline

Every security claim carries one tag and justification:
- **ESTABLISHED** — supported by a complete proof, verified implementation property or cited external result within its stated scope.
- **CONJECTURE** — plausible but not established; include assumptions and failure modes.
- **OPEN PROBLEM** — unresolved and blocking any dependent claim.

Heuristic parameter estimates are not proofs. Successful tests are not proofs of cryptographic security. A valid signature does not prove that an action was authorised. A valid proof does not prove that the input data was truthful.

## 7. Engineering rules

1. Specification first, implementation second.
2. One crate at a time; each crate must compile and test independently.
3. No placeholder APIs represented as implemented functionality.
4. Adversarial tests precede security-sensitive implementation.
5. Verifier dependency isolation is enforced by CI.
6. Agent models and adapters are optional integrations, not mandatory core dependencies.
7. All protected effects are mediated by the effect gateway.
8. Canonical encoding, bounded resource use, replay protection and explicit error handling are required.
9. Every externally visible proof profile is versioned and binds its parameters and transcript rules.
10. Production claims require independent cryptographic and systems review.

## 8. Work packages

| WP | Scope |
|---|---|
| P0 | Complete Akita audit, literature review, formal games and construction review |
| P1 | Field/ring arithmetic, estimator, parameter certificates and commitment construction |
| P2 | Masked fold, zero-knowledge sum-check, extraction and simulator proofs |
| P3 | Classical-ROM composition theorem and prototype |
| P4 | QROM security track, with explicit quantum extraction/reduction route |
| P5 | Independent implementation, adversarial testing and benchmarks |
| P6 | External cryptographic review |
| A0 | Agent threat model, identity/capability model and effect-gateway specification |
| A1 | Deterministic policy, delegation attenuation and audit-event model |
| A2 | Agent runtime interfaces and model/MCP adapters behind strict boundaries |
| A3 | Proof-carrying results and portable evidence envelope |
| A4 | End-to-end demonstrator and independent verification service |
| A5 | Runtime hardening, external security review and deployment qualification |

