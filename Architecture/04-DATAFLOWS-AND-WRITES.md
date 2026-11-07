# Inputs → Transformations → Outputs / Writes

## Data lifecycle
1. Input: repositories, documents, Git, browser observations, sessions, user interactions, models, structured datasets, tests and receipts.
2. Ingestion: discover/capture/receive, source revision, sensitivity and trust.
3. Transformation: parse, extract, normalize, validate, classify, dedupe, resolve IDs, graph-project, enrich, compute, materialize.
4. Knowledge/compute: canonical entity links, graph operations, engine selection, resource pinning.
5. Persistence: transactional SQLite, append-only kernel history, Rust journal, source-native operational ledgers.
6. Outputs: tables, graph query results, CLI output, context bundles, wiki, visual documents, dashboards, events, receipts.
7. Feedback: source changes, reindex, stale detection, reconciliation, error/effect evidence and replay.

## Verified pipeline catalogue
| Pipeline | Input | Transform | Writer / output |
|---|---|---|---|
| HQ indexer | Local files, Git metadata | Hash, extract symbols/imports/annotations, resolve references | SQLite files/symbols/imports/FTS5, per-project transactions |
| HQ KB intake | hq.vivim-kb/1 packages | Digest, secret scan, classify, evidence/proposed decision routing | Transactional HQ records; no authority promotion |
| HQ KB read model | Documents, decisions, terms, sessions, notes, wiki | Normalize, query, score, relate | Read-only query; explicit note APIs only for mutation |
| HQ durable kernel | Versioned knowledge mutation | Provenance/secret/revision validation | Separate kernel/kernel.db append-only revision + current pointers |
| Git event lake | Git log | Parse, dedupe, map changed files | SQLite commit history |
| Rust HQKGX1 importer | JSONL + manifest | Hash, schema, identity, endpoint, conflict validation | Staged graph → journal snapshot |
| Rust browser feed | Sanitized visit events | Validate URL/time/ID, dedupe, project site/visit edges | HQD1 graph delta, ACK after fsync |
| Rust DB adapters | DB nodes, bounded SELECT/WITH | Bounded read-only SQL query | DataTable output; no source DB writes |
| C4 catalogues | Source rows, prior snapshot | Sort, digest, compare, replay, collision checks | Snapshot/delta value; caller decides persistence |
| Wiki | HQ indexed records | Aggregate, format, link, render | Generated Markdown/HTML projections |
| UI/Visual | Canonical document and signals | Layout, render, edit plans | Real writer to be confirmed from TierFlow/Host |
| Host effects | Approved typed actions | Execute/verify/reconcile | Host-native receipts and state, not Rust graph authority |

## Transformation taxonomy
Discovery; extraction; normalization; validation; classification; semantic identity resolution; graph projection; enrichment; computation; materialization; state transitions; reconciliation.

Every transformation needs an owner, implementation pointer, input/output data types, source revisions, deterministic/idempotency expectations, error/refusal behavior, and declared read/write effects. Descriptive graph nodes are not an excuse to create unapproved writers.

## Storage patterns
1. **Ephemeral/rebuildable search**: HQ .hq/ index SQLite and FTS. Safe to rebuild from sources, not sole canonical curated KB store.
2. **Durable semantic knowledge**: HQ kernel/kernel.db, versioned revisions, one writer and current pointers.
3. **Rust graph operational topology**: .hqj journal, mostly full snapshots plus additive web-memory delta frames.
4. **Source-native state**: Host effects and receipts, Studio browser state, TierFlow execution state, UI document revisions; each native owner writes its domain.
5. **Reference data/resources**: original documents, model files, corpora, archived agentic files, optional SQLite/PostgreSQL DB nodes; graph can link without copying full data.

## Write authority
| Data object | Canonical writer | Other graph participants |
|---|---|---|
| Source repository file | Repo owner/editor | Index and reference |
| HQ index tables | HQ indexer | Query and project |
| KB accepted revision | HQ curated kernel writer (target where accepted) | Propose/consume |
| Rust graph journal | Rust journal owner | Validated import/query |
| Engine implementation generation | Rust EngineRegistry in-process | Descriptor/lease requests |
| Host action and receipt | Host/Studio/Android owners | Evidence ingestion |
| TierFlow job state | TierFlow execution owner | Read receipt projections |
| UI canonical document | Existing Visual document owner | Render/query/propose edits |
| Labs trackers | Labs projection owner | Consume receipts; avoid certifying product activation from code |

## Example desired end-to-end flow
TierFlow source changes → HQ detects/reindexes → semantic identity reconciled in durable kernel → versioned graph export → validated Rust import → Rust graph query → source-linked Visual read model / Labs tracker → agent CLI/context refresh. Only first stages and individual adapters have code evidence; the complete continuous chain has not yet been acceptance-tested.

## Production/data-pinning nuance
A pinned engine generation is not a pinned dataset. Resource hash, schema version, source authority and immutable data binding must be separately checked. A mutable SQL connection isn't a content snapshot. Engine install + graph lineage save isn't one atomic transaction.

## Known gaps
Cross-store transaction semantics, source-to-Rust continuous deltas, full UI/Host/TierFlow writer audits, end-to-end replay, user-consent governance for personal feeds, high-volume graph persistence, native acceptance, output freshness/lineage and all-store backup/restore proofs.
