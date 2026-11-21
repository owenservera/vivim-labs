# Shared KB + Repos HQ intelligence kernel — architecture decision

**Decision:** 2026-11-02 · **Owner direction:** KB and Repos HQ together become the shared kernel for VIVIM Web Harness **and** TierFlow. **Status:** target architecture; does not claim a completed data migration, running kernel service, or accepted execution integration.

## 1. What "kernel" means

KB+HQ is the common, authoritative *knowledge and semantic control-plane kernel*, not a new browser controller, a second OS, or an execution-privilege source.

Its responsibilities:
- **Typed knowledge:** facts, claims, hypotheses, decisions, terms, entities, relationships, provenance, freshness, sensitivity, contradiction, supersession and deletion.
- **System model:** registered data types, JSON Schemas, flows, action/capability descriptors, program versions, source IDs, environment facts, feature and use-case contracts.
- **Context and memory:** deterministic evidence selection, FTS5/symbol/graph retrieval, bounded context bundles, omission accounting, cross-project source tracing.
- **Semantic meaning:** a vocabulary and symbol/intent registry with pure interpreter adapters. NL interpretation must resolve to the same typed actions used by graphical controls.
- **Reprogrammable policy data:** declarative rules, views, workflow recipes, feature settings, and evaluation criteria, each versioned and reversible.
- **Evidence and evolution:** change journal, source-of-truth pointers, content hashes, receipts, stale detection, impact analysis, wiki projections, conformance tests.

Both TierFlow and VIVIM consume the same versioned semantic objects; neither invents a second domain model. Repos HQ must also work as an independently installable product.

The kernel is **not the owner of all source content or effects**. VIVIM Host remains owner of device/user operations and its authoritative effect receipts; Studio retains the single browser owner/send-once journal; Android's native broker retains platform effects; TierFlow's third tier retains execution lifecycle and its scoped authorizer. Source authorities remain authoritative for their native observations; HQ owns the curated, canonical KB representation and its revision history. No KB assertion confers approval.

## 2. Canonical source and deployment

**Source-of-truth implementation home:** `owenservera/repos-hq`. KB types and schemas in `owenservera/vivim-web-harness/docs/knowledge/` are the existing design inputs to migrate or publish here **by verified versioned migration, never by silently duplicating or overriding them**. The 44 type, 30 source, 33 job registers and current schema-registry files are the initial reference corpus, not proof the runtime exists.

Distinguish:
1. **Kernel data** — durable, revisioned, independently backed up; never delete when rebuilding an index.
2. **Source connectors** — observe VIVIM Host, TierFlow, local repositories, Studio and other systems; source-native action permissions remain external.
3. **Materialized projections** — FTS, symbol graph, generated wiki, UI caches, device read models; rebuildable with revision/cursor evidence.

Do **not** mix durable knowledge with disposable `.hq/index.db` state until migrations explicitly distinguish them. Candidate layout: `hq/kernel/` (schema/authority/storage/curation), `hq/language/`, `hq/programs/`, `hq/surfaces/`, `hq/adapters/`, `hq/lib/` for existing code, `hq/test/`, and a dedicated state directory for durable kernel data. File placement is a proposal; choose it after a source/DB inventory and migration parity tests.

One versioned kernel interface supports in-process use, local RPC, CLI/MCP and disconnected snapshot export. No cloud dependency, no assumed localhost access from an offline phone. A device never acquires permissions because its snapshot advertises a capability.

## 3. Ownership matrix

| Concern | Canonical owner | Notes |
| --- | --- | --- |
| KB entries, ontology, revisions and curator decisions | **KB+HQ** | Typed, provenance-first, sensitivity-scoped, single logical writer per KB domain |
| Source facts and raw session/browser/device records | **Original source subsystem** | KB references and verifies, does not shadow-write native ledgers |
| Capability catalog, schemas, workflow templates, semantic aliases | **KB+HQ** | Descriptions and rules are data; action providers implement effects |
| Human intent and task orchestration | **TierFlow Tier 1/2** or **VIVIM client/Host** | Resolve through the shared kernel; no independent schema drift |
| Execution of TierFlow jobs | **TierFlow Tier 3** | Verified run status and receipts flow back to KB |
| User/device/provider effects | **Existing VIVIM owners** | Host/Studio/Android broker enforce grants and journal fences |
| Wiki projection and reprogramming controls | **KB+HQ** | Generated from canonical objects, not freehand prose |
| Natural-language interpreter | **Pure, versioned language component** | Reference: `vivim-omega/plugins/vivim-nlcl-pure`; no competing parser |

## 4. Deterministic reprogrammable wiki

The wiki becomes a **typed kernel control surface**, not an independently authoritative frontend. V1 starts with clicks, menus, selectors and parameter forms; natural language is a subsequent input modality.

V1 must expose:
- Select project, type, entity, time window, confidence/authority class and source.
- Select view layout, panels, filters, graph depth, grouping and metrics.
- Configure context budgets, retrieval strategy, approved taxonomy aliases, freshness review and display rules.
- Preview before/after semantic diffs, reference links, affected projections, and required grants.
- Save a named version of a valid configuration, activate it via the registered action, deactivate/rollback it, and inspect the receipt.
- Display syntax-correct canonical command/IR for each click action.

The old `docs/WIKI-CONSOLE.md` is currently read-only by design. **Preserve that legacy read-only query lane.** Add a separate authenticated, explicit **configuration-write lane** into the kernel writer. It may update **only its declared kernel configuration and reviewed metadata**, never shell/OS/provider effects or raw source documents. All mutations use typed validation, expected revision/CAS, scope grants, journal-before-effect and an undo/revert path.

V2 adds natural-language intent via the existing **pure NLCL** design: `interpret(text, world)` to candidate typed IR and an effect preview. The same action contract, approval path and receipt is used by both a selector and a phrase. AI can suggest interpretation, not bypass deterministic validation or authorize itself.

## 5. Inline semantic programming contract for coding agents

Keep existing `@hq:context`, `@hq:why`, `@hq:invariant`, `@hq:see`, `@hq:owner`, `@hq:task`, `@hq:risk`, `@hq:deprecated`. Extend through **versioned typed annotations**; do not overwrite or discard existing comments.

Proposed example (comments explain, never grant authority):

    // @hq:context TierFlow job cancellation
    // @hq:type runtime.job@1
    // @hq:capability runtime.job.cancel@1
    // @hq:effect process:terminate
    // @hq:invariant Repeated cancellation cannot execute a second effect.
    // @hq:evidence test:tests/executor.test.ts
    // @hq:see decision:DEC-CANCEL-001

New annotations (`@hq:type`, `@hq:capability`, `@hq:effect`, `@hq:evidence`) are *proposals*, not yet registered parser/linter kinds. Define an additive version-2 grammar with explicit comment syntax by programming language and strict schema validation. Type IDs come from the shared KB registry; unknown IDs produce typed lint findings, not invented meanings. Every annotation must bind to file hash, symbol/line and a revision. Docs/wiki pages are generated from indexed evidence and registered definitions; prose is explanatory, **not the sole machine truth**.

Use the existing VIVIM Ω deterministic Natural Command Language Layer (NLCL) as a **semantic grammar reference**, not a replacement for the source-code comment syntax. Its 17 symbol families, lexicon, WorldModel and typed IR can power live reprogramming feedback. Source annotations are a separate, stable machine-readable layer that links into the same type/capability IDs.

## 6. TierFlow universal engine + shared kernel

TierFlow's three tiers remain project-agnostic in *execution semantics*. It consumes KB+HQ as the **default shared intelligence kernel**, not as an HQ-only test fixture. HQ-specific SQL/CLI/file conventions live behind a thin transport/provider adapter; they never enter the execution tier. Publish a generic kernel client contract so alternate implementations or embedded distributions can satisfy it. A project without HQ-specific indexing still runs through core project operations; "universal" means arbitrary projects can participate in a shared kernel, not that TierFlow must hardcode a particular repository.

Test on (a) TierFlow itself, (b) Repos HQ itself, and (c) an unrelated fixture repo. Every improvement gets classified as universal TierFlow runtime, reusable KB+HQ knowledge, adapter-only conversion, or cross-product contract. Cross-product deduplication and trace correlation use stable IDs and revision receipts.

## 7. Migration and non-negotiable acceptance gates

1. Inventory overlapping data ownership in `repos-hq`, `vivim-web-harness`, and TierFlow; publish a **contradiction map** against the pre-decision VIVIM Host-owned KB documents.
2. Freeze common identity, schema version, authority, sensitivity, provenance, field ownership, revision, deletion, and export contracts. Preserve VIVIM's 44/30/33 design registries during migration.
3. Build dedicated durable KB storage, append-only revisions, import validation, conflict detection and deterministic replay. Keep current HQ index intact.
4. Implement versioned, read-only kernel discovery and context API. Prove VIVIM and TierFlow receive identical semantic objects for identical scope.
5. Build the selector-first wiki reprogramming UI on a separate, scoped configuration-write channel. Validate rollback and guard against shell/OS/provider escalation.
6. Extend the HQ agent-comment linter with typed V2 annotation grammar, schema links, drift detection and fixed source anchors; run cross-language fixtures.
7. Integrate TierFlow with the kernel via a portable contract; preserve every Tier 1/2/3 boundary and native execution/approval owner.
8. Add VIVIM Host and phone **read-only** kernel projections, then separately authorize and test optional configuration changes through existing Host action contracts.
9. Exercise crashes, conflicting revisions, stale/poisoned sources, malicious comments, private-data exfiltration attempts, offline operation, and migrations/rollback.
10. Switch design-of-record references only after behavior, tests, migration and receipts are verified.

**Current state:** HQ already has a KB *unified read model* (documents, decisions, CANON, sessions, notes and wiki), a VIVIM KB package intake and a deterministic wiki. Those do **not** amount to the shared durable kernel yet. VIVIM's recent KB design documents still propose the Host as KB writer; they are historical implementation options after this owner decision. This decision is the new target, not a retroactive claim of completion.
