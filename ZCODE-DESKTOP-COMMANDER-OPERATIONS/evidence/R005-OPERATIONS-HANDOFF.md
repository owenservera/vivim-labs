# R005 handoff: model route ownership to separate ChatGPT session

Date: 2026-11-21. Recorded by ChatGPT via authorized Desktop Commander at DESKTOP-9KB03LU. This file is a factual handoff and supersedes informal model routing guesses.

## Separation of ownership
**Another ChatGPT session now owns all further model/provider engineering.** This operations session shall NOT alter model/provider settings, selected model policy, model-routing scripts or their tests unless asked explicitly. Continue ZCode / Desktop Commander / ARBOR operations through already verified runtime surfaces and no-cost control-plane probes. Concurrent directory `MODEL-SWITCHING/` exists; preserve it entirely without staging or modifying it.

## Saved implementation inventory
- Canonical per-user model selection intent: `MODEL-SELECTION-POLICY.json`; six prioritized exact pairs from Start Plan GLM Flash, OpenRouter ERNIE, NIM Mistral, MiniMax M Plan M3/M3.1 Flash and NIM Nemotron Super VL.
- Rust discovery: `native/zcode-observer/src/bin/model_inventory.rs`, `cargo` crate's `regex` and `serde_json` dependencies, associated `Cargo.lock`. Reads personal `~/.zcode/v2/provider_config.json` and latest cached builtin `zcode-builtin.json`, uses regex caching, projects only safe provider/model/option fields. Takes optional `--selected`, `ZCODE_MODEL_SELECTION_PATH`, `ZCODE_PROVIDER_CONFIG_PATH`, `ZCODE_BUILTIN_CONFIG_PATH`. Strictly avoid raw secret material.
- One-command wrapper: `scripts/inventory-models-r005.ps1`. Most recent execution: PASS all=342 unique configured candidates; selected=6; 10.45s first successful full run, 17.01s final integrated fmt/test/clippy/inventory rerun. The list is the personal configuration and built-in matching rules, NOT effective runtime registry nor live availability.
- Old diagnostic attempt: `scripts/ping-priority-models-r004.ts`. All six requests lacked reasoning options and failed with ZCode errors. Evidence `evidence/R004-MODEL-PINGS.json` intentionally local-only.
- Improved selection-driven ping: `scripts/ping-filtered-models-r005.ts` consumes `evidence/R005-SELECTED-MODEL-INVENTORY.json`; six explicit model requests maxOutputTokens=24, candidate reasoning options. Evidence `evidence/R005-MODEL-PINGS.json` is also local-only.
- Source references: `evidence/R005-SOURCE-NOTES.md` with Context7 pulls, local layout, candidates, timings, tests and open questions. `.gitignore` keeps generated runtime model/provider inventories and ping telemetry outside publication.

## Actual outcomes and known limitations
Source lookup: Context7 `/zai-org/zcode`, official provider-config codec and model-config optionSpecs. Personal overlay + Account Built-in + ZCode Built-in gives effective provider registry, so the personal config alone cannot prove credential or account entitlement.
The final Rust inventory test run: `cargo fmt --all` exit 0, `cargo test --offline --bin model_inventory` 3 passed / 0 failed, `cargo clippy --offline --bin model_inventory -- -D warnings` exit 0; wrapper `MODEL_INVENTORY_R005_COMPLETE`, overall exit 0. Key regression prevented: recompiling every builtin regex against 342 entries caused very slow discovery; regex cache reduced run to practical time.
Selected pings: exactly ONE strict `PING_OK` PASS, on `minimax-m-plan/MiniMax-M3` (28,596ms, 190 tokens). NIM Mistral and NIM Nemotron returned matched model IDs and token usage but empty visible output (24-token cap consumed largely/entirely by reasoning); this is PARTIAL transport/model-usage evidence, not successful ping. Start Plan GLM provider missing from effective workspace registry; M3.1 Flash still requires a different effective reasoning option; ERNIE produced opaque -32603. No silent fallback or credential modifications. Six sends were queued in one app-server invocation, but observed serial response progression (~14.6s to ~71.5s) does NOT establish backend parallel execution. Do not promise six independent concurrent worker contexts based on this.

## Model-owner recommended next actions (not for operations lane)
Inspect account provider availability and provider-site model options in the effective registry; treat modelOrder as display order, not membership. Re-evaluate requested 'GLM 5.3 Flash Start Plan' route; adjust NIM ping max tokens or reasoning setting with the relevant model owner. Never consume live API quota automatically from status reporting. Preserve identities, credentials and other sessions.

## Operations lane carry-forward
1. Verify ZCode controller native read-only protocol and V4 revision/cursor snapshots without touching global model selection.
2. Preserve and inspect staged continuous improvement fleet topology and status; **not** equivalent to live workflow/agents.
3. Validate safe Host product regression surfaces and Rust observer with actual commands; log all outcomes. Avoid duplicate model tasks.
4. Preserve ARBOR dirty files, any stashes and all worktrees; any app edits must honor ongoing lane ownership.

No ARBOR source changes are part of this handoff.
