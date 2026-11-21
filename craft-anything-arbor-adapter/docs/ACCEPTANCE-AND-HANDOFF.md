# Acceptance, status ledger, and next round

## Current status (2026-11-21)

| Work item | Status | Evidence |
|---|---|---|
| Inspect Craft workspace/extensions/browser tools | PASS (source inspection) | Rust source and docs in named clone; see SOURCE-ASSESSMENT |
| Inspect Craft bundle v1 and evidence identity rules | PASS (source inspection) | `session_tools.rs`, `evidence/src/lib.rs`, `canonical.rs` |
| Inspect ARBOR Studio learned-adapter and Host boundaries | PASS (source inspection) | `adapter-evidence-intake.mjs`, `adapter-factory-service.mjs`, `product_kernel.rs` |
| Labs standalone adapter implementation | PASS (source created) | `src/adapter.mjs`, `src/cli.mjs` |
| Labs Node contract tests | PASS — 7/7 locally | `node --test test/*.test.mjs` on Node v22.16.0 |
| Real Craft bundle import | UNRUN | No genuine exported bundle available in this environment |
| Rust/JS canonical hash parity | UNRUN | Requires native Craft verifier + genuine vectors |
| VIVIM MVP browser capture | UNRUN | No browser/CDP ownership or local MVP runtime in this session |
| Claude.ai runtime surface exploration | UNRUN | No authorized live investigation performed |
| ARBOR Studio read-only integration | UNRUN | No ARBOR product source modified |
| ARBOR Rust integration test | UNRUN | No local ARBOR Rust workspace runtime in this session |
| Craft → Rust harvesting | DEFERRED | Not priority; evidence bridge first |

**Meaning of PASS:** source inspection means source was actually read. Code tests mean only tests shown were actually run. Neither is a runtime product certification. No test evidence invented.

## Acceptance for Round 2 (no new gates for unrelated Rust work)

1. True Craft Rust evidence ID agrees with the Labs adapter on at least one real `web_site_capture` and `web_page_states_capture` bundle, including non-integer numbers or an explicit refusal and remediation. Preserve exact input/output digests.
2. On a first-party MVP, create two unique capture folders, observe 1440/768/390 layout where available, record >=1 actual state change if present, and export a real Craft bundle; if no state controls exist, report as observed zero for that bounded run.
3. Import that bundle and source-artifact paths using existing CLI. Check strict origin, expected counts, independent capture-file digest, unknown IDs, read-only authority and privacy sanitization.
4. Re-run with same bundle and compare normalized outputs. Differences must be explained by actually changed source bytes; no time-based or random IDs.
5. Source-reconcile mapped chat features and broader application surfaces. Don't turn unknown into absent or source presence into live verification.
6. Add the **smallest** ARBOR read-only ingestion path with Core-owned evidence qualification, no false Studio receipt, no effect action. Verify from one actual Studio readback.
7. Host effects, provider send, project creation, graph mutations via a second store, and live external subscriptions remain out-of-scope.

## Main technical blockers / risks

- **Critical:** JS `JSON.parse` loses Rust number lexical type information (`1` vs `1.0`); evidence ID recomputation can differ. Better use a native Craft-crate verifier, or prove a lossless canonical-number bridge before accepting arbitrary output.
- **High:** Real Craft capture folders live outside bundle and have no signed artifact manifest. Need digest/audit readback and retention rules before cross-device use.
- **High:** Full capture may include sensitive page text; private source stays local, short labels are opt-in only for first-party capture.
- **High:** Current Studio evidence intake requires a Core-verified capture; Craft evidence does not inherit that status.
- **Medium:** Broad surface capability vocabulary is not yet in Studio learned-adapter factory. It should remain read-only and separate from provider effect adapters.
- **Medium:** Full webapp coverage cannot be inferred from one page, viewport, or a `complete` tool-local result.
- **Medium:** HTML or screenshot capture is not equivalent to knowing business effects and error recovery.

## Next Rust migration target (later, conditional)

Only after genuine export parity: move the deterministic `Craft evidence → VIVIM surface observation` normalization into Rust and reuse the existing product kernel or a purpose-specific pure module. Keep IO, filesystem, browser/OS effects, approvals and persistence outside that reducer. Do not move Craft's browser session management or website-rebuilding workflow into the product merely for symmetry.

## Continuation anchor

The next agent/ChatGPT turn should open this folder and start with **real bundle/native-hash parity**, not another broad conceptual survey. Take source paths and exact state from the named files; refresh all HEADs, make no claims about test outcomes not executed. Keep Labs folder as research truth; ARBOR source changes are separate implementation work with their own actual runtime evidence.