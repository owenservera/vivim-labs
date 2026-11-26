# P02 R001 — HQKG graph candidate

Preparation only; no SDK activation or Product migration. The exact native donor plus its programmability path dependency and tests have been materialized and blob-verified (169 files including Product reference pins). See closure.json and EVIDENCE.json. The original Cargo attempt failed at the MSVC linker; zero tests claimed.

Working entrypoint after resolving installed Windows developer toolchain: from scratch, overlay the committed p02_pinned_source.rs into native/hqkg/tests/p02_pinned_source.rs (do not alter donor files), then run:

```
cargo test --manifest-path native/hqkg/Cargo.toml --locked --offline --features importer,rust-syntax,native-program --test semantic_source --test import --test reconcile --test graph_indexes --test invalidate-revision --test development_composition --test rust_syntax --test p02_pinned_source
```

The overlay derives digest/line evidence from actual donor source, tests replay, stale anchor refusal, edge reconciliation, tombstones and journal readback. Its exchange is isolated test data, not the live HQ producer or proof of operational acceptance. Preserve native HQKG ownership; reuse Product Identity/ExactRef/Relation and graph-source readback; do not build a second writer. Adaptation obligations are source-bound but not yet compiled into the SDK remediation packet. Resolve licensing, adapter closure, native toolchain and Product/Wiki parity before independent intel-210 review. P03/P06 consume exact source/epoch/namespace readback, not executable authority. No migration without intel-200, intel-210 and owner scope approval.
