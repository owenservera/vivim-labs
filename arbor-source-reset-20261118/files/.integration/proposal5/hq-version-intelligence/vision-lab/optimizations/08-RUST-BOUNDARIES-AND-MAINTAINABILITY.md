# 08 — Native boundaries, maintainability and developer throughput
## The point is less total complexity, not more Rust
ARBOR already has `native/hqkg` (std-heavy graph, journaling, pure compute) and `core/programmability` (typed pure candidates). Its JS/TS surfaces already have real ownership: Host effect authority, Studio browser/session evidence, TierFlow publisher, HQ development intelligence.

A native rewrite is compelling when **measured hotpath execution + shared deterministic reuse + fewer allocations** beats **FFI cost + installer growth + build time + debugging cost + risk**. Source existence and hypothetical frequency are not enough.

## Four implementation strategies
1. **Don't do the work:** skip stale refreshes, avoid duplicate materializations, align idle wakes, eliminate unused projections.
2. **Do less work:** incremental invalidation, query plan/covering index, smaller contextual input, stream instead of buffer.
3. **Do work more efficiently:** fix algorithm/data structure, precompute verified immutable lookups, bounded batching, safer framing.
4. **Move pure computation:** JS→Rust only behind a stable typed ABI, with differential tests, cost measurements and switchback. No Host effect move without a separate owner decision.

## Maintainability as a performance metric
Track cold/warm build minutes, installer bytes, Rust target directory size, Node/Bun cache size, integration-test minutes, number of canonical owners per capability, module fanout, import-closure defects, duplicate type definitions and dependency upgrades. A 20% runtime gain that doubles the build system and breaks rollback may be worse.

### Source-grounded build pressure
`native/hqkg/Cargo.toml` has `[profile.dev] debug=0, incremental=false` and records a historical 4.9GiB debug target estimate. **Do not reuse the historical number as current disk usage**. Re-measure on owner hardware with actual compile commands, record target dir before/after, and compare diagnostic quality.

### One graph; one publisher; one effect writer
- Rust HQKG: reusable pure graph computation and optional adapters; no duplicate canonical graph.
- Host: grants, World/effect actions, delivery/receipt authority.
- Studio: signed-in browser sender and captured evidence.
- TierFlow: canonical visual publisher/revision store.
- HQ: development indexing/semantic analysis.
Any suggested replacement must be an **adapter to these owners**, not a new parallel engine competing to write state.

## Good native candidates vs bad candidates
**Good to profile:** bounded BFS, deterministic graph scoring, transcript delta folding, repeated codec parse, immutable source identity hashing, safe DAG cache and normalization.

**Bad first migrations:** Host consent, session sender, browser I/O, UI rendering, durable one-writer ledgers, cross-device grants, personal original storage. The performance target does not justify relocating security authority.

## Future constraints
PWA-first; low disk, offline, weak CPU and impaired network must be explicit in benchmark scenarios. No provider APIs, no Playwright/Puppeteer, no hidden daemon created merely for profiling. Keep a testable JS fallback if shared Rust substitution is pursued.
