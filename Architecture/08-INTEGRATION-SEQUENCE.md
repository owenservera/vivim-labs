# First Integrations — Compounding Sequence and Dual Configurations
Date: 2026-11-07
Status: proposed execution map, not implementation authorization
Philosophy: [07-INTEGRATION-PHILOSOPHY](07-INTEGRATION-PHILOSOPHY.md)

## Selection logic
Start by connecting the facilities that let the system inspect its own architecture and validate future work: existing HQ indexer, stable semantic identities, Rust graph projection, Rust CLI/doctor, and a read-only visual/observatory. Integrate Host/TierFlow effect paths only after write authorities are visible and test evidence exists. Avoid a new registry or a dev-only runtime becoming a product dependency.

## Sequence and compounding benefit
| Stage | First integration | Immediate verified deliverable sought | How it accelerates successors | Internal DEV-only pieces | End-user PRODUCT shape |
|---|---|---|---|---|---|
| 0 | Baseline/census | Fresh SHAs, source paths, active CLI commands, engines, data stores, writer inventory and gaps | Enables exact dependency ordering, prevents duplicate work | Repo-wide crawls, source explorer, detailed debug logs | Lightweight registered capability and health discovery |
| 1 | HQ Index → graph architecture inventory | Real source-backed EngineSlot, Database, Dataset, Transformation and Command candidates with identity/evidence | The next connector knows where source code and schemas live, and which writers are owners | Broad code indexing, unresolved candidate crosswalk, rich diagnostics, sample corpus | Curated capability/resource descriptions; no unrelated repo source leakage |
| 2 | HQ semantic/kernel → Rust HQKGX1 projection | One durable entity and links imported through existing Rust importer, source-pinned and replayable | Supplies a trusted graph model for CLI, UI, KB and dataflow integration; exposes source-consistency problems early | Full import fixtures, diff inspectors, deterministic replay scripts | Bounded versioned graph synchronization plus minimal validated adapters |
| 3 | Rust query/control ↔ HQ CLI | Commands querying same entity, revision and relationships, with doctor and typed receipt | Makes later engine/adapter integrations testable via reusable CLI recipes; improves agent usability | Raw operator CLI, verbose trace, synthetic failure tests, read-only inspector | Stable typed operation IDs via limited CLI/UI/NL routing; Host owns effects |
| 4 | Graph → read-only Visual UI / Labs observatory | Real graph-derived engine/dataflow map with provenance, stale markers and revision | Shows missing edges and change propagation for every later integration; enables rapid developer feedback | Full system graph, dependency overlays, experimental debug panels | Scoped graph views/visual documents; no dev instrumentation by default |
| 5 | Real TierFlow integration | Job/action/task nodes, emitted runtime evidence, existing execution owner respected | Tests orchestration and feedback path through shared graph | Synthetic workflows, high-cardinality tracing, failure injection | Production adapter exposing permitted job state/actions |
| 6 | Host/Studio/Android evidence and approvals | Source-native receipts and action references joined to graph without shadow writes | Unlocks Oracle and product-grade automation with verifiable authority | Lab signed-in profiles, test approvals, device matrix | Owner consent, native permission enforcement and secure receipts |
| 7 | Oracle + continuous graph | Typed planning/query over known engine/data/CLI operation graph; no bypass | Every new integration becomes discoverable and usable, enabling self-accelerating expansion | Agent debug replay, experiment sandboxes | Owner-scoped personal agent with safe intent/execution separation |

Stages 1–4 are the first compounding integration campaign, but stage 0 is necessary to avoid false conclusions. The sequence is a hypothesis that should be reprioritized from actual dependency evidence, not treated as an inflexible waterfall.

## Very first two useful integrations, in detail

### Stage 1 — HQ indexed source → Rust graph architecture inventory
**Inputs:** Repos HQ project registry, file/symbol/import index, confirmed Rust node kinds, C4 identity crosswalk, source-registered schemas/capability records.
**Transform:** extract engine/data/CLI candidates; attach implementation location and source SHA; classify as observed, implemented, research-only or unknown; map owner/read/write behavior; refuse invented canonical IDs and automatic duplicate merges.
**Output:** a bounded, deterministic graph candidate inventory and side-by-side completeness report. Initially it may be generated as a projection for inspection; only validated owners may write canonical knowledge.
**Why first:** every later integration gains component discovery, dependency paths, machine-verifiable implementation evidence and a list of unsafe or missing seams.
**DEV:** scan every relevant local repo and expose implementation detail, failure modes and unresolved duplicates.
**PRODUCT:** register only validated capability/resource descriptions relevant to the user's installation; no blanket code crawler or lab context.
**Gate:** every mapped candidate links to a source, no new canonical registry, unknowns flagged, no unauthorized writes, repeat run yields stable results.

### Stage 2 — HQ canonical KB identity → Rust graph persistence
**Inputs:** one curated, versioned semantic object and its source/evidence, plus existing HQKGX1 export schema/manifest.
**Transform:** export, verify digest and stable IDs, stage atomically in importer, fail on collisions/invalid refs, persist journal under one writer.
**Output:** Rust graph journal containing that object and relationships, with replay/no-op/diagnostic evidence and query result traceable to HQ revision.
**Why second:** all subsequent UI, CLI, dataflow and observability features share one real, provenance-bearing graph example rather than hand-made sample graphs.
**DEV:** golden fixtures, invalid manifests, rollback/recovery tests, replay and compare tooling.
**PRODUCT:** bounded versioned sync/adapter, trust and minimal resources; no dev fixture dependency.
**Gate:** fresh native Rust/Bun tests, exact source identity match, no-op unchanged import, rejected collision, recover after restart, no second KB writer.

### Stage 3 — Rust graph ↔ shared CLI
Map an HQ CLI query and Rust CLI query to the same source-linked entity and version, and cover CLI doctor, refusal and deterministic output. The CLI is a projection/interface, not a new authority. Ensure a short agent-friendly recipe; do not hide raw operator controls inside user-facing product UI.

### Stage 4 — Rust graph → UI/Labs
Render a read-only graph of real mapped engines/data/flows, including evidence freshness and unknowns; preserve Visual Engine document ownership and renderer. Lab explorer may be rich and source-wide; user-product view must be permission-scoped. Never confuse a rendered edge with product integration acceptance.

## Configuration boundary rule for every stage
For each implementation, record:
- **shared semantic core**: identity, graph node/edge meaning, typed operations, ownership, version pin, receipts.
- **dev profile**: test adapters, mock data, verbose traces, inventory breadth, fixtures, profiling, failure injection.
- **product profile**: only installed approved adapters, user-scoped data, bounded resources, security/grants, user-oriented UI/NL.
- **promotion gate**: parity tests with experimental flags off, no dev paths/imports in release dependency graph, no test credentials, actual Host/device evidence when effects are involved.
- **deletion plan**: expiry/replacement milestone for development-only scaffolding.

Do not implement two separate graphs, two canonical KB writers, or dev/product-specific meaning for the same capability ID.

## Measurement after each integration
Score (1) verified capability delivered, (2) number of successor integrations unlocked, (3) reusable test harness/CLI recipe, (4) cross-repo integration time reduced, (5) replay/idempotency and fresh-source correctness, (6) dev/product parity and contamination, (7) duplicate ownership conflicts, (8) verified native acceptance. Treat measured facts separately from estimates.

## First campaign review questions
- Does Rust need a new graph kind for commands/transforms, or can NodeKind::Other and links suffice without a second schema registry?
- Is existing HQ kernel the canonical curated writer and Rust a materialization, or are particular graph-topology domains genuinely Rust-owned? Clarify before Stage 2.
- Which read-only Visual Runtime entrypoint can show a Rust graph without a second visual-document writer?
- Does the existing CLI route through shared typed operations or are commands currently independent?
- What minimum graph query/identity evidence gives the next developer measurable speedup?
- Which components require actual Windows/Bun/Cargo/Android acceptance versus only source-level verification?

## Work authorization
This is a planning artifact. No production runtime, Rust core or source repository outside Labs is changed by this map. Only approved, proven integration slices may advance to product activation.
