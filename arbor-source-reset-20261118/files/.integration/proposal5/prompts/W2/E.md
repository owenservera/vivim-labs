# W02-E — Paired PWA and TierFlow internal view path

Wave 2 of ARBOR Proposal 5. Start from **W1-G GO HEAD** on `integrate/proposal5-beta-r001`, create `work/p5-w02-e-ui`. Write only: **apps/mobile-relay/, packages/tierflow/, owned view adapters** and disjoint `integration/worker-reports/W02-E.md`. No pushes to integration/main. Coordinate cross-owner source/protocol changes via your report, not by editing another lane's files.

## Mission — Internal Build, Native Security and Contract Wiring
Read all Proposal 5 architecture and current W1-G gate. Treat source-preserving import as the baseline: fully internal ARBOR runtime, no donor checkouts or servers after import, no wholesale JS/Rust rewrite, no new UI feature. Host JS is sole effects/World writer; native and UI proposals inert until Host admission. No secrets/user data. Only G merges and owns root Cargo/Bun files.

1. Fix mobile and TierFlow internal relative references/imports after source relocation; one PWA app and one active visual publisher.
2. Preserve existing mobile pairing/direct phone security and source UI routing, using the B-owned Host endpoints; no second mobile control interpreter as authority.
3. Connect a minimally sufficient source-linked UI status/command read view through imported TierFlow components, with a no-op fallback to existing working UI when a view is unverified.
4. Test PWA static bundle/service worker and TierFlow typed compile/projection, including refusal of stale source revisions; avoid new visual features and layout redesign.

## Required proof
Internal mobile/TierFlow compile and source-backed view continuity; no external renderer service.

Commit source/tests/report with base/head SHA, exact import/adapter paths, test commands and exit codes, Rust/Bun/Windows caveats, source and authorization scopes, unavailable status and blockers. Report HEAD_SHA to the serial W2-G integrator; do not merge.
