# VIVIM — ten copies of the same useful loop
## Round 3: execution-ready design and safety disposition

**Date:** 2026-11-21. **Status:** Design and verification only; no recurring worker launched. **Source:** current ARBOR `arbor-life.dwf.ts`, `arbor-product.dwf.ts`, Windows deployed release `cf2e5c8`, owner tasks OW-001…017, Planning AW/PY/VF proposals, continuous fleet, tracker claims. **Supersedes R002 only for execution-level details; preserves its simple strategy.**

### 1. What gets repeated

One job, from an EXISTING user-visible tracker item:

`Claim test → open the one First Contact PWA → type/interact → capture what appeared → say useful/incorrect/blocked with specific evidence → file one reproducible defect (if any) → ask primary ChatGPT coding lane for narrow fix → independently replay → record outcome in original task → take another`.

The job repeats but its task changes. It is not another application, user dashboard, test database, semantic runtime or build pipeline. A task that passed a minimal milestone remains eligible for *quality improvement*, but its prior PASS evidence is preserved; do not silently reset the reached milestone.

Every job must end in one of:
- **VERIFIED:** fresh independent UI replay with exact observed result and source SHA, no regressions.
- **IMPROVEMENT_FOUND:** concrete fail, actionable reproduction, accepted for source work; not done until fixed and replayed.
- **LIMITED:** partial API/unit proof only; no actual UI coverage.
- **BLOCKED:** missing browser/session grants, model 429, source conflict, tracker owner, or isolation; no fake result.
- **NO_DEFECT:** honest no-op, next corpus/task; do not create a busywork fix.

### 2. What exists; what needs correcting

Verified source facts:
- ARBOR `arbor-life.dwf.ts` is a ~980-line six-phase loop with args `item, goal, mode, speed, maxRounds`, hard-coded three GOALs and a feeder from `docs/product/roadmap/roadmap.v1.json`; it already creates journey, verifier, fixer, and fresh-user replay agents.
- This local definition contains **23 occurrences of port 43187** and **51 legacy /chat /wiki /demo /api/state /api/mutate route references** (text matches, not all executable). It includes an explicit Host nurse that may kill and restart port 43187 after a fix, plus a legacy /demo feedback/journal flow.
- `mode=audit` only switches `canFix` false; it does **not** prove a no-write, no-restart execution. The workflow later writes a roadmap receipt via an agent and sends handoffs; discovery mode can initiate repairs. **DO NOT RUN THIS FILE IN AUDIT OR LIFE MODE AGAINST THE PERSONAL HOST BEFORE PATCH.**
- The original `arbor-product` code still owns historical `public/pages/chat.js`, `wiki.js` and a `vec-compiler.mjs` seam, and freezes `apps/mobile-relay/app.js`. That is not the current one-UI ownership model. Treat its existing source writer as retired for the First Contact UI until real scope reconciliation.
- Existing `scripts/life/scratch-host.mjs` provides separate ephemeral ports and data roots and avoids port 43187 for legacy Host routes. Its `journey` exercises `/api/mutate` and `/api/state`, **not proof of current authenticated First Contact Wiki API or browser use**.
- Laptop currently observed at ~8,010 MB available RAM and 47.35 GB free C:, Host PID 17380 serving immutable `cf2e5c8`. ZCode local SITREP shows 15 saved workflows; two previous agent fanout attempts hit free-model 429. A process name count is not a live worker count.
- The currently deployed immutable release's **25 focused tests passed in this round**: `first-customer.test.mjs` 9/9; `agent-composer-browser.test.mjs` 11/11; `vlang-command-compilation.test.mjs` + `wiki-revision-projection.test.mjs` 5/5. All three commands exited 0, source path `C:\0-BlackBoxProject-0\.arbor-pwa-releases\cf2e5c8da0d5`. Real Android and independent live agent delivery remain UNRUN.

### 3. First ten real assignments

These are TEST ASSIGNMENTS inside existing OW tasks; do not create ten new product tasks. Each gets the same loop, distinct test target and a small corpus. No requirement for ten different models.

| Seat | Existing task references | First realistic interaction | What the worker is checking | If broken, smallest product work |
|---|---|---|---|---|
| 1 | OW-007, OW-017 | `Vim` / `VIVIM` / `wiki` / `roadmap` | Is the Wiki response relevant and honestly source-backed? | Rank/identity/query binding, only if observed wrong |
| 2 | OW-017, AW-007, AW-012 | `show roadmap` / `what's planned` / `do not edit roadmap` | Similar meaning, ambiguous command and negation; typing **must not execute** | Rust/VLANG language rule or safe clarification |
| 3 | OW-003, OW-007, AW-005 | `Roadmap` → `Main` → clear → paste `Vim` | Latest input wins; no obsolete results or send | Rust revision/high-water mark + minimal DOM adapter |
| 4 | OW-002, OW-007, AW-008 | Search, expand real Wiki result, back, clear | Readable linked/source-scoped content, stable interactions | Existing Wiki view/record, no replacement wiki |
| 5 | OW-017, AW-005/010 | Update a disposable Wiki source while old request is pending | Source revision invalidation, precise empty states | Existing Rust query fence and Host projection |
| 6 | OW-007, OW-014, AW-004/013 | Revoke read grant/expire session in scratch test | No leaked title/snippet, no effects on typing | Existing Host grant/read path |
| 7 | OW-009/014 | Authorized agent drafts `Vim` in same composer | Actual on-screen text + Wiki response + distinct actor ACK; no Send | Existing agent composer Rust/TS adapter |
| 8 | OW-002/009 | Human types while agent edit is pending | No overwrite or false ACK | Current conflict fence; never a second editor |
| 9 | OW-013/016 | Compare Rust/WASM output to same UI input | Rules really execute in Rust; browser glue only | Rust migration with parity test and rollback |
| 10 | OW-003/007/014, VF-001/002 | Fresh independent replay of failures from seats 1–9 | Can a second actor reproduce the *exact* visible result? | Regression case + original task receipt |

**First focus:** Seats 1, 3, 4. These harden the achieved Wiki milestone. Then 2/5/6 (meaning/source/safety), 7/8 (agent path), 9/10 (architecture and independent evidence). Seats are fungible and get next eligible tracker case when finished.

### 4. What one actual worker turn should look like

Example job: `OW-017 / first-contact-wiki-relevance / Vim`.

Input receipt envelope: `task=OW-017; sourceSha=cf2e5c8...; build=deployed immutable; mode=observe-only; test text="Vim"; expected behavior=source-appropriate Wiki matches OR truthful no-match; interface=existing First Contact; data=isolated session; permission=read-only; retries=2; no paid model fallback`.

1. Claim only the test slice; do not claim OW-017 application source while Dev owns it.
2. Open the current First Contact page in an isolated and authenticated TEST session, not a new dashboard, not a shared foreground user window.
3. Enter `Vim`; record text visible, Wiki title/sections/links, source revision, response state, and any no-match/error/latency. Redact raw private content in cross-agent reports.
4. Rate what was ACTUALLY observed: **correct/grounded**, **useful**, **clear**, **stable**, **safe**. A count of four matches alone does not establish relevance.
5. Try one nearby perturbation, such as `VIVIM`, deletion or an interrupted fetch. A deterministic test may execute without an LLM call.
6. If there is a failure, save an exact reproduction and minimum source references. If results are good, report NO_DEFECT; do not demand a code change.
7. Primary ChatGPT coder picks up a verified failure, makes the smallest Rust-first fix on GitHub main, and provides commit/test proof. The ZCode observer never patches a dirty local ARBOR tree as a side effect.
8. A separate fresh observer retests the ORIGINAL `Vim` case AND one neighbor using the new source SHA. Only then check in original OW task / claim evidence and return seat to queue.

Compact evidence record (**attach to existing OW/claims receipt, no new store**):
`taskId | sourceSHA | session/run IDs | test input digest/class | expected | actual visible result + wiki/source revision | state PASS/FAIL/BLOCKED/LIMITED | screenshot/observation path | defect or fix SHA | independent replay | next case`.

### 5. Exact changes to make in the existing ZCode file (do not launch before code review)

Keep the existing lifecycle and phase orchestration. Implement **small patches** to `arbor-life.dwf.ts`, not a new DWF fleet:

1. **Hard safety mode:** Add explicit `surface=first-contact` + `role=observer` with test-only checks. Observer must have no source/file write tools, no approval/submit/mutate tools, no receipt-writing LLM, no Host nurse/restart, no roadmap flip, no /demo feedback POST, and no blanket provider escalation. Require current First Contact source SHA. A no-write `mode` string by itself is not enough: test actual runtime grants.
2. **Real entrypoint:** Replace historical UI journey steps with the current `/mobile/index.html` Direct surface, but **do not** aim controlled browser tests at production `127.0.0.1:43187` or tailnet owner's personal journal. Keep a separate cheap non-mutating production smoke check if explicitly authorized.
3. **Isolation adapter:** Reuse `scratch-host.mjs` lifecycle; add/verify genuine test-scoped Direct owner identity and `/mobile/api/first-contact/wiki` read path before UI journey. If authenticated scratch Direct cannot be established, report BLOCKED rather than disable auth or claim browser PASS from a `/api/state` probe.
4. **Task feeder:** Accept an explicit existing `OW-...` or `AW-...` task test slice and look up its original source; do not pretend these IDs exist in old `GOALS` array or `roadmap.v1.json`. For a first pilot, use an explicit pre-reviewed job packet to avoid adding another tracker writer.
5. **Output:** Reporter returns **only** observed behavior + source pins + minimal reproduction to existing tracker/owner, as separate read-only diagnostic. Do not let a workflow auto-write a `verified` roadmap receipt. Integration and tracker keeper act after independently verified evidence.
6. **Source edits:** Remove automatic local source patch/restart logic for owner tasks: source fixes remain the cloud ChatGPT coding lane. If a future isolated ZCode builder receives an explicit separate owner grant and disjoint files, gate it individually and never modify real Host/dirty local main.
7. **Budget:** One assigned case + one neighbor per worker turn, no more than two repair rounds per accepted defect, limited model requests, stop at 429/identity/scope conflict. Reuse existing provider health circuit breaker, no silent paid fallback.
8. **Verification:** Add tests asserting observer cannot write source, call `/api/mutate`, restart 43187, send effects/approvals, or mark goal VERIFIED without real UI proof. Add fresh-browser replay when available. Owner sees PASS/FAIL/LIMITED per task, never an overall green inferred from test exit alone.

`arbor-product` should be treated as an implementation-role template only. Its current frozen `apps/mobile-relay/app.js` rule and /chat /wiki page scopes conflict with the single UI. Do not start it to "help" the First Contact app until rewritten for actual owner claims.

### 6. Real concurrency without stepping on us

A scheduler/controller is only a modest **work assignment mechanism using existing workflow APIs**. Each job is FINITE. The supervisor can refill freed seats on new eligible tracker work; this is not an infinite agent conversation or new database.

- The **user-facing UI** keeps running independently; **no background worker steals focus, keyboard or visible tabs from the owner's active desktop session**.
- Jobs 1–6 begin with read-only deterministic tests and independently scoped scratch sessions. Many tests can run concurrently without model usage; ZCode model worker count is limited by observed quota/health, not the number 10.
- Builder scope: exactly one source writer per file, ChatGPT cloud `main` first. ZCode local worker can test and report; it cannot implicitly patch `apps/mobile-relay/app.js` or Host.
- Independent verifier must never share the builder's reasoning or decide its own success. Fresh UI replay has a different session and, if possible, a different agent.
- ZCode capacity ramp: **first prove 1 real UI session**, **then 2 independent child IDs + two disposable auth sessions**, then 3–5, finally up to 10 distinct assignments as RAM/quota allow. Ten slots may mean five deterministic jobs + two ZCode tests + three pending builder/verifier slots, not 10 concurrent browsers.
- A running worker must record ZCode parent/child/run IDs and acknowledge completion; `session/subagents` empty means no verified child. A saved `.dwf.ts` file or OS ZCode process count is not proof of active parallelism.
- No automatic night schedule until the bounded job and recovery path are separately tested; recurrence must be installed/verified explicitly. A one-off headless call is not persistent scheduling.
- Existing keeper is the one roadmap writer; GitHub `TASK-CLAIMS.md` is the ownership layer; local `OWNER-WORK/REQUESTS.jsonl` is the directive record. Preserve all three, no shadow queue.

### 7. What can run *now*, versus dependencies

| Task | Reality today | Next proof |
|---|---|---|
| Rerun focused tests on immutable built source | **EXECUTED:** 9+11+5=25 passing, exit 0 | Add a real observed Wiki-quality case, not just structural assertion |
| Inspect and compare current Wiki source/semantics | Read-only work possible; no agent cost necessary to inspect | Tie an observed bad match to a specific Rust/Host rule |
| Run the old `arbor-life` in `mode=audit` | **NOT SAFE TO ASSUME:** still writes receipts, routes legacy, possibly effectful | Patch source/permissions and prove no-write |
| Actual isolated First Contact browser session | **UNRUN:** scratch helper exists but legacy routes | Verify secured test identity and browser owner |
| Parallel new ZCode child agents | **UNPROVEN:** prior 429, read-only session queries supported | Two actual child IDs + separate session evidence |
| Running for hours unattended | **NOT CONFIGURED** | Prove bounded job, then scheduler + crash recovery |
| Actual phone or agent physical UI acceptance | **UNRUN** | Real device/screenshot/ACK through current code |
| Source hardening in Rust | Rust core/source available; active JS parity not completely mapped | One defect-driven Rust migration, actual WASM/UI parity |

### 8. Acceptance of the *loop* (not just product)

The first successful loop is ONE task with:
1. Real isolated First Contact user input and visible Wiki output.
2. Actual observed issue (or truthful no-defect) with source-pinned evidence.
3. If issue: smallest Rust-first cloud code fix, test and release into same unchanged PWA, without local dirty code edits.
4. Separate replay by another worker against the same input and a neighbor.
5. OW/claims check-in with exact real state and next case.

The second milestone is **two real independent in-flight workers** without source ownership conflict or foreground-window interference, and with each run reaching a terminal state.

The third is **repeatability**: the worker takes another existing tracker slice after closing one, preserving previous good evidence and retaining failed cases as regression tests. Do not scale to 10 until these three are measured.

### 9. Owner-facing report (tiny)

`VIVIM loop: 2 testing / 1 awaiting code / 1 independently replaying / 6 available`

`Last useful improvement: OW-017 Wiki latest-query-wins; before FAIL; after PASS; source abc123; original input + variant independently verified`

`Main blocker: isolated authenticated First Contact test session not yet proven`

Only show exact observed numbers. Separate jobs waiting for code from jobs actually running. Owner should not have to read 980-line workflow output or 10 full model transcripts.

### 10. Next execution pass

**First patch target:** make a safely read-only `arbor-life` First Contact *test mode*, not a second workflow. Produce a proposed exact diff before activating it; assert no writes, no production Host restarts, no source edits and no automatic paid model calls. Verify its ZCode tool grants and current browser control support.

**First test target:** `OW-017` / `Vim` through current Wiki, independently replayed. If no isolated authenticated browser available, keep it BLOCKED and run existing 25 deterministic tests while addressing precisely that one missing seam.

**Program priority:** quality of actual Wiki content and interactions first; then agent typing; then VLANG/command meaning and Rust parity driven by the observed defects. Every task must be PWA-visible or a direct dependency of a visible task.

**No claim:** No DWF was launched, patched locally, scheduled, or verified for autonomous concurrency by writing this design. The live user's VIVIM and dirty local ARBOR checkout remain untouched.
