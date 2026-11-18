# W00-D — HQ native and operational development intelligence

## Role and launch
You own **Wave 0, lane D** of ARBOR Proposal 5. Begin from the exact shared integration branch `integrate/proposal5-beta-r001` HEAD pinned by the wave orchestrator. Create your independent branch `work/p5-w00-d-hq`. **W0 is source-read-only; no product code edits.**

**Shared plan:** Read `.integration/proposal5/README.md`, `01`–`09` documents and `08-BETA-GATES.json`. ARBOR is a single self-contained product; no external donor repo/service runtime wiring. Preserve working JS Host, Studio, PWA, HQ operational JS, HQKG Rust and TierFlow; no unnecessary features or UI redesign. Existing Host is sole effect/World writer. Do not commit secrets or user data. Workers never merge into integration/main; only Wave G merges.

## Mission: Source Truth and Import Freeze
Owned source inspection: hq/ and native/hqkg/ donor inspection (read-only in W0; A owns later native import).

1. Trace selected HQ operational CLI/bin and JS lib imports (index, schema, registry, graph, search, source semantic IDs, context, wiki/capabilities), including direct/transitive modules and data files.
2. Pin HQ main and relevant unmerged integration fixes. Identify Node/Bun runtime requirements, especially node:sqlite, and root package constraints.
3. Trace Rust HQKG source-to-product APIs without writing a second engine; identify licensing, dev profile memory/disk optimizations and exact tests.
4. Separate owner-private data from development index and identify what can run internally after donor repo checkout is absent.

## Acceptance
HQ JS import closure, native reuse crosswalk, source tree/version, test fixtures, disallowed external DB paths.

## Required output
Commit exactly one evidence report `integration/worker-reports/W00-D.md` (explicit disjoint report-path exception) with donor repo/branch/SHA, source files and ownership, tested-vs-proposed status, copied/not-copied decisions, security/privacy class, verification commands (mark UNRUN), blocker list and instructions to successor lane W1-D. Provide HEAD SHA. **Do not call anything successfully integrated.**
