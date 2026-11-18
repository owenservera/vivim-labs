# 09 — How to turn ideas into evidence without fooling ourselves
No benchmarks were executed while scaffolding this folder. **Code inspection is not a performance measurement**.

## Establish a reproducible baseline
1. Pin actual ARBOR SHA, source manifest, OS, CPU, RAM, disk free, Node/Bun/Rust versions, build profile and compiler flags.
2. Select pure synthetic generators covering small (1x), typical (10x), burst (100x) and adversarial upper bounds. No real user corpus or donor repo comparisons.
3. Run baseline and candidate on the same machine, alternating order after warmup and cool-off to reduce thermal/cache bias. Keep exact commands, exits, seed, sample count, tool versions, raw results.
4. Check **correctness before timing**: exact output hashes, same denial/authorization, no silent dropped events, crash/replay consistency, capped resource usage and no unauthorized effects.
5. Record p50, p95, p99, confidence range and outliers alongside CPU, heap/RSS, disk bytes, cache hits, energy (only if measurable), recovery and build/maintenance costs.
6. Evaluate regression budget per platform and mode; accept a Pareto improvement or explicitly articulate a trade-off with owner's approval.
7. Store benchmark *artifacts* and provenance, not just a polished “X% faster” claim. Mark unavailable metrics `NOT_MEASURED`.

## Test suites worth wiring later
- **HQ:** synthetic 1k/10k/100k files; indexing delta 1 file vs 10%, FTS search p95, WAL growth, rebuilding from zero.
- **Rust HQKG:** existing ignored `bench_vertical.rs` and `benchmark_typed_dag.rs` with real Cargo; cold/warm DAG, 1k/10k nodes, recovery replay and cancellation.
- **Studio:** 0.1/1/10GiB generated raw event series where disk permits, bursty gaps, protobuf adversarial shapes, patch/snapshot rates, sparse seek, crash compaction.
- **Host:** synthetic owner grants and command queue; preflight vs effect timing, revoked authorization, uncertain-send no-resend, invalid clock; no real provider send.
- **TierFlow:** synthetic 1/10/100k revisions (bounded scenarios), 2/5/20fps data, UI doc size near limit, invalid nested documents, Bun SQLite transaction p99.
- **Device:** low RAM/low free disk/thermal throttling and offline PWA, if physical runner exists; otherwise mark UNRUN.
- **Build:** cold/warm Rust, Bun, Node and typed tests; separate developer productivity from deployed CPU.

## Suggested output envelope
```json
{
 "schema":"arbor.optimization.measurement/1",
 "candidateId":"OPT-000",
 "gitSha":"<actual 40-hex>",
 "fixture":{"id":"synthetic-v1","seed":42,"scale":"10x"},
 "environment":{"os":"NOT_MEASURED","cpu":"NOT_MEASURED","runtime":"NOT_MEASURED"},
 "baseline":{"latencyP95Ms":null,"rssPeakMiB":null,"bytesWritten":null},
 "variant":{"latencyP95Ms":null,"rssPeakMiB":null,"bytesWritten":null},
 "correctness":{"executed":false,"pass":null},
 "commands":[],
 "unrun":["No tests executed in scaffolding"],
 "outcome":"RESEARCH_ONLY"
}
```
`null` is honest. Do not report guessed speedup as benchmark output.

## Acceptance hierarchy
`SOURCE_GROUNDED → BASELINE_MEASURED → HYPOTHESIS_TESTED → PARITY_VERIFIED → OWNER_APPROVED → INTEGRATED → DEVICE_ACCEPTED`. A score does not skip any stage. Code review cannot promote itself into an executed benchmark.
