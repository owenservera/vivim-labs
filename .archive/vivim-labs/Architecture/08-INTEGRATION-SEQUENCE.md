# First Integrations — Compounding Sequence and Dual Configurations
Date: 2026-11-07
Status: proposed execution map, not implementation authorization
Philosophy: [07-INTEGRATION-PHILOSOPHY](07-INTEGRATION-PHILOSOPHY.md)

## Selection logic
Start by connecting the **full existing HQ knowledge/command/wiki loop**, not only a code index: deep code and KB intelligence, stable semantic identities, action and natural-language routing, CLI/MCP commands, auto-generated/programmable wiki, Rust graph projection, and a read-only visual/observatory. Integrate Host/TierFlow effect paths only after write authorities are visible and test evidence exists. Avoid a new registry or a dev-only runtime becoming a product dependency.

## Sequence and compounding benefit
| Stage | First integration | Immediate verified deliverable sought | How it accelerates successors | Internal DEV-only pieces | End-user PRODUCT shape |
|---|---|---|---|---|---|
| 0 | Baseline/census | Fresh SHAs, source paths, active CLI commands, engines, data stores, writer inventory and gaps | Enables exact dependency ordering, prevents duplicate work | Repo-wide crawls, source explorer, detailed debug logs | Lightweight registered capability and health discovery |
| 1 | **HQ unified intelligence + language/command + generated/programmable wiki** → architecture graph inventory | Real source-backed nodes AND stable semantic operations, NL/action routing, CLI/MCP bindings and generated wiki pages | The next integration gets discoverability, human/agent commands and an automatically refreshed documentation/control surface | Broad code indexing, route traces, mock action catalogues, rich wiki diagnostics | Curated capabilities, user-scoped NL/CLI/UI actions and generated user-relevant wiki |
| 2 | HQ semantic/kernel → Rust HQKGX1 projection | One durable entity and links imported through existing Rust importer, source-pinned and replayable | Supplies a trusted graph model for CLI, UI, KB and dataflow integration; exposes source-consistency problems early | Full import fixtures, diff inspectors, deterministic replay scripts | Bounded versioned graph synchronization plus minimal validated adapters |
| 3 | Rust query/control ↔ HQ CLI | Commands querying same entity, revision and relationships, with doctor and typed receipt | Makes later engine/adapter integrations testable via reusable CLI recipes; improves agent usability | Raw operator CLI, verbose trace, synthetic failure tests, read-only inspector | Stable typed operation IDs via limited CLI/UI/NL routing; Host owns effects |
| 4 | Graph → read-only Visual UI / Labs observatory | Real graph-derived engine/dataflow map with provenance, stale markers and revision | Shows missing edges and change propagation for every later integration; enables rapid developer feedback | Full system graph, dependency overlays, experimental debug panels | Scoped graph views/visual documents; no dev instrumentation by default |
| 5 | Real TierFlow integration | Job/action/task nodes, emitted runtime evidence, existing execution owner respected | Tests orchestration and feedback path through shared graph | Synthetic workflows, high-cardinality tracing, failure injection | Production adapter exposing permitted job state/actions |
| 6 | Host/Studio/Android evidence and approvals | Source-native receipts and action references joined to graph without shadow writes | Unlocks Oracle and product-grade automation with verifiable authority | Lab signed-in profiles, test approvals, device matrix | Owner consent, native permission enforcement and secure receipts |
| 7 | Oracle + continuous graph | Typed planning/query over known engine/data/CLI operation graph; no bypass | Every new integration becomes discoverable and usable, enabling self-accelerating expansion | Agent debug replay, experiment sandboxes | Owner-scoped personal agent with safe intent/execution separation |

Stages 1–4 are the first compounding integration campaign, but stage 0 is necessary to avoid false conclusions. The sequence is a hypothesis that should be reprioritized from actual dependency evidence, not treated as an inflexible waterfall.

## Very first two useful integrations, in detail

### Stage 1 — HQ unified intelligence/command/wiki → graph architecture inventory
**Inputs:** Repos HQ project registry, file/symbol/import index, KB/context, semantic registry, action registry/router, real CLI and MCP routes, generated wiki, programmable wiki preview/rollback, confirmed Rust node kinds, C4 identity crosswalk, schemas/capability records.
**Transform:** discover engine/data/CLI candidates; bind each to implementation/source SHA, typed semantic operation, action-routing status, generated wiki entry and UI/NL candidates where real; classify implemented/wired/declared/research-only/unknown; map owner/read/write behavior; refuse invented canonical IDs and automatic duplicate merges.
**Output:** a bounded deterministic graph candidate inventory, shared command/action map, generated wiki projection and completeness/drift report. Initially it may be generated as a projection for inspection; only validated owners may write canonical knowledge.
**Why first:** every later integration gains discovery, dependency paths, a common human/agent command surface, automatically generated documentation, implementation evidence and an unsafe/missing seam report.
**DEV:** scan relevant local repos; show implementation detail, route traces, synthetic intent cases, programmable wiki tests, failure modes and unresolved duplicates.
**PRODUCT:** register only validated capabilities and user-relevant KB/wiki views; expose same typed operations through scoped NL, CLI and UI; no blanket repo crawler, traces or lab context.
**Gate:** mapped candidate points to source and operation/wiki identity; one real NL/command candidate resolves deterministically and is visible in generated wiki; declared/unwired is flagged; unknowns fail closed; no unauthorized writes; repeat run stable.

### Stage 2 — HQ canonical KB, command and wiki identities → Rust graph persistence
**Inputs:** a curated versioned semantic object, an associated command/action identity and generated wiki projection with source/evidence, plus HQKGX1 exchange schema/manifest.
**Transform:** export, verify digest and stable IDs, stage atomically in importer, fail on collisions/invalid refs, persist journal under one writer.
**Output:** Rust graph journal containing that object and relationships, with replay/no-op/diagnostic evidence and query result traceable to HQ revision.
**Why second:** subsequent UI, CLI, agent, wiki, dataflow and observability integrations all share a real source- and operation-linked graph rather than hand-made sample graphs.
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


For the corrected, integrated scope and source evidence, see [09 — Unified HQ Intelligence, Commands and Wiki](09-UNIFIED-INTELLIGENCE-COMMAND-WIKI.md).
