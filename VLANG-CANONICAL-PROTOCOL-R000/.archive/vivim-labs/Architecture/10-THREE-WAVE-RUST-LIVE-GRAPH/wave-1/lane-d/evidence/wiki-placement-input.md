# D2-12 Rust-vs-SQLite/wiki placement input — no benchmark performed

**Status:** INTEGRATION-GATED; **no** p50/p95, memory, cold/warm rebuild, journal rehydration or native Cargo measurements available in Lane D this run. This document is an experimental plan, not an ownership migration recommendation.

- **KEEP_EXTERNAL_OWNER**: HQ's existing `buildWiki()` Markdown/HTML writer and its owner-confirmed `wiki-program.mjs` configuration/revision writer. TierFlow Bun/SQLite UI revision store and Host effects are out of scope.
- **SQLITE_CANONICAL** (until independent owner decision): curated HQ KB/semantic record authority, indexed source/FTS and owner-approved revisions; do not duplicate into wiki truth.
- **RUST_ADAPTER** (read projection only): existing HQKG journal graph lineage, dependency traversal, immutable stable-ID read views.
- **SQLITE_DERIVED**: current index/search caches where existing owner permits; verify no duplicate canonical revisions.
- **DEFER_MEASUREMENT**: whether moving wiki semantic join calculation into Rust beats existing HQ/SQLite read, plus native p50/p95 and failure atomicity.

After the owner-machine run, measure input root count/bytes, graph nodes/edges, real article/page count, change/no-op rebuild times, Rust import/query + restart times, SQLite FTS latency, RAM, disk and journal sizes, with source SHAs and exit codes. Compare against F's independently measured rust-sqlite-placement.json; never replace an owner just to improve a synthetic benchmark.
