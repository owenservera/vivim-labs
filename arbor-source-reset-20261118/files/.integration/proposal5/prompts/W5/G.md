# W05-G — Serial Integration Gate: Bounded Fixes and Release Candidate

You are the **sole ARBOR Proposal 5 integration owner** for Wave 5. **DO NOT start while any W05 A–F worker is still active.** All six must return committed branches or an explicit BLOCKED report. Read `.integration/proposal5/09-EXECUTION-PLAYBOOK.md`, this wave's LAUNCH, all A–F report files and Proposal 5 beta gates. You alone may update the shared branch `integrate/proposal5-beta-r001`, root Cargo/Bun manifests/lockfiles, `integration/source-lock.json`, release manifests and CI. Never push to ARBOR main.

## Exclusive gate actions
1. Merge only assigned W5 fixes, keep one Cargo/Bun release state and no unrelated capabilities; rollback unreviewed feature churn.
2. Rerun each finite W4 blocker reproducer and relevant original Host/HQKG/HQ/TierFlow/PWA/Studio test suites after integration, not isolated worker tests only.
3. Run independent donor-removal release build, no secrets/license/policy audit, root dependency closure, native/Host effect source truth and low-storage quota tests.
4. Verify no P0 remains in origin/host identity/session source privacy, duplicate effect writer, no-donor runtime, restart/recovery or data loss. Document physical device/live-provider tests still pending W6.
5. Create staged beta release candidate and version/compatibility manifest; do not publish or migrate real owner data.

## Merge protocol
Pin the exact integration baseline SHA before merging. For each worker in A–F: verify branch base, HEAD, source rights, changed path ownership, actual tests and source evidence. Merge/cherry-pick serially, resolving conflicts through declared code owners. Generate root Cargo/Bun files/locks after merges, run combined tests, and do NOT accept test assertions from individual worker logs as sufficient when integration behavior could differ. Never overwrite concurrent worker commits.

## Acceptance / next-wave contract
GO_FOR_W6 only if staged ARBOR is independently buildable, critical P0 fixes truly pass, no-donor check works and remaining W6 work is physical acceptance/data cutover drill. Otherwise REPAIR_REQUIRED, stay in W5.

Publish `integration/reports/W05-GATE.md` and update the appropriate gates in `integration/beta-gates.json` with exact merged SHA/commit list, command and exit code evidence, device/OS, source/capability status, security blockers, feature continuity, rollback and exact next-wave base SHA. If repairs remain, hand off precise domain assignments; no false GO, no new feature requests.
