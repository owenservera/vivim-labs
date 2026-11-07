# Documentation and index reconciliation — 2026-10-31

**Source:** GitHub `main` at `3a0b37fe3dc2ee449bde17b708b77b927a06e9c0` before this documentation update. **Scope:** read-only tree enumeration, canonical documentation/source metadata crosswalk and targeted source/document inspection; not live VIVIM acceptance.

## Inventory evidence

- Repository tree: **348** eligible documentation/index files; catalog previously held **340** (8 newly added implementation handoffs missing).
- Source tree: **371** eligible non-test code files; source-file catalog previously held **350** (21 new code files missing).
- Earlier source-symbol index covered only **48** files; the first metadata refresh showed 45 unchanged blob-verified entries and 326 pending lexical scans. **The final full scan now covers all 371 of 371 eligible files**, yielding **18,977 lexical declaration records**. These include local bindings; they are not unique exported APIs, behavior proofs or a call graph.
- **10** preexisting non-generated document blob hashes were stale; **4** existing source-file catalog blob hashes stale.
- Atomic registry held **336** requirements and candidates; the authoritative roadmap retains its own task status. Do not upgrade `GUI01` / `GUI02` or engine tasks to accepted based on source and unit tests alone.

## Corrected discovery contracts

- `tools/docs/query.mjs --route generative_ui --json` now reads `context-routes.json.routes` (previously accidentally looked at top level).
- `tools/docs/check.mjs` recognizes the explicitly narrow Linux Grokbot Direct authorization from the roadmap rather than warning that every non-frozen phase is a mismatch.
- Automatic task context routes now include the ten core engine draft locations, six browser-safe Generative UI modules and current integration handoffs.
- Root entrypoints distinguish the full documentation refresh from the narrower runtime permission; earlier `250-entry` / `ten PRDs` references corrected to **336** / **eleven**.

## Actual data versus remaining proof

The full text of **349/349 cataloged Markdown/JSON documents** was fetched through the GitHub connector, including active specs and historical Studio receipts, without retrieval failures. Every eligible source file's Git blob was also parsed using the existing `reindex.mjs` lexical rules; **371/371** have source symbol declarations in the refreshed index. The 11 PRD tables reconcile to **336** registered entries (232 atomic PRD requirements and 104 cross-project candidates). **169** capabilities have at least one lexical candidate declaration; no symbol match is itself a verified feature or runtime invocation.

A sampled internal-link sweep found historical test-design documents under `catalog/test-designs/` linking to `sources/` files that are not vendored in this repository. Preserve those research references as historical provenance, not active checkout paths; do not silently fabricate the missing source content.

This was a GitHub source audit, not a local `npm test` or local-file reindex execution. Local reindex parity and comprehensive Markdown link validation must still be run against a complete working checkout. Never label a declaration index as an execution call graph. The current command `node tools/docs/reindex.mjs --write` regenerates all five derived indexes, and `node tools/docs/reindex.mjs --check` verifies byte-for-byte drift on that same checkout. `node tools/docs/check.mjs --json` and `node tools/docs/query.mjs --route generative_ui --json` inspect structure and retrieval.

## Functional integration remains separate

The Linux Direct/Grokbot runner is still intentionally blocked without a verified native adapter; GUI modules and core engines are standalone drafts rather than real Bootstrap/Build execution; provider sends, browser ownership, Android repaint and Cloud deployment were not exercised. See [integration checkpoint](REPO-WIDE-INTEGRATION-AUDIT-2026-10-31.md).
