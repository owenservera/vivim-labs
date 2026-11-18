# ARBOR Workflow Fleet — Brief for ChatGPT

Prepared 2026-11-17 for the ChatGPT strategy channel (owner-gated; entry via oracle → chatgpt-keeper).
Repo: `owenservera/ARBOR` · Branch: `main` · HEAD at writing: `82d645d`.
Canonical fleet doc: [docs/WORKFLOWS.md](C:/0-BlackBoxProject-0/ARBOR/docs/WORKFLOWS.md). Defs: `.zcode/workflows/*.dwf.ts` (ZCode dynamic-workflow TypeScript, saved via SaveWorkflow, launched via CreateWorkflow; the `.dwf.ts` files are git-ignored by design).

## 1. How the fleet is wired

- **Runtime:** ZCode dynamic workflows. Each def is one TypeScript script with `phase()` markers (user-visible graph), `agent()` subagents (fresh context per role), `world.run()` deterministic commands (gates, curl, bus sends), and typed results. Runs are journaled (`dwf_event` tables) — resume replays finished steps at zero cost; amend supersedes a run and imports its finished work as cache.
- **One Host, one journal, one queue:** everything runs against the live loopback Host `http://127.0.0.1:43187` (apps/vivim-host/server.mjs). Product journal = `dataRoot/product-state/journal.jsonl` (POST `/demo/feedback` appends typed rounds; `poll-round.mjs` reads). Work queue = `docs/product/roadmap/roadmap.v1.json` `buildQueue`. Receipts = `docs/product/roadmap/receipts/<ID>.receipt.json` (schema `arbor.item-receipt/1`).
- **One-writer discipline:** the roadmap JSON has exactly one writer — the keeper flip (batch-engine FLIP_CODE). LIFE never flips the queue; it hands off via the repos_hq bus (`sync.mjs`, kinds claim/release/handoff/status). Per-item leases ride `--kind claim`.
- **Gates are deterministic:** `bun scripts/test-gate.mjs` (change-set manifest, ~300 test files, node24/bun runtimes) is the fast gate; per-item evidence `bun scripts/batch-evidence/<ID>.mjs` (static + `--live` isolated scratch Host); route re-walk = all-200 over 9 product routes.
- **Every run is honest by construction:** gate exit codes decide, not self-claims; a "verified" receipt requires gate 0 + evidence static+live 0 + a fresh-eyes journey replay; NIM subagents cannot read images, so visual checks are programmatic (DOM snapshots via native control-browser, ARIA text) or curl parity.

## 2. The defs — goal, mode, models

| Def | Goal | Key args | Models |
|---|---|---|---|
| `arbor-life` | **Autonomous continuous development** — the default loop: USE→OBSERVE→DIAGNOSE→FIX→VERIFY→EVOLVE. Feeds the next eligible queue item (build or verify-kind), runs a real user journey (browser via native control-browser tools in-subagent, curl fallback), fixes confirmed defects scoped, gates, writes receipts, hands off to the keeper | `mode`=life(default)/audit, `maxRounds`=6, `item` | LEAD: `openrouter-2/baidu/ernie-5.5-flash-preview:free` (ladder: → NIM mistral → inclusion → step-5). WORKHORSE: `new-provider/mistralai/mistral-large-4-1t-a52b` (→ step-5 → inclusion → ernie → minimax last). Health-probed from `model_usage` (≥5 error rows/10min or ≥3 rate-limit rows/10min = skipped). GLM-5.3 retired 2026-11-17 (zai quota 1005) |
| `arbor-build` | Queue consumer: builds one actionable entry with an 8-slot fleet (brief→build→gate→flip) | `item`, `mode` | NIM ladder: super-vl workhorse, ultra-550b lead, lightning/bolt/nano alts |
| `arbor-batch-engine` | Lane-autonomous batch runner: `mode=build` runs a batch; `mode=verify` independently verifies (receipts + one-writer flip); owner batch-exit hole is the only gate | `batchLabel`, `items`, `mode` | Same NIM ladder block |
| `arbor-demo` | Owner-facing walkthrough on the live Host: walks the queue through `/demo`, collects owner verdicts from the journal, stamps approved entries | `mode`=queue/baseline | NIM 8-slot fleet |
| `arbor-roadmap` | Re-baseline (PAUSED): proposal5 corpus + HQ index + gap sweep → roadmap.v1.json. Never run casually — regenerating the roadmap is forbidden while the queue is truthful | `emphasis` | NIM 8-slot fleet |
| `arbor-chatgpt-bridge` | Strategic ChatGPT input via studio.mjs single transport; owner-gated, entry via oracle only | — | NIM fleet |
| `arbor-batch1` | HISTORICAL — batch-1 baseline (dwfrun-e163906c, owner-approved). Never launch | — | — |

## 3. Current state (2026-11-17)

- LIFE lineage `dwfrun-61ab3096 → … → dwfrun-21fae78b` (live): **A00, A01, A02 all verified green** (each: gate 0, evidence static+live 0, fresh-eyes re-test pass, re-walk 9/9, receipt + journal round + keeper handoff). Pushed as `03252dc`.
- Queue: A00-A02 receipts verified (roadmap `verification` field drift UNRUN→keeper flip owed), A03-A08 gated/queued per dependsOn. LIFE continues feeding while this brief circulates.
- Measured model health (last 2h, model_usage): ernie-5.5 181 calls avg 4.6s (healthy lead); NIM mistral 171 calls avg 37.6s (healthy but slow — main pacing cost); minimax/step-5/inclusion untested; GLM dead.
- Known inefficiencies: NIM generation-bound rounds (20-40 min/item vs 5-min target), provider rate-limit stalls, ~3-5 amends per lineage. Open questions for ChatGPT are queued at `.coordination/pdr/chatgpt-questions.md`.

## 4. Guardrails

main only, no PRs; isolated loopback demo data only; never edit server.mjs routes or roadmap.v1.json from LIFE; receipts/queue flips are keeper-owned; owner approvals are owner-only surfaces; ChatGPT is strategic input only, owner-gated.