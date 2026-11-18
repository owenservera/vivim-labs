# W01-E — PWA and TierFlow source import

You are the Proposal 5 **Wave 1, lane E** worker. Pin shared `integrate/proposal5-beta-r001` HEAD after W0-G GO. Create isolated branch `work/p5-w01-e-ui` from that exact SHA, never from donor main. Owned write scope: **apps/mobile-relay/, packages/tierflow/ and designated view adapter**. Leave root manifests/locks and shared integration merging to W1-G.

## Mission — Physical Source Landing and Minimal Import
Read Proposal 5 documents 01–09 and W0 gate source-lock before moving files. ARBOR must be fully internal: donor repositories are source inputs only, never runtime/build dependencies. Preserve current code paths and working behavior; no feature/UI redesign. One Host JS effect writer, no new graph store, no private data or secrets in Git. Only Gate G edits root Cargo/Bun manifests and lockfiles or merges to integration/main.

1. Copy existing paired PWA under apps/mobile-relay preserving import paths and service worker/assets. Keep only one PWA/product identity; exclude APK-only duplicates from beta packaging where not needed.
2. Copy *selected working TierFlow source and its full dependency closure* into packages/tierflow, including typed visual command compiler and renderer tests, retaining upstream source/license lineage.
3. Do not create a new frontend, redesign screens, duplicate command parser/effect authority or require a TierFlow server at runtime.
4. Run existing PWA and TierFlow build/unit tests as currently possible, documenting peer dependencies and Host imports for integration G.

## Acceptance
Mobile and TierFlow code physically present in ARBOR, runnable internal test scripts or exact blockers; one publisher planned.

Commit source, tests and a report `integration/worker-reports/W01-E.md` listing base/head, donor source/commit, included/excluded files, dependency closure, build/test commands and real exit codes, cross-lane interface requests, security findings and status. Never mark a missing provider login/test as passed. Report **HEAD_SHA** to gate G, do not merge.
