# Proposal 5 — HQ Version Intelligence & Full Migration Parity (HVI)
**Date:** 2026-11-12. **Preparation status:** PLANNED / NOT EXECUTED. **One canonical product branch:** `owenservera/ARBOR:main`. **Scope:** all HQ migration/parity, VIVIM code/Labs source inventory, durable N-way version comparison, read-only migration inspector and Host-approved access. **No beta GO / no device GO / no user-data cutover.**

## Mission
Give ARBOR's internal HQ the ability to pin and compare **2, 3 or N immutable versions of the same repository and/or different repositories**, and answer: what survived, was modified, moved, replaced, omitted, duplicated, intentionally excluded, blocked, research-only or verified at runtime? Each finding needs a typed source, exact revision, evidence and completeness/uncertainty. The first dogfood mission is the actual `repos-hq`, `vivim-web-harness`, `tierflow`, `vivim-labs` → ARBOR migration, plus named research branches.

## Execution
S0 (freeze & census) → [A source pins | B identities | C N-way diff | D parity atlas | E reports | F contracts/tests] IN PARALLEL → Z1 independent referee → G1 serial integration → [H Host | U TierFlow | X real audit] IN PARALLEL → Z2 referee → G2 serial integration → R final self-audit

1. Read [mission and audited baseline](00-MISSION-AND-BASELINE.md), [exact source pins](01-SOURCE-PINS.json), [crosswalk](02-SOURCE-CROSSWALK.md), [normative contract](03-NWAY-CONTRACT.md), [ownership/gates](04-OWNERSHIP-AND-WAVE-GATES.md), [test matrix](05-ACCEPTANCE-TEST-MATRIX.md), [integration/rollback](06-INTEGRATION-AND-ROLLBACK.md).
2. Follow [LAUNCH-INDEX](07-LAUNCH-INDEX.md): one S0 conversation; six separate A-F conversations; one Z1; one G1; three H/U/X parallel; one Z2; one G2; one R.
3. Every worker uses **connected GitHub**, reads entire shared contract and its complete work order, creates its own branch from the **post-preparation main SHA after this documentation PR merges**, writes only its owned paths and commits all evidence; never directly edits main or another worker's branch. Work orders may require a freshly pinned integration SHA at the later phase.
4. S0 may report `BLOCKED_SOURCE_DRIFT` instead of GO. No A–F promotion until reconciled. G1/G2 alone may integrate after independent review. Do not call a source-only import `RUNTIME_VERIFIED`.

## Invariants
Host is the **sole effect and personal World authority**; HQ is development intelligence, source analysis and read-only derived projections; Rust HQKG stays the single native graph compute engine; TierFlow is the one visual publisher; Studio is the only signed-in browser sender. Do not add a second scheduler, graph writer, effect executor, database authority, source identity registry or browser controller. Do not import private profiles/corpora/session tokens. No provider AI APIs, Playwright or Puppeteer. PWA-first, APK deferred. External *authorized developer source inputs* are permitted for comparison; **ARBOR code and runtime dependencies must be self-contained**, with pinned offline manifests reproducible without donor checkouts.

## Status vocabulary
`SOURCE_PRESENT`, `SOURCE_IMPORTED`, `SOURCE_PATCHED`, `PREPARED_NOT_PROMOTED`, `TEST_EXECUTED`, `INTEGRATED_UNVERIFIED`, `RUNTIME_VERIFIED`, `BLOCKED`, `UNRUN`. No implied acceptance. Historical R005–R010/foundation/pre-G branches are candidate sources, not `main` implementations.

**Operator note:** This folder contains launchable instructions, not an already-working HVI product. No lane has executed just because the prompt exists.
