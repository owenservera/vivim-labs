# W01-C — Studio and harness import

You are the Proposal 5 **Wave 1, lane C** worker. Pin shared `integrate/proposal5-beta-r001` HEAD after W0-G GO. Create isolated branch `work/p5-w01-c-studio` from that exact SHA, never from donor main. Owned write scope: **studio/, harness/**. Leave root manifests/locks and shared integration merging to W1-G.

## Mission — Physical Source Landing and Minimal Import
Read Proposal 5 documents 01–09 and W0 gate source-lock before moving files. ARBOR must be fully internal: donor repositories are source inputs only, never runtime/build dependencies. Preserve current code paths and working behavior; no feature/UI redesign. One Host JS effect writer, no new graph store, no private data or secrets in Git. Only Gate G edits root Cargo/Bun manifests and lockfiles or merges to integration/main.

1. Physically import current signed-in Chrome/Studio source and required harness helpers under original in-repo paths; maintain real current provider integration and CDP session handling.
2. Preserve supported conversation read/send, exact target/thread, capture evidence and uncertain-send protection. Do not add direct provider APIs, Playwright/Puppeteer or a new execution owner.
3. Copy tests, assets, documentation needed by runnable source, while excluding Chrome profile, cookies, tokens, user media, logs and historical build outputs.
4. Run Studio/harness synthetic tests where possible and pin test/runtime versions; provide expected Host/Studio compatibility contract to lane B and G.

## Acceptance
Source-imported Studio and harness work offline from donor source checkouts for synthetic tests; honest live-login blockers.

Commit source, tests and a report `integration/worker-reports/W01-C.md` listing base/head, donor source/commit, included/excluded files, dependency closure, build/test commands and real exit codes, cross-lane interface requests, security findings and status. Never mark a missing provider login/test as passed. Report **HEAD_SHA** to gate G, do not merge.
