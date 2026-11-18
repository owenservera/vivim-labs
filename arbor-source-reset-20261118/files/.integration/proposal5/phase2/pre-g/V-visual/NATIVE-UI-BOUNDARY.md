# NATIVE-UI-BOUNDARY — exact existing Rust API and post-G extension proposal

**Status: pre-G contract only.** Native source inspected at `owenservera/vivim-web-harness:work/native-user-io-r001-20261112` commit `fa6dea9576c83496d95562b98b7a815fda4aec88`, blob `d8a53bb2a9216721323790ca96b9517af6528bec`. Host and TierFlow live on separate frozen ARBOR A/B/E branches. See [SOURCE-PINS.json](SOURCE-PINS.json).

## 1. Existing, evidenced Rust R001 — NOT the proposed rich wire protocol

`core/programmability/src/user_io.rs` exports `InputEvent::Draft {session_id,sequence,text,composing}`, `Tab {session_id,sequence,expected_revision}`, `Submit {session_id,sequence,expected_revision}`; `UserIoCore::with_trusted_sources`, `install_trusted_sources`, `apply`, `apply_json`. The `Projection` carries `schema`, `session_id`, `interaction_revision`, `input_revision`, `catalog_revision`, `phase`, template/project/focus suggestions/selections, `ready_to_submit`, typed `Derived` reactor evidence, `execution_performed:false`, and `authorization:not_assessed`.

The **implemented** Rust interface is one JSON request → one projection. `apply` guards per-session sequence, source catalog revision on Tab, composition, stale revisions; `Submit` only sets an **inert marker**. Four observed `Evidence` variants: `Template`, `Project`, `Focus`, `Negation`. This source is **not** a working multi-producer live stream or secure personal data binding. The existing Tauri command has an empty trusted source set until genuine authenticated source wiring.

## 2. Proposed Rust → UI read-side port (not yet exported)

```ts
// Proposed adapter; intentionally NOT a Rust or TS runtime implementation
interface ReadSemanticProjection {
  subscribe(sessionRef: string, fromProjectionRevision: number | null):
    AsyncIterable<SemanticSnapshotOrPatch>; // bounded, audience-filtered native read
  snapshot(sessionRef: string): Promise<SemanticSnapshot>; // resnapshot after a gap
}
// Native-owned envelope for every independent source producer
type SemanticSnapshotOrPatch = {
  sessionId: string; inputRevision: number;
  interactionRevision: number; // Rust owner's session CAS
  revisions: { source: string; catalog: string; policy: string; graph: string };
  producerId: string; producerRevision: number;
  coverage: {status:'partial'|'complete'|'denied'|'revoked'|'stale'|'unknown';facets:string[]};
  candidates: readonly Candidate[]; // roles, span UTF16 + UTF8, quote, negation, entities
  permission: 'not_assessed'; effectAuthorization: false;
};
```

Transport must authenticate the session/actor out-of-band; the payload must not be permitted to assert principal, grant, approval or Host policy. Native read reducers deduplicate by producer revision **and** reject changes in source/catalog/policy/graph revisions. For reconnect, snapshots and patches need a trusted monotonic watermark, source-digest evidence, data-class and audience filtering; the V-local prototype demonstrates only the pure revision gate, not that transport.

Read-side fan-out: immutable draft revision → multiple independently subscribed native consumers → typed evidence with producer ID + coverage → shared Rust graph convergence → read-only projection. No single classifier, no new graph store and no mutable renderer-defined source of truth.

## 3. UI → Rust correction side (future, distinct from effect path)

**R001 currently supports `draft_changed`, `tab`, `submit` via `apply_json`**, not arbitrary corrective span edits or Undo. Proposed additions, explicitly owner-reviewed:

```ts
interface CorrectSemanticDraft {
  selectCandidate(x: {sessionId:string;expectedInputRevision:number;expectedInteractionRevision:number;
    expectedCatalogRevision:string;candidateKey:string;acceptedSpan:{startUtf16:number;endUtf16:number};
    replacementText:string;idempotencyKey:string}): Promise<{accepted:boolean;newInputRevision?:number;reason?:string}>;
  undoSelection(x: {sessionId:string;expectedInputRevision:number;correctionId:string;idempotencyKey:string}):
    Promise<{accepted:boolean;newInputRevision?:number;reason?:string}>;
}
```

Only explicit user Tab/click/voice *acceptance* may originate a correction. `DraftChanged` is ephemeral; `Select`/`Correct` are CAS-fenced; no correction may call `Submit` implicitly. Use native session history for durable correction receipts only if policy permits, not the display cache. A visible suggestion can be accepted only against the current surface input + interaction + catalog versions. Source-range editing must be reversible and preserve exactly the original expression as Undo text; quote and negation scopes cannot be dropped while synthesizing an executable positive command.

Local browser/phone retains caret, selection, IME composition, input value and immediate glyph painting. Native projections are asynchronous **decorations** keyed by input and source revisions. While IME composing, keep any last stable source-labelled visual but do not auto-select or rewrite text.

## 4. TierFlow ownership (frozen E source) and authority boundaries

- E consumes `semanticGraph(snapshot)` from `PROTOTYPE/projection.mjs` as an **inert, source-linked visual graph**, maps source IDs to TierFlow `command/ir.ts`, `render/wave3-semantics.ts` and `render/dom.ts` data. It may style Quiet/Assisted/VIEW ALL; modes cannot alter candidate semantics, source revision or risk.
- `packages/tierflow/src/visual-runtime/revisions/port.ts` is an inert UI fixture and accepts injected principal only from trusted broker; do not feed consumer text as VerifiedUiPrincipal or call UI publish/rollback from the projection.
- Frozen B `apps/vivim-host/control/intent-planner.mjs:interpretText/planText` owns authoritative **planning/interpretation** against `action-catalog.v1.json`. `dispatcher.mjs:plan/execute/status` is the sole Host effect/approval/receipt route. No UI preview, model, parser hint or Rust suggestion grants an effect.
- Frozen D `hq/lib/semantic-registry.mjs:getSemanticId/listSemanticIds/resolveSemanticAlias/listSemanticLinks`, and `hq/lib/annotation-v2.mjs:parseAnnotationV2` own semantic ID/source metadata; annotations never register an executor. The HQ compiled knowledge/semantic graph remains distinct from user personal memory or permissions.
- Frozen F `contracts/interop/v1/types.ts` and `protocol.schema.json` own a minimal `SourceRef`, `EvidenceRef`, `InertViewProjection` and risk vocabulary. V's richer schema is a **proposed projection shape**, not a divergent canonical command IR.

## 5. Explicit refused assumptions

No `availableNow`, `permittedForActor`, verified effect, persistent memory receipt, actor token, personal DCB identity or UI publisher revision can be created from V. Risk R3 is displayed as source metadata; it is NOT authorization. Coverage `complete` means one producer completed its declared facets, **not** that every participating subsystem ran. Any absent producer is unknown and must appear as such after the authoritative manifest is wired.
