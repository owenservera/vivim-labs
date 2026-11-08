# A-SOURCE-COVERAGE (PUBLIC AGGREGATE) — AUTQ-01 Lane A

- scanId: `autq01-20261107T2203Z`
- Generated: 2026-11-08 (UTC), continuation run after first STOP
- Lane: A — Repository & commit archaeology (READ-ONLY toward all source repos)
- Status: **PARTIAL** — 5 of 16 core units deep-dived; 9 remain not-reached. This file contains aggregate counts only. No private paths beyond repo names, no private branch names, no private file lists. Private detail (branch names, per-file rows, exact dirty paths) lives in `.private-local/A-SOURCE-COVERAGE.private.json` (gitignored).

## 1. Enumeration totals (C:\0-BlackBoxProject-0 root)

| Census class | Count |
| --- | --- |
| Git repositories (top-level entries) | 79 |
| Non-git directories (name-only census rows) | 138 |
| Loose top-level files (name-only census rows) | 6 |
| **Total census/metadata rows** | **223** |

- 1 shallow clone observed among all repos.
- All 16 core repos/worktrees assigned to Lane A were present and readable; none inaccessible.

## 2. Manifest totals (PRIVATE-SOURCE-MANIFEST.jsonl)

| Metric | Count |
| --- | --- |
| Total manifest rows | 1323 |
| Rows with SHA-256 hash | 938 |
| Rows truncated by per-repo caps (400/400/400/150/150) | 199 |
| Rows classified PRIVATE_CODE | 1008 |
| Rows classified LOCAL_SENSITIVE (name/status only; contents never read) | 307 |
| Rows classified PUBLIC_SAFE | 7 |
| Rows classified CREDENTIAL_DENY (name-only, never opened) | 1 |

By source kind: git-file 936 · census (repo/dir metadata) 217 · local-file 158 (incl. 148 dirty-file inventory rows) · git-history 12.
By origin visibility: private 1157 · local-only 159 · public 7.
By liveness: implemented_unproven 392 · unknown 377 · docs_only 370 · configured 172 · historical 12.
All rows: copyDisposition REF_ONLY (Lane A copies nothing) · decision UNKNOWN (classification is not a keep/retire decision).

## 3. Deep dives — 5 of 16 core units completed (with caps)

| Unit | Files found in automation dirs | Rows written | Truncated by cap | Dirty-file inventory |
| --- | --- | --- | --- | --- |
| repos_hq | 487 | 404 | 87 | 3 files, census-level only |
| vivim-web-harness | 512 | 403 | 112 | 6 files, census-level only |
| SOFTWARE_HQ | 51 | 52 | 0 | 137 files, name+status rows |
| vivim-ui | 64 | 66 | 0 | not rowed (outside mandate) |
| PC_HQ | 11 | 11 | 0 | not rowed (outside mandate) |
| vivim-labs (walk) | — | — | — | 11 files, name+status rows (2 worktree-deleted + 9 untracked) |
| vivim-labs-autq (mission docs) | — | 4 | — | worktree clean (0 dirty) |

Family presence observed in the deep-dived repos (counts of matched files; presence ≠ verified running):

| Family (mission numbering) | repos_hq | vivim-web-harness | SOFTWARE_HQ | vivim-ui | PC_HQ |
| --- | --- | --- | --- | --- | --- |
| 1 Agent roles / supervisors | 34 | 68 | 4 | 4 | 0 observed |
| 2 Workflows / recipes / tools | 143 | 61 | 15 | 38 | 1 |
| 3 Triggers / watchers | 2 | 0 observed | 0 observed | 0 observed | 0 observed |
| 4 Tools / MCP / skills | 8 | 5 | 0 observed | 2 | 0 observed |
| 5 Work/Plan/Step/Attempt runtime | not confirmed | not confirmed | not confirmed | not confirmed | not confirmed |
| 6 Effect / authorization policy | 4 | 2 | 1 | 2 | 0 observed |
| 7 Model routing | 8 | 5 | 12 | 1 | 0 observed |
| 8 State / checkpoints / ledgers | not confirmed | not confirmed | not confirmed | not confirmed | not confirmed |
| 9 Tests / observability | 112 | 5 | 1 | 1 | 1 |
| 10 UI / NL surfaces | partial | partial (docs-heavy) | 0 observed | 0 observed | 0 observed |
| 11 Bootstrap / sync infra | not confirmed | not confirmed | not confirmed | not confirmed | not confirmed |
| 12 History / orphaned / uncommitted | partial | partial | partial | partial | partial |

## 4. Core repos NOT deep-dived (PARTIAL — not reached)

11 of 16 core units remain metadata-only: vivim-workstation, VOMEGA (23 dirty uninventoried), zcode-setup (58 dirty), zcode-bots (17 dirty), zcode-pocket, VIVIM-NEW (218 dirty), web-harness, index (47 dirty), ideation — plus vivim-labs main-tree walk (dirty inventory done, walk not required by mandate) and vivim-labs-autq walk (mission docs rowed only).

## 5. History search — RUN for 5 named sources (13 terms x 5 = 65 queries, 0 NOT_RUN)

Method: `git log --all -i -E --grep <term> --oneline --max-count 20` per source x term; word-bounded forms for ADA/CFA/plan/attempt/lease.

| Term | repos_hq | vivim-web-harness | SOFTWARE_HQ | vivim-labs | vivim-ui |
| --- | --- | --- | --- | --- | --- |
| Automation Design Architect | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND |
| ADA (word-bounded) | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND |
| agentctl | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND |
| Architecture Steward | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND |
| Agent Commons | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND |
| CFA (word-bounded) | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND |
| temporal dispatcher | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND |
| work order variants | HIT 2 | HIT 1 | NOT_FOUND | HIT 1 | HIT 1 |
| plan (word-bounded) | HIT 12 | HIT 20 (capped) | HIT 7 | HIT 1 | HIT 5 |
| attempt (word-bounded) | NOT_FOUND | HIT 13 | NOT_FOUND | NOT_FOUND | HIT 1 |
| self-repair variants | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND |
| lease (word-bounded) | NOT_FOUND | HIT 5 | NOT_FOUND | NOT_FOUND | NOT_FOUND |
| scheduled automation | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND | NOT_FOUND |

Key finding: the "work order" hits in all 5 sources are the same propagated scaffold commit ("docs(agent): scaffold (scoped) local execution work order") — a single documentation scaffold, not five independent implementations. The ADA / agentctl / Architecture Steward / Agent Commons / CFA / temporal-dispatcher / self-repair / scheduled-automation historical-candidate names produced **zero commit-message hits in all 5 sources** — they remain unverified as code, and are now also unverified as commit history in the scanned sources. Short SHAs of every HIT commit are recorded in the private coverage file and in 12 git-history manifest rows for Lane C provenance.

## 6. Deleted-file sweep — RUN for the 5 sources (bounded)

One sweep per source: `git log --all --diff-filter=D --name-only --max-count 30` (last 30 deleting commits), names grepped against the 13 keywords. Deleted-name rows seen: repos_hq 51, vivim-web-harness 108, SOFTWARE_HQ 1, vivim-labs 1, vivim-ui 13. **Keyword matches: 0 in all 5 sources.** Limitation: older deletions beyond the last 30 deleting commits are not covered — a deeper sweep remains open.

## 7. Branch census — RUN for the 5 sources (counts only here)

| Source | Branch count (local + remote-tracking) |
| --- | --- |
| repos_hq | 15 |
| vivim-web-harness | 61 |
| SOFTWARE_HQ | 7 |
| vivim-labs | 6 |
| vivim-ui | 6 |

Branch names are recorded in the private coverage file only.

## 8. Gate status (Lane A scope)

| Gate | Status | Evidence |
| --- | --- | --- |
| G0 BASELINE | PASS (with one noted delta) | All 16 core units present; HEADs pinned for 79 repos; all named baseline HEADs matched exactly; 1 delta: repos_hq dirty 3 vs baseline 2 (HEAD unchanged; no files modified by this lane) |
| G1 COVERAGE | PARTIAL | 5/16 core deep dives complete with caps; vivim-labs (11) + SOFTWARE_HQ (137) dirty files inventoried name-only; history search 65/65 resolved for 5 sources; deleted sweep + branch census run for 5 sources; 9 core units not reached; families 5, 8, 11 not confirmed in any deep-dived repo |
| G2 SECURITY | NOT ASSESSED HERE (Lane D gate) | Manifest secret-pattern acceptance check: 1 raw hit, verified filename false positive (a docs filename tail matches the pattern as a case-insensitive substring); 0 real secret matches; 1 credential-named file recorded name-only; no file contents read |

## 9. What remains for a re-run

1. Deep-dive the 9 remaining core units (400-row cap each; SOFTWARE_HQ-style dirty inventories for VIVIM-NEW 218, zcode-setup 58, index 47, VOMEGA 23, zcode-bots 17).
2. Extend history grep + deleted sweep to the 9 remaining units and beyond the last-30-deleting-commits bound.
3. Functional (Lane C) mapping of the confirmed assets: supervisor/admission, work-orders substrate, model-router/budget, MCP tools, seat-watch, sync-bus contracts, harness runtime/models.json.
