# Gestures R002 — source-verified reuse and gap atlas

Audited 2026-11-21. ARBOR main SHA: cf2e5c8da0d5f46f17a5de0cc24ab838778e60c0.
Static source audit only; NOT a live gesture test. Repin before coding. Consumer is the current First Contact text-box plus AutoWiki PWA.

| Capability | Actual source/symbol | Verified behavior | Disposition / next wire |
| --- | --- | --- | --- |
| Visible Wiki target | apps/mobile-relay/app.js:1487–1541, renderFirstWiki | Real search results render as details.first-wiki-item with page/section, read-only excerpt and grant/revision/digest guard on toggle | REUSE; bind a stable Rust-admitted item identity. Do not derive action identity from raw text. |
| Wiki lifecycle | apps/mobile-relay/app.js:1542–1639, refreshFirstWiki/scheduleFirstWiki/mountFirstWiki | Wiki input has 180ms debounce; stale source and reset logic exist | REUSE; cancel gesture on reset, grant change, rerender, epoch/source change |
| Phone visual selection | apps/mobile-relay/generative-ui/phone-surface.mjs:91–104,153–183 | Revision-gated Build-mode selection and inspector click | REUSE as pattern, not yet linked to First Contact Wiki |
| TierFlow input | packages/tierflow/src/visual-runtime/input/wave4-device-semantics.ts:15–80,111–122,211–222 | Gesture union includes long-press, pinch, swipe, double-tap; normalizeWave4DeviceInput currently returns UNSUPPORTED_GESTURE; cancel is no-effect | REPAIR; inert phase preview through trusted input bindings |
| VEC A039 | .integration/proposal5/vec-ui-engines/agents/A039-gestures-engine.md | Prepared gesture engine work order, status PREPARED_UNRUN; historical branch instruction | REUSE capability taxonomy, no second engine; main-only current policy |
| Pure VEC preview | apps/mobile-relay/vec/ui-adapter.mjs:60–135 | Source-closed identity checks; PURE_UI_PREVIEW for supported UI edits; unsupported complex intent refused | REUSE only supported commands; never claim open-ended voice reasoning |
| TierFlow DOM renderer | apps/mobile-relay/generative-ui/renderer.mjs:68–78,99–102,132–148 | data-component-id mapping; projected click selection is inert/disabled unless explicitly local | REUSE visual projection for preview, not effect |
| Live surface core | core/programmability/src/live_surface.rs:119–149,321–367 | InputKind covers text/composition/key/click/focus/blur/scroll/submit, not gesture phase; admit_action produces inert candidate | REPAIR for phase observations; preserve Host action boundary |
| Activity Fabric | core/programmability/src/activity_fabric/event.rs:20–32,63–97,160–191 | Kind::Gesture and opt-in privacy checks; normalized pointer coords | REUSE for consented semantic episode, not raw motion recording |
| Agent input | apps/mobile-relay/first-contact-composer.ts:25–99; app.js:2828–2841 | Separate human/agent revision and ACK semantics, typingIntervalMs 225 | DEPENDENCY ONLY; no Dev-owned composer edits from Planning |

## Important factual gaps
1. Explicit TierFlow UNSUPPORTED_GESTURE branch is confirmed in the inspected input normalizer.
2. The Rust LiveSurface InputKind has no continuous gesture-phase representation, though Activity Fabric has a higher-level Gesture episode kind.
3. The actual first-contact Wiki renders source-backed read-only details but has no continuous hold/adjust/preview binding in the inspected renderer path. This does NOT prove no gesture logic exists elsewhere.
4. No source or live device proof was found that pressure is reliably force-sensitive, VEC gesture preview is mounted on this Wiki, or Host gesture execution is already implemented.
5. Static ARBOR main is not equivalent to deployed PWA, current phone behavior, full cargo test or actual browser acceptance.

## Collision control
Dev owns PWA typing/First Contact and active app.js. ChatGPT Planning authors source-pinned Docs and a precise non-overlapping handoff, not live ARBOR implementation. Before any product integration refresh GitHub main and the shared claim/ownership overlay.

Pinned tree: https://github.com/owenservera/ARBOR/tree/cf2e5c8da0d5f46f17a5de0cc24ab838778e60c0
