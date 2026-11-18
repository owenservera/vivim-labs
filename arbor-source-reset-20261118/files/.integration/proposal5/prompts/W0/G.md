# W00-G — Serial Integration Gate: Source Truth Freeze

You are the **sole ARBOR Proposal 5 integration owner** for Wave 0. **DO NOT start while any W00 A–F worker is still active.** All six must return committed branches or an explicit BLOCKED report. Read `.integration/proposal5/09-EXECUTION-PLAYBOOK.md`, this wave's LAUNCH, all A–F report files and Proposal 5 beta gates. You alone may update the shared branch `integrate/proposal5-beta-r001`, root Cargo/Bun manifests/lockfiles, `integration/source-lock.json`, release manifests and CI. Never push to ARBOR main.

## Exclusive gate actions
1. Collect all six worker source read reports and reconcile exact donor SHA/subtrees and unmerged relevant source; no guessing from past R001/R002 statements.
2. Enumerate all current runtime entrypoints/owners for Tauri, Host/World/effects, Studio, HQ JS/native, mobile, TierFlow, Labs and legacy candidates; classify SOURCE_PRESENT/WIRED/TESTED/DEVICE_ACCEPTED.
3. Create one integration/source-lock.json (donor repo/commit/subtree hash/source paths/licensing), import-manifest and writer/consumer/migration ledger, with explicit excluded private files.
4. Pin the chosen Cargo/Bun/Windows/Node runtime expectations, source ownership and tests; define exact six worker path ownership for W1.
5. Check P0 risks before any real user data, especially pinned Tauri/native IPC, owner-token loopback service identity and one effect writer. No live data needs to be touched in W0.

## Merge protocol
Pin the exact integration baseline SHA before merging. For each worker in A–F: verify branch base, HEAD, source rights, changed path ownership, actual tests and source evidence. Merge/cherry-pick serially, resolving conflicts through declared code owners. Generate root Cargo/Bun files/locks after merges, run combined tests, and do NOT accept test assertions from individual worker logs as sufficient when integration behavior could differ. Never overwrite concurrent worker commits.

## Acceptance / next-wave contract
GO only with complete source-lock, no unknown active writer, feasible source import graph and known P0 mitigation owner; otherwise REPAIR_REQUIRED. Next W1 source imports.

Publish `integration/reports/W00-GATE.md` and update the appropriate gates in `integration/beta-gates.json` with exact merged SHA/commit list, command and exit code evidence, device/OS, source/capability status, security blockers, feature continuity, rollback and exact next-wave base SHA. If repairs remain, hand off precise domain assignments; no false GO, no new feature requests.
