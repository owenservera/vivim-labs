# ARBOR LIFE R002 — Maximum Development Throughput Steering

**Date:** 2026-11-17  
**Status:** Owner-directed execution steering; not a claim of implementation or acceptance  
**Repository:** `owenservera/ARBOR` · `main`  
**Baseline inspected:** `922ca4e06db93f3b3a727cebe83d655ee196c095`  
**Local workspace:** `C:\0-BlackBoxProject-0\ARBOR`

## ERA 1 — Facts, constraints, and the actual bottleneck

**Objective:** maximize **independently verified, accepted application improvements per hour**, not agent count, model calls, phase count, or raw test count. Preserve all completed work; optimize the existing running `arbor-life` workflow instead of creating another fleet.

The 2026-11-17 briefing in `.coordination/pdr/chatgpt-workflows-explainer.md` reports:

- First ARBOR LIFE lineage completed three green rounds on A00/A01/A02, with recorded gate exit 0, static/live evidence exit 0, journey replays, 9/9 route rewalks, and keeper handoffs. These are reported evidence, not independently rerun by this steering document.
- Development cycles take approximately **20–40 minutes**, against the design's ≤5-minute aspiration for small fixes.
- Recorded mean generation durations: **ERNIE 5.5 Flash 4.6 seconds (181 calls)** and **NIM Mistral Large 4 37.6 seconds (171 calls)**. Provider-specific averages may measure different generation behavior and do not alone establish coding success.
- Multiple `AmendWorkflow` iterations per lineage, provider stalls, and repeated summary/verification steps consume wall time.
- GLM is retired from this routing setup because of reported Z.ai quota error 1005.

**Critical acceptance contradiction:** `docs/product/roadmap/receipts/A00.receipt.json`, `A01.receipt.json`, and `A02.receipt.json` themselves describe shared-Host mutations against port `43187` while their cited isolation contracts require no such effects. A00 records three mutations; A01 and A02 record two each. Some browser observations are also labelled `NOT VERIFIED` in the receipts. Preserve passing deterministic evidence, but do **not** treat the contradictory claims as full isolation/browser acceptance. Correct only the affected acceptance surface, not the completed A00–A02 implementation.

**Queue drift:** canonical `roadmap.v1.json` at the inspected SHA shows A00/A01 `verification: UNRUN`, A02 `STATIC_LIVE_PASS_EXIT0`, and owner acceptance still pending. The read-only `scripts/roadmap/acceleration-queue.mjs` selects `queued|in-progress` items only when dependencies meet its explicit independent-acceptance predicate. Consequently the observed queue has no readily actionable build item through that selector. Reconciliation belongs to the existing keeper, with no unilateral approval or relaxed predicates.

**Infrastructure already present:** `arbor-life`, `arbor-build`, `arbor-demo`, `arbor-batch-engine`, Host journal, roadmap, receipts, `scripts/test-gate.mjs`, change-set tests, dependency cache, per-item evidence runners, native ZCode browser control and isolated scratch-Host test patterns. Reuse them. GitHub Actions still reports `startup_failure` at inspected `main`; a passing local gate does not establish passing CI.

## ERA 2 — Six practical throughput optimizations

### 1. Two-speed LIFE execution, with FAST as default

**FAST** for a small source-pinned issue: deterministic selection → one lightweight diagnosis → one scoped fixer if required → targeted tests → fresh real browser replay against isolated Host → independent acceptance → keeper handoff. Target **3–8 minutes** on sufficiently small issues; ≤5 minutes is an aspiration, not a universal timeout.

**DEEP** only for cross-system architecture, security-sensitive changes, uncertain diagnosis, contradictory evidence, or two failed bounded repair attempts. Escalate to specialists only when the initial path cannot settle the issue. Do not assemble an eight-agent team for a one-file fix.

### 2. Right-size model routing based on measured success and latency

- Feeder selection, dependency checks, dedup, exit-code parsing and evidence comparison: **deterministic code, no LLM**.
- User-journey planning and simple diagnosis: begin with **ERNIE 5.5 Flash** based on observed latency; validate quality.
- Scoped code repair: benchmark ERNIE against healthy faster NIM alternatives on representative tasks, including success and retry costs. Do not simply assume fastest generation means fastest correct fix.
- Complex reasoning: reserve **Mistral Large 4** or another verified specialist for deep problems.
- Verification: deterministic gates + an independent browser/Host oracle; a model is used only for decisions that cannot be coded reliably.
- Health-probe models, avoid dead GLM quota and unhealthy fallback routes. Limit concurrent model calls to demonstrated provider capacity.

### 3. Keep one long-lived coordinator; minimize amendments

Use the existing LIFE coordinator, its journaled execution, resume and stable identities. Handle normal test failures *inside* the cycle with bounded retries. Use `AmendWorkflow` only for actual workflow-definition corrections or eligible recovery. On genuine amendment, explicitly preserve/replicate saved args because existing definitions document an argument-loss failure mode.

Cached completed workflow steps and gate proofs are valid only for their original source/worktree, environment and inputs. Never reuse a prior acceptance result after code changes. Persist checkpoints and ownership leases so restarts do not cause duplicate effects or rework.

### 4. One thin roadmap feeder, one canonical queue

Reuse `scripts/roadmap/acceleration-queue.mjs`, `docs/product/roadmap/roadmap.v1.json`, accepted receipts and existing claim/release mechanisms. The feeder should only:

1. Read canonical status, relevant receipts, source pins, dependency state and active claims.
2. Skip already completed/claimed/duplicate items.
3. Select one truly authorized eligible item and issue a compact work order.
4. Send source-pinned outcomes to the **existing keeper** for authoritative receipt/queue updates.
5. If no item qualifies, emit the exact blocker **once**; notify keeper and allow only bounded authorized read-only exploration meanwhile.

No roadmap regeneration, second scheduler, duplicate message bus, speculative queue flips, rewritten historical receipts, or automatic owner approval.

**Dedup key:** stable issue identity = affected capability + normalized failure condition + expected behavior + scenario identity; separately attach execution identity = baseline source SHA + dirty-worktree digest + environment fingerprint + relevant journal/Host revision + test inputs. Dedup stable issues across restarts while invalidating stale execution evidence when inputs change.

### 5. Execute the smallest safe checks; preserve independent acceptance

- Coding iteration: relevant unit/component tests first.
- Candidate fix: repeat the **same failed UI journey** with fresh actions on an isolated scratch Host.
- Pre-handoff: canonical `bun scripts/test-gate.mjs` with its own change selection, dependency fingerprint, and proof cache; do not bypass fail-closed full-suite triggers.
- Acceptance: independent verifier confirms browser-visible behavior, intended Host-state/journal changes, source-revision binding, and relevant adversarial negative assertions.
- Keeper consumes existing valid, matching receipts instead of recomputing everything.

Avoid touching global files such as `scripts/test-manifest.json` unnecessarily; its change forces broad test selection. Never weaken the suite or turn missing browser evidence into a PASS.

### 6. Overlap only independent work

Keep **one exclusive code writer per overlapping file scope**. Allow read-only exploration and already-safe deterministic checks to run concurrently with scoped development when the shared runtime and machine resources permit it. Perform independent verification after a stable candidate is available. Do not create competing Host effect writers, duplicate LIFE coordinators, or file clobbers. Start with low concurrency and raise only after measuring provider/machine utilization.

## ERA 3 — Direct ZCode execution order

> **ARBOR LIFE R002 — MAXIMUM DEVELOPMENT THROUGHPUT**
>
> **Mode:** HIGH EFFORT · IMPLEMENT NOW · NO RESTARTS  
> **Workspace:** `C:\0-BlackBoxProject-0\ARBOR`  
> **Branch policy:** existing `main`, no new branches or PRs, preserve unrelated changes.
>
> **Mission:** improve the existing `arbor-life` system's rate of genuinely finished, independently verified PWA changes. Do not redesign LIFE, create another fleet, launch historical workflows, regenerate the roadmap, duplicate source surveys or repeat settled engineering.
>
> 1. **Minimal source delta inspection:** check local HEAD, dirty tree, active ZCode runs, existing LIFE definition and runtime state, queued claims, keeper handoffs, provider usage, and receipts. This document was authored against GitHub HEAD `922ca4e06db93f3b3a727cebe83d655ee196c095`; trust a newer local HEAD if present and inspect only the relevant delta. Do not interrupt existing sessions or claim a process state without inspecting it.
> 2. **Repair the acceptance boundary narrowly:** isolate all simulated-user mutations from the shared Host on `43187`; reuse existing scratch Host/data-root tooling. Preserve original A00–A02 receipts as historical artifacts and identify the specific acceptance claims requiring correction or supplemental proof. Do not repeat their implementation. Route adjudication to keeper.
> 3. **Fast/deep path:** implement a default small-fix FAST path within existing LIFE, with DEEP escalation only for stated complexity/repair triggers. One lightweight diagnoser and one scoped writer as necessary. Preserve independent verification.
> 4. **Provider optimization:** instrument actual generation time, retries, provider wait and task success by role. Benchmark fast-eligible repair tasks on healthy models; route deterministic operations without model calls; reserve slow strategic models for complex work. Never use dead GLM quota.
> 5. **Stop amendment churn:** keep the coordinator alive; retry normal tool/test problems within the current cycle. Respect ZCode journals, idempotent commands, leases, saved args and source-revision cache invalidation.
> 6. **Thin feeder only:** consume the existing `readyFrontier`, receipts and claims. Never rewrite roadmap authority. Escalate zero-frontier/drift to keeper once. Submit a deduplicated candidate for novel UI discoveries only via the existing approval path.
> 7. **Shorten the gate path:** targeted tests during edits, the existing canonical gate before handoff, and independent isolated browser replay. No redundant full-suite runs, false-green checks or unpinned cached acceptance.
> 8. **Pipeline safe work:** one scoped code writer, optional read-only explorer, independent verifier. No conflicting writes or extra Host authority. Keep source effects and approvals constrained.
> 9. **Resume actual eligible work:** after the smallest throughput patch, continue from the next authorized existing queue item. If no item is eligible, address the precise keeper/evidence blocker or perform bounded read-only PWA discovery. No fabricated approvals.
> 10. **Measure and report:** per-cycle issue identity, source SHA + worktree digest, FAST/DEEP classification, total wall time, phase durations, model/provider waits, model calls, tests, fresh browser replay, independent verdict, keeper disposition, attempts, retries, tokens/cost where measurable and actual verified improvements/hour.
>
> **Targets:** 2× initial improvement in verified throughput, eventually 4× if measured; ≤5-minute median for properly scoped small fixes; zero intentional duplicate development, unauthorized Host mutations, false-green acceptance, or competing writers. These are targets—not claims or guaranteed outcomes.
>
> **Finish line:** a faster *existing* LIFE runtime with executable improvements, source-pinned proof, original-work preservation and measured throughput improvement. Do not substitute a plan or verbose documentation for the engineering work.
>
> **Proceed immediately; do not restart completed work.**

## Evaluation rubric

| Metric | Baseline / target | Interpretation |
|---|---|---|
| LIFE end-to-end cycle | Recorded ~20–40 min | Verify apples-to-apples by task class |
| FAST small-fix median | Target ≤5 min | Not applicable to cross-system work |
| Verified improvements/hour | Target 2× initial, 4× aspirational | Count only accepted source-pinned improvements |
| Repair/amend churn | Reduce materially | Track retries/amends per accepted issue |
| Duplicate development | Target 0 | Do not rebuild completed tasks |
| Shared Host mutations in simulated tests | Target 0 | Isolation is mandatory |
| Contradictory accepted evidence | Target 0 | Fail closed, refer to keeper |
| CI `startup_failure` | Open operational blocker | Track separately from local test gate |

## Source map for local execution

- `docs/WORKFLOWS.md`
- `docs/design/realtime-dev-loop.v1.md`
- `.coordination/pdr/chatgpt-questions.md`
- `.coordination/pdr/chatgpt-workflows-explainer.md`
- `docs/product/roadmap/roadmap.v1.json`
- `docs/product/roadmap/receipts/A00.receipt.json`
- `docs/product/roadmap/receipts/A01.receipt.json`
- `docs/product/roadmap/receipts/A02.receipt.json`
- `scripts/roadmap/acceleration-queue.mjs`
- `scripts/test-gate.mjs`
- `docs/research/nim-models/NIM-MODEL-ROUTING-GUIDE.md`
- Locally saved ZCode workflow definitions in `.zcode/workflows/` (normally git-ignored)

**Implementation boundaries:** this file is steering only. It does not alter LIFE, a queue entry, a receipt, the Host, or any running ZCode workflow.
