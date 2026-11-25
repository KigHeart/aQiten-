# P0: audit, formalisation, and construction review

P0 is **partially complete, not closed**. The reports now in `spec/` capture
the sections of Akita read so far and a proposed construction, but the source
audit and citation verification remain incomplete. P1 cryptographic
implementation is gated on closing the blocking questions in
`spec/aqt002_independent_review.md`.

## Inputs

- `papers/akita.pdf` (168-page Akita paper, ePrint 2026/1983).
- Akita repository as a read-only reference.
- Primary literature on lattice commitments, lattice zero knowledge,
  Fiat–Shamir in the QROM, and quantum extraction.

## Deliverables and status

1. **Message-by-message leakage audit:** initial version in
   `spec/aqt001_p0_audit.tex` (TR-TEG-2026-AQT-001). Partial: the report
   records exactly which paper sections were read and which remain unread.
2. **Security games G1–G3:** initial formalisation in TR-001; requires review
   against the complete protocol and final construction.
3. **Citation verification:** not complete. Every theorem and reduction used
   in the commitment, masking, and QROM arguments must be checked against its
   primary source.
4. **Construction proposal:** `spec/aqt002_construction_plan.tex`
   (TR-TEG-2026-AQT-002). This is a research plan, not a proved construction.
5. **Independent review:** `spec/aqt002_independent_review.md` records the
   blocking mathematical questions and the order for resolving them.
6. **Executable local check:** `research/test_box_rejection.py` exhaustively
   checks the finite-domain box-rejection identity for small instances. It
   does not establish transcript-level zero knowledge.

## Akita sections still to read

As recorded in TR-001/TR-002: A§3–4, A§9, A§11–13, Appendices A and B, the
remaining portions of C, E, and G. Reconcile this list with the reports as
each section is completed; do not claim the audit is complete prematurely.

## Gate to P1

Before implementing `aqiten-ring` or any cryptographic primitive:

- resolve the ring modulus / NTT compatibility question;
- specify the augmented commitment and fold vector layouts with dimensions;
- finish the source reading and verify the concrete schedule catalogue;
- complete the commitment, revealed-functional, and masked-sum-check definitions;
- define the full rejection/retry transcript and prove its joint distribution;
- produce a reviewed simulator proof and parameter inequalities, or mark the
  relevant construction as unresolved and choose a different design.

The first P1 crate, when the gate is met, should begin with exact reference
arithmetic. An NTT is permitted only for parameter sets whose required
root-of-unity conditions are verified.

## Status snapshot — 2026-11-25

- TR-001 and TR-002 are committed.
- The local box-rejection identity passed 785 exhaustive fixed-witness
  distribution cases, together with the Bernoulli lower-bound and
  (Y=2NB) acceptance checks.
- No Rust crates, cryptographic implementation, or CI workflows exist yet.
- No zero-knowledge or QROM security claim is established for aQiten.
