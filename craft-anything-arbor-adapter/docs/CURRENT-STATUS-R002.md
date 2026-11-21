# R002 status ledger — 2026-11-21

| Work | Status | Exact evidence |
|---|---|---|
| Labs R001 importer 7 tests | PASS | `node --test test/*.test.mjs` |
| R002 12 added contract tests | PASS | Same command: **19 PASS / 0 FAIL** |
| HTML tag normalization, PII filtering | PASS | `src/adapter.mjs`, `test/round2.test.mjs` |
| Capture count/design/P​​NG SHA verification | PASS (unit) | `test/round2.test.mjs` |
| Embedded Craft states vs `states.json` compare | PASS (unit) | `test/round2.test.mjs` |
| Atlas source baseline reconciliation | PASS (unit + CLI) | `src/reconcile.mjs`; `test/round2.test.mjs` |
| Atlas read-only version comparison | PASS (unit + CLI) | `src/diff.mjs`; `test/round2.test.mjs` |
| Native verification receipt protocol | PASS (unit only) | `src/native-verifier.mjs`; `test/native-and-schema.test.mjs` |
| Four JSON schemas | PASS (local Python Draft 2020-12 validation) | `schemas/*.schema.json` |
| Claude source-profile baseline | PASS (source inspection, schema validation) | `baselines/claude-source-baseline.json` (source only) |
| Compile `tools/vivim_verify_bundle.rs` against Craft crate | UNRUN | `cargo` unavailable here |
| Native Rust/JavaScript ID roundtrip on actual Craft bundle | UNRUN | No genuine bundle or toolchain present |
| Craft browser session on actual VIVIM MVP | UNRUN | No local first-party browser/runtime available here |
| Claude authenticated live browser scanning | UNRUN / NOT AUTHORIZED | Not attempted |
| ARBOR Studio/HQKG read-only integration | UNRUN | No ARBOR source changes this round |

**No claim** of working full Craft-to-live-ARBOR integration has been made. This round builds a developer bridge and acceptance instruments for the local team to run on genuine sources.
