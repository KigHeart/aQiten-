# TR-TEG-2026-AQT-002 — independent construction review

**Review status:** design review, not a security proof.  
**Reviewed revision:** 1.0, commit `9f89577f0849c9f5913f74ef33402b77c6fd737e`.  
**Decision:** do not begin the cryptographic PCS implementation yet. Close the items below or explicitly record a justified design change first.

## Blocking questions

### B1 — Ring modulus and transform compatibility

The plan proposes a prime modulus with a possible profile (q\equiv5\pmod 8), while the crate plan calls for a negacyclic NTT over (R_q=\mathbb F_q[X]/(X^d+1)). These are not interchangeable requirements. A length-(d) negacyclic NTT over the base field requires a primitive (2d)-th root of unity, hence (2d\mid(q-1)). The congruence (q\equiv5\pmod8) alone does not provide this for general power-of-two (d).

**Required:** specify the supported (d,q) pairs and whether arithmetic uses (a) a base-field NTT, (b) an extension-field transform, or (c) exact schoolbook/Karatsuba multiplication. Do not promise an NTT until the root condition is checked by code and tests.

### B2 — The hiding commitment is not yet a drop-in replacement

The proposed commitment (t=A_1s+r_1+A'_2r_2) has the familiar LWE-shaped hiding term, and a collision can yield an SIS relation when all differences satisfy the required norm bound. But the statement that every Akita equation continues to hold after replacing (s) by ~tilde{s}=(s,r_1,r_2) is not established. Akita's compression, digit, range, and norm maps have specific domains and coefficient bounds.

**Required:** for every map used by the protocol (including A/B/D/F/H), give its augmented matrix, input alphabet, output dimensions, and a bound on the image of every admissible augmented witness. Prove that the verifier's original checks still characterize the intended relation. Treat this as an OPEN PROBLEM until then.

### B3 — The mask commitment and fold equation need one consistent algebra

The fold writes (t_y=\widetilde A\widetilde y) and checks
\[
\widetilde A z'=\sum_i c_i t_i+t_y.
\]
This is algebraically consistent only when each (t_i) is a commitment under the same augmented linear map and the folded vector includes every commitment-randomness coordinate. The document currently moves between the two-tier commitment notation and the folded source-vector notation without specifying the complete coordinate layout.

**Required:** define one typed vector layout for source digits, commitment randomness, fold masks, and any quotient/auxiliary coordinates. Write each commitment and fold equation with explicit dimensions. Show that the verifier can check the equation without learning the hidden coordinates.

### B4 — Box-rejection lemma is sound locally; transcript integration is not done

For fixed integer (z\in[-B,B]^N), uniform (y\in[-Y,Y]^N), and acceptance iff (y+z\in[-(Y-B),Y-B]^N), the accepted (z'=y+z) is exactly uniform on the smaller box, independent of (z). The exact acceptance probability is
\[
\left(\frac{2(Y-B)+1}{2Y+1}\right)^N.
\]
This does **not** by itself prove that the complete Fiat–Shamir transcript is simulatable. Every rejected attempt must restart all dependent commitments and challenges; the abort/retry transcript, query count, prover work, and distribution of the first accepted transcript must be included in the protocol definition.

**Required:** specify the attempt transcript and prove the joint distribution, not only the marginal distribution of (z'). Add a cap/abort policy and account for its effect on completeness and zero knowledge.

### B5 — Revealed-functional lemma does not yet apply to Fiat–Shamir weights

The leftover-hash argument is valid for a family of matrices with the stated collision bound. Akita's equality, tensor, and gadget weights are structured. The plan correctly flags this as Z3, but the condition that the verifier's matrix is independent of the mask is not automatic when challenges are derived from a transcript containing mask-dependent commitments.

**Required:** state the exact conditioning and hybrid sequence under which the challenge-derived weight family is independent of the mask; then prove the average collision bound for the actual structured family and for multiple functionals across recursion levels. If this cannot be proved, use explicitly sampled uniform weight rows and account for their proof-size and soundness costs.

### B6 — Masked sum-check protocol is underspecified

The text says (H_m=\sum_xm(x)) is “committed”, then uses (H+\rho H_m) as the sum-check claim. A commitment to (H_m) is not itself the scalar (H_m); the verifier needs a precise homomorphic relation or an opening/proof, and the order of commitment, challenge derivation, and disclosure must be fixed. The claim that each round is uniform also needs a conditional-distribution proof given the previously disclosed (H_m) and round messages.

**Required:** provide a complete prover/verifier transcript, including what is sent about (m), how the verifier checks the initial sum, and exactly what is opened at the final random point. Prove the distribution of every round conditioned on the entire prior view.

### B7 — The classical-ROM simulator theorem is still a target statement

The hybrid bound and simulator outline depend on B2–B6. The current report explicitly leaves Z3, Z4, and the composed hybrid proof open. It must not be described as a proven zero-knowledge PCS.

**Required:** write the simulator as an algorithm, list each hybrid, bound every transition, include all abort/retry events and oracle-programming collisions, and prove completeness and soundness are preserved.

## Non-blocking but required before parameter certification

1. The illustrative (q\approx2^{32}) calculation is not a parameter set. Produce the actual schedule table with (N,B_z,Y,q,d,n,m,k), challenge sizes, and per-level bounds.
2. Separate the security of the commitment (MLWE hiding / Module-SIS binding) from the security of Fiat–Shamir and the extraction theorem. Quantum hardness estimates do not imply a QROM reduction.
3. The QROM track remains OPEN. Do not label the full system post-quantum secure until G1 and G3 have explicit quantum-model theorems.
4. Add citations and exact theorem/page references for the BDLOP commitment, the masked sum-check construction, the LHL variant, and every QROM compiler route.
5. Reconcile the repository status: README and P0 plan must not say “charter stage” or “not yet done” when reports have been committed.

## Immediate build order

1. Exhaustively test the box-rejection lemma on small domains (this branch's `research/test_box_rejection.py`).
2. Resolve B1 and B2 with a notation/dimension appendix.
3. Complete the unread Akita sections listed in TR-001/TR-002 and extract concrete schedule parameters.
4. Only then start `aqiten-ring`; initially implement exact reference arithmetic, and add an NTT only for parameter sets satisfying the root-of-unity condition.
5. Write adversarial tests before implementing commitments, folds, or Fiat–Shamir.

**Bottom line:** aQiten has a credible research direction, but the construction plan is not yet a construction. The local box-rejection identity is useful; it is not evidence that the whole protocol is zero knowledge.
