# W05-E — PWA/TierFlow blocker closure

Wave 5 of ARBOR Proposal 5. Start from W4-G GO/**REPAIR_HANDOFF** exact integration SHA; create `work/p5-w05-e-ui`. Own **apps/mobile-relay/, packages/tierflow/, owned visual adapter**, and disjoint `integration/worker-reports/W05-E.md`. Only G merges, changes root lockfiles or declares beta status.

## Mission — Bounded Blocker Repair, Regression Closure and Release Candidate
Read W4-G exact domain-tagged blocker assignments (including REPAIR_HANDOFF) and only repair listed release-blocking defects. Source has already landed internally. Preserve current Host/Studio/HQ/TierFlow/phone behavior; **no feature or UI upgrades**. Any unassigned work = regression only. One Host writer, no outside donor repo/runtime, no secrets. Real test results, not optimistic status.

1. Fix only W4-G assigned broken pairing/phone session, user input/IME, source status display and internal TierFlow build defects; no layout or app-builder redesign.
2. Preserve one mobile PWA and one current source-driven UI publisher; do not create second UI action dispatcher.
3. Retest current device revocation/offline/stale send denial and reasonable accessibility/edit behavior on physical device where available.
4. Check existing source-working media and mission views weren't inadvertently dropped by import.

## Acceptance
Existing PWA UI and pairing path restored with physical-device or honest blocker evidence.

Commit fixes/tests and a report enumerating every assigned blocker and proof status, base/head SHA, command/output, source risk, remaining P0 defects and requested G merges. If you find no assigned blockers, commit a small report with test observations, **not** invented implementation work.
