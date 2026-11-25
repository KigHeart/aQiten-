# Applying AJENTIR runtime principles to aQiten

**Prepared for:** Tilil Engineering Group, the aQiten Team  
**Status:** normative design adaptation; no code is copied and no runtime dependency is introduced.

## 1. Why AJENTIR matters

AJENTIR is a separate Rust-native runtime project. Its architecture baseline centres on durable, cost-aware, model-agnostic execution using a bulk-synchronous-parallel (BSP) super-step. The important idea is not a particular agent API. It is that progress, state reduction, checkpointing, recovery, routing and cost accounting share one explicit unit of progress.

aQiten should use this architectural insight. It should not become an AJENTIR fork, add AJENTIR as an implementation dependency, or duplicate every AJENTIR crate. Both projects remain independent products with distinct repositories and contracts.

## 2. Concepts to adopt

| AJENTIR concept | aQiten adaptation | Why |
|---|---|---|
| BSP super-step | Atomic unit of graph progress | Same-step sibling writes are not partially visible |
| Frozen frontier | Immutable set of runnable nodes | Scheduling and expected completion are explicit |
| Immutable task input view | Per-node snapshot of committed state/artifacts | Prevents race-dependent context |
| Typed channels | Versioned aQiten state channels | Avoids ambiguous shared mutable state |
| Reducer contracts | Deterministic reducers with canonical fold order | Concurrent outputs converge |
| Checkpoints | Content-addressed checkpoints with parent links | Durable resume and inspectable history |
| Task result journal | Durable results keyed by run/step/node/attempt | Avoids needless repeated work |
| Fork-with-edit | Branch a prior state for review/counterfactual execution | Auditable what-if analysis |
| Cost ledger | Durable model/tool/compute/proof/storage/human costs | Cost is a scheduler input |
| Admission policy | Deterministic context selection | Controls token cost and data exposure |
| Schedule simulator | Enumerate completion orders and fault schedules | Tests convergence |
| Effect intent journal | Persist non-pure intent before call | Prevents blind irreversible retries |

## 3. aQiten super-step semantics

1. Freeze active frontier, checkpoint ID, schema version and policy epoch.
2. Compile immutable per-node inputs from declared channels and artifact references. Apply tenant, provenance, context and cost policies.
3. Reserve worst-case token, tool, CPU, memory, proof and wall-clock budgets in stable node-ID order.
4. Execute independent nodes concurrently. A node may be an LLM agent, external agent, deterministic function, tool, relation evaluator, prover or verifier.
5. Persist results, usage, effect intents/results and errors. Each result binds run, step, node, attempt, input digest and implementation version.
6. Require exactly one terminal record or explicit parked/failure resolution for every frozen frontier member.
7. Validate schema, provenance, resource bounds, capability use and effect declaration.
8. Apply state writes using declared reducers in canonical node identity order.
9. Derive next frontier from merged state and validated results. Model routing is a proposal, not authority.
10. Compare-and-swap checkpoint head and fencing epoch; atomically persist state, frontier, ledger and evidence references.

A node MUST NOT observe sibling writes from its current super-step. Dependent work sees outputs only after barrier and commit. Initial implementation should prefer strict barriers over streaming dependencies.

## 4. Agent outputs are first-class inputs

An agent result is a typed artifact, not merely a chat message. It carries artifact ID and content digest, schema/version, producer identity/version, run/step/node/attempt, input artifact references, status, quality metadata with origin, provenance/trust classification, cost/execution metadata and optional proof/evidence references.

Downstream nodes declare accepted artifact types and versions. They can consume, compare, challenge, combine, transform or escalate outputs. Disagreement is not silently converted to truth.

Explicit fan-in policies:
- AllRequired: every input must complete;
- AnyAccepted: any acceptable result suffices;
- Quorum(k,n): at least k accepted results;
- BestUnderEvaluator: select using a versioned deterministic evaluator;
- ConsensusRequired: require a declared agreement predicate;
- HumanAdjudication: park for authorized review.

These are workflow acceptance policies, not truth guarantees. Multiple agents sharing a model/provider/prompt/data are correlated; do not describe them as independent witnesses without evidence.

## 5. Cost-aware scheduling

The ledger tracks model input/output/cache tokens, requests, tool calls and charges, CPU/wall time, memory/storage, proof generation/verification, network and human review.

Reserve worst-case budgets in stable node order; reconcile to actual usage after completion. Ledger values are monotonic; corrections are explicit adjustment events.

Model selection first satisfies a task-class quality threshold. The scheduler may minimize expected cost plus provider-switch/cache penalty and expected recovery cost subject to hard limits and minimum quality. A cheaper model is not selected if measured quality is below threshold. Record model and price manifest versions.

## 6. Crash and effect semantics

A result is reusable only if request digest, node implementation version, provider/model configuration and input digests match.

Effects:
- Pure: no external side effect.
- Idempotent: repeat-safe with stable downstream idempotency key.
- Compensable: registered inverse exists; not guaranteed to restore the world.
- Irreversible: no automatic retry after uncertain outcome.

For every non-pure effect, persist intent before call. On resume, return completed result; reconcile idempotent calls using same key; park ambiguous irreversible effects. A conforming gateway must not dispatch without durable intent.

Do not claim universal exactly-once external execution. Target exactly-once state application inside aQiten plus connector-specific idempotency/reconciliation.

## 7. What aQiten adds

AJENTIR contributes execution semantics. aQiten adds:
1. Authority kernel: identity, capabilities, attenuation, revocation and policy receipts.
2. Effect gateway: sole protected side-effect boundary, with checks immediately before execution.
3. Formal computation jobs: relation IDs, versions, public/private input schemas and deterministic evaluators.
4. Proof lifecycle: profile selection, witness handling, proving, independent verification and receipts.
5. Evidence graph: links among agent artifacts, statements, proofs, approvals, effects and checkpoints.
6. Independent verifier: no model, orchestration, tool or prover imports.
7. Its own lattice PCS/ZK research track and mathematical gates.

BSP determinism is not cryptographic soundness. A checkpoint digest does not prove a computation. A proof does not prove policy correctness. Each mechanism has a distinct claim.

## 8. Recommended crate mapping

| Crate | Role |
|---|---|
| aqiten-agent-core | IDs, task/result/artifact types and lifecycle |
| aqiten-agent-state | Typed channels, reducers, schemas and snapshots |
| aqiten-agent-store | Checkpoint, task/effect journal and artifact store traits |
| aqiten-agent-kernel | Frontier, barrier, deterministic merge and checkpoint transition |
| aqiten-agent-scheduler | Admission, resource reservation, dispatch and routing |
| aqiten-agent-runtime | Run lifecycle, recovery, cancellation, park/resume and leases |
| aqiten-agent-protocol | Canonical task/result/artifact envelopes |
| aqiten-agent-capability | Issue, validate, attenuate and revoke |
| aqiten-agent-policy | Deterministic policy and receipts |
| aqiten-effect-gateway | Protected side-effect execution |
| aqiten-agent-compute | Relation job submission and deterministic execution |
| aqiten-agent-proof | Prover/verifier service contracts and binding |
| aqiten-agent-evidence | Evidence graph, event chain and export |
| aqiten-agent-memory | Scoped memory and retrieval policy |
| aqiten-agent-llm | Model adapters |
| aqiten-agent-mcp | MCP adapter |
| aqiten-agent-a2a | A2A adapter |
| aqiten-agent-telemetry | OTel and observability exporters |
| aqiten-agent-eval | Offline/adversarial evaluation harness |

This is a target map, not a request to create empty crates. Split crates only when a stable contract justifies it.

## 9. Interoperability

First integration is conceptual, not a crate dependency. If cross-project integration is useful, define a versioned interchange protocol around task identity, typed inputs, result artifacts, provenance, budget/usage, run/step correlation, status, failure class and optional evidence references. Never transmit raw signing keys.

AJENTIR must remain free to execute without aQiten cryptography. aQiten must remain free to execute without AJENTIR. A protocol adapter can connect them without coupling internal state models.

## 10. Implementation order

1. Canonical state, artifact and checkpoint semantics.
2. Deterministic reducers and BSP commit kernel.
3. Durable task/effect journal and SQLite store.
4. Capability and policy kernel; mock effect gateway.
5. Deterministic computation jobs and relation registry.
6. Proof service interfaces and independent verifier process.
7. Scheduler budgets and recovery.
8. LLM/MCP/A2A adapters last.
9. Postgres/worker fleet only after store conformance and lease-fencing tests.

## 11. Source

AJENTIR repository: https://github.com/KigHeart/ajentir  
Architecture baseline: https://github.com/KigHeart/ajentir/blob/main/docs/architecture/AJENTIR-V0.2-BASELINE.md

AJENTIR evolves independently. Revalidate its implementation status before treating any design item above as implemented there or in aQiten.
