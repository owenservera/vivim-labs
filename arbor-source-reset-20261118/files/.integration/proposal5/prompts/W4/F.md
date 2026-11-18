# W04-F — Independent no-outside-wiring, licensing and release QA

Wave 4 of ARBOR Proposal 5. Base exact W3-G GO commit from `integrate/proposal5-beta-r001`, branch `work/p5-w04-f-verify`. Write only **tests/, contracts/, capability-spaces/, verification scripts** plus `integration/worker-reports/W04-F.md`. Root lockfiles and cross-owner merges belong to serial W4-G.

## Mission — Independent Security, Recovery and No-Donor Verification
Read Proposal 5 and W3-G reports. This is the red-team wave: demonstrate proof, don't guess. Source imported **inside ARBOR**, no external HQ/TierFlow/Harness runtime, one Host JS journal, no provider APIs, no unsafe native trust. Do not redesign UI or add functionality. Fix only domain-local security defects and report others. Unavailable physical devices/provider sessions are BLOCKED, never simulated PASS.

1. Simulate donor disappearance: move/unmount all donor repo checkouts and stop their background services, then build/test/launch ARBOR with synthetic local content. Scan imports, subprocess commands, Git URLs, paths and runtime ports for outside-VIVIM repo dependencies.
2. Validate build licenses/notice/source ancestry, packaging of Node runtime as needed, and pinned Bun/Cargo/TierFlow/hqkg source versions; no secrets or user profiles in release.
3. Run B01–B12/G00–G13 independent acceptance suite on current integration head, not isolated worker branch only; separate actual PASS from UNRUN/BLOCKED.
4. Audit action catalog vs capability index, source readiness vs available_now, duplicate effect/World writers, duplicated PWA/TierFlow publisher and graph engine.
5. Publish prioritized domain-tagged P0/P1 defect ledger and controlled W5 repair assignments. Do not mark GO if P0 remains.

## Required proof
Adversarial no-donor acceptance and exact W5 repair backlog with independent source/build evidence.

Commit reproducible tests/appropriate local fixes and a report listing base/head SHA, exact commands/exits, OS/device, severity, source evidence and assigned owner of each remaining blocker. Never use a real personal token/data in synthetic adversarial tests. W4-G may issue a documented **REPAIR_HANDOFF** to W5 with a finite bug list; fundamental unscoped security or unknown effect writer remains NO_GO.
