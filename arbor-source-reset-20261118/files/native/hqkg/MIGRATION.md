# Rust HQKG migration into Repos HQ

**Canonical owner:** `owenservera/repos-hq` at `research/rust-knowledge-graph`. This source was migrated from
`owenservera/hq` at commit `388132534fc163f92b1c74fa336a200d0ed56ee9` on 2026-11-03.

The migration preserves the complete Rust crate, including the concurrently
added native `algorithm_adapter.rs` and its tests. It does **not** install the
Rust engine as the active HQ indexer, replace the Bun/SQLite source of truth, or
grant engine execution authority based on graph records.

Graph/compute Rust code, optional DB drivers, future bridges, tests and
conformance fixtures are maintained **here only**. The old HQ repository
should contain only a redirect once cutover is verified.

Rust core is a research artifact until it meets the activation gates in
`docs/RUST-WORKSTREAM.md`; existing HQ Bun algorithms and indexes remain
the default operational path. Repos HQ's canonical source datasets are in
`research/agentic-knowledge-graph` and
`research/knowledge-base-network-graph-signal-intelligence`.

## Original source snapshot

- Upstream repo: https://github.com/owenservera/hq
- Upstream source commit: `388132534fc163f92b1c74fa336a200d0ed56ee9`
- Migrated files: 12 (all files within `research/rust-knowledge-graph/`)
- Each copied file was read from the upstream default branch and inserted
  unchanged under its original relative path into Repos HQ.
- Git blob identities are cross-repository content hashes. Verify exact blob
  SHA parity before deleting the old source.

## Test commands (must actually be run before declaring green)

```sh
cd research/rust-knowledge-graph
cargo test
cargo test --features databases
```

No Rust runtime or cargo tests were available in the remote editing session;
do not mistake committed test sources for executed tests.


**Post-migration importer:** An optional `hqkg-import` adapter now consumes existing JSONL with feature-gated `serde_json`/`sha2`. Default crate builds remain std-only. The tests are committed as source but not yet executed.
