# Wave 1 Preflight — Authorized Scoped Index Reset + Single Logical Project

Date: 2026-11-07
Status: **owner-authorized direction**; local reset NOT executed in this documentation change.

## Clarified owner decision
**YES: reset** the previous indexing *run* and its previous run-owned, rebuildable indexing outputs. **NO: do not remove or redesign** the multi-repository indexing concept/capability. Focus this wave on **one logical VIVIM project** whose source scope includes multiple named folders/repositories. Do not mass-rebuild unrelated projects. The reset clears obsolete run state; then a new initial index run for the single intended logical project is necessary to populate a live Rust graph. Subsequent refresh is incremental.

### Terms to keep separate
1. **Engine home**: Repos HQ source implementing the multi-repo indexer/KB/commands/wiki.
2. **Project identity**: one logical VIVIM architecture/integration project; persists independently of folder count.
3. **Scope roots**: multiple repo/folder paths inside that project, each with source identity, head SHA and exclusions.
4. **Indexing intent**: versioned statement of why a scope root is indexed and which evidence/questions/relationships to extract, discover and test.
5. **Run identity**: unique dev-run/source-revision marker, distinct from project identity.
6. **Storage ownership**: disposable source index/wikis vs durable HQ kernel records vs Rust .hqj vs source-native Host/Visual/TierFlow state.

The architecture may internally reuse the existing HQ `projects.json` multiple entries as **collection sources** while applying a parent logical project ID in a thin adapter. Do not rewrite HQ's registration system or invent a new canonical writer just to express the parent project.

## Indexing intent is first-class
Intent must be attached to the **logical project** and optionally refined per scope root. It is **purpose and selection guidance**, not a trusted instruction that can execute code.

Example draft data:
```json
{
  "schema": "vivim.index-intent/0.1",
  "projectId": "vivim-unified-architecture",
  "purpose": "Discover actual code, semantic knowledge and command surfaces; generate the Rust graph and measure observed-vs-intended wiring.",
  "questions": [
    "Where is a feature implemented?",
    "Which engine and schema own it?",
    "What files, commands and data stores communicate?",
    "What input, transformation and write paths exist?",
    "Which links are documented, statically derived or runtime verified?",
    "What required capability or dependency is missing?"
  ],
  "discover": ["source", "symbol", "import", "annotation", "kb", "semantic-id", "command", "cli", "engine", "dataset", "dependency", "writer", "evidence"],
  "scopes": [
    {"root": "repos-hq", "role": "intelligence-and-rust-core"},
    {"root": "vivim-web-harness", "role": "host-product-runtime"},
    {"root": "tierflow", "role": "workflow-and-visual-runtime"},
    {"root": "vivim-labs", "role": "observatory-and-architecture"}
  ],
  "mode": "dev-observation-only",
  "effectsAllowed": false
}
```
This is an illustrative proposed shape, **not** a new production schema or assertion that all roots are registered locally. Verify physical folders and source access before indexing. Existing HQ project registration may need an adapter to represent one parent project.

## Reset protocol — precise and guarded
1. **Inventory** original workspace root, `projects.json`, `.hq/config.json`, resolved `.hq/index.db`, other files under `.hq/`, wiki output, running index/watch jobs, and `kernel/kernel.db`; record owner/source SHAs, counts and provenance. The past run happened in another repo/workspace; do not assume current CWD points to it.
2. **Stop** only old-run index/watch processes after confirming ownership and avoiding open SQLite writes.
3. **Archive** old run index/output manifests and relevant reference metadata to a local, private, separately named backup before reset. Do not upload sensitive records into Git or Labs.
4. **Preserve** durable KB/semantic/decision/note/session/receipt data and `kernel/kernel.db`. HQ's `.hq/index.db` includes such tables: it is **NOT** safe to drop wholesale merely because the file is documented as a rebuildable index.
5. **Scope reset**: remove/reinitialize *only proven disposable, previous-run-owned source-index projections* or create a new isolated dev index destination and retire the old one from active use. Do not run `hq reset --yes` automatically: it deletes all of `.hq/` and defaults to deleting wiki/home. Any targeted SQL-table purge requires inspection of data dependencies/foreign keys and a validated backup; prefer new dev DB profile if safe selective purge cannot be proven.
6. **Re-scope**: declare one logical VIVIM project with explicit multiple source roots, roles, ignored folders, source revisions and a versioned indexing intent. No unrelated old project in active scope.
7. **Run** one initial index under the new scoped profile (not a mass reindex of old projects), connect KB/semantic IDs and commands; rebuild the **derived wiki output** only after A+B+C are wired to Rust.
8. **Verify**: no old-run roots in new graph; no unintended deletion; immutable source IDs; no secrets; repeated index/import no-op; Rust journal and wiki show one logical project with N validated roots.

## Ownership and success criteria
- **Lane A** owns reset *inspection and reset execution on the real local environment*, project/root/intent scope and safe index adapter. It must request manual intervention if local data cannot be safely classified; no destructive guesswork.
- **Lane B** proves protected curated KB/kernel/semantic history still exists after reset.
- **Lane C** proves command/CLI source records remain discoverable within the new scope.
- **Lane D** judges the regenerated wiki after integration; never treats old wiki pages as new source truth.
- **Lane E** projects one logical project / N source roots / indexing intent into the Rust graph, while preserving source provenance and read/write boundaries.
- **Lane F** verifies before/after reset receipt, excluded/retained scope, no data loss, current indexed and orphan/unknown coverage. No PASS without that evidence.

**PASS:** owner-approved scoped reset actually completed locally, old run no longer contaminates current results, multi-repo collection works as one logical VIVIM project, indexing intent is addressable/queryable, durable knowledge survives, Rust graph and generated wiki reflect current scope. **BLOCKED:** cannot safely classify old DB tables, old writer still running, root scope ambiguous, or durable records might be destroyed.

## Source observations
Repos HQ `hq/lib/registry.mjs` supports registered workspace projects with relative or absolute external paths. `hq/lib/schema.mjs` embeds both file/symbol/import indexes and potentially valuable semantic/notes/decisions/session rows in `.hq/index.db`. `hq/lib/kernel/store.mjs` stores durable records outside `.hq/`. `hq/bin/hq.mjs` `cmdReset` removes `.hq/` recursively; **do not use it as an unexamined scoped reset**.

No reset is performed by saving this document. It must be run where the previous indexing run actually lives, with local filesystem access and a preserved backup.
