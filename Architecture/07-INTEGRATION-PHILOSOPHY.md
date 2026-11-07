# VIVIM Integration Philosophy — Each Step Builds the Next
Date: 2026-11-07
Status: architectural execution philosophy; direction, not authorization for runtime changes
Parent: [One Graph Architecture](README.md)

## 1. Central law: integration must compound
**Every integration must both deliver an independently testable capability now and reduce the cost, risk, or uncertainty of at least one named later integration.** The first step must facilitate the second. A step that only builds bespoke scaffolding is not automatically valuable; its reusable outputs must be explicit.

We know the intended end state: one logical Rust graph, initially one physical Rust core, containing or representing engines, their generations, data resources, transformations, semantic knowledge, commands, evidence and lifecycle. We now sequence integrations to grow the actual system while it simultaneously becomes the instrument for inspecting, testing and accelerating subsequent integrations.

## 2. Integration is product development AND integration-tool development
Every wave has two deliverables:
1. **Production-relevant foundation**: an engine/node/adapter/operation/data path that fits the eventual end-user configuration, with stable identity, provenance, deterministic semantics and source authority.
2. **Internal development leverage**: discovery, CLI, traceability, benchmarks, replay, diagnostics, fixture generation, test automation, progress views and repair guidance used by subsequent waves.

Measure both. Do not claim feature completion merely because the internal tool reports it.

## 3. Two explicit configurations on the same semantics

### DEV / INTERNAL INTEGRATION CONFIG
Purpose: fast, transparent, reproducible implementation and dogfooding.
May include: full repository discovery, source-level symbol indexes, test-only engine drivers, research datasets, synthetic events, verbose traces, CLI diagnostics, integration test harness, replay/diff tools, observatory dashboards, fixture stores, deliberately simulated failures, frequent incremental updates.
Must include: isolation from personal/product data, controlled permissions, separate runtime data paths, revision/source pins, bounded resources, explicit opt-in experimental engines, and no false production certification.
May optimize for depth of instrumentation, control and developer throughput, even when not appropriate for a user installation.

### PRODUCT / END-USER CONFIG
Purpose: private, minimal, secure, reliable operation on user-owned devices.
Include only production-validated engine generations, required data, curated personal knowledge, portable typed commands, owner-visible permissions and receipts, necessary telemetry, data retention/revocation and recoverability. No developer-only control surfaces by default; no lab credentials, source inventories of unrelated repositories, debug traces, research experiments, or privileged bypass.
Supports desktop and paired mobile without assuming a centralized cloud, or making UI/NL interactions dependent on dev-only tooling.
Prioritize resource budgets, isolation, safe upgrades, privacy, accessibility and offline/degraded behavior.

**One graph/operation semantic model, two deployment policies and asset inventories; not two competing knowledge truths.**
No dev configuration may silently become product default. Product mode cannot inherit dev privileges. Promotion requires measured parity and explicitly accepted release evidence.

## 4. Integration styles
A. **Product-shaped first**: directly build a durable node/adapter/operation/graph relationship in its future home; use this when stable boundaries are already understood.
B. **Development accelerator / bridge**: temporarily join systems using a local helper, script, crosswalk, expanded instrumentation, or source scanner. Every bridge has a declared owner, bounded scope, parity tests, replacement/retirement condition, and no independent canonical writer.
C. **Dual-use seam** (preferred where viable): the same typed underlying operation powers the product path; dev mode adds observability, synthetic fixtures, introspection and tests as removable wrappers.
The irreversible mistake to avoid is promoting an integration shortcut into a permanent dependency simply because it sped up one campaign.

## 5. Design order for any integration
1. Identify next two or three integrations it should unlock.
2. Identify actual source owner and write authority; avoid competing stores, engine registries and permission owners.
3. Map input → transformation → output → write, plus resource/schema/CLI dependencies.
4. Decide which portion is final-product structure and which is temporary dev leverage.
5. Implement the smallest end-to-end path through the *real* existing owners and interfaces.
6. Make that path runnable from a deterministic CLI recipe (preferably few commands) and observable through the canonical graph.
7. Test correctness, replay, failure/refusal, stale data, restart, owner approval, and source revision.
8. Materialize receipts/trace links so the next integration can discover and reuse them.
9. Compare dev and product behavior and record what must be omitted, narrowed or adapted for release.
10. Integrate findings back into the architecture graph and update the next dependency order.

## 6. Selection criteria for the next work item
Prefer items with:
- high **downstream unlock** (how many future integrations depend on them),
- high **reuse** across HQ, Rust, UI, TierFlow, Host, agents,
- immediate **testability** against working code and real revisions,
- low **duplicate-authority risk**,
- bounded **implementation complexity**,
- high **production fitness** (even if current wrapper is dev-only).
Do not select by visibility or apparent completion percentage alone.

## 7. Non-negotiable rules
- One logical graph, initially one physical Rust core unless measurements demonstrate a reason to federate.
- One source-native authority per writable domain; graph nodes/reference projections do not grant write permission.
- Reuse HQ indexing, kernel, Rust EngineRegistry/Scheduler/Journal, C4 identity/deltas, Visual and Host owners; extend proven gaps, don't fork registries.
- Keep research algorithm definitions inert until trusted implemented drivers are validated.
- Keep internal diagnostics/debug/synthetic capabilities separate from production data, privacy and execution permissions.
- No provider API assumption, no Playwright/Puppeteer, no production action inferred from source-code or graph presence.
- A test receipt is evidence of the test performed, not blanket certification; live Windows/Android/Host gates remain required.
- No PRs/branches/Actions or force pushes in implementation flow without changed owner direction.
- All changes must support no-op repeat, version-pinned replay, explicit conflicts, recovery and reversible migration where meaningful.
- Improve the integration system by actually using it on the next integration; don't create a disconnected test framework.

## 8. Per-integration proof and handoff
Each integration should leave:
- Before/after topology: nodes/edges/engine slots/data stores/operations added or connected.
- Exact implementation pointers and source SHAs.
- Data input/output/write matrix and authority boundaries.
- Dev setup and expected product setup, with explicit nonportable parts.
- Tests/receipts and gaps, failure handling, rollback.
- At least one **named successor integration** accelerated by this work, and exactly how it will consume the output.
- Retirement or migration criteria for any temporary bridge.

## 9. Success metrics
- End-to-end time to integrate the *next* component falls.
- Number of capabilities discoverable and mapped to real code plus authority.
- Reusable verified integration recipes (not one-off scripts) and fixtures.
- Percent of graph projections with source/revision/evidence links.
- Reindex/import no-op ratio, stale detection and replay correctness.
- Native validation and rejection coverage.
- Product-dependency contamination from dev scaffolding: target zero.
- Duplicate canonical writers/registries introduced: target zero.

## 10. Immediate implementation posture
Start with graph-observable engine/data/CLI inventory from existing Repos HQ index, then stable source identity/KB → Rust projection, then native graph query + CLI evidence, then read-only Visual/Labs observability, then Host/TierFlow and authorized execution. Each step provides verified data, fixtures, change detection and reusable routes for the next step.

This document is the integration philosophy. Specific sequence and scope belong to a separately reviewed integration map. No live core migration or production activation is approved here.
