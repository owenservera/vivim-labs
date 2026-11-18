# W01-D — Internal operational HQ import

You are the Proposal 5 **Wave 1, lane D** worker. Pin shared `integrate/proposal5-beta-r001` HEAD after W0-G GO. Create isolated branch `work/p5-w01-d-hq` from that exact SHA, never from donor main. Owned write scope: **hq/ only**. Leave root manifests/locks and shared integration merging to W1-G.

## Mission — Physical Source Landing and Minimal Import
Read Proposal 5 documents 01–09 and W0 gate source-lock before moving files. ARBOR must be fully internal: donor repositories are source inputs only, never runtime/build dependencies. Preserve current code paths and working behavior; no feature/UI redesign. One Host JS effect writer, no new graph store, no private data or secrets in Git. Only Gate G edits root Cargo/Bun manifests and lockfiles or merges to integration/main.

1. Copy the *required transitive source closure* of HQ hq/bin and hq/lib indexer/search/registry/context/wiki/capability/CLI functions into ARBOR/hq, not an outside symlink or an external HQ API.
2. Preserve relative imports, JS tests, deterministic context/wiki result behavior and original source IDs where compatible; package runtime requirements (node:sqlite/Node/Bun) without rewriting algorithms.
3. Do not import old HQ developer indexes, user DBs, secrets, source cache or giant debug output. Provide safe generated defaults for user-selected project roots.
4. Run internal HQ tests on a synthetic local project and record missing imports and compatibility requirements for the root package integrator.

## Acceptance
HQ operational JS code and source dependencies live in ARBOR/hq, CLI tests run or explicitly blocked; no external HQ checkout.

Commit source, tests and a report `integration/worker-reports/W01-D.md` listing base/head, donor source/commit, included/excluded files, dependency closure, build/test commands and real exit codes, cross-lane interface requests, security findings and status. Never mark a missing provider login/test as passed. Report **HEAD_SHA** to gate G, do not merge.
