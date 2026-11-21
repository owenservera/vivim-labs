# Craft Anything → VIVIM / ARBOR Intelligence Adapter

**Status:** DEVELOPMENT-ONLY WORKING IMPORTER + SOURCE-GROUNDED RESEARCH; NOT an ARBOR production integration. **Date:** 2026-11-21.

This project deliberately lives in **vivim-labs**, not in ARBOR's product kernel and not in the Craft Anything clone. It converts *already exported* Craft Anything investigation evidence into a **sanitized, source-linked, read-only application-surface atlas**. It **does not** operate a browser, approve actions, install plugins, promote learned adapters, send prompts, or write to HQKG/Host.

## Purpose

A single investigation should improve the next one: discover permitted webapp interfaces, record supported observable states and structured unknowns, compare findings with existing Studio/Harness coverage, and eventually feed ARBOR's existing adapter-learning and semantic graph systems. Start with a first-party VIVIM MVP. Claude.ai is an important **capability-reference target**, not a blanket authorization for automated third-party extraction; follow the applicable terms and source permissions.

## What is working in this folder

- `src/adapter.mjs`: versioned Craft bundle v1 parser; deterministic content-ID check; idempotent duplicate consolidation; strict target-origin scoping; privacy-minimized structural observations; optional `capture.json` and `states.json` reading (only with matching Craft-recorded output folder); inert atlas and coverage report.
- `src/cli.mjs`: no-dependency Node 22+ CLI; refuses overwrite and never connects to any provider.
- `test/adapter.test.mjs`: seven Node test cases, including a real CLI process invocation and negative cases. Test inputs are **contract-unit inputs**, not claims that any provider UI was measured.
- `docs/`: source inspection, pipeline and ABI, deployment, risk and handoff instructions.

**Important:** These tests have not run the Craft Rust source. Before accepting actual Craft bundles, the local agent must cross-validate canonical SHA-256 identity against the native `craft_anything_evidence::Evidence::verify()` implementation, especially floating-point values and JSON edge cases. The importer currently fails closed on mismatches. This is a known interop limitation, not a passed parity check.

## Quick start (no provider access, no browser ownership)

From this folder:

```powershell
node --version
node --test test/*.test.mjs
node src/cli.mjs import --bundle 'C:\path\to\craft-evidence.json' --site vivim-mvp --origin 'http://127.0.0.1:4173' --out 'C:\path\to\new-report-directory'
```

The `--out` directory **must not already exist**. It writes `atlas.json` and `coverage.md` there. Nothing is written to the Craft bundle or to ARBOR. The target origin is provided explicitly by the operator; source evidence from a conflicting origin is refused.

Optional artifact-aware import (the file folders must be the precise `result.output_directory` / `result.folder` referenced by a verified Craft record):

```powershell
node src/cli.mjs import --bundle 'C:\captures\evidence.json' --site vivim-mvp --origin 'http://127.0.0.1:4173' --capture-dir 'C:\captures\site' --states-dir 'C:\captures\menus' --out 'C:\captures\report-01'
```

To include short, filtered UI labels for a **first-party application**, append both `--owned-target --include-labels`. This is an operator declaration, NOT an access-control grant. The default preserves structural roles but omits labels; it never includes conversation bodies, request payloads, cookies, token values, raw screenshots or DOM in exported atlas files. Manual inspection is required before distributing even sanitized reports.

## Craft-side collection sequence

On a separately owned investigation browser (never Studio/Harness's Chrome profile/port), from the existing Craft clone:

```powershell
cargo build --release -p craft-anything-cli
.\target\release\craft-anything-cli.exe --clean-room mcp --restricted --allow-web --allow-capture
```

Keep the MCP server alive during the investigation. In the connected MCP client, run `web_site_open` for the permitted MVP URL, `web_site_capture` to a *new* capture directory, `web_page_states_capture` to another *new* directory, `core_evidence_browse` as needed, then `core_evidence_export` to an absolute file path with `overwrite: false`. Each separate one-shot CLI invocation uses a separate Craft session and does not preserve the opened browser. Obtain each tool's input schema from MCP `tools/list`; do not assume free-form CLI arguments. If Rust compilation or browser access fails, record it as UNRUN, not simulated success.

Read `docs/LOCAL-AGENT-RUNBOOK.md` before any ARBOR changes. For a real Craft bundle that contains mixed targets, split or curate an authorized per-origin investigation **with its original Craft IDs intact**, rather than editing records (which breaks their hashes).

## System boundaries

```
[Craft Anything] --v1 evidence export--> [Labs adapter CLI] --> atlas.json + coverage.md
                                                   |
                                          (reviewed source contract)
                                                   v
    [ARBOR Studio existing adapter-evidence-intake & factory, read only]
                                                   |
                                    [existing HQKG / semantic graph]
                                                   |
                  [existing Host plans + approval + receipt; never this tool]
```

This is a development accelerator, not a second browser actor, database, graph writer, capability registry, or world/effect authority. Product startup must never depend on this project.

## What is deliberately not implemented

- Direct Craft-to-Studio ingestion/activation: requires actual accepted source capture verification and a separate ARBOR integration change.
- Browser scraping or scripted exploration of third-party authenticated services.
- Studio/Harness runtime feature promotion, selector rewriting, or Host effect calls.
- Live Craft exported-bundle parity (no genuine run provided during authoring).
- Full design-token, file-payload, screenshot or private-conversation ingestion. Those remain private at source and need separate grants and retention policy.

## Documents

- [`docs/SOURCE-ASSESSMENT.md`](docs/SOURCE-ASSESSMENT.md): source-level findings and exact contracts.
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md): semantic observation and data ownership pipeline.
- [`docs/LOCAL-AGENT-RUNBOOK.md`](docs/LOCAL-AGENT-RUNBOOK.md): actionable execution instructions.
- [`docs/CRAFT-MCP-FIRST-RUN.md`](docs/CRAFT-MCP-FIRST-RUN.md): actual Craft tool names and JSON payloads from source.
- [`docs/CLAUDE-SURFACE-BASELINE.md`](docs/CLAUDE-SURFACE-BASELINE.md): Claude source-level coverage matrix, without fabricated runtime claims.
- [`schemas/application-surface-atlas.schema.json`](schemas/application-surface-atlas.schema.json): draft 2020-12 output contract.
- [`docs/ACCEPTANCE-AND-HANDOFF.md`](docs/ACCEPTANCE-AND-HANDOFF.md): stage evidence, missing work, and continuation instructions.

**Source revisions inspected at planning time**: Craft clone `e64d7cb73d7f39323b7040a984e0f9b3f7981146`, ARBOR `98890827832ea74eba698566cb88bc3a4a7d520c`, Labs `3235146164073a0b487363fac02aa8d569d7cf71` (latest retrieved commit references; local agent must refresh actual branch HEAD). No ARBOR or Craft repository source was changed by this Labs project.