# VIVIM Native User I/O — Parallel Event Fabric (No Central Semantic Router)

Date: 2026-11-12
Status: DESIGN only; no native cutover or acceptance is claimed.
Supersedes the incorrect implication in earlier architecture docs that a single Rust entrypoint means a sequential, single-brain router.

## Primary decision

The architecture is **fan-out and convergence**, not an application command router that chooses a single handler and then sequentially invokes the others.

The transport/ingress boundary authenticates, bounds, timestamps/sequences and publishes a typed immutable interaction event. That boundary is **not** a semantic router and is not allowed to classify intent or decide the one "right" processing destination.

Multiple independent, registered native Rust consumers observe the same input revision concurrently, react only to changes relevant to their own declared dependencies, and publish **typed derived evidence** into a revision-checked interaction graph. Downstream consumers can respond to derived signals as well as original events. Domain effects have one Host-controlled authority path.

One physical publication, multiple simultaneous computational interpretations, later evidence convergence.

Important distinction: **no central semantic router** does NOT mean no event ingress, subscriber table, source provenance, scheduling, fan-out delivery mechanism, sequencing, or bounded resource manager. These are deterministic operating mechanics and must not become another command/intelligence governance layer.

## One physical keystroke, the precise route

Example: user types "g" in the desktop composer.

0. Keyboard/OS/browser input subsystem receives the key; IME/composition and normal DOM input state remain local. For mobile, the web DOM receives a text composition event. No Rust call is needed to paint the glyph or move the caret.
1. A thin surface adapter normalizes a UTF-8 **draft revision** (prefer change ranges and source version over individual raw key bits). It attaches an opaque session ID, surface sequence, revision, compose-state flag and data/privacy class. It cannot assert a trusted principal or effect grant.
2. Authenticated native ingress verifies session ownership from the trusted local IPC or authenticated remote connection, quotas, framing and sequence, and emits an immutable \`DraftChanged\` event in an **ephemeral per-session stream**.
3. The event is **fanned out** by statically/declaratively subscribed interests to the bounded native computational fabric. Independent readers may run in parallel. No root coordinator reads the text and routes it by guessed intent.
4. Lightweight language tokenizer, command-prefix matcher, entity candidate finder, workspace/personal-context resolver, graph retrieval trigger, temporal pattern matcher and semantic visual signal builder may each react. Heavy engines can decline on "g" because their own trigger/dependency condition is not satisfied. The scheduler merely runs ready declared work.
5. Each consumer publishes an output containing \`source_revision\`, \`derivation_id\`, \`input_refs\`, \`candidate_kind\`, \`confidence_kind\` (grounded, tentative, unknown; numeric scores optional only if calibrated), \`evidence_refs\`, \`scope\`, \`ttl\`, \`source_owner\`. No consumer can turn its own candidate into a grant.
6. Outputs are **joined** through typed reducer / graph materialization rules keyed by session+revision+source. This is not a new independent graph store or a new all-powerful planner; reuse the existing shared graph, typed DAG, revision and invalidation machinery with a strictly ephemeral personal namespace for drafts.
7. Native projection consumers read a coherent revisioned view and emit optional \`ViewPatch\`, \`ContextHint\`, \`MissingSlot\`, \`AmbiguityFork\`, etc. TierFlow/DOM draw them; screen-reader alternatives are emitted as equivalent semantic data.
8. When the user types another character, a new input revision invalidates only dependency-connected work. Pending stale work is cancelled/coalesced where safe. A completed old computation cannot overwrite a newer draft's output.
9. On \`Submit\` the system creates a distinct **committed event** with its own idempotency identity and exact draft digest. It is *not* implied by a sequence of \`DraftChanged\` events. The native derived graph may compile a plan, but effects still require the canonical Host's live approvals and claims.
10. User corrections (including refusing an interpretation) are first-class inputs which can invalidate related derived nodes, rather than restart all processing.

## Parallel processing domains (same event, independent subscribers)

| Native computation | Subscriber trigger | Output | Effect-authoritative? |
|---|---|---|---|
| Linguistic/semantic parsing | Draft text range / committed message | Tokens, clauses, negation boundaries, candidate meanings | No |
| Command vocabulary | Relevant changed prefix / explicit selection | Canonical action candidate, required slots, risk metadata from Host catalog | No |
| Entity resolution | Proper noun, @, file, contact, source candidate | Grounded/ambiguous entity refs, provenance | No |
| Knowledge/context | Project focus change, new entity/source | Source-linked graph hints and DCB candidate inputs | No |
| Personal meaning | User-owned rules change / candidate phrases | Revision-pinned preference and learned lexical meaning | No |
| Temporal | Date/relative-time cues / mission changes | Time candidates; cannot silently schedule | No |
| Mission/DAG | Submitted outcome / updated prerequisite evidence | Bounded dependency state, next pure work | No |
| Privacy/capability preflight | New target / audience / effect candidate | Visibility and grant *status*, clarification needed | No |
| Visual semantics | Any relevant derivation | Compact glyphs, risk/uncertainty marks, view suggestions | No |
| Accessibility | View-semantic changes | Textual equivalent, focus announcement, reduced motion semantics | No |
| Observability | Redacted metadata changes | Trace/latency/backpressure, not private content | No |
| Host effect adapter | **Explicit admitted plan only** | Durable attempt/observation/receipt projected into Rust | Yes — Host is authority |

A consumer can subscribe to **input facts and derived facts** without a central orchestrator. The graph's dependency edges and scheduler readiness determine which computations are possible.

## End-to-end consistency rules

1. **Source event identity:** unique session+input revision/ID, exactly attributed to authenticated actor from transport, not from the event body's self-assertion.
2. **Snapshot discipline:** each derivation says *which* immutable input revision(s), catalog/policy versions, source data revisions and graph namespace it used. Mixed revisions cannot silently appear as a coherent present-tense result.
3. **Deterministic graph composition:** duplicate derivations deduplicated; conflicting candidates retained as explicit ambiguity/contradiction, not last-writer-wins.
4. **Clear trust separation:** Observed input, inferred proposal, verified data, policy availability and authorized effect are distinct node/edge kinds and UI states.
5. **No unauthorized persistence:** drafts are per-session ephemeral by default; no committed personal memory, provider message, system automation or effect from draft updates.
6. **Cancel/invalidate:** new edits invalidate dependent computation, not everything; old results cannot overwrite new ones; monotonic projection version.
7. **Resource control:** a bounded executor protects responsiveness. Low-cost lexical parse can run on every bounded draft update; expensive graph retrieval is trigger-driven, coalesced and cacheable. This is engineering scheduling, not semantic routing.
8. **Delivery semantics:** at-least-once publication with identity + idempotent handlers/reducers is implementable; do not promise literal exactly-once delivery in a crash/disconnect distributed path. Effects use Host claim fences and no automatic retry of unknown external attempts.
9. **Security:** subscribers receive scoped projections, not all raw private bodies. Desktop, phone and CLI authorization differences are enforced at ingress and at authoritative reads/effects.
10. **One effect owner:** approved effects are serialized/fenced by the current authoritative Host journal. Parallel candidate computation must never imply parallel unauthorized execution.
11. **Output convergence:** UI does not wait for *all* systems; render partial source-labelled projections and replace/reconcile when better evidence arrives.
12. **Unknown is visible:** stale, partial, blocked, speculative, observed, and independently verified states stay distinct.

## Rust conceptual boundaries (modules are NOT established)

- \`user_io::ingress\` — thin authenticated event transport adaptation and event validation, not intent classification.
- \`user_io::fabric\` — typed input/derived-event publications, static interest subscriptions, bounded fan-out and replay/snapshot support; not a new global bus/datastore.
- \`user_io::reactors\` — adapters registering existing Rust graph/semantic/command/context engines with declared trigger/dependency rules.
- \`user_io::graph\` — reuse the canonical shared Rust graph-node, typed DAG, invalidation and context engine; no separate knowledge graph writer.
- \`user_io::projection\` — coherent per-session, per-audience read-model and typed output stream. Not an effect owner.
- \`user_io::host_bridge\` — exact authority boundary to existing Host plans, approvals, journal, receipts and provider-send rules.
- \`user_io::telemetry\` — content-minimized trace of publications, derivations, cancellation and performance.

Do not add a generic router that calls \`interpret()\`, \`context()\`, \`commands()\` in a fixed order. The consumer subscription/DAG structure defines dependencies and therefore the effective routing.

## Proposed schemas (conceptual)

\`\`\`rust
struct DraftChanged {
    session: SessionRef,
    source_revision: u64,
    surface_sequence: u64,
    content_delta: TextDelta,
    in_composition: bool,
}

struct Derivation {
    source_event: EventRef,
    source_revision: u64,
    graph_revision: GraphRevision,
    engine_generation: EngineGeneration,
    inputs: Vec<EvidenceRef>,
    scope: DataScope,
    result: CandidateOrObservation,
}
\`\`\`

Trusted \`Actor\` comes from the authenticated connection state, never an unverified \`DraftChanged\` body.

## Current implementation / migration caution
- Existing Host control interpreter and dispatcher remain distinct JS systems in product main; HQ graph is Rust research with incomplete product linkage.
- Main Tauri currently starts a Node Host and exposes a pure native program validation command. This is *not* the target direct native parallel user I/O.
- Existing visual composer lab experiments may mimic these signals but must not be confused with real native fanout.
- Retain Host actions, effects and current phone session authority while rebuilding the Rust-facing ingress and event fabric incrementally.
- Prefer reusing existing Rust engine Scheduler / EngineRegistry / typed DAG, rather than new competing async runtime or graph DB.
- A *PWA* still needs a secure transport to the PC Rust process; it cannot directly invoke a native function. No extra semantic middleware should intervene.

## First proof demonstration
1. Desktop editor updates \`"g"\` then \`"gi"\` then \`"git"\`.
2. Rust receives monotonically numbered immutable draft revisions and fans them out to >=4 independent read-only reactors.
3. Command candidate reactor derives a possible Git operation; project graph reactor derives relevant source nodes; visual projection reactor emits appropriate tentative/grounded indicators; privacy reactor cannot grant effects.
4. Results finish in intentionally shuffled order; UI only accepts the correct revisioned projection and never shows old results as current.
5. On final \`Submit\`, committed request and idempotency key are distinct. Existing Host remains the only effect gate.
6. Run equivalence against old JS input semantics, Tauri on Windows and connected phone PWA. Record raw test logs and negative cases.
7. Show that the system has **no single semantic router** by inspecting the dependency/subscription graph and proving multiple parallel independent derivations.

## Architecture outcome

**One authenticated ingress, one shared typed event fabric, many concurrent native computations, one coherent versioned view, one effect owner.** This is *not* a central action router. Event distribution and graph dependency wiring *are* the routing engineering.
