# P0: audit, formalisation, and construction review

P0 is **partially complete, not closed**. The reports capture the Akita sections read so far and a proposed construction, but source reading and citation verification remain incomplete. P1 cryptographic implementation is gated on the mathematical and architecture questions below.

This plan governs the **cryptographic research track**. The broader product architecture is documented separately in `docs/PLATFORM_ARCHITECTURE.md`, `docs/AGENTIC_SECURITY_MODEL.md`, `docs/COMPETITIVE_POSITIONING.md` and `docs/ROADMAP.md`. Agentic product work must not be used to bypass unresolved cryptographic gates.

## Inputs

- papers/akita.pdf (Akita paper, ePrint 2026/1983).
- Akita repository as a read-only reference.
- Primary literature on lattice commitments, lattice zero knowledge, Fiat-Shamir in the QROM, and quantum extraction.

## Deliverables and status

1. Message-by-message leakage audit: spec/aqt001_p0_audit.tex (TR-001); partial, with unread sections listed in the report.
2. Security games G1-G3: initial formalisation in TR-001; review against the complete protocol and final construction remains required.
3. Citation verification: incomplete; theorem statements and reductions must be checked against primary sources.
4. Construction proposal: spec/aqt002_construction_plan.tex (TR-002); a plan, not a proved construction.
5. Independent mathematical review: tracked in PR #1 (not yet merged).
6. Ring architecture gate: tracked in PR #1 (not yet merged). Exact reference arithmetic is required; NTT support is conditional on a verified root-of-unity certificate.
7. Architecture review: docs/ARCHITECTURE_REVIEW.md; reconciles the cryptographic crate tree and dependency graph, including verifier isolation and test-only crates.
8. Product expansion: charter rev. 0.2 defines agentic goals A1-A5; product architecture and threat model are specified in the linked platform documents. These are design specifications, not implementations.

## Akita sections still to read

As recorded in TR-001/TR-002: A Sec. 3-4, 9, 11-13, Appendices A and B, and the remaining portions of C, E, and G. Reconcile this list as sections are completed; do not claim the audit is complete prematurely.

## Gate to P1

- Select a coherent ring architecture; do not combine q = 5 mod 8 with an assumed full base-field negacyclic NTT at power-of-two degree d >= 4.
- Extract Akita's concrete schedule tuple (q,d,n,m) at every level.
- Specify augmented commitment and fold vector layouts with exact dimensions.
- Complete commitment, revealed-functional, and masked-sum-check definitions.
- Define the full rejection/retry transcript and prove its joint distribution.
- Produce a reviewed simulator proof and parameter inequalities, or mark the construction unresolved and choose a different design.
- Enforce a verifier dependency boundary that excludes prover-only code.
- Keep simulator, attacks, Akita oracle, and benchmark harnesses out of production dependency paths.

## Build order

The proposed cryptographic tree is in docs/TREE.md. Begin with error/codec/field, then exact reference ring arithmetic, estimator, transcript/challenges, parameter and schedule validators, and only then commitment. Masked sum-check and fold are blocked on their mathematical proof obligations.

## Status snapshot — 2026-11-25

- TR-001 and TR-002 are committed.
- Box-rejection finite-domain checks exist on PR #1's separate branch; they are not on main.
- main contains the proposed cryptographic crate tree and visual dependency map.
- The product architecture now includes separate agentic authority, runtime, proof/evidence and adapter boundaries.
- No Rust crates or cryptographic implementation exist on main.
- No agent runtime, capability service, effect gateway or proof SDK exists.
- No zero-knowledge, MLWE/MSIS parameter, classical-ROM composition, or QROM security claim is established.
