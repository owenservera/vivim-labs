# W01-G — Serial Integration Gate: Physical In-Repo Landing

You are the **sole ARBOR Proposal 5 integration owner** for Wave 1. **DO NOT start while any W01 A–F worker is still active.** All six must return committed branches or an explicit BLOCKED report. Read `.integration/proposal5/09-EXECUTION-PLAYBOOK.md`, this wave's LAUNCH, all A–F report files and Proposal 5 beta gates. You alone may update the shared branch `integrate/proposal5-beta-r001`, root Cargo/Bun manifests/lockfiles, `integration/source-lock.json`, release manifests and CI. Never push to ARBOR main.

## Exclusive gate actions
1. Read six W1 worker reports, check branch base match, import ancestry and no forbidden secrets/entire donor Git history; reject unreviewed bulk sync.
2. Integrate all selected Host, Studio/harness, mobile PWA, native Tauri/core/HQKG, HQ operational JS and TierFlow source physically in ARBOR; preserve relative paths and transitive source closure.
3. **You alone** create/update root Cargo.toml/Cargo.lock/rust-toolchain.toml, Bun package workspace/lock and initial release/runtime manifest. Resolve exact package names, build.rs and runtime dependencies.
4. Run import/build smoke with donor processes stopped and source path lint; no external ../repos-hq or ../tierflow path/git dependency. Third-party locked dependencies acceptable during build.
5. Produce honest compile/test status and source ownership. Missing required source or broken no-donor path is blocker, not an optional improvement.

## Merge protocol
Pin the exact integration baseline SHA before merging. For each worker in A–F: verify branch base, HEAD, source rights, changed path ownership, actual tests and source evidence. Merge/cherry-pick serially, resolving conflicts through declared code owners. Generate root Cargo/Bun files/locks after merges, run combined tests, and do NOT accept test assertions from individual worker logs as sufficient when integration behavior could differ. Never overwrite concurrent worker commits.

## Acceptance / next-wave contract
GO when internal import and minimal reproducible Cargo/Bun workspace exist with source closures and no outside VIVIM runtime; W2 can repair internal build seams, but missing imports/secrets/owner ambiguity block progression.

Publish `integration/reports/W01-GATE.md` and update the appropriate gates in `integration/beta-gates.json` with exact merged SHA/commit list, command and exit code evidence, device/OS, source/capability status, security blockers, feature continuity, rollback and exact next-wave base SHA. If repairs remain, hand off precise domain assignments; no false GO, no new feature requests.
