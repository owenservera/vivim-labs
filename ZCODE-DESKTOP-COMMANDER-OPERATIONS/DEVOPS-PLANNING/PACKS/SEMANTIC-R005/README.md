# R005 — Semantic truth and human AutoWiki page code preparation

**Status:** Labs source staged / static module check passed / Rust build and execution UNRUN / ARBOR not integrated / Dev acceptance NOT_OBSERVED.

This package extends the [incumbent R003/R004 AutoWiki foundation](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/README.md) rather than introducing a second verifier, graph, wiki engine or journal.

## What is actually coded

| Module | New Rust source | Responsibility |
|---|---|---|
| A — Proof admission | [semantic_evidence.rs](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/src/semantic_evidence.rs) | Map *trusted independently checked* proof outcomes to existing `Verification` and `SentenceClaim`, fail closed without verifier |
| B — Seven-hub page | [page_assembly.rs](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/src/page_assembly.rs) | Assemble permitted committed pages/links and current grounded prose; unreachable hubs stay unavailable |
| C — Read vertical slice | [read_pipeline.rs](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/src/read_pipeline.rs) | Compose existing `readback_guard` -> `semantic_evidence` -> `narrative::realize` -> page assembly, one read-only flow for human and approved agent input |
| Registration | [src/lib.rs](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/src/lib.rs) | Exports the three new modules alongside eight existing R003/R004 modules |

**Source truth:** ARBOR `7ae24ff78a797581b510fe7a78105bbe301c0de4`. The independent Semantic Verifier at `.integration/proposal5/semantic-capability-fabric/verifier/README.md` is **DESIGN_AND_CONTRACT_PREPARED / NOT_IMPLEMENTED / TESTS_UNRUN**. JSON Schema `arbor.sv.proof-receipt/0.1` is a **proposal**, not a working verifier read API. `hq/lib/semantic-capability/verifier-adapter.mjs` was not present. Never treat static semantic occurrences or a copied receipt as independently proven behavior.

## Read order for DevAgent

1. [SOURCE-TRUTH-AND-ABI.md](SOURCE-TRUTH-AND-ABI.md) — precise existing sources, trust/claim/receipt mapping, what remains unproven
2. [DEVAGENT-INTEGRATION.md](DEVAGENT-INTEGRATION.md) — exact paths, function call order, Rust module installation, Host port boundaries and rollback
3. [ACCEPTANCE.md](ACCEPTANCE.md) — real tests, negative gates, receipts, separate GPUI/PWA/phone
4. [R003/R004 baseline pack](../R003/DEVAGENT-APPLY.md) — the prerequisite HQKG scope/identity/read-fence modules

## Non-duplication invariants

- Reuse `core/programmability/src/wiki_read.rs`, the existing R003 narrative + context lens and native HQKG `query_scoped`; no competing graph, compiler, page DB, verifier daemon or work queue.
- Host alone determines current actor/source grants and current Wiki revision/digest. Existing HQKG supplies native graph epoch and semantic source revision — **distinct clocks**.
- The optional `IndependentProofReader` is an injected trusted source-owner boundary; production MUST supply `None` until the real independent verifier adapter exists and is authorized. Synthetic test proofs are not runtime evidence.
- `PROVED_UNDER_MODEL` can support **only** a model-qualified sentence with current pinned source, independent checker authentication, complete scope, matched evidence, checked model limitations and checked explanation wording. `SUPPORTED_BY_TESTS` stays candidate, `REFUTED` stays disputed, and all runtime effect assertions remain unknown until a separate Host-runtime receipt integration exists.
- Human and authorized agent input use one existing read-only projection. No generated partial text triggers a Wiki write, provider send, command execution or task creation.

## Test truth

The existing staged crate now has **81 authored Rust `#[test]` cases**: 37 R003 + 20 R004 + 24 R005. Two HQKG integration tests remain in the R004 Dev pack, outside the standalone crate. The module export/reference audit was source-checked. Cargo/rustc are unavailable in this ChatGPT cloud container, so **cargo test, fmt, clippy, integration, deployment and visual acceptance are UNRUN**.

## Release and ownership

Planning task `AW-SEMANTIC-PRECODE-R005` owns Labs prep only. ChatGPT Dev owns canonical ARBOR source. Do not call this delivered/accepted until Dev explicitly acknowledges the GitHub handoff through `TASK-CLAIMS.md`. GPUI development remains owner-paused; Android is not a prerequisite for Rust/GPUI validation.
