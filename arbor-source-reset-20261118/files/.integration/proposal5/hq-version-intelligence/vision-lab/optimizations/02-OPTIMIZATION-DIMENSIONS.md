# 02 — The permanent optimization checklist
For every source-backed candidate, ask these questions **before** proposing a language migration or architecture rewrite.

| Dimension | Questions | Core evidence |
|---|---|---|
| User latency | Is it slow at p50/p95/p99; cold vs warm; interactive vs background? | end-to-end monotonic trace per phase |
| CPU throughput | What fraction of a core? Which hot loops, parse/JSON copies and sync blocking? | CPU samples, iterations/s, utilization |
| Memory/RSS | What is peak RSS, retained heap, allocator churn, GC, fragment, cache size? | peak, alloc bytes/event, resident after idle |
| Database footprint | Main file, WAL, freelist, indexes, fragmentation, growth/1k records? | page_count, page_size, freelist_count, file sizes |
| DB reads | Rows scanned vs emitted, query count, index selectivity, fanout, seek latency? | EXPLAIN QUERY PLAN, logical/physical reads |
| DB writes | Write amplification, bytes/fsync, transactions, WAL checkpoints and lock waits? | writes/txn, p95 commit, WAL growth |
| Storage lifetime | Protected originals vs derived data; hot/warm/cold; content identity and dedup? | reclaimable bytes, eviction/recovery proof |
| Reliability | Crash/torn write, rollback, ENOSPC, cancellation, corruption, repair? | fault injection, digest, recovery time, data loss |
| Time semantics | Wall/monotonic clocks, expiry, drift, ordering, watermarks, TTL and idempotency? | sequence proof, clock source, uncertainty |
| Scheduling | Actual duty cycle, idle wakes, queues, burst handling, fairness, starvation? | wakeups/min, queue dwell, backlog, deadline miss |
| Encoding | JSON/protobuf/NDJSON bytes, zero-copy, compression ratio, parse failures? | bytes, allocations, encode/decode p95 |
| Compiler | IR reuse, grammar ambiguity, repeated parse/validate, optimizations legality? | parse ms, validation ms, semantic equivalence |
| Verifier | Does cheaper verification preserve every refusal and authority boundary? | negative tests, false accept=0, test receipts |
| Cross-surface | PC/PWA/Host/Rust duplication, marshalling and IPC? | overhead per boundary, source-of-truth ownership |
| Developer speed | Incremental build, test critical path, binary size, hot reload, maintenance churn? | cold/warm build, module coupling, change failure |
| Energy | CPU wakeups, battery, thermal throttling, background network/disk? | Wh/session, wakeups and throttling |
| Privacy | No forbidden source reads, test fixtures synthetic, cache isolation and erasure? | denial receipts, access audit |
| Economics | Token costs, disk replacement, developer hours, model request volume? | unit cost per useful verified action |
| Operational simplicity | Does optimization add more daemons, databases, configs or migrations? | owner count, processes, rollback burden |

## Optimization is multi-objective
**Do not minimize p50 at the cost of data loss or p99 latency, or reduce file size by making recovery impossible.** Report a Pareto vector:
`{latency_p95, latency_p99, throughput, peak_rss, db_bytes, wal_bytes, bytes_written, wakeups_per_hour, recovery_ms, correctness, authority, complexity}`.

Correctness and explicit Host authorization are **constraints**, not weighted preferences. The best change can be *doing less work*, *working less often*, *not materializing a duplicate*, *reusing immutable results*, or *simplifying ownership* — not just writing faster code.

## Red flags
- “This is probably slow” with no profile.
- “Move it to Rust” without measuring boundary overhead or parity.
- Reducing `synchronous=FULL` on authoritative TierFlow writes.
- Compaction without peak-temp-space calculation on low disk.
- Clock/timeout changes without replay, DST, skew, cancellation and crash tests.
- Faster compiler with silently weaker refusal or different meaning.
- Cache hit without scoped source revision and invalidation evidence.
- Benchmark fixture not representative of actual cadence.
- Tests present but not executed.
