# Agentic systems product landscape and aQiten positioning

**Research snapshot:** 25 November 2026  
**Prepared for:** Tilil Engineering Group, the aQiten Team  
**Status:** strategic research; revalidate versions and claims before procurement or publication.

## 1. Executive conclusion

The agentic software market is a stack, not one category:

1. Agent SDKs/harnesses: model loops, tools, handoffs and structured outputs.
2. Graph and multi-agent orchestration: branches, joins, retries and human approval.
3. Durable workflow engines: persistence, recovery, timers and long-running execution.
4. Inter-agent and tool protocols: discovery and communication.
5. Identity, authorization and policy: principals, workload identity, permissions and decisions.
6. Tool gateways and sandboxing: mediated external access and isolated execution.
7. Memory, retrieval and context management: storing and selecting information.
8. Observability and evaluation: traces, debugging, scoring and monitoring.
9. Verifiable computation: formal claims and independent verification.
10. Domain applications: finance, software engineering, security and operations.

The credible strategy is not to reproduce all categories. aQiten should integrate with established products at the edges and own the runtime semantics and trust boundaries central to its proposition.

**Recommended position:** aQiten is a Rust-native, durable, verifiable agentic computation runtime. It coordinates heterogeneous agents and deterministic workers as a typed computation graph. Agent results are versioned, provenance-bearing artifacts and explicit inputs to later work. Deterministic super-step transitions make execution recoverable. Capabilities and policy constrain protected effects. Formal computations may produce independently verifiable proofs. Portable evidence preserves the distinction between assertion, computation, authorization, execution and proof.

This is a product hypothesis, not a claim that aQiten currently implements these capabilities.

## 2. Category map and product comparison

### 2.1 Agent SDKs and harnesses

| Product | Established strength | What aQiten should learn | Why not clone |
|---|---|---|---|
| OpenAI Agents SDK | Agent loop, tools, handoffs, guardrails, sessions and tracing | Small primitives and provider adapter boundaries | Model/tool ergonomics are not the moat |
| Anthropic Claude Agent SDK | Coding-agent harness, hooks, permissions, tools and subagents | Permission hooks and workspace integration | Provider-specific harness |
| Google ADK | Multi-agent composition, workflows, tools, sessions and deployment | Explicit agent composition | Google ecosystem integration is not the differentiator |
| Microsoft Agent Framework | Agents, harness agents, graph/functional workflows and middleware | Separate agent and workflow concepts | Microsoft ecosystem breadth is not the moat |
| AWS Strands Agents | Model-driven agents and AWS integration | Provider abstraction | AWS-first operating model |
| Pydantic AI | Typed Python agents, dependencies, structured output and tools | Strict schema validation at model boundaries | Python-first application framework |
| CrewAI | Role-based crews and flows | Accessible delegation concepts | Role labels do not create authority or proof |
| Agno | Agent runtime/platform, memory, tools and deployment | Self-hosted operational packaging | Broad platform surface |
| Mastra | TypeScript agents, workflows, memory and evals | Developer experience | TypeScript application framework |
| LlamaIndex Workflows | Event-driven, data-heavy workflows | Data-centric event processing | Retrieval pipelines are not the trust kernel |
| smolagents | Lightweight code/action agents | Minimal abstractions | Lightweight harness only |
| AutoGen | Conversational multi-agent patterns; Microsoft now directs new development to Agent Framework | Historical lessons | Not the new architecture baseline |

These products already make model calls, tool calls and delegation straightforward. aQiten should accept their outputs through adapters, not compete on prompt templates, role-playing, model selection UX or basic tool calling.

### 2.2 Orchestration and durable execution

| Product | Strength | Design lesson | aQiten boundary |
|---|---|---|---|
| LangGraph | Explicit graph state, checkpoints, interrupts and human-in-the-loop | State and recovery are first-class | Define own deterministic merge and evidence semantics |
| Temporal | Durable workflows, timers, retries and activities | Durable history and replay discipline | Optional integration; not source of proof semantics |
| Restate | Durable execution, journaling and workflows | Journal state transitions and effects | Durable history is not cryptographic proof |
| Hatchet | Durable task/workflow orchestration and concurrency | Queueing and worker lifecycle | Own typed result and proof contracts |
| Inngest | Event-driven durable functions | Event-triggered execution | Expose events and adapters |
| DBOS | Database-backed durable execution | Transactions anchor state transitions | Keep store backend replaceable |
| Microsoft Agent Framework Workflows | Functional and graph execution | Explicit workflow paths | Add BSP barriers and evidence commit semantics |

aQiten should borrow durable execution principles. It should not build a distributed workflow service before validating embedded runtime semantics. First define state, journal and store contracts; then add Postgres and worker-fleet support.

### 2.3 Interoperability protocols

| Standard | Standardizes | Does not establish |
|---|---|---|
| MCP | Tool/resource/prompt client-server protocol | Whether a call is authorized, safe or true |
| A2A | Agent discovery, task/message/artifact exchange | Whether an artifact is correct or sender authorized |
| OpenAPI / JSON Schema | HTTP interfaces and structured data | Runtime trust, delegation or proof |
| gRPC / Protobuf | Typed service communication | Business authorization or semantic correctness |

Implement protocol adapters at the boundary. The canonical internal task/result/evidence model must not be defined by MCP or A2A. Adapters convert external messages into aQiten envelopes, authenticate callers, apply policy, bound payloads and preserve provenance.

### 2.4 Identity, authorization and policy

| Product/standard | Primary role | aQiten approach |
|---|---|---|
| SPIFFE / SPIRE | Workload identity and short-lived verifiable identity documents | Integrate; do not invent a workload identity standard |
| OAuth 2.1 / OIDC | User/service authentication and delegated access | Use for external principals and provider authorization |
| OPA | General policy decision point and policy distribution | Offer adapter; retain deterministic policy interface |
| Cedar | Policy language and authorization model | Evaluate as optional policy backend |
| Cloud IAM | Cloud resource permissions | Preserve as outer control; not sufficient for task-scoped agent authority |
| Zanzibar-style authorization | Relationship-based authorization | Integrate where resource relationships matter |

Identity answers who is calling. Capability answers what bounded authority was delegated for this task. Policy answers whether this exact request is allowed now. The gateway enforces the decision. A valid signature is necessary but not sufficient for execution.

### 2.5 Tool gateways, sandboxing and security

The market includes MCP/API gateways, secure execution sandboxes, browser/computer-use controls, secrets brokers and prompt-injection filters. Examples include Docker MCP Gateway, Kong-style API gateways and sandboxes such as E2B or provider-hosted coding environments.

These are complementary. aQiten should define a narrow Tool Adapter interface and a mandatory Effect Gateway contract. It should not initially build a browser, shell, container runtime, secrets manager or general-purpose sandbox. Integrate existing isolation systems and ensure their calls remain bounded by aQiten capabilities.

Prompt-injection detectors are defence-in-depth, not the authorization boundary. The gateway must reject an unauthorized request even when semantic detectors consider it benign.

### 2.6 Memory, retrieval and context

Relevant products include LlamaIndex, LangChain retrievers, Letta, Zep, Mem0, Langfuse prompt/skill management, and vector stores such as pgvector, Qdrant, Weaviate and Milvus.

aQiten should not make a vector database its canonical state store. Distinguish execution state (typed, versioned and checkpointed), artifacts (immutable/versioned outputs), working context (selected material admitted to one invocation), long-term memory (separately scoped knowledge) and retrieval indexes (derived/rebuildable).

Every retrieved item needs origin, tenant, scope, version, trust classification and content digest. Retrieved text remains untrusted data.

### 2.7 Observability and evaluation

| Product | Strength | Integration |
|---|---|---|
| Langfuse | Open-source LLM observability, traces, prompts and evaluations | Export traces and prompt/model versions |
| LangSmith | LangChain tracing, datasets and evaluations | Optional trace exporter |
| Arize Phoenix | OTel-oriented tracing and evaluation | OTel/OpenInference integration |
| Braintrust | Experimentation, datasets and evaluations | Export run/evaluation records |
| W&B Weave | LLM tracing and evaluation | Optional exporter |
| Comet Opik | Tracing, evaluation and monitoring | Optional exporter |
| Datadog | Enterprise observability and LLM monitoring | OTel integration |
| AgentOps / Helicone / Galileo | Agent/LLM monitoring and evaluation | Adapters where useful |

Observability answers what happened and how the system behaved. It is not a durable correctness guarantee. Telemetry can be sampled, delayed, redacted or unavailable. The durable execution journal and evidence ledger are separate authoritative records.

### 2.8 Verifiable computation

| System/research | Contribution | Relevance and limitation |
|---|---|---|
| Akita | Lattice-based polynomial commitment direction and transparent-mode reference | Relevant research reference, not an agent runtime |
| Jolt / Lattice Jolt | Lookup-oriented zkVM and lattice-oriented proof research | Benchmark/design comparison; do not infer aQiten parity |
| LatticeFold+ and related folding work | Lattice-based folding/commitment constructions | Research baseline; assumptions and proof obligations are construction-specific |
| RISC Zero / SP1 and other zkVMs | Prove execution of supported program/machine models | Potential integration for execution claims; not interchangeable with aQiten PCS |
| Succinct / Nexus and proving platforms | Proving infrastructure and deployment | Potential backends or competitors depending scope |
| Authenticated Workflows (2026 research) | Cryptographic integrity and runtime policy enforcement across agent workflow boundaries | Close conceptual competitor; aQiten must differentiate through runtime semantics, independent formal computation and eventual PQ-oriented ZK proof track |

A proof is not a generic truth engine. It establishes that a specific encoded relation holds for specified public inputs under a specified protocol/profile. It does not establish source-data truth, business intent correctness or real-world execution.

## 3. Competitive synthesis

The market already contains strong solutions for model/tool loops, graph orchestration, durable workflows, protocols, identity, policy, memory, observability, sandboxed execution and verifiable computation.

The product opportunity is the integration contract:
1. Agent outputs are typed, provenance-bearing artifacts.
2. Agents consume other agents' results as explicit inputs, not shared conversational state.
3. BSP execution freezes each step's input view, executes independent nodes concurrently, validates completion, merges deterministically and commits a checkpoint.
4. Cost, quality, retry and resource budgets are execution state.
5. Authority is granted independently from planning and rechecked at the effect boundary.
6. Deterministic computation is separate from probabilistic reasoning.
7. Formal relations bind computational results to public inputs and versioned specifications.
8. An independent verifier checks proofs without importing the agent runtime or prover.
9. Portable evidence distinguishes assertions, artifacts, decisions, approvals, execution receipts, signatures and proofs.
10. The lifecycle works with no LLM, or with external agents connected through standard protocols.

## 4. aQiten position

**aQiten is a verifiable agentic computation runtime.** It coordinates heterogeneous agents and deterministic workers as a durable computation graph. It treats each worker result as a typed artifact with lineage, applies deterministic state and budget transitions at super-step barriers, mediates protected effects through least-privilege capabilities and policy, and binds selected computational claims to independent proof verification and portable evidence.

This is narrower and more defensible than claiming aQiten is a universal agent framework or complete autonomous AI security platform.

### Product surfaces
- Runtime SDK: agents, tasks, graph nodes, typed channels, reducers, budgets and hooks.
- Runtime engine: BSP execution, scheduling, checkpointing, recovery and reconciliation.
- Authority kernel: identity, capability, policy, approval and revocation.
- Effect gateway: sole protected side-effect boundary.
- Computation service: deterministic versioned relations, proving and independent verification.
- Evidence service: portable evidence graph and offline verification.
- Adapters: LLM, MCP, A2A, OPA/Cedar, SPIFFE, stores, telemetry and external provers.

### Claims aQiten must not make yet
- More secure than existing products without a matched benchmark.
- General prevention of prompt injection.
- Proof of every model thought or natural-language statement.
- Proof of data provenance or business correctness.
- Post-quantum security before reductions and parameter analysis are complete.
- Universal exactly-once effects against arbitrary APIs.
- Production readiness while workspace, verifier and gateway are unimplemented.

## 5. Source register

- OpenAI Agents SDK: https://openai.github.io/openai-agents-python/
- LangGraph: https://docs.langchain.com/oss/python/langgraph/overview
- Microsoft Agent Framework: https://learn.microsoft.com/agent-framework/overview
- Google ADK: https://google.github.io/adk-docs/
- CrewAI: https://docs.crewai.com/
- Pydantic AI: https://ai.pydantic.dev/
- AWS Strands Agents: https://strandsagents.com/
- Mastra: https://mastra.ai/docs
- LlamaIndex Workflows: https://docs.llamaindex.ai/en/stable/module_guides/workflow/
- Temporal: https://docs.temporal.io/
- Restate: https://docs.restate.dev/
- Hatchet: https://docs.hatchet.run/
- Inngest: https://www.inngest.com/docs
- DBOS: https://docs.dbos.dev/
- MCP: https://modelcontextprotocol.io/specification/
- A2A: https://a2a-protocol.org/
- SPIFFE/SPIRE: https://spiffe.io/docs/
- OPA: https://www.openpolicyagent.org/docs
- Cedar: https://docs.cedarpolicy.com/
- Langfuse: https://langfuse.com/docs
- Arize Phoenix: https://arize.com/docs/phoenix
- Braintrust: https://www.braintrust.dev/docs
- Authenticated Workflows: https://arxiv.org/abs/2602.10465
- Akita: https://github.com/LayerZero-Labs/akita
- Jolt: https://github.com/a16z/jolt
- LatticeFold+: https://eprint.iacr.org/2025/247

Vendor documentation describes vendor-supported functionality, not independent comparative validation. Attribute research claims and reproduce benchmarks before using them as aQiten performance/security claims.
