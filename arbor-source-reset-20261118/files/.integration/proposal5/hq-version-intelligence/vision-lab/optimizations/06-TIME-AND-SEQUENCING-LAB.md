# 06 — Sequencing, temporal semantics and workload orchestration
Timing optimization is not just reducing a delay. It is deciding **when to avoid work** without losing event truth.

## Source-grounded machinery
- Host [bootstrap scheduler](https://github.com/owenservera/ARBOR/blob/98f15e327957956ec94189772d5e5813e7b6b07b/apps/vivim-host/bootstrap/scheduler.mjs) dispatches via the existing ledger/Studio and reconciles uncertain results before ever risking a resend. The relevant optimization frontier is *pure preflight and admission*, not concurrent provider effects.
- HQ [watcher](https://github.com/owenservera/ARBOR/blob/98f15e327957956ec94189772d5e5813e7b6b07b/hq/lib/automation.mjs) defaults to 5s polling, 500ms debounce; media scan interval 30 minutes. Measure whether watchers are active at all and how often overlap creates redundant refresh.
- Studio [raw-log](https://github.com/owenservera/ARBOR/blob/98f15e327957956ec94189772d5e5813e7b6b07b/studio/src/store/raw-log.mjs) flushes at 50ms, fsyncs at 1s. Duty-cycle-aware tuning must retain explicit gaps and durable boundary.
- Rust [engine scheduler](https://github.com/owenservera/ARBOR/blob/98f15e327957956ec94189772d5e5813e7b6b07b/native/hqkg/src/engine.rs) is synchronous and checks cancellation between tasks; it cannot interrupt a hung pure plugin mid-call.
- Rust [observability](https://github.com/owenservera/ARBOR/blob/98f15e327957956ec94189772d5e5813e7b6b07b/native/hqkg/src/observability.rs) tracks sequence and epochs; source event time alone is not trustworthy total order.
- [Chronos staged contract](https://github.com/owenservera/ARBOR/blob/98f15e327957956ec94189772d5e5813e7b6b07b/contracts/chronos/staged-contract.v1.json) exists; this source-only pass does **not** establish it as a live canonical clock engine.

## Optimization ideas
1. **Temporal type discipline:** distinguish instant, monotonic duration, deadline, recurrence, wall time, reported source time, logical sequence, epoch, and event-age.
2. **Coalesced wakeups:** align compatible noncritical derived refreshes to fewer idle wakeups, with freshness budgets and safe pause.
3. **Critical-path diagrams:** identify which task groups can run in parallel purely; keep Host ledger/effect ordered.
4. **Adaptive batching:** bound waiting time while reducing fsync and parser overhead; tested with bursts and cancel/restart.
5. **Temporal admission:** refuse tasks if deadline cannot be honored, rather than queue unbounded work.
6. **Sequence-aware replay:** preserve original producer sequence, event ID and watermark even if consumer batches change.
7. **Clock quality receipts:** record time source, offset, uncertainty and timezone context; do not infer cross-device causality from timestamps.
8. **User-defined timing:** high/medium/low triggers should not turn into a hidden always-on daemon.

## Timing-metric contract
For every task: `enqueued_monotonic`, `started_monotonic`, `finished_monotonic`, `queue_ms`, `work_ms`, `wall_observed_at`, `source_epoch`, `producer_seq`, `deadline_budget_ms`, `cancellation_lag_ms`, `clock_uncertainty_ms?`, `run_kind`, `caller_authority`. Monotonic values are local-process only; do not persist their absolute epoch as cross-device truth.

## Boundaries
Do not introduce a second Host scheduler, alter one-writer effect order, silently retry an uncertain send, optimize away a durability receipt, or treat a staged clock contract as installed. Model clocks and concurrency with deterministic injected fixtures first.
