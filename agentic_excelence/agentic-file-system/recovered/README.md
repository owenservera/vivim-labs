# Agentic network graph library — first scaffold

**Repository:** owenservera/repos-hq (the separate repos-HQ project, not owenservera/hq). **Source census:** October 25, 2026.

## Evidence and identities

The source of record is the existing `hq/registry/agentic-file-registry.v1.json`: **217 files**, **209 source records + 8 donor snapshots**, across seven repositories. The 133 candidate filename patterns are a separate *type catalog*, not 133 recovered files. Every file has a stable file ID, original repository path, Git blob identity, source class and snapshot path. Recovered original bodies are progressively stored as inert `.source.txt` copies to avoid automatic agent instruction loading.

## Distinct graph layers

1. **Type graph**: FileType, SectionType, Algorithm, Schema, Taxonomy, Domain, Capability, Policy.
2. **Instance graph**: File, SectionVariant, SourceSnapshot, Evidence, Test, Decision.
3. **Deployment graph**: Task, Context, Agent, AlgorithmPlacement, ActivationLease, Receipt.

A *Best Practices* section is a shared SectionType; its text in an architecture file vs skill vs handoff are distinct SectionVariants. Similar headings are NOT assumed to have interchangeable semantics.

Algorithms do not literally move or mutate their definitions. Stable Algorithm nodes have lease-bound placement edges pointing at target contexts, tasks and sections. HQ selects/validates; Host or other approved owners execute, subject to permission checks. A document is never execution authority.

## Data

- `file-inventory.json`: full 217 records with stable IDs and SHA/provenance.
- `originals/`: recoverable inert source copies, added in verified batches.
- `sections/part-*.jsonl`: observed headings linked to source spans.
- `section-types.json`: shared section families.
- `algorithms.json`: graph algorithm node registry proposals.
- `schema.sql`: additive SQLite graph projection.
- `prior-schema-sources.json`: pointers to existing VIVIM/HQ database/taxonomy authorities.
- `best-practices.md`: provider-standard vs local scoped variants.

## Non-duplication

Reuse `hq/lib/graph.mjs`, `hq/lib/semantic-optimizers.mjs`, `hq/lib/semantic-registry.mjs`, `hq/registry/canonical-taxonomy.v1.json`, `docs/research/semantic-language-kernel/` and the existing TypeScript/Bun/SQLite HQ toolchain. No new runtime parser, new authority layer, browser automation or overlapping canonical schema.


## Rust HQKG core integration — current direction

The existing std-only Rust prototype lives at [`owenservera/repos-hq/research/rust-knowledge-graph`](https://github.com/owenservera/repos-hq/tree/main/research/rust-knowledge-graph). Repos HQ remains the canonical **knowledge/provenance/section/scoring authority**; Rust HQKG is the **portable compute and traversal engine**, not an alternate taxonomy owner.

- [Rust integration contract](HQKG-INTEGRATION.md)
- [Verified progress, counts, remaining gates](INTEGRATION-STATUS-2026-11-03.md)
- [Deterministic HQKG exchange exporter](scripts/export-hqkg.mjs)
- [Final pinned-source section supplement](graph-supplement/section-headings-161-217.jsonl)

**Source audit:** 217/217 original file snapshots match pinned Git hashes. **Projected graph:** 1,582 nodes / 2,771 edges / 1,010 observed heading section variants, including the final supplement. The Rust importer now exists as optional committed source; end-to-end testing is still pending, and runtime activation is not claimed.

Current bridge workflow: extract and curate in Repos HQ -> build SQLite context/evidence projection -> export HQKGX1 crosswalk -> import once in Rust when the bulk importer is ready -> serve bounded read-only compute queries -> record measured outcomes back into HQ evidence. Keep runtime effects and approvals with their current owners.

## Rust core minimum-delta assessment

See [minimum Rust core changes assessment](RUST-CORE-MINIMUM-DELTA-2026-11-03.md). After inspecting the latest Rust `algorithm_adapter.rs`, V1 needs **no mandatory core library API changes**: add a std-only bulk-import adapter, preserve source ID crosswalks, and keep engine installation/reconciliation separate from knowledge authority. Idempotent upserts and bounded neighborhood helpers are optional optimizations, not implementation prerequisites.


**Owner update:** The Rust core itself now lives in [Repos HQ](../rust-knowledge-graph/); see the [three-stream integration map](../GRAPH-KERNEL-UNIFICATION.md).
