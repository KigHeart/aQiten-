# aQiten agentic system specification

**Version:** 0.1.0-draft  
**Date:** 25 November 2026  
**Authoring organisation:** Tilil Engineering Group, the aQiten Team  
**Status:** normative target specification; not a claim of implemented or audited functionality.

The terms MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT and MAY are normative.

## 1. System definition

aQiten is a Rust-native, durable, verifiable agentic computation runtime. It executes heterogeneous agents and deterministic workers as a typed computation graph. Agent outputs are first-class, provenance-bearing artifacts and may become inputs to subsequent agents, tools, deterministic computations or proof jobs. Runtime progress is organized into deterministic super-steps. Protected external effects are mediated by a capability- and policy-enforcing gateway. Selected formal computations may produce cryptographic proofs that an independent verifier can check. A portable evidence graph records what was asserted, computed, authorized, executed and verified.

aQiten is not a foundation model, prompt/crew framework, general-purpose workflow replacement, general-purpose sandbox, replacement for OAuth/SPIFFE/cloud IAM, universal zkVM, or claim that natural-language reasoning can be proven correct.

## 2. Design principles

**P1 — Agent results are data.** Every node produces a typed result envelope. Downstream work consumes explicit artifact references and declared schemas, not hidden shared conversational history.

**P2 — Intelligence does not confer authority.** An LLM, planner, agent, tool response or retrieved document MUST NOT issue capabilities, modify policy, approve its own request or directly execute protected effects.

**P3 — State transitions are deterministic.** Given the same committed checkpoint, schema, validated task results and routing inputs, the transition MUST produce the same checkpoint identity. Completion order MUST NOT affect merge order.

**P4 — Durable progress is explicit.** Persist task results, reservations, effect intents, approvals and checkpoints under a declared durability mode. Recovery distinguishes completed work from uncertain work.

**P5 — Computation and reasoning differ.** Probabilistic reasoning is an untrusted proposal mechanism. Deterministic computation evaluates explicit inputs under versioned code. Proof verification checks a formal statement under a cryptographic profile.

**P6 — Proofs are narrow claims.** A proof binds a relation ID/version, public statement, public input digest, protocol version and parameter profile. It does not imply source-data truth, business-policy correctness, authorization or external execution.

**P7 — Fail closed at authority boundaries.** If identity, capability, policy, budget, replay state, approval or required proof verification cannot be checked, a protected effect MUST NOT execute.

**P8 — Replaceable intelligence, stable trust core.** Model and agent providers are adapters. State, authority, effect gateway, computation contracts and verifier remain provider-independent.

## 3. Actors and trust domains

- Caller: human, service or upstream system submitting work.
- Tenant: security/data isolation boundary.
- Run: durable execution instance.
- Agent definition: versioned instructions, model profile, schemas and permitted operations.
- Agent invocation: one execution of an agent definition.
- Node: graph operation: agent, tool, function, relation evaluator, prover, verifier or external service.
- Scheduler: selects eligible nodes and resource reservations.
- Authority service: issues and revokes capabilities.
- Policy decision point (PDP): deterministic evaluator.
- Policy enforcement point (PEP): gateway/adapter enforcing decisions.
- Effect gateway: only protected side-effect path.
- Computation engine: deterministic executor for registered relations.
- Prover: creates proof for supported relation/profile.
- Verifier: independently checks proof and statement.
- Evidence service: records and exports authenticated evidence.
- Store: durable checkpoints, journals and artifacts.

Tenant isolation applies to artifacts, memory, checkpoints, capability state and evidence. Agent display names are not identities. Human, workload, agent-definition and invocation identities are distinct.

## 4. Core data model

### Run
RunId is unique within deployment namespace. It binds tenant, caller, initial request digest, runtime version, state schema version and creation record. It is never reused.

### Task node
A TaskNode contains stable NodeId, node kind and implementation digest, input/output ports and schema versions, dependency edges, effect class, resource profile, retry policy, timeout/cancellation semantics, required capability class, provenance policy, quality threshold and approval requirement.

### Artifact
An Artifact contains artifact ID, media/type ID, schema ID/version, canonical payload digest, storage reference, producer identity/version, run/step/attempt, input artifact references, trust/sensitivity labels and creation event reference. Optional signatures, proofs and attestations are separate fields.

Payloads are bounded and canonicalized before hashing. Mutation creates a new version. A digest establishes content identity, not correctness.

### Agent result
An AgentResult contains invocation ID, agent definition/version, status (Complete, Partial, Abstained, Rejected, Failed), typed output artifact references, quality metadata with origin, model/provider metadata, prompt/template digest, input artifact references, tool/effect references, usage/cost, error classification and provenance.

Model-generated confidence is not a calibrated probability unless independently calibrated for that task class.

### Computation job
A ComputationJob contains job ID and run/node binding, immutable relation ID/version, public input artifact references, private witness references in protected storage, relation schema/encoding version, evaluator version, proof profile, resource limits, output schema, statement digest and idempotency key.

Unavailable, deprecated or unapproved relation/profile causes rejection.

### Checkpoint
A Checkpoint records run/thread, monotonic super-step, parent digest, state schema/version, canonical snapshot or validated delta, next frontier, task-journal high-water mark, effect-journal references, budget ledger, policy epoch, capability references, park marker, evidence-chain head and runtime/node manifest digests.

Checkpoint ID is derived from canonical encoding. Checkpoints are immutable.

## 5. Runtime execution model

### Graph
A run is a directed graph of typed nodes and edges. An edge declares source/output port, destination/input port, schema compatibility, cardinality, provenance/trust requirement and fan-in policy.

The graph validator checks unique IDs, supported node versions, acyclic per-step dependencies, schema compatibility, required input producers, fan-in, bounded loops, resource maxima and effect boundaries. Loops transition across super-steps, not unbounded recursion within a node.

### BSP super-step
Each step:
1. Freeze frontier.
2. Admit immutable node inputs.
3. Reserve budgets.
4. Dispatch eligible nodes.
5. Journal terminal results.
6. Reach barrier.
7. Validate exact completion coverage.
8. Merge writes deterministically.
9. Compute next frontier.
10. Commit checkpoint.

A node MUST NOT observe sibling writes from its current step. Outputs become visible only after barrier and commit. Strict barriers are the initial model; streaming partial outputs are non-committing events only.

### Frontier
Frontier is the exact set of nodes expected to execute. Each entry binds node ID, implementation digest, attempt, input digest and resource reservation. Commit requires one accepted terminal record for every expected entry or an explicit parked/failure resolution. Missing, duplicate, unexpected or stale results reject the step.

### Deterministic merge
State channels declare type, scope, reducer and version. Reducers declare identity and deterministic behavior. Kernel sorts writes by stable node ID and canonical ordinal before reduction. If reducer is intended to be order-independent, tests SHOULD verify commutativity too. Wall-clock, unseeded randomness and unordered map iteration are prohibited inputs.

A merge failure leaves parent checkpoint unchanged.

## 6. Agent-to-agent computation

The distinguishing interaction is a typed producer-consumer relationship, not conversation handoff alone.

An invocation receives objective/constraints, immutable input snapshot, explicit artifact references, capability-derived operation descriptions, context-admission result, resource budget, quality target and output schema. It returns a validated result envelope and output artifacts.

Downstream agents may consume, compare, challenge, reconcile, transform into relation inputs or escalate. Fan-in MUST explicitly choose:
- AllRequired;
- AnyAccepted;
- Quorum(required,total);
- DeterministicEvaluator(id,version);
- ConsensusPredicate(id,version);
- HumanAdjudication(role set,expiry).

Quorum is a workflow acceptance rule, not truth. Correlated agents sharing model/provider/prompt/data must not be represented as independent witnesses.

A challenge node receives original artifact, input lineage and challenge objective. It returns a separate challenge artifact with claim references, counterevidence, reproducible checks and outcome Supported, Refuted or Inconclusive. Original artifact is immutable.

External MCP/A2A messages are untrusted until authenticated and schema/provenance/policy validated.

## 7. Authority model

### Identity
Authenticate caller using configured credentials. Resolve tenant, principal, roles and policy context. Where deployed, integrate workload identity such as SPIFFE. Human, workload, agent definition and invocation identities remain distinct.

### Capability
A capability binds issuer, subject, tenant, run, task/node scope, action IDs, resource selectors, quantity/spend/compute limits, validity interval, delegation depth/delegate constraints, nonce, revocation epoch, policy version and parent digest for delegated grants.

Capabilities are authenticated. Issuer keys and policy authority are outside model control. Capabilities SHOULD be short-lived. Raw credentials MUST NOT enter model context. Signature is necessary but not sufficient for execution.

### Attenuation
For child capability, actions are a subset, resource scope is no broader, budgets do not increase, expiry is no later, delegation depth strictly decreases, tenant/run binding is unchanged and delegate identity is constrained by parent policy. Ambiguous or invalid attenuation fails.

### Policy
A canonical policy request binds principal, capability, action, resource, request digest, run/step/node, context, budget, approval state and policy epoch.

Decision: Allow, Deny, RequireApproval, Park or Escalate. Receipt includes policy ID/version, decision ID, evaluated input digest, reason codes, obligations and expiry.

Policy MUST be deterministic for fixed bundle/input snapshot. Dynamic external facts must be snapshotted/versioned or request is not reproducible.

## 8. Effect gateway

The gateway is the only protected side-effect path. Tool adapters submit typed EffectRequest and MUST NOT hold bypass credentials.

Before execution gateway:
1. authenticates workload and invocation;
2. validates capability signature and binding;
3. checks action/resource scope;
4. checks budget/rate limits;
5. checks expiry/revocation;
6. checks nonce/replay;
7. evaluates current policy;
8. verifies required approval;
9. verifies required proof receipt and statement binding;
10. validates schema, payload bounds and effect class;
11. persists durable intent;
12. executes adapter;
13. persists result or unknown outcome;
14. emits authenticated evidence event.

Mandatory check unavailable means fail closed. Authorization is not execution success.

Effect classes: Pure (no mutation), Idempotent (downstream stable idempotency contract), Compensable (registered compensation, not guaranteed reversal), Irreversible (no automatic retry after uncertainty). Undeclared defaults to Irreversible.

Persist intent before dispatch. On recovery, return known result; reconcile idempotent calls using same key; park ambiguous irreversible effects. Never infer success from timeout. Exactly-once external execution is conditional on downstream contract.

## 9. Computation and proof plane

### Relation registry
Manifest declares stable relation ID/version, source/spec digest, public/private schemas, canonical encoding, mathematical relation/evaluator, output schema, domain constraints, evaluator digest, proof profile compatibility, test vectors, conformance suite and deprecation policy. Semantic change requires new version.

### Deterministic evaluation
No LLM calls inside evaluator. Same canonical inputs and evaluator version produce same output or classified error. External I/O, wall-clock and unseeded randomness are forbidden.

### Proof lifecycle
Resolve immutable relation; canonicalize statement/inputs; validate witness; resolve approved profile; build transcript domain; prove in isolated component; produce canonical envelope; verify in separate process; emit receipt binding proof/statement digests, relation/version, profile, verifier build and result.

Verifier MUST NOT depend on runtime, agent, model, prover internals or network for mathematical acceptance. It accepts bounded canonical inputs and rejects unknown profiles, noncanonical encodings and mismatched statements.

### Cryptographic maturity gate
The proposed lattice PCS/ZK construction remains research. Do not label proof production-ready or post-quantum secure until construction, binding/hiding/soundness/ZK claims, reductions, parameter constraints and QROM argument (if claimed) are reviewed; implementation tests pass; independent cryptographic review completes. Until then proof backend is experimental and cannot satisfy production policy unless deployment explicitly accepts research risk.

## 10. Evidence graph

Evidence is a DAG of canonical records: CallerRequest, PlanProposal, AgentInvocation, AgentAssertion, Artifact, DeterministicComputation, ProofEnvelope, VerificationReceipt, CapabilityGrant, PolicyDecision, HumanApproval, EffectIntent, EffectResult, ExecutionAttestation, CheckpointCommit and RuntimeManifest.

Edges: derived_from, consumed, delegated_from, authorized_by, executed_as, proves, verified_by, committed_in, supersedes.

Each record has schema/version, digest, producer identity, timestamp source, run correlation, sensitivity and authentication metadata.

Hash chain/signature provides tamper evidence/authenticity under assumptions, not factual truth. Export preserves claim types and verification requirements. Offline verifier checks canonical encodings, signatures and proof receipts with declared trust roots and revocation snapshot semantics.

## 11. Memory and context admission

Memory is not canonical execution state. Scopes: Invocation, Node, Run, Thread, Tenant and Principal. Each item carries provenance, sensitivity, tenant, scope, digest, source version, retention class and trust label.

Admission policy selects Always, Window, ByReference, Summary, RetrieveTopK or Never. Admission is deterministic from checkpoint, policy and step. Retrieved content is untrusted and cannot override policy/capabilities. Cross-tenant retrieval is denied without explicit grant.

Compaction creates a derived artifact with source references and summarizer/version digest; original artifacts remain.

## 12. Cost and quality control

Ledger tracks model, tool, compute, proof, storage, network and human-review costs. Reservations and reconciliations are separate events.

Every node declares maximum attempts, tokens, wall time, CPU/memory where enforceable, tool calls, proof resources, monetary budget and quality threshold or best-effort mode.

Scheduler records model/price manifest and expected cost. It MUST NOT silently lower quality threshold to fit budget. If threshold cannot be met, return BestEffort only if caller policy allows; otherwise Escalated or Failed.

## 13. Run state machine

Primary progression:
Created -> Authenticated -> Planned -> Admitted -> Running -> Barrier -> Merging -> Committing -> Running | AwaitingApproval | AwaitingReconciliation | Completed

Exceptional/terminal states: Completed, BestEffort, Escalated, Failed, Cancelled, Quarantined.

AwaitingApproval and AwaitingReconciliation are durable parked states, not sleeping processes. Resume requires authenticated event/actor, valid lease/fencing epoch and schema compatibility. Cancellation stops dispatch and requests cooperative cancellation; issued irreversible effects are reconciled, not assumed undone. Integrity/schema/journal/evidence failure quarantines run. Completed requires all acceptance predicates, proofs and effects resolved.

## 14. Persistence and recovery

Initial backend SHOULD be SQLite for single-process development. Multi-worker production requires transactional compare-and-swap checkpoint heads, lease epochs and fencing tokens.

Store interfaces: CheckpointStore, TaskJournal, EffectJournal, ArtifactStore, CapabilityStore, ApprovalStore, EvidenceStore and LeaseStore.

Store atomically commits checkpoint head, state, frontier and ledger. Large artifact bytes may use object storage; digest/reference/retention metadata commit transactionally.

Schema migrations are explicit and versioned. Resume rejects unknown schemas or applies deterministic migration; silent reinterpretation is forbidden.

## 15. Runtime crate architecture

### Foundation
- aqiten-core: IDs, errors, clocks and primitives.
- aqiten-codec: canonical bounded encoding.
- aqiten-types: run/task/agent/artifact/result/evidence types.
- aqiten-state: schemas, channels, reducers and snapshots.
- aqiten-store: storage traits and SQLite backend.
- aqiten-journal: task, model response, effect and event journals.

### Deterministic runtime
- aqiten-graph: graph IR, ports, edge constraints and validation.
- aqiten-kernel: frontier, barriers, merge and checkpoint transition.
- aqiten-scheduler: admission, reservations, concurrency, retries and routing.
- aqiten-runtime: lifecycle, dispatch, park/resume, cancellation, recovery and leases.
- aqiten-sim: schedule, crash and fault simulation.

### Authority and effects
- aqiten-identity: principal/workload identity abstraction; SPIFFE adapter separate.
- aqiten-capability: issue, validate, attenuate and revoke.
- aqiten-policy: PDP, policy bundle/version and decision receipt.
- aqiten-approval: durable approvals and single-use consumption.
- aqiten-effect: effect taxonomy, idempotency and reconciliation types.
- aqiten-gateway: PEP and protected side-effect execution.

### Computation and proof
- aqiten-relation: relation manifest and registry.
- aqiten-compute: deterministic relation executor and job lifecycle.
- aqiten-proof-api: stable prover/verifier contracts and envelope types.
- aqiten-prover: experimental proof generation, isolated from verifier.
- aqiten-verifier: independent verifier with minimal dependencies.
- aqiten-evidence: evidence graph, canonical export and offline validation.

### Adapters and operations
- aqiten-agent: agent definition/invocation interface.
- aqiten-model: model provider trait and adapters.
- aqiten-mcp: MCP adapter.
- aqiten-a2a: A2A adapter.
- aqiten-memory: scoped memory and retrieval.
- aqiten-telemetry: OTel and observability exporters.
- aqiten-api: application service API.
- aqiten-cli: developer/operator CLI.
- aqiten-eval: evaluation harness, test/dev first.

### Dependency rules
- aqiten-verifier MUST NOT depend on runtime, agent, model, memory, gateway or prover.
- capability and policy MUST NOT call models.
- gateway is the only protected-effect execution path.
- runtime may depend on authority/compute/proof interfaces; those systems do not depend back on runtime.
- model/MCP/A2A/store/telemetry are edge adapters.
- sim/eval are not production dependencies.
- experimental crypto cannot be silently selected as production verifier.

## 16. Public API targets

Semantic contracts, not final Rust signatures:
- Runtime::submit(TaskRequest) -> RunHandle
- Runtime::inspect(RunId) -> RunSnapshot
- Runtime::resume(RunId, ResumeAuthorization) -> RunHandle
- Runtime::cancel(RunId, CancelAuthorization) -> CancelReceipt
- Runtime::fork(RunId, CheckpointId, ForkSpec) -> RunHandle
- Scheduler::admit(Checkpoint, Frontier) -> AdmissionPlan
- Kernel::commit_step(Checkpoint, ValidatedResults, NextFrontier) -> Checkpoint
- CapabilityIssuer::issue(GrantRequest) -> Capability
- CapabilityVerifier::validate(Capability, InvocationContext) -> CapabilityClaims
- PolicyEngine::evaluate(PolicyRequest) -> PolicyDecision
- EffectGateway::execute(EffectRequest) -> EffectOutcome
- ComputeEngine::evaluate(ComputationJob) -> ComputationResult
- Prover::prove(ProofRequest) -> ProofEnvelope
- Verifier::verify(ProofEnvelope, PublicStatement) -> VerificationReceipt
- EvidenceExporter::export(RunId, EvidenceSelection) -> EvidenceBundle

Every API defines idempotency, cancellation, timeout and error semantics. Executor choice is implementation detail.

## 17. Error model

Classify errors: InvalidInput, SchemaMismatch, Unauthorized, PolicyDenied, ApprovalRequired, CapabilityExpired, CapabilityRevoked, BudgetExceeded, ReplayDetected, Conflict, LeaseLost, TransientProvider, PermanentProvider, ToolRejected, EffectOutcomeUnknown, ProofRejected, UnsupportedProfile, IntegrityFailure, StorageUnavailable, Cancelled, Timeout and InternalInvariantViolation.

Retry depends on error and effect class. Authorization, policy, replay, schema, proof and integrity failures are not blindly retried. Unknown irreversible effects park.

## 18. Security invariants

1. No model output is an authority token.
2. No agent grants itself or another broader authority.
3. Every protected effect passes through gateway.
4. Gateway rechecks authority at execution.
5. Policy uncertainty denies or parks.
6. Child capabilities are attenuated.
7. Every effect has explicit class and idempotency/reconciliation contract.
8. State applies at most once per task identity/committed step.
9. Merge is independent of completion order.
10. Verifier does not trust prover/runtime.
11. Proof receipt binds exact relation, statement, profile and verifier.
12. Evidence never conflates proof, signature, approval, assertion and execution.
13. Tenant/memory boundaries enforced before retrieval and output.
14. Context admission is bounded and recorded.
15. Resource reservations precede dispatch and are reconciled.
16. Schema/journal/evidence corruption quarantines.
17. External protocol messages are untrusted until validated.
18. Revocation and policy epochs checked at effect time.

## 19. Required validation

**Determinism:** permute completion orders; test reducer laws; reject missing/duplicate/unexpected results; compare checkpoint IDs.

**Durability:** crash before dispatch, after provider response, after journal, during merge/commit; lease expiry; stale worker; migration/refusal.

**Authority:** prompt injection; forged/expired/revoked/wrong-task capability; escalation; replay; concurrent budget race; approval replay/expiry/wrong role; gateway bypass; policy outage.

**Effects:** duplicate idempotency key; timeout before/after downstream success; compensation failure; ambiguous irreversible outcome; restart reconciliation.

**Agent quality/provenance:** contradictory agents; correlated quorum; malformed output; fabricated citations; poisoned memory; cross-tenant retrieval; challenge outcomes; model-routing regression.

**Proof/evidence:** altered statement/proof/relation/profile; noncanonical encoding; proof for wrong public input; valid proof but policy denied; valid policy but proof invalid; evidence truncation/substitution/reordering; offline verification.

## 20. Delivery phases

- A0 Contracts: types, state, artifacts, graph and lifecycle. Exit: contract/schema tests.
- A1 BSP kernel: frontier, journal, reducers, deterministic checkpoint. Exit: permutation/fault tests.
- A2 Durable store: SQLite, atomic commit and recovery. Exit: crash matrix.
- A3 Authority: identity, capability, policy, approval and gateway. Exit: no bypass in adversarial mock-tool tests.
- A4 Computation: relation registry/evaluator. Exit: reproducible vectors.
- A5 Proof: experimental prover and separate verifier. Exit: independent rejection of altered inputs.
- A6 Adapters: one model provider, MCP and A2A. Exit: provider-independent runtime contract.
- A7 First vertical: synthetic QALABASH R_spend and mock settlement. Exit: portable evidence and offline verification.
- A8 Production qualification: external crypto/security review, multi-worker store, recovery and deployment security case.

No phase is complete merely because crates compile.
