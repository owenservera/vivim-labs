# 03 — Practical evaluation protocol (research, not execution authorization)

**Goal:** Determine whether Mojo materially improves a *real* ARBOR workload over optimized Rust/native libraries without violating the PWA-first, Host-owned authority architecture.

## Phase A — Find a real bottleneck first

Record a specific workload from the running product:
- caller, authoritative owner, source SHA, input/output contract;
- data shape, batch size, invocation frequency and user-visible latency;
- actual CPU/GPU hardware and deployment platform (no inferred GPU support);
- privacy classification and representative **authorized real** test corpus;
- baseline runtime, compiler flags, warmup policy, memory footprint and real call-chain trace.

**Do not fabricate vector corpora, inference quality or performance data.** No candidate is promoted based on a synthetic demonstration alone. In public-safe Labs, store only *aggregated and sanitized* results. If no authorized corpus is available, record **UNRUN**.

## Phase B — Smallest first evaluation: batched vector normalization / similarity

Choose this only if ARBOR actually adopts dense-vector semantic retrieval and profiling reveals the numerical step is material.

1. Create a pure, bounded mathematical operation: batched dot product/cosine similarity on f32 vectors with explicit shape and stride.
2. Prepare three *functionally equivalent* implementations:
   - optimized stable Rust (default / fallback);
   - existing mature native library if appropriate;
   - Mojo 1.2.0 CPU kernel on a **supported** host.
3. Validate equivalence against a high-precision oracle with predeclared absolute/relative numerical tolerances and reject NaN/Inf and invalid shapes.
4. Benchmark end-to-end and kernel-only times separately; include Rust↔Mojo ABI transfer, initialization and batching cost.
5. Check cold start, steady state, memory peak, dependency size, build time, startup failures, version reproducibility and packaging friction.
6. Compare latency distributions (p50/p95/p99), throughput under product-relevant concurrency and device thermal/power impact when applicable.
7. Do **not** benchmark only against a naive scalar Rust loop or plain interpreted Python.

### Illustrative contract (not checked into product code)

```text
Operation: numerical.cosine_batch.v1
Input: version, batch_a, batch_b, dimension, strides, bounded f32 buffers
Output: version, bounded f32 scores, status/error code
Prohibited: filesystem, network, provider calls, graph writes, Host approvals
Fallback: existing Rust implementation
Ownership: caller owns input/output buffers; no retained pointer or hidden state
```

A productive experiment also needs to show that the relevant *user path* improves, not just a microbenchmark.

## Phase C — Optional local inference evaluation

Only after a supported hardware target and an actual local model workload exist:
- Benchmark an established inference engine first (optimized model + quantization + packaging), then evaluate Mojo/MAX as a challenger.
- Compare model quality at equal evaluation sets, time-to-first-token, sustained tokens/sec, overall latency, RAM/VRAM, energy and deployment bytes.
- Keep model selection, privacy guardrails, approvals, provenance and artifact ownership on Rust/Host.
- Confirm commercial license, redistribution, model-weight rights, driver availability and offline behavior.

Mojo should **not** be the reason for adopting local inference. Product requirements and user benefit come first.

## Phase D — Optional audio/media evaluation

Only if profiling identifies a CPU/GPU bottleneck in an **actual local processing path**. Candidate kernels include spectrogram/mel transforms and image tensor transforms. Maintain the browser/device capture pathway and explicit review/approval flow.

## Evaluation record template

| Field | Required evidence |
|---|---|
| Date / ARBOR full SHA / Mojo version | Concrete versions and source links |
| Product path and owner | Caller, existing integration, authority boundary |
| Platform | OS, CPU/GPU, relevant driver, toolchain and memory |
| Workload | Source provenance; dimensions/batches/frequency (no private content in Labs) |
| Competing implementations | Rust and optimized baseline, Mojo, optional native library |
| Equivalence | Numerical tolerance, deterministic behavior, failures and edge cases |
| Results | Raw command/transcript privately stored; public-safe summary with p50/p95/p99, memory, size |
| Evidence level | SOURCE_ONLY / BUILT / TESTED / HOST_E2E / DEVICE_E2E |
| Decision | REJECT / HOLD / PILOT / ADOPT with owner and rationale |
| Rollback | Flag or backend selection restoring Rust fallback |

## Suggested promotion thresholds (decision policy, *not* observed performance)

Consider piloting only if:
- end-to-end p95 improves by at least **30%**, **or** throughput improves by at least **2×** at equivalent resource and quality budgets;
- correctness, safety and privacy requirements match the incumbent;
- dependency size, deployment support and build maintenance are acceptable;
- same-call source/version provenance survives; no duplicate Host permissions or persistence paths;
- fallback/disablement is tested.

These thresholds are adjustable engineering gates, not facts about Mojo.

## Stop conditions

Stop evaluation if:
- target deployment is unsupported (especially required native Windows/PWA paths);
- there is no measured real product bottleneck;
- Mojo outperforms only a poorly optimized baseline;
- cross-language overhead or runtime size negates compute benefit;
- integration introduces new graph, scheduler, permission or device-effect authority;
- MAX license review does not clearly permit the intended distribution/development mode.

## Ownership and handoff

- **Labs**: research, evidence, alternatives, decisions and source-linked result records.
- **ARBOR native kernel owner**: any eventual typed pure-operation implementation and fallback.
- **ARBOR Host owner**: permissions, persistence, receipts and provider/device effect invariants.
- **HQKG owner**: reuses established graph/algorithm engine, never duplicates it.
- **PWA owner**: no mandatory Mojo artifact; Rust/WASM remains canonical.

**Current execution status:** SOURCE INSPECTED; BENCHMARKS NOT RUN; NO MOJO SOURCE ADDED TO ARBOR; NO APPROVED MIGRATION.
