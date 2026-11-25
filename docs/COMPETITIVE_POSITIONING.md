# aQiten competitive positioning (rev. 0.1)

**Assessment date:** 2026-11-25  
**Purpose:** compare architectural scope and current maturity, not claim superiority.

## 1. Executive assessment

aQiten currently has no Rust workspace, implementation, benchmark suite, agent runtime or production proof verifier. It is not presently competitive with mature PCS implementations, zkVMs or agent frameworks on delivered functionality.

Its proposed differentiation is an integrated architecture for **verifiable agentic systems**:
- post-quantum-oriented lattice proof research;
- zero-knowledge computational claims;
- cryptographically scoped agent authority;
- deterministic policy and protected-effect mediation;
- portable proof-carrying results and evidence.

This is a product thesis to validate, not an established market position.

## 2. Direct cryptographic and proving-system comparisons

| Project | Primary layer | What it already offers | aQiten relationship |
|---|---|---|---|
| Akita PCS | Lattice polynomial commitment | A modular lattice PCS; publicly positioned for Jolt integration | Closest reference for aQiten's PCS research. aQiten must not claim better performance or security without matched benchmarks and proofs. |
| Jolt / Lattice Jolt | zkVM / proving system | A complete RISC-V-oriented proving system with Akita as a lattice PCS backend | Downstream integration target and a strong end-to-end benchmark. aQiten's ambition extends into agent authority/evidence, not just the VM. |
| LatticeFold+ | Lattice folding protocol | Research construction improving prover speed, verification circuit and proof length relative to LatticeFold | Direct research comparator for folding and lattice proof design; not an agent platform. |
| LaBRADOR / LUNA / Greyhound and related lattice ZK work | Lattice ZK arguments and PCS | Established research lines for compact lattice proofs and polynomial evaluation | Must be included in technical literature and benchmark reviews before selecting a construction. |
| FRI/STARK systems | Hash-based proof systems | Transparent, post-quantum-oriented proof architecture with mature implementations and ecosystems | Important alternative baseline; aQiten must compare full proof size, prover/verifier cost, assumptions and setup model. |

The 2026 PCS survey reports a persistent trade-off: lattice and hash-based post-quantum schemes tend to incur larger proofs and/or higher computation than pairing-based systems, while assumptions and setup models differ. Its synthesized performance figures are not a substitute for same-hardware benchmarks.

## 3. Agent framework and runtime comparisons

| Project/category | Primary strength | What aQiten should learn | Intended difference |
|---|---|---|---|
| OpenAI Agents SDK | Agent loop, tools, handoffs, guardrails, sessions and tracing | Developer ergonomics, integrations, tracing and evaluation | aQiten adds deterministic cryptographic authority and proof-carrying computational claims; it should integrate model providers rather than compete on model quality. |
| LangGraph | Stateful agent orchestration, durable execution, streaming and human-in-the-loop | Explicit workflow state, checkpoints and recovery | aQiten's differentiator is a proof/authority/evidence trust layer that can sit beneath or alongside orchestration. |
| MCP and A2A | Tool and agent interoperability protocols | Standardised connectivity and typed interfaces | Connectivity is not authority. aQiten should provide policy/capability enforcement at the effect boundary. |
| Authenticated-workflow research | Cryptographic integrity and policy enforcement across agent workflow boundaries | Authenticate tool/data/context boundaries and enforce intent | A close conceptual neighbour. aQiten must distinguish itself through independently verifiable computation and post-quantum ZK proofs, not claim exclusive ownership of cryptographic agent security. |

## 4. Where aQiten could be distinct

The intended combined workflow is:

1. An agent plans a task.
2. A capability authority grants least-privilege scope.
3. Delegated agents receive attenuated capabilities.
4. A deterministic computation is executed or compiled to a formal relation.
5. A proof establishes a precisely defined computational claim.
6. An independent verifier checks the proof.
7. An effect gateway separately decides whether the next action is authorised.
8. An evidence envelope records the proof, policy decision and execution attestation as distinct claims.

The combination is the product hypothesis. Each component has precedents; integration, assurance and usability are the challenge.

## 5. What would make the comparison credible

Before claiming technical leadership, publish reproducible results against relevant baselines:

### Cryptography
- exact security assumptions and reductions;
- setup model and trust assumptions;
- proof size and public parameter size;
- prover/verifier time and peak memory;
- asymptotic and measured scaling;
- complete transcript zero-knowledge evidence;
- QROM theorem scope;
- independent implementation and external audit.

### Agentic security
- capability forgery, escalation and replay tests;
- prompt-injection-to-tool-action evaluations;
- delegation attenuation tests;
- gateway bypass tests;
- policy fail-closed tests;
- provenance integrity and evidence substitution tests;
- task success and latency measured separately from security outcomes.

### Product integration
- integration with at least one external agent framework;
- one MCP tool path mediated by the effect gateway;
- independent proof verification in a separate process;
- proof/evidence envelope validated by a second organisation;
- synthetic-data financial demonstrator.

## 6. Positioning statement

**aQiten is being designed as a trust and verification layer for autonomous computation: agents coordinate work, deterministic authority controls actions, cryptographic proofs establish specified computational claims, and portable evidence lets independent parties verify the result.**

Do not position aQiten as:
- a better Akita before evidence;
- a complete zkVM before an execution model and compiler exist;
- a universal agent framework;
- a proof that an AI model is truthful;
- production-safe post-quantum infrastructure before the QROM and lattice security arguments are complete.

## 7. Sources for the living comparison

- LayerZero, Akita announcement (9 September 2026): https://layerzero.network/blog/introducing-akita
- JoltBook, Akita backend and current feature boundaries: https://jolt.a16zcrypto.com/how/akita.html
- Akita repository: https://github.com/LayerZero-Labs/akita
- a16z, Lattice Jolt announcement (9 September 2026): https://a16zfrontier.com/posts/article/lattice-snarks-jolt-post-quantum-faster/
- Boneh and Chen, LatticeFold+ (IACR ePrint 2025/247): https://eprint.iacr.org/2025/247
- Survey, Polynomial Commitment Schemes from Classical Constructions to Post-Quantum Directions (2026): https://www.mdpi.com/2410-387X/10/2/27
- OpenAI Agents SDK: https://developers.openai.com/api/docs/guides/agents/sdk
- LangGraph: https://www.langchain.com/langgraph
- Rajagopalan and Rao, Authenticated Workflows: A Systems Approach to Protecting Agentic AI (2026): https://arxiv.org/abs/2602.10465

This comparison should be updated when project versions, papers, deployments and benchmarks change.
