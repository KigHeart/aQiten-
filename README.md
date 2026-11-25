# aQiten

An independent lattice polynomial-commitment research framework targeting a
zero-knowledge PCS and a QROM security argument, building on ideas in Akita.
This is a separate implementation, not a fork.

**Status: research/specification stage. No cryptographic implementation exists.
Nothing in this repository may be used to protect secrets.** The proposed
construction has open proof obligations; it is not a security proof.

- CHARTER.md — goals, security games G1-G3, scope and engineering rules.
- docs/P0_PLAN.md — source-audit status and remaining work.
- docs/TREE.md — proposed crate architecture, DAG and build order.
- docs/ARCHITECTURE_REVIEW.md — review of the tree and dependency graph.
- docs/crate_dag.png — visual dependency map (dependency edges, not runtime order).
- spec/ — formal specifications and proofs, before implementation.
- papers/akita.pdf — read-only research reference.

The ring architecture is gated: exact reference negacyclic multiplication comes
first. A full base-field NTT is permitted only when the chosen (q,d) has a
verified primitive 2d-th root. The q = 5 mod 8 condition does not permit such
an NTT for power-of-two d >= 4.

Maintained by Tilil Engineering Group. Licensed under Apache-2.0 OR MIT.
