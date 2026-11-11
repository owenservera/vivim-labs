# VIVIM Personal Knowledge Lifecycle — discovery and integration scaffold
Date: 2026-11-11. Status: DESIGN / RESEARCH SCAFFOLD, **not implemented or accepted**.

## Mission
Enable a user to bring their existing digital life into a **local, owner-controlled, living personal knowledge base** during onboarding and continuously thereafter, with verifiable sources, temporal truth, source-scoped consent, revocation, and task-specific Dynamic Context Bundles (DCBs).

The user must get value quickly, without waiting for all historical ingestion. Imported data never becomes executable instructions. Model extraction is candidate evidence only. Never claim a provider adapter works without source-specific proof.

## Architectural invariants
1. **Reuse one Rust graph-node kernel** from `repos-hq/research/rust-knowledge-graph`, preserving Development vs Personal namespaces. Do not create a new graph engine or governance system.
2. **Host owns personal authority**: source identity, owner permissions, browser effects, personal evidence, World state, durable journals, export, revocation. The shared Rust engine computes; it does not independently obtain provider permissions.
3. **HQ owns development intelligence**, not a person's private corpus. Its DCB and index algorithms may be reused via domain-safe interfaces.
4. **TierFlow projects read-only** knowledge, onboarding, reconciliation, correction, and transparent context previews across PC and phone.
5. **Labs owns research and synthetic fixtures**. Do not put real private exports, secrets, tokens, raw user messages or personal graph snapshots in Git.
6. **No provider APIs** for signed-in AI webapp execution in the current VIVIM product. Local export parsing, file access, user-granted OS facilities and signed-in browser workflows are distinct intake methods. The Google Data Portability API is a researched option, not an approved product exception.
7. Source readability is not consent. Ingest consent is not outbound model disclosure. Consent is not a guarantee of provider terms or supported automation. No covert scraping or access-control bypass.
8. Every derived assertion preserves source anchors, effective/observed/recorded time, confidence, trust and owner acceptance. Proposals are not decisions; decisions are not implementations; implementations are not verified outcomes.

## Files
- [LIFECYCLE.md](LIFECYCLE.md): transitions from fresh install to bootstrap, backfill, steady state, repair, account disconnect, export and deletion.
- [SOURCE-ATLAS.md](SOURCE-ATLAS.md): local, browser, AI, cloud, collaboration, communication, social, mobile and privacy-sensitive source families; access modes and verification.
- [PIPELINE-CONTRACTS.md](PIPELINE-CONTRACTS.md): capture -> evidence -> knowledge -> graph -> DCB, idempotency, permissions, receipts, deletion and failure design.
- [IMPLEMENTATION-ROADMAP.md](IMPLEMENTATION-ROADMAP.md): bounded vertical slices, repository ownership, integration truth gates and test cases.

## Recovered foundations and source links
- Shared Rust graph [Lane B integration handoff](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/integration/LANE-B-SHARED-KERNEL-20261110.md): source-implemented / build-UNRUN / product-wiring-BLOCKED at handoff; refresh heads and verify.
- HQ [DCB protocol](https://github.com/owenservera/repos-hq/blob/main/docs/DCB-PROTOCOL.md): development-first deterministic composer.
- Existing [personal DCB recovery](https://github.com/owenservera/vivim-labs/blob/main/documentation-consolidation/vivim-web-harness/docs/architecture/DYNAMIC-CONTEXT-BUNDLES-REVIEW-2026-10-31.md): design and partial legacy current-harness fragments, not proven unified runtime.
- [DATA-007 ingestion permissions](https://github.com/owenservera/repos-hq/blob/main/docs/architecture/DATA-007-ingestion-permission.md): documented specification only.
- [KB consolidation](https://github.com/owenservera/vivim-labs/tree/main/KB-consolidation): discovery scaffold; coordinate rather than fork.
- [TierFlow decisions](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/DECISIONS.md): KB authority elsewhere, TierFlow projection only.

## Labs implementation update — 2026-11-11

The original lifecycle files in this directory remain the design authority, but [runtime/](runtime/) now contains a **real Labs-stage implementation**: T=0 profile candidate intake, safe ZIP32/ZIP64 streaming, JSON normalization, resumable Host-journal batch processing, scoped Rust graph proposals, synthetic tests and a read-only archive inspector. The recent sprint passed **28 synthetic Node.js tests locally**.

This does **not** mean Host/Studio/Rust/TierFlow product wiring has been accepted. The runtime's [ARCHITECTURE.md](runtime/ARCHITECTURE.md) records required adapter contracts, proof boundaries and remaining device/real-account gates.

## Scope boundary and handoff
This folder maps the **personal** knowledge lifecycle; it does not override the broader KB consolidation effort. Before implementation, pin exact main SHAs and identify current Host producer/consumer paths, schema versions, Rust linkage, actual grants and integration receipts. Record outcomes as IMPLEMENTED, WIRED, TESTED, DEVICE-VERIFIED, DESIGN-ONLY, or UNKNOWN separately.

## User acceptance scenario
Import two historical exports containing duplicate messages, a proposed project decision, an explicit acceptance, a later contradictory claim and a verified outcome. Show source-backed chronology. Send a minimal DCB only with destination permission; block unauthorized sensitive contents. Re-import unchanged data without duplication. Revoke a source, cascade invalidation, and prove its content is absent from retrieval and outbound context. Resume after a forced crash.
