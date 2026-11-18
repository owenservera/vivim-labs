# W00-C — Studio and signed-in browser

## Role and launch
You own **Wave 0, lane C** of ARBOR Proposal 5. Begin from the exact shared integration branch `integrate/proposal5-beta-r001` HEAD pinned by the wave orchestrator. Create your independent branch `work/p5-w00-c-studio`. **W0 is source-read-only; no product code edits.**

**Shared plan:** Read `.integration/proposal5/README.md`, `01`–`09` documents and `08-BETA-GATES.json`. ARBOR is a single self-contained product; no external donor repo/service runtime wiring. Preserve working JS Host, Studio, PWA, HQ operational JS, HQKG Rust and TierFlow; no unnecessary features or UI redesign. Existing Host is sole effect/World writer. Do not commit secrets or user data. Workers never merge into integration/main; only Wave G merges.

## Mission: Source Truth and Import Freeze
Owned source inspection: studio/, harness/ (read-only in W0).

1. Inventory Studio/harness entrypoints, signed-in Chrome profile CDP control and real send/receive flow with exact target/thread and effect receipt boundaries.
2. List internal imports, package dependencies/assets, process launch paths and relative-path assumptions required by Host.
3. Identify browser-source evidence/capture, provider failures and retry/uncertain-send guards. Distinguish known tested provider from designed adapter.
4. Flag provider APIs, Playwright/Puppeteer, secrets/profile files, old test-only branches and unsupported automation paths for exclusion.

## Acceptance
Selected Studio/harness import graph, source hashes, smoke commands and no-provider-API compatibility list.

## Required output
Commit exactly one evidence report `integration/worker-reports/W00-C.md` (explicit disjoint report-path exception) with donor repo/branch/SHA, source files and ownership, tested-vs-proposed status, copied/not-copied decisions, security/privacy class, verification commands (mark UNRUN), blocker list and instructions to successor lane W1-C. Provide HEAD SHA. **Do not call anything successfully integrated.**
