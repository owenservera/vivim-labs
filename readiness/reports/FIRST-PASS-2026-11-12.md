# VIVIM Labs → ARBOR: First-pass readiness report

**Assessment completed:** GitHub API read of pinned tracked Git trees (not a local execution of unit tests).
**Date:** 2026-11-12
**Purpose:** classify all major Labs domains into destinations, maturity, beta relevance and owner-dependent gates. No product code merged.

## Source inventory

- Pinned Labs main: `022608f5797e7b4a14ccc4c393bd3c5a4704a4be` — **885 tracked blobs**.
- Pinned Node 0 branch: `61d9fc15c55144c4f0b79afa248981c2b0ca9bd6` — **902 tracked blobs**.
- Pinned UI branch: `56bb145ea1eb479e4d8ec629163a84322797367a` — **888 tracked blobs**.
- **44 placement entries:** 8 easy, 10 core, 9 high_value, 17 hold. All 44 pointed to present tracked paths at their pinned commits.
- **Fresh unit/build/Host/Windows/Android runs: UNRUN in this GitHub-tree assessment.** Recorded test outcomes below come from prior Labs files, not a new run.

## Priority source observations

| Component | Labs source evidence | Known source-level state | ARBOR owner / target | Current action |
| --- | --- | --- | --- | --- |
| T=0 personal knowledge | `personal-knowledge-lifecycle/runtime/` — 28 source files, 13 tests | Labs docs report synthetic pass, production Host/Rust wiring **not accepted** | B+A → Host onboarding + native graph | Prepare integration after W2 |
| Capability Observatory | `capability-observatory/lane-d/` — 11 files, 1 test file | Previously reported 7/7 local tests; 0 certified product capabilities | F → `tools/integration/observatory` | Easy test and schema reuse |
| Voice-to-tasks | `vivim/voice-to-tasks/workbench/` — 5 files, 1 test file | Pure workbench source present; no real microphone/ASR/task commit | F tests, B/E feature owner | Promote tests, not claim Day-0 completion |
| Chronos | `chronos-toolkit/wave2/` staged Rust, JS and TS | Prior report: 3/3 JS and 3/3 TS; Cargo UNRUN; not activated | A/C/D/E + F | Contract/tests first; no second scheduler |
| Composer R005 | UI branch `nlp-command-ui-lab/tests/` — 6 test files | Prior report 28/28 and 100/100 synthetic; browser/device UNRUN | E/F → TierFlow/test projections | Low-cost fixture/semantic promotion |
| Node 0 Prime | Node 0 branch `user-io-rust-core/` — 17 documents | Architectural PRD, Harness draft not compiled or integrated | A/F → `native/node0` | Preserve architecture and negative tests only |
| AutoWiki R008 / Rock R009 | UI branch vision docs | Design with HQ research prototypes, not product writer | D/E and B/D | Post-beta except critical storage guards |
| Automation Queen | `automation-queen/` — 14 files | Discovery/copy-only research, no executor | B/F | Do not ship separate orchestrator |

## All component placement and source scan

| ID | Category | Component | Files | Status | Target in ARBOR | Beta disposition |
| --- | --- | --- | ---: | --- | --- | --- |
| E01 | easy | Capability Observatory seed and schema | 4 | SOURCE_TREE_PRESENT / REPORTED_SYNTHETIC_TESTS | `integration/capability-evidence/` | PROMOTE_TESTS |
| E02 | easy | Capability Observatory lane D observer | 11 | SOURCE_TREE_PRESENT / REPORTED_SYNTHETIC_TESTS | `tools/integration/observatory/` | PROMOTE_TOOL |
| E03 | easy | Verified extension hook research | 3 | SOURCE_TREE_PRESENT / SOURCE_RESEARCH | `capability-spaces/extensions/` | PROMOTE_CONTRACT |
| E04 | easy | Voice-to-tasks pure workbench | 5 | SOURCE_TREE_PRESENT / SOURCE_TESTS_RECORDED | `tests/voice-to-tasks/` | PROMOTE_TESTS |
| E05 | easy | Chronos staged contract | 1 | SOURCE_TREE_PRESENT / STAGED_PARTIAL_TESTS | `contracts/chronos/` | PROMOTE_CONTRACT |
| E06 | easy | UI R005 pure semantic fixtures | 6 | SOURCE_TREE_PRESENT / REPORTED_SYNTHETIC_TESTS | `tests/interaction/` | PROMOTE_TESTS |
| E07 | easy | HQ wiki/provenance research fixtures | 2 | SOURCE_TREE_PRESENT / SOURCE_RESEARCH | `tests/hq-provenance/` | REVIEW_FIXTURES |
| E08 | easy | Development source and capability taxonomy | 106 | SOURCE_TREE_PRESENT / RESEARCH_MIXED | `integration/source-evidence/` | REFERENCE_ONLY |
| C01 | core | T=0 personal knowledge ZIP/runtime | 28 | SOURCE_TREE_PRESENT / REPORTED_SYNTHETIC_TESTS | `apps/vivim-host/onboarding/personal-knowledge/` | INTEGRATE_AFTER_W2 |
| C02 | core | Personal knowledge lifecycle contracts | 1 | SOURCE_TREE_PRESENT / DESIGN_CONTRACT | `contracts/personal-knowledge/` | PREPARE_ADAPTER |
| C03 | core | Node 0 Prime native I/O architecture | 17 | SOURCE_TREE_PRESENT / DESIGN_AND_DRAFT | `native/node0/` | PREPARE_NATIVE |
| C04 | core | Chronos HQKG Rust staging | 3 | SOURCE_TREE_PRESENT / STAGED_PARTIAL_TESTS | `native/hqkg/` | INTEGRATE_AFTER_HQKG |
| C05 | core | Chronos internal HQ JavaScript lens | 3 | SOURCE_TREE_PRESENT / STAGED_PARTIAL_TESTS | `hq/` | PREPARE_ADAPTER |
| C06 | core | Chronos Studio read projection | 3 | SOURCE_TREE_PRESENT / REPORTED_SYNTHETIC_TESTS | `studio/` | PREPARE_ADAPTER |
| C07 | core | Chronos TierFlow read projection | 2 | SOURCE_TREE_PRESENT / REPORTED_SYNTHETIC_TESTS | `packages/tierflow/` | PROMOTE_AFTER_W2 |
| C08 | core | Voice-to-tasks product path | 9 | SOURCE_TREE_PRESENT / DESIGN_PLUS_PURE_WORKBENCH | `capability-spaces/voice-to-tasks/` | BLOCKED_LIVE_PIPELINE |
| C09 | core | Native graph/convergence architecture | 40 | SOURCE_TREE_PRESENT / RESEARCH_MIXED | `native/hqkg/` | SELECTIVE_REUSE |
| C10 | core | Configuration and personal graph research | 3 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `capability-spaces/personalization/` | DEFER_ADAPTER |
| H01 | high_value | R006 predictive composer 100 ideas | 1 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `capability-spaces/interaction/` | POST_BETA |
| H02 | high_value | R007 VIEW ALL full interpretation | 1 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `packages/tierflow/` | POST_BETA |
| H03 | high_value | R008 per-screen AutoWiki | 1 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `capability-spaces/knowledge/` | POST_BETA |
| H04 | high_value | R009 Rock Core storage budget | 1 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `config/storage/` | POST_BETA |
| H05 | high_value | General app builder | 1 | SOURCE_TREE_PRESENT / RELATED_DESIGN_REFERENCE_ONLY | `capability-spaces/app-builder/` | POST_BETA |
| H06 | high_value | Automation Queen | 14 | SOURCE_TREE_PRESENT / DISCOVERY_ONLY | `capability-spaces/automation/` | POST_BETA |
| H07 | high_value | Intelligence algorithm families | 8 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `native/hqkg/ or hq/` | POST_BETA |
| H08 | high_value | Context/learning/memory research | 2 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `capability-spaces/context-dcb/` | POST_BETA |
| H09 | high_value | Experimental voice ASR and media improvements | 1 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `capability-spaces/voice-to-tasks/` | POST_BETA |
| X01 | hold | Labs developer orchestration scaffold | 29 | SOURCE_TREE_PRESENT / NON_PRODUCT_TOOL | `none (Labs internal)` | RETAIN_LABS |
| X02 | hold | Agentic excellence archaeology | 305 | SOURCE_TREE_PRESENT / ARCHIVE | `none (research/archive)` | RETAIN_LABS |
| X03 | hold | Documentation consolidation archive | 140 | SOURCE_TREE_PRESENT / ARCHIVE | `none (archive)` | RETAIN_LABS |
| X04 | hold | Legacy agents/automation files | 62 | SOURCE_TREE_PRESENT / RESEARCH_MIXED | `none (archive or review)` | REVIEW_PRIVACY |
| X05 | hold | Corpora and research history | 15 | SOURCE_TREE_PRESENT / RESEARCH_MIXED | `none (restricted source review)` | REVIEW_PRIVACY |
| X06 | hold | Z.ai round work products | 16 | SOURCE_TREE_PRESENT / RESEARCH_MIXED | `integration/source-provenance/` | RECONCILE_SHA |
| X07 | hold | Local coding model/router toolkit | 17 | SOURCE_TREE_PRESENT / NON_PRODUCT_TOOL | `none (PC tooling)` | RETAIN_LABS |
| X08 | hold | Past orphaned work | 23 | SOURCE_TREE_PRESENT / ARCHIVE | `none until source audit` | RETAIN_LABS |
| X09 | hold | Performance research | 4 | SOURCE_TREE_PRESENT / RESEARCH_MIXED | `tests/performance/` | PROMOTE_BENCHMARK_IDEAS |
| X10 | hold | KB and docs consolidation notes | 6 | SOURCE_TREE_PRESENT / ARCHIVE | `none (archive)` | RETAIN_LABS |
| X11 | hold | Labs main documentation + policies | 20 | SOURCE_TREE_PRESENT / RESEARCH_MIXED | `none (Labs internal)` | RETAIN_LABS |
| X12 | hold | Old handoff evidence folders | 3 | SOURCE_TREE_PRESENT / RESEARCH_MIXED | `integration/source-evidence/` | SELECT_REDACTED |
| X13 | hold | Personal knowledge research docs | 36 | SOURCE_TREE_PRESENT / DESIGN_MIXED | `docs/architecture/personal-knowledge/` | SELECT_ACCEPTED |
| X14 | hold | Memory research | 2 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `capability-spaces/personal-world/` | POST_BETA |
| X15 | hold | Learning research | 1 | SOURCE_TREE_PRESENT / DESIGN_ONLY | `capability-spaces/personalization/` | POST_BETA |
| X16 | hold | Labs misc work docs | 2 | SOURCE_TREE_PRESENT / RESEARCH_MIXED | `none (internal work)` | REVIEW_ONLY |
| X17 | hold | Labs root acceptance tests | 4 | SOURCE_TREE_PRESENT / LABS_LOCAL_TESTS | `tests/ only after source-specific review` | RETAIN_LABS |

## Coverage and audit boundaries

- Main branch top-level tracked groups: 32; mapped groups: 25.
- Unmapped entries are repository root metadata, not evidence of missing product features: `.gitignore`, `AGENTS.md`, `DISCUSSION-RECORD.md`, `README.md`, `bun.lock`, `package.json`, `tsconfig.json`.
- Prefix counts can overlap when parent and child review categories both reference one source (e.g. architecture research). **Do not sum file counts as unique source files.**
- All source files have not been individually code-reviewed; this is a broad git-tree first pass plus targeted source/README inspections.
- Paths under archived corpora may contain personal or third-party material; this report records metadata only, never copies or displays their contents.

## Promotion decisions

**READY FOR DISJOINT EVIDENCE PREP:** E01, E02, E03, E04, E05, E06, E07 (only exact selected fixtures). No production source write.
**PREPARE IN OWNER LANES AFTER W1–W2 GATES:** C01–C10. Original source owners A–F decide code placement; G integrates.
**POST-BETA DESIGN/TRIAL:** H01–H09. R007, AutoWiki, Rock Core full compactor, app builder and autonomous tooling are not beta implementation claims.
**KEEP LABS / MANUAL REVIEW:** X01–X17, excluding only explicitly accepted synthetic tests or sanitized contract fragments.

## Exit criteria for the next readiness run

Execute the Node first-pass scanner and domain unit suites on a runner with complete pinned Git history, record stdout/exit codes, and update this report with **new actual evidence**. Real Windows/browser/PWA and Host integration gates remain distinct. Do not promote SOURCE_TREE_PRESENT to BUILD_PASSED without executable receipts.
