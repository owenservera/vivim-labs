# R004 — shared Rust gesture core on ARBOR main
Date: 2026-11-22 (Europe/Madrid). This is a **source integration receipt**, not GPUI/PWA acceptance.

## Coded + pushed
- ARBOR [bb6873b3be280980b453fba006ff5c771d207471](https://github.com/owenservera/ARBOR/commit/bb6873b3be280980b453fba006ff5c771d207471), parent `4a8f63e86dc4384ef53bdc96c4c81068bb10bdd7`.
- Exact changed source paths: `core/programmability/src/gesture_interaction.rs` (from Labs R003 staged source blob `e5f9d40e22a8bebce65dff4f7112c4e4600235d0`), `core/programmability/src/lib.rs` (new `pub mod gesture_interaction;`).
- GitHub re-fetch of main after commit returned exact module blob and module registration. Main had advanced with independent commits; no unrelated source edited by this lane.
- Shared Labs `TASK-CLAIMS.md` records ChatGPT Dev / gesture-core-20261122 acceptance of the core-only handoff and CODED/PUSHED status; GPUI and phone adapter paths remain UNCLAIMED.

## Implemented contract (inert only)
`Session::begin` accepts bounded trusted source identity and reviewed candidates; `advance` handles Dwell/Move/Release/Cancel with monotonic sequence, source revision, actor and pointer guards; `propose` after a separate Apply returns `host_authorization_required:true`, `authorized:false`, `effect_invoked:false`; `invalidate` closes stale sessions.

No new Host route, grants, effect engine, UI store, journal, WebView, PWA shell or GPUI renderer. No real device input is yet calling the module.

## Evidence state
| Gate | State | Reason |
| --- | --- | --- |
| GitHub source presence and export on main | PASS (source inspection) | File and registration fetched after commit |
| Exact code SHA parity with Labs | PASS (source inspection) | ARBOR blob equals staged Labs `e5f9d40` |
| Authored Rust tests | 17 markers verified | Test definitions are not executions |
| `cargo test --manifest-path core/programmability/Cargo.toml gesture_interaction --lib` | UNRUN | `cargo` / `rustc` unavailable in cloud container; internet unavailable for install |
| Native GPUI cargo regression, WASM/build, PWA regression | UNRUN | Uncompiled core and adapters not integrated |
| First Contact on native PC and mobile; hold/preview/cancel | UNRUN | No GPUI or phone adapter wired |
| Authorized Apply and same-Host readback | UNRUN | Out of this pure-core source scope |
| GitHub Actions runs query for this commit | None returned | No CI PASS can be claimed |

## Next bounded source tasks
1. Run real cargo tests in authorized cloud CI or explicitly approved post-push local build before declaring Rust passing; do not request local source edits.
2. Inspect/claim existing native GPUI event and view paths (not app chrome) for shared gesture phases; no browser fallback on PC.
3. Inspect/claim existing phone Wiki/result + Rust/WASM bridge and connect PointerEvent to same reducer; preserve scroll, tap, IME and Wiki freshness.
4. Add tests for equivalent semantic phases in GPUI/PWA, stale/revoked/duplicate events and exact Host revision; release to unchanged PWA URL only after independent device acceptance.
5. Preserve the existing A039/VEC/TierFlow feature identity; do not create a competing second gesture engine.

Current outcome PV-15: **NOT ACCEPTED, not deployed**. R004 advances the GEST-04 source state only.
