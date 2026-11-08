# Lane D — Copy Summary (Public Aggregate)

**Scan:** AUTQ-01 · `autq01-20261107T2203Z` · generated 2026-11-08
**Companion evidence:** `.private-local/COPY-RECEIPTS.jsonl`, `.private-local/D-COPY-COVERAGE.private.json` (gitignored, private)

This file contains aggregate counts only. No private paths, hashes, or source identifiers.

## Outcome

| Metric | Count |
| --- | --- |
| Manifest rows reviewed | 1323 |
| Eligible for copy | 938 |
| Copied + hash-verified (private corpus) | 929 |
| Verified existing (idempotent re-run) | 0 |
| Blocked | 1 |
| Secret-blocked (copy deleted) | 8 |
| Skipped (no reason pending) | 0 |
| Conflict | 0 |

Reconciliation: 929 + 0 + 1 + 0 + 8 + 0 = 938 (holds).

## Private corpus mirror (gitignored `.private-local/corpus/`)

Bytes copied and verified: 20,176,757 (~19.2 MiB). Disk usage 13M. Every copy was
SHA256-verified against the pinned manifest hash at copy time; a 10-file random
re-hash audit returned 10/10 MATCH.

| Origin repo (slug) | Copied |
| --- | --- |
| repos_hq | 394 |
| vivim-web-harness | 402 |
| SOFTWARE_HQ | 52 |
| vivim-ui | 66 |
| PC_HQ | 11 |
| vivim-labs-autq (worktree) | 4 |

## Non-copied eligible rows (9) — all with reasons

- 1 BLOCKED: source content drifted from the pinned manifest hash after discovery;
  copy refused rather than copied unverified.
- 8 SECRET_BLOCKED: per-file secret-pattern gate matched (credential-shaped strings
  in test fixtures); the copies were deleted and only the path retained in the
  private receipt.

## Public corpus (03-corpus-public/)

**Empty.** All 7 PUBLIC_SAFE manifest rows are repo-level census records
(sourceKind `other`, no file hash, no file path); census rows are never copied, so
the license gate had no file rows to evaluate. No sidecars were written. Publication
of any public corpus remains subject to the separate PUBLIC_RELEASE_REVIEW gate.

## Gates

- **G2 / SECURITY: PASS (lane scope)** — corpus tree is gitignored (check-ignore probe
  proof); zero public copies; 8 secret-pattern copies deleted; final whole-tree secret
  sweep returned 0 hits; `git status` shows nothing under `.private-local/`.
- **G3 / COPY INTEGRITY: PASS (lane scope)** — all copied files hash-match the pinned
  originals; originals were read-only (no git state touched, no staging, no commits);
  every non-copied eligible row carries an explicit reason in the receipts.

## Exclusions applied (never copied)

217 census rows (kind `other`, name-only), 12 git-history rows, 154 LOCAL_SENSITIVE
rows, 1 CREDENTIAL_DENY row, 1 row without a pinned hash. Also excluded by rule:
non-text extensions, files >1MB, and anything under node_modules/.git/.hq state or
browser profiles.
