# Context7 surface-upgrade program — VIVIM

**Status:** research program, 2026-11-02. Evidence-only; no runtime claim, no implementation authorization implied beyond the standing Windows roadmap. Every external claim below traces to a recorded Context7 pull in `SOFTWARE_HQ/context7/`.
**Scope owner:** this document is a proposal map. `docs/roadmap.json` remains the only task-status authority; the atomic registry remains the feature inventory.

---

## 0. How `docs/INDEX.md` routed this work

`docs/INDEX.md` was read in full (57 lines) and used as the routing table, not as background. The rows that changed the research:

| INDEX row | Source opened | What it contributed |
|---|---|---|
| "What features exist / are wanted?" | `docs/capabilities/atomic-registry.json` (354 records) | Complete feature inventory; 250 PRD atoms + 104 cross-project candidates; 143 P0 |
| "Which **end-to-end user journeys** actually reach the UI…" | `docs/ux/USER-JOURNEY-ACTIVATION-GRAPH.v1.json`, `ACTIVATION-GAPS.v1.json` | 39 activation gaps used as the demand signal for each family |
| "How mature is text control…" | `docs/design/UNIVERSAL-TEXT-CONTROL-V1.md`, `docs/kpi/UTC-MATURITY-KPIS.md` | UTC-01..07 and the 19-action catalog |
| "How does Bootstrap V1 use generative UI?" | `docs/design/GENERATIVE-UI-BOOTSTRAP-V1.md` | GUI-01..10, `ui-proposal/1`, `ui-revision/1`, `render-plan/1` |
| "How is the cross-device plugin platform being researched?" | `docs/plugins/README.md`, `PLUGIN-TAXONOMY-AND-PLACEMENT.md`, `ARCHITECTURE-DECISION-FRONTIER.md` | Contribution families, placement modes, 10 invariants |
| "Where are the machine-readable data ingress, lineage and utility contracts…" | `docs/data-intelligence/*.json`, `docs/architecture/PERSONAL-DATA-INTELLIGENCE-UTILITY-V1.md` | 13 ingress classes, `vivim.data-asset/1`, PDV-01..06 |
| "How do capabilities produce user value…" | `docs/value/value-drivers.v1.json` | 13 value drivers VAL01–VAL13 |
| "What's the actual existing versus target topology?" | `docs/ARCHITECTURE-CURRENT-AND-TARGET.md` | Source-bound current/target split |
| "Is it currently running?" | Local receipts only | This program makes no runtime claim |
| "What depends on what?" | `docs/capabilities/CRITICAL-PATH.md`, topology | Dependency ordering for the waves |

Derived scope numbers (machine-checked from the registry, not copied):

- 354 atoms: `250 atomic_feature` + `104 cross_project_candidate`
- priorities: P0 143, P1 129, P2 54, untriaged 28
- decision state: `execution_dependent` 309, `human_review_needed` 42, `implemented` 3
- 76 atoms are `missing_atomic_child` from the two-day gap review
- 39 activation gaps, 13 value drivers, 12 root journeys, 120 journey steps
- every atom is `executionWave: unassigned`; P0 is product significance, not permission

---

## 1. What VIVIM should be best at

Ten bets, ordered by trust leverage, not by novelty. Each is a property competitors do not have together: local-first, signed-in web apps, one owner, phone + PC, verifiable effects.

| # | Bet | Why it is the differentiator | Families |
|---|---|---|---|
| B1 | **Verified effect ledger** — every send/edit/config change is journaled before effect, read back after, and never silently retried | The product promise is "trust every permission and change" (VAL03); G17 shows send-once is not yet journey-accepted | F2, F4, F10, F13 |
| B2 | **One canonical conversation identity** — one ID across provider thread, Studio tab, relay command, phone view, export | G15 names multiple authorities; every other feature (history, search, World, handoff) depends on it | F1 |
| B3 | **Device-bound trust without cloud** — QR pairing, hardware-backed key, per-device revocation, replay rejection | G12/G13; Direct is the differentiator vs cloud-only relays | F9 |
| B4 | **Owner-visible honest state** — `starting/degraded/ready/blocked`, observedAt, stale marking, no false green | G03/G04/G22/G37; VAL02/VAL03 | F2, F12, F14 |
| B5 | **Revision-safe generative UI** — hostile input cannot execute; preview/undo is exact | GUI-01..09; the phone Build path (G24) is the product's first wow | F5 |
| B6 | **Personal data utility with lineage** — assets, evidence, marginal utility, correction cascade | PDV-01..06; X-032; "make my data more useful over time" (VAL05) | F6, F7 |
| B7 | **Text parity for every capability** — one descriptor generates GUI, grammar, MCP tool and docs | UTC invariant; G26/G27/G28 show partial coverage | F11 |
| B8 | **Capability-safe plugins** — declarative first, WASM later, never raw CDP/FS/network | Plugin frontier; X-004; the platform cannot ship an unsafe loader | F8 |
| B9 | **Agent interop without losing authority** — MCP/A2A/AG-UI as edges, Host as authorizer | AGENT-17/18; GUI-10; future-proofs against protocol churn | F3 |
| B10 | **One-command, signed, recoverable install** — MSI/Tauri + APK integrity + cold reboot | G01/G02/G03/G11; without it none of the above reaches a real user | F14 |

**Explicit non-goals:** not a hosted SaaS, not a provider-API client, not a general RPA tool, not a public plugin marketplace in V1, not a multi-tenant sync service.

---

## 2. Family map (14 families)

Each family lists: VIVIM surfaces, atoms/gaps, target features, external references, and the upgrade moves that matter. `[P:slug]` = recorded pull, path `SOFTWARE_HQ/context7/raw/*-<slug>.json`.

### F1 — Canonical state, identity and local-first sync

**Surfaces:** `studio/src/store/{versioned,raw-log}.mjs`; `apps/vivim-host/bootstrap/ledger.mjs`; `apps/vivim-host/conversations/*`; `studio/src/studio/conversation-api.mjs`; `apps/mobile-relay/transport.mjs`.
**Atoms/gaps:** STATE-01/02/04/11/12/15/22; WORLD-01/02/03/22; G14, G15.
**Best-at:** stable conversation ID; crash-safe revision log; offline last-known state; branch/restore; deterministic replay and export.

**Evidence**
- Yjs: update events, `Y.applyUpdate`, provider composition, offline persistence, awareness, undo manager, snapshots/GC `[P:yjs-core-doc-updates-providers-awareness]`, `[P:yjs-conflicts-subdocs-snapshots-undo-awareness]`.
- Automerge: `save()/load()/saveIncremental()/loadIncremental()`, `merge()`, `forkAt()`, `getHeads()` for old versions, conflict patches `[P:automerge-doc-history-sync-conflicts]`.
- Automerge Repo: `Repo`, `DocHandle`, storage adapters (IndexedDB), network adapters (WebSocket/BroadcastChannel), `Repo.import/export`, presence `[P:automerge-repo-storage-network-sync]`.
- Electric: shapes/partial replication `[P:electric-shapes-sync-partial-replication]`; PowerSync: upload queue + sync rules `[P:powersync-client-upload-conflicts]`.

**Upgrade moves**
1. Mint one Host-owned `conversation_id`; provider thread is an attribute (`provider`, `provider_thread_ref`, `tab_id`), never the identity. Add a unique index and an alias table for imported/historical threads. This is the minimal fix for G15.
2. Add `vivim.world-op/1`: `{op_id, conversation_id, actor, seq, causal_parents[], kind, payload, payload_hash, idempotency_key, created_at}`. Persist before apply; materialize views from the log.
3. Keep one append-only `ops` table plus periodic snapshots. Snapshot = `(last_seq, state_hash)`; replay must reproduce `state_hash`.
4. Use Yjs only where co-editing is real (Build canvas, shared note). Do not make a CRDT the World authority: Yjs/Automerge are merge engines, not authorization or evidence stores.
5. If content-addressed history is needed, copy Automerge's `heads`/`forkAt` model rather than its runtime: store revision DAG + `parent_rev`, support `restore` as a new revision (never destructive).
6. Offline: last-known projection with `observedAt`, `source`, `stale_after`; block sends while offline (STATE-22).
7. Export: `ops` + snapshots + schema versions in one archive; import verifies hashes and replays.

**Acceptance evidence:** kill Host between op write and materialization → reopen reconciles with no partial state; same conversation ID from phone/relay/Studio; replay hash equality; branch/restore preserves both revisions.

---

### F2 — Durable execution, supervision and receipts

**Surfaces:** `apps/vivim-host/control/journal.mjs`; `apps/vivim-host/bootstrap/{service,command-worker,scheduler}.mjs`; `studio/src/studio/harness-api.mjs`; `harness/src/interface/{approval,standing}.mjs`; grokbot worker.
**Atoms/gaps:** STATE-02/03/11; RUN-03/06/13/20; DEVLOOP-08/17/18; AGENT-13; G01/G03/G17/G37.
**Best-at:** journal-before-effect; uncertain never auto-retried; resumable steps; leases/heartbeats; receipts a human can read.

**Evidence**
- Durable Workflow: steps, retries, signals, `awaitWithTimeout` `[P:durable-workflow-steps-retries-signals]`.
- Rivet actors: explicit lifecycle state diagram, actor state, RPC `[P:rivet-actors-state-rpc-persistence]`.
- XState: parallel states, persistence, guards, invoke `[P:xstate-actors-persistence-guards]`.
- LangGraph: checkpointing, interrupts, streaming, subgraphs `[P:langgraph-state-checkpoint-interrupts]`.

**Upgrade moves**
1. Promote the existing control journal semantics to a shared `vivim.job/1`: states `planned → approved → claimed → running → applied | failed | uncertain`; transitions only by CAS on `rev`.
2. `claim` is the effect fence: one exclusive claim file/row per job; a second claimant gets `409`.
3. Lease row `{worker_id, fence_token, expires_at}` with heartbeat; expired lease forces `uncertain`, never reassignment.
4. Every command carries `idempotency_key`; duplicate returns the original receipt.
5. Multi-step tasks (dev loop, bootstrap passes) persist step boundaries; resume only from the last committed step; uncertain steps pause for the owner.
6. Human-in-the-loop is a first-class state, not a timeout: `awaiting_owner` with a bounded prompt.
7. Do not adopt a Python/Temporal runtime; port the state-machine + checkpoint semantics into the existing Node/SQLite stack.

**Acceptance evidence:** kill -9 at each transition → restart yields the same terminal state; duplicate command test; lease expiry test; one trace shows every step.

---

### F3 — Agent and tool interoperability (MCP / A2A / AG-UI)

**Surfaces:** `harness/bin/wh-mcp.mjs`; `apps/vivim-host/control/action-catalog.v1.json`; `studio/src/studio/{conversation-tools,harness-api}.mjs`; plugin frontier.
**Atoms/gaps:** AGENT-12/13/17/18/22; GUI-10; X-004; G31.
**Best-at:** VIVIM as MCP server for local tools; MCP client for native agents; A2A agent card for the team; AG-UI event stream for Build.

**Evidence**
- MCP: architecture, transports, lifecycle, authorization, tools/resources/prompts, elicitation `[P:mcp-spec-architecture-transports-authorization]`; TS SDK tool registration + Streamable HTTP `[P:mcp-ts-sdk-server-tools-streamable-http]`.
- A2A: `/.well-known/agent-card.json`, `POST /message:send`, task lifecycle, push-notification webhook, authenticated extended card `[P:a2a-agent-card-task-lifecycle]`.
- AG-UI: event type enum, `ToolCallStart/Args/End/Result`, state deltas, SSE client `[P:agui-events-run-lifecycle-state]`.

**Upgrade moves**
1. Generate MCP tool definitions from the action catalog: one descriptor → JSON Schema input/output → MCP tool + text grammar + GUI. Host authorization stays outside the tool.
2. Expose a loopback Streamable HTTP MCP endpoint; use stdio only for child agents. Require origin + owner token; no network bind by default.
3. Publish an A2A Agent Card for the local agent team: skills, auth schemes, streaming support, push endpoint. Long tasks use A2A task states; phone notifications use the webhook.
4. Adopt AG-UI event names for Build/phone streams (`RUN_STARTED`, `STATE_DELTA`, `TOOL_CALL_*`, `RUN_FINISHED`) instead of bespoke event payloads.
5. Tool annotations (`readOnlyHint`, `destructiveHint`) are hints only; the Host policy engine decides.

**Acceptance evidence:** MCP Inspector roundtrip; agent-card schema validation; AG-UI event replay produces identical surface; a tool call cannot bypass the policy gate.

---

### F4 — Browser effect ownership (CDP)

**Surfaces:** `studio/src/cdp/{client,discovery}.mjs`; `studio/src/capture/*`; `harness/src/core/cdp.mjs`; `harness/src/providers/*`; `studio/src/chrome/browser-ownership.mjs`.
**Atoms/gaps:** SEND-01..22; STATE-08/09; G04/G17; X-059/060/065/067.
**Best-at:** attach-only; exact tab/thread binding; draft-safe send; verified model/effort; read-back evidence; reconnect without duplicate effects.

**Evidence**
- CDP: Target/Runtime/Page/Input/Network/Tracing domains; protocol mapping; typed command/event envelopes `[P:cdp-target-attach-session-domains]`.
- chrome-devtools-mcp: attach via `--browser-url`, network request inspection, console messages, performance traces `[P:chrome-devtools-mcp-attach-inspection]`.

**Upgrade moves**
1. Attach with `Target.setDiscoverTargets` + `Target.attachToTarget {flatten:true}`; route every call with `sessionId`. Never assume a global session.
2. Enable per-session `Page`, `Runtime`, `Network`; use `Page.frameNavigated` and `Target.targetInfoChanged` for lifecycle; use `Runtime.evaluate` with `awaitPromise:true, returnByValue:true` for read-back.
3. Type with `Input.insertText` for long prompts (no key-by-key drift); verify composer value and `contenteditable` state before Enter; chunk only when the site requires it.
4. Capture `Network.requestWillBeSent`/`responseReceived` as evidence; correlate the request with the command ID.
5. Ownership proof: `Browser.getVersion` + target list + profile path + a per-profile lock. Refuse to act if a foreign controller owns the tab.
6. Reconnect: re-list targets, re-attach by stable target ID; if the target is gone, mark `uncertain`; never resend.
7. Draft protection: read composer text before send; if non-empty and not ours, return `DRAFT_PRESENT` with a bounded preview.

**Acceptance evidence:** 5,000-char prompt typed and read back exactly; send while foreign draft exists → blocked; connection kill mid-send → uncertain; no duplicate on reconnect.

---

### F5 — Generative UI, revisions and canvas

**Surfaces:** `apps/mobile-relay/generative-ui/{catalog,compiler,renderer,design-composer,phone-surface,semantic-projection}.mjs`; `build-canvas`; `docs/design/GENERATIVE-UI-BOOTSTRAP-V1.md`.
**Atoms/gaps:** GUI-01..10; UX-15/16/18/22; G24/G26; X-018.
**Best-at:** hostile-input-safe catalog; revisioned proposals with exact undo; schema-driven forms; canvas/list parity; accessibility.

**Evidence**
- JSON Schema 2020-12: `$vocabulary`, `$defs`, `unevaluatedProperties`, format `[P:json-schema-2020-12-keywords-vocabularies]`.
- JSON Schema Form: `uiSchema`, widgets, validation `[P:json-schema-form-uischema-widgets]`.
- tldraw: `Store.put`, records, bindings, collaborators, persistence `[P:tldraw-store-records-bindings-collab]`.
- React Flow: controlled `nodes/edges`, `useNodesState`, custom nodes `[P:reactflow-nodes-edges-controlled-state]`.
- AG-UI state deltas `[P:agui-events-run-lifecycle-state]`.

**Upgrade moves**
1. Keep the closed catalog. Add a JSON Schema for the proposal with `additionalProperties:false`, `unevaluatedProperties:false`, explicit `$defs`, and size/node/depth limits.
2. Three identities: `surface_id` (stable screen), `revision_id` (monotonic), `proposal_id` (one model output). Render plan is a pure function of `(proposal, theme, catalog_version)`.
3. Validate with Ajv 2020-12 in standalone mode; reject unknown components/props before any write.
4. Undo = restore the prior revision by ID, not inverse-patch guesswork. Keep a bounded revision ring per surface.
5. Canvas: adopt tldraw's record+binding store pattern for Build; bind each canvas record to a World type/version so restore is coherent (X-050).
6. Graph surfaces: React Flow controlled state; layout is deterministic and saved, never recomputed on every render.
7. Accessibility: every generated control carries role/name/state; tap parity with text actions (GUI-07).

**Acceptance evidence:** fuzz corpus (cyclic, oversized, unknown component, prototype pollution) → rejected without writes; undo restores exact surface hash; screen-reader labels present; phone renders same surface as PC.

---

### F6 — World schema, evidence and provenance

**Surfaces:** `docs/architecture/vaws/CORE-KERNEL.md`; `studio/src/studio/world-service.mjs`; `studio/src/intel/{graph,correlate,temporal-truth}.mjs`; personal agent store.
**Atoms/gaps:** WORLD-01..22; X-043..051; G21/G22/G36.
**Best-at:** claim vs observation separation; owner override precedence; declarative type registry; temporal facts; per-claim lineage; redaction.

**Evidence**
- OpenLineage: run/dataset/job event model + facets `[P:openlineage-events-facets-lineage]`.
- ODRL: Permission/Prohibition/Duty, constraints, conflict `[P:odrl-policy-permission-prohibition-duty]`.
- JSON-LD: `@context`, expansion, compaction, framing `[P:jsonld-context-expansion-framing]`.
- sqlite JSON1/STRICT tables `[P:sqlite-wal-json1-strict-generated]`.

**Upgrade moves**
1. Split records: `Observation` (immutable, source, captured_at, digest) and `Assertion` (subject, predicate, object, status `believed|confirmed|rejected`, valid_from/to, evidence_ids).
2. Owner override is a `confirmed` assertion with `authority=owner`; model output can never set that authority. Hard NO is an ODRL `Prohibition` record with owner-only revocation.
3. Type registry: one JSON Schema per type + `type_version`; migration is declarative (add/rename/deprecate), never generated SQL.
4. Evidence envelope gets OpenLineage-style facets: `source`, `capture`, `transform`, `privacy`, `confidence`; each facet versioned.
5. Temporal: `valid_from/valid_to` + `observed_at`; queries default to current, history explicit.
6. Redaction is applied at projection, not storage: a `DataClass` per field; phone/relay projections filter; provenance remains local.
7. Export as JSON-LD with a stable context; import validates context + type versions.

**Acceptance evidence:** one claim with two contradicting observations renders both; hard NO cannot be removed by a model; export/reimport preserves evidence links; PC-only fields absent from cloud projection.

---

### F7 — Personal data intelligence and retrieval

**Surfaces:** `apps/vivim-host/data-intelligence/{journal-adapter,catalog}.mjs`; `apps/vivim-host/bootstrap/ledger.mjs`; `studio/src/agents/personal-agent-memory-search.mjs`; `docs/data-intelligence/*`.
**Atoms/gaps:** PDV-01..06; X-032; G21/G32/G36; VAL05.
**Best-at:** asset inventory with lineage; marginal utility; local hybrid search; correction cascade; no over-collection.

**Evidence**
- SQLite FTS5: prefix queries, `bm25`, `snippet`, tokenizers `[P:sqlite-fts5-query-bm25-highlight]`; existing pull `-sqlite-fts5-rank-snippet`.
- WAL/JSON1/STRICT/generated columns `[P:sqlite-wal-json1-strict-generated]`.
- sqlite-vec: `vec0` KNN + metadata constraints `[P:sqlite-vec-knn-metadata-quantization]`.
- DuckDB: `ATTACH` SQLite, Parquet, appender `[P:duckdb-parquet-appender-attach-sqlite]`.
- OpenLineage facets `[P:openlineage-events-facets-lineage]`.

**Upgrade moves**
1. Tables: `assets`, `asset_sources`, `asset_facets`, `utility_assessments`, `asset_fts` (external content + triggers), `asset_vec` (sqlite-vec).
2. FTS5 with `content='assets'`, `content_rowid='id'`, triggers for insert/update/delete; rank with weighted `bm25(asset_fts, 5.0, 1.0, 2.0)`; return `snippet()`.
3. Vector: `vec0` with `distance_metric=cosine`, metadata columns for `data_class`, `source`, `time_bucket`; KNN with `WHERE` constraints.
4. Hybrid rank: reciprocal rank fusion of BM25 + KNN; always show why a result matched (term vs semantic).
5. Utility: store `U(B)`, `U(B+X)`, rubric version, confidence, evidence IDs; recompute is deterministic; never a single opaque score.
6. Correction cascade: owner correction creates a new assertion + marks derived assets stale; a job recomputes dependents (G36).
7. DuckDB only for offline analysis/export over Parquet; SQLite remains the authority.

**Acceptance evidence:** re-ingest unchanged source → 0 new assets; search p95 and relevance fixture; correction removes derived items; utility report reproducible from raw evidence.

---

### F8 — Plugin platform and capability sandbox

**Surfaces:** `docs/plugins/*`; action catalog; MCP tools; extension points in Host/Studio.
**Atoms/gaps:** X-004; plugin frontier; G38.
**Best-at:** declarative-first manifest; per-plugin grants; sandboxed execution; versioned ABI; same semantics on phone/PC.

**Evidence**
- Extism: manifest, host functions, memory limits, allowed hosts, PDK `[P:extism-manifest-host-functions-limits]`.
- WIT/component model: worlds, imports/exports, versioning `[P:wit-worlds-imports-exports-versioning]`.
- MCP tool contract `[P:mcp-spec-architecture-transports-authorization]`.

**Upgrade moves**
1. V1 ships **declarative plugins only**: actions, views, schemas, policies, themes. A manifest JSON Schema; Host validates; nothing executes.
2. V2 adds WASM via Extism: host functions limited to `kv_read/kv_write`, `emit_event`, `call_tool`; no filesystem, no network, no CDP; memory/time limits; deterministic.
3. WIT world defines the ABI: `import host:{kv,events,tools}`; `export plugin:{init,handle}`; semver per interface.
4. Grants: manifest lists capabilities; owner approves per plugin; grants are records with scope/expiry/revocation. A plugin cannot widen its own grant.
5. Distribution: content-addressed bundle + signature; no marketplace in V1; local install only.
6. Placement: Host-executed for data/actions; phone-local only for pure view/theme; split for provider adapters (explicit broker).

**Acceptance evidence:** malicious plugin cannot read FS/network; timeout kills; manifest validation; version negotiation; revoke takes effect before next call.

---

### F9 — Device trust, pairing and local transport

**Surfaces:** `apps/vivim-host/apk/{pairing,enrollment}.mjs`; `apps/vivim-host/direct-phone.mjs`; relay cookie; `studio/src/chrome/browser-ownership.mjs`.
**Atoms/gaps:** UX-03; G12/G13; RUN-07/20; X-024; VAL02.
**Best-at:** QR pairing with one-time secret; hardware-backed device key; mutual auth; replay rejection; per-device revocation; no cloud dependency.

**Evidence**
- Noise handshake patterns/rekey/channel binding `[P:noise-handshake-patterns-rekey-binding]`.
- JOSE JWT/JWE/JWKS `[P:jose-jwt-jwe-jwks-key-management]`.
- Tailscale ACL/identity/serve `[P:tailscale-acl-identity-serve]`; Headscale preauth/ACL `[P:headscale-preauth-acl-node-registration]`; WireGuard keys/keepalive `[P:wireguard-keys-allowedips-keepalive]`.
- Android Keystore biometric keys + invalidation `[P:android-keystore-biometric-encrypted-storage]`; App Links `autoVerify` + assetlinks `[P:android-applinks-assetlinks-intent-filters]`; WebView `postWebMessage`/`addJavascriptInterface` security `[P:android-webview-jsinterface-postwebmessage]`.

**Upgrade moves**
1. Pairing payload: `{pairing_id, host_id, host_pubkey, secret, expires_at, transports[]}`; QR only; secret single-use, ≤120 s.
2. Phone generates a non-exportable signing key in Android Keystore (`setUserAuthenticationRequired`, `setInvalidatedByBiometricEnrollment`); proves possession on pair and on each session.
3. Session: Noise `XX` (or `IK` with a pinned host key) over loopback/Tailscale; bind channel to `host_id||device_id`; or a short-lived JOSE token with `cnf` if Noise is too heavy. Rotate keys; reject replayed nonce.
4. Store `devices{device_id,pubkey,label,created_at,last_seen,revoked_at}`; revocation is immediate and audited.
5. App Links: `autoVerify` + `https://<host>/.well-known/assetlinks.json` with SHA-256 cert fingerprint; exact host/path; reject unverified intents.
6. Tailscale: tagged nodes, expiring preauth keys, `serve` only `/mobile`; never touch other routes (RUN-07).
7. WebView: prefer `WebViewCompat.postWebMessage`/`addWebMessageListener` over `addJavascriptInterface`; restrict interfaces; never expose native objects to arbitrary origins.

**Acceptance evidence:** stolen/expired QR fails; replay fails; revoke one device leaves others; assetlinks verifies; no cloud secret needed for Direct.

---

### F10 — Relay, idempotency and event streams

**Surfaces:** `apps/mobile-relay/lib/relay-core.mjs`; `redis-store.mjs`/`blob-store.mjs`; `studio/src/studio/relay-agent.mjs`; `studio/src/studio/bridge-commands.mjs`.
**Atoms/gaps:** STATE-06/07/12/21; SEND-06/12/20; X-099; G14/G17.
**Best-at:** append-only command revisions; exactly-once claim; resumable SSE; no duplicate effects; bounded payloads.

**Evidence**
- Standard Webhooks signature + replay `[P:standard-webhooks-signature-replay-idempotency]`.
- EventSource parser: `Last-Event-ID`, retry, streaming `[P:eventsource-parser-resume-retry]`.
- Transactional outbox: idempotency, retries, nested outbox `[P:transaction-outbox-idempotency-retries]`.

**Upgrade moves**
1. Command envelope: `{command_id, idempotency_key, rev, expected_rev, actor, kind, payload, status, effect_fence}`; immutable revisions; latest-wins only with CAS.
2. Host outbox table written in the same transaction as the state change; relay worker drains with backoff; consumer acks; dead-letter after N.
3. SSE: `id:` = monotonic sequence; server honors `Last-Event-ID`; heartbeat every 15 s; `retry:` hint; bounded buffer per client.
4. Webhooks: `webhook-id`, `webhook-timestamp`, `webhook-signature`; reject >5 min; store IDs for idempotency.
5. Blob store: deterministic key `cmd/<id>/<rev>.json`; no random suffix; cleanup only after ack + retention.
6. Cloud fallback uses the same event types as Direct; polling is last resort and visibly marked.

**Acceptance evidence:** reconnect mid-stream → no gaps/duplicates; duplicate command → 409; relay outage → outbox drains; blob listing matches command count.

---

### F11 — Universal text control and action surface

**Surfaces:** `apps/vivim-host/control/{dispatcher,intent-planner,journal,lexicon}.mjs`; `action-catalog.v1.json`; `apps/mobile-relay/control-language.mjs`.
**Atoms/gaps:** UTC-01..07; G26/G27/G28; VAL01.
**Best-at:** one descriptor → GUI + grammar + MCP + docs; deterministic parse with AI fallback; risk-based preview; undo; discoverability.

**Evidence**
- Fuzzy ranking/unicode `[P:fuzzysearch-ranking-unicode]`.
- Typed routes/validation `[P:hono-routing-middleware-rpc-validation]`.
- JSON Schema/ Ajv for action args `[P:json-schema-2020-12-keywords-vocabularies]`, `[P:ajv-2020-12-standalone-formats]`.
- State-machine semantics `[P:xstate-actors-persistence-guards]`.

**Upgrade moves**
1. Action descriptor: `{id, title, verbs[], args_schema, risk R0-R3, preview, execute, undo, surfaces[], requires[]}`. Generate the catalog, text grammar, MCP tool and UI from it — no drift.
2. Pipeline: normalize → lexicon → grammar → candidate set → risk → preview → approval → execute → receipt. AI may propose candidates but they are validated against `args_schema` and re-authorized by Host.
3. Command palette ranking: normalized fuzzy match + recency/frequency + surface affinity; show why matched.
4. Unknown intent: return bounded help + top 3 actions; never guess-execute.
5. Keep the fixed executor allowlist (no shell/SQL/URL/code); R2/R3 require explicit approval.
6. Coverage test: every GUI control must map to an action ID and a text example; CI fails on gaps (G28).

**Acceptance evidence:** catalog text-parity test; unknown intent safe; R2 approval required; undo works; action count and GUI count reconcile.

---

### F12 — Observability, KPI and receipts

**Surfaces:** `tools/kpi/utc-maturity.mjs`; `tools/perf/*`; Host diagnostics; relay status.
**Atoms/gaps:** PERF-01..21; G39; VAL11.
**Best-at:** per-command hop timing; evidence receipts; privacy-safe telemetry; SLOs; replay.

**Evidence**
- OpenTelemetry JS: `NodeSDK`, traces/metrics/logs, context propagation, sampling, exporters `[P:otel-js-sdk-context-sampling-exporters]`.
- Pino: redaction, child loggers, serializers, transports `[P:pino-redaction-child-transports]`.

**Upgrade moves**
1. One span per hop: `phone.submit → relay.receive → host.claim → studio.tab_bound → composer.typed → send → first_token → reply.settled → phone.shown`. Attributes: `command_id`, `provider`, `model`, `outcome`; never prompt/reply text.
2. Metrics: histogram per hop; counters for `uncertain`, `duplicate`, `draft_present`, `auth_wall`; per-provider success rate.
3. Logs: Pino child logger per command; redaction paths for authorization/cookies/prompt/reply; local file rotation.
4. Default export: local JSONL/OTLP to a loopback collector; remote export opt-in and filtered.
5. KPI report reads the same spans; no parallel measurement code.
6. Receipts: every terminal state links to a receipt row; UI can open it (STATE-14).

**Acceptance evidence:** one command trace contains all hops; grep finds no prompt text in logs; p95 report reproducible; uncertain rate matches journal.

---

### F13 — Policy, authorization and egress

**Surfaces:** `apps/vivim-host/control/dispatcher.mjs`; `bootstrap/capability-activation.mjs`; approvals/standing; relay egress; plugin grants.
**Atoms/gaps:** OWN-06/08; G06/G35; WORLD-21; X-103; VAL03.
**Best-at:** one policy decision point; deny-by-default egress; scoped grants; audit; owner override.

**Evidence**
- Cedar: `forbid` wins, entities, authorization request, schema validation, partial evaluation `[P:cedar-policy-forbid-entities-validation]`.
- OPA: Rego, partial evaluation, decision logs, Wasm bundles `[P:opa-rego-partial-eval-decision-logs]`.
- ODRL rights model `[P:odrl-policy-permission-prohibition-duty]`.

**Upgrade moves**
1. Define entities: `Owner`, `Device`, `Action`, `Provider`, `DataClass`, `Plugin`, `Conversation`.
2. Policy decision: `authorize(principal, action, resource, context)`; deny by default; `forbid` overrides `permit`. Start with a small in-house evaluator or Cedar if the Rust/WASM dependency is acceptable.
3. Every effect goes through one gate: relay send, provider send, file write, plugin call, export. No bypass.
4. Global pause = an egress deny rule with highest priority; health check proves no send path bypasses it (G35).
5. Grants: `AuthorityGrant{id, principal, scope, expires_at, revoked_at, evidence}`; memory cannot create or widen a grant (WORLD-21).
6. Decision log: policy version, inputs hash, decision, reason; queryable from diagnostics.
7. ODRL mapping for data rights: permission/prohibition/duty + constraints; exportable.

**Acceptance evidence:** global pause blocks all sends; policy change logged; no code path bypasses the gate; grant revoke effective immediately.

---

### F14 — Packaging, startup and mobile runtime

**Surfaces:** `src-tauri/src/main.rs`; `tools/packaging/*`; `studio/android/build.ps1`; APK registry; `apps/mobile-relay/sw.js`.
**Atoms/gaps:** RUN-01..22; G01/G02/G03/G10/G11; X-027/X-091; VAL02.
**Best-at:** one-command idempotent start; signed update; cold-reboot recovery; APK integrity; honest readiness.

**Evidence**
- Tauri: updater signature validation, MSI bundle, sidecar, single-instance/deep-link plugins, capabilities `[P:tauri-updater-msi-sidecar-capabilities]`, `[P:tauri-window-autostart-deeplink-single-instance]`.
- Android: Keystore `[P:android-keystore-biometric-encrypted-storage]`; App Links `[P:android-applinks-assetlinks-intent-filters]`; foreground service/boot/battery `[P:android-foreground-notifications-boot-battery]`; APK signature schemes `[P:android-apk-signature-integrity]`.
- Service worker caching/update `[P:webdev-service-worker-caching-update]`; SQLite WASM/OPFS `[P:sqlite-wasm-opfs-worker]`.

**Upgrade moves**
1. Tauri updater: signing key + `pubkey` in config; `latest.json` per platform; verify signature before install; keep previous version for rollback.
2. MSI bundles the Node host as a sidecar; single-instance plugin; deep-link plugin for `vivim://pair`.
3. Startup state machine: `starting → degraded → ready | blocked`, per component; second `start` adopts, never duplicates (RUN-03).
4. Readiness is evidence-based: process identity + health endpoint + provider attach + phone ack; a tab being open is not readiness (G04).
5. Android: foreground service for long jobs with a persistent notification; boot receiver opt-in; battery-optimization explainer; `startForeground` within the required window.
6. APK: v2/v3 signing, cert fingerprint recorded in the release registry, `apksigner verify` in CI (G02/G11).
7. Service worker: precache shell; stale-while-revalidate assets; network-first API; `skipWaiting` + explicit update prompt; never cache prompts.
8. Offline phone: SQLite WASM/OPFS only if needed for last-known state; otherwise IndexedDB with the same projection contract.

**Acceptance evidence:** reboot → login → phone ready; update signature verified; APK verify passes; offline shell loads; second start is idempotent.

---

## 3. Repository / library identification

Selected via 59 recorded Context7 searches/resolves. Stars/trust are Context7 metadata at pull time, not an endorsement.

| Family | Context7 library | Stars | Trust | Why |
|---|---|---|---|---|
| F1 | `/yjs/yjs` | 19,083 | 9.7 | smallest proven CRDT update/awareness model |
| F1 | `/automerge/automerge` | 4,736 | 9.2 | save/load/merge/heads + conflict patches |
| F1 | `/automerge/automerge-repo` | 639 | 9.2 | storage/network adapter separation |
| F1 | `/electric-sql/electric` | 8,125 | 8.9 | shapes/partial replication semantics |
| F1 | `/powersync-ja/powersync-js` | 430 | 9.1 | upload queue + conflict handling |
| F2 | `/durable-workflow/workflow` | 1,188 | 9.0 | durable steps/signals reference |
| F2 | `/rivet-dev/actors` | 6,135 | 9.8 | actor lifecycle/state/RPC |
| F2 | `/statelyai/xstate` | 28,195 | 9.2 (docs lib) | state machine + persistence |
| F2 | `/langchain-ai/langgraph` | 12,599 | 9.2 | checkpoint/interrupt semantics |
| F3 | `/modelcontextprotocol/modelcontextprotocol` | 3,050 | 9.0 | MCP spec |
| F3 | `/modelcontextprotocol/typescript-sdk` | 6,464 | 9.0 | TS server/client |
| F3 | `/websites/a2a-protocol` | — | 7.7 | A2A spec |
| F3/F5 | `/ag-ui-protocol/ag-ui` | 746 | 7.2 | agent→UI event protocol |
| F4 | `/chromedevtools/devtools-protocol` | 1,233 | 8.2 | CDP domains |
| F4 | `/chromedevtools/chrome-devtools-mcp` | 54 | 8.8 | attach/inspect patterns |
| F5 | `/json-schema-org/json-schema-spec` | 4,261 | 7.5 | 2020-12 contract |
| F5 | `/remoteoss/json-schema-form` | 142 | 9.4 | uiSchema/widgets |
| F5 | `/tldraw/tldraw` | 39,935 | 9.5 | canvas store/records/bindings |
| F5 | `/xyflow/xyflow` | 29,143 | 9.5 | controlled graph UI |
| F6/F7 | `/openlineage/openlineage` | 2,046 | 7.8 | lineage event/facets |
| F6/F13 | `/w3c/odrl` | 22 | 9.8 | rights/prohibition/duty |
| F6 | `/json-ld/json-ld.org` | 887 | 7.4 | portable context/framing |
| F7 | `/websites/sqlite_docs` | — | 9.7 | FTS5/WAL/JSON1 |
| F7 | `/asg017/sqlite-vec` | 5,633 | 10.0 | local KNN |
| F7 | `/duckdb/duckdb-web` | 214 | 8.9 | offline analytics/Parquet |
| F8 | `/extism/extism` | 5,070 | 9.8 | WASM plugin host |
| F8 | `/bytecodealliance/wit-bindgen` | 1,222 | 9.6 | WIT ABI |
| F9 | `/juanfont/headscale` | 27,475 | 8.1 | self-hosted tailnet control |
| F9 | `/tailscale/tailscale` | 22,293 | 9.9 | ACL/identity/serve |
| F9 | `/holepunchto/noise-handshake` | 20 | 9.7 | Noise patterns |
| F9 | `/panva/jose` | 6,381 | 9.7 | JWT/JWE/JWKS |
| F9 | `/websites/wireguard` | — | 10.0 | key/keepalive model |
| F9/F14 | `/websites/developer_android` | — | 10.0 | Keystore/App Links/WebView/services/signing |
| F10 | `/standard-webhooks/standard-webhooks` | 1,620 | 7.8 | signature/replay |
| F10 | `/rexxars/eventsource-parser` | 373 | 10.0 | SSE resume/retry |
| F10 | `/gruelbox/transaction-outbox` | 324 | 7.6 | outbox/idempotency |
| F11 | `/lithammer/fuzzysearch` | 1,321 | 9.2 | fuzzy ranking |
| F16 | `/honojs/hono` | 23,823 | 7.3 | typed HTTP/RPC |
| F12 | `/open-telemetry/opentelemetry-js` | 3,018 | 9.7 | traces/metrics/logs |
| F12 | `/pinojs/pino` | 15,300 | 9.5 | redaction/child loggers |
| F13 | `/cedar-policy/cedar` | 1,357 | 8.5 | forbid/entity policy |
| F13 | `/open-policy-agent/opa` | 10,229 | 9.2 | Rego/decision logs |
| F14 | `/googlechrome/web.dev` | 3,594 | 9.0 | service worker lifecycle |
| F14 | `/sqlite/sqlite-wasm` | 862 | 6.6 | OPFS persistence |
| F14 | `/tauri-apps/tauri` | 91,940 | 9.5 | updater/MSI/sidecar |
| F16 | `/colinhacks/zod` | 37,520 | 9.6 | TS schema |
| F16 | `/ajv-validator/ajv` | 14,172 | 9.1 | runtime JSON Schema |
| F16 | `/websites/spec_openapis_oas_latest` | — | 9.9 | OpenAPI 3.1 |

**Not selected / not available in Context7:** Temporal (no library hit; used Durable Workflow as semantics reference), SPAKE2 (no usable JS library hit; use Noise/JOSE instead), A2UI (not found; AG-UI covers the event layer), Automerge as World authority (rejected on scope grounds, not quality).

---

## 4. Cross-cutting contracts to standardize

These are the contract surfaces the 14 families share. Standardize once; reuse everywhere.

1. `vivim.envelope/1` — `{schema, id, rev, idempotency_key, actor, created_at, payload_hash}`.
2. `vivim.job/1` — state machine + lease + receipt pointer.
3. `vivim.conversation/1` — Host-owned ID + provider binding + tab binding.
4. `vivim.world-op/1` — append-only op with causal parents.
5. `vivim.claim/1` — observation vs assertion, status, temporal validity, evidence IDs.
6. `vivim.data-asset/1` — already proposed; add OpenLineage-style facets.
7. `vivim.ui-proposal/1`, `vivim.ui-revision/1`, `vivim.render-plan/1` — already proposed; add `surface_id`.
8. `vivim.action/1` — descriptor → GUI/grammar/MCP/docs.
9. `vivim.policy-decision/1` — principal/action/resource/context/decision/reason.
10. `vivim.receipt/1` — terminal state + evidence links + hashes.

Rule: every contract has a JSON Schema, a version, a migration note, and a replay test. No contract is "documented" without a validator.

---

## 5. Priority waves

**Wave 0 — stop the trust leaks (P0, weeks):** F1 conversation ID (G15), F2 shared job/lease (G17), F13 egress pause (G35), F9 device-bound pairing (G13), F12 receipt spans. These are prerequisites for every later claim.

**Wave 1 — make the phone path honest:** F4 attach/read-back, F10 SSE resume + outbox, F14 readiness + update signature, F11 text parity. Target: one full journey with receipts (pair → connect → send → reply → reconnect).

**Wave 2 — make Build real:** F5 revision-safe renderer, F6 claim/evidence kernel, F7 asset index + hybrid search. Target: F02/F01 → Build surface → undo, with evidence.

**Wave 3 — platform edges:** F3 MCP server + agent card, F8 declarative plugins, F6 export/import. Target: one external tool and one declarative plugin under grants.

**Wave 4 — hardening:** policy engine (Cedar/OPA), WASM plugins, multi-device, packaging polish.

Sequencing constraint: F2 and F13 precede F3/F8 (otherwise tools/plugins inherit an unaudited effect path). F1 precedes F5/F6/F7 (otherwise surfaces point at unstable identity).

---

## 6. What not to adopt

- **CRDT everywhere.** Yjs/Automerge solve merge, not authority, audit or policy. Use them at the editable edge; keep the op log and evidence store ordinary SQLite.
- **Event sourcing for every table.** Use it for effects/commands and World revisions; keep read models and config simple.
- **A Python/Temporal/LangGraph runtime.** Port the semantics (checkpoint, interrupt, retry) into the existing Node stack; adding a second runtime multiplies the supervision problem.
- **MCP as the internal plugin ABI.** MCP is an edge protocol; internal plugins need a capability manifest + Host broker first.
- **WASM plugins in V1.** Declarative plugins deliver value without a sandbox to defend.
- **Full Cedar/OPA adoption on day one.** Start with one decision function and a small policy set; adopt an engine when the policy surface justifies it.
- **Tailscale as the only transport.** Direct is a differentiator; keep loopback + QR pairing working without a tailnet.
- **A public plugin marketplace.** Distribution is a security problem before it is a growth problem.

---

## 7. Evidence ledger

- Canon store: `C:\0-BlackBoxProject-0\SOFTWARE_HQ\context7\`; manifest `index.json` now 176 entries.
- This program issued **115** recorded Context7 calls: 59 searches/resolves + 56 doc pulls (2 probe + 54 targeted).
- All 54 targeted doc pulls returned 15–24 KB each; ~1.1 MB of verbatim docs; 2,472 headings. Every pull records exact `libraryId`, `query`, endpoint, status and body.
- Request twins: `context7/requests/<stamp>-<slug>.request.json`; batch plans: `context7/plans/2026-11-02-*.json`; runner: `context7/batch-run.mjs`.
- Key pull slugs are cited inline as `[P:slug]`. To re-read one: `Get-Content context7/raw/*-<slug>.json`.
- No VIVIM runtime claim is made. Source paths are cited from the repository, not inferred from Context7.

**Known evidence limits:** Context7 is a documentation index, not a benchmark; trust scores are metadata. Android pulls are reference docs, not device receipts. MCP/A2A/AG-UI are moving specs; version pins must be recorded at implementation time. The GPL/AGPL status of any code copied from these projects was not assessed; use them as design references, not vendored source.

---

## 8. Next pulls (wave 2, not yet recorded)

These are the remaining contract gaps worth pulling before design freeze:

1. JSON Patch (RFC 6902) and JSON Merge Patch — revision application semantics.
2. OAuth 2.1 / DPoP — proof-of-possession tokens as a Noise alternative.
3. WebAuthn Level 3 — passkey option for desktop owner auth.
4. A2UI — if it exists as a distinct spec; else record the dead end.
5. OpenTelemetry semantic conventions — stable attribute names for the hop spans.
6. WCAG 2.2 — generated-UI contrast/focus/motion criteria.
7. ULID/UUIDv7 — command ID ordering.
8. MCP authorization — resource indicators and token audience.
9. Trusted Web Activity / Digital Asset Links — if the APK wraps the PWA.
10. JSON Schema output/`format` vocabularies — strict proposal validation.

Each will be recorded with the same runner; a failed resolve is recorded as evidence of absence.

---

## 9. Immediate next actions (design, not code)

1. Write `vivim.job/1` + `vivim.conversation/1` schemas and a replay test plan (F1/F2).
2. Add a single `authorize()` gate and an egress deny path; wire the global pause test (F13).
3. Specify device pairing payload + Keystore proof + revocation (F9).
4. Add hop spans and redaction to one end-to-end command (F12).
5. Freeze the proposal/revision IDs and fuzz corpus for Build (F5).

These five produce the evidence the rest of the program depends on.
