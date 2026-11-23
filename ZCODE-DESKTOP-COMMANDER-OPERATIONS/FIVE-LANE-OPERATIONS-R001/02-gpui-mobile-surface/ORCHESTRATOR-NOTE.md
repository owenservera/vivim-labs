# L02 / GPUI & mobile release — current orchestrator note

## OWNER OBJECTIVE — ONE GPUI RENDERER, PWA AS REMOTE VIEWPORT (2026-11-23)

**Latest explicit owner decision; supersedes previous separate GPUI-vs-PWA renderer or shared-schema-only parity plans.** VIVIM must have **one actual product UI renderer: native Rust GPUI**. The installed Android PWA at the **same existing owner HTTPS URL** is a **thin remote viewport of GPUI-rendered output**, not a second app interpreting a component tree into an independent DOM product interface. Adding or modifying a button, layout, animation or component must be done **once in GPUI** and visible on both native Windows and the paired phone. A phone gesture, keyboard/IME event or button tap returns to the correct owned GPUI viewport/session; effects, grants, receipts and approvals remain with the incumbent Host.

### Required implementation/acceptance path (sequence, not a duplicate tracker)

1. **Complete/reconcile existing S03 claim before overlapping source edits.** Refresh ARBOR `main`, #31/#33, Labs TASK-CLAIMS and L02 file owners. Preserve GPUI, Host and PWA production functions while building a candidate. Do not edit Dev-owned `apps/mobile-relay/app.js` or Host files without a new nonconflicting CAS checkout.
2. **Inspect the real Rust GPUI render pipeline and the existing Host/PWA transport.** Identify whether GPUI can render a separately sized **phone viewport** and export frames directly; if not, implement a small source-owned native capture/offscreen bridge (prefer Rust), reusing existing packaging, identity, pairing, receipts and connection. **No second native or web product renderer, action dispatcher, journal, browser controller, Host or parallel protocol.** A shared JSON design/IR alone is NOT acceptance of this objective.
3. **Build the thin PWA client:** GPUI frames (or verified frame deltas) **down**, timestamped/revision-fenced pointer/touch, focus, keyboard, IME composition, scroll, accessibility input **up** over the existing authenticated channel. The PWA may retain native browser transport, reconnect, pairing, offline/unavailable notice, virtual keyboard and accessibility affordances; it must not build independent product layout, navigation, buttons or UI state.
4. **Button proof first:** add **one real inert/preview native GPUI button** to the source-owned phone viewport; show that its same rendered appearance occurs in the PWA without a web button definition, and a real phone tap maps back to that exact GPUI control ID with a visible no-effect receipt. Then migrate the existing composer, Wiki, navigation and richer controls without losing behaviors. Keep provider sends/Host mutations approval-gated.
5. **Performance and safety:** responsive phone-sized viewport with deterministic scale/hit testing, transport backpressure/adaptive frame rate, durable owner/session/revision correlation, no duplicate taps on reconnect, secure pairing and private frame exposure, IME/multiline and accessibility, stale/disconnected Host and GPUI failure states. A sleeping/offline desktop cannot promise newly rendered frames; the PWA must present an honest limited offline shell, not counterfeit UI.
6. **Release gates:** exact-SHA Rust/JS tests, repeatable GPUI-frame→PWA render and PWA-touch→GPUI control tests, isolated build/provenance and rollback; verified PC and physical Android screenshots, latency/jank and source/Host receipt. Keep old PWA operational until the new route is accepted; no live restart, pairing/URL change or unapproved promotion.

**Explicit distinction:** **single renderer** (GPUI pixels/frames shown on phone) is stronger than **two renderers reading the same contract**. Do not propose the latter as final fulfillment. Source authority: this L02 objective + owner comment on canonical [#31](https://github.com/owenservera/ARBOR/issues/31), with delivery status in existing [#33](https://github.com/owenservera/ARBOR/issues/33) and claims; do **not** start another tracker. All pre-existing S02–S14/U01–U04/C04–C05 tasks remain open until their actual acceptance evidence is complete.

**Snapshot:** 2026-11-23; owner instruction: continue coding/testing across successive unclaimed tasks, do not stop after a SITREP. This is an asynchronous GitHub note, **not an agent wake or source claim**. Refresh [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33) and [live claims](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md) before acting; source changed after observation.

**Verified progress:** Last confirmed L02: S02 rollback-preserving package swap committed 0ac635a95f5eca9aabceb08a0d7ddf96cfae3b77, exact test 15/15 PASS; S04 root Cargo.lock and bun.lock is ACTIVE in claims at the latest check. Neither same-SHA live Host+GPUI+PWA nor physical Android is accepted.

**Next safe execution:** Finish S04 root lockfiles in detached verified source with real toolchain, commit only claimed files; release claim with exact-SHA test and preserve prior package rollback. Then isolated S02/S05 candidate, stable URL and paired-device proof.

**Boundaries:** Respect incumbent app.js and Dev PY-001 Host files; no live Host restart, phone URL/pairing change, or release promotion without no-effect preflight and rollback.

**Cross-lane handoff:** [Orchestrator R005](../ORCHESTRATOR-NEXT-ACTIONS-R005.md) and [critical path R004](../STRATEGIC-EXECUTION-SEQUENCE-R004.md). One claim→code→exact-source test→receipt→release→next task cycle at a time. At true stop, post exact SHA/tests/blockers; never claim an agent is still running after a ChatGPT turn ends.

## Orchestrator observed follow-up — 2026-11-23 21:57 UTC

**New evidence:** L02 S04 root Cargo.lock + bun.lock committed [ARBOR 6c583d7](https://github.com/owenservera/ARBOR/commit/6c583d7b0d04981dc363110f83e4ad02603c386a), scoped claim released; lock metadata verified, actual full package/live GPUI/Android STILL OPEN. Continue S02/S03/S05 one immutable same-source stage+rollback and validate installed Bun version.

**Interpretation:** Source-tested increments are real; canonical #33 delivery checkboxes remain subject to separate live/native/phone acceptance. Recheck current claims before code writes. No automatic agent wake or model execution is implied by this note.
