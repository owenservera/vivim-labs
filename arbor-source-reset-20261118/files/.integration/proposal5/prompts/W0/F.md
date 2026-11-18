# W00-F — Independent source evidence, Labs and release risk

## Role and launch
You own **Wave 0, lane F** of ARBOR Proposal 5. Begin from the exact shared integration branch `integrate/proposal5-beta-r001` HEAD pinned by the wave orchestrator. Create your independent branch `work/p5-w00-f-verify`. **W0 is source-read-only; no product code edits.**

**Shared plan:** Read `.integration/proposal5/README.md`, `01`–`09` documents and `08-BETA-GATES.json`. ARBOR is a single self-contained product; no external donor repo/service runtime wiring. Preserve working JS Host, Studio, PWA, HQ operational JS, HQKG Rust and TierFlow; no unnecessary features or UI redesign. Existing Host is sole effect/World writer. Do not commit secrets or user data. Workers never merge into integration/main; only Wave G merges.

## Mission: Source Truth and Import Freeze
Owned source inspection: contracts/, capability-spaces/, tests/ evidence ONLY (no feature code in W0).

1. Inventory ratified vs designed Labs Node0, voice-to-tasks, personal knowledge, DCB, Chronos, app builder, observability, AutoWiki and storage bounds. Do not promote design as wired implementation.
2. Prepare source security and leak hygiene rules: no Chrome profiles, tokens, raw chats, personal World DB, node_modules, target, logs or outdated API-provider SDKs in import.
3. Inventory existing test suites, Windows build requirements, PWA physical acceptance and supported provider live-login needs; mark unrun conditions.
4. Propose a canonical capability continuity matrix and coverage for all 14 G00–G13 beta gates, including donor-removal test.

## Acceptance
Ratification/source status matrix, import exclusion list, real verification plan and independent P0 risk audit.

## Required output
Commit exactly one evidence report `integration/worker-reports/W00-F.md` (explicit disjoint report-path exception) with donor repo/branch/SHA, source files and ownership, tested-vs-proposed status, copied/not-copied decisions, security/privacy class, verification commands (mark UNRUN), blocker list and instructions to successor lane W1-F. Provide HEAD SHA. **Do not call anything successfully integrated.**
