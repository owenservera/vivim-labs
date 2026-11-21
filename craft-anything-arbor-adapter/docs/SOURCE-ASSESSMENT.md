# Source assessment: Craft Anything, Studio, Harness, HQKG

## Primary sources inspected

| System | Actual source | Finding |
|---|---|---|
| Craft | `crates/engine/src/tools/session_tools.rs` | `core_evidence_export` writes `{format:'craft-anything-evidence-bundle',version:1,records,unknowns}` to an absolute path using atomic file output. Craft's import verifies evidence IDs. |
| Craft | `crates/evidence/src/lib.rs` | Evidence comprises `id`, `subject`, `operation`, `params`, `result`, `summary`, `confidence`, `authority`, `environment`, `limitations`, `coverage`, `locations`, `links`, `carries_target_content`. The evidence ID excludes `summary`, `subject.name` and `carries_target_content`. |
| Craft | `crates/evidence/src/canonical.rs` | Object keys are sorted by UTF-16 units. Rust `serde_json::Number::to_string` is used for numeric values; string output uses minimal JSON escapes; depth bounded at 512. A JS native JSON parser cannot always preserve the distinction between `1` and `1.0`. |
| Craft | `crates/engine/src/tools/website_tools.rs` | `web_site_open` owns an isolated browser; `web_site_capture` emits an evidence record plus `capture.json`, README, screenshots; `web_page_states_capture` produces an evidence record plus `states.json`/PNG. Exact paths are present in the evidence result. |
| Craft | `crates/engine/src/tools/browser_tools.rs` | `web_walkthrough_record` is active, consent-gated CDP (goto/click/fill/press/select/wait) with step-level recordings; `web_page_show` is passive but Deep-only under Craft's current clean-room allowlist. |
| Craft | `crates/engine/src/clean_room.rs` | Clean room permits visitor-view page capture, state capture and export, but not full JavaScript/source inspection or scripted `web_walkthrough_record`. This is not a provider-specific Terms-of-Service grant. |
| Craft | `crates/engine/src/tools/mod.rs`, `docs/extending.md` | Each tool has `ToolSpec`, schema, effects and registry; external MCP plugins and optional WASM components are supported. External JSON export is the simplest current seam; no fork required. |
| ARBOR | `studio/src/studio/adapter-evidence-intake.mjs` | Existing evidence intake requires Core-verified captured sources; accepts chat-centric features, validates scope/account/producer/digest and rejects false 'absent' when coverage isn't complete. |
| ARBOR | `studio/src/studio/adapter-factory-service.mjs` | Existing learned-adapter factory only promotes server-reverified, **read-only preview** candidates to versioned store; learned selectors do not enter send path. |
| ARBOR | `studio/src/studio/adaptive-adapter-factory.mjs` | Current feature vocabulary: composer, send-control, user-turn, assistant-turn, stop-control, thread-route, citation, artifact, task-progress, stream-component. More general app features require a new read-only atlas projection. |
| ARBOR | `studio/src/profiles/sites/claude.mjs` and `harness/src/providers/claude.mjs` | Both support Claude chat interactions, but profile details differ: Harness retains generic `textarea` fallback Studio deliberately dropped after a hydration race. This is a candidate reconciliation finding, **not a measured runtime regression**. |
| ARBOR | `core/programmability/src/product_kernel.rs` | Rust product kernel dispatches pure functions; it has no provider browser or effect authority. Later source-stable normalization can be migrated here. |
| ARBOR | `native/hqkg/src/bin/hqkg-web-feed.rs` / `src/web_memory.rs` | Existing feed accepts metadata-only visit records, not full arbitrary UI evidence. Claude private-route restrictions and navigation-only payload must not be weakened for Craft. |
| ARBOR | `hq/lib/kg-export.mjs` | Existing HQKGX1 exporter is explicitly owned by HQ source index and carries source metadata only; arbitrary Craft records should not be forged into its producer namespace. |
| Labs | `Architecture/07-INTEGRATION-PHILOSOPHY.md` | Each bridge must deliver product-relevant evidence and reduce cost of successor integration. Dev tooling and product capabilities remain separated; Host alone owns world effects. |

All paths above are within the following repositories:

- https://github.com/owenservera/craft-anything-VIVIM
- https://github.com/owenservera/ARBOR
- https://github.com/owenservera/vivim-labs

## Tool capability classes

**Passive/visible:** `web_site_open`, `web_site_capture`, `web_page_states_capture`, `web_page_screenshot_take`, `web_pixels_compare`. Note that opening a site runs its scripts, and state capture triggers UI expansion—it is not a harmless network-only operation.

**Deep/passive:** `web_page_show`, `web_page_js_analyze`, `web_page_watch`, `web_scripts_save`, `web_modules_trace`, `web_listeners_list`; availability and permissions depend on Craft's clean-room and consent policy.

**Active:** `web_walkthrough_record` deliberately interacts with the page and is not in the clean-room allowlist. Only perform against a site we own or have specific permission to automate.

**Transfer:** `core_evidence_export` and `core_evidence_browse`; the exported JSON bundle is version 1. The capture folder files are *not* embedded in that bundle, so they require separately grounded provenance and are not considered Craft-content-attested by the importer.

## Why not a Craft plugin yet?

1. JSON bundle export exists, is versioned, and decouples browser ownership from ARBOR. A plugin adds runtime/process coupling without improving the first learning loop.
2. External MCP server plugins in Craft are *ingress* for third-party tools, not a built-in one-way ARBOR evidence export system. Bidirectional calls risk duplicating authority.
3. WASM component plugins have no filesystem, network or environment imports. They cannot directly stream evidence into ARBOR.
4. Native Craft `ToolSpec` changes would require a fork and conformance tests during another Rust convergence campaign. Defer.

## Genuine uncertainties

- Native Rust → JS evidence hash parity across noncanonical floats, minus zero, exponent formatting, Unicode string edge cases and pathological nesting is **UNRUN**. The JS importer validates typical bundles and fails closed; do not treat passing same-language tests as cross-language certification.
- Source snapshots were inspected, but no local Craft binary, authenticated Claude browser or real VIVIM capture was available in this authoring environment.
- `capture.json` outline shape can evolve and may contain sensitive page text. Current importer extracts bounded structural controls and hides names by default.
- No runtime Studio promotion, full capability atlas view or HQKG bridge was implemented in ARBOR this round.
- License: Craft workspace declares MIT OR Apache-2.0; verify actual dependency licenses and preserve notices if code is later copied. None was copied here.

## Security and scope note

Claude.ai is an example capability target; it does not authorize an automated crawl or reverse engineering of a service. Use published product material or permissioned/manual observations for third-party reference research; automated end-to-end interaction tests should first target VIVIM-owned webapps. Never use the existing Harness/Studio signed-in Chrome profile for Craft: preserve separate browser ownership and credentials.