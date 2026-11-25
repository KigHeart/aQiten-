# Ring architecture gate (TR-TEG-2026-AQT-003)

**Status:** design gate; no production parameter set selected.  
**Scope:** resolve the algebraic compatibility question before implementing `aqiten-ring`.

## 1. Immediate incompatibility

The construction plan currently combines:

- a prime coefficient field with (q \equiv 5 \pmod 8); and
- a negacyclic ring (R_q=\mathbb F_q[X]/(X^d+1)), with a possible full base-field NTT.

A full negacyclic NTT of length (d) over (mathbb F_q) requires a primitive (2d)-th root of unity. Equivalently,

\[
2d \mid (q-1).
\]

For power-of-two (d\ge 4), this is incompatible with (q\equiv5\pmod8): the latter gives (v_2(q-1)=2), while (2d\) is divisible by at least (8). Thus **there is no full base-field negacyclic NTT for the stated modulus class at degree (d\ge4)**.

This does not make negacyclic multiplication impossible. It means the implementation must not assume a full NTT under that modulus constraint.

## 2. Three coherent choices

| Choice | Coefficient modulus | Multiplication | Consequence |
|---|---|---|---|
| A | Keep prime (q\equiv5\pmod8) | Exact schoolbook/reference multiplication first; later investigate Karatsuba or extension-field transforms | Preserves the stated challenge/invertibility condition; no full base-field NTT for power-of-two (d\ge4) |
| B | Choose prime (q\equiv1\pmod{2d}) | Full negacyclic NTT | Efficient transform path, but the current (q\equiv5\pmod8) challenge argument must be re-derived or replaced |
| C | Keep the challenge field and use a distinct transform modulus / CRT | Exact multi-modulus multiplication with proven reconstruction bound | More complex; every auxiliary modulus and reconstruction bound must be specified, and this does not change the protocol's coefficient field |

**Current decision:** do not select A, B, or C yet. Build and test the exact reference ring independently of any NTT. Keep NTT support conditional on a verified root certificate. The challenge-set proof and ring multiplication implementation are separate design decisions.

## 3. Required executable invariants

For every candidate pair ((q,d)), the audit must check:

1. (q) is prime.
2. (d\ge1).
3. A full negacyclic NTT is declared available only if (2d\mid q-1) and an element (omega\in\mathbb F_q) of exact order (2d) is exhibited.
4. The certificate verifies (omega^d=-1\pmod q) and (omega^{2d}=1\pmod q), plus exact order (for power-of-two (2d), the first two conditions suffice).
5. Reference multiplication is performed in (mathbb Z[X]/(X^d+1)) and reduced coefficient-wise modulo (q); the wrap from degree (i+j\ge d) contributes a minus sign.
6. No parameter is called cryptographically secure by this audit. It says nothing about MLWE, Module-SIS, extraction, or QROM security.

## 4. Why the reference implementation comes first

The reference ring is the executable specification against which any optimized backend must be differentially tested. It avoids coupling protocol correctness to a transform assumption. Once the protocol's modulus and degree are fixed, an optimized backend can be admitted only if it matches the reference for randomized and boundary vectors.

## 5. Blocking follow-up

Before choosing a profile, extract from Akita's actual shipped schedules the tuple ((q,d,n,m)) at every level, and resolve whether its (q\equiv5\pmod8) condition is essential to the challenge-difference invertibility proof. Then choose one of A/B/C and re-run the complete binding, hiding, no-wrap, and challenge-set inequalities together.

Until that is done, `aqiten-ring` should contain exact reference arithmetic and parameter validation only—not an assumed NTT and not protocol-facing cryptographic APIs.
