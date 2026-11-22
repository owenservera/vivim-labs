# ARBOR PWA / Host — pinned post-push verification receipt

**Observed:** 2026-11-22 18:17–18:22 UTC  
**Role:** local read-only verifier using a previously committed cloud/GitHub source checkout; not a product source coder  
**Device:** DESKTOP-9KB03LU  
**Tested commit:** `10006454b433efd53affc74e5e7a3d6518f96432`  
**Source checkout:** `C:\0-BlackBoxProject-0\.arbor-pwa-releases\10006454b433` (clean git status, exact SHA)  
**Privacy:** raw test outputs, command details and all receipts stay in the existing Git-ignored private local archive; only reviewed counts and receipt paths here.

## 1. Why this checkout
- GitHub `main` was initially observed at `37984694509c3d9a63919a7dab87a812f3937a70`; GitHub compare showed it was **one documentation-only commit ahead** of the tested checkout. At that point code and tests in the checked source were identical to `main`.
- During verification, other cloud coding sessions advanced `main` to `87143c69a587d1456d01b148735d37785ad0a7f4`: **12 commits ahead** of tested SHA with changes in `apps/mobile-relay/app.js`, Wiki test files, Host read-port/server, GPUI source and others. Therefore **the green test results below do not certify this later HEAD**.
- The most recently observed isolated PWA release checkout `e64cc59c5623a08527b814cf9ba15b42189c81a5` was still 10 commits behind `87143c6` and was not used as a surrogate for unverified latest source.
- Do not modify product code or the currently running Host/GPUI launch configuration to satisfy this test report. A cloud/GitHub-sourced immutable release for the new HEAD should be created through the existing release workflow and reverified.

## 2. Existing exact-SHA targeted suites — PASS
`node --test --test-concurrency=1` directly on the real shipped source code (no copied mock source), with separate private stdout and SHA-256 receipt.

| Source test files | TAP result | Process | Private receipt location |
|---|---|---|---|
| `apps/mobile-relay/test/{wiki-live-context,pwa-asset-graph,service-worker,first-customer}.test.mjs` | **29 tests, 29 pass, 0 fail, 0 skipped** | exit 0 | `validation-runs/20261122T1817361712678Z-ARBOR-PWA-WIKI-OFFLINE-R001-3f9f4360/receipt.json` |
| `apps/vivim-host/test/direct-phone.test.mjs` + `apps/mobile-relay/test/agent-composer-browser.test.mjs` | **14 tests, 14 pass, 0 fail, 0 skipped** | exit 0 | `validation-runs/20261122T1818353729994Z-ARBOR-HOST-DIRECT-PHONE-COMPOSER-fd63c0d3/receipt.json` |

**Targeted total:** 43 test assertions across these separately invoked suites. Counts may overlap with the canonical runner below; never add them together as distinct coverage.

## 3. Existing canonical PWA/Host runner — PASS
The repository's own `integration/scripts/pwa-run-now.mjs` at tested SHA was executed with Node 24 using the preexisting full-output private recorder. No duplicate test harness, provider call, branch, build deployment or source edit.

- Runner exit code: **0**
- Runner schema: `arbor.pwa-node-test-result/1`
- Runner commands: **25 / 25 passed, 0 failed**
- Components: **22 source syntax checks** and **3 official grouped test invocations**
- TAP suites: **196 tests, 196 passed, 0 failed, 0 cancelled, 0 skipped**
- TAP breakdown: **162 mobile/Build/offline**, **22 Host/phone/auth**, **12 media/revocation**
- Process raw stdout bytes: **23,499**, stderr bytes: **0**
- Full private receipt: `validation-runs/20261122T1819301452344Z-ARBOR-PWA-CANONICAL-CI-R001-f4e3dd44/receipt.json`
- Private derived code verification: same folder, `code-verification-summary.json`, schema `vivim-pwa-canonical-code-verification-v1`, status `PASS`
- Verified source-pinned receipt parser: Labs `scripts/summarize-pwa-verification-receipt.ps1`, checks official runner JSON, counts TAP assertions, source SHA and raw stdout hash, and explicitly leaves visual/device proof `NOT_CHECKED`.

## 4. Cloud CI status, kept distinct from source test result
GitHub Actions recent `main` runs were marked `startup_failure` **before any jobs were created**. For example run [45053550538](https://github.com/owenservera/ARBOR/actions/runs/45053550538) at `37984694509c3d9a63919a7dab87a812f3937a70` had `status=completed`, `conclusion=startup_failure`, jobs=`[]`. This cannot be counted as code test failure or green CI. The local runner gives test evidence at the pinned tested SHA, but cloud CI itself remains non-green. Root cause not diagnosed by this read-only pass.

## 5. Not tested and next gated handoff
- **Latest source (at least `87143c6`)**: revalidation **REQUIRED** because meaningful PWA/Wiki/Host/GPUI files changed after `1000645`.
- **GPUI visible typing/menu/screenshot**, live Wiki readback, PC ↔ phone draft synchronization and physical Android input: **UNRUN by this pass**. Node tests are not screen evidence.
- **CI startup failure:** inspect GitHub Actions workflow configuration/account capacity/runner startup separately without guessing cause; do not mark product tests as failed.
- **Owner handoff:** canonical ARBOR [100-step surface ledger](https://github.com/owenservera/ARBOR/blob/main/docs/product/CANONICAL-SURFACE-100-STEP-LEDGER.md) remains product authority. Current ChatGPT Dev cloud source owner should repin a clean verification release at then-current HEAD, rerun `node integration/scripts/pwa-run-now.mjs` under the private full-output recorder, and perform native GPUI/phone acceptance separately. This read-only observability lane does not overwrite that owner's task claim.

**Outcome:** Code-level validation **PASS for tested SHA only**, new head **UNRUN**, cloud CI **startup_failure with zero jobs**, physical acceptance **UNRUN**.

## 6. Subsequent fresh-source check and guarded local verification attempt

- A follow-up GitHub branch-ref check observed `main=384c6368a44d9934b9fce112b29c6e3e952a08eb`, **18 commits ahead** of the fully tested `10006454` SHA, with **18 changed files**, including mobile Wiki/PWA source, Host read port/server, Oracle Wiki test, and native GPUI source. This later HEAD remains **UNRUN**, not covered by the 196-test PASS.
- Local immutable snapshot inventory found `e64cc59c5623a08527b814cf9ba15b42189c81a5` to be the newest nearby existing release, but it was already behind the earlier `87143c6` current main. No test was attributed to a source version that wasn't actually checked out.
- A read-only preflight for a possible new isolated verification worktree was refused by Desktop Commander's PowerShell blocked-command validator: `the check for blocked commands failed (PowerShell did not answer in 5 seconds), so nothing was run`. **No checkout was created, no code was edited, no CI run was initiated and no product effect occurred**. Do not attempt to bypass the safety preflight. This is a local tool infrastructure blocker, not a failed ARBOR source test.
- **Handoff next step:** the existing cloud/GitHub Dev owner should select and pin one final source revision, produce an immutable verification checkout through the normal approved release workflow, and rerun the same canonical suites using the private receipt recorder. Parallel new commits require new SHA-bound test acceptance, but do not invalidate the integrity of older receipts.
