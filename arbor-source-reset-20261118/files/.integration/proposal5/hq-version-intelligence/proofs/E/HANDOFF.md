# HVI E — owner handoff (source-only)

**Gate:** `PREPARED_NOT_PROMOTED`, not G1 approval or beta acceptance. **Write branch:** `work/p5-hvi-w1-e-reports-r001`; **immutable worker base:** `f9823a5131e73e7562db5a4ec14e1ec481557d69`. Prerequisite S0 branch `work/p5-hvi-s0-source-lock-r001@2ef7a9f66419d681e040b459ba4762a63aa9564f` exposes `GO_TO_PARALLEL` and `workerBaseSha` in `proofs/S0/WORKER-BASE.json`.

## Implemented

`hq/version-intelligence/report/index.mjs` performs deterministic schema-checked, bounded, pure JSON/Markdown/CSV/evidence projection. Its N-way table is pinned-SHA aware, but it is **not** the N-way comparator. It will not read external repos, grant permissions, emit effects, install dependencies, write a graph, publish UI or infer runtime from a source claim. Synthetic 2/3/5-pin inputs and generated report outputs live in `proofs/E/samples/`; these sample SHA values and findings are deliberately invented fixtures and **must not be treated as repository evidence**.

## G1 handoff and constraints

1. Resolve A/C/F exact schema at final worker commits. If their actual `VersionComparison` shape differs, build an explicit, versioned converter under integrator ownership and fail on missing pin/evidence fields; never silently treat a branch label as a commit SHA.
2. Run the saved synthetic suite and benchmark under **Node 24** in integrated ARBOR checkout, record command exit, resource metrics and observed truncation; this lane's isolated Node22 local test is not that gate.
3. Integrate B/D parity/capability and identity fields explicitly (no alias that silently upgrades similarity to confirmed replacement, or partial scan to missing).
4. Independently source-check `sourcePins`, `evidenceRefs`, acceptance receipts and any owner decisions. Report source-digest equality is not runtime proof or cryptographic attestation.
5. Only after G1 `GO_FOR_W2`, H may expose a read port with per-read actor/session/live Host grant checks and U may display inert projection. E does not modify `hq/lib/arbor-service.mjs` or TierFlow, nor can it authorize review/merge.

## Blockers

- A/C/D/F are parallel and are not imported or assumed integrated. Actual C/F envelope compatibility and honest executed-receipt adapter remain **UNVERIFIED** until G1.
- ARBOR original W01-G and beta gates remain **NO_GO / REPAIR_REQUIRED**: root Cargo/Bun locks, real integrated Node24, Windows, physical Android, signed-in Chrome, Host security grant/revoke and no-donor closure are **UNRUN or BLOCKED**. None of this report work resolves those product blockers.
- `hq/lib/capabilities.mjs` measures static package evidence, Labs observatory is seeded source data, PG00 is preparation-only. Neither supplies accepted runtime claims.

**Next owner:** Z1 independent referee, then G1 serial integrator. No merge to `main` is authorized for E.
