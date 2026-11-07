# Phased Delivery and Verification Specification

Status: greenfield implementation and verification plan. Numeric performance values are proposed release targets, not measured results. No milestone may claim completion without the evidence defined here.

Companion document: `05-security-operations.md`. Its SEC-01 through SEC-10 gates are part of this plan's exit criteria.

## 1. Delivery principles

1. Deliver one thin vertical slice before broad feature work: pair a device, edit a small draft, publish it, render it, execute one deterministic action, persist the transaction, reconnect, and verify the audit record.
2. Keep the Rust core authoritative from the first slice. Do not prototype authorization or persistent state in TypeScript with an intention to move it later.
3. Establish contracts and fault semantics before scale work. Protocol, capability, event-log, and ABI test kits grow with the slice.
4. Expose no capability without a negative test. Every permitted operation has tests for missing, narrowed, expired, and revoked authority.
5. Add untrusted user-supplied WASM only after the deterministic profile, resource containment, and differential harness exist. A fixed conformance module may prove the vertical slice earlier.
6. Treat transport delivery as at-least-once and authoritative command processing as idempotent. Do not claim exactly-once network delivery.
7. Keep trusted TypeScript plugins, public ingress, external telemetry, and automatic updates off the critical path and disabled by default.
8. Optimize only against a recorded workload and environment. No latency, throughput, memory, availability, or recovery claim is valid without a retained report.

## 2. Reference state and protocol model

Use these concepts consistently across packages and tests:

- **Actor:** stable local user identity. **Device:** a paired cryptographic credential belonging to an actor. **Session:** short-lived proof bound to one device credential.
- **Draft:** mutable declarative working state with its own revision. Draft preview has no live runtime capabilities.
- **Published version:** immutable, content-addressed UI/logic artifact with schema version, ABI version, source draft revision, publisher, and declared runtime requests.
- **Runtime grant:** separate approval limiting the effects a published version may request. Publication is not runtime approval.
- **Command:** canonical request containing command ID, actor/device/session proof, resource and base revision, typed payload, and idempotency scope.
- **Transaction:** Rust-core atomic decision and mutation. A committed mutation produces state, an event, a persistent deduplication result, and an audit record.
- **Host log cursor:** monotonically increasing position used for event delivery and resume. An aggregate/resource revision supports optimistic concurrency; the host cursor supports gap-free streaming.
- **Snapshot:** verified state at a host log cursor. A snapshot plus the log suffix must equal full replay.
- **Reduction:** deterministic application of an event to prior state. Reduction never checks wall time, random values, network, filesystem, or process environment.
- **Execution:** deterministic WASM transformation for an exact published version, followed by Rust validation and an authorized transaction.

A command based on a stale resource revision is rejected with a typed conflict unless an explicit, deterministic merge contract exists. Offline replay does not bypass current capability, revision, schema, or published-version checks.

Canonical error categories include at least: unauthenticated, forbidden, not-found-or-concealed, conflict, invalid-schema, unsupported-version, resource-exhausted, rate-limited, execution-trapped, unavailable, and integrity-failure. Not-found versus concealment is a resource-specific privacy decision and must be consistent between queries and events.

## 3. Module boundaries and repository-neutral packages

Names below describe logical packages, not a prescribed repository layout.

| Package | Owns | May depend on | Must not own or do |
| --- | --- | --- | --- |
| `protocol` | Versioned canonical schemas for commands, events, UI documents, errors, pairing, session envelopes, and resume cursors | Serialization primitives only | Authorization decisions, storage, transport listeners, native UI |
| `core-rs` | Authoritative state, transactions, invariants, capabilities, event log, snapshots, deduplication, audit chain, migrations | `protocol`, `wasm-abi` as interfaces | HTTP/WebSocket policy, mobile rendering, arbitrary TypeScript callbacks |
| `wasm-abi` | Deterministic guest/host buffers, canonical values, feature profile, error and resource semantics | `protocol` value definitions | Host time/random/network/filesystem, secret access, implicit mutable globals |
| `wasm-runtime-rs` | Artifact validation, isolated compilation/execution, fuel/memory/structural limits, output decoding, cache keys | `wasm-abi`, `protocol` | Final capability grants, direct authoritative state writes, executing requested effects |
| `host-transport-ts` | TypeScript 7+ HTTP/WebSocket termination, connection/session envelope validation, connection quotas, fan-out, backpressure | `protocol`, narrow `core-rs` command/query API | Independent authorization, direct state mutation, alternate event ordering |
| `host-control-ts` | TypeScript 7+ pairing/control APIs, update coordination, health endpoints, optional trusted-plugin host | `protocol`, narrow administrative APIs | Granting itself capabilities, storing authoritative UI state, exposing public ingress by default |
| `mobile-renderer` | Declarative component rendering, accessibility semantics, local ephemeral UI state, typed action dispatch | `protocol` | Arbitrary code/markup execution, authority decisions, hidden persistence |
| `mobile-editor` | Draft editing, validation assistance, conflict presentation, accessible publish/execute requests | `protocol`, `mobile-renderer` | Implying publish from edit, granting runtime effects, queuing privileged offline execution |
| `conformance-kit` | Golden vectors, contract tests, reference models, WASM differential cases, fuzzers, crash/fault injection | All public contracts | Production runtime dependency or relaxed test-only authority |
| `trusted-plugin-sdk` | Optional narrow IPC/API types and plugin lifecycle contract | `protocol` | Implying that plugins are sandboxed like WASM; bypassing Rust transactions |

Dependency rules:

- Production packages never depend on `conformance-kit`.
- TypeScript and mobile code cannot import Rust storage internals or modify the event log directly.
- `protocol` remains independent of runtime implementations so all parties can consume identical golden vectors.
- `core-rs` exposes commands and queries, not unrestricted storage mutation. Runtime effect requests re-enter that same command/capability path.
- The mobile app never receives a general-purpose plugin or native-module loading mechanism through declarative UI.
- Public schema changes are versioned and reviewed alongside compatibility tests, not edited only at one endpoint.

## 4. Milestones and dependency order

### Dependency graph

`M0 → M1 → M2 → M3 → M4 → M5 → M6`

M2 and M3 may overlap only after their shared M1 contracts are stable, but M3 exit requires M2 recovery semantics. M4 requires M3's immutable publication/runtime-grant lifecycle. M5 requires M2 and M4. M6 requires every earlier exit gate. Trusted third-party plugin support is a later optional milestone and is never needed to satisfy the core product slice.

### M0 — Contracts, threat model, and verification foundation

**Deliverables**

- Canonical identifiers, command/event/envelope schemas, version negotiation, error taxonomy, size limits, and idempotency semantics.
- Initial capability matrix covering draft/publish/execute/runtime grants and administration.
- Declarative component/action schema and integer-only deterministic WASM ABI profile.
- State reduction, host cursor, snapshot, conflict, and reconnect semantics.
- Trust-boundary review matching `05-security-operations.md`, including actual Tailscale deployment assumptions and default-deny exposure.
- Reference-model and fault-injection plan with explicit durability points.

**Exit gate G0 — Contract readiness**

- Golden vectors round-trip every initial schema in each planned consumer; malformed and unsupported-version vectors have specified errors.
- Every authoritative command identifies its resource, base revision, capability, idempotency behavior, audit metadata, and possible event output.
- Capability tests enumerate allowed and denied draft, publish, execute, and runtime-effect combinations; no implicit grant exists.
- The event-log model proves on small exhaustive/model-checked cases that replay is deterministic and duplicate command IDs do not duplicate state effects.
- All initial limits are values in shared contract/test fixtures, not inconsistent per-package constants.
- Open product decisions retain the safe defaults in Section 15.

### M1 — Thin vertical slice

**Scope**

One host, one local owner, one paired mobile device, direct tailnet-only connectivity, one small declarative screen, and one fixed integer-only WASM conformance artifact. The flow is pairing → authenticated session → draft edit → immutable publish → mobile render → typed action → deterministic WASM execution → Rust transaction → event fan-out → durable restart/reconnect → audit verification.

Only the owner UI is required, but synthetic actors and capability fixtures must exercise denial paths. No offline privileged queue, arbitrary plugin, public ingress, external telemetry, automatic update, or general third-party WASM upload is included.

**Exit gate G1 — End-to-end authority and persistence**

- A tailnet-reachable but unpaired device receives no application session, UI projection, or mutation. Expired/replayed pairing and revoked device credentials fail (SEC-01).
- A draft-only fixture cannot publish or execute; a publish-only fixture cannot execute or obtain runtime effects; an execute fixture cannot edit or publish (SEC-02).
- The published screen renders from schema-validated data and dispatches only its typed action.
- Repeating the same command ID after a client timeout returns the persisted result without another state transition or duplicate event.
- Restarting TypeScript alone, the WASM worker alone, and the whole host preserves committed state and resumes from a gap-free cursor.
- State, event, deduplication result, and audit record refer to the same transaction; no partial commit is observable.
- An unauthorized/invalid action has no authoritative state effect and creates only the specified security/audit result.
- The slice has retained contract, integration, negative-security, restart, and accessibility-smoke evidence. No performance claim is required yet.

### M2 — Event log, multi-client realtime, and crash recovery

**Deliverables**

- Durable atomic event log, snapshots, deduplication retention, integrity checks, and backup format.
- Multi-client subscriptions, resume cursors, snapshot-plus-suffix recovery, slow-consumer handling, and deterministic conflict behavior.
- Typed client stale/offline status and explicit resynchronization.
- Fault injection around every persistence and fan-out boundary.

**Exit gate G2 — Durable, gap-free realtime state**

- Model/property tests in Section 7 pass across generated command/event histories, including duplicates, stale revisions, crashes, and snapshot boundaries.
- A client acknowledging a command can recover its result by command ID after any tested crash point; a blind retry never creates a second mutation.
- Snapshot-plus-suffix state hash equals full-log replay for generated and production-like histories.
- Two clients with competing edits observe the same authoritative order. The loser receives a typed conflict and can rebase without silently overwriting the winner.
- Resume with a valid cursor yields no gap or duplication in authoritative event sequence; an expired cursor triggers verified snapshot recovery.
- A slow consumer is bounded/disconnected without unbounded host queues and can recover the complete authoritative history (SEC-06).
- Crash and corruption cases enter read-only/integrity-failure mode when required; publish and execute never proceed on an unverified log (SEC-05).

### M3 — Self-reprogrammable editor and capability lifecycle

**Deliverables**

- Accessible mobile draft editor, validation, immutable publication, runtime-grant approval, version selection, and safe conflict/rebase UX.
- Multi-actor/device administration, delegation ceilings, revocation, and subscription refiltering.
- Complete declarative component states, error fallback, and sensitive-data projection rules.

**Exit gate G3 — Safe self-reprogramming**

- The full actor/resource/action capability matrix passes, including delegation attenuation, expiry, device ceilings, recursive revocation, and post-revocation subscription behavior (SEC-02).
- Editing a published version creates a new draft; publication creates a new immutable version; old executions remain bound to their selected version and grant.
- Publish preview accurately identifies artifact hash/version and requested runtime capabilities. Approving publication alone grants none of them.
- Draft preview has zero authoritative effects. Offline stale publish/execute requests require fresh online checks and never execute silently.
- Oversized/deep/unknown/malicious UI documents produce accessible safe errors and no action dispatch (SEC-10).
- Secret/private UI canary values are absent from unauthorized projections and emitted telemetry (SEC-07).
- Screen readers, focus, dynamic type, contrast, touch targets, and editor errors pass the manual/automated accessibility scope in Section 12.

### M4 — General deterministic WASM and sandbox hardening

**Deliverables**

- User artifact validation, deterministic feature enforcement, isolated worker lifecycle, bounded queues, cache policy, and resource diagnostics.
- Differential execution harness, malformed-module corpus, and deterministic replay evidence.
- Runtime-effect broker that validates every guest request through Rust capabilities.

**Exit gate G4 — Deterministic and contained user logic**

- All SEC-03 import/feature/cache cases pass. No guest can import network, filesystem, environment, clock, random, thread, or secret APIs.
- Differential runs across fresh/reused instances, process restarts, supported CPU architectures, and supported engines produce identical canonical logical results or specified deterministic resource traps.
- Fuel exhaustion, memory/output caps, malformed modules, non-profile numbers/instructions, start-section abuse, and compiler/worker failure commit no partial state (SEC-04).
- Guest-requested effects without exact published-version runtime grants are denied before any effect. Excess effect counts and output bytes are rejected atomically.
- A killed or exhausted worker does not crash the Rust core, block unrelated recovery operations, or poison later executions.
- Fuzzers and minimized regression corpora meet Section 9 exit criteria with no unresolved critical/high-severity finding.

### M5 — Security operations, supply chain, and recoverability

**Deliverables**

- Signed updates/manifests, SBOM/provenance, migration framework, verified backups, audit checkpoints, rotation/revocation tools, and incident runbooks.
- Rate/quota/backpressure completion and privacy-reviewed local observability.
- Optional independent audit checkpoint export; external telemetry remains off unless separately approved.

**Exit gate G5 — Operable security and recovery**

- SEC-01, SEC-04, SEC-05, SEC-06, SEC-07, SEC-08, and SEC-09 scenarios pass with evidence.
- Stolen-device containment revokes future access; malicious-artifact containment disables execute/publish; audit tamper forces integrity-safe behavior.
- Signed update verification rejects unsigned, wrong-key, modified, incompatible, and unauthorized-downgrade packages before activation.
- Backup/restore and migration/rollback drills in Section 13 preserve state, event, deduplication, and audit continuity.
- Sustained quota and hostile-input tests show bounded work/queues and useful typed errors without exposing private payloads.
- Operators can complete pairing, revocation, verification, backup, restore, and incident procedures using documented role-separated workflows.

### M6 — Cross-platform release candidate and evidence pack

**Deliverables**

- Supported host/mobile matrix, real-device accessibility results, performance/soak reports, compatibility results, known limitations, and final recovery drill.
- Reproducible release evidence keyed to artifact hashes and source revision.

**Exit gate G6 — Release acceptance**

- Every earlier gate remains green on release artifacts, not only development builds.
- The support matrix in Section 11 passes, or unsupported combinations are explicitly blocked with a typed compatibility message and documented scope.
- Proposed performance/soak budgets in Section 10 are met on the named reference environment, or formally revised before release with rationale. A report created after the fact cannot be replaced by an assertion.
- No open critical/high-severity security, data-loss, crash-recovery, or accessibility defect remains. Medium/low exceptions have an owner, rationale, compensating control, and expiry.
- A final clean recovery from the signed release backup succeeds and records observed RPO/RTO without turning one observation into a general guarantee.

## 5. Contract verification

Maintain language-neutral golden vectors and consumer/provider tests for every public boundary.

### Protocol and transport contracts

- Canonical encoding/decoding round trips, including Unicode, empty/maximal values, integer boundaries, unknown fields, duplicate fields, invalid UTF-8, recursion/depth, and non-canonical encodings.
- Schema and ABI version negotiation: older/newer mobile, host, UI document, and WASM artifact combinations either interoperate as specified or return `unsupported-version` without mutation.
- WebSocket fragmentation, message limits, ping/pong, reconnect, half-open connections, out-of-order delivery attempts, duplicate frames, and malformed close/retry behavior.
- Authentication on every command, not only connection establishment. Session revocation and capability reduction apply at a defined cursor.
- Idempotency scope, same-command/different-payload detection, persistent result lookup, and retention-expiry behavior.
- Resume-cursor validation, snapshot fallback, event filtering, and no cross-resource data leakage.

### Core and capability contracts

- For each capability: allowed case plus unauthenticated, wrong actor, wrong resource, wrong device ceiling, expired, revoked, insufficient constraint, and stale-session cases.
- Denial produces no state mutation. Privileged-denial auditing follows the specified path.
- Draft/publish/execute/runtime grant separation and immutable publication behavior.
- Effect requests are reauthorized by Rust even when transport and WASM metadata appear valid.
- TypeScript cannot call a test-only or administrative path that bypasses the production capability decision.

### Mobile declarative contracts

- Every component/action/asset schema has golden render semantics, validation failures, accessibility metadata, and bounded-size cases.
- Data cannot introduce arbitrary markup, native code, URLs, or unnamed host functions.
- UI version and action compatibility failures preserve an accessible safe state.

Contract tests run against both provider and all consumers. Updating a golden vector requires reviewing the semantic change; regenerating snapshots until tests pass is not valid verification.

## 6. Model and property tests: reduction and event log

Implement a small independent reference model rather than comparing the core only to itself. Generate valid and invalid command histories with actors, resources, revisions, capabilities, duplicates, reconnects, crashes, and migrations.

Required properties:

1. **Deterministic fold:** the same initial state and ordered events always yield the same canonical state hash.
2. **Replay equivalence:** full replay, incremental reduction, and snapshot-plus-suffix produce equivalent authoritative state.
3. **Atomic commit:** after every injected failure, each transaction is wholly absent or has its state/event/dedup/audit records wholly present and mutually consistent.
4. **Idempotency:** repeated command IDs within scope return the original result and add no state effect or event. Reusing an ID for different canonical content fails.
5. **No unauthorized mutation:** denied, expired, revoked, stale-session, or out-of-scope commands never alter authoritative state.
6. **Revision safety:** stale base revisions cannot silently overwrite newer state. A conflict has no hidden partial effect.
7. **Log order:** committed host cursors are unique and gap-free in the retained authoritative sequence; all connected clients converge to the same order after recovery.
8. **No lost acknowledged event:** any transaction acknowledged as committed is present after restart and in subsequent replay, subject only to an explicitly tested disaster-recovery RPO.
9. **Bounded truncation:** log/snapshot/dedup retention never removes information still required for an advertised resume or idempotency guarantee.
10. **Audit linkage:** every committed mutation has exactly one matching audit record and correct previous-hash linkage. Tampering is detected.
11. **Round-trip stability:** canonical state, event, snapshot, and migration formats survive encode/decode without semantic drift.
12. **Isolation:** one resource's invalid command, WASM trap, or slow subscriber cannot corrupt or reorder another resource's committed transaction.

Do not assert commutativity for operations whose order matters. Where commutative operations are intentionally supported, define and test the exact merge rule and its capability/revision semantics.

Use bounded exhaustive exploration for small state spaces and randomized property tests for larger histories. Persist failing seeds and minimized histories as permanent regressions.

## 7. Deterministic WASM differential verification

Use at least one independent reference path and, where supported, two engine/configuration paths. Agreement with the same implementation repeated in one process is insufficient.

Differential dimensions:

- fresh instance versus reset/reused instance;
- cold versus warm compilation cache;
- same process versus new process and worker restart;
- supported host operating systems and CPU architectures, including big/little-endian assumptions if ever introduced;
- supported engine versions and deterministic feature profiles;
- memory addresses, allocation patterns, and queue interleavings;
- optimization levels, provided the deterministic profile remains identical.

Compare canonical logical output, state operations, UI patch, typed effect requests, and specified deterministic trap category. Fuel consumption may differ between metering implementations and must not be compared for exact equality unless the ABI defines one deterministic metering algorithm. Wall-watchdog outcomes are liveness events, not logical replay inputs.

Corpus cases include empty/minimal/maximum modules, malformed binaries, unknown imports, WASI imports, memory growth, infinite loops, deep recursion, oversized outputs, excessive effects, integer overflow semantics, canonical serialization boundaries, forbidden floats/SIMD/threads, start-section traps, cache-key mismatches, and guest diagnostics containing secret-like values.

For every input within the accepted deterministic profile, repeated canonical logical results must match. Any architecture-, time-, address-, cache-, or instance-dependent state result blocks general user-WASM release.

## 8. Fuzzing and adversarial input program

Fuzz these surfaces independently and end to end:

- canonical protocol values and binary/text encodings;
- HTTP/WebSocket parsing, fragmentation, compression if enabled, and connection lifecycle;
- pairing/challenge/session envelopes and command deduplication;
- declarative UI trees, actions, asset references, Unicode, and accessibility metadata;
- WASM module validation, compilation, ABI buffers, output decoding, and reset logic;
- event log, snapshots, audit chain, deduplication store, backup, restore, and migrations;
- trusted-plugin IPC if plugins are ever enabled.

Use structure-aware and coverage-guided fuzzing where available, with sanitizer-enabled native runs where supported. Set per-case time/memory limits and test parser behavior under truncation, duplication, recursion, compression bombs, and oversized length declarations.

Required oracle: no process crash outside the isolated worker, no unbounded allocation/work, no unauthorized commit, no cross-tenant disclosure, and no acceptance of non-canonical data that later decodes differently. Expected rejection is success; panic, deadlock, partial commit, or inconsistent error mapping is a defect.

Exit requires a fixed minimum campaign agreed before the release candidate, zero unresolved critical/high findings, all failures minimized into regression fixtures, and recorded campaign duration, artifact hash, corpus hash, coverage approach, and seeds. A time-boxed fuzz run is evidence for that run, not proof of absence of vulnerabilities.

## 9. Crash, corruption, and recovery verification

Inject failures before, during, and after each durable write and commit acknowledgment:

- Rust core state/event/dedup/audit transaction;
- snapshot creation and replacement;
- checkpoint signing and audit verification;
- TypeScript fan-out and client acknowledgment;
- WASM compilation, execution, output return, and worker cleanup;
- backup, restore, schema migration, and update activation.

Test TypeScript termination, WASM worker termination, Rust termination, whole-process termination, and OS/power-loss simulation where the platform supports credible fault injection. Distinguish acknowledged durable commits from merely accepted requests.

If a client has an ambiguous outcome, it queries by command ID or retries the identical canonical command. It never changes the payload under the same ID and never assumes failure means absence. Recovery must resolve the original result without a second effect.

Corruption cases include truncated event, modified state, replaced snapshot, missing dedup record, broken audit link, invalid signed checkpoint, wrong artifact hash, and interrupted migration. Detection leads to specified integrity-safe behavior: no publish/execute/admin mutation, verified snapshot/log fallback where valid, or backup restore.

Exit requires that every committed transaction satisfies atomicity, all clients converge after recovery, audit continuity is either verified or explicitly restored through the recovery ceremony, and later authorized commands can proceed only after integrity checks pass (SEC-05).

## 10. Multi-client, reconnect, and offline verification

Scenarios must include:

- two devices for one actor and multiple actors with overlapping and disjoint grants;
- simultaneous edit/publish/execute commands and deterministic conflict results;
- reconnect before command acknowledgment, after commit before acknowledgment, and after event delivery;
- duplicated command and subscription messages, delayed frames, cursor gaps, expired cursors, and snapshot fallback;
- Tailscale disconnect/reconnect, IP/interface changes, host restart, and mobile background/foreground transitions;
- slow consumers, paused mobile apps, large snapshots, and bounded coalescing;
- device/session revocation while connected and capability narrowing during an active subscription;
- device clock skew and incorrect local timestamps.

Offline policy for the initial release:

- Cached read-only published UI may be shown with a visible stale indicator if the actor is authorized to retain that cache.
- Local draft editing may continue in protected device storage.
- Offline publish, execute, runtime effects, device administration, and capability delegation are not queued for automatic replay.
- On reconnect, local drafts are compared by base revision. Conflicts require explicit rebase/overwrite choices with the required current capability.
- Reauthentication and Rust-core checks occur online. Device time cannot extend session or grant lifetime.

Acceptance requires convergence to the authoritative host state, no silent stale write, no duplicate command effect, no privileged offline execution, no post-revocation event delivery beyond the defined log position, and clear accessible stale/conflict/error presentation.

## 11. Cross-OS and mobile support matrix

Define exact supported versions at release time. At minimum, test:

| Lane | Required coverage |
| --- | --- |
| Host OS | Windows, macOS, and Linux on supported CPU architectures; include ARM64 and x86-64 where claimed |
| Mobile | Android minimum and current supported releases; iOS minimum and current supported releases |
| Real devices | At least one representative physical Android and one physical iOS device for release smoke, accessibility, backgrounding, keystore, and reconnect tests |
| Filesystem/storage | Case sensitivity, long/non-ASCII paths, restrictive permissions, full disk, read-only recovery, interrupted writes, backup/restore |
| Runtime | Process restart, clock/time-zone changes, locale differences, sleep/resume, resource pressure |
| Network | Direct Tailscale path, reconnect, unauthorized network, intended proxy/Serve path if supported, no unintended public listener |
| UI | Small/large screens, rotation/resizing where supported, largest OS font, dark/light/high-contrast modes, RTL and long localized strings |
| Platform security | Keystore lock/unlock behavior, credential replacement, app reinstall, cache protection, revoked device |

Run full deterministic and contract suites on every claimed host architecture. Run mobile contract suites broadly, but require physical-device evidence for secure-key, backgrounding, accessibility, and lifecycle behavior. Simulator-only success does not establish those properties.

An untested combination is unsupported, not presumed compatible. Unsupported schema/ABI/platform combinations fail closed with actionable compatibility errors.

## 12. Performance, capacity, and soak targets

### Reference workload

Before running a release benchmark, pin the host hardware/OS, mobile devices, release artifact hashes, Tailscale path and measured network conditions, storage type, dataset, schema/ABI versions, and all limits. Use release builds and production-equivalent durability/audit settings.

Initial workload:

- 20 paired devices, 20 connected clients, with a documented mix of reads, commands, and subscriptions;
- canonical command/event payloads of 1 KiB excluding separately tested large snapshots;
- one published UI document of representative supported complexity, below the 1 MiB/10,000-node policy ceiling;
- a 500,000-fuel integer-only reducer for execution benchmarks;
- 10 transactions/second aggregate during soak, with 1% exact duplicate retries;
- each client reconnects once per 10 minutes during soak;
- at least 10,000 measured transactions for percentile latency tests after a documented warm-up.

### Proposed release targets

These are acceptance budgets to verify, not existing results:

| Measure | Proposed target on named reference environment |
| --- | --- |
| Rust transaction commit, excluding user-WASM execution | p95 ≤ 50 ms; p99 ≤ 200 ms |
| Publish-to-render at a connected reference mobile client on controlled direct Tailscale path | p95 ≤ 500 ms; p99 ≤ 1.5 s |
| 500,000-fuel deterministic reducer execution | p95 ≤ 50 ms, with no watchdog abort |
| Resume using a verified 1 MiB snapshot plus 10,000-event suffix | p95 ≤ 2 s |
| Correctness under load | Zero lost acknowledged events, duplicate command effects, state-hash divergence, or unauthorized deliveries |
| Queue behavior | All queues remain below configured bounds; overload yields typed backpressure rather than unbounded memory growth |
| Soak | 24 hours at reference workload with no crash, integrity failure, or persistent unresponsive interval beyond the documented recovery target |
| Soak memory | At defined quiescent checkpoints after warm-up, retained memory grows ≤ 10% over the run and documented fixed caches stay within their maxima |

Report distributions and environment details, not only averages. Separate host processing from network time and report both for end-to-end measurements. A failed budget blocks performance signoff until corrected or the target is explicitly revised with product approval; it must not be hidden by relaxing durability, audit, sandbox, or accessibility controls.

Run at least one overload test above expected workload to verify bounded degradation. Capacity numbers from the reference environment are not universal promises for other hosts or networks.

## 13. Accessibility verification

Target WCAG 2.2 AA principles for the mobile UI and editor while honoring native Android and iOS accessibility conventions.

Use automated checks plus manual VoiceOver and TalkBack passes for every component family and for the complete pair/edit/publish/execute/error/recovery flows.

Required cases:

- accessible names, roles, values, hints, states, and actions for all controls;
- logical reading and focus order after initial render, state update, validation error, conflict, and published-version change;
- no unexpected focus theft or excessive live-region announcements during realtime updates;
- visible focus and operability with keyboard/switch/assistive input where the platform supports it;
- text contrast at least 4.5:1 for normal text and 3:1 for large text; meaningful non-text UI contrast at least 3:1;
- no status or error communicated by color alone;
- touch targets of at least 44×44 points on iOS and 48×48 dp on Android, with adequate spacing;
- largest supported OS font, text reflow, truncation alternatives, long localization, and RTL;
- reduced motion and no required time-sensitive gesture without an accessible alternative;
- accessible draft validation, publish confirmation, capability/runtime-request explanation, stale/offline status, and destructive-action confirmation;
- unauthorized, malformed, or integrity-failure UI produces a safe accessible message without sensitive internals or executable fallback content (SEC-10).

Release requires no open critical/high accessibility barrier in a core flow. Automated scanner output alone is insufficient evidence.

## 14. Migration, rollback, and update drills

Test state, event, snapshot, audit, deduplication, protocol, UI schema, ABI, and published-artifact compatibility independently.

Required drill matrix:

1. Previous release `N` to candidate `N+1`, including representative full datasets and published WASM artifacts.
2. Older mobile with newer host and newer mobile with older host within the advertised compatibility window.
3. Unsupported schema/ABI/engine combinations, which must fail without mutation.
4. Interruption at every migration and activation step, followed by automatic or documented recovery.
5. Migration failure after backup but before activation; no partial externally visible state.
6. Binary rollback to `N` where data compatibility is guaranteed.
7. Where rollback is unsafe, restore the verified `N` backup and prove event/state/dedup/audit consistency.
8. Signed trust-root rotation and emergency revocation of a compromised update key.
9. Audit checkpoint continuity across migration, rollback, restore, and retention truncation.

Every update creates or confirms a verified restorable backup before destructive migration. Backups are encrypted, access-controlled, and tested by restoration rather than existence checks. A rollback never silently removes security policy or re-enables revoked devices, capabilities, plugins, or artifacts.

Acceptance requires signed-artifact verification before activation, atomic or recoverably staged migration, explicit compatibility failures, no duplicate replay effect, no lost committed transaction within the stated recovery objective, and verified audit continuity (SEC-08). Record observed recovery point and elapsed recovery time for each drill; do not generalize a single result into an RPO/RTO guarantee.

## 15. Open product decisions and safe defaults

Work may proceed with the defaults below. Changing one requires updating contracts, threat analysis, and affected acceptance tests.

| Open decision | Safe default until decided |
| --- | --- |
| Identity provider and tenancy | Local host-generated actors, one bootstrap owner, explicit invites; no mandatory external provider |
| Tailnet enrollment trust | Tailscale node binding is optional supporting evidence; explicit application pairing and owner approval remain required |
| Network exposure | Direct intended Tailscale interfaces only; no Funnel, public reverse proxy, LAN fallback, or silent downgrade |
| Additional TLS | Optional on a proven direct Tailscale WireGuard path; mandatory wherever any path segment is not proven Tailscale-protected |
| Multi-user sharing | Single-owner resources initially; explicit per-resource grants, no broad wildcard sharing |
| Offline behavior | Protected stale read cache and local draft edits only; no automatic privileged offline replay |
| Conflict resolution | Reject stale base revisions; explicit user rebase; no silent last-write-wins |
| WASM numbers/features | Integer-only deterministic profile; floats, threads, shared memory, and unreviewed SIMD remain disabled |
| Runtime effects | No ambient effects; zero preview grants; narrow per-published-version approval |
| Trusted TypeScript plugins | Disabled and absent from the core slice; separate administrative trust and signed installation if later enabled |
| Telemetry and audit export | Local-only redacted telemetry; no remote crash reporting; independent audit checkpoint export optional |
| Storage encryption | Host full-disk encryption and protected mobile caches; application-layer encryption only after key/recovery review |
| Audit/log retention | Bounded documented retention; checkpoint before truncation; never shorten advertised resume/idempotency guarantees |
| Updates | Manual, signed, staged updates with verified backup; no unverified automatic code fetch |
| Recovery objectives | No numerical RPO/RTO promise until drills and product approval define them |
| Performance environment | No cross-platform performance promise beyond tested named reference environments |
| Accessibility/localization scope | Core flows must be accessible from the first public release; unsupported locales fail safely rather than becoming unusable |

## 16. Definition of done and evidence requirements

### Every implementation change

- Public contracts, version compatibility, negative authorization cases, and limits are updated with the behavior change.
- Unit, contract, relevant model/property, and regression tests pass on the change's affected boundaries.
- New parser/executor surfaces have adversarial and fuzz regression cases.
- Failure behavior preserves atomic state/event/dedup/audit semantics and does not weaken safe defaults.
- Logs/metrics/traces pass sensitive-data review and contain no new raw UI, action payload, or secret channel.
- Mobile UI changes include accessibility semantics and relevant manual/automated checks.
- No performance or security claim is made without an artifact-linked report.

### Every milestone

- Its G0–G6 exit conditions are demonstrated on the milestone artifact and retained in an evidence manifest.
- The manifest records source revision, artifact hashes, schema/ABI/engine versions, test seeds, host/mobile environment, and actual outcomes.
- Failures and exceptions are visible. A subagent or implementation summary without test output/artifacts is not completion evidence.
- Security and operations sections affected by the milestone remain consistent with this plan and `05-security-operations.md`.

### Release candidate

- G0 through G6 are all satisfied on signed release artifacts.
- All applicable SEC-01 through SEC-10 gates pass; exceptions follow the documented risk-acceptance rule.
- No unresolved critical/high security, integrity, data-loss, crash-recovery, or core-flow accessibility defect exists.
- Contract, model/property, differential, fuzz, crash, multi-client, cross-platform, performance, soak, accessibility, migration, rollback, backup, and incident evidence is present and reproducible.
- The release clearly names supported/unsupported OS, architecture, mobile, schema, ABI, network, and plugin combinations.
- Operators have tested procedures for enrollment, revocation, quota changes, signed update, audit verification, backup/restore, and incident containment.
- Default configuration has no public ingress, external telemetry, trusted third-party plugin, privileged offline execution, or unsigned update path.
- Final reports distinguish measured observations from targets and explicitly state residual risks and unverified combinations.
