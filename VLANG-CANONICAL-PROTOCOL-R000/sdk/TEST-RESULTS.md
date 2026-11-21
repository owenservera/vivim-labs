# SDK R0 verification ledger — 2026-11-21

Scope: **local reconstructed source mirror**, not ARBOR main or a GitHub Actions checkout. Node v22.16.0.

- `node --check src/index.mjs`: passed on local mirror.
- `node --test test/sdk.test.mjs`: **8 passed, 0 failed**, on local mirror.
- Contract cases covered: source hash binding; no runtime authority; stale source refusal before parsing; unsafe/unsupported path refusal; Rust file-header limitation; wrong-source record rejection; independent context verdict; lineage refusal on relocation; missing-port refusal.
- Tests inject bounded mock parser/context ports; they **do not** prove real `hq/lib/annotation-v2.mjs` or `hq/lib/context.mjs` integration, filesystem freshness, semantic registry provenance, tests/receipts, or Rust compilation.
- Exact GitHub repository copy not checked out locally; tests there are **UNRUN**.
- No ARBOR files changed by this package.

Next proof: execute on a real ARBOR checkout at a pinned commit with the existing HQ parser and source-aware `verifyContext`, then run production-adapter negative corpus; record exact commands and commit hashes.
