# Copy-on-write historical state and tracked page mutation

Base 50d74edcbd5febe62b064923d5924f85baca3cea. E33 source-only.
- Seven large world history tables (blobs, artifacts, evidence, attempt history, run costs, resource receipts, host costs) now use typed copy-on-write maps. Read API and serialized map shape remain unchanged. Clones share immutable map data; writes fork the affected map. Insert/get_mut/remove track exact keys; fallback mutable dereference/clear marks the entire table dirty. Tracking is bounded and not serialized.
- Internal store writer skips serialization of unchanged tracked history, reuses the committed directory, and updates only affected adaptive pages for tracked keys. Fallback full rewrite on untracked mutation or missing legacy directory. Existing public index writer retains complete state semantics. Receipts retain their prior incremental path.
- This is not yet persistent per-key RAM mutation: first modification of a shared map still copies that map; other world tables and reconciliation remain fully materialized. Exact dirty sets are derived from trusted loaded state, never accepted from caller. Future work: grouped page patches, resident cache and fine-grained dependency invalidation.
- Source fixture for serialized shape, COW isolation and legacy writer compatibility authored. No compile/test or empirical speed claim.
