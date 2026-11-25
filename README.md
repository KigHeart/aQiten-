# aQiten

An independent lattice polynomial-commitment research and engineering project:
the target is a **zero-knowledge** PCS with a defensible **quantum-random-oracle
(QROM)** security argument, building on ideas in the Akita scheme (Dao et al.,
IACR ePrint 2026/1983). This is an independent implementation, not a fork.

**Current status: research/specification stage. No cryptographic implementation
exists. Nothing in this repository may be used to protect secrets.** The
construction plan contains open proof obligations; it is not a security proof.

## Repository map

- `CHARTER.md` — goals, security games G1–G3, scope, and engineering rules.
- `docs/P0_PLAN.md` — P0 audit status and remaining source-reading work.
- `spec/aqt001_p0_audit.tex` / `.pdf` — initial Akita leakage audit and games.
- `spec/aqt002_construction_plan.tex` / `.pdf` — proposed masked construction
  and its explicitly open proof obligations.
- `spec/aqt002_independent_review.md` — blocking review questions before
  cryptographic implementation.
- `research/test_box_rejection.py` — exhaustive finite-domain check of one
  local masking identity; not a cryptographic test or proof.
- `papers/akita.pdf` — reference paper.

## Engineering discipline

1. Specification and adversarial tests precede cryptographic code.
2. Every claim is tagged ESTABLISHED, CONJECTURE, or OPEN PROBLEM.
3. No feature may be called zero knowledge unless the complete transcript is
   covered by a simulator argument.
4. No post-quantum/QROM security claim until the relevant reductions are proved.
5. Run the local research checks with:
   `python -m unittest research.test_box_rejection -v`.

Maintained by Tilil Engineering Group. Licensed under Apache-2.0 OR MIT.
Akita is a separate project of LayerZero Labs / a16z crypto (MIT OR Apache-2.0);
any code or text derived from it must be marked and attributed in `NOTICE`.
