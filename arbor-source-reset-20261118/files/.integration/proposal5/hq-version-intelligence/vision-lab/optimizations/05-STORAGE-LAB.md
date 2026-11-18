# 05 — Storage, database and durability strategy lab
## Strategic direction
Not one giant database; **one clear authority per datum, and minimal derived duplication**. HQ index is rebuildable development evidence; TierFlow UI revisions are authoritative; Studio raw originals are protected; Rust HQKG journal is canonical native graph computation history; Host World/effects remain Host-owned.

## Inventory and candidate strategies
| Surface | Source-grounded architecture | Possible experiment (not approved) | Must preserve |
|---|---|---|---|
| HQ SQLite | Node `DatabaseSync`; WAL/NORMAL; FTS5 prefixes | measure total DB and WAL bytes vs source corpus; EXPLAIN index scans, run query-plan regression | source path/source pin truth and rebuildability |
| TierFlow SQLite | Bun SQLite WAL/FULL, per-world CAS and retention | delta+checkpoint revision representation vs full snapshots under synthetic documents | exact per-world revision and power-loss durability |
| Studio raw logs | append-only segmented, bounded queued bytes, sidecar seek index, gap receipts | stride, flush interval, segment size, compression after durable close, archive tiers | never hide dropped events or mutate originals |
| Studio compaction | writes new segments and compaction receipt | model temporary disk multiplier; preflight space; streaming rather than full copy | verifiable compaction and source original protection |
| Rust journal | HQJ1/HQD1 and `compact_verified` | incremental recovery index / split cold history with verified compaction | integrity, fsync before acknowledgment, restart correctness |
| VersionedStore | vNNNN immutable JSON, per-name cross-process locks | archive superseded versions only behind retention and recovery proof | CAS, lock ownership, rollback |
| Source manifest caches | immutable commit/tree/blob pins | content-addressed derived, bounded, owner-scoped cache reused only when authorized | revision identity, consent, invalidation, erasure |
| PWA offline | current source not fully profiled here | quota-aware layers for drafts vs derived previews; cache budgets | offline refusal, revocation, no effects offline |

## Storage hierarchy proposal
**R0 Protected:** personal originals, authoritative World, grants, receipts, committed UI revisions, active journal, mandatory recovery metadata. No probabilistic eviction or unverified compaction.

**R1 Rebuildable but important:** HQ index, FTS, semantic search materialization, verified pure derived graph projections. Reconstructable from protected pins/canonical source after failure, with explicit staleness.

**R2 Reusable derived cache:** hashed source parse trees, comparison report projections, TierFlow geometry, build caches and compiler artifacts. Quota + LRU/TTL may be safe *only* after proper scope/identity and dependency invalidation.

**R3 Ephemeral:** transient decode buffers, per-request sorted tables, speculative IR, previews. Memory-bounded; no durability promises.

## The five numbers every owner should know
1. Disk total and free (including safe low-disk reserve).
2. Size by owner: protected originals, SQLite base/WAL, indexes, journals, derived cache, build targets.
3. Read/write amplification: bytes read/written per useful indexed file/event/revision.
4. Peak extra space during compaction, rebuild and migration, including partial artifacts.
5. Crash recovery and rebuild time p95, including torn writes, ENOSPC and scope revocation.

## Hypotheses worth examining
- FTS prefix indexes may be sizeable; inspect `PRAGMA page_count`, `freelist_count`, `dbstat` availability, FTS segment stats before suggesting a tokenizer or prefix change.
- TierFlow FULL vs HQ NORMAL intentionally differ. **Do not “optimize” by copying the cheaper durability setting to authoritative data.**
- Raw logs have explicit gap semantics; compress or batch only if missing records stay detectable.
- Avoid automatically migrating every JSON/SQLite subsystem into Rust. A correct index and a small lazy cache can outperform expensive replatforming.
- Dedup across private namespaces may leak equality via hashes; scope addressing carefully.

## Non-negotiable experiments
ENOSPC before/during/after fsync; torn frame at replay; pinned-source digest mismatch; journal rollback; SQLite busy retry and concurrency; cold-start rebuild; delete derived cache only; revocation/replay; two-user isolation; power-loss model. No storage savings count unless recovery remains correct.
