# L04 — VLANG + Visual Feedback + Gestures agent directive

**Lead:** Dev source owner + ZCode deterministic verifier. **Priority:** P0/P1. **Master issue tasks:** U06/U07/Z01 failure isolation first, then U08/U09/G02-G05..

## Mission
Fix current native/Host natural-language intent parity and safety regressions; connect approved commands to deterministic visual feedback and shared GPUI/mobile gesture semantics.

## Allowed scope
Observe exact-source VLANG tests, deterministic planner, Host approvals, VEC visual state and the existing Rust gesture reducer. ZCode is a scoped no-effect verifier, not a second product-code writer.

## Existing infrastructure to reuse (read/verify before launching)
- `ARBOR/.zcode/workflows/arbor-vlang-iterative-observers.dwf.ts`
- `ARBOR/.zcode/workflows/arbor-product-verification-parallel.dwf.ts`

## First work packet
Read native-verifier/latest.json and vlang-iterations/latest.json, capturing the tested SHA, named failing assertions and timeout vs actual test failure. Check if the existing VIVIM-VLANG-Observer is running; do not launch another copy. Produce a narrow reproducible defect for Dev.

## Non-negotiable stop boundaries
No direct unapproved Host effects, no synthetic tests passed off as physical proof, no concurrent observer runs, no new VLANG engine or bypass of the Host action approval/journal.

## Acceptance target
VLANG/SAFETY source-pinned suites all finish without timeout, native/Host decision parity verified, approved command visibly progresses through state and receipts, physical long-press/IME test recorded separately.

## Reporting and coordination
Read `../DIRECTIVE.md` and live #33/#32/#31, then current Labs `TASK-CLAIMS.md`; **never count a plan as a running agent**. Begin read-only; do not check out a source claim owned by another role. Follow the [handoff template](HANDOFF.md). Report in ERA 1/2/3 with real SHA and exact test/runtime evidence, unresolved blockers and a next action.
