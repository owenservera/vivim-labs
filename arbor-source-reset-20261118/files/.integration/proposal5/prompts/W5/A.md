# W05-A — Native Rust and Tauri blocker closure

Wave 5 of ARBOR Proposal 5. Start from W4-G GO/**REPAIR_HANDOFF** exact integration SHA; create `work/p5-w05-a-native`. Own **src-tauri/, native/, core/programmability/**, and disjoint `integration/worker-reports/W05-A.md`. Only G merges, changes root lockfiles or declares beta status.

## Mission — Bounded Blocker Repair, Regression Closure and Release Candidate
Read W4-G exact domain-tagged blocker assignments (including REPAIR_HANDOFF) and only repair listed release-blocking defects. Source has already landed internally. Preserve current Host/Studio/HQ/TierFlow/phone behavior; **no feature or UI upgrades**. Any unassigned work = regression only. One Host writer, no outside donor repo/runtime, no secrets. Real test results, not optimistic status.

1. Read W4-G named native P0/P1 blockers, fix only assigned Tauri/native origin/bootstrap/hqkg/Node0 correctness or release build defects. If none assigned, run regression and report NO_CHANGES.
2. Preserve imported HQKG public API and source parity; any algorithm redesign or broad Rust rewrite requires a proven gate-breaking defect.
3. Rerun native security misuse tests, bounded input/IME/revision tests, cargo test/clippy on supported Windows environment; root lockfile changes go through G.
4. Remove build dependencies on donor paths and ensure native packaging uses current ARBOR source only. Report exact before/after reproducer.

## Acceptance
Every assigned native blocker has a verified regression PASS or explicit unresolved BLOCKED; no scope creep.

Commit fixes/tests and a report enumerating every assigned blocker and proof status, base/head SHA, command/output, source risk, remaining P0 defects and requested G merges. If you find no assigned blockers, commit a small report with test observations, **not** invented implementation work.
