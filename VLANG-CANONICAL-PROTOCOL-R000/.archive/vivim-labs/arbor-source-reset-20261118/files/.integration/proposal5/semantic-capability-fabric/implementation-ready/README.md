# SCF implementation-ready engineering kit
**State:** NEAR-CODE BLUEPRINT / NO PRODUCT SOURCE CHANGES / NO RUNTIME TESTS. **Authoring date:** 2026-11-12. **Parent:** Proposal 5 SCF-R011 + SV-R012 draft PR #13.
## Purpose
This kit is the executable decision record that another agent can implement without redesigning the architecture. It contains target file paths, concrete JS module signatures, full data/authority semantics, SQL schema outline, v3 grammar, error handling, dependency graph, test vectors, integration gates and exact worker order. The only code-like content here is reference pseudocode/specification under the proposal tree; no `hq/lib`, Host, Rust or TierFlow implementation was modified.

## Start here — strict order
1. `00-SOURCE-LOCK.md`: exact inspected source, conflicts and gate reconciliation.
2. `01-DECISION-LOCK.md`: 20 non-negotiable decisions (remove interpretation).
3. `02-TARGET-FILE-PLAN.json`: exact target files, owners, creation vs modification.
4. `03-DEPS-AND-INTERFACES.md`: typed seam contracts and serial integration order.
5. `04-ABI-ERROR-SEMANTICS.md` + `contracts/`: machine schemas, deterministic digests and errors.
6. `recipes/`: module-by-module almost-code reference implementation, bounded logic and edge cases.
7. `fixtures/` and `05-TEST-AND-RELEASE-GATES.md`: golden tests and required results.
8. `06-INTEGRATION-ROLLBACK.md` and `prompts/`: launch-safe worker execution order and integration instructions.
9. `implementation-manifest.json`: machine dependency DAG, ready/not-run status, allowed writes.

## Locked external constraint
HVI scope is governed by `.integration/proposal5/hq-version-intelligence/10-NATIVE-TOOLING-ONLY-SCOPE.md` as of 2026-11-12. **Do not run a real donor migration audit, parse actual Labs donor content for comparison, or auto-schedule scans during this implementation wave**. Only synthetic invented Git fixtures for HVI/SCF comparison acceptance; actual source module imports to verify build/compatibility are permitted.

## Existing sources inspected
`hq/lib/protocol.mjs`, `annotation-v2.mjs`, `extract.mjs`, `indexer.mjs`, `semantic-registry.mjs`, `schema.mjs`, `wiki.mjs`, `wiki-program.mjs`, `arbor-service.mjs`, `catalogue-delta.mjs`, `hq/bin/arbor-hq.mjs`, research `compileSemanticWiki()`, SCF/SV proposal contracts and HVI governing scope. The v2 parser **returns records even on unknown refs** plus errors; consumers MUST reject error-bearing source before publication. The legacy `extractAnnotations` indexes only v1 kinds; do not mistake those DB rows for v2/v3 verified facts.

## Non-negotiable outcomes
- Small agent-authored real-comment headers, repeatable `@hq:capability` refs per file.
- Exact file-vs-symbol anchoring with no copied-tag false proof.
- Compiled source, semantic and evidence facts cached by immutable source pin.
- Existing HQ semantic registry/SQLite/wiki/HQKG and accepted HVI reused; no new canonical truth store.
- Independent Semantic Verifier as a separate checker and distinct test oracle.
- Host runtime grants, personal data and effects remain outside comments/wikis.
- All declines/incomplete scans/capability ambiguities survive into report/UI as `UNKNOWN` or `BLOCKED`.
- Source/fixture/CI/runtime acceptance are separate; no beta GO.
