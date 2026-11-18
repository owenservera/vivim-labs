# The Optimization Observatory
## A research garden for faster, smaller, safer and easier-to-maintain VIVIM

> **Optimize the lifetime cost of a capability, not merely the speed of a function.**

This is the optimization sub-laboratory of [The Living Software Universe](../README.md), under Proposal 5. Its first pass is **grounded in ARBOR source**, not a runtime profile. It is deliberately broad: databases, storage, indexes, computation, timing, coding architecture, encoders, compilers, verifiers, platform performance, recovery, privacy, power and developer ergonomics.

**Immutable source pin:** ARBOR `main@98f15e327957956ec94189772d5e5813e7b6b07b` observed 2026-11-12. All 59 candidate source paths have been checked against its non-truncated Git tree; their Git blob IDs are in [opportunities.json](opportunities.json). **No benchmark commands were run, no source behavior was changed and no real repository comparisons were performed.**

### Follow the atlas
| File | Purpose |
|---|---|
| [01-SOURCE-GROUNDED-BASELINE.md](01-SOURCE-GROUNDED-BASELINE.md) | What is actually implemented and what remains unmeasured |
| [02-OPTIMIZATION-DIMENSIONS.md](02-OPTIMIZATION-DIMENSIONS.md) | The permanent questions to ask about any system |
| [03-PRIORITY-ATLAS.md](03-PRIORITY-ATLAS.md) | First-pass ranked opportunities and cross-discipline triage |
| [04-CADENCE-AND-WORKLOADS.md](04-CADENCE-AND-WORKLOADS.md) | Always-on, high-frequency, user-defined H/M/L, dev/admin H/M/L |
| [05-STORAGE-LAB.md](05-STORAGE-LAB.md) | DB size, read/write speeds, WAL, logs, indexes, retention and recovery |
| [06-TIME-AND-SEQUENCING-LAB.md](06-TIME-AND-SEQUENCING-LAB.md) | Scheduling, clock semantics, deadlines, event order and batching |
| [07-COMPILERS-CODECS-VERIFIERS.md](07-COMPILERS-CODECS-VERIFIERS.md) | IR, codecs, parsing, proofs, validation and optimization legality |
| [08-RUST-BOUNDARIES-AND-MAINTAINABILITY.md](08-RUST-BOUNDARIES-AND-MAINTAINABILITY.md) | Rust/native vs JS, build footprint, cohesion and test costs |
| [09-MEASUREMENT-PROTOCOL.md](09-MEASUREMENT-PROTOCOL.md) | Reproducible synthetic probes and regression-proof acceptance |
| [10-FIRST-DRAFT-RESEARCH-PRIORITIES.md](10-FIRST-DRAFT-RESEARCH-PRIORITIES.md) | What to investigate first, in order, without authorizing rewrites |
| [11-EVIDENCE-AND-DECISION-LEDGER.md](11-EVIDENCE-AND-DECISION-LEDGER.md) | Explicit unknowns, trade-offs and decisions before any optimization |
| [opportunities.json](opportunities.json) | Full machine-readable ranked 59-candidate atlas |
| [opportunities.csv](opportunities.csv) | Flat machine-readable rank/score/owner-agnostic view |
| [metrics.json](metrics.json) | Metric and gate taxonomy |
| [source-inventory.json](source-inventory.json) | Fixed path/blob evidence and observation trust |

### Seven lenses
**Storage · Computation · Time · Codecs · Compiler · Verifier · Maintainability.** Cross-cut every lens with privacy/safety, reliability, correctness, battery, disk, ergonomics, economics and portability.

### Principles
- **Measure before porting to Rust.** Different language ≠ better performance.
- **Peak and lifetime cost are different.** Five milliseconds on every frame can matter more than one slow nightly job; a rare 8 GiB spike still matters.
- **Keep authoritative originals safe.** Never reduce durability, consent checks, receipts, rollback or recovery to win a benchmark.
- **Budget idle overhead first on constrained machines.** Always-on work consumes battery, IO and memory even when nobody requests an answer.
- **Benchmarks require executable receipts.** An estimate or source comment isn't a measured p95.
- **This garden creates no execution order.** The standing HVI policy still forbids new real-repository comparison runs; only separately authorized future work may implement an optimization.

### How to grow it
Attach one `OPT-###` record to every proposal, cite exact source pin, capture measured before/after and p95/p99 worst cases, label trade-offs and owner boundaries, then promote through the normal owner-approved work-order process. `P0_MEASURE_FIRST` means measure, **not** implement.
