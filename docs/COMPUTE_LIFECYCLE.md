# aQiten compute lifecycle

**Version:** 0.1.0-draft  
**Authoring organisation:** Tilil Engineering Group, the aQiten Team  
**Purpose:** define the complete path from caller request to committed, independently verifiable outcome and any authorized external effect.

## 1. Lifecycle overview

aQiten treats a run as a durable computation, not a long chat session. A run is a graph of typed tasks. Some nodes use probabilistic agents; others are deterministic functions, tools, relation evaluators, provers, verifiers or human approval waits. The runtime advances the graph through BSP super-steps. Each completed node emits a typed artifact and journal record. State becomes visible only after deterministic barrier, merge and checkpoint commit.

Complete lifecycle:
1. Submit request.
2. Authenticate caller and establish tenant.
3. Normalize request and create run.
4. Validate task contract and graph.
5. Propose plan.
6. Validate plan and resource envelope.
7. Issue root capability and bind to run.
8. Freeze first frontier.
9. Admit context and immutable inputs.
10. Reserve cost/resource budgets.
11. Execute agents and deterministic workers.
12. Validate and journal results.
13. Reach barrier and resolve incomplete work.
14. Merge writes deterministically.
15. Route next frontier.
16. Execute formal relation.
17. Generate proof if required.
18. Verify proof independently.
19. Evaluate final policy and approval.
20. Prepare effect intent.
21. Execute/reconcile protected effect.
22. Commit final checkpoint.
23. Export evidence graph and verification package.

LLM, proof and external effect are optional nodes. The lifecycle works without any of them.

## 2. Stage-by-stage lifecycle

### Stage 0 — Request submission

TaskRequest includes authenticated principal context, tenant, objective/class, input artifact references or bounded payload, required output schema, provider constraints, quality threshold, cost/token/compute/wall-clock limits, data sensitivity/retention, requested effects, proof requirements and idempotency key.

Natural-language objective is intent, not executable instruction. Record canonical request digest. Reject malformed, oversized, unsupported or unauthenticated requests before dispatch.

### Stage 1 — Authentication and tenant binding

Authenticate caller. Resolve tenant, principal, roles and policy context. Where deployed, resolve workload identity via SPIFFE/SPIRE or equivalent.

Persist PrincipalContext: principal ID/type, tenant ID, auth method/credential reference, attributes, trust domain, policy epoch and timestamp source. Secrets/bearer credentials never enter agent context.

### Stage 2 — Run creation

Allocate RunId and initial checkpoint binding request digest, tenant/principal, runtime version, schema version, unplanned graph status, initial ledger, retention policy and evidence genesis record.

Persist before model planning. Retry with same caller idempotency key returns existing run; different request digest is a conflict.

### Stage 3 — Plan proposal

Planner may propose decomposition, specialists, deterministic operations, schemas, dependency edges, fan-in policies, candidate relation, requested tools/effects, proof requirement, quality and cost. Output is untrusted and parsed into graph IR.

Planner cannot mint capabilities, alter policy, choose unapproved cryptographic profiles, call protected tools or self-verify.

### Stage 4 — Graph validation and plan admission

Deterministic validator checks unique IDs, supported node kinds/versions, acyclic per-step dependencies, schema compatibility, required input producers, fan-in semantics, bounded loops, retry/timeouts, effect declarations, relation/profile availability, tenant scope, resource/cost envelope and policy restrictions.

Plan gets content digest and immutable version. High-risk plan may require human/policy approval before execution.

### Stage 5 — Capability issuance

Authority evaluates admitted plan and issues root capability limited by node/action classes, resource selectors, budgets/rates, validity, tenant/run, delegation depth/delegate classes and approval roles. Plan proposal does not create permission.

### Stage 6 — Frontier freeze

Kernel freezes runnable frontier. Each entry binds node ID, implementation digest, attempt, immutable input digest, capability reference, resource reservation and deadline. No node is silently inserted after freeze; plan revision creates a new transition.

### Stage 7 — Context and input admission

For each agent, construct immutable InvocationInput from caller inputs, upstream artifacts, permitted channels, selected memory, policy/task metadata, tool manifests and prior summaries.

Admission is versioned and deterministic. Enforce tenant/scope, trust, provenance, sensitivity/redaction, token/byte budgets, stable prompt prefix, retrieval and summary rules. Model never receives capability secrets or signing keys.

### Stage 8 — Resource reservation

Reserve worst-case tokens, model requests, tool calls/charges, CPU/memory/wall time, proof resources, storage/network and human review in stable node-ID order. If reservations exceed budget, reject, reduce concurrency or request explicit best-effort decision. Never silently overspend.

### Stage 9 — Node execution

Dispatch eligible nodes concurrently: LLM agent, external A2A agent, MCP/native tool, deterministic function, transform, relation evaluator, prover, verifier or approval wait.

Each runs against frozen input. It cannot see sibling writes from same step. Model request identity includes provider/model manifest, request digest, prompt/template version, tool schemas and sampling/config.

Journal completed provider response before reusable recovery state.

### Stage 10 — Result validation and journaling

Validate run/step/node/attempt binding, input digest, output schema/version, payload size/canonical encoding, provenance, resource usage, effect class and required signatures/attestations. Reject undeclared side effects.

Create immutable TaskResultRecord with output refs, usage, error status, effect refs and trace correlation. Agent cannot report protected action success without gateway receipt.

### Stage 11 — Barrier

Each frozen node must reach allowed terminal state: completed, accepted partial, abstained, failed, parked for approval/reconciliation or cancelled. Missing result is not completion. Duplicate/unexpected results reject. A parked state can be committed only explicitly.

### Stage 12 — Deterministic merge

Verify exact frontier coverage. Sort writes by stable node ID and write ordinal. Apply typed reducers, reject unknown channels/type mismatch, compute state digest, reconcile resource reservations, append cost ledger and attach artifact/evidence refs.

Completion order cannot affect checkpoint. On failure parent checkpoint remains authoritative.

### Stage 13 — Routing

Derive next frontier from committed state, graph, validated results, policy and budget. Model may propose route, but deterministic validation/policy decides. Loops use new super-steps with bounded iteration counters. Enforce maximum depth, attempts and total cost.

### Stage 14 — Formal computation

Submit ComputationJob. Resolve immutable relation version; validate public/private schemas; canonicalize inputs; check domain/range constraints; bind evaluator version; compute deterministically; emit ComputationRecord with input/output digests and execution metadata.

Do not infer relation from natural language at execution time. Model may suggest; registry and policy approve exact version.

### Stage 15 — Proof generation

When supported, prepare public statement/witness commitment; select approved profile; invoke bounded prover; produce canonical proof bytes; bind to statement, relation version, transcript/protocol and parameter profile; create ProofEnvelope.

Prover is untrusted to verifier. Proof generation failure is computation failure, not authorization decision. Experimental lattice backend MUST be labelled experimental and cannot satisfy production proof policy unless deployment explicitly accepts research risk.

### Stage 16 — Independent verification

Separate verifier checks canonical encoding, supported relation/profile, statement digest, proof/transcript, version compatibility and mathematical equations.

VerificationReceipt binds accepted/rejected status, statement/proof digests, relation/version, profile, verifier build/version, timestamp source and error code. It says this verifier accepted this proof under this profile; it does not authorize effects.

### Stage 17 — Final policy and approval

Before protected effect, gateway evaluates exact request. Policy may require valid capability, current revocation, required proof receipt, relation/version, amount/resource ceilings, human role approval, separation of duties and fresh source confirmation.

Approval binds run/step/node, effect request digest, amount/resource, expiry, role and single-use nonce. Any change invalidates approval.

### Stage 18 — Effect intent and execution

Gateway persists intent before dispatch. It binds effect ID, request digest, capability/policy/approval IDs, proof receipt, idempotency key, connector/version and effect class.

Adapter returns Succeeded, Rejected, FailedKnownNoEffect or OutcomeUnknown. Unknown is not success and is not automatically retried for irreversible effect. Park for reconciliation.

### Stage 19 — Final commit

After required nodes, proofs, approvals and effects resolve, commit final checkpoint with compare-and-swap against expected head and fencing epoch.

Completed requires all acceptance predicates. If caller permits lower assurance/incomplete result, BestEffort is possible with omissions recorded.

### Stage 20 — Evidence export

Export request digest, plan/graph, runtime/node manifests, agent artifacts, input lineage, cost ledger, capabilities/delegation, policy decisions, approvals, effect intents/results, relation/computation, proof, verification receipt, checkpoint chain, signatures and trust roots.

Do not export private witness, secrets or restricted memory by default. Include redaction declarations and references for withheld data.

## 3. Example: QALABASH R_spend

Use synthetic data and mock settlement.

1. Caller submits private spend-validation task with public policy inputs and private witness.
2. Runtime authenticates caller and binds tenant/run.
3. Planner proposes data preparation, policy interpretation, relation evaluation and proof tasks.
4. Graph validator resolves approved R_spend version.
5. Authority grants narrowly scoped node capabilities.
6. Data agent emits normalized typed artifact.
7. Policy-analysis agent emits interpretation artifact; it is an assertion, not authority.
8. Deterministic evaluator validates against approved relation and policy inputs.
9. Prover creates proof for exact statement.
10. Separate verifier validates.
11. Gateway checks capability, proof receipt, spend ceiling, approval and mock target.
12. Gateway persists effect intent and calls mock settlement.
13. Connector returns result with idempotency key.
14. Runtime commits checkpoint/evidence.
15. Offline verifier validates proof, signatures, artifact digests and checkpoint chain.

Negative tests: false statement rejects; correct proof but wrong amount/resource is denied; proof for wrong relation version rejects; expired capability denies; replayed effect rejects or returns prior idempotent result; timeout after possible downstream success parks; agent success claim without gateway receipt rejects; altered evidence fails digest/signature validation.

## 4. Sequence diagram

~~~mermaid
sequenceDiagram
    autonumber
    participant C as Caller
    participant R as Runtime
    participant P as Planner
    participant A as Authority
    participant S as Scheduler/Kernel
    participant W as Agent/Worker
    participant D as Relation Engine
    participant Z as Prover
    participant V as Independent Verifier
    participant G as Effect Gateway
    participant X as External Connector
    participant E as Evidence Store

    C->>R: TaskRequest + idempotency key
    R->>A: Authenticate principal / tenant
    A-->>R: PrincipalContext
    R->>E: Persist Run + initial checkpoint
    R->>P: Plan from immutable request
    P-->>R: PlanProposal + typed graph
    R->>S: Validate graph and freeze plan
    S->>A: Request root capability
    A-->>S: Capability grant
    S->>E: Persist frontier + reservations
    par Independent nodes
        S->>W: Execute against immutable inputs
        W-->>E: Journal artifacts and results
    end
    S->>S: Barrier and deterministic merge
    S->>D: Submit versioned relation job
    D-->>Z: Canonical statement + witness
    Z-->>V: ProofEnvelope
    V->>V: Independent verification
    V-->>E: VerificationReceipt
    S->>G: EffectRequest + proof receipt
    G->>A: Recheck capability/policy/approval
    A-->>G: Allow, deny or park
    G->>E: Persist EffectIntent
    G->>X: Execute with idempotency key
    X-->>G: Effect outcome
    G->>E: Persist EffectResult
    S->>E: Final checkpoint + evidence graph
    R-->>C: Run status + evidence reference
~~~

## 5. Failure and recovery matrix

| Failure point | Recovery | Must not happen |
|---|---|---|
| Before run commit | Retry same idempotency key | Duplicate logical run |
| During planning | Reuse matching journaled response or bounded retry | Silent plan change |
| Provider response before journal | Call may repeat; record uncertainty/cost | Claim exact replay |
| After result journal | Reuse if input/node digests match | Unneeded repeat |
| Before barrier commit | Rebuild barrier from journal | Merge partial frontier |
| During merge/commit | Rollback or CAS retry from same parent | Partial visibility |
| Lease expiry | Fence stale worker | Stale commit |
| Before effect intent | No effect should be dispatched | Unjournaled side effect |
| Intent without result | Reconcile; retry only under idempotency contract | Blind irreversible retry |
| After proof creation | Verify envelope independently | Treat prover success as verification |
| Verification timeout | Retry bounded; fail closed | Treat timeout as acceptance |
| Evidence corruption | Quarantine; verify ancestor | Export corrupted evidence |
| Policy unavailable | Deny or park | Fail open |

## 6. Lifecycle acceptance criteria

Ready for first demonstrator only when run creation is idempotent; graph validation deterministic; frontier immutable; completion order does not change checkpoint; every result input-bound and journaled; cost reserved/reconciled; restart does not reapply committed state; delegation cannot escalate; protected effects are gateway-only; unknown irreversible outcomes park; verification is independent; proof/policy/approval/effect remain distinct; evidence verifies offline; synthetic false R_spend and unauthorized mock effects are rejected.

## 7. Scope boundary

First implementation is single-process, SQLite-backed and mock-effect-only. Establish deterministic state and authority before distributed workers, browser automation, general-purpose sandbox, arbitrary-code proving, real settlement or production secrets. Multi-worker durability and external-effect correctness require separate conformance programmes.
