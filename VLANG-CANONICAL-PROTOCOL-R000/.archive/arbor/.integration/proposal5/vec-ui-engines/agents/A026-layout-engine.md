# A026 — Layout Engine

Wave: 1 | Type: engine | Status: PREPARED_UNRUN.
Branch: work/p5-vec-eng-a026-layout.
Base: work/p5-vec-ui-engine-foundation-r001; re-pin the latest SHA before branching. Contract SHA: b9932fac199b59a97be228f9f584de4b1dd287b8.
Exclusive write path: packages/vec-ui-engines/implementations/layout/.

## Objective
Build the canonical Layout composition implementation covering 14 declared capabilities. Reuse shared value/token validators; declare unsupported ones and proof for supported ones.

## Implementation contract
The engine has exclusive ownership of its domain and must delegate reusable primitive parsing to shared validators.
Create within the exclusive directory: implementation.mjs, implementation.test.mjs and proof.json.
Expose versioned input/output schemas, typed data-only effects:none, capability IDs, dependencies, explicit unsupported cases and immutable results.
No source comment, UI document, token pack or semantic declaration grants authority.
Add one valid, one invalid, one boundary, one missing dependency, one stale revision, and one repeatability synthetic test (document inapplicable cases).

## Dependencies
A001, A003, A012. Missing prerequisite can be stubbed for isolated development but NOT counted accepted.

## Shared instructions
- Foundation source: packages/vec-ui-engines/src/{catalog,validators,registry,compiler,tokens,enum-choices}.mjs.
- Shared specifications: .integration/proposal5/vec-ui-engines/{00-MASTER-ARCHITECTURE,01-CONTRACTS-AND-OWNERSHIP,02-WAVE-AND-MERGE-GATES}.md.
- Consumer taxonomy: apps/mobile-relay/vec/ui-taxonomy/registry.mjs; existing PWA and TierFlow schemas remain closed.
- Never edit shared source/contracts/root manifests or another agent folder; commit only exclusive scope.
- No arbitrary CSS/JavaScript/HTML, providers, secrets, permissions, Host actions, graph writers, or new UI publisher.
- Use invented synthetic fixtures; report actual test commands and exit codes, list UNRUN separately.
- No merge/main write or runtime/device GO; integration lead independently accepts verified contributions.

## Required result
Commit only assigned paths on the named branch. Include source SHA, commands/exits, BLOCKED/UNRUN, independent verifier needs and next gate.
Status SOURCE_READY only if source exists; tests PASS only with genuine exits. Never merge autonomously.
