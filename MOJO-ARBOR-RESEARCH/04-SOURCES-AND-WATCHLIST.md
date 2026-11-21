# 04 — Sources, provenance and version watchlist

**Research date:** 2026-11-21. URLs below are upstream sources or ARBOR code pointers. Time-sensitive facts require revalidation on the next research round.

## Mojo upstream — confirmed on review date

| Claim / research topic | Upstream reference |
|---|---|
| Stable 1.2.0, 2026-10-22; nightlies separate and moving | [Mojo releases](https://mojolang.org/releases/) |
| v1.2 pattern matching and expanded GPU math functions | [1.2 changelog](https://mojolang.org/releases/v1.2.0/) |
| Open-source Mojo compiler/language licensing | [Mojo homepage](https://mojolang.org/), [modular/modular](https://github.com/modular/modular) |
| Python modules use CPython; no automatic speedup for existing Python code | [Python interoperability](https://mojolang.org/docs/manual/python/), [calling Python from Mojo](https://mojolang.org/docs/manual/python/python-from-mojo/) |
| C-compatible shared-library exports; runtime init caveats | [Compilation targets](https://mojolang.org/docs/tools/compilation/), [initialize_runtime](https://mojolang.org/docs/std/runtime/asyncrt/initialize_runtime/) |
| Native Linux/macOS; Windows via WSL; GPU requirements | [System requirements](https://mojolang.org/docs/requirements/) |
| Async, networking, threading, WASM and broader systems targets are roadmap areas | [2026 LLVM developers meeting notes](https://www.modular.com/blog/modular-llvm-dev-meeting-2026) |
| MAX source-available components and separate licensing | [MAX License](https://www.modular.com/legal/max-license), [FAQ](https://forum.modular.com/t/max-license-faq/3578), [26.7 release](https://www.modular.com/blog/26-7) |

**Interpretation warning:** The language/compiler's Apache license is not a substitute for reviewing the exact license on bundled MAX runtime/graph/accelerator components. Consult counsel before material commercial redistribution decisions. Roadmap items are not features delivered or committed to dates.

## ARBOR source pointers (read-only evidence)

- [Rust product kernel](https://github.com/owenservera/ARBOR/blob/main/core/programmability/src/product_kernel.rs)
- [VEC visual IR](https://github.com/owenservera/ARBOR/blob/main/core/programmability/src/vec.rs)
- [VEC admission](https://github.com/owenservera/ARBOR/blob/main/core/programmability/src/vec_admission.rs)
- [Rust/WASM UI bridge](https://github.com/owenservera/ARBOR/blob/main/packages/vec-ui-engines/src/rust-wasm.mjs)
- [HQKG retrieval](https://github.com/owenservera/ARBOR/blob/main/native/hqkg/src/context_retrieval.rs)
- [HQKG algorithm adapter](https://github.com/owenservera/ARBOR/blob/main/native/hqkg/src/algorithm_adapter.rs)
- [HQKG research README](https://github.com/owenservera/ARBOR/blob/main/native/hqkg/README.md)
- [Studio BM25](https://github.com/owenservera/ARBOR/blob/main/studio/src/intel/search.mjs)
- [Local voice capture](https://github.com/owenservera/ARBOR/blob/main/apps/mobile-relay/ux/voice-capture.mjs)
- [Local dictation](https://github.com/owenservera/ARBOR/blob/main/apps/mobile-relay/ux/local-dictation.mjs)

The links above resolve current `main` and can move. Snapshot inspected while `main` reported `2be47cf3410b2e4de110e2331903c459942d8488`; for deterministic review, replace `blob/main` with a full commit SHA.

## Radar: reevaluate on meaningful change, not on a fixed hype cycle

1. **Native Windows binary support** — actual compiler/runtime/toolchain working outside WSL.
2. **Mojo WASM target** — complete browser ABI/toolchain/packaging support relevant to PWA, not just prototype codegen.
3. **Rust/Mojo C ABI stability** — shared library, memory ownership, runtime initialization and packaging under supported platforms.
4. **Native CPU and GPU perf** — independent, reproducible workloads vs optimized Rust/C/C++/vendor libraries with end-to-end costs.
5. **Async, networking and packaging** — mature systems usage if future ARBOR requirements demand it.
6. **MAX licensing and GPU compatibility** — monitor exact components, licenses, device targets and redistribution requirements.
7. **Python bridge** — compare actual call overhead to PyO3; do not assume speedup.

## Evidence labels

- **UPSTREAM_DOC**: upstream documentation/release/license claim, not an ARBOR test.
- **SOURCE_INSPECTED**: code read, not built or exercised.
- **BENCHMARK_UNRUN**: no performance result.
- **PROPOSED_ARCHITECTURE**: design option, not implementation.
- **ADOPTION_UNAPPROVED**: no authority to change ARBOR production code.

Last upstream review 2026-11-21; next review should be triggered by supported-target news or a real numerical bottleneck in ARBOR.
