# W03-F — End-to-end beta acceptance cases and capability continuity

Wave 3 of ARBOR Proposal 5. Start from exact W2-G GO SHA of `integrate/proposal5-beta-r001`, branch `work/p5-w03-f-verify`. Own **tests/, contracts/, capability-spaces/** plus disjoint report `integration/worker-reports/W03-F.md`. No shared branch merge or root file edits.

## Mission — Real Beta Journeys and Minimal Native Node 0
Read Proposal 5 (01–09), previous G gates, canonical source-lock and release compatibility matrix. **As-is first:** migration of existing working features and truthful statuses; avoid new UI/features. Fully self-contained ARBOR, one actual Host effect writer; signed-in Chrome browser (no APIs/Playwright/Puppeteer); scoped HQ Development vs Personal data; no private user data in Git. G is only integration/root lock owner.

1. Implement actual test scripts/fixtures for B01–B12 in 05-FULL-BETA-RELEASE-CONTRACT and map results to G00–G13 machine-readable gates. Do not mark unrun cases passing.
2. Verify that UI, HQ, Host, native and phone refer to the same source action/graph identifiers and distinguish SOURCE_PRESENT, RUNTIME_WIRED and VERIFIED.
3. Inventory source-existing media, voice-to-tasks, personal context, mission feed, app-builder and visual modes; accept operational ones and record precise beta exceptions for unimplemented Labs ideas.
4. Build independent regression fixtures for wrong target, stale source, negation, producer availability, sensitive body disclosure, unsupported input and uncertain effect.
5. Produce an independent user-journey failure list for the next gate, with exact owner/domain.

## Required proof
Runnable acceptance suite and truth-based continuity ledger, with real tests vs UNRUN separately disclosed.

Commit changes and detailed report with base/head SHA, test commands/exits, actual supported source IDs, runtime/environment, availability/auth and P0 blockers. If real owner login or physical phone needed, report a **specific pending acceptance** rather than mock success. G will reconcile and authorize next wave.
