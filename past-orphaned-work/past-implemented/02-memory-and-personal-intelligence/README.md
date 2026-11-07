# 02-memory-and-personal-intelligence

**Shelf:** Past implemented · **Status:** intake only.

- **IMP-MEM-01 — Cortex predictive scheduler**: [historical source](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/backend/src/cortex/prediction.rs). Source catalog attests code; runtime integration unverified
- **IMP-MEM-02 — Cortex memory tier model**: [historical source](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/backend/src/cortex/memory_tier.rs). Source catalog attests code; investigate actual quantization separately
- **IMP-MEM-03 — Legacy memory backend compression hook**: [historical source](https://github.com/owenservera/vivim-final/blob/main/src/engines/memory/memory-backend.ts). Pre-compress and session switch hooks; runtime invocation unverified
- **IMP-MEM-04 — Current receipt-backed working memory**: [historical source](https://github.com/owenservera/repos-hq/blob/main/research/personal-intelligence/working-memory/index.mjs). CURRENT implementation, compare; not orphan candidate

**Next depth pass:** inspect actual code, tests, imports and current consumers; compare with canonical Repos HQ / TierFlow / Harness contracts; record benchmarks and permission negatives before KEEP/ADAPT/RETIRE decisions.
