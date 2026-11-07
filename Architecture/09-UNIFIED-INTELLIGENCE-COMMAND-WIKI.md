# HQ as the Unified Intelligence, Command and Programmable Wiki System
Date: 2026-11-07
Status: architecture correction and integration priority
Related: [Integration Philosophy](07-INTEGRATION-PHILOSOPHY.md), [Integration Sequence](08-INTEGRATION-SEQUENCE.md)

## Correction
Do not describe the first VIVIM integration as *deep code indexing*. That omits its primary design purpose.

**Repos HQ is the combined, continuously updated knowledge, semantic-command and generated-wiki system, spanning several user and agent surfaces.** Deep indexing is the evidence acquisition and discovery layer, not the product itself. The integrated feedback loop is:

**Sources and code + KB + operational evidence → extraction/indexing + canonical semantic IDs → knowledge and dependency graph → natural-language/action interpretation and deterministic typed operations → CLI / MCP / programmable wiki / visual controls / agents → approved owner actions or versioned configuration writes → receipts and new evidence → reindex/refresh.**

Every operation should be discoverable and addressable by the same identity in all surfaces. Natural-language text and visual controls are interpreters/projections over canonical typed operations, not independent semantics or write authorities.

## Actual existing building blocks in Repos HQ
- `hq/lib/indexer.mjs`, `walk.mjs`, `extract.mjs`, `schema.mjs`, `graph.mjs`: incremental source discovery, symbols, imports, inline @hq annotations, FTS and dependency links.
- `hq/lib/kb.mjs`, `kb-intake.mjs`, `context.mjs`: document/decision/term/session/note/wiki read model; source package intake; deterministic context bundles.
- `hq/lib/semantic-registry.mjs`: stable versioned IDs for types, capabilities, actions, schemas, effects, events and contracts; sources, aliases, status and explicit owner promotion.
- `hq/lib/semantic-optimizers.mjs`: pure alias disambiguation, bounded evidence ranking and planning helpers.
- `hq/lib/action-registry.mjs`: action expansion with domain/verb/object IDs, wired vs declared distinction, risk/actor/approval metadata.
- `hq/lib/action-router.mjs`: deterministic scoring of registered actions against plain-English input; returns NO_MATCH rather than inventing an action; does not execute.
- `hq/lib/wiki.mjs`: deterministic generated Markdown wiki and standalone HTML browser page from indexed source and knowledge models.
- `hq/lib/wiki-program.mjs`: data-only wiki view program validation, preview, revisioned activation and rollback, explicitly scoped to configuration and not OS/browser effects.
- `hq/lib/comment-wiki-bridge.mjs`: fail-closed annotated/comment-program → semantic action binding → preview/activation of wiki configuration; does not dispatch effects.
- `hq/lib/nomenclature.mjs`: user-language to engineering-language taxonomy bridge.
- `hq/bin/hq.mjs`: CLI surfaces; MCP and other transports are other access surfaces.
- `docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md`: prior intended end-state for deterministic NL interpretation, same typed action from click and phrase, reprogrammable wiki and source-authority separation.
- External pure NLCL reference is identified in older design documents. Do **not** claim the entire natural-language compiler, complete NL CLI, or cross-product unified routing is already wired and production tested; validate exact command coverage in an implementation census.

## Placement inside one Rust graph
| HQ building block | Rust conceptual component | Graph relationships |
|---|---|---|
| Incremental indexing | Transformation engine + data-source nodes | extracts_from, indexed_by, depends_on |
| Semantic registry/aliases | Semantic knowledge + identity | defines, aliases, supersedes, implemented_by |
| Action registry/router | Operation/capability + language/command engine | provides, interpreted_as, requires_authority |
| KB and context engine | Knowledge + computation | derived_from, selected_for, supported_by |
| Generated wiki | Materialization + interface engine | presents, links_to, generated_from |
| Wiki programmable config | Configuration/controls + visualization | configures, previewed_as, activated_revision |
| CLI/MCP/NL/UI | Interface nodes to same operation | invokes, renders, proposes |
| Receipts/evidence | Observation + history | produced_by, verifies, supersedes |

Engine representation should distinguish the stable logical slot from the concrete implementation, generation, and runtime. Native Rust graph computation may execute within Rust; the HQ Bun implementation stays external until an approved adapter or migration exists. The graph can know operations without acquiring privileges to run them.

## First integration MUST preserve the entire loop
### First: unify HQ semantic discovery + commands + wiki as one real subsystem
Inventory actual code-indexed capabilities, semantic IDs, action handlers, CLI/MCP commands, generated wiki pages and programmable wiki operations. Identify consistent IDs, equivalent surfaces, missing links and declaration vs wired/accepted state. Make the combined system observable from one versioned graph projection. Do not construct a separate engine census disconnected from how users issue commands and navigate the wiki.

**Internal DEV variant:** full multi-repo source introspection, unresolved candidates, operator CLI/diagnostics, test intents, action routing traces, generated architecture wiki and visualization, mock action catalogues, no privileged effects.

**End-user PRODUCT variant:** installed/user-scoped knowledge and capability model, constrained NL/CLI/UI access, automatically generated personal/system wiki, canonical safe operations, explicit permissions, receipts, no lab-only code inventory or raw developer controls.

**Acceleration:** immediately gives subsequent integrations an index of actual implementation, a semantic name and command, a generated documentation/wiki page and a common human/agent control surface. The *integration system itself* becomes the workbench for adding every next engine.

### Second: connect that integrated HQ system to Rust HQKG
Map source/semantic/action/wiki identities to Rust nodes and edges: sources, datasets, EngineSlots, actual implementation generations where verified, operations, command/UI/wiki views, transformation provenance and named owner. Use existing HQKGX1 importer where possible. Query the same entity and operation through HQ and Rust without inventing a parallel registry, new writer or premature executable Rust plugin.

**DEV:** comprehensive graph inspector, golden fixtures, command-route traces, generated wiki drift view, validation/replay tests.

**PRODUCT:** bounded graph synchronization and user-relevant capability/wiki projections; no dev-only dependencies.

**Acceleration:** Rust graph becomes an integrated architecture and capability map, while HQ's wiki and commands become the direct human/agent means to discover/test the next Rust/data/UI integration.

### Third: wire the real Visual Engine to that same model
Render the Rust/HQ graph and generated wiki views as read-only visual projections first; use the same canonical operation from menu/click/phrase/CLI, preview configuration changes through the existing owner, and log receipts. Preserve the Visual Runtime's own document writer and Host's privileged effects owner.

**Acceleration:** turns integration visibility into immediately usable UI, exposes missing links, and tests universal programmability on real engine/resource nodes.

## Required proof
- A source change updates index, semantic references, generated wiki and graph projection predictably.
- One user intent or CLI operation resolves to an actual registered action and can be surfaced by the wiki/UI with consistent semantic identity.
- Unknown/ambiguous input refuses or requests explicit selection; no blind execution.
- Declared but unwired actions remain explicitly marked and cannot be falsely shown as executed.
- Configuration write passes preview, source authority and revision checks, with rollback.
- Browser/OS/provider effects remain with Host, Studio and platform brokers; no side-door from wiki, CLI, Rust graph node or NL router.
- Developer setup can exercise the exact operation semantics with mock data/trace, while product setup excludes dev-only permissions/paths and passes release tests.

## Principle
**Do not integrate a crawler and then later bolt on language, commands and wiki. Integrate the self-describing knowledge-command-wiki system as a whole, first as an end-to-end bounded vertical slice, and use it to integrate everything else.**
