# W06-G — Serial Integration Gate: Owner-Ready Beta Handoff

You are the **sole ARBOR Proposal 5 integration owner** for Wave 6. **DO NOT start while any W06 A–F worker is still active.** All six must return committed branches or an explicit BLOCKED report. Read `.integration/proposal5/09-EXECUTION-PLAYBOOK.md`, this wave's LAUNCH, all A–F report files and Proposal 5 beta gates. You alone may update the shared branch `integrate/proposal5-beta-r001`, root Cargo/Bun manifests/lockfiles, `integration/source-lock.json`, release manifests and CI. Never push to ARBOR main.

## Exclusive gate actions
1. Review six W6 worker reports, staged Windows installer, clean install and first-run tests, live supported provider conversation and physical paired PWA acceptance.
2. Confirm in-repo HQ JS index/search/context/wiki and HQKG graph work with donor checkouts/processes unavailable, plus source-scoped privacy and low-storage bounds.
3. Verify exact Host World/grants/journal backed-up dry-run, no duplicate effect writer, claim/uncertain recovery and reversible rollback; do not touch real private data without approval.
4. Publish final B01–B12/G00–G13 machine-readable evidence, accepted capability list, beta limitations, source SHA/manifest/license audit and outstanding unavoidable external needs.
5. If all hard gates pass, return READY_FOR_OWNER_GO with one explicit request for real-data cutover/publication authorization. Only after an explicit affirmative owner approval and verified backup may install/switch/cut over; never falsely say live migration done.

## Merge protocol
Pin the exact integration baseline SHA before merging. For each worker in A–F: verify branch base, HEAD, source rights, changed path ownership, actual tests and source evidence. Merge/cherry-pick serially, resolving conflicts through declared code owners. Generate root Cargo/Bun files/locks after merges, run combined tests, and do NOT accept test assertions from individual worker logs as sufficient when integration behavior could differ. Never overwrite concurrent worker commits.

## Acceptance / next-wave contract
READY_FOR_OWNER_GO on full staged beta acceptance and tested rollback; COMPLETE only after explicit owner-approved actual one-writer cutover and documented smoke. If physical device/provider or security gate unrun, report BLOCKED and do not claim functional beta.

Publish `integration/reports/W06-GATE.md` and update the appropriate gates in `integration/beta-gates.json` with exact merged SHA/commit list, command and exit code evidence, device/OS, source/capability status, security blockers, feature continuity, rollback and exact next-wave base SHA. If repairs remain, hand off precise domain assignments; no false GO, no new feature requests.
