# Incremental change and reconciliation

Improve work required to compare authorized knowledge/UI snapshots while preserving existing cursor, CAS, tombstone, revocation and resnapshot semantics.

**Explore:** canonical structural fingerprints, bounded incremental diff, safe invalidation and resnapshot triggers. **Measure:** CPU/memory/latency, missed changes, false updates and stale-state exposure during 1k/10k-event tests.

No alternate revision writer, offline KB or replacement transport.
