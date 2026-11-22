# R003 — shared gesture acceptance matrix
Every gate remains UNRUN for actual product. This extends R002 G0–G9, never retroactively upgrades them to PASS.

| Gate | Native GPUI/PC | Responsive phone PWA | Evidence needed |
| --- | --- | --- | --- |
| G0 | Real source-backed Wiki result and stable target | Same Wiki source identity/revision | exact Host source/digest and viewport revision |
| G1 | Mouse/touch/keyboard hold reveals contextual possibilities while active | Long press reveals while finger remains down | real frames; no separate UI fallback |
| G2 | Movement/refinement updates inert candidate | Finger movement updates same semantic candidate | candidate ID and visual revision, not only CSS mock |
| G3 | Escape/focus loss/release cancels without persisted mutation | pointercancel/scroll/background cancel safely | no effect and actual source unchanged |
| G4 | stale source, grant, cross-viewport revision invalidates preview | same | accepted/refused with precise source fences |
| G5 | explicit Apply goes to existing Host grant/plan/receipt | same | zero side effects before explicit approval |
| G6 | GPUI reads real Host effect outcome | PWA reads same outcome | original Host receipt, PC/phone source-backed redraw |
| G7 | keyboard/accessibility/drag-free path | screen-reader/tap alternative | actual usable controls, reduced-motion path |
| G8 | native Windows app (not Edge/WebView), app shell unaffected | same unchanged phone PWA URL | simultaneous physical device evidence |
| G9 | consented Activity Fabric episode, no raw capture by default | same | redacted journal proof or zero capture |
| G10 | GPUI/PWA produce same typed gesture phases for same semantic scenario | same | Rust event traces from both without divergent effects |
| G11 | visible PC→phone and phone→PC shared state confirmation after authorized changes | same | distinct source revisions/receipts, no guessed atomicity |

## Compiler-level staged module checks
- 17 authored `#[test]` test markers in shared [Rust precode](src/gesture_interaction.rs), NOT 17 passing tests.
- Static delimiter balance: PASS by pure source-text check; Rust compiler, formatter, clippy and actual unit tests: UNRUN because this ChatGPT cloud environment currently lacks rustc/cargo.
- Main-entry consumer packaging, GPUI adapter, WASM, PWA, Host and device acceptance: UNRUN.

## Safety negatives
Unknown candidate; malformed source digest; changed Wiki revision; wrong actor/pointer; duplicate/late input; early hold; release without choice; repeat Apply; cancelled/stale state; revoked grant; no browser scroll interception; keyboard alternative; offline/disconnect. No silent paid model or autonomous effect runner is needed to prove this core.
