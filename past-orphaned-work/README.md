# Past orphaned work — recovery index (documentation only)

**Status:** initial source-linked inventory, 2026-11-07. **Owner:** VIVIM Labs. **No production authority.**

This is an intake and traceability space for valuable earlier VIVIM algorithms, code, designs and theoretical research that may not have migrated to the current platform. **Orphaned means *unverified adoption*, not proven unused or defective.** Original repositories remain canonical for their historical source; Repos HQ remains the knowledge/context integration authority, TierFlow owns portable runtime concerns, and VIVIM Web Harness owns existing Host/device contracts. Do not clone their authority here.

## Two non-overlapping evidence shelves

- [past-implemented](past-implemented/README.md): source code demonstrably exists. Substatus: *source-present / tested-in-origin / consumer-observed / parity-migrated / production-verified*; only grant latter statuses with proof. An implementation may have originated in a former product but still be orphaned from today's runtime.
- [past-researched](past-researched/README.md): mathematical proposals, design notes, papers, benchmarks proposed, and algorithms not verified as executable. A researched concept can have a related implementation elsewhere: cross-link both shelves without conflating evidence.

## Shared high-level end-state platform taxonomy

Both shelves use matching domains so later audit lanes can compare source code versus research without forcing a premature architecture:

1. `01-context-and-dcb/`: relevance selection, information-theoretic optimization, budgets, cache, context diffs, audience projection.
2. `02-memory-and-personal-intelligence/`: working/episodic/semantic/procedural memory, consolidation, temporality, predictive scheduling, Oracle.
3. `03-knowledge-graph-and-retrieval/`: graph search, entity relationships, hybrid ranking, indexing, evidence provenance.
4. `04-capability-intelligence/`: discovery, registries, capability resolution, matching, confidence, policy constraints.
5. `05-parsing-and-protocols/`: streaming parsers, AST repair, structural induction, protocol discovery.
6. `06-automation-and-orchestration/`: planning, workflows, tool routing, device/agent coordination.
7. `07-self-healing-and-reliability/`: diagnosis, repair, recovery, drift, reconciliation, failure classification.
8. `08-optimization-and-resource-intelligence/`: scheduling, knapsack, compression, cost/quality, memory efficiency.
9. `09-security-privacy-and-authority/`: consent, provenance, revocation, safety, scoped execution.
10. `10-interface-and-device-intelligence/`: responsive projection, accessibility, generated UI, adaptive interaction.

This is a **folder taxonomy**, not approval for ten new engines. Individual entries can cross-reference another domain but have one primary folder.

## Initial source set and audit boundary

Included now: `owenservera/vivim-platform` (Edge PWA Cortex, server context, ACU, automation), `owenservera/vivim-final` (parsing, repair, kernel), `owenservera/code-intelligence-platform` (code indexing, learning and memory), plus migration targets `owenservera/repos-hq`, `owenservera/tierflow`, and `owenservera/vivim-web-harness`. `owenservera/cap-store` exists but its contents were not available in this audit; **uninspected**. No confident match was established for a separate `cap-intelligence` repository. Do not claim a complete capability-store audit.

**Explicit stop scope:** Context/DCB, memory, self-healing, optimization, parsing, automation and capability intelligence. Other domains are empty end-state scaffolds; do not survey everything else in this pass.

## Promotion and deeper-pass rule

Every future algorithm record needs: stable ID; original repository/path/commit; source kind (code vs research); problem solved; mathematical assumptions; existing caller evidence; closest current target/callsite; duplicate or conflicting authority; security boundary; test/benchmark status; recommendation KEEP / ADAPT / ALREADY MIGRATED / BENCHMARK / RETIRE / UNKNOWN; and acceptance proof. The initial shelf entries deliberately contain **no migration approval**. Before integrating, execute code-level equivalence and regression tests against the current DCB/kernel, measured recall-vs-token benchmarks and negative permission cases. Preserve unmodified originals and distinguish historical docs from running code.

## Initial finding

DCB is implemented in Repos HQ (`hq/lib/context.mjs`, `hq.context-bundle/1`) with CLI/MCP/OpenCode adapters and provenance/budget support, but session-to-DCB and cross-product consolidation have recorded gaps. Earlier `vivim-platform/edge-pwa` Cortex thermodynamics, adaptive assembly, predictive scheduling, SimHash/quantization, context differ, and ACU entropy scoring are preserved in source or research; **parity with today's DCB has not been proven**. VIVIM Final also has implemented parser/repair and healing subsystems. See the shelves for precise file-level links.

No source modifications, production migrations, deployments or external effects were made as part of this documentation survey.
