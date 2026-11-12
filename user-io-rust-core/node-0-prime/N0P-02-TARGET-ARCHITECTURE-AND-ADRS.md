# N0P-02 — Target Architecture, Language Decisions and Dataflow ADRs

Date: 2026-11-12
Status: Proposed target system, **not** claim of current wiring.

## Strategic reset: the user is not operating a command router

VIVIM's primary user-facing object is an evolving **human expression**, not a command. Multiple meanings may coexist:
- Question to answer
- Source/entity references
- Preference or personal memory candidate
- DCB/corpus hints
- Possible actions and dependencies
- Conditions, negation, literal quotations and temporal scope
- Predicted next intentions
- User corrections/consent/permission constraints

A central router selecting exactly one interpretation destroys information. However, a mechanically deterministic system still needs: authenticated ingress; bounded subscriber admission; typed dependencies; consistent snapshots; source-scoped capability checks; a projection reducer; explicit commit/admit/receipt barriers. This is **not** another semantic router.

## The five data planes (logical, not five services or stores)

### Plane 0 — Human Edit Plane (UI owned)
- Live text, caret, selection, composition/IME, paste, undo/redo, keyboard focus, touch, accessibility and voice partials.
- The editor controls the *original expression* and its lossless text history. Semantic engines cannot rewrite original human words just by producing a prediction.
- Echo is local and synchronous. The native engine cannot block typing.
- For a mobile PWA the phone may hold **uncommitted offline drafts** with user consent; this is not Host execution authority.
- Input normalization produces immutable snapshots/patches with stable editor revision and source/character range mapping, not one opaque "keypress" per operation.

### Plane 1 — Trusted Ingress & Ephemeral Input Stream (Rust native)
- Windows: bundled Tauri/native bridge receives bounded \`DraftChanged\`, \`SelectionChanged\`, \`VoicePartial\`, \`VoiceFinal\`, \`AcceptCandidate\`, \`Submit\`, \`Correct\`, \`Reject\`, \`Cancel\`.
- PWA: same logical protocol over authenticated encrypted transport to PC Rust native endpoint; the phone cannot call a native PC function directly.
- CLI/scheduled: authenticated local IPC with separate principal/automation authority. Scheduled work cannot impersonate an interactive user approval.
- Ingress authenticates principal, device, origin and session from trusted connection; validates sizes, source revision, sequence and privacy audience; **does not decide semantic meaning**.
- Drafts are ephemeral; a committed input has a separate stable request identity and durability conditions.

### Plane 2 — Pure Concurrent Derivation (shared Rust graph engine)
- Independent reactors subscribe to typed input and derived signals: lexical, command grammar, named entity, context, temporal, personal semantics, DCB, knowledge, prediction, source capability, contradiction, explanation, visual semantics, observability.
- They declare **required source classes, input/derived signal types, budget, invalidation dependencies, output schema and deadline**. The subscriber manifest is an engineering contract, not a semantic gatekeeper.
- An authenticated source adapter provides only the permitted **reference snapshot** each reactor requires; one user's private draft is never fanned out as raw bytes to every engine or to development HQ.
- Pure work can precompute likely future results (source reads, source index/cache, typed DAG) with opt-in and resource bounds.
- Derived outputs are tagged as hypotheses/observations, pinned to exact input/source/engine/policy revisions. Observed identity ≠ permission.
- Work is incremental and graph dependency-driven. A new source change invalidates the consumers whose requested evidence depends on that source, not all sessions.

### Plane 3 — Intent Commitment & Effect Authority (Host owned)
- On \`Submit\`, the chosen typed interpretation becomes \`CommittedUserIntent\` with exact original text reference, selected atoms, explicit ambiguities, expected source revisions, idempotency ID and actor/session.
- If it is not well grounded, ask one accurate clarification rather than inventing target IDs. A normal free-form question does not have to map to a registered effect command.
- Deterministic compilation of a candidate plan can occur in Rust; actual effect admission, current grants, exclusive claim, browser-send fence, receipts and recovery remain with canonical Host controls until formally migrated.
- Read-only computation with no effect may produce a verified result if a source owner supplies reliable proof; do not count a model assertion as independent verification.
- Provider browser adapter is a fault-isolated, explicit capability. No provider APIs.
- Existing durable journal reused for effect attempt and receipt. If committed intent durability is not supported, extend one authoritative Host transaction/record family, do not introduce a second uncoordinated event database.

### Plane 4 — Revisioned Output Projection (Rust semantic, UI presentation)
- All clients observe a single **versioned, audience-filtered, accountable interpretation trace**: multiple candidate nodes per phrase, exact source anchors, producer status, confidence kind, provenance, negative/conditional relations, permitted DCB/memory facets, read/effect status, lossless text anchors.
- Publish partial views that explicitly say \`partial\`, \`stale\`, \`not_run\`, \`denied\`, \`timed_out\`, \`unavailable\` where applicable.
- Quiet / Assisted / VIEW ALL are presentation masks over the same authorized trace. "VIEW ALL" means all eligible participating producers and emitted/suppressed explanations, not an impossible guarantee of every conceivable meaning.
- TierFlow/editor owns glyphs, labels, text geometry, focus choreography, inline selectors, motions and screen-reader delivery.
- Verified outcomes and user corrections feed back as new typed observations, potentially changing future prediction ranking only with scoped consent and provenance.

## Concrete end-to-end flow

\`\`\`
Human types or speaks
    │
    ▼
OS / browser / IME / accessible input → LOCAL ECHO, UNDO, CARET (never blocked)
    │  immutable text/voice snapshot or delta; local revision N
    ▼
TRUSTED NATIVE INGESS (Rust)
 authenticates actor/device/surface; session seq; privacy/purpose; bounded bytes
    │  publishes event ID + source revision; NO semantic target choice
    ├───────> language/quotation/negation reactor
    ├───────> command grammar / action frame reactor
    ├───────> entity + workspace identity reactor
    ├───────> personal preference / DCB candidate reactor, if authorized
    ├───────> knowledge / graph neighbors, budgeted read port
    ├───────> temporal + prediction reactors
    └───────> producer-coverage and health observation
                │ outputs can trigger further *pure* reactors by typed deps
                ▼
     REVISIONED EVIDENCE/DEPENDENCY JOIN
     preserves competing meanings + source/capability/time version vector
                │
                ├──────> partial typed projection → UI / accessibility / VIEW ALL
                │
                └──────> explicit Submit / corrected selection
                              │
                              ▼
                     COMMITTED USER INTENT
                              │
                         current rechecks
                              │
                              ▼
                         HOST ADMISSION
                   exact plan, current grants,
                   effect claim/receipt journal
                              │
                 provider/browser/World effects only
                              │
                              ▼
                    observed outcome/evidence
                              │
                              ▼
                      VERIFIED USER RESULT
                    (or failed / uncertain / partial)
\`\`\`

## Concurrency & consistency: replacing the wrong R001 hot path

### Current source slice
- \`UserIoCore::apply\` updates session then calls \`self.fabric.react(...)\` synchronously.
- \`Fabric::react\` spawns scoped threads per event and joins all.
- Tauri \`user_io_event\` locks \`Mutex<UserIoCore>\` around the entire \`apply_json\` call.
- Consequence: fan-out reactors run simultaneously *inside one blocked state transition*, while all other state mutations wait. This is a prototype, not the real-time design.

### Target two-phase native pattern
1. **Capture:** short write lock/CAS on a session actor stores latest immutable draft revision and a snapshot of trusted source pins; return an acknowledgment or semantic cache immediately.
2. **Schedule:** publish typed work to **one bounded native executor**, use reusable work slots and priority classes. Do not spawn 4+ OS threads on each keystroke. Regulated subscribers receive minimum necessary inputs only. Pure computations may finish in any order.
3. **Derive:** compute outside the session state lock. Emit \`Derivation {event_ref, input_rev, source_vector, engine_generation, producer_id, data_class, result}\`.
4. **Converge:** short CAS to append a derived result to a coherent evidence view *only if all required revision and access predicates still hold*. Conflicting candidate meanings remain separate, not overwritten.
5. **Project:** stream source-labelled partial updates to UI, with a fast initial result and later stable results. Local UI ignores stale patch IDs; never erases human input to "correct" it.
6. **Commit:** Submit has a separate durable identity; re-ground current source versions if needed and produce a clear revision-stale/clarify result. It never uses speculative permission as an authority token.
7. **Recover:** invalidation, timeout or crash returns a typed status and restores authorized source snapshots as possible; no unobserved external effect replay.

Illustrative Rust-like structure (NOT source code in repo):
\`\`\`rust
fn ingest(trusted: TrustedSession, event: UserInput) -> Result<RevisionAck> {
    // short, authenticated, revision-CAS mutation
    let snapshot = sessions.capture(trusted, event)?;
    executor.publish_bounded(snapshot.minimal_reactor_views());
    Ok(snapshot.ack())
}

async fn derive(task: PureReactorJob) {
    let evidence = task.run_with_budget().await;
    if task.dependency_pins_current() {
        evidence_projection.try_commit(evidence); // short CAS
    } else {
        telemetry.mark_stale(task.id);
    }
}
\`\`\`
Separate CPU executor (bounded threads) from async authorized I/O, and from serial Host effect admission. Applying Tokio alone without limits does NOT solve backpressure.

### Data versioning is multidimensional
The target envelope must include, as applicable:
- inputRevision / editorRevision / sessionRevision
- actorSessionVersion / deviceGrantRevision
- hostWorldRevision / sourceOwnerRevision
- commandCatalogRevision / personalRuleRevision
- project/entity evidence digest + content revision
- graphEpoch / indexWatermark / engineGeneration
- authorizationPolicyRevision / outputAudience
- correlation IDs and causation/reference set

Not every event needs every field. Every derivation must pin the subset it depends upon, and a projection must only join compatible subsets. A candidate grounded in an old project revision cannot silently appear "current" beside a newly revoked permission. A result's SHA is integrity of bytes, not proof of source authenticity.

### Causality, delivery and retries
- In-process publish+derive may be at least once; idempotent derived event identity deduplicates duplicates.
- Cross-device input messages have monotonic sequence, ack, resumable snapshots and explicit rebasing. Do not claim exactly-once network delivery.
- Committed user intent has stable idempotency; effect attempts have durable claim fences; recovery investigates uncertain external actions rather than retrying.
- A "cancel" of a speculative query can stop compute; a "cancel" of an already attempted external browser action cannot pretend to undo it.

## Prediction: an adaptive cost planner, not a semantic authority

The predictiveness is an **independent reactor** with private, consented acceptance/correction evidence. It may adjust:
- Which eligible template/slot candidates are shown first
- Which read-only source computations are prewarmed
- How much computation is spent on likely contexts
- Which high-level view hints are prepared for TierFlow

It may NOT:
- Change command meaning, privacy scope, effect risk, Host grants or policy
- Invent canonical project/contact IDs
- Suppress all other reactors or classify unsent text as authorized operation
- Automatically send to an AI/provider, write memory or start background effects
- Make an unavailable source appear "current" because the user often requests it

A cold-start user still gets correct, transparent source-derived suggestions, not fake personalization.

## User text and Unicode contract

The UX depends on the exact identity of every character. Define:
- Preserve unmodified source text; never roundtrip via lossy normalized token string.
- UTF-8 bytes as Rust storage representation; explicit mapping to JavaScript UTF-16 offsets and Unicode grapheme boundaries.
- \`DraftDelta\` includes \`range\`, \`beforeHash\`, \`afterHash\`, \`compositionId\`, editor \`revision\` and \`caret/selection\` position metadata if authorized.
- Render-source tokens and graph annotations refer to **stable source-span IDs** or revision-bound ranges. A selected chip is just a reversible presentation view of original source.
- Parser can case-fold/normalize for search, but semantic equivalence uses original text plus locale-aware rules.
- Handle IME, dead keys, emoji ZWJ, bidi, accents, code spans, quoted literals, paste, undo/redo and screen-reader focus. Tests compare both plain text and rendered selection ranges.
- Browser \`beforeinput\` is not guaranteed for every edit; do not design on cancelable \`beforeinput\` only. Reconcile \`input\` and composition events as documented by MDN/W3C.

## Artifact and process choices: would I change the programming languages?

### ADR-001 — Rust owns product interaction state & pure computation: YES
**Keep/change:** use existing Rust product core; make native graph computation a canonical crate consumed by Windows Host/HQ, not a JS-called function at the end of an otherwise JS-semantic pipeline.
**Why:** typed state machines, compile-time separation of candidate/effect, safe concurrency, deterministic revision logic, resource budgets, portable native runtime. This is where correctness is most valuable.
**Caution:** Rust doesn't automatically solve input semantics, memory safety of JS frontend, wrong authorization, graph consistency or UI accessibility.

### ADR-002 — TypeScript owns edit/render/microinteraction: YES
**Keep/change:** retain webview/TierFlow; make frontend control logic type-safe TS and generated contract bindings rather than scattering untyped \`.mjs\` authority assumptions in UI.
**Why:** platform DOM, rich text, IME, accessibility, fluid rendering. A Rust renderer would be an expensive regression without solving first-hop semantics.
**Caution:** UI's local proposals are not a second authoritative command interpreter.

### ADR-003 — Node as semantic Host hot path: NO
**Change:** eliminate mandatory \`Tauri → localhost page → JS Host planner → other JS modules → Rust\` for native semantic processing. Prefer \`UI → authenticated Tauri native Rust\` directly. Migrate provider-agnostic pure semantics into native in parity-verified slices. Keep Node/Studio signed-in-browser execution as domain adapter.
**Reason:** fewer inconsistent truth owners, less state duplication and simpler crash/latency profile. This is NOT an instruction to rewrite all Node tools before product release.

### ADR-003B — Host effect ownership vs implementation language: MIGRATE TO RUST EVENTUALLY
**Keep:** Host as the *domain owner* of effects and exact approvals. Its existing JS control journal and claim fences remain authoritative until differential and Windows crash tests establish a durable migration.
**Change:** with enough engineering budget, migrate the Host admission state machine, approval checks and journal durability into the *same native Rust Host process* as the interaction state, using one compatible source-owned transaction/receipt format. Node/Studio remains a **narrow browser adapter** and may never become a parallel journal writer. No dual-write cutover: one writer at a time, read-only shadow comparator, strict owner-controlled migration and rollback. This reduces the final cross-language trust boundary for irreversible effects without changing domain ownership.
**Reason:** the ideal end state has one native operational authority for input-to-commit-to-claim, while separately isolating unreliable signed-in web app automation. Rewriting JS blindly without proving durability would be worse than keeping it.

### ADR-004 — Tauri desktop shell: KEEP, HARDEN
**Change:** upgrade coherent Tauri pinned version family to currently patched supported 2.x; remove wildcard loopback remote capabilities in production; decide between bundled UI assets + native IPC vs exact loopback origin with explicit ACL. Audit global Tauri exposure, capabilities, request origin, CSP, packaging and native commands.
**Reason:** Tauri remains suitable Rust native shell. The present pinned version is a serious dependency-health problem, not evidence that Electron or a custom shell would help.

### ADR-005 — Shared Rust graph engine location: ONE CANONICAL CRATE
**Change:** package existing HQ \`research/rust-knowledge-graph/\` algorithms into one reusable, supported crate/artifact and bind product core to a pinned version. I would ultimately put **shipped product primitives in the Harness native workspace** and have HQ consume the same versioned crate for development-intelligence use. Keep HQ research original as migration source/evidence until parity, not another shipping copy.
**Reason:** shipping a personal-data runtime from an HQ research folder or copying diverging crates is fragile.
**Conflict to resolve explicitly:** existing directives call HQ research "Rust source of truth"; this ADR proposes revising **release ownership**, not silently overwriting it. Owner-controlled commit and dependency-graph review required.

### ADR-006 — Generic global event bus or distributed actor framework: NO
**Change:** typed in-process subscription graph using existing engine/scheduler and bounded channels where appropriate, not Kafka, NATS, Redis, a global service bus or an additional custom event DB.
**Reason:** one-user local-first PC product, high event volume, very low latency, strict data ownership. Build only what measured load needs.
**Caution:** delivery mechanics, partial order and backpressure remain essential; "no central router" does not mean "no subscription manifests."

### ADR-007 — Data serialization: GENERATED CONTRACT, SIMPLE WIRE
**Change:** canonical Rust domain types, explicit versioned schema and generated TS types/validators; JSON as interop baseline and redacted inspection representation. Use faster binary encoding only if measured payloads justify it.
**Reason:** current inconsistent hand-authored JS/TS/Rust shapes can silently drift. Arbitrary \`serde_json::Value\` is acceptable only as a bounded external deserialization boundary, not the internal semantics.

### ADR-008 — Storage: NO NEW MASTER GRAPH
**Keep:** source-owned persistent Host World/effect journal, HQ development index, user-private personal graph, shared Rust compute graph algorithms. Add materialized read projections as needed with explicit lifecycle.
**Change:** adopt a formal source owner + consistency model for each entity/claim/revision, rather than treating all graph nodes as equivalently trusted.
**Reason:** one queryable logical living graph can have multiple authoritative domains and cannot turn every speculative candidate into a durable fact.

### ADR-009 — PWA-first mobile, one PC Host effect authority: YES
**Keep/change:** preserve paired phone PWA, direct connection and offline drafts; move authoritative semantic processing into same native Rust core that desktop uses. Optional WASM for low-latency **inert** local preview only, with exact revision reconciliation.
**Reason:** minimize duplicate Android app surface and native bridge complexity; PWA cannot avoid physical network hop to PC.

### ADR-010 — Models as orchestration source: NO
**Keep:** signed-in provider browser apps, no provider APIs, DCB memory-off approach, evidence-driven suggestions.
**Change:** provider model output is just an untrusted proposal/answer node. Rust graph and Host policy decide what is inspectable and executable. Native product must work when no provider is connected, offline, during an outage, or when model latency is seconds.
**Reason:** the first 50 ms of interaction should not require a model token.

## Why "unlimited coding budget" does not imply "rewrite everything"

The expensive risks are not lines of JS vs Rust. They are **competing authorities, undefined state transitions, unbounded concurrency, insufficient proof, and migration without a rollback**. Use the budget to create one coherent Rust session/evidence/control contract; migrate only those existing modules whose semantics can be proven, retire redundant paths, and preserve excellent domain-specific code (Host journaling, Studio browser leases, HQ indexing, TierFlow presentation).

## Prime UX SLOs (targets, NOT measured)

1. Local editing: never waits for native response; caret/IME remains frame-responsive under engine failure.
2. Hot-prefix meaning: first candidate should have a tight local CPU budget, measured at p50/p95/p99 on baseline Windows hardware.
3. Grounding: slower source-backed hints arrive asynchronously without erasing text, replacing current candidate with stale results, or lying about freshness.
4. Selection: a visible candidate can be chosen/rejected/undone without jumping caret; all selections bind exact source and revision IDs.
5. Safety: zero auto-provider sends, permanent memory writes, approvals or Host effects from draft events or prediction.
6. Availability: independent local dialogue/command understanding remains useful when external providers, HQ index, Node browser adapter or mobile network are down.
7. Truth: every user-visible result has \`suggested\`, \`source-grounded\`, \`authorized\`, \`observed\`, \`verified\` or \`unknown\` provenance-equivalent semantics.

We should set numerical budgets after measuring packaged Windows and real paired phone. Don't claim <30 ms performance from synthetic in-process probes.

## Non-goals

This RFC does not authorize new user memory ingestion, new service, new UI features or change of visual grammar. It is a rewrite of **contracts, boundaries, resource discipline and operational truth** of the existing system, not product scope expansion.
