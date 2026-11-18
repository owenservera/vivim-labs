# W05-C — Studio/browser blocker closure

Wave 5 of ARBOR Proposal 5. Start from W4-G GO/**REPAIR_HANDOFF** exact integration SHA; create `work/p5-w05-c-studio`. Own **studio/, harness/**, and disjoint `integration/worker-reports/W05-C.md`. Only G merges, changes root lockfiles or declares beta status.

## Mission — Bounded Blocker Repair, Regression Closure and Release Candidate
Read W4-G exact domain-tagged blocker assignments (including REPAIR_HANDOFF) and only repair listed release-blocking defects. Source has already landed internally. Preserve current Host/Studio/HQ/TierFlow/phone behavior; **no feature or UI upgrades**. Any unassigned work = regression only. One Host writer, no outside donor repo/runtime, no secrets. Real test results, not optimistic status.

1. Fix only supported provider Chrome session, import/path, lease/send-once or response evidence defects blocking beta; do not broaden provider support or introduce APIs.
2. Re-run synthetic provider fault cases and authorized live smoke when login available. If not, preserve explicit LIVE_LOGIN_BLOCKED with user steps.
3. Verify browser worker packaging and no donor runtime/imports, exact thread/body correlation and resilient uncertain-send statuses.
4. Communicate any required Host contract patch to B/G rather than editing Host-owned files.

## Acceptance
Studio existing supported flow returns source/receipt evidence or exact platform/login blocker; no blind resend.

Commit fixes/tests and a report enumerating every assigned blocker and proof status, base/head SHA, command/output, source risk, remaining P0 defects and requested G merges. If you find no assigned blockers, commit a small report with test observations, **not** invented implementation work.
