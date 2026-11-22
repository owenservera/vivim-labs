# Gestures R003 — bounded ChatGPT Dev integration recipe
Status: READY_FOR_HANDOFF, RECEIVER ACK NOT OBSERVED. Production source owner: ChatGPT Dev, via GitHub ARBOR main only. Planning authored only Labs precode.

## Preflight
1. Refresh ARBOR main, read AGENTS.md + ONE-CANONICAL-LIVE-SURFACE.md + 2026-11-22 full GPUI/PWA handoff. Check TASK-CLAIMS for active source owners and reconcile collisions.
2. Inspect the actual native GPUI semantic target renderer in native/gpui-desktop/src/main.rs and native visual.rs, current PWA Wiki target mounting in apps/mobile-relay/app.js, and the already existing LiveSurface/VEC/Host paths.
3. Validate semantics and source IDs before assuming every Wiki row is a canonical action target. Existing First Contact rows are still read-only contextual results.

## Minimal cloud-code activation sequence
1. **Core:** copy/review [gesture_interaction.rs](src/gesture_interaction.rs) into core/programmability/src/gesture_interaction.rs, add `pub mod gesture_interaction;` to core/programmability/src/lib.rs. Keep it a pure library. Rebase against current source and run actual native tests before declaring PASS.
2. **Native GPUI adapter:** bind real native input phases to the same Rust Session state. Separate Windows app chrome remains outside shared product. Native typing/Wiki results and menu interactions must not regress; no browser UI fallback on PC.
3. **WASM/phone adapter:** expose a narrow pure Rust ABI through existing product kernel/browser bridge rather than authoring a JavaScript gesture reducer; TypeScript 7+ only maps browser PointerEvents + cancellation into the ABI. Bind to a source-backed Wiki/component, not an invented test card.
4. **Preview:** use existing VEC/TierFlow renderer and LiveSurface revisions for an inert, accessible contextual strip. Do not directly mutate the Wiki or send provider actions from down/move/up/release.
5. **Host action (later):** separate explicit Apply only, existing Host planner/grant/approval/receipt; do not create gesture-effect endpoint. If no safe existing First Contact command is available, keep this phase BLOCKED.
6. **Reconcile:** verify exact source revision and receipt on GPUI and phone PWA. A menu screenshot or 200 HTTP is not PC↔phone synchronization.
7. **Tests:** native rustc/cargo, WASM, browser negative/cancel, native GPUI manual, phone-browser manual, cross-device parity. Pin current deployed SHA, original phone URL, screenshot/trace refs. Each acceptance gate is separate.

## Staged source contract caveats
The precode does NOT independently verify Host-granted target/catalog provenance. Trusted caller must do so, otherwise its choice allowlist can be forged before pure reduction. Input monotonic elapsed_ms comes from a platform monotonic clock; it is not a wall-time timestamp and must not be fabricated from receipt time. Consumer must cancel/clear the Session if source changes even if no new pointer event arrives. Implement device-specific pressure/haptics only as a guarded optional enhancement; do not assume support.

## Tests to run after source checkout
- `cargo test --manifest-path core/programmability/Cargo.toml gesture_interaction --lib` (requires module registration and code compiling; **UNRUN here**).
- `cargo test --manifest-path native/gpui-desktop/Cargo.toml --lib` (regression).
- `cargo check --manifest-path native/gpui-desktop/Cargo.toml --features desktop --locked` (native compile).
- PWA browser test runner following existing apps/mobile-relay/package.json; do not infer a command from old notes.
- Real device manual gestures and source-stamped readback; do not run effectful tests on owner’s personal production Wiki.

## Handoff status
PREPARED / OFFERED_TO:CHATGPT-DEV. No source checkout, receiver ACK, ARBOR commit or gesture deployment is represented by this Labs packet. Any Dev acceptance must be explicitly written to the existing TASK-CLAIMS.md overlay.
