# VIVIM Personal Knowledge — T=0 acquisition runtime (Labs)

Status: **Labs implementation / synthetic tests passing / no production Host or Rust cutover.**

This code is staged inside `vivim-labs/personal-knowledge-lifecycle/runtime`, not installed as a second Host or graph runtime. It is intentionally provider-API-free.

## Working paths

**Fast lane:** user-approved custom instructions, memory summary, project instructions, or verified Studio F02/F01 responses -> `normalizeFastLaneSnapshot` -> Host-injected evidence candidate commit -> Host journal receipt -> T0 coordinator metadata. No auto-acceptance of memory claims.

**History lane:** user-selected official ChatGPT ZIP -> safe ZIP32/ZIP64 reader -> streaming `conversations.json` -> source-qualified normalization -> bounded batches -> existing Host ledger checkpoint -> Host evidence+Rust graph proposal bridge -> verified Host receipt -> T0 coordinator metadata. No browser export scraping.

Both use Host consent checks, explicit account/World scope and exact receipts. Browser execution remains in the existing Studio app.

## Commands

```sh
cd personal-knowledge-lifecycle/runtime
npm test
npm run bench
npm run inspect -- /absolute/path/to/chatgpt-export.zip
```

The inspect command prints **counts only** and never uploads an archive. The benchmark creates its own small synthetic archive. Do not commit any real user exports to Git.

## Core files

- `src/t0-core.mjs`: grants, actions and source receipts.
- `src/fastlane-snapshot.mjs`: T=0 profile candidate ingestion.
- `src/zip-reader.mjs`, `src/json-array-stream.mjs`, `src/chatgpt-normalize.mjs`: loss-aware local archive parsing and normalization.
- `src/chatgpt-import.mjs`: checkpoint/reconcile/recover worker.
- `src/host-ledger-port.mjs`: existing Host SQLite ledger adapter.
- `src/scoped-graph-proposal.mjs`: deterministic Rust-scoped graph proposal.
- `src/host-archive-wiring.mjs`: injectable production integration boundary.

Read [ARCHITECTURE.md](ARCHITECTURE.md) for concrete API contracts, status by file, T=0 lifecycle, failure handling and unverified production gates.

## Important limitations

A fully integrated product requires real Host authority and evidence-writer methods, existing Rust typed bridge, Studio and TierFlow wiring, identity and deletion propagation, real-account ZIP schema testing and device acceptance. The method names documented as required ports are **interface proposals**, not currently verified existing Host APIs.

Do not report Labs synthetic results as real ChatGPT ingestion throughput.
