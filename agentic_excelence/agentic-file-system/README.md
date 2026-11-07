# Agentic File System — VIVIM Labs migration

**Status:** public-safe, source-pinned research copy; no live runtime modifications.
**Source:** private Repos HQ main at acad98697575c2f3fe2fcbf847e19a8dcd3670db.

## Migrated research

- **205 of 217 original agentic file bodies** copied byte-for-byte from public-origin repositories as inert `.source.txt` snapshots under [recovered/originals/](recovered/originals/); every source Git blob SHA matched.
- **12 private-origin source bodies** remain in private Repos HQ. Their catalog IDs, source paths and hashes are preserved in [MIGRATION-MANIFEST.json](MIGRATION-MANIFEST.json), not the contents or private headings.
- **65 of 75 original research artifacts** copied intact or sanitized into [recovered/](recovered/): ontology, flavor variants, original heading batches and receipt shards, algorithms, heuristic scores, inventory, architecture research and a public-safe graph subset.
- **10 research artifacts** (five implementation scripts, two bulk runtime exports, three cross-repository schema inventories) remain private and are identified in the migration manifest.
- An additional snapshot of the [filetype registry](recovered/registry/agentic-file-registry.v1.json) and inert [AGENTIC-RULEBOOK and DCB-PROTOCOL](recovered/contracts/) is included for source traceability. Canonical authority remains in Repos HQ.

All 217 files and 292 original research-path records are indexed in the migration manifest. This was a **copy**, not a destructive removal from Repos HQ.

## Shared and unique sections

Read [SHARED-UNIQUE-SECTION-ATLAS.md](SHARED-UNIQUE-SECTION-ATLAS.md), then the [217-file machine-readable section atlas](SECTION-ATLAS-217.json).

The observed research contains 13 shared section concepts and **1,010 section variants**: 842 base headings, 168 additions, and 48 source-pinned replacements of existing IDs. The newly added headings remain unclassified until a verifier assigns them a type. Literal headings reused across files are *not* evidence that the instructions are interchangeable.

- **Shared concept** — e.g., Verification exists across multiple file families.
- **Unique variant** — in AGENTS.md it can mean working rules; in SKILL.md, runnable fixture/stop conditions; in SITREP.md, receipts; in HANDOFF.md, exact restart evidence.
- **Literal repeated heading** — identical wording across files can have different owners, lifecycles and effect boundaries.
- **File-specific heading** — observed unique wording may still express a shared semantic concept.

Research sources are preserved at [section types](recovered/section-types.json), [section flavors](recovered/section-flavors.json), [heading variants](recovered/section-variants-observed.jsonl), [public supplement](recovered/graph-supplement/section-headings-161-217.public.jsonl), [section shards](recovered/sections/), [algorithms](recovered/algorithms.json), [scores](recovered/scores-217x10-v0.csv), [public graph nodes](recovered/graph-nodes.public.jsonl), and [public graph edges](recovered/graph-edges.public.jsonl).

## Next experiment

Connect this file/section corpus to the [Wave 1.5 DCB + LRU + virtual-agent experimental plan](../context-corpus-wave-1-5/): source-grounded section classifiers, shared section recipes, file-specific variants, five inert candidate documents, and controlled experiments measuring evidence recall, provider-native validity, stale-context refusal and token efficiency.

No file in this research folder grants authority to execute actions, install providers, use browser sessions, access private memories or modify authoritative agent files. The generated or recovered instructions remain inert data.
