# aQiten platform architecture (rev. 0.1)

## 1. Product definition

**aQiten is verifiable agentic infrastructure.** It is designed to let autonomous agents coordinate bounded work, invoke formally specified computations, and return evidence that independent systems can check.

It is not simply an LLM agent framework with a cryptographic library attached. The platform separates intelligence, authority, computation/proof and evidence so that model behaviour is replaceable while verification and action controls remain deterministic.

## 2. Four planes

### 2.1 Intelligence plane

Owns:
- task interpretation and decomposition;
- planning and specialist selection;
- agent-to-agent communication;
- model/provider adapters;
- optional memory retrieval and context assembly.

Does not own:
- capability issuance;
- policy approval;
- proof verification;
- protected side-effect execution.

Model outputs are untrusted proposals. Tool arguments, retrieved documents and delegated messages are treated as untrusted inputs.

### 2.2 Authority plane

Owns:
- caller and agent identity binding;
- signed, scoped and expiring capabilities;
- deterministic policy evaluation;
- delegation attenuation;
- budgets, quotas, expiry and revocation;
- human approval requirements;
- effect-gateway decisions.

Authority is checked at the point of use, not merely when a plan is created. A capability is not a general-purpose bearer permission: it is bound to subject, issuer, task, action set, resource scope, budget, expiry, delegation depth and nonce/replay rules.

### 2.3 Computation and proof plane

Owns:
- formal relation identifiers and versions;
- deterministic computation interfaces;
- relation/constraint compilation, once compiler correctness is established;
- lattice-based PCS and zero-knowledge proof construction;
- independent proof verification;
- approved cryptographic parameter profiles.

A proof establishes only that the specified relation holds for the supplied public inputs under the specified cryptographic profile. It does not prove that the relation captures the correct business intent, that input data is truthful, or that an external action occurred.

### 2.4 Evidence plane

Owns:
- canonical proof envelopes;
- authenticated event records;
- execution attestations;
- provenance and software/parameter identifiers;
- evidence export and verification receipts;
- retention and audit policy.

Evidence types remain distinct:
- model assertion;
- signed agent statement;
- gateway execution record;
- human approval;
- cryptographic proof;
- independent verification receipt.

A hash chain or signature makes a record tamper-evident/authenticated under its key assumptions; it does not make the record factually true.

## 3. End-to-end workflow

1. An authenticated caller submits a task and its public context.
2. The runtime creates a bounded task and assigns an immutable task identifier.
3. An agent proposes a plan. The plan is parsed into typed operations; natural-language text is not executed.
4. A policy-controlled issuer grants a least-privilege capability.
5. Delegation creates attenuated child capabilities for specialist agents.
6. Tool requests are submitted through the effect gateway.
7. The gateway checks caller/agent identity, capability signature, task binding, action/resource scope, budget, expiry, nonce/replay status, policy and required human approval.
8. The operation executes in a sandbox or explicitly configured connector.
9. If a formal computational claim is required, the computation is represented by a versioned relation and processed by the proof engine.
10. An independent verifier checks the proof and public statement.
11. The evidence plane packages the proof status, policy receipt, execution record and provenance without conflating them.

## 4. Product surfaces

| Surface | User | Main value |
|---|---|---|
| Developer SDK | Rust/application developers | Typed interfaces for tasks, capabilities, computations and proofs |
| Agent runtime | Platform operators | Durable execution, scheduling, delegation and observability |
| Effect gateway | Security/platform teams | Central enforcement point for protected actions |
| Proof service | Cryptographic/computation teams | Proving and independent verification |
| Evidence service | Auditors and counterparties | Portable, independently checkable evidence |
| Domain packages | Finance, identity, AI and enterprise teams | Predefined relations, workflows and policy templates |

## 5. Product non-goals

- Replacing every existing agent framework.
- Treating LLM judgement as a security boundary.
- Claiming that every agent thought or natural-language statement can be proven.
- Claiming that signatures prove truth or authorisation.
- Claiming that a ZK proof proves source-data provenance.
- Automatically compiling arbitrary Rust, Python or shell programs into sound proofs.
- Exposing an experimental ZK construction as production-safe before proof and audit gates pass.

## 6. First vertical

The first vertical is private, verifiable financial computation using the QALABASH `R_spend` relation and synthetic data. The demonstrator should include:
- task assignment and scoped capability;
- bounded delegation;
- deterministic relation evaluation;
- proof generation and independent verification;
- policy-gated mock settlement action;
- replay and unauthorised-action rejection;
- portable evidence export.

This validates the architecture without handling real money or production secrets.
