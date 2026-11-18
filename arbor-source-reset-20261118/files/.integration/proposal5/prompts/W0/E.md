# W00-E — Mobile PWA, TierFlow and visual runtime

## Role and launch
You own **Wave 0, lane E** of ARBOR Proposal 5. Begin from the exact shared integration branch `integrate/proposal5-beta-r001` HEAD pinned by the wave orchestrator. Create your independent branch `work/p5-w00-e-ui`. **W0 is source-read-only; no product code edits.**

**Shared plan:** Read `.integration/proposal5/README.md`, `01`–`09` documents and `08-BETA-GATES.json`. ARBOR is a single self-contained product; no external donor repo/service runtime wiring. Preserve working JS Host, Studio, PWA, HQ operational JS, HQKG Rust and TierFlow; no unnecessary features or UI redesign. Existing Host is sole effect/World writer. Do not commit secrets or user data. Workers never merge into integration/main; only Wave G merges.

## Mission: Source Truth and Import Freeze
Owned source inspection: apps/mobile-relay/, packages/tierflow/ (read-only in W0).

1. Inventory paired mobile PWA runtime and service worker, direct-phone authenticated transport, signing/pairing/revocation, offline behavior, and source of visual status.
2. Inventory TierFlow actual renderer, command IR/crosswalk, phone adapter, UI publisher, optional Rust/server components and testable package imports.
3. Identify duplicate app shells, planners and mobile execution authorities to avoid, without premature UI redesign.
4. Record working phone journeys, visual components suitable for direct import and which Labs R005/R007 concepts are only synthetic/design.

## Acceptance
Mobile/TierFlow file closure, exact build/test scripts, one-app UI ownership and evidence-backed beta visual scope.

## Required output
Commit exactly one evidence report `integration/worker-reports/W00-E.md` (explicit disjoint report-path exception) with donor repo/branch/SHA, source files and ownership, tested-vs-proposed status, copied/not-copied decisions, security/privacy class, verification commands (mark UNRUN), blocker list and instructions to successor lane W1-E. Provide HEAD SHA. **Do not call anything successfully integrated.**
