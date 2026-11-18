# Realtime dev loop v1 — design

Date: 2026-11-16. Status: DESIGN (active lanes keep running; this plugs in underneath them).
Goal: the fastest honest path from queue item → verified change on the **live Host**, with the manager developing directly against that Host between cycles — one workflow actively interacting with the Host, not batch walkthroughs.

## 1. What the Host already gives us (measured today)

Live surface at `http://127.0.0.1:43187` (all 200 on re-walk): `/` `/chat` `/wiki` `/api/state` `/api/roadmap` `/api/journal` `/api/file-surface` `/api/mutate` `/demo` `/demo/demo-shell.html` plus the `/api/host/*` and `/api/onboarding/*` families.

- **Feedback channel is polling by design**: `POST /demo/feedback` appends a typed round `{round, approveAll, verdicts[{decision}]}` to the shared product journal (`dataRoot/product-state/journal.jsonl`, served read-through at `/demo/journal.jsonl` and `/api/journal`); `bun apps/vivim-host/public/demo/poll-round.mjs --since <round> | --last N` prints rounds as JSON. One-shot CLI, disk-read only (C9) — **no SSE/WebSocket exists**, so "realtime" = short poll interval (5–15s) against the journal, not push.
- Owner/manager verdicts and lane notes are both just typed journal entries — one bus for humans and machines.

## 2. The loop — one long-lived workflow: `arbor-realtime-dev`

Owns the Host and cycles tight:

1. **Host health**: GET `/api/state` every cycle; on 000 → boot `bun apps/vivim-host/server.mjs` detached (VIVIM_HOST_PORT=43187), wait for 200, log boot.
2. **Select**: read `docs/product/roadmap/roadmap.v1.json` buildQueue → next actionable (`queued|in-progress|approved`, dependsOn respected; `failed` only if requeued).
3. **Build scoped**: 1 lead + ≤6 workhorse subagents on the item's partition (same brief→build→gate shape as arbor-build, but no survey re-reads — the queue note carries the source pointers).
4. **Gate**: `bun scripts/test-gate.mjs` (change-set selection) must exit 0; ≤2 repair rounds.
5. **Publish evidence to the Host**: POST the run's receipt/verdict as a journal round via `POST /demo/feedback`; re-walk product routes (all-200 check).
6. **Collect verdicts**: `poll-round --since <lastRound>` every 5–15s while the cycle is open; `approve` verdicts → flip queue entry via the one-writer FLIP_CODE (`process.argv.slice(1)` fix) + write `docs/product/roadmap/receipts/<ID>.receipt.json`; `revise` → next cycle repairs that item.
7. **Next item.** Target: ≤5 min per scoped item end-to-end.

## 3. Manager-in-the-loop contract (how I develop from there)

- The main session talks to the **same Host**: reads `/api/state /api/journal /api/roadmap`; writes verdicts/notes as journal rounds via `POST /demo/feedback` (typed `{round, verdicts:[{itemId, decision, note}]}`).
- Between cycles I edit files directly in the worktree; the workflow's next gate run sees them (git diff), so manager edits and lane edits land in the same gate — no parallel-write conflicts as long as: **workflow owns queue flips + receipts (one-writer FLIP_CODE); manager owns direct file edits + verdicts.** Demo lane keeps the visual walkthrough UI on `/demo` and never edits product source outside its sanctioned demo paths.
- Escalation channel stays the repos_hq bus (`sync.mjs`), checked first act of every turn.

## 4. Model mix (routing per `docs/research/nim-models/NIM-MODEL-ROUTING-GUIDE.md`)

- **NIM primary**: workhorse `new-provider/nvidia/nemotron-3.5-super-vl-120b-a12b` (1344 calls/24h, TTFT 2.4s, 6 errs), lead `new-provider/nvidia/nemotron-3-ultra-550b-a55b`; health-probed ladder (skip model at ≥5 error rows/10 min in `model_usage`).
- **GLM-5.3-Flash added** (owner directive 2026-11-16): reachable via `account:zai-start-plan/GLM-5.3-Flash` — probe verdict OK today (4 calls / 26s wall). Use as **second lead / fast workhorse**. Constraints (measured earlier): start-plan pool ≈4 account-wide slots server-side and the manager's own turn counts → **at most 1–2 GLM subagents in flight at once**; `thoughtLevel: low` = ~3× faster than max → use low for mechanical parcels, high for the brief/gate judgment; skip GLM when its model_usage error rows ≥5/10min (extend the ladder probe to `GLM-5.3-Flash`). Do NOT use `new-provider-3/glm-5.3-flash` — that local proxy lane is dead (ECONNREFUSED :31415).
- Fallbacks: lightning-30b → bolt-e4b (NIM), mistral-large-4 for strategic/main.

## 5. Failure modes carried into the design

From the NIM guidebook §4 (all measured today): untyped ask + JSON.parse (+1 strict retry) for briefs; trimmed prompt payloads; unique agent names per loop; `bun -e` argv `slice(1)`; no Date.now/fetch/fs in scripts; NIM subagents have no image input (programmatic visual checks only, `notCovered`); AmendWorkflow drops args (verify-mode must not depend on args).

## 6. Relationship to the running lanes

- `arbor-realtime-dev` (new) = fast code path on the live Host, manager-in-the-loop.
- `arbor-demo` (running, `dwfrun-ff0aa66c`) = visual walkthrough + human verdicts UI on `/demo` — keep; it feeds the same journal.
- `arbor-build` chain (A07 running) = same queue consumer; realtime-dev supersedes it for speed once proven — keep build as the fallback until realtime-dev proves 3 consecutive green cycles.
- `arbor-batch-engine` verify = independent receipts per item; realtime-dev calls its runner per item when `scripts/batch-evidence/<ID>.mjs` exists.
- `arbor-roadmap` stays paused (queue truthful; one-writer safety) until a re-baseline is wanted.

## 7. Acceptance for v1

Three consecutive cycles with: gate exit 0, receipt written, queue flipped, re-walk all-200, owner/manager verdict consumed from the journal within one cycle. Then adopt as the default path and retire per-item arbor-build launches.
