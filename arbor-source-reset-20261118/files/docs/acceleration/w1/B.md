# ARBOR Acceleration R002 — W1 Lane B: Host/Node24 handoff

**Decision: PREPARED_NOT_ACCEPTED.** No Node24 Host execution, browser/device acceptance, or keeper acceptance has been observed. Source changes are committed only to branch **work/accel-r002-w1-b-runtime-proof**. This report is not a keeper receipt and does not change the canonical queue.

## 1. Authority, pins, and write scope

- Common main and branch HEAD before changes: **2a42f1af025db8aacb965326b11c73da671ffb3f** (verified via GitHub branch reads).
- Last code/CI commit before this report: **4420899f338dbe1d297edd058fe83b7db7885d44**. This report adds another commit; its own commit is the final branch HEAD, reported separately to the owner.
- Only four paths are writable: scripts/acceleration/live-host-check.mjs; scripts/acceleration/live-host-check.test.mjs; .github/workflows/acceleration-r002-w1-b.yml; docs/acceleration/w1/B.md.
- No main edits, PR, merge, branch reset or changes outside Lane B.
- Inspected shared parallel README/order, canonical roadmap and acceleration companion, handoff/PDR exit, Host server and product journal, Wiki JS/HTML and seed, WIKI-BASELINE evidence script, test manifest, product tests, package manifests and W01-G workflow. Root and other-lane source remains read-only.

### Source deliverables

1. **Host runner:** one-command loopback Host, ephemeral private dataRoot, exact Git SHA/environment/Bun presence, per-route status/latency, real chat and Wiki write path, denial cases, CAS, journal byte agreement, restart replay, evidence child process with exact exit/timeout/stdout/stderr excerpts, public-demo journal digest comparison, and guaranteed temporary-root cleanup. A Host import failure is captured as a JSON failure receipt rather than escaping before the receipt.
2. **Runner tests:** assertions for Node24, real HTTP statuses, refusal matrix, restart persistence, direct JSONL parity, evidence verdict and isolation/cleanup.
3. **Workflow:** Node24, Git checkout, read-only GitHub permissions, no build hooks, branch-specific push trigger, three test commands with literal exit files and stdout/stderr artifacts; missing root lock is recorded without manufacturing one.
4. **This report:** exact evidence vs UNRUN, source ownership and integration handoff.

## 2. Runtime contract added — implementation, not observed acceptance

Run from a complete ARBOR checkout with Node24:

    node scripts/acceleration/live-host-check.mjs
    node --test scripts/acceleration/live-host-check.test.mjs
    node --test apps/vivim-host/test/product-journal.test.mjs apps/vivim-host/test/product-routes.test.mjs

The one-command runner starts the **actual** createHostServer({dataRoot, autoPump:false}) on 127.0.0.1 port 0. It probes GET /chat, /wiki, /demo, both screens' JS and CSS, /api/state, /api/wiki, /api/journal, /api/roadmap, /api/file-surface. It tests typed chat append, unknown-kind/schema refusal, cross-origin refusal, unauthorized/no-CSRF Wiki refusal, successful bounded Wiki CAS edit, stale/missing-revision and traversal refusal, raw JSONL/API equality, unknown route, invalid method and fresh-Host restart replay. A child Node process runs WIKI-BASELINE.mjs --live with the ephemeral base URL and explicit isolation flag, and its output is independently checked.

The receipt intentionally states browserDom=UNRUN, physicalDevice=UNRUN, independentKeeperAcceptance=UNRUN. No test code grants privileged effects, signs into providers, pairs a real device or edits user data. These are source-inspected claims, **not** runtime proof until executed.

## 3. Executed observations and literal exits

| Command/observation | Literal status | Evidentiary meaning |
|---|---|---|
| GitHub read/branch/compare, mandatory source inspection | Successful; base-to-code diff only 3 owned paths, 4 commits ahead, 0 behind | Source/ownership check; no Node test |
| V8 JS syntax parse on new code after adapting ESM import/meta for parse | PASS in tool-side parser; **no Node process exit** | Syntax only, not Host execution |
| Local node --version | **exit 0**, v22.16.0 | Node24 requirement NOT MET |
| Local bun --version | **exit 127**, command not found | Bun unavailable |
| Local checkout at ARBOR | **NOT PRESENT**; GitHub DNS unavailable from process container | Cannot start actual Host there |
| node scripts/acceleration/live-host-check.mjs | **UNRUN**, no process exit | Node24/full source checkout unavailable |
| node --test scripts/acceleration/live-host-check.test.mjs | **UNRUN**, no process exit | Same blocker |
| node --test apps/vivim-host/test/product-journal.test.mjs apps/vivim-host/test/product-routes.test.mjs | **UNRUN**, no process exit | Same blocker |
| GitHub Actions run **43649140538**, commit 4420899f338dbe1d297edd058fe83b7db7885d44 | completed, **startup_failure, 0 jobs** | No Node/Bun setup or test process launched |
| GitHub Actions run **43649027719**, commit 605cb57acd30514801fedf53236b5a38d6dc1edd | completed, **startup_failure, 0 jobs** | Earlier zero-job symptom |
| GitHub Actions run **43643059357**, baseline commit | completed, **startup_failure, 0 jobs** | Zero-job behavior predates Lane B |

**All required real HTTP/Node24 and browser/device tests remain UNRUN.** The GitHub startup_failure is not a test-process exit and must never be counted as pass or as a test failure. No Node24 stdout/stderr exists yet.

## 4. CI and source closure blockers

A GitHub contents read for root bun.lock on the branch returned **404**; no frozen dependency closure can currently be inferred. This is a source observation, **not a measured install failure**. The scoped workflow records the missing lock, does not generate it, and proceeds to the direct Node24 test path when a runner schedules. Actual import failures, if any, will be recorded by that runner. Root locks/manifests and repository-wide workflow recovery belong to build recovery Lane G.

GitHub Actions run payloads return path=BuildFailed, conclusion=startup_failure, jobs=0. Whether the cause is workflow parsing, repository configuration, scheduling, billing or another platform condition is **not established**. Do not modify the global W01-G workflow or claim the new B workflow executed. The local fallback is the three exact Node24 commands above.

## 5. Cross-lane integration and next frontier

- **Lane A / Host authority:** preserve the typed mutation contract, same-origin/control refusal, statuses 403/409/428, append-only private journal, fresh-revision compare-and-swap, and readProjection across restart. Rerun B on integration SHA rather than assuming branch-only evidence applies to A's changes.
- **Lane C / Wiki surface:** preserve real /wiki HTML, /wiki.js, /wiki.css as browser-consumable external assets. DOM and visual claims require a real browser; B's raw HTTP contract cannot certify those.
- **Lane D / Wiki evidence:** preserve source/API-only verdict and explicit isolated mode for WIKI-BASELINE; B does not own that script.
- **Lane G / build recovery:** resolve zero-job CI and missing lock/source closure in its own scope, supply a Node24 runner with exact exit artifacts; no edits to B-owned paths.
- **Keeper/integrator:** compare the four-path B diff, integrate serially with fresh A/C/D pins, execute Node24 Host CLI/tests and independent keeper checks, then explicitly accept/reject A01. Keep main unchanged until authorized integration.

Canonical roadmap remains A00/A01/A02 **in-progress, verification UNRUN, owner acceptance PENDING**; A03/A04/A05/A06/A07/A08 stay gated. Ready frontier remains independent A00/A01/A02 runtime verification and CI source-closure recovery, not autonomous downstream effects.

Subsequent-work acceleration: **NOT_YET_MEASURED**. Historical receipts and beta ledger contradictions are preserved.

**Final lane verdict: PREPARED_NOT_ACCEPTED / SOURCE_COMMITTED / NODE24_HTTP_UNRUN / CI_ZERO_JOBS / BROWSER_DEVICE_UNRUN.**
