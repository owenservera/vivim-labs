# ARBOR Proposal 5 — Foundational Architecture / FA-001

**Owner:** ARBOR foundational product architecture and design | **Date:** 2026-11-12 | **Disposition:** SOURCE-BASED PROPOSAL / NOT RUNTIME ACCEPTED.
**Scope:** Cross-cutting VIVIM foundation, starting from the Semantic Wiki Compiler / deterministic composer / VIEW ALL / AutoWiki / Rock Core research and expanding to native extensibility, trusted effects, personal knowledge, developer programmability, PWA, automation, Studio and media/voice/app-building seams.
**Canonical evidence:** [SOURCE-ATLAS.v1.json](./SOURCE-ATLAS.v1.json), 43 selected blob-pinned actual source/research entries; exhaustive repo scan/runtime acceptance not claimed.
**Destination:** ARBOR single self-contained repository. **Do not merge or edit current Wave 1 A–F workers, G, root manifests, main or active runtimes from this architecture branch.**

## 0. Decision: one reprogrammable system, not a new platform parallel to ARBOR

VIVIM's differentiated architecture is **natural language + directly manipulated visual UI + native programmable capabilities**, backed by privately owned source/context and Host-owned effect truth. The wiki is an explanatory and *compiled semantic read service*, not the central dispatcher. A source comment or prompt can declare meaning, but cannot mint authority. The native engine evaluates bounded pure computation and projections; the Host retains exact effect authority. No external provider APIs. Existing signed-in Chrome web-app interactions stay in the existing Studio/browser owner, without launching a new Playwright/Puppeteer worker.

**Baseline hard constraints grounded in actual code:**

- **A Rust:** `native/hqkg/src/engine.rs` already has generational `EngineRegistry`, immutable leases, `Scheduler` and cancellation; `typed_dag.rs` has versioned namespace/source/cache semantics; `scoped_graph.rs` validates *structure*, not authentication; `core/programmability/src/lib.rs` validates pure programs without network, persistence, timers or permission grants.
- **B Host:** `intent-planner.mjs` handles deterministic input against the v6 action catalog; `command-program.mjs` produces an inert multi-command DAG; `dispatcher.mjs` owns approved effects and journal; `personal-rule-store.mjs` requires an owner-approved verifier.
- **C Studio:** `plugins/registry.mjs` is a deterministic registry, not an installed sandbox; `recipes/catalog.mjs` is a read-only declared recipe list; the personal-agent and DCB sources are at different maturity levels.
- **D HQ:** `annotation-v2.mjs`, semantic-registry, wiki-program, comment-wiki-bridge and arbor-service are real source-backed software. They implement **distinct languages/owners**. They must not be merged indiscriminately into a single parser, registry or writer.
- **E PWA/TierFlow:** `control-language.mjs` is a browser-safe semantic peer of Host NCLL; `visual-runtime/command/compile.ts` creates inert typed visual commands; `generative-ui/compiler.mjs` prepares UI proposals; `sw.js` caches shell only and never replays API/effects.
- **F:** `contracts/interop/v1` is a *type mirror/projection schema*, not authenticated runtime contract ownership.
- **Labs:** genuine research code exists for ChatGPT T=0 streaming/normalization, voice-to-task proposals, source-observatory and pure semantic prototypes; broad plugin marketplace, native fan-out, predictive 100-ideas, universal AutoWiki/VIEW ALL and autonomous agent orchestration are NOT accepted deployed capability.

### Three planes, one authority map

```text
USER PLANE
  Desktop composer + native source-aware PWA + TierFlow views + AutoWiki
    │ typed gestures, drafts, voice partials, keyboard, Tab, source-linked views
    ▼
MEANING / COMPUTE PLANE (PURE, REPROGRAMMABLE, INERT)
  ARBOR HQ semantic IDs + source comments + WikiIR + DCB read adapters
        ├─→ compiled semantic grammar packs (approved, source attested)
        ├─→ shared Rust engine registry + typed DAG + bounded event fanout
        ├─→ source/context/Rock semantic read graph
        └─→ one revisioned InterpretationTrace, many visual density masks
    │ no grants; unresolved identity or stale source => refusal / clarification
    ▼
AUTHORITY / DURABILITY PLANE
  Host World, action catalog, original-text validation, policy/approval,
  journal/receipt, owner personal data + DCB consent / user-data lifecycle
        ├─→ Studio exact signed-in Chrome thread adapter (send-once evidence)
        ├─→ TierFlow UI World draft/publish owner as delegated/validated
        └─→ native OS/device resource gateways only with verified Host grant
              ↓ actual result/receipt/evidence status
  Read-only feedback → user / HQ source observations / AutoWiki / trace
```

**Design rule:** capability IDs and source refs connect the planes; they do not collapse their different authorities. HQ development wiki must not become a public copy of personal knowledge. HQ kernel store is not the Host World/receipt ledger. Native scheduler is not an owner effect dispatcher or durable automation scheduler.

## 1. Reprogrammability: concrete contract, not a label

A user or developer can **reprogram a capability** only by going through a complete controlled lifecycle:

1. **Describe** a capability source (`sourceRef`, owner, version, typed signature/slot schema, effect class, test anchors, meta-comment refs). Proposals are inert.
2. **Compile** a deterministic frame, source-comment semantic IR and optionally a pure computation graph in existing HQ/native tools. Unknown/deprecated IDs, unsafe comment handlers, untyped effects and grammar collisions fail closed.
3. **Constrain** with policy: allowed source namespace, owner, read/write class, resource bounds, output audience, mutable state boundary, expected revision, revocation, rollback and provenance.
4. **Prove** in source + synthetic negative tests. A test report must be *actually executed* and tied to an immutable source digest; imported code/test files are not PASS.
5. **Review/admit** a versioned data-only capability/grammar/visual package through existing owner registries. For effects, no activation without Host action-catalog binding and its existing approvals. A capability cannot add arbitrary handler IDs to Host.
6. **Activate** a new generation of pure Rust engine/grammar or a data-only UI/recipe projection with compare-and-swap expected version. Existing pinned generations remain usable until drained; revocation prevents new effects and can suppress old private read results.
7. **Observe** interpretation coverage, latency, reasons, denial, runtime engine generation, deterministic replay, action receipts, disk use and failure/uncertain outcomes using the existing observability/Host journal.
8. **Rollback** to prior approved version via existing owner writer and source pin; native hot swap never rewrites a historic Host effect, restarts an uncertain send or restores revoked personal data.

**Not implied:** arbitrary external code execution, a trusted plugin marketplace, free-form semantic comment execution, hot loaded unreviewed WASM/native binaries, a new shared action catalog or an AI training process that upgrades its own permissions.

### Proposed minimum capability package interface (unimplemented)

`CapabilityManifest{capabilityId,packageVersion,ownerDomain,sourceRef,compilerVersion,semanticGrammarRef,typedInputs,typedOutputs,effectRisk,allowedAudiences,pureEngineSlot?,resourceBudget,visualRoles[],testsDigest,compatibility,revocationClass}`.

`CapabilityActivation{manifestDigest,expectedRegistryRevision,expectedWorldRevision,admissionOwner,ownerDecisionReceipt,engineGeneration?,status}`.

Pure engines may be hot-swapped via existing `EngineRegistry` with generational leases; Host effects remain catalog-owned, and a UI program is data-only with its own publisher/World revision. Native infrastructure must build and test the real serializable typed ABI in Rust/TypeScript/JS; a paper schema without a consumer remains DESIGN_ONLY.

## 2. User intent → semantic compilation → real capability execution

Use source-linked semantic meaning as the common language of the system, anchored to existing code:

- **Source truth:** D `annotation-v2`, `semantic-registry`, `wiki-program`, comment-wiki bridge and HQ source walker.
- **Research staging:** pre-G `S-semantic` carries an exact pure HQ source compiler and inert adapter. There is no accepted `@hq:semantic/3` parser. The existing Rust `@hq:wiki` comment program is a **different grammar**; adapters must preserve both and only converge in reviewed semantics.
- **Local hot interpreter:** Existing E browser-safe Omega control language remains a fast pure first pass; native shared Rust may later compute bounded slots, clauses, entity graph and typed DAG *without becoming a competing Host planner*.
- **Context:** D authorized source refs + B authorized personal read ports; no private source content in development/CI machine wiki.
- **Submission:** candidate DAG and source-index evidence are inert. B Host recomputes original owner text/selected typed fields against current catalog/world/grants, asks for approval and owns effect receipts.
- **Feedback:** a common source-input revision receives multiple parallel pure producers (lexicon, source context, DCB refs, source-wiki, time, capability state, optional model proposal) and emits a read-only `InterpretationTrace`. Each node reports identity, confidence origin, source revision, privacy/producer coverage, authority status and actual receipt status separately.

**UI rule:** a visual prediction, double-circle, icon color, node wire or selected Tab slot must never be interpreted as permission. Wording such as "understood" is not "authorized"; "source wired" is not "device verified"; "predicted" is not "persisted memory."

## 3. Native event machinery: full bidirectional pipeline

**Existing:** A engine registry, typed DAG, pure programmability; Labs `user-io-rust-core` proposes independent subscriber fanout, and an isolated P5 N0-IO branch captures bounded event contract. **Missing:** a verified Host/Tauri actor attestation, real Rust input ingress/producer coordinator, cancellation/epoch semantics across Host/PWA, and end-to-end device parity.

**Proposed native backbone inside existing crates, not a new daemon:**

- **Ingress:** `DraftChanged|TabAccepted|SubmitRequested|VoicePartial|VoiceFinal|FocusChanged|SourceRevoked` with trusted actor injected by verified Host/native boundary. Do not trust actor/grant asserted in browser payload.
- **Snapshot:** `{actorScope,sessionId,textRevision,sourceEpoch,worldRevision,policyRevision,engineGenerations,locale,audience,traceId}`. On IME composition, render local text first and avoid contradictory committed effects.
- **Fanout:** bounded independent *pure* subscribers in A's `EngineRegistry`/typed DAG (not serial user-visible central router): syntax, source lookup, wiki IR, entity resolution, DCB read hints, intent constraints, predictive completion, visual hints. Each has deadline, budget, cancellation and producer ID.
- **Reducer:** a pure, bounded version comparison merges only matching snapshot/producer revision evidence into one trace. A late async result is skipped with explicit coverage state. Source revoked mid-flight invalidates private items and provenance.
- **Outgress:** TierFlow/Host or PWA can show trace deltas, suggestions, article refs and actual external Host status. PWA never accepts a self-generated authorization grant.
- **Effect boundary:** `SubmitRequested` is not an effect. The Host validates the *original text* and current decision context; confirmation/receipt is a separate event. Unknown sends are terminal-uncertain until owner reconciles, never auto-retry.

**Reprogramming proof:** install two pure computation generations under a known engine slot; pin old lease, swap to new verified generation, issue simultaneous event scopes, cancel/stale one, and demonstrate deterministic output without permission changes or second graph.

## 4. Context, personal intelligence, DCB and safe knowledge

Three distinct data scopes:

- **Development:** D HQ code/docs/comments/test evidence/wiki. Source-index SQLite and generated wiki are rebuildable, *except* any actual authoritative ledger/unique provenance/decision records protected by their writer.
- **Personal:** B Host World, user-supplied exports, preferences, consent and private knowledge. C Studio has DCB definition metadata and agent scaffolding; Labs has import/normalization staged code. These are not proof of a complete accepted user-memory service.
- **Presentation:** E TierFlow UI World, reversible drafts, source-revision overlays and phone cache. A visual page is a projection, not an authority or secondary personal DB.

Dynamic Context Bundles must distinguish a **flavor/family hint** from a verified persisted definition ID and from a projected bundle at the current source revision. Any automatic learning writes only into Host accepted owner-scoped personal rules or memories after provenance, consent, retention and deletion gates. A model proposal can never silently become durable user preference.

**AutoWiki** = ephemeral screen-state mirror backed by *authorized* sources. Preserve source origin and recency, but recent is not necessarily accurate or authoritative. A screen switch must not regenerate an ever-growing wiki archive or create a new permanent memory. Personal historical conversation text stays in owner custody and must respect source deletion/revocation.

## 5. Rock Core, storage and observability

No user should need to manually babysit growing caches. The **minimum** beta contract is a measured, per-volume reserve, protected unique-source/Host-journal categories, capped rebuildable HQ index/wiki/cache bytes, and transparent degraded state. **Advanced** personalized Rock membership, hot/warm/cold source rehydration and compaction are separate data-safety development requiring real crash/ENOSPC/restore tests.

- Never reclaim original exports, private memory, accepted DCB definitions, Host effect receipts, active World, device grants or unique journal segments by labeling them "index".
- No compaction requiring temporary duplication without scratch-space admission. Full mode is still capped by OS reserve.
- Source recovery proof must exist before eviction; a missing cold source becomes `unavailable`, not "false" or fabricated knowledge.
- Instrument one capability truth ledger using F schema and Labs observatory *read-only output*, without making the observatory a new catalog.
- Every UI claim is one of `DESIGNED`, `SOURCE_PRESENT`, `PREPARED`, `INTEGRATED`, `TEST_EXECUTED`, `RUNTIME_VERIFIED`, `DEVICE_ACCEPTED`, `BLOCKED`, with direct source/receipt link.

## 6. Beyond our thread: other foundational service families

**Browser/Studio** — preserve exact selected conversation thread, Chrome signed-in tab identity and send-once/unknown handling; no provider APIs or headless replacement. Expose source-selected *read-only* conversation/selection snapshots to an authorized semantic read port, never raw browser profile.

**Programmable automation and Chronos** — distinguish the existing HQ file watcher (workspace/dev) from end-user scheduling. Native pure time evaluation may be reused, but Host owns scheduled effect authorization and durable claims. No second clock daemon/scheduler/Queen executor. For user recurring missions, store versioned rule, bounded trigger, actor/scope, dry-run preview, policy and explicit owner review; timer firing proposes/admitted effect only through Host.

**Agent/mission/skills/extension** — C Studio agent/recipe registries and Labs plugin designs are source inputs, not a production plugin system. Plugin capability admission starts with a known **read-only or pure** extension (one source-pinned, signed/verified manifest) and grows only with quotas, actor policies, sandbox boundaries, revocation, rollback and package compatibility. Missions are deterministic state machines that reuse existing Host journal and native pure DAG; unknown external effects are never retried by opaque agents.

**App builder and generative UI** — E has an inert UI proposal compiler; TierFlow has typed World/visual revision support. First product proof: user requests a simple interface, receives source-linked preview/diff, approves one draft revision and sees the same view on Windows/PWA. Do not equate a preview with a full-stack app runtime or invent a second publisher.

**Voice** — Labs pure voice-to-task evaluator exists. Stream bounded partial/final events through native I/O; only accepted final transcript + typed proposal + Host owner review produces durable tasks. Real ASR, recorder, language and device acceptance separately required.

**Media** — B has a `media/` source subtree; source receipt/streaming and byte-range/audio/video device behavior require a separate source census and real tests. Do not use media as excuse to create another storage store, phone origin, approval writer or cloud fallback.

**UI and access** — PWA-first on real HTTPS browser with authenticated device key and Host origin. Accessibility, IME, tablet widths and keyboard parity are release gates; defer native APK shipping, signing, Android bridge and Play Store integration. Preserve existing E service worker invariant: cache static shell, never cache/replay auth-bearing API or effect requests.

## 7. Explicit source/code uncertainties before any implementation

- A engines are source-implemented; a **tested Rust actor-authenticated ingress and hot-swappable end-user capability pack is not observed**.
- B has 19 source actions in catalog v6; that is not whole-VIVIM command-space completeness, nor verified all-action reachability.
- D source imports include source walker, comments and wiki; combining HQ wiki and user personal source without Host grant is not accepted.
- C plugins/recipes and Labs extension research do not establish an installed sandbox ABI or API permission architecture.
- E UI and PWA source are imported, but browser PWA Host static allowlist and session replay/attestation require B-owned repair; physical phone tests unrun.
- F type mirror presence does not establish native/TS/JS schema equality or source authorization.
- The *real* G merge has not run. This branch's all post-G paths and wave scope are proposals conditional on G's exact combined HEAD and repair ledger.

**Result:** a single, source-grounded roadmap for making VIVIM genuinely extensible and deterministic, with native computation and trusted Host effects, while preserving the current source-owned architecture rather than opening a second parallel platform.
