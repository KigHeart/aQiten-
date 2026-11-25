# aQiten product and engineering roadmap (rev. 0.1)

## North star

Build verifiable agentic infrastructure: autonomous agents coordinate bounded work; deterministic authority controls protected actions; cryptographic proofs establish specified computational claims; independent systems verify portable evidence.

## Stage 0 — research integrity and architecture

**Now.**

- Complete the Akita paper/source audit.
- Resolve the open construction questions in the independent review.
- Specify the full security games and simulator.
- Define the agent threat model and capability semantics.
- Preserve strict cryptographic/agent dependency separation.
- No production claims or protected-data use.

Exit: reviewed construction proposal, architecture contracts and threat model.

## Stage 1 — cryptographic foundation

- error, codec, field and exact reference ring arithmetic;
- estimator and parameter certificate validator;
- transcript and challenge sampling;
- commitment core/prover/verifier separation;
- masked sum-check and folding only after mathematical proof obligations are resolved.

Exit: complete, independently reviewable PCS/ZK construction and reference tests. A QROM target requires a QROM theorem; lattice assumptions alone are insufficient.

## Stage 2 — deterministic authority kernel

Build before adding autonomous side effects:
- agent/task identifiers and state machine;
- authenticated capability format;
- deterministic policy evaluation;
- delegation attenuation;
- replay protection, expiry, budgets and revocation;
- effect gateway with mock tools;
- adversarial tests for bypass, escalation and fail-open behaviour.

Exit: a non-LLM test harness cannot exceed granted authority, and every protected mock action is mediated.

## Stage 3 — proof-carrying computation

- formal relation registry and version identifiers;
- deterministic computation interface;
- prover/verifier SDK;
- canonical proof envelope;
- independent verification process;
- evidence schema that distinguishes proof, signature, approval and execution record.

Exit: a separate process verifies a proof and rejects altered statements, profiles and encodings.

## Stage 4 — agentic runtime

- durable task runtime and checkpoints;
- planner/orchestrator interface;
- specialist agents and bounded delegation;
- tool interfaces routed through the effect gateway;
- scoped memory and provenance;
- LLM provider adapters and MCP adapter as optional edge packages.

Exit: one agentic workflow completes a synthetic task without direct tool bypass and with a verifiable evidence package.

## Stage 5 — first vertical: private financial computation

Use QALABASH `R_spend` with synthetic data:
- agent receives task and capability;
- relation is selected from a reviewed registry;
- computation and proof are generated;
- independent verifier checks public statement;
- policy gateway approves or rejects a mock settlement;
- evidence package is exported and checked independently.

Exit: false statements, unauthorised actions, replay and altered evidence are rejected.

## Stage 6 — production qualification

- external cryptographic review and implementation audit;
- independent systems/security review;
- reproducible builds and parameter provenance;
- load, fault, recovery and denial-of-service testing;
- memory, tenant isolation, data retention and key lifecycle reviews;
- performance baselines against Akita/Jolt and relevant hash-based alternatives;
- production operations, incident response and version policy.

Exit: explicit product security case and deployment-specific acceptance. Passing tests alone is not sufficient.

## Scope discipline

Do not build a general-purpose agent framework, universal zkVM, arbitrary-code compiler, distributed prover cluster and enterprise dashboard simultaneously. Validate the cryptographic foundation and authority kernel first; then ship one end-to-end workflow.
