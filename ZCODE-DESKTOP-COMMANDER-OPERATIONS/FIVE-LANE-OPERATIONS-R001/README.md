
> **Live orchestration sequence (23 Nov R004):** [Source-grounded five-lane critical path, real ZCode/Desktop Commander sitrep and next ten accountable steps](STRATEGIC-EXECUTION-SEQUENCE-R004.md). This controls dependencies and source collisions but does **not** replace the live #33 backlog or enable unapproved effects.

# VIVIM / ARBOR — Five Parallel Lanes (R001)

A ready-to-run **five-lane operational handoff** using existing `ARBOR/.zcode/workflows` and canonical issue/claims authority. The five lanes are organizational workspaces and starting prompts, not new ZCode DWF registrations or automatic provider/model sends.

**Directory:** `ZCODE-DESKTOP-COMMANDER-OPERATIONS/FIVE-LANE-OPERATIONS-R001` in `owenservera/vivim-labs`.

| Lane | Mission | Existing workflow references | First state |
|---|---|---|---|
| [L01 Signed-in AI Conversations](01-chatgpt-provider/START-PROMPT.md) | A01–A16 and C01–C03; signed-in provider and capture proof. | `arbor-area-c.dwf.ts`, `arbor-chatgpt-bridge.dwf.ts` | `READY_NOT_LAUNCHED` |
| [L02 GPUI + Android PWA Convergence](02-gpui-mobile-surface/START-PROMPT.md) | S02–S14, U01–U04 and C04–C05; native and phone parity. | `arbor-area-d.dwf.ts`, `arbor-area-e.dwf.ts` | `READY_NOT_LAUNCHED` |
| [L03 Authorized Work + Oracle AutoWiki](03-work-oracle-autowiki/START-PROMPT.md) | U05 and O01–O06; PY-001 and U07 are dependencies, not duplicate primary claims. | `arbor-area-b.dwf.ts`, `arbor-oracle-corpus-verify.dwf.ts` | `READY_NOT_LAUNCHED` |
| [L04 VLANG + Visual Feedback + Gestures](04-vlang-visual-gestures/START-PROMPT.md) | U06–U11 and G02–G05; Z01 is historically completed, new failure proofs remain useful. | `arbor-vlang-iterative-observers.dwf.ts`, `arbor-product-verification-parallel.dwf.ts` | `READY_NOT_LAUNCHED` |
| [L05 QA, ZCode Admission + Release Evidence](05-verification-models-release/START-PROMPT.md) | Z02–Z04, Z07, R01, R03–R04 and R06–R07; independent evidence only. | `arbor-product-verification-parallel.dwf.ts`, `arbor-area-a.dwf.ts` | `READY_NOT_LAUNCHED` |


> **Strategically revalidated 2026-11-23 (R002):** [Read the complete source-grounded audit](STRATEGIC-AUDIT-R002.md), [corrected one-owner-per-task board](LANE-BOARD.md) and [R002 manifest](manifest.json) before starting a lane. All **64 open #33 items** now map to exactly one primary lane. Existing A–E/ZCode DWF references are **not permission to launch**: older workflow definitions can write/commit, send to ChatGPT or restart production Host. Preserve one primary ARBOR Dev writer and the incumbent model registry. This package does not launch five model agents.

## Quick start

1. Read [AGENTS.md](AGENTS.md), [DIRECTIVE.md](DIRECTIVE.md), [LANE-BOARD.md](LANE-BOARD.md), and up-to-date GitHub #33/#32/#31 + Labs TASK-CLAIMS. The root `AGENTS.md` is a local lane contract, not a grant to run product effects.
2. Treat the five folders as independent mission workspaces; use existing authorized ChatGPT/ZCode sessions when supported, and **do not assume cross-thread auto-start or wake**. For each, copy the corresponding `START-PROMPT.md` into that lane's session. Start no more than two **model-driven** read-only ZCode workers until verified runId/liveness/quota/cost.
3. Use incumbent `ARBOR/.zcode/workflows` **as references only by default**. Apply the R002 admission matrix and inspect current implementations, invoked scripts, source SHA, live grants and model cost before any invocation. Do **not** run `arbor-life` or enable new always-on execution. An independent ChatGPT session can follow a lane prompt without any ZCode workflow execution.
4. Every session produces its own handoff using `HANDOFF.md`. One primary Dev source writer applies claimed ARBOR code; other lanes can verify/prepare without source conflict.
5. Consolidate completed receipts into existing [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33); do not create a second task ledger. The existing hourly ChatGPT continuous-dev automation remains the only ChatGPT scheduler.

**Validation:** `node --experimental-strip-types FIVE-LANE-OPERATIONS-R001/validate.ts` when run from the parent operations folder (Node 24+). It checks only the local package structure and source references; it does *not* start sessions or prove model execution.

**Current baseline:** master #33 recorded 7/71 verified, 64 open; source-level ChatGPT/Studio tests and prior native verifier are distinct; no genuine provider/phone reply or five live model workers established by this folder package. Refresh the evidence at run time.

## R003: real canonical-issue coverage validation

The upgraded [`validate.ts`](validate.ts) checks the full R002 task-range mapping for duplicates, malformed ranges, safe launch configuration and the **27 current launch assets**. Without an issue body it explicitly returns `coverage.verifiedAgainstIssue=false`: an internal 64-ID count is not a current canonical issue comparison.

For a **live GitHub #33 check** on a machine with an authenticated GitHub CLI, use a temporary file and keep the main checkout untouched:

```powershell
$issue = Join-Path $env:TEMP 'arbor-issue33-current.md'
gh issue view 33 --repo owenservera/ARBOR --json body --jq .body |
  Out-File -FilePath $issue -Encoding utf8
node --experimental-strip-types .\validate.ts --issue-body $issue
Remove-Item $issue -Force
```

The `--issue-body` argument is optional for local structural checks, but **required to claim the lane manifest agrees with current #33**. Validation reads issue text and the manifest; it does not change the tracker, claim tasks or launch a worker. Regression tests: `node --experimental-strip-types --test validate.test.mjs` ([test source](validate.test.mjs)).

**Exact receipt, 2026-11-23:** the isolated Windows test archive at Labs main `793fa1a7dae1824a8299141657914ab7dc8402d7` completed **9/9 tests PASS**, validator exit 0. The independently fetched **actual #33 body** then yielded `open:64, checked:7, verifiedAgainstIssue:true, errors:[], issues:[]`, exit 0. The original dirty Labs checkout was preserved, not reset. This is tracker-coverage proof, not five launched agents or live phone/provider acceptance.

## R004 — full-workspace durability and work inventory

The launch files are now **complete work entrypoints** instead of starter prompts only. Each of the five folders has these additional documents:

- `WORK-INDEX.md`: every currently assigned canonical #33 task, the real task wording, source link, task dependencies and end-to-end acceptance. Current mapping = **64 open/7 checked**, exactly one primary among five lanes.
- `PLANNING-CROSSWALK.md`: all **37 original PL/AW/PY/GP/VF preparation candidates** allocated exactly once as *references*, with original priority, proposal state, prerequisites and the Planning source link. These are **not** 37 additional canonical tasks.
- `CONTEXT-AND-TOOLS.md`: authoritative ARBOR/Labs/Planning/Oracle documents and source code, intended ZCode workflows and safety/admission notes, Desktop Commander and cloud GitHub responsibilities.
- `NEXT-CYCLE.md`: a dependency-ranked continuing work queue, implementation and verification loop, evidence protocol and exit criterion.

The full ChatGPT `CHATGPT-LAUNCH.md` in every folder now explicitly **requires all four files** before task selection and at each recurring cycle. Existing `AGENTS.md`, `START-PROMPT.md` and `HANDOFF.md` remain.

| Lane | Full canonical work list | Planning crosswalk | Source/tool registry | Next-cycle queue |
|---|---|---|---|---|
| L01 | [19 tasks](01-chatgpt-provider/WORK-INDEX.md) | [1 proposal](01-chatgpt-provider/PLANNING-CROSSWALK.md) | [links](01-chatgpt-provider/CONTEXT-AND-TOOLS.md) | [queue](01-chatgpt-provider/NEXT-CYCLE.md) |
| L02 | [19 tasks](02-gpui-mobile-surface/WORK-INDEX.md) | [6 proposals](02-gpui-mobile-surface/PLANNING-CROSSWALK.md) | [links](02-gpui-mobile-surface/CONTEXT-AND-TOOLS.md) | [queue](02-gpui-mobile-surface/NEXT-CYCLE.md) |
| L03 | [7 tasks](03-work-oracle-autowiki/WORK-INDEX.md) | [16 proposals](03-work-oracle-autowiki/PLANNING-CROSSWALK.md) | [links](03-work-oracle-autowiki/CONTEXT-AND-TOOLS.md) | [queue](03-work-oracle-autowiki/NEXT-CYCLE.md) |
| L04 | [10 tasks](04-vlang-visual-gestures/WORK-INDEX.md) | [3 proposals](04-vlang-visual-gestures/PLANNING-CROSSWALK.md) | [links](04-vlang-visual-gestures/CONTEXT-AND-TOOLS.md) | [queue](04-vlang-visual-gestures/NEXT-CYCLE.md) |
| L05 | [9 tasks](05-verification-models-release/WORK-INDEX.md) | [11 proposals](05-verification-models-release/PLANNING-CROSSWALK.md) | [links](05-verification-models-release/CONTEXT-AND-TOOLS.md) | [queue](05-verification-models-release/NEXT-CYCLE.md) |

**Validation:** isolated exact GitHub Labs main `f3b4c2b1624a1646d5e60b1e01e951d90194e37a`, Windows Node 24 `validate.test.mjs` **9/9 PASS**; `validate.ts --issue-body <actual authenticated #33 issue text>` output `filesChecked:47`, `open:64`, `checked:7`, `verifiedAgainstIssue:true`, empty errors, exit 0. GitHub archive contained 49 files under the workspace. The original Labs checkout had one unrelated modified file, which was neither overwritten nor reset.

**Maintainability:** these files are a source-linked snapshot and operational runbook. New work, task completion, changed source or new owner directives **must be refreshed from #33, TASK-CLAIMS and Planning at each scheduled cycle**; never silently promote a historical snapshot. The existing hourly central automation and native observer are not replaced. Five actual ChatGPT lane automations still require individual creation/verification in their own launched chats.
