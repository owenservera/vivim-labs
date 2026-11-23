# VLANG exact-source intermittent regression — reproducible no-effect triage packet

**Date:** 2026-11-23 · **Canonical ledger:** [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33) · **Scope:** existing `OPS-OBSERVERS` / Z01/Z07. This is an incident addendum to R001, not a new master task or permission to edit the parser.

## Actual evidence, preserve both red and green

- **Source SHA:** ARBOR `516a23e826e812bea7e86c1bad64683d0c495923`; existing isolated Windows native verifier `local-logs/native-verifier/latest.json` recorded `state=BLOCKED`, `error=PARALLEL_TEST_RUNNER_EXIT_3`; start `2026-11-23T19:46:18.1202910Z`, finish `2026-11-23T19:49:51.3763274Z`; existing runner `maxConcurrentLanes=1`, no source writes, Host effects, provider sends or release promotion. Native hash `389C67DB9FA6913E1B257A5339846BCEA5BA96FB4429DC168059A0D2B33CEC14`, runner hash `5D929411261DB2D365F422AE80E5F2E13848F24130000F351F8277F1C569D6CB`.
- **VLANG scope:** `FAIL` with timeout after ~65 sec; summary `tests/pass/fail=null`, so **do not fabricate a count**. Captured `command-intent-native.test.mjs:565`: expected `INVALID_READING`, actual `undefined`. `vlang-command-compilation.test.mjs:96`: expected HTTP 200, observed 409 `CONTROL_PLAN_STALE`. Could be source behavior or resource-/timing-dependent; this incident does not establish causality.
- **Safety scope:** 159/160 PASS, one failure at `command-intent-v01.test.mjs:122`, exact corpus `Set theme to ember and the theme to ink`: expected `clarification_required`, observed `refused`. Do not change test expectations to make it green.
- **AutoWiki scope:** 12/12 PASS. **Visual scope:** 21/21 PASS. All were no-effect deterministic test lanes.
- **Prior genuinely green source:** ARBOR `dfb5480b17f5daa8861352e0cb81dd97e01e9f9d` passed 215/215 at run `vlang-2026-11-23T19-34-47-623Z`. This historical green does **not** transfer to the 516a red run.
- **Separate Studio security tests:** source `516a23e8` **17/17 PASS**, repeated at `f44ae9b5`; **separate Host phone-session** `8/8 PASS` and **Studio Work CAS** `10/10 PASS` at `7d6705f7`. These suites do not overrule the VLANG failure.

## Safe deterministic reproduction — ZCode / independent verifier

1. Read current Task Scheduler `VIVIM-VLANG-Observer` and `local-logs/native-verifier/latest.json`. At the last inspection the newest scheduled run was **RUNNING** (started 2026-11-23 20:56:20 Europe/Madrid) while this red receipt was from the preceding run. **Do not launch a second full observer, build a competing Rust executable, restart Host, or run anything effectful while it is running.**
2. Once the existing task is Ready, on one source-pinned isolated ARBOR snapshot of the **red** SHA, run exactly the failing tests without parallel native workloads. Source test names: `apps/vivim-host/test/command-intent-native.test.mjs`, `apps/vivim-host/test/vlang-command-compilation.test.mjs`, and `apps/vivim-host/test/command-intent-v01.test.mjs`. Prefer `node --test --test-concurrency=1 --test-name-pattern <exact case>` for narrow reproductions, then the unchanged full file. Never blindly run this against production Host or the user's private data root; each test must use its declared isolated fixture.
3. For each failed invocation capture **source SHA, native binary SHA256, command, exit code, elapsed time, native JSON error/refusal (bounded), and test assertion**. Compare identical input at red SHA and current HEAD. The existing native helper timeout of roughly 1.2s and constrained Windows concurrency are hypotheses, not established root cause.
4. If an isolated case fails deterministically at the exact same SHA, transfer a specific source defect to ChatGPT Dev on GitHub `main` with non-overlapping file claim. If it passes in isolation but the full scheduler fails, investigate competing Rust/Node processes and startup/timeout budgets **without** suppressing test assertions or claiming a source regression.
5. Only after a complete no-effect source-pinned serial 215/215 suite has passed for the **current** candidate may the agent label that source `SOURCE_TESTED`. Android, signed-in provider send and deployment gates remain independent.

## Other-agent handoff

Current application owner: ChatGPT Dev through GitHub `main`. ZCode is a bounded deterministic test observer; Desktop Commander supplies Windows timing/device evidence. No parallel editors to the same file. Report in original #33 and Labs `TASK-CLAIMS.md` when a genuine verifier run or source defect is recovered. This packet is a diagnostic receipt, not live agent notification or an automatic claim.

`TASK=Z01 | SOURCE_SHA | RUN_ID | NATIVE_SHA256 | RUNNER_SHA256 | TEST_CASE | EXPECTED | ACTUAL | ELAPSED | ISOLATED_PASS/FAIL | FULL_SUITE | HOST_EFFECTS=0 | PROVIDER_SENDS=0 | NEXT_ACTION`
