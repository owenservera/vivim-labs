# R006 — sustained multi-task five-lane execution contract

**Owner direction (23 November 2026):** The five existing ChatGPT development chats are not five one-shot SITREP generators. Each lane has a long, finite assigned backlog. **Do real engineering across consecutive tasks during a single invocation**; aim for a sustained 3–5-hour *campaign* when an appropriate execution environment supports it. A single ordinary ChatGPT turn still has finite compute/tool limits and cannot promise to remain awake afterward. Scheduled continuations reopen the same existing queue; a passive GitHub watcher is **not** a coding agent.

## Every lane: nested loop inside its current turn

**While tools and safe execution budget remain:**
1. Read current canonical ARBOR main and [#33](https://github.com/owenservera/ARBOR/issues/33), Labs [TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md), own [NEXT-CYCLE.md](README.md) and own ORCHESTRATOR-NOTE.md. Refresh *each time* before claiming new files. These resources already cover 64 open tasks; **do not build another queue**.
2. **First resume the exact active claimed task you already own**. Do not cherry-pick a different task because it looks simpler; if impossible, record an exact blocker and release only when safe.
3. Make the smallest real source implementation in your claimed existing scope; push to canonical ARBOR `main` with CAS/no competing changes. Run **exact source SHA** tests. Publish receipt to existing #33 and release claim, without marking wider acceptance complete unless there is actual Host/native/Android evidence.
4. **Do not finish here.** Select the next independent, unclaimed item from your full `WORK-INDEX.md` and `NEXT-CYCLE.md` / #33; claim, code, test, push, receipt and release again. Repeat rather than writing a second progress report. Track per-task source SHA, scope and test count.
5. If blocked by live effects, paid model limits, claimed files, runner policy or Android permissions, pivot immediately to a safe disjoint source/test scope from your backlog (do not wait for another lane). A provider-send block cannot block test-only work; an L02 lockfile build cannot block an L04 parser regression.
6. **Only when truly unable to perform further useful actions:** hand off current source, exact tests, open claim, next executable source edit/command, and reason, in own lane folder. Reports are **at the end of a working sequence**, not a substitute for engineering.
7. No duplicate Host/frontend/router or scheduler. Rust-first; existing MJS/TypeScript seams where necessary. Avoid effectful `arbor-life` and `arbor-chatgpt-bridge`; no historical uncertain send retries, unapproved paid-model fallbacks, private corpus ingestion, live Host restart, browser sends or device pairing modifications. Claim ownership and release safety are not optional.

## Concrete pickup on current claim snapshot (refresh before edits)

| Lane | **Resume first** (only if still ACTIVE) | Then consume remaining original assignments |
| --- | --- | --- |
| L01 | **A04 fresh Studio owner/session identity gate** — relay-chat-studio + direct-preflight test only | A01–A16 and assigned C tasks; safe signed-in thread/composer, no historical replay |
| L02 | **S03 same-SHA native Host/GPUI/PWA build preflight** (no source claim) | S02–S14, U01–U04, assigned C tasks; isolate immutable release and phone proof |
| L03 | **U05 full-source Wiki validation** — wiki_read.rs + new contract test only | U05, O01–O06; grants, readback, Work CAS; protect Dev PY-001 |
| L04 | **U08 composer completion source drift** — composer-assist + its test only | U06–U11, G02–G05; current-SHA negative native safety tests and semantic approval |
| L05 | **R07 frozen manifest self-containment** — packaging manifest + its test only | Z02–Z04/Z07/R01/R03/R04/R06/R07; R04 CI zero-job remains infrastructural |

**Progress is measured by real commits/fully pinned tests/accepted delivery stages, NOT length of the chat or amount of reporting.** Treat dates/SHAs here as a snapshot; live claims win. L01/L02/L03 currently have hourly scheduled resume; L04/L05 only have active chats unless a new verified executor or scheduling slot exists. The private Windows watcher runs every five minutes solely to observe public source activity; it does not send instructions into chats.
