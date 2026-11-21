# Rust-First ARBOR ↔ ZCode ↔ ChatGPT Bridge — R002 Design

## Desired end state
ChatGPT acts as user-facing conversational coordinator. Desktop Commander supplies authorized remote filesystem/process entry. A narrowly scoped Rust ZCode Protocol client owns stdio transport and native message parsing. ARBOR Host remains the sole world-effect / approval / receipt authority. ZCode Agent remains responsible for its own models, sessions, tools and workflows. HQ broker arbitrates cross-seat wake and message delivery. Do not merge distinct authoritative stores.

```text
ChatGPT thread N
  └─ Desktop Commander authorized command / file access
       └─ ARBOR Host-owned bridge (Rust: zcode_bridge)
            ├─ ZCode app-server child process (NDJSON stdio)
            │    ├─ native legacy reads: session/list, workflows/list, plugins/list
            │    └─ V4 controller and session subscriptions / commands
            ├─ HQ seat/bus adapter (external; not ZCode's native child-session tree)
            ├─ session + workflow observer (read-only projections)
            ├─ provider/model selector (explicit owner budget, no silent fallback)
            └─ approval / receipt journal + revocation + recovery
```

## Rust module plan
- `transport.rs`: `tokio::process::Command`, bounded line reader, UTF-8 NDJSON parser, max frame byte size, stdout/stderr isolation, lifecycle cancellation.
- `schema.rs`: `serde` envelopes, strict tagged request ID and error handling. Separate V4 and legacy types, version gates.
- `session.rs`: list/read, subscribe/unsubscribe, revision cursor, parent-child graph, resume handles. Never derive IDs from titles.
- `workflow.rs`: list/run read-only, register future effect commands only behind Host-issued approvals, cancellation and reconciliation.
- `provider.rs`: capability catalog, providerId/modelId, availability + quota facts with freshness, never expose secrets.
- `hq_bus.rs`: external broker client, seat liveness, leases, dedupe and receipts; no direct writes to legacy bus files without contracts.
- `policy.rs`: mode/permission scope, whitelists, per-task cost and concurrency caps, session-level revocation.
- `observability.rs`: span/event IDs, token metrics, schema errors, structured PASS/FAIL/UNRUN traces, no raw prompts unless explicitly needed.
- `recovery.rs`: reconnect after child restart; classify in-flight unknown outcomes before retrying; prevent duplicate effects.

## Wire contract
Request ID is required; native app-server accepts JSON lines of `{id,method,params}`. Server emits `{id,result}`, `{id,error}`, and notifications `{method,params}`. Current V4 source introduces `clientHello`, `hello`, `v4/command` and topic subscription frames. Validate wire version and capabilities before subscribing or issuing V4 mutation commands.
Sample native safe request: `{"id":"q1","method":"session/list","params":{"limit":5}}`. Never send unsupported ACP-only methods straight to ZCode.

## Deterministic safety
Invariant: no command is retried after an ambiguous side effect without reconciled receipt; no force Git operations, no provider key copying, no unapproved model billing. Run agents in plan/read-only discovery first. Host persists approvals and effects; do not reimplement permission system in the bridge. Prefer Rust compiled components; temporary Node TypeScript protocol probe is a disposable validation harness, not a second product runtime.

## Rollout
R002 (now): documentation + live protocol probe. R003: Rust read-only client with compatibility tests and automated SITREP. R004: explicit model select + agent spawn/cancel + cost guard. R005: verified HQ seat messages with receipts. R006: ARBOR Host-native app surface and PWA status panel.

## Open ADRs
- Which component owns ZCode process lifecycle when user has desktop app already running?
- Is V4 transport public/stable enough to pin, or should we version-gate by installed package?
- Where exactly to place the narrow bridge in ARBOR Rust graph without adding a second database?
- How to choose live seat vs cold persisted session without waking the wrong thread?
- Must use same Host-owned authority if ZCode's computer-use plugin is enabled.
