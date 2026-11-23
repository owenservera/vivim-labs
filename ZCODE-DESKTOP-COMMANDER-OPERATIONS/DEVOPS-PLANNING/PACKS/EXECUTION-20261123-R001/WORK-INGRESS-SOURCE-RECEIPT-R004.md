# PY-001 Dev source implementation + 43-case exact-SHA receipt — 23 November 2026

**Status: CODED/PUSHED/TESTED for protected source seams; not deployed/Host Work created/phone verified.**
**Exact ARBOR SHA:** `a34a7f5a88afb687f86fc9fba79a7e886052f48c` (Windows isolated detached worktree, unmodified owner checkout). **No ARBOR app source written by Desktop Commander**; GitHub connector committed directly on `main`. This is a companion to [PY-001 R003 contract](PWA-BUILD-INTENT-R003.md), not another product source or task ledger.

## Product code advanced on ARBOR main

| New/reused file | Result | Boundary |
| --- | --- | --- |
| [`apps/vivim-host/control/pwa-work-admission.mjs`](https://github.com/owenservera/ARBOR/blob/main/apps/vivim-host/control/pwa-work-admission.mjs) | **NEW** explicit Build-to-Work Host admission policy | No HTTP route; requires injected incumbent Host reviewed owner action, source-backed `pwa.view` revision/digest, identity/grant, existing Work CAS; unknown writer outcome = uncertain with no retry |
| [`apps/vivim-host/test/pwa-work-admission.test.mjs`](https://github.com/owenservera/ARBOR/blob/main/apps/vivim-host/test/pwa-work-admission.test.mjs) | **NEW** 13 actual tests | silent draft/IME/preview, forged grants, stale revisions, owner approval, CAS, no second item, no provider sends |
| [`apps/vivim-host/onboarding/studio-port.mjs`](https://github.com/owenservera/ARBOR/blob/main/apps/vivim-host/onboarding/studio-port.mjs) | **REPAIR** existing Host-private Studio port; adds `workStatus` and `workCreate` | Reuses `GET /api/v1/personal-agent/status`, `POST/GET /api/v1/personal-agent/work`; one POST, exact response + stored readback, local scope fingerprint, no alternate store, no new Studio/Chrome worker |
| [`apps/vivim-host/test/pwa-work-studio-port.test.mjs`](https://github.com/owenservera/ARBOR/blob/main/apps/vivim-host/test/pwa-work-studio-port.test.mjs) | **NEW** 10 tests | exact Studio route/token/CAS, unknown post-response, no implicit corpus bootstrap, integrated existing Work reducer |
| [`apps/vivim-host/server.mjs`](https://github.com/owenservera/ARBOR/blob/main/apps/vivim-host/server.mjs) | **REPAIR** an *owner-only GET* `/api/host/control/work-status` after current Host owner/session authorization | No mutations or phone/private data; uninitialized is returned explicitly, Studio outage sanitized; owner revocation rechecked after await |
| [`apps/vivim-host/test/pwa-work-host-status.test.mjs`](https://github.com/owenservera/ARBOR/blob/main/apps/vivim-host/test/pwa-work-host-status.test.mjs) | **NEW** 5 actual Host HTTP tests | unauthorized/wrong Origin/non-GET zero Studio reads, normal uninitialized/readiness, 503 unavailable |
| `studio/test/unit/pwa-work-ingress-reuse.test.mjs` | **REUSED**, previously authored by another agent | 6 tests on incumbent `personal-agent-work.mjs` pure state reducer, not a live PWA writer |

**Important:** `workCreate` is a Host-private adapter method, **not** an admitted public endpoint. The new admission service requires a genuinely **existing Host-reviewed** work action; this action's *production* approval producer is **not wired yet**. The owner-only Work status GET has real source tests but was **not promoted to the live Host**. No app.js, provider route, Chrome session, Rust kernel, device grant implementation, task bus or data root was replaced.

## Actual test receipt — source exactly a34a7f5a

Run on connected owner Windows machine in an isolated detached Git worktree, Node v24.22.1, **not** the live runtime:

```powershell
node --check apps/vivim-host/server.mjs
node --test --test-concurrency=1 apps/vivim-host/test/pwa-work-admission.test.mjs apps/vivim-host/test/pwa-work-studio-port.test.mjs apps/vivim-host/test/pwa-work-host-status.test.mjs studio/test/unit/pwa-work-ingress-reuse.test.mjs
# 33 tests | 33 passed | 0 failed | exit 0

node --test --test-concurrency=1 apps/vivim-host/test/studio-operations.test.mjs apps/vivim-host/test/studio-approved-actions.test.mjs
# 10 tests | 10 passed | 0 failed | exit 0
```

**Total independent targeted+existing compatibility = 44/44 PASS.** First narrow attempt at older SHA `6cbb6e7` was 11/12 (test expected uncertain for a **definite** key collision); corrected assertion only at `f44ae9b5`, 12/12 then 22/22 at `358bd923`, final 33+10 at `a34a7f5a`. No tests removed, no asserts relaxed, no source edited on PC, no provider sends, no Host effect, no live/physical phone check. Old deterministic 215-case ZCode receipt **does not include the new 43-case suite**.


**Latest safety iteration:** [ARBOR `7f7825d`](https://github.com/owenservera/ARBOR/commit/7f7825d72c189516902562484014d61055a4ef25) distinguishes Studio `STUDIO_WORK_NOT_READY` / pre-POST unavailability as definite `WORK_STORE_NOT_READY`; [test `a34a7f5`](https://github.com/owenservera/ARBOR/commit/a34a7f5a88afb687f86fc9fba79a7e886052f48c) adds the negative case and independently preserves `WORK_COMMIT_UNCERTAIN` for uncertain post-send outcomes. Full exact-SHA six-file combined Node run: **44/44 PASS**, 0 fail, exit 0. These assertions were not relaxed.

## Real owner-PC observation (independent of isolated tests)

A single **read-only GET** to the existing *live* Studio `http://127.0.0.1:38730/api/v1/personal-agent/status` returned HTTP **200**, `initialized:false`, `scope:local-data-directory`, `version:null`. No private data printed, no POST or bootstrap. **This is an actual blocking source truth**: existing Studio Personal Agent local Work persistence is not initialized in the currently observed live identity. Never initialize it as a side effect of loading Build or PWA/GPUI.

## Exact next integration work (not accepted or deployed)

1. **Host owner-review producer (PY-002):** attach real `personal.work.create` to incumbent Host Control plan/approval/journal, or identify a demonstrably equivalent reviewed action; bind its true actor/grant/source-digest receipt to `createPwaWorkAdmission` with no client-forged approval. The new module intentionally refuses without this producer. Consider Host sole effect authority; don't make Studio a second effect owner.
2. **Explicit user action (PY-001):** PWA Build's existing `Preview change / Activate` path remains for visual changes and separate hidden provider-design send. Add an explicit, distinct Submit Work button via source owner after reconciling old `app.js` claims. Typing/voice drafts and preview remain inert.
3. **Owner-approved Personal Agent initialization (PY-002 dependency):** use existing Studio bootstrap stage only after an explicit owner decision/current identity and Host route policy; current live status is `uninitialized`. Do **not** silently call initialization from a page load, PWA event or worker.
4. **Phone work readback (PY-004):** existing LiveSystemState and paired device scopes, no direct private Studio API access on the phone. Distinct SOURCE→TEST→PACKAGE→HOST→GPUI→ANDROID steps per #31.
5. **Dispatch (PY-005):** default `not-dispatched`/MANUAL until a separately authorized ZCode/ChatGPT coding session has a verified current claim, healthy cost-approved model and durable ACK. No background worker started.

**Observed verdict:** CODED/PUSHED and **44/44 tests PASS** at exact 2a518; OWNER-ONLY LIVE ROUTE DEPLOYMENT **UNRUN**; LIVE WORK INITIALIZATION **ABSENT**; PWA Submit **UNWIRED**; Host-review action **UNWIRED**; real Work POST **NOT PERFORMED**; phone ACK **UNRUN**; providerSends **0**; live sourceWrites **0**. 

**Handoff:** Dev retains the active PY-001 claim until the next scope/producer is accepted and implemented. Oracle can validate the evidence gates; Planning can work on nonoverlapping preparation. See [canonical Task Claims](../../../TASK-CLAIMS.md) and [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33).
