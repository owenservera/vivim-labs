# GESTURES R002 — the smallest real PWA work packages

This is a PREPARATION breakdown attached to proposed outcome PV-15, not another active task queue. Each GEST item is a bounded dependency under the existing owner/Dev claims. No automatic worker, scheduling or model cost.

| ID | Priority | What changes / exact target | Existing source to reuse | Dependency | Observable criterion | Proposed owner / state |
| --- | --- | --- | --- | --- | --- | --- |
| GEST-01 | P0 | Ground current First Contact Wiki and gesture source atlas | app.js; wave4-device-semantics.ts; live_surface.rs | none | source pin, REUSE/REPAIR/gap truth | Planning / PREPARED |
| GEST-02 | P0 | Attach admitted page/section/source identity to actually rendered Wiki result | app.js renderFirstWiki and Rust Wiki source stamp | GEST-01, PV-01 | visible item maps to stable current ID; stale refused | Dev after acceptance / UNCLAIMED |
| GEST-03 | P0 | Minimal pointer input adapter: down/dwell/move/up/cancel, pan arbitration | First Contact DOM; browser PointerEvent | GEST-02 | UI reacts during hold, scrolling works | Dev after acceptance / UNCLAIMED |
| GEST-04 | P0 | Pure Rust deterministic gesture session reducer (inert) | live_surface.rs, live_system.rs | GEST-02 | reproducible transition + cancellation tests | Dev after acceptance / UNCLAIMED |
| GEST-05 | P0 | Reuse TierFlow gesture declaration, remove unsupported case only for accepted inert phase | wave4-device-semantics.ts, A039 source taxonomy | GEST-04 | verified effects-none normalized candidate | Dev after acceptance / UNCLAIMED |
| GEST-06 | P0 | Typed contextual proposals and live visual preview | vec/ui-adapter.mjs, generative-ui/renderer.mjs, TierFlow | GEST-02/04/05 | slide changes candidate preview without Host effect | Dev after acceptance / UNCLAIMED |
| GEST-07 | P0 | Explicit cancel and stale-source/revocation semantics | firstWikiScope/high-water, Host revision guard | GEST-03/04/06 | abort leaves state unchanged and visible stale/refusal | Dev after acceptance / UNCLAIMED |
| GEST-08 | P1 | One accessible preview surface (anchored strip first); same semantic path for keyboard/tap | First Contact Wiki markup, TierFlow renderer | GEST-06/07 | real PWA visible hold/preview/cancel; alternate activation | Dev after acceptance / UNCLAIMED |
| GEST-09 | P1 | Explicit Apply via pre-existing Host Control, approvals and receipt | live_surface admit_action and existing Host dispatcher | GEST-07/08, PV-06 | approved low-risk action/refusal with exact readback | Dev after acceptance / UNCLAIMED |
| GEST-10 | P1 | Optional consented Activity Fabric episode + correction without raw trace | activity_fabric/event.rs | GEST-07/08 | opt-in preview/cancel event safely observed in Wiki timeline | Dev after acceptance / UNCLAIMED |
| GEST-11 | P0 | Independent Rust/Host/browser adverse test matrix and one-UI regressions | existing test suites, current PWA runbook | GEST-03 through 10 | G0-G9 separately evidenced; no false PASS | Independent verifier / UNCLAIMED |
| GEST-12 | P0 | Same-link deployment/physical PWA acceptance + rollback evidence | existing Dev source-to-live playbook | GEST-08/11 for read-only; 09 later | in-progress visual proof, cancellation, source SHA, rollback | Dev/local scoped verifier / UNCLAIMED |

## Dependency waves (not parallel UI projects)
- W0 **COMPLETED planning evidence:** GEST-01, philosophy, source map, gates and handoff only.
- W1 **Read-only tactile prototype:** GEST-02/03/04/05 -> GEST-06/07/08 -> GEST-11/12. The owner can see a real in-progress hold and cancel.
- W2 **Executable semantics:** GEST-09 after existing PV-06 admission; no new Host permission.
- W3 **Optional learning and modalities:** GEST-10 and later speech refinement; no automatic background compiler.

## Ownership and execution
Planning may enrich this packet, but must not edit Dev-owned ARBOR source or claim GEST-02..09 as active. First check shared TASK-CLAIMS and owner registry; receiver acceptance must be explicit. Dev code source in cloud/GitHub main, Rust first, TS7+ only for DOM. Local operator after push may reproduce physical PWA only. No parallel journal, graph, router or task queue.

The full machine-readable projection is manifest.v1.json; task statuses there are proposed preparation metadata, NOT an authoritative live registry.
