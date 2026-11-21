# VLANG R000 — Canonical protocol deepening and convergence

Status: SCAFFOLD / not yet executed. VIVIM Labs research and implementation preparation, not ARBOR runtime code.

Start with MASTER-R000.txt; run lanes 01–08 in parallel, then 09-CONVERGENCE. Produce a single canonical implementation-ready protocol, then execute VACP R001 on ARBOR main. No duplicate semantic engine.

Baseline: ARBOR bd516b9d1ecc0794b5af39bfda919371a488b019. Companion package: ../ARBOR-VLANG-UPGRADE (product-language work; do not duplicate).
## Agentic SDK interfaces

The research SDK includes an MCP stdio server, a supplementary LSP server, and a shared read-only ARBOR HQ annotation adapter. See [`sdk/AGENTIC-MCP-LSP.md`](sdk/AGENTIC-MCP-LSP.md) for setup, supported operations and verification limitations. This is research scaffolding, not yet ARBOR runtime wiring.
