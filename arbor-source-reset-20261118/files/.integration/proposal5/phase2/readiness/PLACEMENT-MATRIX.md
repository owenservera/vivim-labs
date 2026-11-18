# ARBOR P5 — Labs source placement & first-pass readiness

**Completed:** pinned GitHub tracked-tree scan, NOT compilation or real device acceptance.
**Coverage:** 44 reviewed components. Machine-readable audit: [SOURCE-TO-ARBOR.json](SOURCE-TO-ARBOR.json).

## Truth rules

- SOURCE_TREE_PRESENT means actual tracked files at exact source commit, not an installed capability.
- Earlier test counts are SOURCE-REPORTED. Fresh CI, Cargo, Windows, Host, signed-in browser and phone acceptance remain separately UNRUN.
- Some references are design files, not implementations. Sensitive recovered corpora must not be imported.
- No external donor runtime wiring; Host alone writes World/effects, Rust HQKG is one graph engine, TierFlow only renders.

## Easy migration (8)

| ID | Source & evidence | ARBOR destination | Owner | Decision |
| --- | --- | --- | --- | --- |
| E01 | [Capability Observatory seed and schema](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/capability-observatory/data/) — 4 files, REPORTED_SYNTHETIC_TESTS; SOURCE_TREE_PRESENT | integration/capability-evidence/ | F | PROMOTE_TESTS |
| E02 | [Capability Observatory lane D observer](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/capability-observatory/lane-d/) — 11 files, REPORTED_SYNTHETIC_TESTS; SOURCE_TREE_PRESENT | tools/integration/observatory/ | F | PROMOTE_TOOL |
| E03 | [Verified extension hook research](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/vivim/extension-points/) — 3 files, SOURCE_RESEARCH; SOURCE_TREE_PRESENT | capability-spaces/extensions/ | F | PROMOTE_CONTRACT |
| E04 | [Voice-to-tasks pure workbench](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/vivim/voice-to-tasks/workbench/) — 5 files, SOURCE_TESTS_RECORDED; SOURCE_TREE_PRESENT | tests/voice-to-tasks/ | F | PROMOTE_TESTS |
| E05 | [Chronos staged contract](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/chronos-toolkit/wave2/repos-hq/contracts/chronos/) — 1 files, STAGED_PARTIAL_TESTS; SOURCE_TREE_PRESENT | contracts/chronos/ | F | PROMOTE_CONTRACT |
| E06 | [UI R005 pure semantic fixtures](https://github.com/owenservera/vivim-labs/tree/56bb145ea1eb479e4d8ec629163a84322797367a/nlp-command-ui-lab/tests/) — 6 files, REPORTED_SYNTHETIC_TESTS; SOURCE_TREE_PRESENT | tests/interaction/ | E+F | PROMOTE_TESTS |
| E07 | [HQ wiki/provenance research fixtures](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/Architecture/10-THREE-WAVE-RUST-LIVE-GRAPH/wave-1/lane-d/wiki-oracle) — 2 files, SOURCE_RESEARCH; SOURCE_TREE_PRESENT | tests/hq-provenance/ | D+F | REVIEW_FIXTURES |
| E08 | [Development source and capability taxonomy](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/Architecture/) — 106 files, RESEARCH_MIXED; SOURCE_TREE_PRESENT | integration/source-evidence/ | F | REFERENCE_ONLY |

## Core dev/migration (10)

| ID | Source & evidence | ARBOR destination | Owner | Decision |
| --- | --- | --- | --- | --- |
| C01 | [T=0 personal knowledge ZIP/runtime](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/personal-knowledge-lifecycle/runtime/) — 28 files, REPORTED_SYNTHETIC_TESTS; SOURCE_TREE_PRESENT | apps/vivim-host/onboarding/personal-knowledge/ | B+A | INTEGRATE_AFTER_W2 |
| C02 | [Personal knowledge lifecycle contracts](https://github.com/owenservera/vivim-labs/blob/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/personal-knowledge-lifecycle/PIPELINE-CONTRACTS.md) — 1 files, DESIGN_CONTRACT; SOURCE_TREE_PRESENT | contracts/personal-knowledge/ | B+F | PREPARE_ADAPTER |
| C03 | [Node 0 Prime native I/O architecture](https://github.com/owenservera/vivim-labs/tree/61d9fc15c55144c4f0b79afa248981c2b0ca9bd6/user-io-rust-core/) — 17 files, DESIGN_AND_DRAFT; SOURCE_TREE_PRESENT | native/node0/ | A+F | PREPARE_NATIVE |
| C04 | [Chronos HQKG Rust staging](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/chronos-toolkit/wave2/repos-hq/research/rust-knowledge-graph/) — 3 files, STAGED_PARTIAL_TESTS; SOURCE_TREE_PRESENT | native/hqkg/ | A | INTEGRATE_AFTER_HQKG |
| C05 | [Chronos internal HQ JavaScript lens](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/chronos-toolkit/wave2/repos-hq/hq/) — 3 files, STAGED_PARTIAL_TESTS; SOURCE_TREE_PRESENT | hq/ | D | PREPARE_ADAPTER |
| C06 | [Chronos Studio read projection](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/chronos-toolkit/wave2/vivim-web-harness/) — 3 files, REPORTED_SYNTHETIC_TESTS; SOURCE_TREE_PRESENT | studio/ | C | PREPARE_ADAPTER |
| C07 | [Chronos TierFlow read projection](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/chronos-toolkit/wave2/tierflow/) — 2 files, REPORTED_SYNTHETIC_TESTS; SOURCE_TREE_PRESENT | packages/tierflow/ | E | PROMOTE_AFTER_W2 |
| C08 | [Voice-to-tasks product path](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/vivim/voice-to-tasks/) — 9 files, DESIGN_PLUS_PURE_WORKBENCH; SOURCE_TREE_PRESENT | capability-spaces/voice-to-tasks/ | B+E | BLOCKED_LIVE_PIPELINE |
| C09 | [Native graph/convergence architecture](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/Architecture/10-THREE-WAVE-RUST-LIVE-GRAPH/wave-1/lane-d/) — 40 files, RESEARCH_MIXED; SOURCE_TREE_PRESENT | native/hqkg/ | A+D | SELECTIVE_REUSE |
| C10 | [Configuration and personal graph research](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/personalization/) — 3 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | capability-spaces/personalization/ | B+F | DEFER_ADAPTER |

## High-value pending (9)

| ID | Source & evidence | ARBOR destination | Owner | Decision |
| --- | --- | --- | --- | --- |
| H01 | [R006 predictive composer 100 ideas](https://github.com/owenservera/vivim-labs/tree/56bb145ea1eb479e4d8ec629163a84322797367a/nlp-command-ui-lab/docs/R006-) — 1 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | capability-spaces/interaction/ | A+E | POST_BETA |
| H02 | [R007 VIEW ALL full interpretation](https://github.com/owenservera/vivim-labs/tree/56bb145ea1eb479e4d8ec629163a84322797367a/nlp-command-ui-lab/docs/R007-) — 1 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | packages/tierflow/ | E | POST_BETA |
| H03 | [R008 per-screen AutoWiki](https://github.com/owenservera/vivim-labs/tree/56bb145ea1eb479e4d8ec629163a84322797367a/nlp-command-ui-lab/docs/R008-) — 1 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | capability-spaces/knowledge/ | D+E | POST_BETA |
| H04 | [R009 Rock Core storage budget](https://github.com/owenservera/vivim-labs/tree/56bb145ea1eb479e4d8ec629163a84322797367a/nlp-command-ui-lab/docs/R009-) — 1 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | config/storage/ | B+D | POST_BETA |
| H05 | [General app builder](https://github.com/owenservera/vivim-labs/blob/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/KB-consolidation/docs/UI-PROGRAMMABILITY.md) — 1 files, RELATED_DESIGN_REFERENCE_ONLY; SOURCE_TREE_PRESENT | capability-spaces/app-builder/ | B+E | POST_BETA |
| H06 | [Automation Queen](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/automation-queen/) — 14 files, DISCOVERY_ONLY; SOURCE_TREE_PRESENT | capability-spaces/automation/ | B+F | POST_BETA |
| H07 | [Intelligence algorithm families](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/intelligence/) — 8 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | native/hqkg/ or hq/ | A+D | POST_BETA |
| H08 | [Context/learning/memory research](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/context/) — 2 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | capability-spaces/context-dcb/ | B+D | POST_BETA |
| H09 | [Experimental voice ASR and media improvements](https://github.com/owenservera/vivim-labs/blob/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/vivim/voice-to-tasks/ARCHITECTURE-RESEARCH.md) — 1 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | capability-spaces/voice-to-tasks/ | B+E | POST_BETA |

## Retain/review (17)

| ID | Source & evidence | ARBOR destination | Owner | Decision |
| --- | --- | --- | --- | --- |
| X01 | [Labs developer orchestration scaffold](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/labs/) — 29 files, NON_PRODUCT_TOOL; SOURCE_TREE_PRESENT | none (Labs internal) | LABS | RETAIN_LABS |
| X02 | [Agentic excellence archaeology](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/agentic_excelence/) — 305 files, ARCHIVE; SOURCE_TREE_PRESENT | none (research/archive) | LABS | RETAIN_LABS |
| X03 | [Documentation consolidation archive](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/documentation-consolidation/) — 140 files, ARCHIVE; SOURCE_TREE_PRESENT | none (archive) | LABS | RETAIN_LABS |
| X04 | [Legacy agents/automation files](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/agents/) — 62 files, RESEARCH_MIXED; SOURCE_TREE_PRESENT | none (archive or review) | LABS | REVIEW_PRIVACY |
| X05 | [Corpora and research history](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/corpora/) — 15 files, RESEARCH_MIXED; SOURCE_TREE_PRESENT | none (restricted source review) | LABS | REVIEW_PRIVACY |
| X06 | [Z.ai round work products](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/zai/) — 16 files, RESEARCH_MIXED; SOURCE_TREE_PRESENT | integration/source-provenance/ | G | RECONCILE_SHA |
| X07 | [Local coding model/router toolkit](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/research-intake/) — 17 files, NON_PRODUCT_TOOL; SOURCE_TREE_PRESENT | none (PC tooling) | LABS | RETAIN_LABS |
| X08 | [Past orphaned work](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/past-orphaned-work/) — 23 files, ARCHIVE; SOURCE_TREE_PRESENT | none until source audit | LABS | RETAIN_LABS |
| X09 | [Performance research](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/performance/) — 4 files, RESEARCH_MIXED; SOURCE_TREE_PRESENT | tests/performance/ | F | PROMOTE_BENCHMARK_IDEAS |
| X10 | [KB and docs consolidation notes](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/KB-consolidation/) — 6 files, ARCHIVE; SOURCE_TREE_PRESENT | none (archive) | LABS | RETAIN_LABS |
| X11 | [Labs main documentation + policies](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/docs/) — 20 files, RESEARCH_MIXED; SOURCE_TREE_PRESENT | none (Labs internal) | LABS | RETAIN_LABS |
| X12 | [Old handoff evidence folders](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/evidence/) — 3 files, RESEARCH_MIXED; SOURCE_TREE_PRESENT | integration/source-evidence/ | F | SELECT_REDACTED |
| X13 | [Personal knowledge research docs](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/personal-knowledge-lifecycle/) — 36 files, DESIGN_MIXED; SOURCE_TREE_PRESENT | docs/architecture/personal-knowledge/ | B+F | SELECT_ACCEPTED |
| X14 | [Memory research](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/memory/) — 2 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | capability-spaces/personal-world/ | B | POST_BETA |
| X15 | [Learning research](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/learning) — 1 files, DESIGN_ONLY; SOURCE_TREE_PRESENT | capability-spaces/personalization/ | B | POST_BETA |
| X16 | [Labs misc work docs](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/work/) — 2 files, RESEARCH_MIXED; SOURCE_TREE_PRESENT | none (internal work) | LABS | REVIEW_ONLY |
| X17 | [Labs root acceptance tests](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be/tests/) — 4 files, LABS_LOCAL_TESTS; SOURCE_TREE_PRESENT | tests/ only after source-specific review | F | RETAIN_LABS |

## Integration order with no A–F collisions

1. While W1 A–F run: Labs E-category evidence/tests prepared on isolated branches only. No changes to A–F owners or shared manifests.
2. After W1-G: G deduplicates Labs against physically imported source, then assigns E items to existing F/E/D owner branches.
3. After W2-G: production owners A/B/C/D/E can implement selected core adapters for T=0, Chronos, Node 0 and verified voice seams.
4. W3/W4: prove accepted beta workflows on real integrated source and devices; keep high-value R006–R009 and app builder research deferred.

## Important first-pass corrections

- Voice-to-tasks includes a real pure workbench, but microphone/ASR/task persistence is not verified.
- Personal-knowledge T=0 includes ZIP/stream/checkpoint source and synthetic tests, but full Host/Rust integration is unaccepted.
- Chronos has staged Rust/JS/TS source with previous partial test evidence; Cargo and end-to-end acceptance are unrun.
- R005 has Labs UI implementation and synthetic tests; R007 is primarily design.
- No standalone working user app builder was located in the inspected Labs trees; the item points to a related UI-programmability document only.
- Observatory is valuable as verification tooling but cannot become a second product action catalog.

## Status transition policy

PREPARED_NOT_PROMOTED → SOURCE_REVIEWED → OWNER_APPROVED → CONTRACT_TESTED → RUNTIME_WIRED → ENVIRONMENT_ACCEPTED → RELEASE_ACCEPTED.

Only source owner + serial G may authorize promotion. Missing real tests are UNRUN, not PASS.
