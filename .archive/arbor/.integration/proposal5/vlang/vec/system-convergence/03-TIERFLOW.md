# TierFlow deep research — the visible programming environment

## Source-level truth
TierFlow has **three different UI payload models**:
- `packages/tierflow/src/ui/types.ts`: persisted World/1, objects, five semantic layers, relationships and data flows; pure transforms in `world.ts`, source-preserved versioned SQLite/CAS in `store.ts`.
- `packages/tierflow/src/visual-runtime/document/schema.ts`: Canonical Visual `vivim.ui-document/1`, 10 component kinds, 32KiB/64-node/7-deep limits, accessibility, typed binding and action references, fail-closed validator and inert RenderPlan.
- `apps/mobile-relay/generative-ui/catalog.mjs`: phone Build `vivim.ui-surface/1`, separate theme/component tree with its own validator/renderer.

A **real safe subset** World→Visual adapter exists at `world-visual-bridge.ts`: maps static text, box and plane to text/cards, rejects dynamic/unsupported objects, retains source relationships/dataflows only in the pinned original, creates no knowledge/action bindings and only proposes review-required reverse *move* edits. The generic mapping remains incomplete; `lane-b/studio-editing.ts` still reports `SCHEMA_BRIDGE_NOT_READY`.

TierFlow command compiler currently supports closed exact `/semantic.id@1 key="value"` grammar; it is not unrestricted natural-language intent analysis. Host phone already has deterministic NLCL. Final-wave Lane A combines trusted NLCL projections and visual controls to create **inert** authoring operations. Template/preview, multi-device integration and publisher fence remain non-effectful source; no physical acceptance.

## The dream
Every UI object becomes a **programmable affordance**: discover its source, permissions, transform, update cadence, revision, last receipt and edit possibilities. The user says 'show my active tasks here' or drags a task source to a card. Both yield the same typed ProgramDelta candidate. TierFlow previews what would change, including denied/unknown data, and Host approves the program/effect; user never learns JSON or Rust.

Instead of relying on a model for every Build request (`apps/mobile-relay/build-composer.mjs` currently drafts a model prompt), try **NLCL+installed SDK+typed TierFlow operations first**, then optional LLM proposal for unfamiliar layouts. Ordinary execution uses precompiled Rust/Host functions and updates a single authorized TierFlow projection on phone and Windows.

**Key prerequisite:** choose the existing Host-controlled canonical publisher and implement loss-reportable crosswalk across World/1, Visual/1 and phone UI Surface/1. Do not activate TierFlow's historical independent server/store as a duplicate. Inert visual diff/gesture must not grant an effect. A UI rollback is not an undo of a previously sent message or deletion.

## TierFlow source atlas
| Inspected source | Finding | Evidence |
|---|---|---|
| [types.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/ui/types.ts) | World/1 objects, layers, relationships and dataflows | SOURCE_PRESENT |
| [world.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/ui/world.ts) | Pure World transformations and closed transform catalog | SOURCE_PRESENT_PURE |
| [store.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/ui/store.ts) | Bun SQLite draft/revision history; not a second active product store | SOURCE_PRESENT_UNACCEPTED_PUBLISHER |
| [schema.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/document/schema.ts) | Visual/1 bounded 2D-first tree, a11y and binding refs | SOURCE_PRESENT_INERT |
| [validate.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/document/validate.ts) | Closed fail-safe document validator | SOURCE_PRESENT_INERT |
| [dom.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/render/dom.ts) | Semantic DOM render plan, not product mount | SOURCE_PRESENT_INERT |
| [compile.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/command/compile.ts) | Slash-only exact semantic command compiler | SOURCE_PRESENT_INERT |
| [normalize.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/final-wave/lane-a/normalize.ts) | UI authoring candidates via trusted HQ/NLCL projections | SOURCE_PRESENT_INERT |
| [studio-editing.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/final-wave/lane-b/studio-editing.ts) | Review/CAS integration; visual2d not fully bridged | SOURCE_PRESENT_BRIDGE_INCOMPLETE |
| [templates.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/final-wave/lane-c/templates.ts) | Metadata-only UI templates | SOURCE_PRESENT_INERT |
| [world-visual-bridge.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/final-wave/integration/world-visual-bridge.ts) | Static text/card projection and reverse move proposal; no dynamic bindings | SOURCE_PRESENT_SAFE_SUBSET |
| [schema-boundary.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/final-wave/integration/schema-boundary.ts) | Persisted World/1 != canonical Visual/1 | SOURCE_PRESENT_BRIDGE_INCOMPLETE |
| [master.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/final-wave/integration/master.ts) | Final-wave composition, no native/device GO | SOURCE_PRESENT_SYNTHETIC_ONLY |
| [cutover-dry-run.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/final-wave/lane-g/cutover-dry-run.ts) | Single publisher fence dry-run only | SOURCE_PRESENT_INERT |
| [verification.ts](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/src/visual-runtime/final-wave/lane-h/verification.ts) | Real Windows/Android/Host evidence gates | SOURCE_PRESENT_GATE_UNRUN |
| [package.json](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/packages/tierflow/package.json) | Internal TierFlow package, no product server launcher | SOURCE_PRESENT |
| [control-language.mjs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/apps/mobile-relay/control-language.mjs) | Deterministic NLCL typed intent resolver | SOURCE_PRESENT |
| [build-composer.mjs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/apps/mobile-relay/build-composer.mjs) | Current Build drafts UI via provider design prompt | SOURCE_PRESENT_LLM_FIRST |
| [catalog.mjs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/apps/mobile-relay/generative-ui/catalog.mjs) | Phone ui-surface/1 is separate schema | SOURCE_PRESENT_THIRD_UI_SCHEMA |
| [host-revision-guard.mjs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/apps/mobile-relay/generative-ui/host-revision-guard.mjs) | Committed Host visual revision read guard | SOURCE_PRESENT_DEVICE_UNRUN |
| [command-program.mjs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/apps/vivim-host/control/command-program.mjs) | Existing Host action DAG compiler | SOURCE_PRESENT_INERT |
| [dispatcher.mjs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/apps/vivim-host/control/dispatcher.mjs) | Current effect approvals, dispatch, journal and receipts | SOURCE_PRESENT_AUTHORITY |
| [personal-rule-store.mjs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/apps/vivim-host/control/personal-rule-store.mjs) | Owner-reviewed local personal semantic rules | SOURCE_PRESENT |
| [main.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/src-tauri/src/main.rs) | Native Windows shell and Tauri Rust validation adapter | SOURCE_PRESENT_NATIVE_UNRUN |
| [model.mjs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/apps/mobile-relay/ux/model.mjs) | Host-grounded six-tab mobile model | SOURCE_PRESENT |

## Acceptance gaps
Real Host mount, dynamic bindings, source-to-visual crosswalk, phone/Windows parity, accessibility, one publisher, permission/revoke safety and low-disk rollback remain separate gates. Source-present fixture modules do not equal full installed app acceptance.