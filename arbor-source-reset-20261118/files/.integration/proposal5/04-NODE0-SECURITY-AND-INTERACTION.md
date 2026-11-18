# Proposal 5 / 04 — Node 0 Prime: Minimum Safe Native Upgrade for a Beta

## Scope

Node 0 is mission-critical but beta-first centralization **does not** require a full semantic engine rewrite. It requires one correct, testable native input/projection path coexisting with existing Host semantic execution until the exact intended commands have parity.

## Required upgrades (P0, cannot defer)

1. **Patch Tauri coherently.** Current donor Cargo.toml pins `tauri = "=2.0.0"` and related 2.0.0 packages; prior official advisories affect that range, including a later fix at >=2.11.6. Select a currently supported patched v2 release, update full lockfile/capabilities and verify Windows installer. Do not merely change a version line.
2. **Native origin/process identity.** Restrict `src-tauri/capabilities/` to the actual trusted webview and exact allowed origins. Do not send persistent owner bootstrap bearer token to an unverified localhost listener merely because an HTTP health response succeeds. Implement a process-bound handshake/secure inherited channel or native session mint with collision/impersonation tests.
3. **Input actor binding.** Native event ingress derives principal/device/session from authenticated Host/Tauri context; untrusted renderer-supplied actor/session/policy fields cannot grant source reads.
4. **Bounded computation.** The R001 Rust spike used a Mutex around the whole `apply_json` computation and spawned scoped threads per input event. This is not the beta hotpath. Use short session snapshot/ack, existing native graph scheduler with bounded queues, replace-latest drafts, deadlines and CAS evidence commit. Never coalesce committed Submit/Approve/Cancel/Receipt.
5. **No draft effects.** Type/Tab suggestions and speculative HQKG reads are inert. Current Host JS dispatcher remains exclusive effect authority with approval/grant/claim/receipt; no native duplicate effect admission.
6. **Source truth.** Native syntax/semantics use current internal Host action IDs and exact project/source refs; no fixture-only `full` action treated as available. Source unavailability and timeouts are visible.
7. **Private source isolation.** HQ dev-intelligence source data and Host personal World data have separate permission scopes, graph namespaces and cache keys; raw drafts are not broadcast to irrelevant source processors.
8. **Offline/phone honesty.** The paired PWA has local text/IME and provisional cached views. When disconnected it cannot claim current Host auth/permission/effect status or run effects; reconnect revalidates source/device revisions.
9. **Effect uncertainty.** Existing Host browser action claims/receipts remain durable, no automatic retry after uncertain send.
10. **Exact build parity.** The same proposed input frame/scope/version is encoded in Rust and TypeScript through generated contracts and strict validators; cross-surface submissions go through the same Host authority.

## Event schema: beta minimum

```text
InputSnapshot:
  schemaVersion
  trustedActorRef, trustedDeviceRef, trustedSessionRef
  editorRevision, interactionRevision, eventId
  originalTextRef or bounded text delta
  inputKind (draft, accept, submit, cancel, voice_final)
  sourceScope, privacyClass
  expectedRevision and optional sourceVersionVector

DerivedTrace:
  inputRevision
  producerId, producerRunStatus
  candidateFrameId, candidateType
  sourceRefs with owner/revision
  confidenceKind (rule, accepted-pattern, source-observed, model-proposed)
  status (candidate/grounded/ambiguous/stale/unavailable)
  targetAudience, invalidationRefs

CommittedIntent:
  sourceExpressionDigest
  selectedFrame and exact slot IDs
  actor/session and stable request ID
  admissionStatus (UNASSESSED until Host explicitly checks)
```

Only user expressions and typed semantic state are unified; UI caret/IME remains local and instant. Views cannot promote an untrusted source claim or a high prediction score into a grant.

## Beta path

```text
ARBO​​R desktop editor/PWA editor (local echo)
           │
  Actor-bound internal ingress
           │
  Node0 read-only event + bounded HQKG computations
     ├─ quick command/read candidate
     ├─ grounded source references
     ├─ safe prediction/ranking
     └─ source evidence/availability
           │
  Versioned interpreted trace
           │
  In-repo TierFlow/Host UI projection
           │
  Human explicit Submit (if chosen)
           │
  Existing in-repo Host JS planner/admission/journal (ONE WRITER)
           │
  Existing Studio/Chrome worker, observed receipt
           │
  Same source-linked status across desktop and PWA
```

The word "ARBOR" here is the local product, not a separate network service.

## Beta coverage vs deferred sophistication

**Implement now:** native read-only source-driven candidate projection, session revision CAS, basic predictive ranking using explicit choice history, current source status, bounded task work, safe Tab behavior, privacy and trust tests, PWA authenticated forwarding, source-linked intent preview. Preserve proven Host language/compiler for actual command execution until semantic parity.

**Deferred:** complete R007 overlapping 10-layer VIEW ALL hypergraph, sophisticated model ranker, 100 predictive capabilities, cross-device offline model inference, arbitrary program synthesis, general extension sandbox, full Rust Host effect/journal rewrite, universal multimodal live streaming and microsecond scheduling.

## Verification matrix

- Press/release typing, undo, focus, long paste, IME Spanish/Catalan accents, emoji/UTF-8↔UTF-16 spans: the native graph cannot prevent local typing or corrupt selected text.
- Rapid 500-edit event stream and hung reactor: memory/threads bounded; slow outputs fail stale CAS, no UI freeze.
- Wrong actor/session/policy/invalid native command or foreign webview: no personal knowledge leakage.
- Two Tabs on a source-backed existing read-only command: only current project/slot refs may become selected; no implicit effect.
- Stop internal HQ index or Studio provider adapter: input remains usable and provenance says unavailable, not "none exists".
- Revoke paired phone before Submit: no stale effect admission.
- Crash after effect claim and lose browser reply: status uncertain, no duplicate send.
- Run without external donor checkouts: all tests use imported internal code.

**R001 test source is useful; its unverified concurrency design is not a shipping component.**
