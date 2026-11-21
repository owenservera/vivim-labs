# ChatGPT Dev integration handoff — GESTURES R002

**Status:** PREPARED / OFFER NOT ACKNOWLEDGED. Dev owns active ARBOR source; this Planning folder is NOT permission to modify it. Before starting, fetch current GitHub main and shared TASK-CLAIMS.md, confirm no other owner holds paths, and use source pin cf2e5c8 only as this preparation baseline.

## The concrete user-visible outcome
Keep the exact current First Contact one-text-box UI and stable PWA URL. With a real non-sensitive Wiki result: press/hold visibly reveals contextual operations while the finger is down; motion changes an inert preview; release and cancel do not mutate; an explicit accessible action can request approved Host execution only after the first read-only path works. Visible Wiki readback must reflect real source revisions.

## Reuse, do not rebuild
- First Contact result DOM and Rust-backed search: apps/mobile-relay/app.js renderFirstWiki, refreshFirstWiki, mountFirstWiki.
- Existing TierFlow input: packages/tierflow/src/visual-runtime/input/wave4-device-semantics.ts, currently UNSUPPORTED_GESTURE.
- Existing Rust LiveSurfaceDocument / LiveSystemState: core/programmability/src/live_surface.rs, live_system.rs.
- Existing Activity Fabric: core/programmability/src/activity_fabric/event.rs.
- VEC preview and TierFlow source identity: apps/mobile-relay/vec/ui-adapter.mjs and generative-ui/renderer.mjs.
- Existing A039 Gestures Engine specification is historical PREPARED_UNRUN, not installed production.
- Host Control handles all effects. No gesture-effect endpoint, no duplicate graph/DB/router and no auto-playing click simulation.

## First integration slice — read-only hold/preview/cancel
1. Add stable semantic identity to visible First Contact Wiki result (page/section PLUS current admitted Rust source ID/digest/revision; never raw heading-only identity).
2. Add a minimal DOM/PointerEvent TS7+ capture adapter scoped to actual Wiki item and browser lifecycle; preserve native scrolling/tap/details and IME. Use no global touch-action:none.
3. Add deterministic Rust session transition validator; validate origin actor, target, revision, pointer id, monotonically increasing sequence, expiry and cancel/recovery. Return inert preview only.
4. Extend existing TierFlow trusted input normalization for supported inert gesture phases; leave unsupported modalities explicit, reusing A039 taxonomy, not implementing a duplicate engine.
5. Render one accessible contextual preview through existing visual projection with user-visible loading/stale/unsupported states. On release retain only an explicit option to Apply; never execute.
6. Verify source/grant changes invalidate the preview, and no Host effect appears on down/move/up/cancel. Run actual browser+phone PWA acceptance before any effectful phase.

## Second slice — effect through Host, not gesture
- Only after G0-G4 and G7 are independently accepted: wire a safe existing Host action to a separate explicit Apply control.
- Reuse existing grant, approval, idempotence, journal and receipt; preserve revision fences and denial/readback handling.
- If no such action is approved for First Contact, leave G5/G6 BLOCKED; do not turn ambient Wiki lookup into a writer.

## Bounded ownership/merge instructions
- Planning has changed **only Labs documentation** in this round. Do not infer Dev acceptance from a link.
- Dev source paths are potentially contested: app.js, First Contact composer, TierFlow wave4 semantics, Rust live_surface, VEC adapter. Claim exact files first; preserve concurrent dirty paths and small direct-main commits. No new branch/PR by default.
- Rust first; TypeScript 7+ only for browser PointerEvent/DOM binding. Keep UI preview deterministic and non-effectful.
- Reuse current Dev source-to-live playbook: SOURCE -> TEST -> PACKAGE -> SERVE -> AUTH-ACK -> HTTPS -> PHONE. Report actual tests and current deployed SHA, not this historical source pin.

## First code review and proof order
- Source pin and claim diff; decide actual supported stable Wiki identity.
- Pure transition tests, TierFlow negative tests, browser pointer/scroll/cancel tests, Host no-effect assertions.
- Then existing PWA 180ms typing and authorized agent typing regressions; do not break main user experience.
- Release through current stable link and independently capture at least three meaningful real frames (down/hold preview, altered selection, cancellation), plus source-fenced evidence.
- Only then test explicit Host-approved Apply in an authorized isolated state and receipt/readback.

## Completion receipt to return to Planning / Oracle
PV-15, GEST work IDs and G-gates; accepted Dev claim ID; current ARBOR main SHA; changed paths; actual exact test commands and exits; released SHA; same PWA URL; user-visible actions/results; screenshots/receipts; fresh regressions; gaps/rollback. The Planning claim stays unaccepted until Dev says ACCEPTED_BY in the shared claim overlay.

See [SOURCE-ATLAS-R002.md](SOURCE-ATLAS-R002.md), [ARCHITECTURE-R002.md](ARCHITECTURE-R002.md), [WORK-BREAKDOWN-R002.md](WORK-BREAKDOWN-R002.md), [ACCEPTANCE-R002.md](ACCEPTANCE-R002.md).
