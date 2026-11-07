# Security and Operations Specification

Status: greenfield architecture proposal. This document defines required controls and testable gates; it does not assert that any control has been implemented or measured.

## 1. Scope and security invariants

The system consists of a custom mobile application, a TypeScript 7+ transport/API/control plane, a Rust core, and a deterministic WebAssembly (WASM) execution environment. Tailscale supplies network connectivity. The Rust core is the only authority for persistent state, transactions, event ordering, and capability decisions. TypeScript types, mobile validation, Tailscale identity, and WASM sandboxing are useful controls, but none is an independent authorization boundary.

The following invariants apply to every design and release:

1. A network route to the host is not permission to use the application.
2. Every authoritative mutation is a Rust-core transaction with a current, explicit capability decision.
3. Draft editing, publishing, and execution are separate privileges. Possession of one never implies another.
4. User logic receives no ambient network, filesystem, environment, clock, random, or secret access.
5. WASM output is untrusted input to the Rust core, not an authorized state mutation or host effect.
6. A published artifact is immutable and content-addressed. Execution selects an exact published version and a separately granted runtime capability set.
7. Session authentication does not create standing authority; revocation and capability changes affect subsequent commands and subscriptions.
8. Transport retries cannot create duplicate authoritative mutations. Deduplication is persistent and transactionally tied to the command result.
9. A committed state event, its deduplication record, and its audit record are atomic. If this cannot be guaranteed, the mutation is not committed.
10. Security and observability data must not silently become a secondary store of private UI content.
11. All external interfaces have bounded input, work, memory, queues, and retained state.
12. Failure of a non-authoritative component must not permit writes. The safe mode is read-only or disconnected, depending on host-integrity confidence.

Normative terms **must**, **should**, and **may** indicate requirement strength. Numeric limits below are conservative initial policy defaults, not measured capacity claims.

## 2. Trust boundaries

| Boundary | Data crossing it | Required treatment |
| --- | --- | --- |
| Mobile device to Tailscale network | Pairing, session, commands, subscriptions | Authenticate the application device and user independently of network reachability. Validate all payloads. |
| Tailscale network to TypeScript 7+ host | HTTP/WebSocket messages and connection metadata | Bind to intended Tailscale interfaces only. Never trust client-supplied identity headers. Apply connection and message limits before expensive work. |
| TypeScript to Rust core | Canonical commands, queries, capability requests | Treat TypeScript as a protocol adapter, not an authorization authority. Use a narrow typed API and schema validation at the Rust boundary. |
| Rust core to WASM worker | Published artifact, state input, typed action | Use a versioned deterministic ABI. Validate the artifact and enforce deterministic imports and resource limits. |
| WASM worker to Rust core | Candidate output and requested effects | Validate schema, size, state invariants, published-version binding, and every requested capability. |
| Rust core to persistent storage | State, events, snapshots, audit, deduplication | Use atomic transactions, integrity verification, durable-commit rules, and protected backups. |
| TypeScript to trusted plugins | Control-plane requests and responses | Treat plugins as host-trusted code. No draft or WASM publication flow may install one. |
| Host to telemetry/export systems | Metrics, logs, traces, audit checkpoints | Export is disabled by default. Redact before emission and apply an explicit allowlist. |

A compromised host administrator or Rust process can defeat application-level controls. The design limits lateral damage and provides evidence and recovery; it does not claim to protect data from a fully compromised, unlocked host.

## 3. Identity, pairing, and session authentication

### 3.1 Tailscale is not application authentication

Tailscale normally provides authenticated, encrypted WireGuard transport between enrolled nodes, plus tailnet policy. That does not, by itself, establish which human is using the mobile app, whether the device is permitted to use this application, or which application capability an action requires. Tailnet ACLs therefore remain a network-layer control, not a substitute for pairing, sessions, or Rust-core authorization.

A Tailscale node identity may be used as an additional enrollment signal only when the host obtains it from an authentic local Tailscale interface and binds it to the pairing record. Client-supplied node names, tailnet names, source addresses, or headers are insufficient proof. A tailnet device joining the network must not automatically become an application administrator.

### 3.2 Provider-neutral actor identity

Use stable, host-generated actor and device identifiers. Display names, email addresses, Tailscale names, and external subject claims are mutable attributes, never primary keys.

Support identity through adapters without requiring a specific provider:

- **Local owner identity, default:** a host-created owner approves pairing from an already trusted local console or existing owner session.
- **Additional local users:** an owner invites an actor and assigns an initial capability bundle with least privilege.
- **Tailnet-assisted enrollment:** optional, using verified Tailscale node information as one signal plus explicit host approval.
- **External OIDC or enterprise identity:** optional adapter. The Rust core maps a verified external subject to a local actor; external group names do not directly grant authority.

Bootstrap creates one owner credential through a physically or administratively trusted host path. If no owner exists, remote mobile pairing must remain disabled. Recovery of the last owner requires an explicit host-local recovery ceremony, not an unauthenticated network endpoint.

### 3.3 Paired-device credentials

Each mobile installation generates a device-specific asymmetric key pair. Ed25519 or P-256 is acceptable after platform support is verified; the algorithm and key attestation are versioned fields rather than hard-coded assumptions. Private keys should be non-exportable and protected by the platform keystore or secure hardware where available.

A device record contains at least:

- host-generated device ID and bound actor ID;
- public key and fingerprint;
- key algorithm and credential version;
- optional verified Tailscale node binding;
- creation, last-use, expiry, revocation, and replacement metadata;
- user-visible device name;
- device capability ceiling, which can only narrow the actor's effective capabilities.

Pairing uses a high-entropy, single-use, short-lived code created by the host and approved through an existing trusted path. The code is stored only as a hash, expires after at most 10 minutes, has one successful use, and is invalidated after five failed attempts. A new device proves possession of its private key by signing a canonical challenge containing the host ID, device ID, server nonce, expiry, and pairing purpose. Challenge replay, cross-host replay, and code reuse must fail.

Optional platform attestation may raise assurance but must not be required for basic operation. Attestation failure must not be silently treated as a successful identity proof.

### 3.4 Session establishment and use

A paired device exchanges a fresh signed challenge for a short-lived, high-entropy opaque session token. The session is bound to the actor, device, credential version, authentication strength, and a server-generated session ID. Only a hash of the token is retained by the host. Tokens appear in authorization headers or an equivalent protected protocol field, never in URLs, logs, UI documents, or analytics labels.

Recommended defaults are a 12-hour absolute session lifetime and a 30-minute idle timeout for privileged sessions, with silent cryptographic reauthentication while the device credential remains valid. These are policy values, not substitutes for revocation.

For every command, including each WebSocket message, the Rust core computes effective authority from current state:

`effective capabilities = actor capabilities ∩ device ceiling ∩ session ceiling ∩ artifact/runtime grant`

Subscriptions are reauthorized as new revisions are produced. A revocation or reduction of capability takes effect at a defined log sequence: later commands are denied and later matching events are not delivered. Already delivered data cannot be recalled.

Every mutation carries a unique command ID and idempotency scope. Challenges and nonces must not be reused as long-lived request signatures. Connection migration, reconnect, and IP changes require valid application session proof; source-IP stability is not a security assumption.

## 4. Least-privilege capability model

### 4.1 Decision point and structure

The Rust core checks capabilities at the transaction boundary and again for host effects requested by user logic. TypeScript may hide unavailable controls, and the mobile editor may guide the user, but neither can grant authority.

A capability contains:

- action;
- resource type and resource selector;
- optional constraints, such as maximum document size or permitted effect type;
- grantor and purpose;
- issuance and expiry metadata;
- optional delegation ceiling.

The default is deny. Grants should be resource-specific; global wildcards are reserved for the bootstrap owner and emergency recovery. Delegation must be attenuating: a delegate cannot gain an action, resource, constraint, or lifetime that the delegator lacks. Revoking a grant revokes dependent delegations.

### 4.2 Draft, publish, and execute split

Use distinct capability families:

| Capability | Permits | Explicitly does not permit |
| --- | --- | --- |
| `ui.read` | Read an authorized published UI projection | Reading another actor's private draft by default |
| `ui.draft.read` | Read an authorized draft | Editing, publishing, or executing it |
| `ui.draft.write` | Create or edit a declarative draft and private preview input | Publishing, installing plugins, or requesting live host effects |
| `ui.publish` | Turn a validated draft revision into an immutable published version | Executing it or granting its requested runtime capabilities |
| `ui.execute` | Invoke typed actions on an exact published version | Editing the draft, publishing a new version, or exceeding runtime grants |
| `ui.runtime.<effect>` | Request a narrowly scoped effect through the Rust broker | Direct network, filesystem, secret, or process access |
| `state.read` / `state.write` | Access explicitly scoped application state | Bypassing event-log and invariant checks |
| `device.pair` / `device.revoke` | Manage authorized device records | Expanding actor or runtime capabilities without the corresponding grant |
| `plugin.manage` | Perform the separate trusted-plugin administration flow | Installation through ordinary UI publication |
| `audit.read` / `recovery.manage` | Read authorized audit metadata or perform recovery operations | Unrestricted UI/state access unless separately granted |

Publishing records the artifact hash, schema and ABI versions, declared runtime requests, publisher, and source draft revision. Publication does not approve dangerous declarations. An authorized runtime grant is a separate decision, and its capability set is persisted with the published-version execution policy.

Draft preview runs with zero live runtime capabilities. It may use deterministic local or host-side simulation, but it cannot mutate authoritative state. `ui.execute` on a draft is invalid. A published version cannot be edited in place; a new draft and new publication are required.

## 5. Declarative UI and renderer protections

The UI document is data, not executable markup. The renderer supports only an allowlisted component and action vocabulary.

- Reject unknown schema versions and unsafe component types by default; forward-compatible unknown-field handling must be specified per field rather than universally ignored.
- Bound document bytes, node count, nesting depth, string length, collection size, asset count, and action count before rendering or publishing.
- Do not interpret arbitrary HTML, JavaScript, CSS expressions, native module names, deep links, or file URLs as UI content.
- Assets use typed, authorized references. Remote asset loading is disabled by default and, if later enabled, requires URL policy, size limits, content validation, and privacy review.
- A UI action carries only a typed action ID and validated arguments. It cannot name an arbitrary host function.
- Error fallback replaces an unsafe subtree or document without executing pending actions.
- The editor cannot visually disguise a different published artifact. Publish confirmation shows artifact identity and requested runtime capabilities in understandable language.
- UI updates must preserve accessibility semantics and must not turn data-controlled text into accessibility actions or privileged navigation.

Initial document limits are 1 MiB serialized size, 10,000 nodes, depth 64, 256 actions per document, and 16 KiB per user-visible string. Stricter component-specific limits still apply.

## 6. Deterministic WASM sandbox

### 6.1 Execution model

User logic is a pure, versioned transformation over explicit input. It returns candidate state operations, a UI projection or patch, and typed effect requests. The Rust core validates this output and performs any authoritative transaction. There is no direct host callback that can bypass this step.

The host ABI defines canonical serialization, numeric rules, buffer ownership, input/output versions, trap/error mapping, and the published-artifact hash. The logical input contains only declared state and action data. Host time, random values, environment data, network responses, and secrets are not implicit inputs. If a future effect needs such data, the Rust core obtains it under capability control and supplies it as an explicit, recorded input to a later deterministic invocation.

### 6.2 Import and feature policy

The initial profile permits only the minimal memory/result ABI needed to read input and return output. It denies:

- WASI and all unknown imports;
- network and socket access;
- filesystem and preopened-directory access;
- environment-variable access;
- clocks and monotonic timers;
- random-number imports;
- process, thread, and shared-memory operations;
- unreviewed SIMD, relaxed-determinism, or engine-specific features;
- floating-point instructions in the initial integer-only profile;
- automatic start functions with side effects or unreviewed initialization behavior.

Compilation caches are keyed by validated artifact hash, engine version, deterministic feature profile, and ABI version. A cache hit must never bypass artifact validation or policy checks.

Future floating-point support requires a separately versioned determinism specification and differential evidence across supported engines and architectures. Engine determinism must not be inferred merely because two runs on one machine agree.

### 6.3 Resource policy

These limits are mandatory initial defaults for untrusted user logic:

| Resource | Initial default | Required behavior on exhaustion |
| --- | --- | --- |
| Decoded module artifact | 1 MiB | Reject before compilation |
| Canonical invocation input | 256 KiB | Reject before execution |
| Canonical output | 256 KiB | Trap/reject; commit no partial output |
| Linear memory | 16 MiB maximum, non-shared | Trap/reject; no host allocation retry loop |
| Deterministic fuel | 5,000,000 units per invocation | Deterministic resource trap; commit no partial output |
| Wall-clock liveness watchdog | 100 ms per interactive invocation | Abort only the isolated worker; no partial commit |
| Compilation liveness watchdog | 5 seconds | Reject compilation and release worker resources |
| Typed effect requests | 64 per invocation | Reject excess output before any effect |
| Diagnostic guest output | 16 KiB per invocation | Truncate by counted bytes; never include secrets by default |
| Queued executions | 8 per device, with a bounded global queue | Apply backpressure and return a typed resource error |

Fuel and structural limits determine normal deterministic reducer outcomes. The wall-clock watchdog is a liveness backstop, not a logical clock input and not a substitute for fuel. A watchdog abort invalidates the uncommitted attempt and is audited; replay does not use elapsed wall time to calculate state.

Run untrusted compilation and execution in an isolatable worker with bounded queues. Engine traps, out-of-memory conditions, and worker death must not terminate the Rust core or corrupt another tenant's execution. Instance reuse is permitted only after proving complete reset of memory, globals, tables, fuel, output buffers, and diagnostics. Fresh-instance differential tests must produce the same canonical logical result.

The WASM sandbox is defense in depth, not the only boundary. Rust must still enforce state invariants, output schemas, capabilities, and transaction limits.

## 7. Trusted TypeScript plugin risk

TypeScript 7+ compile-time types do not sandbox runtime code. An in-process plugin may read process-accessible data, alter transport behavior, consume resources, or exploit dependencies. It is therefore fully trusted host code, not equivalent to deterministic user WASM.

Policy:

- Plugins are disabled by default and are not required for the thin vertical slice.
- Only an explicitly authorized host administrator can install one through a separate administrative flow.
- Packages require pinned versions, lockfile integrity, provenance review, a software bill of materials, and signature verification under the update trust policy.
- Remote code loading, dynamic evaluation, install-time network fetch, and mobile-driven plugin installation are prohibited by default.
- The plugin receives a narrow API, not unrestricted core storage access. Every requested state mutation still enters the Rust command and capability path.
- Third-party plugins should run out of process with least OS privilege, restricted filesystem/network access, bounded IPC, and independently revocable credentials. This reduces impact but does not make them untrusted or safe by assertion.
- Each plugin has an emergency kill switch and an audited enable/disable state. Disabling a plugin must not disable core recovery or audit verification.

A compromised in-process plugin can steal any secret available to that process and falsify pre-core observations. The threat model and release documentation must state this residual risk.

## 8. Audit integrity and tamper evidence

Maintain an append-only security audit chain in the Rust core. Each record includes a host log sequence, transaction ID where applicable, actor/device/session IDs, action, resource ID, capability decision, published artifact hash where applicable, result category, previous-record hash, and record hash. Host timestamps may aid operations but are not deterministic logic inputs and are not proof of event order.

For an authorized committed mutation, state changes, the event-log append, command deduplication result, and audit append commit atomically. Denied privileged commands and administrative actions are also recorded through an ordered audit path. Failure to append an audit record fails closed for publish, execute, plugin, identity, and recovery operations.

Tamper evidence requires more than a local hash chain:

- periodically sign a checkpoint over the latest audit head and state/log identity;
- protect checkpoint signing keys separately from ordinary application data;
- optionally export checkpoints to an independent append-only or off-host store;
- verify chain continuity on startup, on a schedule, before backup completion, and after restore;
- record retention deletion and checkpoint the retained head before truncating old records.

A local hash chain can detect accidental or unsophisticated modification, but a host administrator able to rewrite the entire chain and its keys can also rewrite history. Independent checkpoint export is the mitigation when stronger non-repudiation is required.

Audit payloads contain metadata and content hashes, not raw private UI text, action arguments, tokens, or secrets. Authorized investigators need a separate, audited process to correlate a hash with protected source content.

## 9. Secrets and storage protection

### 9.1 Secrets

- Prefer the host OS keychain/credential vault. Otherwise use an encrypted host-only secret file with restrictive permissions and a documented key-protection trade-off.
- Never embed secrets in declarative UI, WASM artifacts, snapshots exposed to clients, source repositories, logs, crash dumps, or metrics.
- Store session tokens and pairing codes as hashes. Store device public keys, not exported private keys.
- WASM receives secret references only as opaque typed effect parameters when absolutely necessary; the Rust broker resolves the reference without returning secret material to guest logic.
- TypeScript plugins receive only explicitly issued credentials and should not share the core's master keys.
- Support independent rotation of session-signing/verification material, checkpoint keys, storage keys, plugin credentials, and update trust roots. Rotation and revocation are audited.
- Backups require separate access control and encryption keys. A Tailscale identity alone must not decrypt a backup.

### 9.2 Storage encryption options

Choose per deployment rather than assuming Tailscale protects stored data:

1. **Default:** host full-disk encryption plus restrictive OS permissions. This protects powered-off storage but not a compromised running host.
2. **Stronger application-layer option:** encrypted database/object storage using envelope encryption, with data-encryption keys wrapped by a key in the host vault. Document performance, backup, search, and recovery implications.
3. **Backups and exports:** always separately encrypted and integrity-protected, including when application-layer database encryption is absent.

Encryption does not provide all integrity guarantees; retain authenticated encryption or explicit MAC/signature verification as appropriate. Secure deletion on flash, copy-on-write filesystems, and replicated backups is best-effort and must not be promised as immediate physical erasure.

Mobile caches are minimized, excluded from unencrypted platform backups where supported, and protected with platform storage/keystore controls. Revoking a device prevents future access but cannot erase data already copied off the device.

## 10. Transport encryption and exposure decisions

Base the TLS decision on the actual configured path, not on the presence of a Tailscale client:

| Actual path | Transport decision |
| --- | --- |
| Direct host/mobile communication over host Tailscale interfaces using Tailscale's WireGuard data path | Tailscale provides authenticated network encryption. Additional TLS is optional defense in depth, but application pairing, sessions, and capabilities remain mandatory. |
| Tailscale Serve terminating within the intended tailnet | Verify the Serve configuration and certificate path. No public Funnel exposure by default. Application authentication remains mandatory. |
| Any reverse proxy, relay, port forward, or route where Tailscale encryption of the complete path is not proven | Modern TLS with certificate validation is mandatory. Do not claim tailnet-only protection for that segment. |
| Tailscale Funnel or another public ingress | Disabled by default and out of scope until a separate public-threat review, hardened TLS, abuse controls, and explicit product approval are complete. |
| Non-Tailscale LAN or Internet fallback | Disabled by default. If enabled, require TLS and the same application authentication and authorization; never downgrade silently. |

Release verification must inspect effective listener and proxy configuration and test from an unauthorized network. Certificate validation must never be disabled to make development or mobile networking easier.

## 11. Rate limits, quotas, and backpressure

Apply limits per actor, device, session, resource, and globally. Network source may be an additional signal but not the sole identity. Use bounded token buckets and bounded work queues; return typed rate/resource errors with retry guidance. Do not silently drop accepted authoritative commands.

Initial policy defaults:

| Control | Initial default |
| --- | --- |
| Pairing failures | 5 per code; broader source/device throttling after repeated failures |
| Session authentication attempts | 10 per minute per device |
| New sessions | 6 per minute per device; 4 active sessions per device |
| Inbound WebSocket frame | 64 KiB, including bounded fragmentation |
| Command submissions | 20 per second, burst 40, per device |
| Publish operations | 6 per minute per actor |
| WASM compilation | 2 per minute per actor, maximum 2 queued compilation jobs |
| WASM execution | 20 per second, burst 40, per device; execution limits in Section 6 also apply |
| Actor storage | 64 MiB across private drafts and owned non-log application data, with separately bounded log retention |
| Active subscriptions | 32 per device |

Limits are configuration policy, not claims of sustainable system capacity. Lower limits may be necessary on constrained hosts.

Slow consumers receive coalesced UI projections where semantics permit, never reordered or silently lost authoritative event history. A client that cannot keep up is disconnected with a resumable cursor; it subsequently resumes from the event log or receives a snapshot plus a gap-free suffix. Queue growth must trigger backpressure before host memory exhaustion.

## 12. Secure update and software supply chain

Cover the Rust core, TypeScript host, WASM engine/worker, mobile app, built-in component schemas, trusted plugins, and published executable artifacts.

- Pin direct and transitive dependencies and verify lockfile integrity.
- Produce an SBOM, provenance record, and vulnerability report for each release.
- Sign update manifests containing artifact hashes, versions, supported architectures, schema/ABI compatibility, migration version, and minimum supported reader version.
- Verify signatures and hashes before installation or activation. Mobile store signing alone does not authorize custom host/plugin artifacts.
- Keep update trust roots separate from ordinary user publish capabilities. Root rotation requires an explicit audited ceremony.
- Use reproducible builds where practical and record unexplained differences as release risks.
- Disable automatic updates by default until signed verification, staged activation, health checks, and recovery are proven. Never fetch and execute code directly from an unverified mutable location.
- Published UI/WASM artifacts are immutable and content-addressed. The publisher identity and host publication record are audited; requested runtime capabilities still require separate approval.
- Support authorized rollback of binaries. Prevent silent downgrade of security-critical state or protocol versions. If data cannot be downgraded safely, recovery uses a verified backup and an explicit restore procedure.

## 13. Threat register

| Threat | Primary controls | Residual risk / required detection |
| --- | --- | --- |
| Unauthorized tailnet node reaches host | Tailscale interface binding plus independent device pairing, actor identity, and sessions | Tailnet compromise remains relevant; alert on pairing and new-device events |
| Stolen mobile device or private key | Platform keystore, user/device ceilings, short sessions, revocation, optional attestation | Already exported data is unrecoverable; detect unusual device/session activity |
| Stolen bearer session token | Short lifetime, device binding, hashed storage, revocation, reauthentication | Token use before revocation is possible; device-bound proof should be strengthened where platforms permit |
| Replayed pairing, command, or reconnect | Single-use challenges, nonce expiry, persistent command deduplication, cursor validation | Ambiguous commit outcomes require status lookup, never blind re-execution |
| Draft editor escalates to publisher/executor | Separate Rust-enforced capabilities and runtime grants; immutable publications | Malicious authorized publisher remains possible; audit and capability review required |
| Malicious or non-deterministic WASM | Import/feature denylist, integer-only initial profile, fuel/memory/output limits, Rust output validation, differential tests | Engine vulnerability remains; isolate worker and patch signed updates promptly |
| Renderer/content injection | Declarative allowlist, no arbitrary markup/code, size/depth limits, typed actions and assets | Component implementation bugs require mobile fuzzing and signed updates |
| Malicious or compromised TypeScript plugin | Disabled by default, separate admin trust, signing, least privilege, kill switch | In-process compromise can expose host-process data; residual risk must be accepted explicitly |
| Resource exhaustion | Rate limits, quotas, bounded queues, fuel/memory/time caps, slow-consumer disconnect | Host/network saturation remains possible; track rejected work and queue depth |
| Audit/history rewrite | Atomic hash chain, signed checkpoints, optional independent export, restore verification | Without independent export, privileged host rewrite cannot be fully prevented |
| Malicious update or dependency | Pinning, SBOM, signatures, provenance, staged activation and recovery | Signing-key compromise is high impact; separate keys and rotation/recovery ceremony |
| Offline rollback or stale privileged action | Published version pinning, base revisions, fresh server capability checks, no queued offline execution by default | Offline cached information may remain visible on the device |
| Sensitive UI data leaks through telemetry | Allowlisted metadata, payload redaction, cardinality limits, local-only default | Indirect identifiers can still be sensitive; review exports and access regularly |
| Host administrator or core compromise | OS hardening, encryption at rest, independent checkpoints, backups, incident isolation | Application cannot guarantee confidentiality or integrity after complete unlocked-host compromise |

## 14. Incident response and recovery

### 14.1 Immediate containment

1. Freeze the current evidence set: audit head, log cursor, artifact hashes, active sessions, host/engine versions, and relevant redacted diagnostics.
2. Revoke suspect devices, sessions, runtime grants, and update/plugin credentials. Rotation order must not leave recovery dependent on the compromised credential.
3. Disable publish, execute, and trusted plugins while preserving an explicitly authorized read-only mode when host integrity is still credible.
4. If core integrity is uncertain, disconnect application clients rather than trusting potentially falsified reads.
5. Preserve affected immutable UI/WASM artifacts for offline analysis; do not execute them again on the production host.

### 14.2 Eradication and recovery

- Verify the audit chain and signed checkpoints before trusting the event log.
- Restore only signed, known-good host and mobile artifacts. Revoke compromised signing material before accepting new updates.
- Restore a verified backup or replay to a known-good checkpoint. Validate state hashes, event-log continuity, deduplication records, and published artifact hashes before reopening writes.
- If a migration contributed to the incident, follow the tested rollback/restore path rather than improvising a reverse migration.
- Re-enable capabilities in stages: local owner recovery, read-only client access, draft editing, publishing, then execution. Trusted plugins remain off until separately cleared.
- Notify affected actors of revoked devices/sessions and potentially exposed cached data without including private UI payloads in broad notifications.
- Record root cause, control failures, recovery evidence, remaining risk, and follow-up owners. Update threat and test matrices.

Recovery objectives are product decisions. Until explicit RPO/RTO targets are approved, no recovery-time or zero-data-loss claim is permitted.

## 15. Privacy-preserving observability

Collect enough information to diagnose security and liveness without collecting the UI document itself.

Recommended signals:

- command counts and latency by broad action/result category;
- active sessions, reconnects, subscription gaps, and slow-consumer disconnects;
- WASM compilation/execution traps by bounded reason code, fuel exhaustion, watchdog aborts, memory/output rejection, and queue depth;
- capability denials by action category and resource type, with rate-limited actor/device metadata;
- storage usage buckets, event-log/snapshot sizes, audit verification results, and checkpoint age;
- update verification, migration, rollback, and recovery outcomes;
- host, schema, ABI, and engine versions as bounded dimensions.

Prohibited by default: raw UI text, component trees, action arguments, state payloads, tokens, pairing codes, private keys, secret values, full published artifacts, unbounded user IDs, and stack frames containing request payloads. Logs use an allowlist and redact before emission. Error sampling must not bypass redaction.

Use opaque trace IDs and, where cross-system correlation is essential, pseudonymous identifiers with protected rotation. Avoid high-cardinality labels. Telemetry export and remote crash reporting are off by default; enabling either requires a data-flow review, access control, retention policy, and operator-visible configuration.

## 16. Security acceptance gates

These gates are verified through the delivery milestones in `06-delivery-verification.md`. A gate is passed only with retained evidence; the wording here is not evidence of completion.

| Gate | Testable acceptance condition | Delivery gate |
| --- | --- | --- |
| SEC-01 Network and identity | An unpaired but tailnet-reachable device cannot establish an application session or read/mutate state. Pairing replay, expired challenges, revoked keys, and cross-host challenges fail. No public listener exists in the default configuration. | G1, G5 |
| SEC-02 Capability separation | A matrix covering every actor/resource/action proves that draft read/write does not imply publish or execute, publish does not grant runtime effects, and revocation blocks later commands and subscription events at the defined log position. | G1, G3 |
| SEC-03 Deterministic sandbox | Unknown imports, WASI, clock/random/network/filesystem imports, non-profile instructions, cache-policy mismatches, and oversized artifacts are rejected. Repeated and cross-environment differential runs agree on canonical logical results or deterministic resource traps. | G4 |
| SEC-04 Resource containment | Infinite loops, memory growth, output floods, malicious modules, queue saturation, and worker crashes exhaust only their bounded allowance. The Rust core remains consistent and accepts later authorized work. | G4, G5 |
| SEC-05 Transaction and audit integrity | State, event, deduplication, and audit records survive crash points atomically. Single-record modification, reordering, deletion, snapshot substitution, and invalid checkpoints are detected. | G2, G5 |
| SEC-06 Backpressure and quotas | Rates and byte/storage/connection limits reject excess work with typed errors. Slow consumers cannot grow unbounded queues or cause silent authoritative history loss. | G2, G5, G6 |
| SEC-07 Secret and privacy hygiene | Automated adversarial fixtures containing secret-like and private UI values show no prohibited values in logs, metrics, traces, crash reports, URLs, or client-ineligible projections. | G3, G5 |
| SEC-08 Update and rollback trust | Unsigned, wrongly signed, hash-mismatched, incompatible, and silently downgraded artifacts are rejected. Signed migration/rollback drills preserve verified state and audit continuity. | G5, G6 |
| SEC-09 Incident recovery | A simulated stolen device, malicious artifact, plugin compromise, and audit-tamper event follow the runbook to containment and verified recovery without privileged writes during the unsafe interval. | G5, G6 |
| SEC-10 Accessibility-safe failure | Invalid or unauthorized UI content produces an accessible, non-executing error state without focus theft, hidden privileged actions, or sensitive detail exposure. | G3, G6 |

Release requires every applicable gate to pass. Any accepted exception names the exposed actors/data, compensating control, expiry, and accountable owner; an undocumented exception is a failure.
