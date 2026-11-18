# W01-B — Host and World import

You are the Proposal 5 **Wave 1, lane B** worker. Pin shared `integrate/proposal5-beta-r001` HEAD after W0-G GO. Create isolated branch `work/p5-w01-b-host` from that exact SHA, never from donor main. Owned write scope: **apps/vivim-host/**. Leave root manifests/locks and shared integration merging to W1-G.

## Mission — Physical Source Landing and Minimal Import
Read Proposal 5 documents 01–09 and W0 gate source-lock before moving files. ARBOR must be fully internal: donor repositories are source inputs only, never runtime/build dependencies. Preserve current code paths and working behavior; no feature/UI redesign. One Host JS effect writer, no new graph store, no private data or secrets in Git. Only Gate G edits root Cargo/Bun manifests and lockfiles or merges to integration/main.

1. Copy current canonical Harness Host subtree preserving paths/imports and its complete verified internal dependency closure within Host. Do not import user data, credentials, logs, production journals or SQLite user DBs.
2. Preserve action catalog, NLCL planner, approval, one effect claim/journal/receipt/recovery implementation and owner World storage code exactly unless required import fix.
3. Carry over internal status/onboarding/media/World/mission modules used by current runtime; classify optional source-only files rather than silently deleting them.
4. Run available Host source-level tests with synthetic fixtures; document all imports that require Studio/harness/mobile (other lanes own), and supply integration linker notes.
5. Do not modify src-tauri, studio, packages/tierflow or root dependency locks; leave cross-owner changes as explicit requests.

## Acceptance
Source-preserving Host subtree and working synthetic test results, one-writer invariant, unresolved cross-owner import list.

Commit source, tests and a report `integration/worker-reports/W01-B.md` listing base/head, donor source/commit, included/excluded files, dependency closure, build/test commands and real exit codes, cross-lane interface requests, security findings and status. Never mark a missing provider login/test as passed. Report **HEAD_SHA** to gate G, do not merge.
