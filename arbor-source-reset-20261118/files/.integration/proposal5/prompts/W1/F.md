# W01-F — Contracts, selected Labs and continuity evidence

You are the Proposal 5 **Wave 1, lane F** worker. Pin shared `integrate/proposal5-beta-r001` HEAD after W0-G GO. Create isolated branch `work/p5-w01-f-verify` from that exact SHA, never from donor main. Owned write scope: **contracts/, capability-spaces/, tests/, disjoint worker reports**. Leave root manifests/locks and shared integration merging to W1-G.

## Mission — Physical Source Landing and Minimal Import
Read Proposal 5 documents 01–09 and W0 gate source-lock before moving files. ARBOR must be fully internal: donor repositories are source inputs only, never runtime/build dependencies. Preserve current code paths and working behavior; no feature/UI redesign. One Host JS effect writer, no new graph store, no private data or secrets in Git. Only Gate G edits root Cargo/Bun manifests and lockfiles or merges to integration/main.

1. Import selected accepted Labs contract definitions, synthetic fixtures and source-backed components into owning ARBOR contracts/tests/capability-space folders; no Labs runtime path imports or claims that design documents are code.
2. Create an initial read-only source-derived capability continuity inventory from Host/HQ/TierFlow/Rust source IDs, marking SOURCE_PRESENT vs TESTED vs AVAILABLE; never duplicate executable action catalog.
3. Port baseline compatibility tests that detect altered action IDs/risks, personal source class, exact receipt states and UI semantic mapping.
4. Build source-vs-destination file provenance ledger proposals for G without writing shared integration/source-lock.json.
5. Run source privacy/secret/path lint on imported code and record clean/blocked status; don't inspect actual personal credentials.

## Acceptance
Selected Labs fixtures/contracts and generated capability metadata inside ARBOR, no fake read/write authority, report any hidden external links.

Commit source, tests and a report `integration/worker-reports/W01-F.md` listing base/head, donor source/commit, included/excluded files, dependency closure, build/test commands and real exit codes, cross-lane interface requests, security findings and status. Never mark a missing provider login/test as passed. Report **HEAD_SHA** to gate G, do not merge.
