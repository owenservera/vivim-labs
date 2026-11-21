# Local agent handoff — execute in order, do not invent verification

## Assignment / constraints

User has paused other ARBOR work to prioritize the Craft-to-ARBOR evidence pipeline. Canonical owners remain unchanged. **Do not create PRs, branches, second graphs, second browser owners, extra write authority, provider API dependencies, Playwright/Puppeteer, or synthetic product data.** Work on `main` with non-force fast-forward conflict resolution and preserve existing changes/stashes. No user sign-in secrets in logs, Git, reports, screenshots or prompts. Reuse Bun when a package runner is needed; bridge itself requires only Node 22+.

**Input repositories:** `owenservera/vivim-labs` (this study/bridge), `owenservera/craft-anything-VIVIM` (whole investigator, unchanged), `owenservera/ARBOR` (live product; do not modify until integration boundaries understood). Refresh all real HEADs before writing. The known local project root was `C:\0-BlackBoxProject-0`, but discover the actual checkout paths instead of assuming it exists.

## Step A — establish truth

1. Open this README, source assessment, architecture, and `src/adapter.mjs`; run `node --test test/*.test.mjs` from the adapter folder. Record versions, exact counts, failures and exit codes. Do not claim a real Craft run based on the current contract tests.
2. Inspect Craft `crates/evidence/src/canonical.rs`, `lib.rs`, and `crates/engine/src/tools/session_tools.rs` on refreshed HEAD. **Close the Rust/JS canonical hash parity gap.** Preferred: generate genuine `core_evidence_export` test vectors including floating-point ratios, mixed Unicode keys, negative zero, nested data and numeric extremes. Match `Evidence::verify` exactly or call a minimal Rust verifier backed by Craft's actual crate. Never loosen hash checking to claim an import passes.
3. Identify the exact working VIVIM MVP URL/origin and existing Studio/Harness browser lease/profile. Craft MUST use its own separate private Chromium session; never attach to VIVIM's signed-in Chrome or port 9333/38702, never copy stored credentials.
4. Confirm legal/policy authorization for each investigation target. Claude.ai can be a documentation-backed reference, but automated authenticated scanning of a third-party service is not assumed authorized. Start full automation against a first-party MVP.

## Step B — run genuine Craft investigation

From the Craft clone (PowerShell):

```powershell
cargo --version
cargo build --release -p craft-anything-cli
.\target\release\craft-anything-cli.exe --clean-room mcp --restricted --allow-web --allow-capture
```

Then from one MCP client connection, discover tool schemas using `tools/list` and perform a real capture of the owned MVP:

- `web_site_open`: explicit URL and `signed_in:false` unless authorized and needed.
- `web_site_capture`: `output_directory` must be a fresh path; capture 1440 and 390 widths, optionally 768; inspect returned evidence ID and exact folder path.
- `web_page_states_capture`: fresh state folder; inspect its evidence ID, labels, changes and coverage limitations. The tool expands menus/tabs and performs site-defined interactions, so approval and target scope matter.
- `core_evidence_export`: absolute new JSON path, `overwrite:false`. Preserve original exported bytes, records, and unknowns. Do not edit source records to hide data: editing invalidates IDs. Protect raw files outside version control.

Only use `web_walkthrough_record` if target ownership and explicit consent for active interaction are confirmed; it is not a clean-room tool. Do not use Build/Rebuild to generate a replacement UI: Study/capture is the goal.

## Step C — normalize and verify (Labs code)

```powershell
node --test test/*.test.mjs
node src/cli.mjs import --bundle 'C:\captures\study-01.json' --site vivim-mvp --origin 'http://127.0.0.1:4173' --capture-dir 'C:\captures\site-01' --states-dir 'C:\captures\states-01' --out 'C:\captures\report-01'
```

Replace paths/origin with the actual values from Craft's evidence (not guessed). For app-owned visible control labels, add `--owned-target --include-labels` only after verifying this is an isolated, nonprivate target. Inspect `atlas.json` and `coverage.md` manually for leakage. Keep reports outside public repos; this Labs source code contains no real captures.

Capture exact counts: original records, validated records, scoped browser records, captures, distinct controls, states, unknown IDs, errors and discarded/unscoped evidence. Rerun with the same input into a fresh output directory and compare digests; same data must yield semantically identical output.

## Step D — read-only ARBOR bridge (separate change, after genuine capture)

1. Inspect refreshed ARBOR `studio/src/studio/adapter-evidence-intake.mjs`, `adapter-factory-service.mjs`, `adaptive-adapter-factory.mjs`, `studio/src/profiles/sites/claude.mjs`, `harness/src/providers/claude.mjs`, `hq/lib/kg-export.mjs`, native HQKG journal and Host effect owner before any edit.
2. Introduce one source-neutral, **read-only external observation port** with verified evidence provenance. Do not forge Studio's Core capture verifier or `SAVED_REAL_CAPTURE` classification. The current `adapter-evidence-intake.mjs` only handles its existing chat features: respect that enum.
3. Bind an admitted observation to existing semantic identities; show the data in Studio using current read/projection patterns. Implement only observation display and diff initially. No remote provider effect and no automatic adapter activation.
4. Reconcile one known profile difference: Studio Claude hydration avoids early `textarea` selector, whereas Harness has a fallback. Mark this as source discrepancy pending runtime verification—not an automatic code fix.
5. Only after independently held-out checks, propose one read-only profile feature through Studio's existing factory. To enable an effect later, use Host's existing approval, account binding and receipt, not this pipeline.
6. Run actual tests and record PASS/FAIL/UNRUN. Preserve other lane changes and commit directly to `main` only after real integration behavior is observed.

## Minimum next-turn deliverables

A. Genuine Craft bundle with exact engine version and native evidence verification receipts (private; never Git); B. imported `atlas.json` with verified source IDs and captured controls; C. source-level Studio/Harness gap matrix for the same app; D. one read-only Studio visualization or query of the atlas with source citations; E. exact commands, test counts, blockers, and HEAD SHAs. If any item cannot be completed, mark UNRUN and explain why.

## Copy/paste local-agent directive

> Continue the Craft Anything → VIVIM/ARBOR intelligence adapter from `vivim-labs/craft-anything-arbor-adapter/`. First run the seven current Node contract tests, cross-check Craft's Rust evidence digest implementation on genuine exported records (especially float/number cases), and capture our authorized MVP in an isolated Craft MCP browser. Import its genuine bundle + capture.json + states.json via the Labs CLI, generating private atlas.json and coverage.md, then reconcile observed UI states with current Studio/Harness profiles. Add only the smallest read-only external-evidence intake and Atlas view to ARBOR; do NOT modify Host authority, invent Studio capture verification, auto-enable selectors, create new graph stores, or disturb Rust convergence. Report commands and exact PASS/FAIL/UNRUN, source SHAs, privacy decisions, real coverage gaps and the next smallest viable end-to-end change. All repository changes on main, no branches/PRs, preserve other edits and stashes.