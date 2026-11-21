# 02 — ARBOR subsystem matrix and integration architecture

**Source inspected:** `owenservera/ARBOR`, `main`, around commit `2be47cf3410b2e4de110e2331903c459942d8488` on 2026-11-21. This is a source-reading assessment; no runtime/cargo/device tests were executed for the research.

## 1. Direct source observations

| Source file | Observed responsibility | Consequence for Mojo |
|---|---|---|
| [core/programmability/src/product_kernel.rs](https://github.com/owenservera/ARBOR/blob/main/core/programmability/src/product_kernel.rs) | Pure Rust dispatch for commands, Wiki, VEC and personal/control state, with no authority for filesystem/network/device effects | Preserve shared Rust kernel; Mojo must not introduce parallel dispatch/authority |
| [core/programmability/src/vec.rs](https://github.com/owenservera/ARBOR/blob/main/core/programmability/src/vec.rs) | Deterministic source/revision-aware visual IR and bounded render elements | Mojo would not accelerate dominant consistency problem |
| [core/programmability/src/vec_admission.rs](https://github.com/owenservera/ARBOR/blob/main/core/programmability/src/vec_admission.rs) | Bounded/inert semantic admission preserving authority separation | Remain Rust, no GPU dependency |
| [packages/vec-ui-engines/src/rust-wasm.mjs](https://github.com/owenservera/ARBOR/blob/main/packages/vec-ui-engines/src/rust-wasm.mjs) | Browser loader and ABI checking, including bounded WASM inputs/outputs | Rust/WASM is already deployed architecture; Mojo WASM only a future watch |
| [native/hqkg/src/context_retrieval.rs](https://github.com/owenservera/ARBOR/blob/main/native/hqkg/src/context_retrieval.rs) | Bounded graph BFS, ranking, provenance, token budgets, staleness/watermarks | Keep retrieval and graph authority in Rust |
| [native/hqkg/src/algorithm_adapter.rs](https://github.com/owenservera/ARBOR/blob/main/native/hqkg/src/algorithm_adapter.rs) | Existing engine/algorithm interface and *inert* catalog identities; uses established engine lease/registry | Potential non-authoritative compute extension point, not new scheduler |
| [native/hqkg/README.md](https://github.com/owenservera/ARBOR/blob/main/native/hqkg/README.md) | Research graph/kernel with journal and hot-swappable compute engines; README cautions that this is not the live Bun indexer | Avoid assuming HQKG experimental crate is already a production Host service |
| [studio/src/intel/search.mjs](https://github.com/owenservera/ARBOR/blob/main/studio/src/intel/search.mjs) | Incremental BM25 index, explicitly without embeddings | New embedding math could present a measurable opportunity, but does not yet exist here |
| [studio/src/intel/similarity.mjs](https://github.com/owenservera/ARBOR/blob/main/studio/src/intel/similarity.mjs) | Deterministic text/record similarity scoring | Mostly small scalar/text operations; keep current implementation |
| [apps/mobile-relay/ux/voice-capture.mjs](https://github.com/owenservera/ARBOR/blob/main/apps/mobile-relay/ux/voice-capture.mjs) | Local microphone recording and reviewed text handoff | Audio capture is browser-owned; no Mojo path needed |
| [apps/mobile-relay/ux/local-dictation.mjs](https://github.com/owenservera/ARBOR/blob/main/apps/mobile-relay/ux/local-dictation.mjs) | Optional browser-provided local recognition, no remote fallback | Future native ASR compute may differ, but present dictation path is not a Mojo target |

**Evidence limitations:** File reads establish implementation intent and code structure, not deployment/performance correctness. Feature completion and source revisions may change after this snapshot. Every future adoption decision must re-read HEAD, inspect callers, benchmark a real product workload and verify actual device behavior.

## 2. Decision matrix

| Area | Now | Future | Why |
|---|---|---|---|
| Host, receipt, journal, approvals | **Rust only** | Rust only | World effects and authorization correctness |
| Command/NLCL and personal rules | **Rust only** | Rust only | Grammar and state-machine rules |
| VEC/VLANG, Wiki projection and PWA UI | **Rust / JS / WASM** | Revisit only after proven Mojo WASM maturity | Browser portability/source revision invariants |
| HQKG graph/retrieval/ingest | **Rust** | Rust graph + possible optional numerical backend | Provenance/graph ownership must remain canonical |
| Semantic memory dense-vector math | Not yet evidenced as native hot path | **First Mojo evaluation candidate** | Batched dot products, cosine, normalization, quantization |
| Local model inference | No Mojo integration observed | **High strategic potential** | Optimized hardware kernels, model-specific execution |
| Voice/audio feature extraction | Browser-local capture/dictation | **Selective Mojo evaluation** | FFT, mel features, batch/stream signal processing |
| Image/video analysis | No source-grounded Mojo need | Evaluate if numerical work becomes dominant | Pixel/tensor/signal kernels |
| GPUI desktop and Tauri lifecycle | **Rust** | Rust | Windowing/event loop and Windows compatibility |
| Sync, transport, private sharing | Existing owner | Same | I/O/security rather than mathematical throughput |

## 3. Proposed future plug-in boundary (not implemented)

```text
PWA / GPUI clients
       |
       v
Host authority (Rust): approvals, leases, receipts, persistence
       |
       v
existing typed pure-operation / engine interface
       |
       +-- Rust default implementation (always available)
       |
       +-- optional native numerical backend (Mojo via C ABI)
       |       accepted inputs: bounded typed buffers + algorithm/version ID
       |       returns: numerical buffers + status, no effects
       |
       +-- optional existing library / Python worker (benchmark baseline)
       |
       v
Rust postvalidation, provenance binding, state projection
```

### Guardrails

- No second source of truth: ARBOR graph, journal and Host persist/authorize.
- No provider API dependency and no implicit internet access.
- No unsigned/ambient library loading, arbitrary code execution or blanket Python import within the Host.
- Prefer opaque byte arrays, scalar dimensions/strides, numeric status codes and explicit alloc/free rules over shared language-native objects.
- Bound input/output lengths and memory. Reject stale model/algorithm versions, unsupported devices and invalid results.
- Lock algorithm IDs, model artifacts and exact runtime/compiler versions; disable optional backend on mismatch.
- Keep user corpus and model weights private and outside Labs; record only sanitized measurement summaries and hashes.
- Avoid a runtime-required Mojo backend until native Windows packaging or a supported deployment model is established.

## 4. Potential engineering crossover

Mojo's MLIR/LLVM approach may offer *ideas* about specialization and accelerator lowering for ARBOR's future numerical pipelines. It is **not** evidence that VEC semantic compiler itself should move to Mojo: VEC's semantics, source identities and revision handling are not the same thing as GPU kernel compilation.

Likewise, the HQKG catalog of 30 research algorithms contains metadata records; do not call them executable Mojo opportunities unless a real driver/workload and owner have been identified.
