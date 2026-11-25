# aQiten platform dependency and trust flow

This is the platform-level diagram. The original `crate_dag.png` depicts only the cryptographic crate DAG.

```mermaid
flowchart TB
  subgraph INT["Intelligence plane — untrusted proposals"]
    LLM["LLM / model adapters"]
    ORCH["Agent orchestrator"]
    RT["Agent runtime"]
    MEM["Scoped memory"]
    TOOLS["Typed tool adapters"]
    LLM --> ORCH
    MEM --> ORCH
    ORCH --> RT
    RT --> TOOLS
  end

  subgraph AUTH["Authority plane — deterministic enforcement"]
    ID["Identity"]
    CAP["Capability issuer / verifier"]
    POL["Policy engine"]
    DEL["Delegation attenuation"]
    GATE["Effect gateway"]
    ID --> CAP
    CAP --> DEL
    DEL --> POL
    POL --> GATE
  end

  subgraph PROOF["Computation and proof plane"]
    REL["Versioned relations"]
    COMP["Deterministic computation / compiler"]
    PROVER["aQiten prover / PCS"]
    VER["Independent verifier"]
    REL --> COMP
    COMP --> PROVER
    PROVER --> VER
  end

  subgraph EVID["Evidence plane"]
    ENV["Canonical proof envelope"]
    LOG["Authenticated event / provenance records"]
    RECEIPT["Verification and policy receipts"]
    ENV --> RECEIPT
    LOG --> RECEIPT
  end

  ORCH -. "proposes plan" .-> CAP
  TOOLS -->|"requests protected action"| GATE
  GATE -->|"authorised operation"| TOOLS
  RT -->|"submits relation job"| COMP
  VER --> ENV
  GATE --> LOG
  RT --> LOG
```

## Trust interpretation

- The dotted plan edge is a request, not a grant.
- The capability and policy path is deterministic and independent of model judgement.
- The effect gateway is the only path for protected side effects.
- The proof verifier is independent of the agent runtime and prover.
- The evidence plane records different claim types separately; it does not turn signatures or logs into proofs.
- This is a target architecture, not a deployed system.
