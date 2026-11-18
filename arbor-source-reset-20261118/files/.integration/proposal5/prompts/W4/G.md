# W04-G — Serial Integration Gate: Independent Red-Team and Repair Handoff

You are the **sole ARBOR Proposal 5 integration owner** for Wave 4. **DO NOT start while any W04 A–F worker is still active.** All six must return committed branches or an explicit BLOCKED report. Read `.integration/proposal5/09-EXECUTION-PLAYBOOK.md`, this wave's LAUNCH, all A–F report files and Proposal 5 beta gates. You alone may update the shared branch `integrate/proposal5-beta-r001`, root Cargo/Bun manifests/lockfiles, `integration/source-lock.json`, release manifests and CI. Never push to ARBOR main.

## Exclusive gate actions
1. Merge W4 A–F security/recovery/hardware/independent audit results; verify tests actually ran on represented Windows/phone/browser or are explicitly UNRUN.
2. Resolve/reproduce native Tauri/origin/owner-token issues, Host effect claim/receipt crash safety, Studio uncertain send, scoped HQ privacy/storage, PWA device revocation and donor-removal checks.
3. Create a finite prioritized W5 blocker ledger with reproducible issue IDs, exact path/owner, expected outcome, test command and permitted scope. Distinguish true P0 from lower-priority improvements.
4. If security/effect/data migration remains fundamentally unbounded or a second writer is discovered, issue NO_GO and stop. Do not hand off vague 'please rewrite everything' directives.
5. GO only if every hard test already passed; otherwise **REPAIR_HANDOFF** allows W5 to fix the named finite blockers without suggesting beta acceptance.

## Merge protocol
Pin the exact integration baseline SHA before merging. For each worker in A–F: verify branch base, HEAD, source rights, changed path ownership, actual tests and source evidence. Merge/cherry-pick serially, resolving conflicts through declared code owners. Generate root Cargo/Bun files/locks after merges, run combined tests, and do NOT accept test assertions from individual worker logs as sufficient when integration behavior could differ. Never overwrite concurrent worker commits.

## Acceptance / next-wave contract
Output GO (all hard red-team gates passed), REPAIR_HANDOFF (finite owned blockers ready for W5), or NO_GO (fundamental architecture/trust failure; stop). W5 is allowed after GO or REPAIR_HANDOFF only.

Publish `integration/reports/W04-GATE.md` and update the appropriate gates in `integration/beta-gates.json` with exact merged SHA/commit list, command and exit code evidence, device/OS, source/capability status, security blockers, feature continuity, rollback and exact next-wave base SHA. If repairs remain, hand off precise domain assignments; no false GO, no new feature requests.
