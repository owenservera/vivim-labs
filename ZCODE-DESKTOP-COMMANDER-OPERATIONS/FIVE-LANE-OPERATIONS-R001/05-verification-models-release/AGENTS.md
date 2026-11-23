# L05 — QA, ZCode Admission + Release Evidence agent directive

**Lead:** Planning/QA coordinator + Desktop Commander observer. **Priority:** P0/P1. **Master issue tasks:** Z01/Z02/Z03/Z07 and R04 first; other R/Z tasks once blockers close..

## Mission
Keep exact-SHA verification, model admission, CI startup investigation, source/claim reconciliation and release evidence coherent without creating a second orchestrator or scheduler.

## Allowed scope
Labs-only evidence and coordination by default; read-only Windows/ZCode tests may be run in bounded safe windows, original Dev keeps all ARBOR source changes unless explicitly handed off. Reuse existing hourly ChatGPT dev automation and ZCode model/router infrastructure.

## Existing infrastructure to reuse (read/verify before launching)
- `ARBOR/.zcode/workflows/arbor-product-verification-parallel.dwf.ts`
- `ARBOR/.zcode/workflows/arbor-area-a.dwf.ts`

## First work packet
Refresh ARBOR #33, #24, current Labs claim registry, local observer receipt and actual ZCode dwf_run process/event metadata. Distinguish workflow definitions from running agents, count only tested permitted model seats, and identify one highest-impact unblocker.

## Non-negotiable stop boundaries
Do not invoke arbor-life auto restart; do not repeat 429 probes, spend on unapproved models, start duplicate scheduler, mutate SQLite, publish raw secrets or mark Actions startup_failure as code FAIL.

## Acceptance target
Every reported PASS has current SHA/command/exit/source receipt; a model worker is called active only with runId, alive process, events and output; CI jobs truly execute; packaged/runtime/device stages separately attested.

## Reporting and coordination
Read `../DIRECTIVE.md` and live #33/#32/#31, then current Labs `TASK-CLAIMS.md`; **never count a plan as a running agent**. Begin read-only; do not check out a source claim owned by another role. Follow the [handoff template](HANDOFF.md). Report in ERA 1/2/3 with real SHA and exact test/runtime evidence, unresolved blockers and a next action.
