# R001-01 — PRD: Parallel Native Event Fabric & Revision-Convergent State

Status: Architectural contract, with limited pure native Rust slice committed. No product E2E.

## Problem
Legacy user paths are scattered: Host JS command planner + dispatcher; mobile direct-phone routing and NLCL; Lab visual compiler; HQ Rust graph research and operational JS index; existing product-native pure program validator. We need a **single computational substrate** without creating a "first AI", sequential parser router or second authority.

## Invariants
1. A human draft is first echoed locally in the UI/IME. When the semantic input crosses into Rust it is one immutable versioned input event, published once.
2. A set of independently subscribed deterministic reactors simultaneously sees the same safe bounded event. No one has sole claim to interpret it; signals are classified and can coexist.
3. Derived facts publish source event identity, revision, source owner, catalog/graph pins and evidence provenance. Results are candidates until authenticated owners verify them.
4. A typed converge/materialization reducer forms one coherent, revisioned view. Never replace newer evidence with stale older work; conflicts remain explicit.
5. Domain effects are out of this pure path. Host owns effect authorization and send-once journals.
6. Scope and privacy are first-class: source access is authorized before handing a read snapshot to a reactor, not inferred from IDs/digests.
7. No new global bus, journal or datastore: use existing Rust graph-node engine, typed DAG, invalidation and Scheduler when packaging verification makes that possible.

## Input envelope — target versioned contract
\`\`\`
UserIoEvent {
  schema: "vivim.native-user-io/1";
  authenticatedActor: // injected from trusted ingress, NEVER client supplied
  sessionId; surfaceId; sequence; inputRevision; interactionRevision;
  eventId; kind; payload;
  composerMode; expectedRevision?;
  sourceRefs[]; dataClass; audience;
  parentEventId?; correlationId; causationRefs[];
}
\`\`\`
R001 implementation exposes only \`draft_changed\`, \`tab\`, \`submit\`. The actual \`apply_json\` wire schema is simpler and has no actor; in R001 it serves only non-authoritative, read-only native hints. Treat it as an experiment, not a future auth contract.

## Reactor registry and native scheduling
Each owner-registered reactor publishes:
- Reactor ID, supported input/derived signal kinds and source capabilities needed.
- Pure vs externally observable workload classification.
- Source/graph revision dependency set and invalidation conditions.
- Max CPU/IO budget, priority lane, cancellation behavior and backpressure policy.
- Stable candidate schema and evidence labels; no authorizable effect method.

The subscriptions are the routing: \`DraftChanged\` may start lexical, command, entity, predictive preference and visual-semantic computations; a grounded project event may trigger graph-neighbor reads; a predicted focus may trigger a source-safe speculative read.

Round 1 code uses **four scoped Rust threads per accepted event** in the same existing product crate to prove independent reactive semantics. This is a design spike; migrate to the existing shared Rust Scheduler/bounded work queue before shipping any per-keystroke production path. Do not create another always-on event runtime. Native scheduler never acts as an intent router.

## Read/write separation
- Local visual echo: compositor does it synchronously; no Rust round-trip to paint characters.
- Rust ephemeral draft: transient per-session text/semantic state with bounded memory, no permanent memory record by default.
- Rust pure derived evidence: parse, sources, indexes, predictions, context candidacy.
- Host writes/effects: only after separately submitted, admitted, current plans and authenticated grant checks.
- HQ development indexes: read-only code/project source intelligence, never receive private user drafts by default.

## Event ordering & idempotency
- Session-local \`sequence\` is monotone; stale or duplicate events refuse.
- Native \`interaction_revision\` increments on accepted draft, Tab and Submit; stale Tab fails CAS.
- \`input_revision\` increments on draft input; derives refer to that immutable text.
- Each source change invalidates linked suggestions; no old project/alias after revocation.
- No promise of literal exactly-once network delivery; use event IDs, idempotent reducers and Host effect fencing.
- Compound/multi-device interactions need an owner-validated merge/rebase protocol in later rounds; no naive last-writer-wins.
- Preview output is always marked \`execution_performed=false\`; no action, even at full intent specification.

## Reactive example
\`DraftChanged("full")\` -> in parallel:
- \`language\` checks conservatively for cancel/negation cues
- \`command\` checks currently installed trusted template triggers
- \`entity\` emits only actively grounded project refs
- \`prediction\` scores known focus options from explicitly accepted counts

A pure join generates suggested template/project/focus. \`Tab\` selects only visible candidates with current revision; the second Tab selects focus. Actual \`submit\` currently produces only a nonexecuting marker.

## Failure/negative requirements
- Blank, overlarge, null-containing text; invalid session IDs; unknown wire properties; client-side "grant", "actor" or "approved" fields -> refusal.
- Non-monotonic sequence and wrong expected revision -> refusal without changing state.
- IME composition active -> suppress command/predictor completion, no Tab commit.
- Negation/stop clause -> no actionable completion inference.
- Missing project or unregistered template -> no fabricated grounding and no magical two-Tab guarantee.
- Source update/revocation -> invalidate previous offers; no replay of old suggestions.
- Reactor crash -> observable refusal and no submitted effect, with fault isolation in future bounded scheduler.
- Slow graph result -> partial projection with stale/unknown markers; never block local typing.

## Verification requested
Pure differential corpus from Host NLCL and existing R005 semantic-lens, not wholesale replacement until parity. Add source/graph revision race tests, concurrent surface tests, cancellation and backpressure stress, measured p50/p95/p99. Current Rust source tests cover only the basic read-only portion. Never equate a green syntactic test with real live graph wiring.
