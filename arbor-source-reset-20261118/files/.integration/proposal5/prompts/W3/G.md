# W03-G — Serial Integration Gate: Beta Journeys and Continuity

You are the **sole ARBOR Proposal 5 integration owner** for Wave 3. **DO NOT start while any W03 A–F worker is still active.** All six must return committed branches or an explicit BLOCKED report. Read `.integration/proposal5/09-EXECUTION-PLAYBOOK.md`, this wave's LAUNCH, all A–F report files and Proposal 5 beta gates. You alone may update the shared branch `integrate/proposal5-beta-r001`, root Cargo/Bun manifests/lockfiles, `integration/source-lock.json`, release manifests and CI. Never push to ARBOR main.

## Exclusive gate actions
1. Merge A–F only after reports; evaluate B01–B12 against actual Windows/phone/provider availability. Source-only/synthetic runs do not count as device/provider PASS.
2. Demonstrate safe Host current catalog read, one journaled approval/receipt case, bounded Node0 candidate/revision read, internal HQ source→search→context/wiki and TierFlow/phone status.
3. Verify a real signed-in Chrome provider conversation when owner session available; if unavailable, mark LIVE_LOGIN_BLOCKED and make final W6 acceptance dependent on it.
4. Build continuity matrix for previously working donor media/mission/voice/context/AppBuilder surfaces; preserve operational ones or explicitly scope-except with owner decision. Do not implement Labs design-only features.
5. Ensure no concurrent Host effect writer, no false available_now source status, no PWA offline effect grant, no external donor runtime paths.

## Merge protocol
Pin the exact integration baseline SHA before merging. For each worker in A–F: verify branch base, HEAD, source rights, changed path ownership, actual tests and source evidence. Merge/cherry-pick serially, resolving conflicts through declared code owners. Generate root Cargo/Bun files/locks after merges, run combined tests, and do NOT accept test assertions from individual worker logs as sufficient when integration behavior could differ. Never overwrite concurrent worker commits.

## Acceptance / next-wave contract
GO to red-team W4 when the integrated beta core is source-backed and runnable and all remaining external device/provider acceptance is precisely isolated; missing critical implementation still NO_GO.

Publish `integration/reports/W03-GATE.md` and update the appropriate gates in `integration/beta-gates.json` with exact merged SHA/commit list, command and exit code evidence, device/OS, source/capability status, security blockers, feature continuity, rollback and exact next-wave base SHA. If repairs remain, hand off precise domain assignments; no false GO, no new feature requests.
