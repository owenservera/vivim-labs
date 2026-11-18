# W05-B — Host and World blocker closure

Wave 5 of ARBOR Proposal 5. Start from W4-G GO/**REPAIR_HANDOFF** exact integration SHA; create `work/p5-w05-b-host`. Own **apps/vivim-host/**, and disjoint `integration/worker-reports/W05-B.md`. Only G merges, changes root lockfiles or declares beta status.

## Mission — Bounded Blocker Repair, Regression Closure and Release Candidate
Read W4-G exact domain-tagged blocker assignments (including REPAIR_HANDOFF) and only repair listed release-blocking defects. Source has already landed internally. Preserve current Host/Studio/HQ/TierFlow/phone behavior; **no feature or UI upgrades**. Any unassigned work = regression only. One Host writer, no outside donor repo/runtime, no secrets. Real test results, not optimistic status.

1. Fix only Host/source/World/control and token handshake defects assigned by W4-G; preserve one-effect-writer identity and current action ID/risk semantics.
2. Rerun plan→approval→claim→receipt and uncertain-send crash/recovery fault injection; no duplicate writes or unsafe retries.
3. Verify Host source data migration/World permissions and restart behavior with synthetic copy; never mutate real owner data without approval.
4. Provide exact source-compatible Host status endpoint and internal package references, no external Harness process.

## Acceptance
Host fixes covered by reproducible effect/auth/recovery tests; no extra action authority.

Commit fixes/tests and a report enumerating every assigned blocker and proof status, base/head SHA, command/output, source risk, remaining P0 defects and requested G merges. If you find no assigned blockers, commit a small report with test observations, **not** invented implementation work.
