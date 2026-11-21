# Open-world data-system discovery taxonomy

Every artifact can belong to multiple classes. These classes are starting points, not an exclusion list.

1. **Schemas/models**: JSON Schema, SQL DDL, Rust structs, TS types, OpenAPI, protocol envelopes, DTO, AST/IR, graph edge types, command language, validation, identifiers, versioning, migrations, UI documents and component models.
2. **Durable stores**: SQLite/WAL, PostgreSQL, JSON/JSONL/YAML/CSV, files, Git objects, journals, audit logs, archives, snapshots, checkpoints, browser profile, IndexedDB, local storage, device stores, backups.
3. **Transient stores**: in-memory state, caches, worker/session state, queues, drafts, offline buffers, token stores, event buses, ephemeral model context, renderer trees, pending approvals.
4. **Transports/protocols**: HTTP, WebSocket, SSE, IPC, CLI/stdin/stdout, MCP, CDP, browser DOM, native bridges, Android Intents, WebView, app relay, polling, Git sync, QR pairing, local sockets, file watchers, plugin adapters.
5. **Transformations**: parsers, extraction, serialization, normalize, validate, encode/decode, schema mapping, projection, redact, enrich, summarize, tokenize, index, FTS, embed, rank, merge, reconcile, deduplicate, graph traversal, version conversion, diff/patch, render, compile, import/export.
6. **Compute engines**: SQL query, graph and Rust algorithm, knowledge retrieval, contextual rerank, policy/rules, workflow, scheduler, migration, UI command execution, publishing, sync, telemetry and testing.
7. **Event/state flows**: source→ingest→transform→write→index→project→transport→consumer→receipt; CAS, retries, consistency, offline replay, idempotence, recovery, tombstones, revocations, retention.
8. **Security/governance**: access scope, consent, authority owner, secrets, encryption, provenance, actor/device identity, policy-bound projections, purge, private data, trust boundaries.
9. **Lifecycle**: research, prototype, implemented, tested, integrated, deployed, live, stale, archived, deprecated, orphaned, unknown.

Record actual writer/reader paths, source revision and evidence, not just a conceptual feature name. Two stores using SQLite may be necessary independent authorities; two writers for the same logical object require investigation.
