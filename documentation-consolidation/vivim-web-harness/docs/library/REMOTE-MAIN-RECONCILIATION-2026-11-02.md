# Remote-to-main reconciliation — 2026-11-02

**Repository:** owenservera/vivim-web-harness. **Audit basis:** remote `main` at `c2c0cdf73586508b6d39ecffc0c7b72a95b0b73f` at inspection time. **Scope:** all 40 non-main remote branches and 41 PR records, with GitHub compare on each branch; selected full-tree/blob comparisons and file reads. This is a **source audit**, not a test or a claim that all branch work is integrated.

## Update after PR resolution (2026-11-02)\nAll six PRs that were open at this audit's snapshot have since been resolved. **#31 was squash-merged** into `main` as `harness/docs/design/m1.md` at commit `668b1268`. **#41, #40, #35 and #22 were closed** after comparing historical code against newer main implementations; these closures do not certify feature-level UX parity. **#20 was closed deferred and not merged**, preserving the no-Vercel scope. A subsequent current-main doc reconciliation updates the navigation and outdated scope text. The original numbers and PR listings below remain **dated audit evidence**, not live status. Remaining divergent branches must be selectively recovered where meaningful.\n\n## Result

- 21 branches are `behind` with **0 unique commits** relative to main: no merge necessary.
- 19 branches are `diverged` with **one or more commits not ancestrally contained in main**. Many came through squash/rework, so commit ancestry alone does not imply missing behavior.
- Six PRs are still open: **#41**, **#40**, **#35**, **#31**, **#22**, **#20**. #35 targets `mobile-relay`, not `main`; #23 was merged into `mobile-bootstrap-bridge`, not `main`.
- The 2026-11-02 plugin research work **is on main** at `c2c0cdf`.
- **Do not bulk-merge** old feature branches (many are 330–810 main commits behind) or remove branches without reviewed diff/semantic verification. This report is not a substitute for porting missing work.

## Branch inventory

### Fully behind main (21; 0 ahead)
`attach-cdp-target`, `build/add-agent-fleet-and-agent-swarm-to-the-h-mvv7jkyq`, `build/g2-second-model-review-gate-create-src-b-mvv6i5tx`, `build/g4-tab-hygiene-harness-owned-tab-registr-mvv676o6`, `build/g4-wiring-tab-hygiene-in-the-send-path-c-mvv69jum`, `build-protocol`, `build-resume`, `build-test-env`, `dev-auto-approve`, `fix-typed-reason`, `g2-review-cli`, `gate1-goals`, `model-awareness`, `qwen-submit`, `relay-chat-fixes`, `review-followups`, `self-build`, `thread-url-multiline`, `typing-normalize`, `wh-tabs`, `windows-compat`.


### Diverged (19; unique commit ancestry)
`chore/git-reconciliation-2026-11-01`, `design/neckar-sandstone-lab`, `design/m1`, `dev-auto-approve-router`, `docs/handoff`, `docs/ux-shotgun-2026-10-30`, `feat/utc-nlcl-language-core`, `feature/bootstrap-tab-v1`, `feature/personal-ai-bootstrap`, `feature/share-live-feed-20261030`, `gate1-north-star`, `handoff-redis`, `handoff-scripts`, `mobile-bootstrap-bridge`, `mobile-relay`, `roadmap-chatv1`, `roadmap-models`, `roadmap-prefs`, `roundrobin`.

## Higher-value unresolved candidates

| Branch / PR | Concrete evidence | Disposition |
|---|---|---|
| `design/neckar-sandstone-lab` / #41 | Ahead 3/behind 330; full-tree comparison found **no paths in the branch absent from main**. Main already has `docs/design/DESIGN-SYSTEM-NECKAR-SANDSTONE-LAB.md` and `docs/design/design-tokens.v3.json`; many modified files differ due to later work. | **Semantic parity check** of current UI and PR; do not replace newer assets with older snapshots |
| `chore/git-reconciliation-2026-11-01` / #40 | Ahead 1/behind 330; branch introduces `docs/library/GIT-RECONCILIATION-2026-11-01.md` absent from main; other differences concern generated catalogs. The historical report itself refers to an older baseline and claims branch deletions, but 40 remote non-main branches currently exist. | **Do not overwrite** current generated indexes with older ones; preserve history via this fresh audit |
| `feature/personal-ai-bootstrap` | Ahead 8/behind 785; full-tree comparison found **19 missing paths on main**, including `harness/src/onboarding/{compare-runner,grant}.mjs`, `harness/src/providers/kimi.mjs`, `studio/src/agents/personal-ai-baseline.mjs` and associated tests/catalogs. | **Highest priority selective code autopsy**, then port only still-applicable pieces into current Host/Studio with tests; no wholesale merge |
| `roundrobin` | Ahead 1/behind 811; `harness/src/agents/roundrobin.mjs` exists only in branch. Read source: seed-ordered turns, persistent state, journal read-back and uncertain-fence design; older `roles.mjs` cannot replace current effort-aware parser/role set. | Selectively adapt roundrobin into current harness, with send-once and model identity tests |
| `handoff-scripts` | Ahead 1/behind 799; `scripts/rcm` and `scripts/vd` missing from main. | Inspect for hard-coded profiles, legacy CLI calls, secrets or obsolete authority before selectively incorporating |
| `feature/share-live-feed-20261030` / #20 | Ahead 8/behind 810; `apps/share-web/*` and `harness/src/share/feed-publisher.mjs` absent. Earlier requested current path excludes Vercel; branch includes `vercel.json`. | Preserve as candidate but **do not activate cloud/Vercel code** under current product direction |
| `feat/utc-nlcl-language-core` | Ahead 2/behind 334; current main has NLCL, Host control planner/dispatcher and tests, but commits remain divergent. | Compare **semantic missing behavior** (especially KPI and lexical updates), not bulk rebase/merge |
| `feature/bootstrap-tab-v1` / #35 | Ahead 2/behind 800; modifies mobile app and index; PR targets old `mobile-relay`. Current main has newer Talk/Build/Control path. | Validate UX parity and close or retarget only real gaps |
| `mobile-bootstrap-bridge` / #22, `mobile-relay` / #23 | Nested PR history; main has newer Direct transport, APK/Studio and Host paths. | Trace unmatched behavior/receipts before resolution |
| `design/m1` / #31 | Ahead 1/behind 803; standalone historical `harness/docs/design/m1.md` candidate. | Documentation relevance check; no code merge needed |
| `dev-auto-approve-router` / closed-unmerged #39 | Historic bypass proposal; risk to per-send approval and owner safety. | **Do not import** without fresh explicit safety review |
| Remaining diverged documentation/roadmap branches | Older PRs often squash-merged or superseded; current main has `docs/roadmap.json` as the sole task ledger. | File-content comparison and deliberate closeout; never replace latest roadmap with historical version |

## Why not merge all divergent branches?

A squash merge/rewrite means a branch may be ahead in Git ancestry while all its useful content is on main. For example, the Neckar design branch has no **unique file paths**, and main contains its design files, but this alone still does not prove byte-for-byte or rendered UX parity. Conversely, the personal bootstrap branch contains source files demonstrably absent from main, but older imports and owners may clash with the current control/journal architecture. Automated merge would risk regressing accepted code, code ownership, privacy/approval policy and current phone flow.

## Completion gate

"Everything important is on main" requires a reviewed **per-branch decision**: (a) exact merged/semantically superseded, (b) selective port + tests, (c) explicitly deferred/obsolete, or (d) needs user product decision. Only (a)/(b) count as landed. For each (b), capture source SHA/path, target implementation, test receipts, Host/Studio authority preservation, and device acceptance where needed. Do not mark completion from ancestry or source presence alone.

## Current actions this audit performed

- Enumerated remote branches/PRs and compared **every** branch against main.
- Compared full trees for selected high-value branches and inspected RoundRobin/roles source plus prior reconciliation report.
- Added this evidence report directly to `main`; **no arbitrary feature code merged or branches deleted**.
- No local Windows/Linux runtime or Android physical testing was performed.

**Owner direction:** All source integration should land on `main`; use reviewed selective ports, avoid creating fresh feature branches, and preserve remote history until safe classification.
