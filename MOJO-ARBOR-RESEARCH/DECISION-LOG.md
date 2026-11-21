# Mojo / ARBOR decision log

## 2026-11-21 — R000: Research established

**Decision:** Keep Rust as the product authority and default compute/runtime language. Do not integrate Mojo into ARBOR now. Preserve Mojo as a candidate for optional, bounded numerical/AI accelerator kernels once an actual workload and supported deployment target exist.

**Why:**
- Current ARBOR product code primarily solves provenance, validation, deterministic compilation, state-machine and Host authority problems.
- Rust already runs the shared product kernel and browser WASM bridge.
- Mojo's differentiated value is compute specialization, especially GPU/AI kernels, not Python-library access alone.
- Windows native and WASM targets are current deployment gaps.
- MAX has separately licensed components requiring deliberate review.

**Evidence:** ARBOR `main` source read on 2026-11-21, observed commit `2be47cf3410b2e4de110e2331903c459942d8488`; official Mojo v1.2 docs and release; system requirements; MAX License.

**Confidence:** High for 'do not replace the Rust Host/kernel'; medium for prioritization of future Mojo kernels (not yet benchmarked).

**Implementation:** NONE. **Benchmarks:** UNRUN. **ARBOR writes:** NONE.

### Open questions

- What first real *product* numerical workload makes computation dominate end-to-end latency?
- Will Mojo provide a supported, reasonably small native Windows release/runtime?
- Which existing Rust/native backend is the strongest apples-to-apples competitor?
- Can any optional Mojo module ship without MAX, or does the exact application require MAX components?
- What concrete deployment footprint and license obligations would an optional compute pack create?
- Is Mojo substantially better than simply using existing optimized Rust or an off-the-shelf inference runtime?

### Reopen conditions

- A profiled bottleneck with actual user benefit.
- A compatible deployment target, licensing and bounded pure-ABI contract.
- Benchmark results meeting the [evaluation protocol](03-EVALUATION-AND-ADOPTION.md).
- ARBOR source owner requests a pilot, retaining a Rust fallback.

### Publication/authority guardrails

This Labs research log is non-authoritative. It is not an ARBOR roadmap update, an approved production dependency or a performance claim. Keep tests, exact source SHA, and device acceptance truthful and source-linked.
