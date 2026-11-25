# P0: audit and formalisation (not yet done)

Inputs: the Akita paper (ePrint 2026/1983, to be supplied in full including
Appendices C, D, F, G), the Akita repository (read-only reference), and the
literature on lattice ZK, hiding PCS and Fiat-Shamir in the QROM.

Deliverables:
1. `spec/p0_audit.tex`: what Akita proves, under which assumptions, and where
   each of its messages leaks (sum-check rounds, fold commitments, terminal
   response), message by message.
2. Formal games G1-G3 and the leakage table feeding them.
3. A citations table: every cited result checked against its source, with
   status verified / unverified.
4. Parameter questions to settle in P1 (modulus form, challenge family,
   digit base, masking strategy: flooding vs rejection).

Gate: P1 does not start until (1)-(3) are reviewed.

## Status (2026-11-25)
`spec/aqt001_p0_audit.tex` (TR-TEG-2026-AQT-001, Rev. 1.0) delivers items 1 and 2 for the sections
of the paper read so far, and lists exactly what is still unread (see its Sec. 1) and the P1 gate.
Item 3 (citation table) is not done: literature claims for the QROM track are unverified.

## Update (2026-11-25, later)
`spec/aqt002_construction_plan.tex` (TR-TEG-2026-AQT-002) gives the construction plan: principles D1-D6,
masked fold (box rejection, committed mask, extraction lemma), revealed-functional lemma (open for structured
weights, item Z3), masked sum-check, parameter inequalities I1-I6, crate plan, QROM routes Q1-Q3, milestones.
Still unread from the Akita paper: A Sec. 3-4, 9, 11-13, App. A, B, rest of C, E, G.
