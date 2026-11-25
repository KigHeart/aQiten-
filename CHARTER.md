# aQiten charter (rev. 0.1, 2026-11-25)

## 1. Purpose
Provide a multilinear polynomial commitment scheme (PCS) and a sum-check engine
that are (i) post-quantum, (ii) *zero-knowledge*, and (iii) analysed in the
quantum random-oracle model (QROM), so that a SNARK built on them can protect
private witnesses. First customer: the QALABASH payment relation `R_spend`
(32-bit-field, hash-based statements). The library must stand on its own.

## 2. Relation to Akita
Akita (LayerZero Labs) is a transparent lattice PCS: binding and
knowledge-sound in the classical ROM, no hiding, response revealed in clear
(paper Sec. 8.2, Sec. 14; `book/src/roadmap/zero-knowledge.md`). aQiten starts
from the same Module-SIS folding idea and adds what Akita lacks. We write our
own implementation. Akita serves as a reference and a differential-testing
oracle for the *transparent* mode only. Reused code is marked and attributed.

## 3. Security goals (to be fixed as formal games in `spec/` before any code)
* **G1 Binding / knowledge soundness in the QROM.** Extraction against quantum
  provers with quantum oracle access.
* **G2 Hiding.** Commitments to f0 and f1 are computationally indistinguishable
  (MLWE).
* **G3 Evaluation zero knowledge.** A simulator, given only (params, x, f(x)),
  produces a view of (commitment, proof) indistinguishable from the real one;
  it may program the random oracle. Covers *every* transcript message from the
  first commitment to the terminal check, including sum-check rounds and fold
  commitments.
Hard rule: no partial "zk" mode may exist. A feature named zero knowledge
protects the whole proof, or it is an internal test of one component.

## 4. Epistemic discipline
Every claim in code comments, docs and reports carries one tag with a
justification and source: ESTABLISHED, CONJECTURE (with how it fails), or OPEN
PROBLEM. Heuristic parameter models are never presented as proofs. The H1
lesson from QALABASH (a commitment that binds but does not hide) is a standing
regression test: a lattice-reduction harness runs against every commitment.

## 5. Rules of work
1. Spec first (LaTeX, TR numbering), code second.
2. One crate at a time, each complete and runnable alone, depending only on the
   crates it calls. Finish a crate before starting the next.
3. Inspect what exists before writing; no placeholders; files and functions
   named for what they do.
4. Local validation (fmt, clippy, tests, deny, audit) before every push.
5. Anything cryptographic needs an adversarial test written before the
   implementation it targets.

## 6. Work packages
| WP | Content |
|----|---------|
| P0 | Full audit of the Akita paper and code; literature check; formal games G1-G3 |
| P1 | Ring/field algebra and parameter estimator crate; hiding (BDLOP-form) commitment; LLL regression |
| P2 | Masked fold; ZK sum-check; extraction and simulator harnesses |
| P3 | Classical-ROM composition theorem and prototype |
| P4 | QROM track (state-restoration / collapsing / round-reduction routes), parallel to P2-P3 |
| P5 | Implementation, benchmarks against Akita |
| P6 | External review |
