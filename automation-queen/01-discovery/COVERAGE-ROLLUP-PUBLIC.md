# COVERAGE-ROLLUP-PUBLIC.md — AUTQ-01 final counts (scanId autq01-20261107T2203Z)

Every number below was re-derived by Lane E from the jsonl/json artifacts (manifest, merge receipt, copy receipts, graph, duplicates) — none carried over unverified. Base pass = Lane A deep dives; A2 = fan-out merge (receipt: 1,323 + 1,538 appended − 18 duplicate ids = 2,861).

## Final counts

| Metric | Count | Source of truth |
| --- | --- | --- |
| Total discovered candidates (manifest rows) | **2,861** (base pass 1,323 + 1,538 fan-out) | PRIVATE-SOURCE-MANIFEST.jsonl · A2-MERGE-RECEIPT |
| Census rows (repo/dir metadata, name-only) | 217 | manifest sourceKind=other |
| Hashed rows (SHA-256 pinned) | 2,390 (938 in the copy-eligible base pass) | manifest originalSha256 |
| Exact mirrored private copies | **930** (929 + 1 re-pinned refresh) | COPY-RECEIPTS.jsonl result=COPIED |
| Bytes copied and verified | **20,235,524** (20,176,757 baseline + 58,767 refresh) | receipts bytes sum; 10/10 random re-hash MATCH |
| Public copies | **0** (7 PUBLIC_SAFE rows are census records; no file path → license gate had nothing to copy) | D-COPY-COVERAGE.publicCorpusOutcome |
| Blocked (reason recorded) | 1 (hash drift → re-pinned and copied as the refresh above) | receipts BLOCKED + COPIED pair |
| Secret-excluded | **8** (per-file gate; copies deleted, paths retained; post-copy tree sweep 0) | D-COPY-COVERAGE.secretGate |
| Duplicate clusters | **86** = 67 bit-identical (218 member rows) + 10 near (47) + 9 semantic (35) | C-DUPLICATES.private.jsonl |
| Dependency edges | 163 (calls 50 · triggers 27 · depends_on 18 · duplicates 45 · others 23) | C-DEPENDENCY-GRAPH.private.jsonl |
| References-only rows | 307 LOCAL_SENSITIVE (name/status only) + 28 git-history + 217 census + 1,460 hashed-but-REF_ONLY fan-out rows + 181 unhashable (no-hash/binary/oversize) | manifest copyDisposition=REF_ONLY |
| Inaccessible named | **0** · Unknown roots | **10** (1 secret-ref-only, 8 unknown-review, 1 redacted vendor task payload) | A-SOURCE-COVERAGE.inaccessible=[] · B-UNKNOWN-ROOTS |
| Historical rows | 28 (12 base git-history + 16 A2 deleted-path rows) | manifest sourceKind=git-history + A2-history.jsonl |
| Observed-running processes (project-named) | 65 (38 ZCode + 24 node + 3 bun) of 326 rows | B-LIVE-INVENTORY §4 |
| Active schedulers | 2 actively firing (≈5-min git cadence lastResult 0; 3-day model-catalog cadence lastResult 0) + 1 ready on-demand + 1 running third-party vendor agent | B §1 |
| Project-relevant services | 12 (6 running / 6 stopped) | B §3 |
| Startup surfaces | 23 entries across 5 locations; 8 project/agent-related | B §2 |
| Current-source-verified rows | 2,390 hashed (evidenceLevel source-verified on 2,696 rows incl. line-anchored confirmations) | manifest evidenceLevel |
| Uncertain liveness (unknown) | **601** (plus 1,273 implemented_unproven + 781 docs_only — claims, not observations) | manifest liveness |
| Deep dives | **15/16** core units (+ Labs dirty inventory 11 rows; Labs worktree mission-docs 4 rows) | A-SOURCE-COVERAGE + A2 fan-out |
| Repos not-reached | **0** (the 9 previously not-reached repos now walked: 31/200/250/102/29/379/201/251/65 rows) | A2-MERGE-RECEIPT inputs |
| History term verdicts | 130/130 resolved across two sweeps (13 terms × 5 sources × 2), 0 timeouts; ADA raw hits adjudicated NOT_FOUND (80 subjects, 0 word-boundary matches) | A2-history-coverage + Lane A coverage |
| Confirmed families | 12/12 with evidence; F5/F8/F11 CONFIRMED_PRESENT via 32 line-anchored rows; one named tree confirmed ABSENT (nothing fabricated) | FAMILIES-CONFIRMATION |
| Flags | contradictory_authority 2 · ghost_agent 3 · dead_job 13 · unsafe_retry 2 · missing_approval 7 · orphaned_source 8 · docs_only_claimed 4 | C-FLAGS-REGISTER |

## Catalog completeness per scope

- **Git-file scope:** 1,968 rows across 79 repos + worktrees; hashed 1,950; 18 unhashed (unhashable or capped) — deep walks capped at 400 rows on three repos (87 / 112 truncated), one ~1.4k skill corpus excluded, one 26-row tail.
- **Local-file scope:** 648 rows (dirty/untracked/mission docs); hashed 440; unhashable/binary 208; all name+status or hash-only — contents of sensitive files never read.
- **Machine scope:** one 9-minute read-only snapshot (2026-11-07 22:07–22:16 UTC) covering tasks, services, processes, startup, config keys/names; no probe re-runs.
- **History scope:** commit-message grep (capped per query) + deleted-file sweeps (last-30 deleting commits base pass) + 16 named deleted-path rows; deeper archaeology open.

## Honesty line

Nothing here says "everything copied": 930 of 2,861 candidates are byte-mirrored (all that was eligible under the safety rules); 8 are secret-excluded, 1,931 are reference-only by design or classification, 10 roots are unknown, and 601 rows have unknown liveness. Gates: G0 PASS · G1 PASS_WITH_LIMITS · G2 PASS · G3 PASS · G4 PASS · G5 PARTIAL · G6 PASS_WITH_LIMITS · G7 PASS_WITH_LIMITS (self-graded) · G8 UNRUN-UNTIL-COORDINATOR · G9 PASS — see 07-evidence/GATE-REPORT.md.
