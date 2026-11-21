> **SUPERSEDED AS AN EXECUTION ORDER.** Use [EXECUTE-WAVE-1-FIRST.md](EXECUTE-WAVE-1-FIRST.md) and [WAVE-1-LAUNCH-PROMPTS.md](WAVE-1-LAUNCH-PROMPTS.md). Wave 1 must be run and analyzed before the Wave 2 design. This document is retained as earlier conceptual context.

# Wave 1 — BOOTSTRAP: Make Rust read real HQ data immediately

**Objective:** A running Rust graph containing *live-origin* HQ indexed code plus KB, semantic-command and wiki source records, not only the historical agentic research graph. Close with a real first-query demo and source-linked actual/target baseline.
**Parallel lanes:** A–F. Each must publish own code/tests and HANDOFF before Integration-1. **Wave 1 must not wait until Wave 2 to exercise Rust.**

| Lane | Ownership | Build / change | Immediate acceptance | Unlocks |
|---|---|---|---|---|
| **1A — Source & index acquisition** | Repos HQ HQ index scope, safe read adapters | Confirm all in-scope repo roots, file/symbol/import/annotation extractors, SHA tracking, ignore and secret rules; export bounded observed entities/references from real HQ index | Existing HQ DB reused; deterministic indexed source records; unindexed/ignored coverage explicit | 2A live changefeed; 2B provenance |
| **1B — KB & identity acquisition** | Repos HQ KB/semantic **read** adapter | Source-link decisions, terms, docs, notes, sessions where permitted, durable kernel records, semantic IDs/aliases and revision/authority; no owner auto-promotion | Bounded read, privacy/secret filtering, proposed vs current preserved, missing ledgers visible | 2B graph reconciliation; 2E gap reconciliation |
| **1C — Action/CLI/command census** | Repos HQ action and CLI read adapter | Discover actual `hq` commands, Rust `hqkg*`, declared/wired action registry entries, NL/action routing and corresponding executable/readonly/effect boundaries; inspect TierFlow/Host/SOFTWARE_HQ CLI claims without false confirmation | Every row has command/handler source or unknown; exposed path/action intent and owner | 2C unified command graph |
| **1D — Wiki & programmable controls** | HQ wiki read/export adapter | Inspect generated wiki page identities, page source links, wiki-program config preview/version status and comment-wiki command bindings; add source-linked wiki inventory | No unauthorized program activation; deterministic wiki refs and actual generated pages | 2D graph-backed wiki |
| **1E — Rust HQKG runtime + live importer** | Rust importer/engine/journal + integration-owned projection paths | Validate existing HQKGX1 schema and importer, add bounded live HQ projection for A–D, import to isolated DEV journal via existing `hqkg-import`, query actual nodes and edges | A real `hqkg` query returns at least one code source, semantic ID, command, wiki record, with provenance; unchanged import no-op; no second writer | 2A–F foundation and tests |
| **1F — Actual/target baseline + independent test** | Labs campaign report, read-only reconciliation | Assemble intended system graph from Architecture/ docs and existing catalogues without mistaking goals for reality; produce `actual vs target` classifications with missing/orphan/unknown; independently test 1E | Baseline comparison and runnable reproduction receipt, count by evidence class, known gaps and false claims clearly marked | 2E gap/impact model |

### Integration-1 gate
- Fresh actual Repos HQ `main` plus source repos SHAs recorded. No invented runtime status.
- Real workspace HQ index/KB/semantic/action/wiki records become canonical HQKGX1 payload, safely imported into dedicated Rust DEV journal.
- Exact command to query Rust nodes and edges documented, no fake test/demo data masquerading as integrated capability.
- At least **one representative object from each of code, KB, commands and wiki** visible through Rust; missing categories reported **BLOCKED**, not silently replaced by fixtures.
- HQ source and Rust graph IDs/crosswalk match, no conflict overwrite, importer no-op on unchanged input; corrupted/secret payloads refused.
- `actual-vs-target.md/json` explicitly distinguishes planned/researched/source-found/declared/wired/integrated/tested/device-verified/unknown.
- Six HANDOFF records + Integration-1 evidence with source SHAs and test commands. No live Host actions triggered.

### Suggested first local smoke commands (verify flags and tool availability first)
```sh
# from repos-hq root; do not run reset
node hq/bin/hq.mjs status
node hq/bin/hq.mjs index
node hq/bin/hq.mjs wiki --check
bun research/agentic-knowledge-graph/scripts/export-hqkg.mjs
bun research/graph-kernel-integration/scripts/export-unified-hqkg.mjs
cargo test --manifest-path research/rust-knowledge-graph/Cargo.toml --features importer
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --features importer --bin hqkg-import -- <dev-journal.hqj> <live-hqkgx1.jsonl> --dry-run
```
The live HQKGX1 file is a Wave-1 integration deliverable; do not substitute the static research export as proof of live indexed code/commands/wiki. Switch `--dry-run` off only when content validation and journal ownership are confirmed.

### Parallel ownership cautions
1A–1D use new narrow source adapters and do not simultaneously rewrite `hq/bin/hq.mjs`, `hq/lib/schema.mjs` or wiki core. 1E owns integration-owned Rust import/projection. 1F owns Labs evidence, not operational source code. A cross-lane source edit requires explicit Integration-1 owner.
