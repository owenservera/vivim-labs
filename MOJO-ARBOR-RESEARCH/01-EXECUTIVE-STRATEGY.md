# 01 — Executive strategy: Should ARBOR use Mojo?

**Assessment:** 2026-11-21. **Disposition:** RETAIN_RUST; RESEARCH_MOJO_COMPUTE_ONLY.

## 1. Executive answer

Mojo is a compiled, Python-inspired systems language designed for CPU, GPU and AI-accelerator kernels. It combines low-level memory/layout and vector programming with a high-level programming style, Python interoperability and compiler specialization. Its defining promise is *less friction between numerical/AI research and efficient hardware-native implementations* — not automatic acceleration of arbitrary Python imports.

Rust is already well suited to ARBOR's Host-authoritative application kernel: memory-safe systems work, deterministic state machines, provenance, journal/replay, typed interfaces, security-relevant failure behavior, portability and Rust-to-WASM.

**Recommendation:** keep the product's authoritative Rust paths; do not plan any broad Rust -> Mojo migration. If product evolution introduces large, frequently repeated numerical/AI calculations, investigate Mojo as one optional specialized backend behind existing interfaces. Compare against *optimized Rust and existing native libraries*, not only pure Python.

## 2. Mojo's three principal differentiated benefits

1. **Unified CPU/GPU programming:** hardware-aware kernels and specialization within one language and toolchain for supported devices.
2. **Native performance control with productive syntax:** SIMD, memory/layout control and compile-time specialization; useful for kernels whose actual profile is dominated by computation.
3. **Python interoperability and iterative optimization:** call real installed Python packages through CPython and selectively rewrite actual hot paths in native Mojo, while preserving Python-side research workflows.

### Important correction: Python interoperability ≠ automatically faster Python

Mojo imports and calls existing Python modules via CPython; the imported code remains Python/native extension code, with interpreter and call-boundary overhead where applicable. Rust can do the same via PyO3 or an isolated Python process. Only measured native Mojo code replacing a bottleneck, or a genuinely better kernel, can establish a speed advantage.

See: [Mojo Python interoperability](https://mojolang.org/docs/manual/python/) and [calling Python from Mojo](https://mojolang.org/docs/manual/python/python-from-mojo/).

## 3. Concrete language comparison for VIVIM

| Requirement | Rust | Mojo | Implication |
|---|---|---|---|
| Device/Host authority and durable state | Strong mature ecosystem | General systems features still evolving | **Rust** |
| Native Windows release | Supported | Windows supported through WSL; not native as of research date | **Rust** |
| Browser/WASM | Mature toolchain and existing ARBOR ABI | WASM backend on roadmap, not available for required PWA role | **Rust** |
| Async, network, file/process integration | Mature | Broader async/network capabilities are 2027 roadmap work | **Rust** |
| Graph traversal/provenance/replay | Strong fit | No clear Mojo advantage | **Rust** |
| Native numerics/SIMD | Strong; native alternatives exist | Strong language focus | **Benchmark both** |
| GPU kernel development | Possible, typically fragmented across crates/toolchains | Major language/platform focus | **Mojo evaluation candidate** |
| Calling Python packages | PyO3/subprocess | CPython interoperability built into workflow | **Neither automatically faster** |
| Operational complexity for 0-cash PWA-first product | Existing toolchain | Adds compiler/runtime, versioning and licensing review | **Keep optional** |

## 4. Freshness, maturity and ecosystem constraints

- Latest stable Mojo version checked: **1.2.0, released 2026-10-22**; nightly is a moving development channel, not a deployment promise. [Official releases](https://mojolang.org/releases/).
- Mojo compiler/language is open source under **Apache 2.0 with LLVM exception**, see [language site](https://mojolang.org/) and repository license.
- MAX components have differing licenses, including a **MAX License** distinct from Apache. Do not equate open source Mojo compiler with unrestricted MAX graph/runtime redistribution; review the exact modules and their terms before using them: [MAX License](https://www.modular.com/legal/max-license), [official FAQ](https://forum.modular.com/t/max-license-faq/3578).
- Mojo currently runs natively on Linux/macOS and via WSL on Windows. This is a material limit for a native Windows Host and the PWA. [Requirements](https://mojolang.org/docs/requirements/).
- Compiler documentation supports C-compatible exported functions/shared libraries, which suggests a Rust FFI route; it does not establish that ARBOR has such an implementation or that all target platforms are supported. [Compilation](https://mojolang.org/docs/tools/compilation/).
- The 2026-11 roadmap emphasizes GPU/AI kernels, with broader systems-programming and WASM support future work. Dates/targets are aspirations, not delivered capabilities. [Modular LLVM Dev Meeting report](https://www.modular.com/blog/modular-llvm-dev-meeting-2026).

## 5. Fit with VIVIM's economic and architectural thesis

VIVIM aims to stop paying repeated model reasoning costs for behavior already learned and validated. There are **two different economic levers**:

- **Avoid work:** compile/record validated behavior into deterministic Rust contracts; use Host-controlled receipts and replay. Mojo doesn't solve this.
- **Make necessary numerical work cheaper:** batch/accelerate math used in semantic memory, local model inference, audio/media or numerical optimization. This is Mojo's potential role.

Therefore Mojo can complement the second lever but is irrelevant to replacing the first. An alternative off-the-shelf local inference engine may outperform a bespoke Mojo module in *time-to-useful-product* even if Mojo is technically promising.

## 6. What would change this verdict?

A measured product bottleneck plus:
- supported deployment target (including native packaging and reproducibility);
- correct, documented Rust↔Mojo C ABI/memory/exception ownership;
- material end-to-end gains vs optimized Rust and native-library alternatives;
- no degradation of Host/world authority, provenance, security or browser behavior;
- acceptable build size, update friction, license scope and rollback capability.

Absent that evidence, choose **zero Mojo code in production**. This document records a research candidate, not an approved roadmap change.
