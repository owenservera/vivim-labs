# Bounded metadata batching and index traversal

Base: 080cdce3a68e2bf2b73be8131234aefbcf64c7f2. E33/R06 source closure.

- Host-only InitializeBatch admits at most 256 static declarations and a bounded serialized payload. It rejects nested batches, worker actions, model send authority, and external effects. All declarations share one durable revision; any invalid declaration aborts the transition. Settlement runs once after the complete batch.
- Native qualification uses batches of 32 declarations in a fresh isolated root. The entire seed is not one transaction; a failed seed leaves incomplete evidence and cannot be mistaken for a successful run.
- Scan supports bounded entry/page/byte budgets, verified pages and immutable root cursors. A stale cursor after a new commit is rejected. Point-in-time scans are hash-bucket ordered, not globally alphabetical. Legacy nested tables without indexes require migration. The scan checks current head and visited objects, not every retained journal or object. Large individual values may require point lookup.
- CLI: kernel <store> scan <table> <entry-limit> <page-limit> <byte-budget> [cursor-json-file]. Pass page.next for continuation.
- Authored negative and replay fixtures for atomic initialization and cursor fencing, plus autonomous demand policy scope and native-first fixture.

Still open: full-world transaction materialization, full legacy migration qualification, bounded long-term receipt retention, real builds/tests, and actual OS authority. No verified score credit.
