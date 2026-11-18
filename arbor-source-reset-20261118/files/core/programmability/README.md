# Native programmability core — bounded Rust source spike

Owner: **vivim-web-harness product repository**. Source branch: `work/native-programmability-core-spike-20261110`.

This is an **in-process library**, not a daemon, governance layer, CLI, graph or configuration store. It formalizes one common contract: typed information sources + transformation steps + policy version caps -> a deterministic inert DAG with stable refusal reasons. It deliberately has **zero dependencies** and no Host effects. It does not authenticate proofs, authorize grants, load files, assemble DCB source content, run missions, dispatch AI turns, persist state or display UI.

`cargo test --manifest-path core/programmability/Cargo.toml` (requires Rust installed). Tests are authored but **not run in this environment** because cargo/rustc is unavailable.

### Product integration boundaries
- Existing Host `control/dispatcher.mjs`, `control/journal.mjs` keep exclusive effect authority and claim/receipt ownership.
- Existing DCB `dcb-v2-planner.mjs` and `dcb-dispatch-gate.mjs` keep exact-thread and per-source disclosure checks.
- Existing Studio `ai-orchestrator.mjs` keeps browser lease and send-once journaling.
- TierFlow remains visual projection. HQ Rust knowledge graph remains development research; do **not** import this crate into HQ as product authority.
- To ship: identify the canonical product native ABI/UniFFI plan for Windows **and** Android; add a trusted serialization/validation adapter; verify no downgrade of source-owner policy; compare JS and Rust outputs; benchmark; run Windows Host + physical Android acceptance. Until then, no effect path calls this module.

### Critical trust caveat
`PolicyLimits` is caller-supplied, not authenticated. A malicious caller can forge it; accepted programs are never execution permits. The Host must independently re-check **current** grants, policy revision, target catalog, privacy source disclosure and effect journal immediately before any effect. Digest means equality, not source authenticity.

### Design direction
Keep domain-specific IRs (commands, contexts, missions, recoveries, UI) and map them to one small native transformation core. Keep policy values in their existing authoritative stores. Do not introduce universal executable DSL, new DB or rules governance service. Expand native primitives only when proven cross-domain consumers need the semantics.
