# Full User-Facing I/O — Rust Interaction Contract & Migration

Status: DESIGN. Maps existing code observed through connected GitHub on 2026-11-12; not a live end-to-end verification.

## A. User-facing I/O is a bidirectional native protocol

**Input event** (authenticated transport supplies actor; actor is NOT accepted as a client assertion):
- schema/version; session ID; monotonically increasing surface sequence; idempotency key for committed actions; expected interaction and World revisions; input modality; interaction kind; content reference or bounded payload; explicit privacy class and audience; correlation/causation IDs; locale/accessibility hints (never used as grants).
- kinds: DraftChanged, Submit, VoicePartial, VoiceFinal, Select, EditStructured, Attach, Approve, Reject, Cancel, Correct, FocusChanged, OfflineDraftReconcile.
- voice partials, typing and focus produce only ephemeral preview operations; Submit/VoiceFinal/Select may create a durable request; Approve must bind exact Host-validated plan digest and current grants.

**Output projection** (native emits revision-stamped, ordered, audience-filtered messages):
- Ack / Reject; ContextHints; CommandCandidates; GroundedEntities; MissingSlots; IntentDelta; ViewPatch; Explain; PlanPreview; ApprovalRequired; StateTransition; ProgressObservation; ActionNeeded; ResponseChunk; VerifiedReceiptRef; Inconclusive; ReconcileSnapshot.
- each output must distinguish proposed, available, authorized, attempted, observed, and independently verified. Never show a source-backed finding as verified simply because a model says so.
- UI projections cannot carry user tokens, permanent Host grants or raw sensitive evidence outside the authenticated audience.

**Streams**:
- per-session revisioned snapshot + deltas; reconnect resumes from acknowledged revision or falls back to authorized snapshot; out-of-order, duplicated and stale events must be discarded/declined without changing authority.
- coalescing/backpressure for DraftChanged; never silently coalesce approval, submit, cancellation or receipts.
- generated content can stream; final user-facing completion must be based on observed outcome, not estimated success.
- local caret/IME/animation frames remain platform-local to guarantee responsiveness and accessibility.

## B. Source-backed boundary inventory (sampled, needs full audit)
| Current owner/path | Observed responsibility | Target |
| --- | --- | --- |
| Harness/src-tauri/src/main.rs | Tauri starts Node Host, loads loopback page; Rust exposes pure validate_native_program | Add native UserIO ingest/subscribe API; retain Host bootstrap while migrating |
| Harness/core/programmability/src/lib.rs | Pure typed program validation only; no scheduler, grants or effects | Converge reusable types/validation with shared Rust graph engine, do not create competing executor |
| Harness/apps/vivim-host/server.mjs | HTTP Host, control/auth, bootstrap, static UI and more | Retain transport and legacy endpoint compatibility; remove semantic-orchestration role after parity |
| Harness/apps/vivim-host/control/intent-planner.mjs | Deterministic text/typed planning using existing control language and catalog | Migrate interpretation/compiler into native core with differential tests, single owner catalog |
| Harness/apps/vivim-host/control/dispatcher.mjs | Source-owned approval, claim and effect receipt path | Preserve effects authority; Rust emits candidate and invokes exact authoritative Host admission |
| Harness/apps/vivim-host/direct-phone.mjs | Authenticated phone route via trusted proxy and session infrastructure | Route authorized phone interactions to native core, without duplicate planning |
| Harness/apps/mobile-relay/ux/mission-experience.mjs | Read-only mission status projection | Reuse as compatibility renderer until native revisioned projections parity |
| Harness/harness/src/context/dcb-dispatch-gate.mjs | Candidate-only DCB disclosure checks and handoff, actual Host grants/send elsewhere | Port pure selection/validation cautiously; preserve Host source authorization |
| TierFlow/src/visual-runtime/command/compile.ts | Inert visual command compilation | Visual dialect of native command candidates, not alternate action authority |
| HQ/research/rust-knowledge-graph/src/typed_dag.rs | Versioned pure graph transformation DAG, shared Scheduler and EngineRegistry | Reuse in packaged native runtime only after provenance, ABI, resource and crash tests |
| Labs/nlp-command-ui-lab/ | Live composer research, visual signals and contextual wiki experiment | Consume native projection protocol; never become the core |

Do not infer production readiness from source presence. Pin fresh main SHAs and inventory deeper live references and wiring before editing.

## C. Runtime topology

1. Desktop: OS input / DOM control -> thin Tauri event adapter -> Rust interaction API -> existing graph engine (pure work) -> candidate / plan; any effect goes via current canonical Host authority -> receipt -> Rust state projection -> TierFlow/DOM screen reader.
2. Phone PWA: browser UI -> authenticated HTTPS/WebSocket or equivalent secure channel -> Rust API in PC Host process -> same graph/plan path -> authorized resulting projection -> device adapter (display, haptics, OS notification only under device permission). No fake direct Rust call from a PWA.
3. CLI/agent/automation: trusted local IPC -> same Rust input envelope; background initiators are not user approvals. Their permitted scopes must be explicit and capability-bounded.
4. Provider browser send: Rust produces a scoped, source-referenced plan and context candidate; Host validates current consent + exact target; Studio runs authorized signed-in browser action; Host persists attempt/receipt; Rust observes result. AI output is an assertion until independently verified.
5. Native mobile later: same source contracts; optional local pure engine via one chosen FFI/JNI boundary, no independent PC World authority.

### Critical performance distinction
NO HOPS means one semantic owner, not forcing every keystroke through an interprocess/network round trip. Text echo, IME handling, pointer motion and frame rendering stay local. The Rust semantic preview can debounce, cancel and reuse incremental graph state; stale preview results must never replace newer results.

## D. Rust module placement

Extend/converge the existing product-native Rust foundation rather than create another daemon:
- user_io::contract: input envelopes, source authentication boundary, state/projection schema
- user_io::session: revision CAS, lifecycle, ephemeral drafts, disconnect/resume
- user_io::interpret: deterministic language/control compilation and ambiguity
- user_io::context: graph-backed retrieval and DCB audience proposals
- user_io::coordinate: bounded plan DAG and mission status, never a separate scheduler
- user_io::project: output projections and a11y semantic equivalents
- user_io::bridge: existing Host effect-owner adapter + receipts, no new effect owner

Names are **proposed module boundaries**, not claimed existing code. Reuse existing hqkg/shared engine and current native programmability types after dependency and duplicate audits. The product native interface should be owned where product authority is integrated, not in HQ's development index.

## E. Single authority principles
- Authenticity: Host/Tauri transport establishes authenticated actor and device. A caller-supplied source ID, SHA or proof is not itself trusted.
- Interpret != authorize; planning != effects; digest != origin authenticity; generated prose != independent verification.
- Host's existing action catalog stays canonical for effect IDs and risk levels until an owner-approved single-source migration.
- The existing journal remains effect authority. No direct extra effects from Rust's pure graph scheduler.
- Policy/permission revocation wins over a pinned planning snapshot.
- No unapproved personal evidence into HQ development graph.
- No provider APIs; signed-in browser apps only.
- No implicit replay of a provider send with uncertain outcome.
- Offline phone actions are drafts until resubmitted to and validated by the PC Host.

## F. Integration phases, not a big-bang rewrite

### Phase 0: Truth map
Record exact SHA + file:line + live caller -> interpreter -> effect writer -> projection for desktop, phone, Studio, TierFlow, CLI, accessibility, voice and scheduled interactions. Classify canonical/adapter/projection/duplicate_candidate/experimental/historical. Build differential fixtures and an invocation graph.

### Phase 1: Native read-only first hop
Add Tauri UserIO ingest + streaming event projection without granting an effect. Prove status, contextual hints, ambiguity, explanation, and pure plan previews on the same Rust state revision. Retain old JS behavior as monitored compatibility fallback; flag divergences.

### Phase 2: Semantic convergence
Port deterministic command semantics, context scope and graph computation by reusing existing Rust graph-node modules. Eliminate JS intermediate interpreters only after corpus parity, negative tests and benchmarks. Wire composer lab to the SAME native projection contract.

### Phase 3: Effect-owner bridge
Native plan -> canonical Host approval/admission/journal/execute -> native projection of Host receipt. Test submit and approve idempotency, exact plan digest, expiry, revocation, crash-after-claim and post-send uncertainty. Effects remain Host-owned until explicit verified Rust migration.

### Phase 4: Phone/CLI/accessibility/voice coverage
Authenticated PWA link; reconnection and offline drafts; correct screen-reader equivalents; voice partial/final semantics; local haptics and notifications; CLI parity. Avoid introducing APK as a blocker for PWA-first release.

### Phase 5: Production cutover and consolidation
Single native semantic owner; old controllers become transport/presentation adapters or are retired. Maintain a rollback switch per surface. Release only with evidence ledger covering packaged Windows Host, paired phone, actual signed-in provider browser, and crash recovery.

## G. Scope decision for visual composer innovations
The 100-idea visual research catalog in the existing Labs visual branch remains independently testable. Its grounded entity chips, ambiguity forks, semantic diffs, risk markers, attention budget and accessibility outputs become VIEW logic driven by native projections; no direct visual changes grant effects. Prioritise experiments that exercise native state revision, provenance and uncertainty signals.

## H. Explicit refusal states
Unknown actor; expired grants; stale revision; conflicting idempotency keys; unattested source; missing action in canonical catalog; incompatible audience; absent provider thread; unsupported device; closed session; offline authority absent; unknown effect outcome; replay of claimed action; unverifiable evidence. No optimistic rendering of effect success in these states.
