# Mojo × Rust × VIVIM / ARBOR — dedicated research

**Status:** RESEARCH-ONLY / NO PRODUCT ADOPTION AUTHORIZED  
**Research snapshot:** 2026-11-21 (Europe/Madrid)  
**ARBOR source baseline inspected:** `owenservera/ARBOR@2be47cf3410b2e4de110e2331903c459942d8488` (repository HEAD observed during this research). Individual files were read on `main`; no build or device testing was run.  
**Lab ownership:** VIVIM Labs maintains this decision/evidence record; ARBOR owns product code, integration, authorization, persistence, deployment and runtime decisions.

## Executive decision

**Retain Rust as the universal product/kernel/Host/WASM implementation. Consider Mojo only as an *optional native numerical or accelerator kernel provider* when a real workload, supported platform and verified benchmark justify it.**

No existing ARBOR subsystem should be rewritten to Mojo at present. The leading opportunities are future vector/embedding math, local AI inference, audio/speech feature extraction, and image/tensor processing. Python interoperability is useful, but not itself a sufficient reason: Mojo calls existing Python packages through CPython, just as Rust can interoperate via PyO3.

### Read the research

1. [Executive strategy and language comparison](01-EXECUTIVE-STRATEGY.md) — what Mojo is good at, what it is *not*, current platform limits, and strategic verdict.
2. [ARBOR source-grounded subsystem map](02-ARBOR-SUBSYSTEM-MATRIX.md) — concrete keep/evaluate decisions, candidate ordering, integration boundaries and observed sources.
3. [Evaluation and adoption protocol](03-EVALUATION-AND-ADOPTION.md) — work-selection criteria, apples-to-apples benchmark protocol, safety boundary, stop/rollback criteria and handoff.
4. [Upstream evidence and technology radar](04-SOURCES-AND-WATCHLIST.md) — dated upstream URLs, licensing, release and roadmap watch items.
5. [Decision log](DECISION-LOG.md) — explicit decisions, evidence level, unresolved research and next review criteria.

## Three research horizons

| Horizon | Recommendation | Why |
|---|---|---|
| Current product migration | **Rust, no Mojo dependency** | Core workloads are correctness, provenance, deterministic state, Host authority and browser/WASM interoperability |
| First feasible native experiment | **Mojo vs optimized Rust on real batched vector math** | A pure mathematical interface can be benchmarked without moving ownership or persistence |
| Future optional accelerator | **Mojo/MAX *if warranted*** | GPU/AI specialization may be useful for local inference and multimodal workloads, subject to deployment and license review |

## Non-negotiable product boundaries

- Host remains the single authority for approvals, receipts, filesystem/device effects, provider interaction and durable persistence.
- HQKG remains the graph and journal authority. Mojo must **not** introduce a second graph, database, scheduling layer or permission system.
- VEC remains Rust/WASM-native for the PWA; avoid a compulsory native/GPU dependency in the mobile web client.
- Candidate compute functions must be inert (typed input -> bounded deterministic output), isolated, and disposable; there must be no silent Python imports, provider API calls or model downloads.
- Real observed performance and readback evidence are required before adoption. No synthetic product claims, phantom tests or migration-by-enthusiasm.
- This is **not** an execution work order. Product-owner approval and ARBOR source-grounded implementation planning are separate.

## Scope and privacy

Only technical architecture, research sources, safe path references, decision rationale and proposed experiments belong here. Do not add personal memory, transcripts, local machine identifiers, browser sessions, credentials, customer corpora, private logs or copied private repository source.

For the source of truth, refer to [ARBOR](https://github.com/owenservera/ARBOR) and its live code rather than freezing a second product inventory in Labs.
