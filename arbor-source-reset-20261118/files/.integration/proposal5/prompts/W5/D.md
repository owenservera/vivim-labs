# W05-D — HQ intelligence and graph-read blocker closure

Wave 5 of ARBOR Proposal 5. Start from W4-G GO/**REPAIR_HANDOFF** exact integration SHA; create `work/p5-w05-d-hq`. Own **hq/**, and disjoint `integration/worker-reports/W05-D.md`. Only G merges, changes root lockfiles or declares beta status.

## Mission — Bounded Blocker Repair, Regression Closure and Release Candidate
Read W4-G exact domain-tagged blocker assignments (including REPAIR_HANDOFF) and only repair listed release-blocking defects. Source has already landed internally. Preserve current Host/Studio/HQ/TierFlow/phone behavior; **no feature or UI upgrades**. Any unassigned work = regression only. One Host writer, no outside donor repo/runtime, no secrets. Real test results, not optimistic status.

1. Fix only W4-G assigned in-repo HQ index/search/context/wiki source dependency, deterministic outputs, source freshness, privacy or low-storage defects.
2. Rerun internal HQ synthetic project indexing/wiki/context and source denial tests with donor checkouts unavailable.
3. Maintain Development/Personal namespace separation through existing native adapter and Host owner read grants; no new HQ runtime service.
4. Do not port HQ JS into Rust or expand AutoWiki/personal corpus beyond beta source-proven scope.

## Acceptance
HQ source correctness and self-contained runtime established; disk/secret-filter regressions repaired.

Commit fixes/tests and a report enumerating every assigned blocker and proof status, base/head SHA, command/output, source risk, remaining P0 defects and requested G merges. If you find no assigned blockers, commit a small report with test observations, **not** invented implementation work.
