# N0P-04 — Source Truth Audit and Security Evidence

Date: 2026-11-12. Read-only connected GitHub source audit and official public advisories. No user-machine runtime test, cargo build, test execution, external exploit reproduction or device acceptance was run here.

## Pinned observed source branches (at audit)
| Repository | Branch | SHA | Audit scope |
| --- | --- | --- | --- |
| \`owenservera/vivim-web-harness\` | \`main\` | \`cae97afe848d79957c9d6c698a90996a758bfd56\` | Host, control, phone, Tauri configs |
| \`owenservera/vivim-web-harness\` | \`work/native-user-io-r001-20261112\` | \`fa6dea9576c83496d95562b98b7a815fda4aec88\` | First native parallel Rust I/O source slice, draft PR #55 |
| \`owenservera/repos-hq\` | \`main\` | \`7efb52b251fd56c9257a3184ac39dc19d84340b5\` | Rust research graph and engine |
| \`owenservera/tierflow\` | \`main\` | \`50822f11ee63c3d80f631f8fea7dcb9b19bffc38\` | Visual command/host adapter baseline (prior source read) |
| \`owenservera/vivim-labs\` | \`lab/nlp-visual-command-ui-20261111\` | \`96d97f8da36c376a611bb90f454b30ab376d1260\` | R005/R006/R007 UI visual interpretation |
| \`owenservera/repos-hq\` | \`research/interpretation-xray-intelligence-surface-20261112\` | [pin exact branch before merge] | HQ source census and interpretation-surface gap audit |

## Source facts vs inference

### 1. Tauri native shell currently boots a Node Host
Source: [Harness \`src-tauri/src/main.rs\`](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs)
- Imports \`Command\` and \`Stdio\`, launches packaged or checkout \`node apps/vivim-host/server.mjs\`.
- Waits for loopback \`http://127.0.0.1:43187/api/host/status\`.
- Loads the loopback Host UI into Tauri webview; native commands are \`host_is_running\`, \`app_version\`, \`validate_native_program\`.
**Conclusion:** In product main, desktop user interaction is not yet a comprehensive Rust-native I/O path. This is confirmed from source, not runtime.

### 2. R001 native spike is an inert demonstration, not product integration
Source: [R001 Rust \`user_io.rs\`](https://github.com/owenservera/vivim-web-harness/blob/work/native-user-io-r001-20261112/core/programmability/src/user_io.rs), [native command](https://github.com/owenservera/vivim-web-harness/blob/work/native-user-io-r001-20261112/src-tauri/src/main.rs), [draft PR #55](https://github.com/owenservera/vivim-web-harness/pull/55).
- \`Fabric::react\` iterates four registered subscribers and uses \`thread::scope\` per input invocation; joins all before projection.
- \`UserIoCore::apply\` mutates session, invokes the fabric synchronously and sets \`last_projection\`.
- Native Tauri \`user_io_event\` owns a \`Mutex<UserIoCore>\` and locks while calling \`apply_json\`.
- \`TrustedSources::default()\` means no real project, action or personalized history is installed in Tauri by default.
- \`UserIoCore::apply_json\` only supports draft, tab and submit test kinds; \`submit\` sets an inert status, no Host effect.
- Source-committed six Rust unit test declarations exist; no successful cargo test or Tauri build was observed.
**Conclusion:** Useful typed event/projection experiment; unacceptable as a production per-keystroke implementation without replacing locking/scheduling and trusted source boundary.

### 3. Current Host control semantics and effect owner are JavaScript
Sources: [Host planner](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/intent-planner.mjs), [NLCL language adapter](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/control-language.mjs), [dispatcher](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs), [journal](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/journal.mjs).
- Host planner builds typed candidates using NLCL and current action catalog.
- Dispatcher plan/execute path checks exact current plan, approval and an effect claim before fixed executor and receipt.
- Journal contains per-plan files and append-only event data; phone client explicitly treats uncertain effect outcome as not safe to retry.
- Host action catalog v6 has 19 entries and no canonical full-project-update action observed in ID/title/examples.
**Conclusion:** Host existing safeguards should be preserved; a Rust rewrite must prove equivalent semantics. The synthetic \`full\` action from R001 is not a real registered product action.

### 4. Phone PWA currently has a distinct JS transport/control path
Sources: [direct-phone](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/direct-phone.mjs), [mobile control client](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/control-client.mjs), [mobile app](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/app.js).
- \`direct-phone.mjs\` receives authorized phone traffic behind a trusted proxy and imports Host planner.
- \`control-client.mjs\` uses \`/mobile/api/control/\` for preview/execution, with owner/device checks; it is not calling the Rust I/O spike.
- Mobile app also distinguishes VIVIM command control versus provider conversation sending.
**Conclusion:** One unified user-facing semantic session across desktop and phone is target design, not current observed implementation. Keep security boundary and explicit provider effect semantics.

### 5. Existing Rust graph core has reusable computation but is not the shipping product interaction engine
Sources: [HQ \`engine.rs\`](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs), [\`typed_dag.rs\`](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/typed_dag.rs), [\`scoped_graph.rs\`](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/scoped_graph.rs).
- Engine registry manages versioned compute engine generations, leases and capability validation; scheduler/budget/cancellation primitives exist.
- Typed DAG supports pure dependency execution, stale generation checks and bounded work; scoped graph preserves namespace/provenance boundaries.
- Product-native \`core/programmability\` is separately a pure program validator in Harness and does not import HQ research graph as its runtime foundation.
**Conclusion:** Architectural convergence/packaging and ABI or Rust crate-level dependency acceptance still required; no second graph database should be created.

### 6. UI Labs R007 materially enlarges Node 0's output contract
Source: [UI R007 Total Interpretation](https://github.com/owenservera/vivim-labs/blob/lab/nlp-visual-command-ui-20261111/nlp-command-ui-lab/docs/R007-TOTAL-INTERPRETATION-VIEW-ALL-AND-HQ-HANDOFF.md), UI branch head \`96d97f8da36c376a611bb90f454b30ab376d1260\`.
- R005 reports 28/28 Node tests and 100/100 synthetic semantic probes (Lab only), keyboard/menu density controls, no browser/device visual acceptance.
- R006 documents 100 predictive realtime/two-Tab ideas (proposals, not source-implemented).
- R007 defines 10 semantic layers: L0 original text, L1 language, L2 identity, L3 knowledge, L4 memory, L5 DCB, L6 operations, L7 prediction, L8 epistemic truth and L9 presentation mask.
- R007 requires overlapping node/edge relationships, producer run/skipped evidence, privacy-scoped full trace, not a single command winner.
**Conclusion:** Rust I/O must produce a multi-facet, revisioned, authorization-filtered interpretation trace. TierFlow owns presentation.

### 7. HQ R007 companion audit establishes source producers, not live coverage
Source: [HQ interpretation surface audit](https://github.com/owenservera/repos-hq/blob/research/interpretation-xray-intelligence-surface-20261112/research/semantic-interpretation-surface/HQ-SURFACE-AND-GAP-AUDIT.md), [producer census](https://github.com/owenservera/repos-hq/blob/research/interpretation-xray-intelligence-surface-20261112/research/semantic-interpretation-surface/source-census.v0.json).
- Identifies 43 interpretation facets and a sample source census of 31 producers/observations; not a complete runtime census.
- Explicitly says no single currently observed end-to-end input-span -> typed interpretations -> source/graph/effect/status -> visual trace is wired.
- Tracks source-owned memory, DCB, authority, knowledge, suppressions, not-run/denied producer accounting.
**Conclusion:** Reuse that source census as an **audit input** to a runtime producer manifest, not as a false claim of full active coverage.

## Security research with confirmed upstream reference

### Advisory A — Origin confusion
- Official GitHub: https://github.com/tauri-apps/tauri/security/advisories/GHSA-7gmj-67g7-phm9
- Published May 6, 2026; affected \`tauri\` versions \`>=2.0, <=2.11.0\`, patched \`>=2.11.1\`.
- Incorrect local-origin classification on Windows and Android allowed certain remote origins to be treated as local.
- Tauri 2.11.1 also fixed enforcement of ACL for some remote-origin custom command calls.
- **VIVIM source pin \`=2.0.0\` is in affected range**. Actual runtime exploitability not tested.

### Advisory B — Channel response read isolation
- Official GitHub: https://github.com/tauri-apps/tauri/security/advisories/GHSA-w28w-mhc8-qvjv
- Published September 26, 2026; affected \`tauri\` versions \`>=2.0.0, <=2.11.5\`, patched \`>=2.11.6\`.
- Improper ACL enforcement for internal channel \`fetch\` could expose queued cross-webview data to a renderer with invoke access; exploit preconditions matter. 
- **VIVIM source pin \`=2.0.0\` falls in affected range**, even if R001 uses return values rather than channels. Upgrade the whole native shell before sensitive I/O channels are introduced.

### Capability and editor references
- [Official Tauri capabilities](https://tauri.app/security/capabilities/) — remote API access is opt-in and scopes need care.
- [Official Tauri runtime authority](https://v2.tauri.app/security/runtime-authority/) — origin capability matching before native command invoke.
- [Tokio bounded channels documentation](https://tokio.rs/tokio/tutorial/channels) — unbounded queues are a denial-of-service risk.
- [MDN beforeinput](https://developer.mozilla.org/en-US/docs/Web/API/Element/beforeinput_event) — not every modification fires a cancelable beforeinput, including IME/autocorrect cases.
- [W3C Input Events Level 2](https://www.w3.org/TR/input-events-2/) — composition event ordering and source editing rules.

## No-go assumptions to avoid
- Do not claim current product binary uses every dependency in source until inspecting lockfile and actual Windows package.
- Do not claim \`remote.urls\` wildcard alone proves external IPC exploitability; it is a configuration that requires careful restriction and testing.
- Do not claim HQ/Host source registry \`verified\` means current user capability/permission.
- Do not claim R005 synthetic test results validate real user editing, screen readers or phone.
- Do not claim R001's four threads are a high-performance parallel runtime; the state mutex and join mean synchronous completion barrier.
- Do not claim all 43 HQ facets or 31 source producers execute for a user utterance today.
- Do not claim a new system is required wherever an existing Host/HQ/TierFlow implementation can be upgraded in place.

## Actionable product truth statement

**Current source reality:** product main still has Node-hosted semantic UI and control paths; the R001 Rust kernel is a separate pure experimental entrypoint not wired to real trusted sources; the UI/HQ interpretation catalogs are richer than the native output contract; Tauri dependency source pins predate known security patches.

**Target reality:** patched native first trusted semantic endpoint, bounded concurrent graph computation, one actor- and source-bound event/evidence contract, existing effect journal authority, and one faithful desktop/phone/accessible projection. Acceptance must be demonstrated with actual packaged Windows and PWA runs, not with source inspection.
