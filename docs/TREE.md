# aQiten workspace and platform tree (rev. 0.2, 2026-11-25)

**Status: target architecture; no Rust workspace or implementation exists yet.** This document is normative for the proposed repository structure. Product planes are not identical to Cargo crates. Crates are introduced only when their contracts and tests are ready. The platform-level trust flow is shown in [PLATFORM_DAG.md](PLATFORM_DAG.md).

## 1. System boundary

aQiten has four planes:

- **Intelligence:** LLM adapters, planning, orchestration and delegation.
- **Authority:** identities, capabilities, policy, approvals and effect mediation.
- **Computation/proof:** relations, arithmetic, commitments, sum-check, folding, proving and verification.
- **Evidence:** canonical proof envelopes, signed events, provenance and audit export.

The allowed dependency direction is from agent/application layers toward stable authority and proof interfaces. The cryptographic verifier has no dependency on the agent plane.

## 2. Cryptographic and computation core

### Foundation

| Crate | Owns | Principal API surface (provisional) |
|---|---|---|
| `aqiten-error` | Typed errors and checked size arithmetic | `Error`, `Result<T>`, checked length/range helpers |
| `aqiten-codec` | Canonical encoding and strict decoding | `Encode`, `DecodeCanonical`, bounded decoder |
| `aqiten-field` | Selected finite-field arithmetic | `FieldElement`, checked constructors, add/mul/invert |
| `aqiten-ring` | Ring arithmetic and exact reference backend | `RingElement`, negacyclic multiply, digit decomposition, norm checks |
| `aqiten-estimator` | Offline security/performance cost models | candidate estimates, assumption-labelled reports |

### Protocol mathematics

| Crate | Owns | Principal API surface (provisional) |
|---|---|---|
| `aqiten-transcript` | Domain-separated transcript and query accounting | append, derive challenge, query ledger |
| `aqiten-challenges` | Challenge distributions and bounded samplers | sample, validate support and norm predicates |
| `aqiten-params` | Parameter certificate and independent validator | generate candidate, validate certificate |

### Commitment and proof components

| Crate | Owns | Principal API surface (provisional) |
|---|---|---|
| `aqiten-commit-core` | Public commitment parameters and public types | commitment parameters, public commitment encoding |
| `aqiten-commit-prover` | Witness-side commitment generation | commit, randomness handling |
| `aqiten-commit-verifier` | Public commitment/opening checks only | verify commitment/opening |
| `aqiten-sumcheck` | Ordinary and, only after proof, masked sum-check | prove rounds, verify rounds |
| `aqiten-fold` | Folding and mask propagation | fold prover/verifier interfaces |
| `aqiten-schedule` | Schedule artifacts and deterministic validation | schedule, validate schedule |

### Protocol roles

| Crate | Owns | Principal API surface (provisional) |
|---|---|---|
| `aqiten-types` | Canonical public statements, proofs and setup types | `Statement`, `Proof`, `PublicParameters` |
| `aqiten-prover` | Proof sequencing and retry/restart semantics | `prove` |
| `aqiten-verifier` | Independent verification and one acceptance rule | `verify` |
| `aqiten-setup` | Public setup artifact generation | `setup`, artifact validation |
| `aqiten-pcs` | Public orchestration API for PCS | `commit`, `prove`, `verify` facade |

The APIs above are design targets, not current functions. Final signatures depend on the proved construction.

## 3. Agentic infrastructure crates

The agent subsystem is a separate layer. It must not be imported by the cryptographic verifier.

| Crate | Responsibility | Security boundary |
|---|---|---|
| `aqiten-agent-core` | Agent/task identifiers, typed task states, run lifecycle | No model-provider dependency |
| `aqiten-agent-protocol` | Typed messages, task/result envelopes, correlation and replay fields | Canonical bounded messages |
| `aqiten-agent-capability` | Signed, scoped, expiring capabilities and attenuation | No self-grant; issuer/verifier separated |
| `aqiten-agent-policy` | Deterministic policy evaluation and decision receipts | Model output is never policy authority |
| `aqiten-agent-delegation` | Child-task delegation and capability attenuation | Cannot expand parent authority |
| `aqiten-effect-gateway` | Sole execution boundary for protected side effects | Re-check identity, capability, policy, budget and approval at execution time |
| `aqiten-agent-runtime` | Durable task execution, checkpoints, cancellation and scheduling | Bounded resources and explicit state transitions |
| `aqiten-agent-orchestrator` | Plan graphs, specialist selection and agent coordination | Plans are proposals, not permissions |
| `aqiten-agent-tools` | Typed tool contracts and sandbox adapters | No unrestricted shell/network by default |
| `aqiten-agent-proof` | Bind agent claims to formal relations and proof objects | Does not treat natural-language claims as proven |
| `aqiten-agent-evidence` | Authenticated event records, provenance and export | Distinguishes signatures, attestations and ZK proofs |
| `aqiten-agent-memory` | Scoped, versioned, policy-controlled task memory | Tenant isolation, provenance and retention policy |
| `aqiten-agent-llm` | Optional model/provider adapters | Feature/adapter boundary; no core dependency |
| `aqiten-agent-mcp` | Optional Model Context Protocol adapter | MCP transport is not authorisation |
| `aqiten-agent-api` | Application-facing API and service integration | Authenticated callers and rate/resource limits |

### Research and test-only crates

| Crate | Purpose | Restriction |
|---|---|---|
| `aqiten-sim` | Simulator and extraction harnesses | dev/test only |
| `aqiten-attacks` | Hiding/binding and protocol attack harnesses | dev/test only |
| `aqiten-akita-oracle` | Differential tests against transparent Akita mode | feature-gated, off by default |
| `aqiten-bench` | Performance and size measurements | dev/bench only |
| `aqiten-agent-eval` | Agent adversarial evaluations and regression suites | dev/test only |

## 4. Dependency rules

### Cryptographic boundary

- `aqiten-verifier` may depend on public types, codec, field/ring, transcript, challenge/parameter validation, sum-check verification and commitment-verifier.
- It must not depend on prover, fold-prover, setup, simulator, attacks, LLM adapters, agent runtime, memory or tool crates.
- `aqiten-commit-verifier` contains no witness sampling or randomness-generation code.
- `aqiten-types` contains public wire types only; no prover internals.
- `aqiten-estimator` is an offline model. Parameter acceptance is performed by a deterministic validator, not by the estimator.

### Agent boundary

- `aqiten-agent-orchestrator` may depend on agent-core, protocol and runtime interfaces; it does not issue capabilities.
- `aqiten-agent-capability` and `aqiten-agent-policy` are deterministic and do not call LLMs.
- `aqiten-agent-delegation` can attenuate a parent capability only.
- `aqiten-effect-gateway` is the only component permitted to execute protected side effects.
- Tool adapters call the gateway; they do not bypass it.
- LLM and MCP adapters are optional and live at the edge.
- `aqiten-agent-proof` calls stable public proof interfaces; the proof verifier does not call back into agent code.
- Evidence records distinguish what was proved from what was merely signed or reported.

### Cross-plane rule

The proof core may be used without agents. Agents may use proof services. Neither side may assume the other proves claims outside its contract.

## 5. Runtime trust flow

1. A caller submits a task with authenticated context.
2. The runtime creates a task record and binds it to a caller/tenant.
3. The orchestrator proposes a plan and subtask graph.
4. A capability issuer or policy-controlled authority grants a bounded capability.
5. Delegation derives child capabilities with no greater authority.
6. Agents request tools through typed interfaces.
7. The effect gateway revalidates capability, policy, budget, expiry, replay nonce and human approval immediately before execution.
8. Deterministic computations run against explicit inputs and relation versions.
9. The prover returns a proof; an independent verifier checks it.
10. The evidence service records task, agent, policy, execution attestations and proof status as distinct evidence types.

A successful proof does not authorise a side effect. A policy approval does not prove a computation. A signature does not establish truth of a claim.

## 6. Initial demonstrator

Use the QALABASH `R_spend` relation and synthetic/test data:

- one planner agent;
- one data-preparation agent;
- one computation/prover worker;
- one independent verifier;
- one policy/effect gateway;
- one evidence exporter.

The first demo should prove a defined payment relation, reject a false statement, prevent an unauthorised action, reject replayed requests and export a verifiable evidence package. It must not move real money or use production secrets.

## 7. Build sequence

| Phase | Build | Exit gate |
|---|---|---|
| C0 | error, codec, field | canonical encoding, checked arithmetic, known-answer tests |
| C1 | ring, estimator | exact reference arithmetic; certified parameters only |
| C2 | transcript, challenges, params, schedule | domain separation, query accounting, deterministic certificate validation |
| C3 | commitment core/prover/verifier | complete construction and adversarial tests; no verifier-to-prover dependency |
| C4 | sum-check, fold | dimension-complete equations and reviewed distribution/simulation proofs |
| C5 | types, setup, prover, verifier, PCS | complete proof acceptance rule and independent verification |
| A0 | agent-core, protocol, capability, policy | identity/capability threat model and deterministic policy tests |
| A1 | delegation, effect-gateway, tools | attenuation, replay protection, fail-closed side-effect tests |
| A2 | runtime, orchestrator, evidence | durable task lifecycle and evidence semantics |
| A3 | proof integration, memory, API | end-to-end synthetic-data demonstrator |
| A4 | LLM/MCP adapters, eval, runtime hardening | provider-independent integration and adversarial evaluation |

Do not implement all crates at once. Each phase requires a reviewed contract and passing tests before dependent crates begin.

## 8. Ring and NTT constraint

A full negacyclic NTT of degree (d) over (mathbb F_q) requires a primitive (2d)-th root, hence (2d\mid(q-1)). The proposed (q\equiv5\pmod8) profile does not permit a full base-field NTT for power-of-two (d\ge4). Exact reference multiplication is mandatory; an NTT backend is optional only with a verified root certificate and separately approved parameter profile.

## 9. Status

This is a target workspace design, not an inventory of implemented crates. The repository currently contains research documents, the Akita paper and the prior architecture review. There is no Cargo workspace, no agent runtime, no proof SDK and no production verifier.
