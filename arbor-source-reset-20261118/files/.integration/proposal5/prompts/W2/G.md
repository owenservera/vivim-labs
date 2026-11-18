# W02-G — Serial Integration Gate: Internal Contracts and Secure Runtime Vertical Slice

You are the **sole ARBOR Proposal 5 integration owner** for Wave 2. **DO NOT start while any W02 A–F worker is still active.** All six must return committed branches or an explicit BLOCKED report. Read `.integration/proposal5/09-EXECUTION-PLAYBOOK.md`, this wave's LAUNCH, all A–F report files and Proposal 5 beta gates. You alone may update the shared branch `integrate/proposal5-beta-r001`, root Cargo/Bun manifests/lockfiles, `integration/source-lock.json`, release manifests and CI. Never push to ARBOR main.

## Exclusive gate actions
1. Merge A–F in deterministic order, regenerate root lockfiles solely here, run Cargo/Bun tests and cross-language contract fixtures against in-repo source.
2. Verify HQKG native linked to product workspace; current Host is one active World/effect writer; imported Studio, HQ JS and TierFlow use only internal paths.
3. Verify source-backed minimal internal path Tauri/native→bounded graph read→Host read/status→source-linked UI or CLI; no synthetic fixture-only action misrepresented as live.
4. Check new Tauri patched supported version/complete lock resolution and exact IPC/origin scope; verify Host process identity before persistent token transfer using synthetic adversarial path.
5. Check PWA Host pairing and actor/source revision schema; no independent phone effect approval. Produce explicit unsupported/blocked source status.

## Merge protocol
Pin the exact integration baseline SHA before merging. For each worker in A–F: verify branch base, HEAD, source rights, changed path ownership, actual tests and source evidence. Merge/cherry-pick serially, resolving conflicts through declared code owners. Generate root Cargo/Bun files/locks after merges, run combined tests, and do NOT accept test assertions from individual worker logs as sufficient when integration behavior could differ. Never overwrite concurrent worker commits.

## Acceptance / next-wave contract
GO when core compiled internal vertical slice and critical native trust boundary verified at source + synthetic tests; real provider/phone/installer E2E remains for W3/W4. P0 origin/owner-token issue cannot be deferred.

Publish `integration/reports/W02-GATE.md` and update the appropriate gates in `integration/beta-gates.json` with exact merged SHA/commit list, command and exit code evidence, device/OS, source/capability status, security blockers, feature continuity, rollback and exact next-wave base SHA. If repairs remain, hand off precise domain assignments; no false GO, no new feature requests.
