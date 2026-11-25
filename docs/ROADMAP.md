# aQiten product and engineering roadmap (rev. 0.2)

## North star

Build a verifiable agentic computation runtime: heterogeneous agents coordinate typed work; deterministic state and scheduling make execution recoverable; authority controls protected actions; formal computations establish narrow claims; independent systems verify portable evidence.

## Stage 0 — research integrity and specification

- Complete Akita paper/source audit and independent construction review.
- Resolve mathematical blockers and state security games precisely.
- Maintain the competitive product landscape.
- Finalize agentic system specification and compute lifecycle.
- Define typed artifacts, BSP semantics, capability semantics and evidence schema.
- Preserve cryptographic/runtime dependency separation.

Exit: reviewed construction proposal, normative runtime contracts and threat model. No production claims or protected data.

## Stage 1 — cryptographic foundation (parallel research track)

- error, codec, field and exact reference ring arithmetic;
- estimator and parameter certificate validator;
- transcript and challenge sampling;
- commitment core/prover/verifier separation;
- masked sum-check and folding only after mathematical obligations are resolved.

Exit: complete, independently reviewable PCS/ZK construction and reference tests. QROM target requires a QROM theorem; lattice assumptions alone are insufficient.

## Stage 2 — deterministic runtime kernel

- canonical IDs, task/result/artifact envelopes;
- versioned state schemas, typed channels and reducer laws;
- graph IR and deterministic graph validation;
- frozen frontier, immutable node input and BSP barrier;
- deterministic merge and content-addressed checkpoints;
- schedule permutation simulator.

Exit: same validated task results produce the same checkpoint regardless of completion order; incomplete/duplicate frontiers reject.

## Stage 3 — durability and cost

- SQLite checkpoint/task/effect journal;
- model-response journaling and idempotent run submission;
- fork-with-edit and provenance;
- budget reservation/reconciliation;
- crash recovery, park/resume and single-process fencing.

Exit: crash matrix passes; committed state is not applied twice; uncertain effects are not blindly retried.

## Stage 4 — deterministic authority kernel

- principal/workload identity abstraction;
- authenticated capabilities and delegation attenuation;
- deterministic policy decision and receipts;
- approval records, expiry and single-use consumption;
- effect taxonomy and mandatory gateway;
- mock tool adapters and adversarial bypass tests.

Exit: non-LLM test harness cannot exceed granted authority; every protected mock action is mediated.

## Stage 5 — formal computation and proof integration

- versioned relation registry;
- deterministic evaluator and canonical statement;
- stable prover/verifier interfaces;
- independent verifier process and canonical proof envelope;
- evidence graph and offline verification.

Exit: separate process rejects altered statements, relation versions, profiles and encodings. Experimental proof backend clearly labelled.

## Stage 6 — agentic runtime and adapters

- planner/orchestrator and typed fan-in policies;
- specialist agents consuming other agents' artifacts;
- challenge/critic and reconciliation nodes;
- scoped memory/context admission;
- one model provider, MCP and A2A adapters;
- OpenTelemetry exporter and evaluation harness.

Exit: agent workflow completes without tool bypass and produces inspectable provenance/evidence.

## Stage 7 — first vertical

Use synthetic QALABASH R_spend and mock settlement:
- bounded agent tasks and capabilities;
- typed data-preparation and policy-analysis artifacts;
- deterministic relation evaluation;
- proof generation and independent verification;
- policy-gated mock settlement;
- evidence export and offline verification.

Exit: false statement, wrong relation, unauthorized action, replay and altered evidence are rejected.

## Stage 8 — production qualification

- external cryptographic review and implementation audit;
- independent systems/security review;
- reproducible builds and parameter provenance;
- load, fault, recovery and denial-of-service testing;
- tenant isolation, data retention and key lifecycle reviews;
- multi-worker store and lease fencing;
- performance baselines against Akita/Jolt, zkVMs and relevant hash-based alternatives;
- operational incident response and version policy.

Exit: explicit product security case and deployment-specific acceptance. Passing tests alone is not sufficient.

## Scope discipline

Do not build a universal agent framework, zkVM, arbitrary-code compiler, distributed prover cluster, browser automation suite and enterprise dashboard simultaneously. The first agentic milestone is the deterministic BSP kernel plus authority gateway—not a sophisticated LLM loop.
