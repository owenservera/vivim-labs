# R001 — Master PRD: Native User I/O as an Internally Consistent Rust System

Date: 2026-11-12
Status: PRD accepted for engineering experiment / source implemented partially / build & E2E UNVERIFIED.
Product owner scope: Rust core side. UI workstream owns presentation and high-level affordances.

## Read baselines (pin)
- UI Labs branch \`lab/nlp-visual-command-ui-20261111\`: \`20f393c953e2b6d83201d18fee2f2dd56deeaa68\`
- Harness main baseline: \`cae97afe848d79957c9d6c698a90996a758bfd56\`
- HQ main baseline: \`7efb52b251fd56c9257a3184ac39dc19d84340b5\`
- Native implementation branch: \`work/native-user-io-r001-20261112\`, initial source head \`fa6dea9576c83496d95562b98b7a815fda4aec88\`. A branch SHA is not compilation evidence.

## UI team evidence interpreted accurately
- R005 reports 28/28 Node tests and 100/100 synthetic probes. It built a source-backed read-only semantic lens, keyboard Tab/Esc and context-sensitive menus with quiet/assisted/X-ray modes; browser/device acceptance remains unrun.
- R006 is 100 predictive, realtime, two-Tab **design proposals**. It explicitly requests parallel Rust hypothesis fan-out, source-verified project/focus slots, read-only speculative HQ/graph prewarming and out-of-order protection.
- Present Lab architecture remains source-backed visual projection, not a product command execution owner; this Rust workstream must not fork that semantic authority.

## Product outcome
User initiates any interaction through typing, voice, touch, accessible keyboard, connected phone, etc. A typed immutable event crosses one authenticated transport ingress and fans out concurrently to native Rust computational systems. Each independent subsystem observes relevant events and derived evidence, produces versioned, source-linked results and invalidates stale dependencies. A native projection of **one coherent interaction state** supports all user-facing output surfaces. No semantic master router decides which computation runs.

The predictor can rank probable future intent frames and warm permitted pure computations. In the reference flow, typing \`full\` exposes a likely project update; Tab 1 selects a grounded project, Tab 2 selects a preferred focus such as \`next steps\`; the result is a fully specified, **unexecuted** intent. Explicit submission and Host effect authorization remain distinct.

## Ownership matrix
| Layer | Owns | Must not own |
| --- | --- | --- |
| Rust event ingress/fabric | Bounded input revision, independently subscribed native reactors, typed derived signals, snapshot/CAS, convergence | Permission minting, semantic master routing, new global event DB |
| Rust graph/compute | Existing graph-node algorithms, typed DAG, invalidation, scheduling, provenance, source-safe retrieval | Personal corpus in HQ development DB, unsafe external actions |
| Rust predictive reactor | Per-user scoped ranking of confirmed templates/slots, pure speculative prewarm, rejection/correction learning | New command identities, grants, automatic provider sends |
| Host authority | Trusted principals, action IDs/risk, current grants, World revisions, approval, exact effect journal and receipts | UI layout |
| Studio provider browser | Signed-in web-app operation through existing Host lease and send-once controls | Provider APIs or inferred effect success |
| UI lab/TierFlow | Text spans, chip shape, symbol, color, layout, menu physics, accessibility language and navigation | Independent command execution, separate predictive truth |
| HQ | Development-intelligence source and Rust graph research | Product auth, user-private personal fact ownership |

## Round 1 must build (MUST)
R001-REQ-01: Read UI R005/R006 and crosswalk capabilities/unfinished gaps into Rust entrypoint contract.
R001-REQ-02: Add an existing-product-crate-native, pure \`user_io\` module, **not** a new daemon or repository.
R001-REQ-03: Implement authenticated-transport-ready typed draft events and independent Rust reactors. Real actor authentication is a later Host binding; R001 native input itself cannot assert authority.
R001-REQ-04: Implement deterministic revisioned evidence convergence where completion order cannot change output order.
R001-REQ-05: Implement \`full\` template prefix and grounded active-project resolution **from trusted native-supplied references**, with no fixture masquerading as live data.
R001-REQ-06: Predict focus choices from accepted-pattern counts and update in-memory ranks only on explicit Tab acceptance.
R001-REQ-07: Two-Tab progression with revision CAS, IME composing guard, negation guard and inert submit marker.
R001-REQ-08: Create a pure Tauri \`user_io_event\` command; do not wire any real effect endpoint, secret source, or UI server path yet.
R001-REQ-09: Write unit tests for successful flow and critical refusal/negative paths.
R001-REQ-10: Document exact gaps required to reach production and keep UI's high abstractions separate.

## Round 1 should design (SHOULD)
- Bidirectional native event/projection protocol for text, voice, touch, accessibility, device and automation input; only the read-only text+Tab subset implemented first.
- Subscription interfaces and graph dependency lifecycle for pure source reads with pinned evidence.
- Cancellation/backpressure and bounded global scheduler migration from per-event scoped threads.
- Native single-writer session storage and mobile cross-device continuation with authoritative Host revisions.
- User-private, opt-in, bounded predictive habit store; correction/undo/forgetting and model calibration.
- Host-authorized source/project grounding and capability/readiness proof from canonical catalogs.
- Rust graph crate convergence plan, packaging, FFI discipline and no duplicated graph or journal.
- Read-only output-to-TierFlow contract, no formatting authority in native Rust.

## Round 1 non-goals
- No new AI provider API, Playwright/Puppeteer, provider send, or automatic AI request while typing.
- No new central semantic router, planner hierarchy, user knowledge database, standalone scheduler or effect journal.
- No phone APK build, native JNI binding, production personal-memory import, semantic Unicode span editor, signed-in browser acceptance or actual provider operations.
- No claiming Rust code built, tests passed or UI integration completed without evidence.

## Source implementation footprint
- \`vivim-web-harness/core/programmability/src/user_io.rs\`: four independent parallel read-only reactors, template/project/focus evidence, bounded sessions, CAS Tab/Submit, strict JSON boundary, test fixtures.
- \`core/programmability/src/lib.rs\`: exposes \`user_io\`.
- \`core/programmability/examples/user_io_r001.rs\`: synthetic two-Tab demonstration, explicitly fixture-only.
- \`src-tauri/src/main.rs\`: pure \`user_io_event\` read-only native command. It has **empty trusted sources by default**. The existing Host UI does not yet call it.
- \`core/programmability/USER-IO-R001.md\`: scope, run commands and truthful limitations.

## Architecture contracts and evidence
See:
- [R001-01 Event Fabric](R001-01-NATIVE-EVENT-FABRIC-PRD.md)
- [R001-02 Prediction](R001-02-PREDICTION-AND-PERSONALIZATION-PRD.md)
- [R001-03 Output Bridge](R001-03-NATIVE-OUTPUT-AND-UI-BOUNDARY-PRD.md)
- [R001-04 Acceptance & Integration](R001-04-ACCEPTANCE-AND-INTEGRATION-PRD.md)
- [Prior parallel-fabric architecture](../PARALLEL-EVENT-FABRIC-ARCHITECTURE.md)

## Definition of Done for R001 source checkpoint
Code has a source-owned branch; module is registered in the existing Rust crate and a read-only Tauri command has been authored; negative-path tests are committed; PRD suite and source map land in Labs; all unrun verification is clearly marked BLOCKED/UNRUN. This is **not** production GO.

## Definition of Done for R001 build gate (NOT YET MET)
On owner Windows workstation, \`cargo test --manifest-path core/programmability/Cargo.toml user_io\`, example run and Tauri cargo check complete; source line/SHA evidence captured; no test regressions; inspect Windows installer packaging; then a UI source adapter proves the same no-effects protocol from a real composer. Only after those observations can a later owner issue build GO.
