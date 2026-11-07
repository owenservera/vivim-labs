# 01-context-and-dcb

**Shelf:** Past implemented · **Status:** intake only.

- **IMP-CTX-01 — Context thermodynamics (Rust)**: [historical source](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/backend/src/cortex/thermodynamics.rs). Rust implementation present; current DCB feature parity unverified
- **IMP-CTX-02 — Context thermodynamics (TS)**: [historical source](https://github.com/owenservera/vivim-platform/blob/main/server/src/context/context-thermodynamics.ts). Separate implementation; compare semantics and avoid cloning
- **IMP-CTX-03 — Cortex assembly and context diff**: [historical source](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/backend/src/cortex/mod.rs). Rust composition code exists; end-to-end consumers unverified
- **IMP-CTX-04 — Canonical current DCB (comparison target)**: [historical source](https://github.com/owenservera/repos-hq/blob/main/hq/lib/context.mjs). CURRENT implementation, not orphan candidate; CLI/MCP/OpenCode present

**Next depth pass:** inspect actual code, tests, imports and current consumers; compare with canonical Repos HQ / TierFlow / Harness contracts; record benchmarks and permission negatives before KEEP/ADAPT/RETIRE decisions.
