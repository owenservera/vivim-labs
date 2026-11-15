# Closed-loop development architecture — dynamic workflows, no second engine

## One observable development loop

`INTAKE → SOURCE_LOCK → ROUTE → EXPLORE → SPECIFY → IMPLEMENT → VERIFY → REVIEW → REPAIR/REVERIFY → KEEPER_HANDOFF → NEXT_QUEUE`

Terminal states: `ACCEPTED`, `BLOCKED`, `STOPPED`, `FAILED`, `CANCELLED`; run statuses reported from ZCode not manufactured. The workflow should create the next queue only as a reviewable artifact; **never** activate its own next run on an infinite loop.

| Phase | Inputs | Agent/driver | Evidence | Branching |
|---|---|---|---|---|
| SOURCE_LOCK | task, root, Git HEAD | `world.run`/read-only Explore | hash, clean/dirty, branch | dirty or wrong repo = STOP |
| ROUTE | requirements, model shortlist | triage model and deterministic policy | selected role, rationale, cap | no permitted model = BLOCK |
| EXPLORE | pinned source | explorer | paths, callers, tests, confidence | unknown provenance = BLOCK |
| SPECIFY | findings | planner + reviewer | scope, acceptance, tests | scope mismatch = revise once |
| IMPLEMENT | approved bounded scope | implementer | changed files, commit, self-tests | outside scope = BLOCK |
| VERIFY | fresh SHA | independent verifier | literal commands/exits | no tests = UNRUN, no pass |
| REVIEW | diff + test receipts | skeptical reviewer | risk findings | high-risk = BLOCK |
| REPAIR | failed check and exact reproduction | original implementer | new revision, changed receipt | max 2 repair rounds |
| HANDOFF | evidence + diff | keeper | source lock, GO/PARTIAL/NO_GO | no automatic main merge |
| NEXT_QUEUE | accepted and blocked items | planner/keeper | small ranked tasks | only owner/keeper can activate |

## Authority and safety

- ZCode is the workflow scheduler, journal, UI, resume/amend authority. No extra Node service owns workflow state.
- The Git checkout owns source state. Use `git worktree add` only after confirming actual branch name, clean tree and disk headroom; model text never determines an executable command without a fixed allowlisted adapter.
- Single integration keeper owns convergence; lanes own disjoint paths. No independent worker writes roadmap/release acceptance.
- A read-only exploration agent may not modify code; work agents do not touch secrets, auth cookies, personal corpus, or external actions. Tool permissions must enforce this where supported; a persona instruction is **not** a sandbox.
- Independent tests and source locks outrank model claims. Agent reports are untrusted until checked against actual process receipts.
- No Playwright/Puppeteer and no provider API integrations **inside ARBOR**. This kit configures **ZCode itself** to call OpenRouter; ARBOR's runtime remains signed-in-web-only and Host-authorized.
- ZCode may require explicit confirmation or permissions; pause for required human approval rather than bypassing checks.

## Work budgets and fanout

Start with **3–4 concurrent asks** total, not the hardware maximum. For free models many `:free` variants allow roughly 50 requests/day for low-tier accounts and 20/min; a single agent can consume multiple calls per ask. Set a **global daily budget** (e.g. 35 calls, leaving headroom), a wall-clock run cap, max changed files (e.g. 12), max touched paths and max repair iterations (2). ZCode's shared governor reduces concurrent requests, but it does **not** guarantee daily cost/volume ceilings. If a hard daily cap is needed, enforce via OpenRouter key limits/provider quotas or an approved gate outside workflow scripts; do not present a prompt-only guard as enforcement.

## Failure conditions

- 402 or nonzero price: BLOCK (no paid fallback).
- 429/quota: STOPPED provider or retry only with bounded backoff on an eligible free model as permitted by provider and workflow; never spin.
- Model output malformed: retry a single formatting repair; if still malformed mark failed, don't infer truth.
- Check mismatch: provide a failing receipt and route once to original author; no self-verification as independent acceptance.
- Source changed since baseline: stop or amend via fresh source lock; no blind patch application.
- Conflict: keeper reconciliation, no forced checkout/reset.
- User cancellation: preserve run artifacts and stop effects.
- Missing local external tool/runtime: mark UNRUN and emit blocker; do not synthesize passing exits.

## Metrics and success gates

Per run: origin SHA, task intent, model IDs, role allocation, requests observed (if host reports), unique files touched, commits, test pass/fail/skip, elapsed, rerun attempts, accepted changes, token/cost if genuinely provided. Quality measured by independent test success and owner-visible behavior; **throughput by accepted tasks per real resource consumption**, not synthetic quality scores. Recommend paired A/B on a frozen task corpus for model selection: 10 diverse bugfixes, 10 features, 10 reviews, scored by deterministic tests, diff size, rework rate and model spend. Route changes only after verified improvement.

## R004 integration target

The adjacent ARBOR sprint remains running. This kit lands only in vivim-labs research folder, does not touch ARBOR, and supplies ZCode a reusable workflow to apply later in a separate authorized repo/branch. First pilot in a scratch repo, then ARBOR branch with explicit scope.