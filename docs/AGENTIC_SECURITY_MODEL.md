# aQiten agent security model (rev. 0.1)

## 1. Threat model

Assume:
- model output may be wrong, manipulated or adversarially influenced;
- retrieved documents and tool responses may contain prompt injection;
- agents may be compromised or behave outside their assigned task;
- delegated agents may collude or return fabricated results;
- network messages may be delayed, duplicated, replayed or reordered;
- credentials may leak;
- tool adapters may be buggy;
- event logs may be incomplete or altered;
- an honest proof can establish only the exact relation it encodes.

The security design must not rely on an LLM correctly recognising malicious instructions.

## 2. Authority objects

A capability should bind at least:
- issuer identity and subject identity;
- task and tenant/workspace;
- permitted action identifiers;
- resource scope;
- maximum spend/compute/time budgets;
- not-before and expiry;
- delegation depth and allowed delegate set;
- nonce or unique identifier;
- policy version and revocation epoch.

Capabilities must be authenticated. The verifier must check every field and current revocation/policy state. A valid signature is necessary but not sufficient for execution.

## 3. Delegation

Delegation is attenuation:
- child actions are a subset of parent actions;
- child resources are a subset of parent resources;
- child budgets do not exceed the remaining parent budget;
- child expiry is no later than parent expiry;
- child delegation depth is lower;
- child is bound to a specific task and delegate identity.

No agent may mint a capability. Only a configured authority service may issue the root capability. Delegation must be auditable and revocable.

## 4. Effect gateway

Every protected side effect passes through one deterministic gateway. Examples include sending money, changing records, deploying code, sending external messages, changing access controls or invoking privileged APIs.

At execution time the gateway checks:
1. authenticated principal and agent identity;
2. capability authenticity and task binding;
3. action and resource scope;
4. budget and rate limits;
5. expiry and revocation;
6. nonce/replay status;
7. deterministic policy;
8. human approval where required;
9. request schema and resource limits.

If any mandatory check is unavailable or fails, the operation is rejected. The agent cannot call the protected connector directly.

## 5. Proofs and evidence

Keep the following separate:

| Evidence | Establishes | Does not establish |
|---|---|---|
| Agent/model statement | What the agent asserted | Truth |
| Digital signature | Control of a signing key over a message | Correctness or permission |
| Capability | Bounded authority if valid and accepted by policy | Correctness of computation |
| Gateway record | What the gateway reports it executed | Truth of external source data |
| Execution attestation | A defined execution environment made a statement | General correctness outside its attestation model |
| ZK proof | A specified relation holds for public inputs | That the relation is the right policy or data is truthful |
| Verification receipt | A verifier accepted a specific proof/profile | Correctness of the whole business workflow |

A proof-carrying result must bind a canonical public statement, relation identifier/version, public inputs, proof bytes, parameter profile, transcript/protocol version and relevant evidence references.

## 6. Required adversarial tests

- prompt injection requests a privileged action;
- agent asks a peer to expand its capability;
- child capability exceeds parent resource/action scope;
- expired or revoked capability;
- duplicate nonce and replayed tool request;
- altered proof envelope or non-canonical encoding;
- valid proof for the wrong relation identifier;
- valid proof with mismatched public inputs;
- gateway unavailable or policy timeout;
- tool returns malicious content;
- agent reports success when the gateway rejected the action;
- evidence record is reordered, truncated or substituted;
- LLM provider returns malformed tool arguments;
- memory from one tenant/task is presented to another.

## 7. Security invariants

- No model output is an authority token.
- No tool adapter bypasses the effect gateway.
- No proof producer is its own trusted verifier.
- No verifier imports agent or model code.
- No child capability exceeds its parent.
- No protected action executes on policy uncertainty.
- No evidence format silently upgrades an assertion into a proof.
- No cryptographic security claim is inferred from agent evaluation scores.
