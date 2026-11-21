# VIVIM: 10 simple tracker-driven improvement loops — R002 (supersedes R001)

Date: 2026-11-21. Status: **DESIGN ONLY — NO BACKGROUND WORKERS LAUNCHED**.
Decision: the owner rejected the R001 "Parallel Evolution Engine" as overengineered. No stochastic harness, no alternative UI, no new fleet, no duplicate tracker, no artificial architecture milestones. Keep **one text-box First Contact PWA** and repeatedly improve the two reached product behaviors: type → Wiki responds; local agent can enter text in that same composer through the existing authorized path (the independent agent-originated live ACK remains to be verified).

## Five-day rationale (Nov 16–21)

- Nov 16: roadmap is a thin feeder; preserve already completed work.
- Nov 17: first-time-user agent should use actual PWA, observe failures, verify and replay fresh. Existing `docs/workflows/arbor-user-loop/README.md` defines this inside `arbor-life`, not a second loop.
- Nov 18: owner changed ARBOR LIFE to **90% product / 10% verification**; no orchestrator project or endless ceremony; target typed NL → preview/approval → Host → Wiki effect as eventual higher milestone.
- Nov 19: requested **1–10 concurrent dynamic workers** derived from the living master tracker, one builder → independent verifier → repair, no duplicate edits.
- Nov 21: one First Contact composer wins over GPUI and alternate dashboards; Windows PWA via Tailnet observed typing **"Roadmap" → 4 Wiki hits** and **"Main" → 1**, revision 1; one-UI source release `cf2e5c8` proven. User reports "Vim" produces useful Wiki output. Keep what works, expand usefulness. Rust-first on behavior, minimal browser adapter, no new UI.

## The simple definition

**One identical job loop, 1–10 copies.** Each copy checks out one *existing tracker task* (or narrow tested slice), drives actual PWA text input where possible, records observed output, identifies one specific weakness, proposes/improves it, reruns the same test and a fresh regression, then checks the result back into the existing tracker. No worker can mark itself DONE based on its own prose. Each finite job ends and can take the next eligible tracker item when an existing runner/orchestrator starts it. "Background" means actual bounded ZCode worker processes, not a model-thinking promise.

`TAKE EXISTING TRACKER ITEM → TYPE/INTERACT → OBSERVE WIKI → FIX ONE DEFECT → RETYPE → VERIFY → RECORD → NEXT`

Use existing `arbor-life` (USE→OBSERVE→DIAGNOSE→FIX→VERIFY/REPLAY), `arbor-product` for a scoped producer, `arbor-batch-engine` for independent acceptance, and existing keeper for queue transitions. Do **not** launch `arbor-demo` as another customer UI, do not auto-run historical `arbor-batch1`, do not introduce a competing planner. The 10-lane Continuous Improvement Fleet R004 is a set of **skills/owners to consult**, NOT ten programs that must all start.

## What is "the list"?
1. **OWNER-WORK/REQUESTS.jsonl + NOW.md**: 17 owner-requested OW-20261121-001..017 work items. Local append-only ledger and manually refreshed NOW projection. **They are not 17 ZCode workflows.** The on-disk NOW predates the later `cf2e5c8` release; latest result is in GitHub `TASK-CLAIMS.md` and `OWNER-ONE-UI-20261121.md`.
2. **TASK-CLAIMS.md** on Labs GitHub main: check-in/out and executor locks ONLY; not backlog.
3. **DEVOPS-PLANNING/MASTER-PLAN-R001.md** on Labs main: 37 planning proposals (AW, PL, PY, GP, VF), *not* 37 active tasks. For this program, pull only tasks that visibly improve the one PWA or are hard prerequisites. GPUI remains retired as customer interface, not an active target.
4. **ARBOR canonical roadmap** (docs/product/roadmap/roadmap.v1.json) supplies its existing product queue when relevant; reconcile OW/PL IDs to that queue, never create duplicates or silently flip keeper-owned status.
5. **No mirror database or replacement JSON tracker.** Cross-reference IDs in receipts, preserve existing source history and GitHub claims.

## Ten initial assignments, not ten permanent job descriptions

All follow the SAME cycle. Each is a narrow test of a real current product behavior and a path to a small Rust/Host/PWA improvement. Prioritize read-only Wiki before effectful commands.

| Slot | First test | OW / Planning mapping | What good means |
|--|--|--|--|
| 1 | Type "Vim", "VIVIM", "wiki", "roadmap", check actual relevant matches | OW-007, OW-017 | Relevant grounded Wiki results or explicit no-match, no invented content |
| 2 | Try varied everyday phrasings and incomplete words | OW-017; AW-012 | Relevant search; ambiguity stays non-effectful |
| 3 | Type, erase, paste, quickly replace query | OW-003, OW-007; AW-005 | Latest entered text wins; no stale Wiki flash |
| 4 | Open returned Wiki details, navigate back, clear input | OW-002, OW-007; AW-008 | Readable results with correct source/revision, returns to clean composer |
| 5 | Change the Wiki source revision or remove a section in isolated test state | OW-017; AW-005/010 | No stale/ghost Wiki result after real source revision |
| 6 | Check access/expired grant/offline, no Send on typing | OW-007/014; AW-004/013 | Safe empty/refusal; no unauthorized effects/data |
| 7 | Have local authorized agent put text into same composer | OW-009/014 | Real composer shows text; real Wiki reacts; distinct actor ACK; no submit |
| 8 | Human edits while agent draft is pending | OW-002/009 | Human input wins or a visible stale/conflict refusal; no overwrite |
| 9 | Check actual Rust vs browser behavior at First Contact/Wiki seams | OW-013/016 | Rust owns deterministic rules; remaining browser glue justified, parity tests |
| 10 | Fresh-user rerun, broken-result inspection, real screenshots & tracker receipt | OW-003/007/014; VF-001/002 | Different verifier reproduces result and logs PASS/FAIL/UNRUN honestly |

Slots are fungible: if slot 4 finishes, it can take the next tracker task. The same code file cannot be owned by two writers. Avoid ten simultaneous expensive models. Launch initially 1, then 2–3, then 5/10 only after ZCode child IDs, browser isolation, provider health, memory/disk, and completion receipts prove real concurrency. Prior OpenRouter 429 means unverified fanout, not ten active agents.

## Exact one-job handoff
Input: existing `taskId`, current ARBOR main SHA, existing First Contact URL/isolated test route, brief user-visible input or goal, expected behavior, max rounds and provider budget.
Worker: OBSERVE actual UI, compare with expected, capture minimal redacted screenshot/DOM + Wiki revision and Host observation, classify PASS/FAIL/BLOCKED; diagnose only failure; deliver one narrowly scoped patch **proposal** to ChatGPT cloud coder unless specifically permitted disjoint source write ownership. ChatGPT codes and pushes to `main` and requests scoped local retest. A different worker reruns original input and a nearby variation (fresh session). Only accepted proof advances task.
Output: `taskId | input/goal | observed | result | exact source SHA | screenshot/Host observation ref | defect/fix ref | next test`. Store it with existing task check-in/receipt path; do not create another acceptance system.
Budget: finite two repair rounds on one defect; stop on no progress, conflict, no usable local agent/browser, identity missing, 429 or unclear model billing. No auto paid fallback. Type-tests are always read-only; mutate test Wiki only on disposable isolated Host state with genuine test-scoped grants, NEVER the production Host/user Wiki.

## What I (ChatGPT) do vs ZCode
- **ChatGPT + owner**: choose goals and decide what usefulness means, read summary, do hard design and real Rust/source integration in cloud GitHub main, approve rollout based on independent evidence.
- **ZCode local jobs**: independently exercise actual PWA/UI and existing tests, generate concise defect observations, run bounded approved local verification, return receipts. ZCode may use existing `arbor-life` DWF; no new fleet system.
- **Existing keeper/integrator**: exclusive source path/queue checkouts, serial integration where paths overlap, no concurrent local dirty main edits. Local Desktop Commander is runtime verification support, not primary product source writer.

## Start sequence after this design
1. Rebase `arbor-life` *instruction scope* from obsolete 3-page /chat/wiki/demo and port-43187 demo mutation journeys to **one current First Contact PWA**. Make any programmatic test use exact authorized test identity; never weaken Host security to make tests easy. Do not start old DWF unchanged.
2. Test just one same-input repeated cycle: `Vim` → observe actual Wiki → note one quality issue → reproduce → run focused Rust/Host check → independently retype and compare. Log compact receipt under the existing OW-007/017 task.
3. Verify two distinct ZCode worker run/session IDs, two isolated sessions, no duplicated edits. If unavailable, run deterministic tests in parallel and preserve UI proof in one controlled browser seat; report the limit.
4. When steady, allocate up to ten work items dynamically from OW-007/009/014/017 and dependent OW-001/002/003/013/016 / AW/VF tasks, always visible or directly supporting the visible composer. Review output with owner while heavy coding continues separately.
5. On each accepted regression improvement add its test to the existing gate. Leave the product URL/UI unchanged. Proceed to typed intent/preview/approval only when read-only Wiki is useful enough, without demanding artificial perfection.

## Deliberately removed from R001
No Monte Carlo platform, no hundred UI clone simulator, no ten permanent specialized orchestration engines, no new stochastic scenario database, no fabricated success thresholds, no new customer screen. Existing Rust, VLANG, HQKG, Host and journal are tools used only as a specific tracker need demands.

## Status
**DESIGN ONLY.** One live Windows PWA typing baseline is known; ten concurrent workers, automated recurring scheduling, real agent-originated ACK, isolated first-contact browser sessions and Android parity are NOT verified. An existing ZCode workflow file is not itself a running background worker.
