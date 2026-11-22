# Gestures R003 — one live interaction, two viewports
Date: 2026-11-22. Status: SHARED CORE PRECODE STAGED, NOT IN ARBOR.

## Superseding architecture
The owner rule in ARBOR docs/product/ONE-CANONICAL-LIVE-SURFACE.md supersedes the R002 phone-only reading:
**one canonical VEC/LiveSurface semantic state → native GPUI Windows master viewport + responsive mobile PWA viewport.**
Native GPUI handles Windows touch/mouse/keyboard; PWA handles browser PointerEvents. The two adapters must produce the SAME typed input contract. Shared Rust logic, not browser heuristics or GPUI-specific permission code, interprets an interaction in progress.

GPUI is a native authoring/rendering environment, **not** the Host authority. TierFlow capabilities migrate to the shared Rust-led visual pipeline gradually; do not delete existing PWA/TierFlow code before parity proof.

## Gesture as a continuous dialogue
1. **Acquire:** source-backed Wiki/card/component identity with current surface revision, Wiki source revision+digest, scope fingerprint and actor identity. Coordinates and visible labels do not grant identity.
2. **Dwell/preview:** time-based long press (not presumed physical pressure) reveals reviewed candidate IDs. No action can execute while the pointer remains down.
3. **Explore:** movement among trusted candidate IDs changes an INERT preview. Unknown candidate refused; source/digest/actor/pointer mismatch fail closed.
4. **Release:** ordinary release leaves preview without applying, while short tap falls back to ordinary platform input behavior.
5. **Commit:** an independent explicit Apply control returns an inert proposal. Existing Host must check current actor, grant, policy, idempotence, source revision, target, action contract and receipt.
6. **Observe:** real Host outcome, Wiki revision, PWA presentation and GPUI native presentation reconcile through the current LiveSystemState/VEC source. No synthetic “success” or independent device journals.

## Staged code
[Shared Rust reducer](src/gesture_interaction.rs), no dependencies other than std, 17 authored tests. It has no network/storage/timer/DOM/window interfaces. The external caller MUST use already trusted source/catalog inputs and recheck their freshness; the pure reducer cannot authenticate source content or grant claims.

- `Session::begin`: requires bounded valid source identity, 250–900 ms trusted dwell and up to eight unique reviewed candidates.
- `advance`: Dwell / Move / Release / Cancel; monotonic sequence; actor/pointer/snapshot fences; no effectful operation.
- `propose`: only after separate Release+explicit Apply; returns `authorized:false`, `effect_invoked:false`, `host_authorization_required:true`.
- `invalidate`: terminates on source change, focus lost, revocation, transport loss or renderer unmount.

## Two adapter contracts (design, NOT connected)
- GPUI native: use actual current native/gpui-desktop event/focus/semantic widget path, not a WebView, Edge or a new UI shell. Feed target + input phase to the shared Rust reducer. Paint in native GPUI, outside the persistent Windows menu chrome. Native keyboard/accessibility alternatives must reach the same candidate.
- Mobile PWA: attach an eligible component in the existing First Contact Wiki/results; TypeScript 7+ only for pointer capture/cancel/DOM. Preserve native browser scroll, selection, IME, OS gestures; avoid global touch-action:none. Feed the SAME target/phase to Rust/WASM bridge.
- Both viewports: use existing LiveSurfaceDocument/visual revision identities, TierFlow/VEC preview semantics and Host effect authority. **A preview is ephemeral local state**; source-backed visible confirmations require readback.

## What is NOT done
Neither adapter is wired, no product Rust lib.rs registration, no WASM export, no live Host input/event bridge, no GPUI gesture surface, no PWA gesture surface, no cross-device gesture acceptance, no physical-pressure support and no real Rust compiler test. This is pre-integration source, not shipped behavior.

Related: [R002 evidence](SOURCE-ATLAS-R002.md), [Dev integration guide](R003-DEV-INTEGRATION.md), [Gates](R003-ACCEPTANCE.md).
