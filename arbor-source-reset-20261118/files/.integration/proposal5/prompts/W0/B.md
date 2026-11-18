# W00-B — Host, World and effect authority

## Role and launch
You own **Wave 0, lane B** of ARBOR Proposal 5. Begin from the exact shared integration branch `integrate/proposal5-beta-r001` HEAD pinned by the wave orchestrator. Create your independent branch `work/p5-w00-b-host`. **W0 is source-read-only; no product code edits.**

**Shared plan:** Read `.integration/proposal5/README.md`, `01`–`09` documents and `08-BETA-GATES.json`. ARBOR is a single self-contained product; no external donor repo/service runtime wiring. Preserve working JS Host, Studio, PWA, HQ operational JS, HQKG Rust and TierFlow; no unnecessary features or UI redesign. Existing Host is sole effect/World writer. Do not commit secrets or user data. Workers never merge into integration/main; only Wave G merges.

## Mission: Source Truth and Import Freeze
Owned source inspection: apps/vivim-host/ (read-only in W0).

1. Inventory real Host/server entrypoints, active command catalog including version/IDs/risk/readiness, planner and one effect dispatcher/journal/receipt path.
2. Trace Host startup/owner token, World data owners, config, approvals, recovery and exact claim fence; identify effects vs read-only commands.
3. Inventory working Host onboarding, conversations, source access, media/byte-range streaming, missions, native app builder and personal World with statuses grounded in source/tests.
4. Identify Host import dependency closure and existing tests; mark source-present vs wired vs device-accepted; do not invent supported actions.

## Acceptance
Host writer/reader map, source SHA, test list, runtime gaps, record formats and migration blockers.

## Required output
Commit exactly one evidence report `integration/worker-reports/W00-B.md` (explicit disjoint report-path exception) with donor repo/branch/SHA, source files and ownership, tested-vs-proposed status, copied/not-copied decisions, security/privacy class, verification commands (mark UNRUN), blocker list and instructions to successor lane W1-B. Provide HEAD SHA. **Do not call anything successfully integrated.**
