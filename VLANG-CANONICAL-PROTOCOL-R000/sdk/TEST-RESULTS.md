# SDK R0 verification ledger — 2026-11-21

Scope: **local reconstructed source mirror**, not ARBOR main or a GitHub Actions checkout. Node v22.16.0.

- `node --check src/index.mjs`: passed on local mirror.
- `node --test test/sdk.test.mjs`: **8 passed, 0 failed**, on local mirror.
- Contract cases covered: source hash binding; no runtime authority; stale source refusal before parsing; unsafe/unsupported path refusal; Rust file-header limitation; wrong-source record rejection; independent context verdict; lineage refusal on relocation; missing-port refusal.
- Tests inject bounded mock parser/context ports; they **do not** prove real `hq/lib/annotation-v2.mjs` or `hq/lib/context.mjs` integration, filesystem freshness, semantic registry provenance, tests/receipts, or Rust compilation.
- Exact GitHub repository copy not checked out locally; tests there are **UNRUN**.
- No ARBOR files changed by this package.

Next proof: execute on a real ARBOR checkout at a pinned commit with the existing HQ parser and source-aware `verifyContext`, then run production-adapter negative corpus; record exact commands and commit hashes.

## 2026-11-21 — MCP/LSP research scaffold

- Five new agentic interface modules pass `node --check` on the local worktree mirror: `adapters/arbor-hq.mjs`, `servers/{mcp,lsp}.mjs`, `bin/{vlang-mcp,vlang-lsp}.mjs`.
- `node --test test/*.test.mjs`: **11 passed, 0 failed**. Eight prior SDK tests plus three new source adapter filesystem tests. New tests use an isolated parser fixture and do NOT establish production HQ parser compatibility.
- Real repo source package installation: **UNRUN** (registry DNS unavailable in this execution environment).
- MCP 2026-07-28 session/tool handshake: **UNRUN**.
- LSP 3.17 initialize, framed transport, hover and diagnostics against a live editor: **UNRUN**.
- ARBOR HQ parser and context integration against a cloned current main: **UNRUN**. The context port deliberately refuses use until a real verifier is connected.
- Labs `main` commits: `16b3d0400f4b3da259644f1101f1fb559c23ac0b` (MCP/LSP sources), `f99e5b7a2f9d389e05780523c59c6e2afc779323` (adapter tests).
