# ARBOR Workflow Fleet — Guide, Expectations, Objectives, Status

*Single tracked reference for the dynamic-workflow fleet. Written 2026-11-16 by the manager thread after the owner-directed checkin/commit. The def files themselves live in [.zcode/workflows/](C:/0-BlackBoxProject-0/ARBOR/.zcode/workflows/) which is git-ignored (session artifacts) — this document is the committed, versioned explanation. Run drafts live in `.zcode/workflow-drafts/`.*

**Machine queue truth:** [docs/product/roadmap/roadmap.v1.json](C:/0-BlackBoxProject-0/ARBOR/docs/product/roadmap/roadmap.v1.json) (sole writer: keeper thread via the one-writer flip script). [ROADMAP.md](C:/0-BlackBoxProject-0/ARBOR/docs/product/roadmap/ROADMAP.md) is a projection and may lag — the JSON wins on any disagreement.

---

## 1. Objectives — what the fleet is for

The product is a 3-page PWA on the ONE production Host (`apps/vivim-host/server.mjs`), served at **real product routes** (owner ruling P7-A, 2026-11-14): `/chat` (NLCL-first chat box), `/wiki` (realtime wiki over roadmap.v1.json + journal), `/demo` (cockpit; machine artifacts only). AI-use-native day 1: the same endpoints serve human and machine; programmatic control, no screenshots for routine work.

The fleet runs the **self-driving acceleration loop**:

```
arbor-roadmap ──> roadmap.v1.json + ROADMAP.md (re-baseline from HEAD)
      │
      ▼
arbor-demo ──────> walkthrough on the live Host, owner verdicts via demo journal
      │            (stamps approved queue entries)
      ▼
arbor-build ─────> consumes next actionable queue item, 8-slot fleet, gate per change
      │
      ▼
arbor-batch-engine (mode=verify) ──> independent second-implementation verification,
      │                                   evidence receipts, one-writer queue flip
      └────────────> next batch (loop)
```

Standing owner principles:

- **Self-reducing effort** — every batch must measurably reduce owner+team effort. Batch-1 cost the owner exactly 1 decision (batch-exit gate). A batch that does not reduce effort is wrong by design.
- **FULL-AUTO standing** (owner, 2026-11-14, while owner is asleep): the oracle auto-approves batch exit gates on the evidence standard; escalate only on both-provider outage, a dishonest gate, or an integrity violation.
- **Roadmap gate canon** — nothing is built without roadmap registration + approval; proposals only, no parallel build outside the queue.
- Priority order (owner 2026-11-14): demo >> build >> new queue batch >> demo > build. Windows install is an ordinary workstream, never a gate.

## 2. The workflows

| # | Workflow | Role in loop | Key args | When to use |
|---|----------|--------------|----------|-------------|
| 1 | `arbor-roadmap` | Re-baseline: proposal5 corpus + HQ index + gap sweep → roadmap.v1.json + ROADMAP.md from the live tree at HEAD, then park for owner prioritization | `emphasis` (json, optional) | Roadmap must be re-synthesized from current HEAD (never from a superseded run's recovery dir) |
| 2 | `arbor-demo` | Queue walkthrough on the live Host; collects owner verdicts from the shared demo journal (`poll-round --since/--last`); stamps approved entries | `mode` = `queue` (default) / `baseline` | A batch needs owner-facing walkthrough evidence before build consumes it |
| 3 | `arbor-build` | Queue consumer: builds the next actionable entry (dependsOn order) with an 8-slot fleet; change-set gate per item | `item` (one itemId), `mode` | After demo approved the queue; run in the continuous loop |
| 4 | `arbor-batch-engine` | Lane-autonomous batch runner: `mode=build` runs an approved batch; `mode=verify` independently verifies (receipts + one-writer flip only). Owner batch-exit hole is the only gate | `batchLabel`, `items` (json), `mode` | Batch 2+; verify mode when independent evidence is owed against the live Host |
| 5 | `arbor-chatgpt-bridge` | Strategic ChatGPT input via studio.mjs single transport; owner-gated, entry via oracle only (lane → oracle → manager → bridge → ChatGPT) | — | Never a direct lane transport; receipts + requestId dedup |
| 6 | `arbor-batch1` | **HISTORICAL** — batch-1 3-page baseline that ran as `dwfrun-e163906c` (completed, owner-approved 2026-11-14) | — | Never launch; reference for per-stage gate design |
| 7 | `arbor-life` | Autonomous continuous development v2 (owner 2026-11-17): 6-phase always-on pipeline — routing+host (deterministic boot, no model), feed (pre-fires next item so the seat never idles), USE (real-user journey on isolated scratch Host), OBSERVE (read-only verifier, ≤12 tool calls), FIX (FAST = 1 fixer + 1 test writer; DEEP = 3+1), VERIFY+handoff (gate ∥ evidence ∥ fresh-eyes replay concurrent). **User-loop mode (owner "go" 2026-11-17, [arbor-user-loop spec](workflows/arbor-user-loop/README.md)): unsettled goals GOAL-1..3 (open Wiki from Chat, live ARBOR progress, NL wiki edit) outrank queue items; a fresh simulated user (USER persona: no source reads, real UI via control-browser, behavior-only report, cannot self-certify) attempts the goal; OBSERVE never short-circuits goal rounds (user success is independently cross-checked); replay = a fresh user session told nothing about the fix; receipt verified only on browser-mode success — programmatic-partial success records honest "gated" + blocker.** Per-model concurrency cap = 2 (in-script semaphore, $level-stripped key). Ladder NIM-first: step-5 → mistral → ernie → inclusion → minimax (+nemotron-120b reserve); per-round re-probe skips hot models (any rate_limit in last 2 min = hot). GLM-5.3 gated out 2026-11-17: zai id case-ambiguous, FREEllmAPI id network-down — re-add when resolvable. Per-phase wall timings in `phase-times` table. Never edits roadmap.v1.json — keeper flips | `mode` = `life` (default) / `audit`, `maxRounds` (default 6), `speed` = `fast`/`deep` (auto: fast for verify/goal-kind), `item` | Continuous improvement loop; supersedes per-item arbor-build launches only after 3 consecutive green cycles |

Registry state 2026-11-16: **7 valid, 0 invalid** (batch1's missing frontmatter repaired this date; CRLF in def frontmatter normalized — it was surfacing as a `"build\r"` default arg; arbor-life added this date).

## 3. Expectations — operating rules every lane follows

**Running:** saved defs run via `CreateWorkflow saved:{name, args}` (user confirms the run). `AmendWorkflow` supersedes a run and imports finished work as warm cache — **known failure mode: it drops saved args; re-pass them on amend.** Runs are owned by the session/process that started them; from another session a live run shows `possibly_interrupted` (annotation, not verdict). An `errored` run is amended, not resumed; `stopped` runs resume in place; never resume a `superseded` run.

**Gates & evidence:** the standard is receipts + gate exit codes + a re-walk over real product routes — never self-marked. Owner gates exist only at batch exit and are presented as a question, never programmatically. Workflow runs do **not** run git commit. `roadmap.v1.json` has exactly one writer (keeper flip script); queue statuses `in-progress/queued/approved` are lane-truth.

**Models:** defs carry a fallback ladder health-probed at run start from `model_usage` (≥5 rate-limit/overload/timeout rows in 10 min → skipped). Owner routing 2026-11-15: OpenRouter primary `openrouter-2/baidu/ernie-5.5-flash-preview:free`, helper `openrouter-2/nvidia/nemotron-3.5-super-vl:free`; fleet defs since re-pinned to the NVIDIA NIM ladder (2026-11-16). Do not route deepseek/deepseek-flash (unavailable).

**Subagents:** dispatched sequentially (two concurrent manager subagents trip the user concurrency limit); every Agent-tool subagent returns cavecrew-compressed output; managers steer dispatched agents at least every 3 minutes.

**Comms:** the bus is `bun C:\0-BlackBoxProject-0\repos_hq\scripts\sync.mjs` — `check --as <id>` is the first act of every turn; a message in a file wakes nobody, only a reply proves a wake; standing rules also go to `*`. Lane split: demo builds, keeper independently verifies + receipts + flips queue.

## 4. Status as of 2026-11-16 ~14:00

- **HEAD:** `3dae90c` + this checkpoint commit (owner-directed commit/push of all lane work, 2026-11-16).
- **Test gate:** change-set gate exit 0 on the A04 change at 2026-11-16 13:49 (run `dwfrun-93ce2d7d`). Manifest now **303 entries** — `a02-verify-no-change.test.mjs` registered this date (was invisible to the gate; passes 2/2 standalone).
- **Queue** ([roadmap.v1.json](C:/0-BlackBoxProject-0/ARBOR/docs/product/roadmap/roadmap.v1.json)): A00 gated, A01 gated, A02 gated, A03 **failed** (blocks A05/A07 via dependsOn), A04 gated, A05–A08 queued. Verification fields still read `UNRUN` although [receipts exist](C:/0-BlackBoxProject-0/ARBOR/docs/product/roadmap/receipts/) — keeper one-writer flip pending (drift, see §5).
- **Key runs:** `dwfrun-e163906c` batch-1 build completed + owner-approved (7 stages, re-walk all-200 at product routes, receipts in [.coordination/artifacts/demo/](C:/0-BlackBoxProject-0/ARBOR/.coordination/artifacts/demo/)); `dwfrun-14ea8b6c` roadmap completed; `dwfrun-fa42e97a` batch-engine verify A00–A02 completed (A00 22/22, A01 16/16, A02 27/27 + receipts); builds `81e8d09` (A00), `8714703d` (A02 amended), `3c67bf1f` (A03), `93ce2d7d` (A04, gate exit 0). **In flight:** `dwfrun-ff0aa66c` arbor-demo queue walkthrough amended v2 — phase 7/14 (boot production Host), 48/49 steps settled, `possibly_interrupted` from this session's view. Paused: `dwfrun-83977512` roadmap re-baseline (owner: queue already truthful, one-writer safety).
- **PDR:** batch-1 exit record at [.coordination/pdr/PDR-batch1-exit.md](C:/0-BlackBoxProject-0/ARBOR/.coordination/pdr/PDR-batch1-exit.md).

## 5. Inefficiencies found & acceleration (2026-11-16)

Applied in this checkpoint:

| Finding | Evidence | Fix applied |
|---|---|---|
| New test file invisible to the gate | `apps/mobile-relay/test/a02-verify-no-change.test.mjs` absent from [test-manifest.json](C:/0-BlackBoxProject-0/ARBOR/scripts/test-manifest.json) (302 entries) | Registered (303), passes 2/2 |
| Registry noise: invalid def | `arbor-batch1.dwf.ts` failed frontmatter parse every `ListSavedWorkflows` | Frontmatter added, marked HISTORICAL → 6 valid / 0 invalid |
| Corrupt arg default | CRLF frontmatter surfaced as default `"build\r"` on arbor-build | Def files normalized to LF |
| Scratch at repo root | `.worktrees/`, `C:tmpfail-lines.txt` untracked junk | Added to [.gitignore](C:/0-BlackBoxProject-0/ARBOR/.gitignore) |
| Fleet unexplained in tracked files | defs are git-ignored; nothing committed described them | This document |

Queued (owner or lane action, not done here — one-writer and ownership rules):

| Inefficiency | Why it costs | Next acceleration |
|---|---|---|
| Serial dependsOn chain | A03 `failed` blocks A05 (deps A01,A02,A03) and A07 | Build independent items in parallel (A04 already ran concurrently); re-plan queue so a failed item doesn't stall unaffected lanes |
| `verification: UNRUN` drift in JSON | receipts exist but the machine truth wasn't flipped — verify evidence invisible to consumers | keeper runs batch-engine `mode=verify` flip (sole writer) |
| ROADMAP.md projection lags JSON | humans read the stale markdown (says A01/A02 in-progress; JSON says gated) | regenerate projection from JSON after each flip |
| AmendWorkflow drops saved args | every amend risks silently re-running with defaults | re-pass args on every amend (rule); engine-side fix pending |
| Errored→amended cycles | demo lineage a168fbdf → 287aed84 → ff0aa66c: each amend = new run + cache import | compile-clean + lint check of defs before launch |
| 191 unread bus lines for the manager seat | steering noise; "consume mail first, ack always" rule (DEBT-1) not yet adopted | batch-consume inbox; post standing rules to `*` |
| Cron wakes inherit workspace default model | model flap; durable fix is owner pinning the workspace default to ernie | owner action |
| `SaveWorkflow/CreateWorkflow` absent from subagent toolset | arbor-chatgpt-bridge can't be launched from a lane session | owner launches from an engine-tools session |
| Phase timing disabled in arbor-build ("0ms" phases) | phase cost invisible, bottlenecks unmeasurable | re-enable timing in def |
| Fleet defs not versioned | `.zcode/` ignored → fleet has no git history | owner decides: vendor defs under a tracked path (e.g. `docs/workflows/`) or accept this doc as the reference |

## 6. Immediate next moves

1. Keep `dwfrun-ff0aa66c` (demo walkthrough) steered ≤3 min cadence; it owns boot of the production Host right now.
2. After A00–A02 verify-flip lands: launch `arbor-batch-engine` (`mode=build`, `items=["A05"…]`) once A03's blocker clears, or re-plan the queue so A06 runs in parallel with A05.
3. Owner: pin workspace default model to ernie; decide on vendoring the def files.
