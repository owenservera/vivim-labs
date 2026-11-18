# W01-A — Native Rust and Tauri import

You are the Proposal 5 **Wave 1, lane A** worker. Pin shared `integrate/proposal5-beta-r001` HEAD after W0-G GO. Create isolated branch `work/p5-w01-a-native` from that exact SHA, never from donor main. Owned write scope: **src-tauri/, core/programmability/, native/hqkg/, native/protocol/ and native/node0/ only**. Leave root manifests/locks and shared integration merging to W1-G.

## Mission — Physical Source Landing and Minimal Import
Read Proposal 5 documents 01–09 and W0 gate source-lock before moving files. ARBOR must be fully internal: donor repositories are source inputs only, never runtime/build dependencies. Preserve current code paths and working behavior; no feature/UI redesign. One Host JS effect writer, no new graph store, no private data or secrets in Git. Only Gate G edits root Cargo/Bun manifests and lockfiles or merges to integration/main.

1. Import source-preserving Harness src-tauri and core/programmability dependency closure into those exact ARBOR paths, retaining package identity, build scripts and tests. Do not run donor binaries from outside.
2. Import exact HQKG Rust source under native/hqkg including algorithms, typed DAG, engine registry, importer, tests, licensing and relevant docs. Preserve public hqkg API and features; do not rewrite algorithms.
3. Stage minimal source files/README for native/protocol and native/node0 only if needed to compile the imported workspace; label prototypes nonproduction.
4. Inspect source tree for local machine paths/secrets; exclude target/, profile/DB, build outputs. Do NOT edit root Cargo.toml/Cargo.lock (G owns them).
5. Run isolated Rust tests where possible with explicit source SHA; report Windows/MSVC/security blockers separately rather than claiming complete native wiring.

## Acceptance
Rust/Tauri source exists fully INSIDE ARBOR, import ancestry recorded, no external path dep, isolated tests and unrun cases reported.

Commit source, tests and a report `integration/worker-reports/W01-A.md` listing base/head, donor source/commit, included/excluded files, dependency closure, build/test commands and real exit codes, cross-lane interface requests, security findings and status. Never mark a missing provider login/test as passed. Report **HEAD_SHA** to gate G, do not merge.
