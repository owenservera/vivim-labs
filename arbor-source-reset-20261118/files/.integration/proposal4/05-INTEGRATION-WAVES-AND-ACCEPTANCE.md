# Proposal 4 — First Master Integration: One-Time Convergence Without Wholesale Rewrite

**Status:** execution strategy proposal only.  
**Primary objective:** integrate the current VIVIM product into an extensible, composable, programmable foundation with one truthful operational model.  
**Explicit exclusion:** this is not a greenfield rewrite of every feature.

## 1. What "one-time" must mean

The owner wants one complete architectural integration, not repeated rounds that each add a new system to wire. This proposal commits to **one canonical destination, one transition in semantic ownership, one release identity and one controlled product cutover**.

It does **not** mean one massive untestable commit or one fragile instant replacement. Work can occur as independent source-verified migration slices on an ARBOR branch while the existing product continues to work. The old product remains authoritative until the destination passes real acceptance. At cutover, only one owner writes effects, and old live entrypoints are retired together.

The first integration should not block on replacing Node/JS, rewriting Studio Chrome adapters, rewriting TierFlow or migrating every pure graph algorithm into custom native code. Those are **future implementation substitutions under stable ports**, not requirements for a single logical system.

## 2. Reuse-before-replace sequencing

### Phase P0 — Ground truth and selective source audit

Inputs:
- Harness command/action catalog, JS Host dispatch/journal, Tauri origin/bootstrap, mobile pairing, Studio session exact-send.
- HQ Rust engine/typed DAG, source identity, command/wiki development algorithms.
- TierFlow rendered visual runtime, typed inert commands and phone adapters.
- Labs Node 0 Prime, R007 VIEW ALL, knowledge/DCB, AutoWiki/rock storage, Chronos, voice-to-tasks, extensions, observatory.
- Legacy private `vivim` and `vivim-platform` as donors only.

Outputs:
- machine-readable source-to-capability inventory with exact repo/path/commit SHA;
- unique canonical IDs and ownership, source authority, data classification, effect implications;
- classification `BIND/PROMOTE/ADAPT/CONVERGE/REPLACE-LATER/ARCHIVE/REVIEW`;
- gap and duplication report identifying every active runtime consumer;
- scoped security analysis before exposing private data or native commands.

**Gate:** no runtime-owner or effect-journal replacement before complete writer census and exact rollback plan.

### Phase P1 — One typed identity/port/trace protocol

Do the small but decisive structural work first:
1. Define canonical `IdentityRef, SourceBinding, CapabilityDescriptor, OperationIR, ProgramIR, ViewProjection, Signal, EffectStatus, ReceiptRef` with Rust domain types and TS runtime validation generated from one source.
2. Bind current Host action catalog IDs/risks/source status as **source-provided capabilities**, not ARBOR replacement registrations.
3. Bind HQ source semantic IDs/graph engine slots and TierFlow visual command IDs by verified crosswalk.
4. Represent `declared/source_wired/build_tested/runtime_available/authorized/observed/verified` as different statuses.
5. Define source/policy/version vector for heterogeneous graph joins.
6. Build a pure read-only test harness that can enumerate accepted capabilities without needing provider authentication.

**Gate:** same identity/status across CLI, Rust, browser UI adapter, source view; missing sources remain unknown.

### Phase P2 — Reuse HQKG computation inside the integration backbone

1. Import/build/pin existing HQKG Rust library or a source-preserving dependency with generation leases and typed DAG.
2. Add one compilation/read-adapter path that maps existing Host/HQ/TierFlow source ports to typed DAG input/output.
3. Use existing engine registry; do not build a new scheduler or universal event store.
4. Add bounded reactor subscriptions and input-revision handling for Node 0; avoid R001 per-keypress thread creation and whole-core mutex.
5. Implement short input snapshots and asynchronously converged source-compatibility projections, with partial/stale data.
6. Restrict private-source access per producer and purpose.

**Gate:** one typed `DraftChanged` can run multiple pure independent existing computations and return one coherent, revisioned, privacy-safe trace while original text/IME remains responsive.

### Phase P3 — Integrate the *existing* product as capability modules

Bridge (don't wholesale copy) the currently operating subsystems:
- Host `vivim.status.show` read-only control action and catalog enumeration.
- Current World/project-source read adapter if it actually exists and authenticates.
- Studio Chrome signed-in provider read and one admitted send-once proof, behind Host effect port.
- Existing mobile paired session and read-only source link.
- Existing DCB composer / current context source selection, when source/authorization proof exists.
- Existing media library read and streaming, with resource and path safeguards.
- TierFlow visual runtime's existing inert document/view projection.

Use first-class module descriptors to group port identity, source owner, view, tests and configuration. Program compiler composes the modules but cannot invent an operation if no verified source owner implements it.

**Gate:** read-only UI and CLI can explain which capabilities are running, via what implementation and source revision, and exactly why any module is unavailable. External effects still admitted by the *same existing Host writer*.

### Phase P4 — Unify the user-facing experience as projections

1. Mount one responsive UI shell for desktop and paired PWA; avoid separate command implementations per surface.
2. Present R005 visual semantics, R006 predictive slots and R007 VIEW ALL over the same native trace as source tests allow.
3. AutoWiki becomes a view recipe over current authorized graph with a bounded Rock Core and quota-controlled materializations; no independent article truth store.
4. Mission feed, status, conversation, project context and app-builder view use the same capabilities/identity and state transitions.
5. UI can create/edit *program proposals* visually and versioned configuration changes, not direct effect grants.
6. Preserve IME/accessibility/local editor and explicit desktop/phone transport divergence without semantic drift.

**Gate:** one chosen source resource appears with the same ID, authority, revision and effect state in chat, wiki, mission and visual inspection. Offline PWA does not fabricate online proof.

### Phase P5 — Make extension/programmability future-ready without a feature explosion

Implement **only the extension substrate and 2–3 source-verified examples**, not a giant plugin ecosystem:
- A configurable, read-only `status → format → view` program built from actual Host status/catalog sources.
- A real source-backed project/context read → graph computation → AutoWiki view if current project access is available.
- A user-configurable view/profile recipe with revisioned preview, activation and rollback.
- One trusted, replacement-safe pure Rust engine behind a generation-pinned capability port.

The extension architecture must describe future placement/resource/scope constraints **even if loading arbitrary third-party code is not implemented**. Validate one honest `UNAVAILABLE` for uninstalled/unsupported ports.

**Gate:** a new read-only capability view does not require introducing a second frontend/backend source of truth, independent effect API or graph store.

### Phase P6 — Security, reconciliation, migration and one actual product cutover

Critical tests before cutover:
- Tauri dependency audit and upgrade to a patched supported v2; exact window/origin IPC restrictions.
- Authenticate Host process instance; stop sending long-lived owner tokens to an unverified localhost listener.
- Actor/device/session scope isolation; phone stale/revoked grant denial.
- Full 500-edit/stale/IME/Unicode test, slow/hanging reactor, source revocation and privacy denial.
- Host JS effects/claim/receipt exact parity; crash recovery before/after claim and external send. **Do not double-write.**
- Windows packaged installer (clean install + upgrade + rollback) and physical paired PWA journey.
- Source-owned data backups: World, personal graph, DCB definitions, conversations, media indexes, original documents, receipts. Preserve old source IDs/links in migration, and never commit private data.
- Build/CPU/RAM/disk evaluation on a modest Windows laptop; degradation should remain usable even if graph/corpus is incomplete.
- Verify all modules are either integrated, intentionally unavailable, or explicitly deferred with a source evidence trail; never silently drop a live capability.

**Cutover:** old product effect writer stops and in-flight claims are reconciled; new ARBOR release becomes the single app identity; rollback can restore the previous reader/writer safely without two active writers. Remove old launchable paths from delivered artifacts only after observed parity.

## 3. The minimum runnable proof — without inventing a missing capability

**Use `vivim.status.show`, not the fictitious `fixture.full.update`, for the first live source-backed test.** The Host catalog version 6 declares `vivim.status.show` as `source_wired`; runtime availability and execution must still be demonstrated on the actual PC.

Scenario:
1. User opens ARBOR desktop composer and types `status`.
2. The local editor paints immediately. Node 0 emits a versioned event to the bounded graph.
3. Existing Host action catalog and current status read port are source-adapted into a typed `Capability`; additional source diagnostics may run in parallel if permitted.
4. The system projects a status candidate to TierFlow UI and CLI with the same ID and source revision.
5. An explicit request uses the current Host read-only control port; no grant is invented by the UI.
6. The response becomes source-linked evidence; the UI, wiki and diagnostics share it.
7. Change the view configuration (compact/detailed) and show the same underlying source ID/status.
8. A missing Host or stale source creates an honest unavailable/stale projection; typing stays responsive.

This tests identity, typed port binding, source truth, composition, view reuse, pure effects separation, resource budgeting and failure behavior **before migrating any sensitive data**.

Second proof (when source exists): project read → HQKG scoped graph transform → two different views (chat and AutoWiki) with no new content store. Third proof (only after gating): committed effect → single Host journal → Studio observed receipt and crash-safe uncertainty.

## 4. First integration *not* required to accomplish

- Rewrite all Host JS modules in Rust.
- Replace current signed-in Chrome driver with a Rust CDP implementation.
- Rewrite TierFlow visual runtime and animation system.
- Replace all data formats with a new graph database.
- Implement all 153 candidate Lab extension surfaces as live hooks.
- Build a general-purpose untrusted native plugin loader or cloud marketplace.
- Migrate old provider-API experiments into the product.
- Promote every Labs algorithm or VIVIM history branch without proof.
- Invent real-time verified `full update` capability when no source owner implements it.
- Build APK instead of making one reliable paired PWA.
- Add extra permanent daemon/services just to decouple work that can use an in-process bounded executor.

## 5. Completion gates and evidence ledger

| Gate | What must be observed before GO |
| --- | --- |
| A — System identity | Same stable IDs, version pins and source-owner references across Rust, TS, Host, CLI and view |
| B — Single truth | View/mission/wiki/program statuses derive from source owners; no divergent action readiness/grants |
| C — Typed composability | Real source-backed capability ports compose with deterministic checking, explanation, cancellation and failure handling |
| D — Node 0 responsiveness | Locally lossless editor and bounded native computation; fast typing never blocked by external source work |
| E — Security and privacy | Patched Tauri, exact Host identity, scoped actor/session/source admission and restricted view audience |
| F — One effect writer | Existing Host or migrated Rust implementation, never both; claims, idempotency, uncertain state and receipts proven |
| G — Configuration | Owner-scoped config change preview/activate/revert and quota, with no permission mutation |
| H — Extensibility | Source-verified new declarative view or trusted pure transform works without new service or manual duplicate catalog |
| I — Labs graduation | Each selected lab component has source/test/owner/disposition; no false integration or production claims |
| J — Windows/PWA product | Clean install, old-data safe migration, paired phone, physical device and representative low-resource benchmarks |
| K — Controlled retirement | All legacy entrypoints classified and retired only after direct consumers are observed migrated and rollback tested |

Each gate output: exact repo/branch/SHA, file path, prior owner, adapter version, test command, environment, status, observed result, input/source data class, risk, actor/policy proof, rollback and approving owner. No test files == a passed test.

## 6. Risks specific to this proposal

- **Over-abstraction:** an ontology/typed model designed too broadly can consume the entire budget before product integration. Limit v1 types/ports to observed source needs, expand only with working slices.
- **Adapter permanence:** if wrappers are never retired, ARBOR could become a thin facade over a fragmented product. Track adapter count, call depth, latency and replacement condition as first-class metrics.
- **One system truth becoming global locking:** do not require a transaction across HQ, Studio, PWA and Host sources for every read. Use source-version vectors and honest partial/unknown states.
- **Composability as policy bypass:** capability ports and programs are not grants. Effects stay behind Host admission.
- **Schema version proliferation:** choose one IDL and generated validators, not a YAML spec per owner with independent semantics.
- **Over-extensible UI:** allow visual innovation without permitting views to author authoritative claims.
- **Too much speculative work on a full local corpus:** bound Rock Core, caches and predictive processing by default.
- **Mis-scoped migration:** first integration must know exactly which owner currently writes each personal/mission/effect record; source documents alone do not certify live wiring.
- **Unreviewed old private code:** do not import tracked `node_modules`, old API provider configuration, session profiles or credentials.
- **Unsafe false GO:** only device and source-backed evidence can promote an adapter to accepted status.

## 7. What "future-proof" realistically buys

The architecture can absorb better Rust algorithms, a new provider web UI, future voice/gesture capture, user-defined apps, specialized knowledge sources and a different renderer **without changing the canonical identity, source, program, approval or receipt semantics**.

It cannot predict every future UI requirement, guarantee arbitrary third-party sandbox security, avoid protocol migrations, or eliminate hardware and network boundaries. Versioned ports, typed compositions, generational engines and reversible config are the practical foundations for evolution.

**One-time integration success is behavioral unity with safe extensibility, not proof every implementation has already moved to one programming language.**
