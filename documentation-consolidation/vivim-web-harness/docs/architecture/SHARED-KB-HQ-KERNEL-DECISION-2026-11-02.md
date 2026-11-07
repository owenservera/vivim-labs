# Architecture decision — KB + Repos HQ kernel for VIVIM and TierFlow

**Date:** 2026-11-02 · **Status:** owner-directed target architecture, not an implemented Host migration or acceptance receipt.

## Decision

**The Knowledge Base (KB) and Repos HQ together are the shared knowledge and semantic kernel for both VIVIM Web Harness and TierFlow.** Repos HQ is the canonical implementation home of the kernel, not a passive reader of packages produced by VIVIM and not a VIVIM-only submodule. Its schema, knowledge, memory/context selection, capability registry, versioned rules, semantic vocabulary, provenance and wiki projections are common infrastructure for both products.

The current `docs/architecture/CORE-KNOWLEDGEBASE-V1.md`, `CORE-KNOWLEDGEBASE-TECHNICAL-SPEC-V1.md`, and `CORE-KNOWLEDGEBASE-REPO-IMPACT-V1.md` remain *historical design evidence*. Their proposed Host-owned KB tables and `apps/vivim-host/bootstrap/kb/` placement **are superseded as the target**. They were explicitly design-only, so this decision does not presume a deployed Host KB to remove. Preserve their useful type, sensitivity, freshness, source, job, curation and retrieval contracts as migration inputs; do not implement two competing KB kernels.

**Canonical design:** `owenservera/repos-hq/docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md`.

## What stays authoritative in VIVIM

- The **Host** remains authority for its device/personal-data operations, grants, ledger facts and receipts; source-native records stay in the owning source.
- **Studio/CDP** remains the exclusive browser-effect owner, preserving send-once and session identity fences.
- **Android** native platform effects remain with its permissioned broker, including offline boundaries.
- VIVIM's `docs/knowledge/` registries (44 types, 30 sources, 33 jobs, existing schemas and prompt evals) are the initial canonical **input specification** to import or version-publish into the shared kernel, not a second independent runtime implementation.
- `docs/roadmap.json` and existing source files remain source-native status/implementation authorities until an explicitly tested ownership migration. A KB catalog assertion does not change execution authority.

## Consumers and mapping

- **VIVIM Host and desktop/mobile UI:** read capabilities, KB contexts, user-scoped projections, semantic schemas and safe settings from the kernel, through versioned typed interfaces. Existing Host action contracts remain the only authorized effect path.
- **TierFlow:** query the same kernel for knowledge, context, action contracts and policy data, while its 3-tier runtime remains reusable across arbitrary projects. A project-specific Repos HQ adapter translates implementation details only; it does not own project execution semantics.
- **Repos HQ wiki:** a kernel-backed projection with programmable views and schema-defined menus/selectors in V1. Any configuration mutation requires a dedicated authenticated scoped writer, reversible versions and a receipt; legacy read-only wiki queries remain read-only.
- **Natural language (future wiki V2):** reuse the existing `vivim-omega/plugins/vivim-nlcl-pure` deterministic interpretation pipeline. Both clicks and parsed intent yield the same validated typed action and pass through the same authorizer; LLM suggestions never grant effects.
- **Coding-agent semantic annotations:** extend existing `@hq:` comment protocol via schema-checked typed declarations referring to kernel type/action IDs. These annotations are source evidence, not program instructions or authorization.

## Migration acceptance

1. Reconcile existing KB owners (Host bootstrap, Studio, harness, Context Lake, HQ, TierFlow) with a clear field-level authority table.
2. Freeze stable type IDs, schema versions, provenance, freshness, sensitivity, canonical keys, revision CAS and tombstone policy against the existing 44/30/33 registries.
3. Implement durable, backup-capable kernel storage and its single writer under Repos HQ, separated from rebuildable indexes.
4. Publish typed, scoped kernel read APIs and prove TierFlow and VIVIM obtain matching KB revisions for the same inputs and permissions.
5. Migrate presentation-only consumers and wiki configuration actions first; do not reroute existing Host/Studio effects without explicit authorization parity.
6. Prove offline/degraded behavior, cross-platform compatibility, privacy projection, malicious-data isolation, rollback and no loss of existing Host records.

**Not done:** no shared kernel binary/runtime, data ownership migration, device acceptance or production registry publication is claimed by this architecture decision. It is a source-of-truth correction for future implementation work.
