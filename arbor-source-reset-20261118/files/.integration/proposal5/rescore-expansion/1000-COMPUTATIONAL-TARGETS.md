# ARBOR Proposal 5 — Expanded Top 1,000 non-Rust computational opportunity assessment

**Status:** SOURCE-BASED DISCOVERY / NOT A RUST PORTING APPROVAL.  
**Pinned ARBOR commit:** `98f15e327957956ec94189772d5e5813e7b6b07b` · **Tree:** `a8127d14aa5f410721cc67d4c3cb422947b33280`.  
**Machine-readable counterpart:** [1000-COMPUTATIONAL-TARGETS.json](./1000-COMPUTATIONAL-TARGETS.json) · **Source declaration receipts:** [evidence/](./evidence/).  
**Original curated baseline:** [100-COMPUTATIONAL-TARGETS.md](./100-COMPUTATIONAL-TARGETS.md).

## Scope and honesty of the 1,000

**Exactly 1,000 distinct assessment rows:** 100 original curated module-level assessments; **626** additional path-verified non-test JS/TS modules; **274** specific functions whose declarations were found in real ARBOR source. Function-level rows are intentionally nested within some module rows; **1,000 rows do not imply 1,000 independently implemented engines, 1,000 missing Rust capabilities, or 1,000 warranted ports.** File-level discovery means *review for bounded computation*, not that the file itself is computationally hot. Some items are explicitly tagged KEEP_JS. The original 100 are re-ranked together with all additional entries; each keeps its `legacy_rank` field in JSON.

**Score:** `7×workload + 4×frequency + 3×reuse + 2×native_fit – evidence uncertainty penalty – I/O authority penalty` (heuristic, **not** a benchmark). Scores, CPU/RAM pressure (1–5), cadence and primary triggers are provisional. High predicted CPU/peak RAM does not mean large cumulative energy use.

**Cadence definitions:** `ALWAYS_ON` = evidenced continuous duty-cycle (none verified from this source census); `HIGH_FREQUENCY` = active per-message/frame hot path; `FREQUENT` = repeated event-triggered; `USER_DEFINED` = behavior-dependent (H/M/L); `MEDIUM_FREQUENCY` = periodic/incremental; `LOW_FREQUENCY` = infrequent bulk/manual; `DEV_ADMIN` = dev/admin work (H/M/L). The `Always-on?` column shows **potential** background worker names, not verified continuous operation. The separate H/M/L column denotes activity intensity, not calls per second.

**Evidence tiers:** `CURATED_PATH` = carried from source-verified original 100; `TRACKED_PATH` = file path and Git blob SHA verified only; `DECLARATION` = exact named function declaration matched in pinned source text. `KEEP_JS` means preserving effect/I/O/UI ownership is the likely best choice; `REVIEW_ONLY` and `NATIVE_PROBE` require caller tracing and measured profiling. **Do not port on score alone.**

## Source-grounded distribution

| Dimension | Counts |
|---|---|
| Record types | CURATED_MODULE: 100 · DECLARED_FUNCTION: 274 · SOURCE_MODULE: 626 |
| Areas | HARNESS: 92 · HOST: 88 · HQ: 209 · MOBILE: 77 · OTHER: 4 · STUDIO: 407 · TIERFLOW: 123 |
| Cadences | DEV_ADMIN: 294 · FREQUENT: 262 · HIGH_FREQUENCY: 120 · LOW_FREQUENCY: 9 · MEDIUM_FREQUENCY: 8 · USER_DEFINED: 307 |
| Recommendations | ACCEL: 28 · CORE: 72 · KEEP_JS: 169 · NATIVE_PROBE: 297 · REVIEW_ONLY: 434 |
| Always-on verified | 0 (background-candidate flags: 12; not proven) |

## Ranked 1–1,000 inventory

Columns: **CPU / RAM / W / F** are estimated ordinal 1–5; User and Dev are separate behavior intensity labels. **Evidence** is crucial: module vs declared function. Source paths are relative to pinned ARBOR root. Machine format contains exact source blob SHAs, complete score breakdown, legacy ranks and activation triggers.

| # | Area | Candidate computation or source unit | Source path · symbol | CPU | RAM | W | F | Priority | Cadence | User | Dev | Potential always-on? | Evidence | Recommendation |
|---:|---|---|---|---:|---:|---:|---:|---:|---|---|---|---|---|---|
| 1 | TIERFLOW | Revision replay and CAS check | `packages/tierflow/src/visual-runtime/revisions/wave3-atomic-replay.ts` | 5 | 5 | 5 | 5 | **80** | FREQUENT | H | — | — | CURATED_PATH | CORE |
| 2 | STUDIO | Knowledge retrieval ranker | `studio/src/intel/rank.mjs` | 5 | 5 | 5 | 5 | **80** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 3 | STUDIO | Transcript event replay reducer | `studio/src/replay/reducer.mjs` | 5 | 5 | 5 | 5 | **80** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 4 | HARNESS | DCB source selection | `harness/src/context/dcb-source-selection.mjs` | 5 | 3 | 5 | 5 | **80** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 5 | HQ | Bounded task context budgeting | `hq/lib/context-budget.mjs` | 5 | 3 | 5 | 5 | **80** | DEV_ADMIN | — | H | — | CURATED_PATH | CORE |
| 6 | MOBILE | NLCL typed command interpretation | `apps/mobile-relay/nlcl/interpret.mjs` | 5 | 2 | 5 | 5 | **80** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 7 | MOBILE | NLCL lexical analysis | `apps/mobile-relay/nlcl/lexer.mjs` | 5 | 2 | 5 | 5 | **80** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 8 | MOBILE | NLCL grammar recognition | `apps/mobile-relay/nlcl/recognize.mjs` | 5 | 2 | 5 | 5 | **80** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 9 | HOST | Command program validation | `apps/vivim-host/control/command-program.mjs` | 5 | 2 | 5 | 5 | **80** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 10 | TIERFLOW | Typed visual command compilation | `packages/tierflow/src/visual-runtime/command/compile.ts` | 5 | 2 | 5 | 5 | **80** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 11 | STUDIO | Captured content redaction detector | `studio/src/capture/redact.mjs` | 5 | 2 | 5 | 5 | **80** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 12 | HARNESS | Replayable provider reply extraction | `harness/src/core/reply.mjs` | 5 | 5 | 5 | 5 | **78** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 13 | HQ | Source language and import extraction | `hq/lib/extract.mjs` | 5 | 5 | 5 | 5 | **78** | DEV_ADMIN | — | H | — | CURATED_PATH | CORE |
| 14 | STUDIO | Personal knowledge context search | `studio/src/agents/personal-agent-memory-search.mjs` | 5 | 5 | 5 | 5 | **78** | USER_DEFINED | H | — | — | CURATED_PATH | ACCEL |
| 15 | STUDIO | Captured conversation stream decoding | `studio/src/decode/decoders.mjs` | 5 | 5 | 5 | 5 | **78** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 16 | STUDIO | World temporal graph assembly | `studio/src/intel/graph.mjs` | 5 | 5 | 5 | 5 | **78** | MEDIUM_FREQUENCY | M | — | — | CURATED_PATH | ACCEL |
| 17 | STUDIO | Replay message stream construction | `studio/src/replay/stream-messages.mjs` | 5 | 5 | 5 | 5 | **78** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 18 | STUDIO | World full-text search | `studio/src/studio/world-search.mjs` | 5 | 5 | 5 | 5 | **78** | USER_DEFINED | H | — | — | CURATED_PATH | ACCEL |
| 19 | STUDIO | Conversation turn identity reconstruction | `studio/src/capture/thread-identity.mjs` | 5 | 4 | 5 | 5 | **78** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 20 | HOST | Command context grounding | `apps/vivim-host/control/command-context-grounding.mjs` | 5 | 3 | 5 | 5 | **78** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 21 | HARNESS | DCB deterministic composer | `harness/src/context/dcb-composer.mjs` | 5 | 3 | 5 | 5 | **78** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 22 | HARNESS | DCB invalidation projection | `harness/src/context/dcb-invalidation.mjs` | 5 | 3 | 5 | 5 | **78** | FREQUENT | M | — | — | CURATED_PATH | CORE |
| 23 | HARNESS | DCB V2 planning | `harness/src/context/dcb-v2-planner.mjs` | 5 | 3 | 5 | 5 | **78** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 24 | HQ | Context candidate selection and packing | `hq/lib/context.mjs` | 5 | 3 | 5 | 5 | **78** | DEV_ADMIN | — | H | — | CURATED_PATH | CORE |
| 25 | TIERFLOW | Cross-device session reconciliation | `packages/tierflow/src/visual-runtime/revisions/wave4-session-reconcile.ts` | 5 | 3 | 5 | 5 | **78** | FREQUENT | H | — | — | CURATED_PATH | CORE |
| 26 | MOBILE | Generative UI plan compiler | `apps/mobile-relay/generative-ui/compiler.mjs` | 5 | 2 | 5 | 5 | **78** | USER_DEFINED | M | — | — | CURATED_PATH | CORE |
| 27 | HOST | Web memory observation deduplication | `apps/vivim-host/bootstrap/web-memory.mjs` | 5 | 2 | 5 | 5 | **78** | FREQUENT | H | — | — | CURATED_PATH | CORE |
| 28 | HOST | Intent candidate planning | `apps/vivim-host/control/intent-planner.mjs` | 5 | 2 | 5 | 5 | **78** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 29 | HOST | Personal semantic candidate observer | `apps/vivim-host/control/personal-semantic-observer.mjs` | 5 | 2 | 5 | 5 | **78** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 30 | HQ | Repository file discovery | `hq/lib/walk.mjs` | 5 | 2 | 5 | 5 | **78** | DEV_ADMIN | — | H | — | CURATED_PATH | ACCEL |
| 31 | STUDIO | Turn journal normalization | `studio/src/capture/thread-turn-journal.mjs` | 5 | 2 | 5 | 5 | **78** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | ACCEL |
| 32 | STUDIO | Delta event reconstruction | `studio/src/decode/delta.mjs` | 5 | 2 | 5 | 5 | **78** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 33 | STUDIO | Protobuf message decode | `studio/src/decode/protobuf.mjs` | 5 | 2 | 5 | 5 | **78** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 34 | HOST | Natural language lexicon scanning | `apps/vivim-host/control/lexicon.mjs` | 5 | 2 | 5 | 5 | **77** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 35 | TIERFLOW | Visual document validation | `packages/tierflow/src/visual-runtime/document/validate.ts` | 5 | 2 | 5 | 5 | **77** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 36 | HOST | Bootstrap observation and claim indexing | `apps/vivim-host/bootstrap/ledger.mjs` | 5 | 5 | 5 | 5 | **76** | MEDIUM_FREQUENCY | M | — | — | CURATED_PATH | ACCEL |
| 37 | HOST | Local evidence envelope validation | `apps/vivim-host/corpus/intake/host-intake.mjs` | 5 | 5 | 5 | 4 | **76** | FREQUENT | H | — | — | CURATED_PATH | CORE |
| 38 | HARNESS | DCB dependency graph lineage | `harness/src/context/dcb-graph-lineage.mjs` | 5 | 5 | 5 | 4 | **76** | USER_DEFINED | M | — | — | CURATED_PATH | CORE |
| 39 | HQ | Dependency and blast-radius traversal | `hq/lib/graph.mjs` | 5 | 5 | 5 | 4 | **76** | DEV_ADMIN | — | L | — | CURATED_PATH | CORE |
| 40 | HQ | Incremental source indexing | `hq/lib/indexer.mjs` | 5 | 5 | 5 | 5 | **76** | DEV_ADMIN | — | H | — | CURATED_PATH | ACCEL |
| 41 | HQ | Graph exchange delta construction | `hq/lib/kg-export.mjs` | 5 | 5 | 5 | 4 | **76** | DEV_ADMIN | — | H | — | CURATED_PATH | CORE |
| 42 | HQ | Full-text and symbol candidate ranking | `hq/lib/search.mjs` | 5 | 5 | 5 | 5 | **76** | DEV_ADMIN | — | L | — | CURATED_PATH | ACCEL |
| 43 | STUDIO | Graph analytics and projection | `studio/src/intel/graph-analytics.mjs` | 5 | 5 | 5 | 4 | **76** | USER_DEFINED | M | — | — | CURATED_PATH | CORE |
| 44 | HQ | Replay Catalogue Delta declared function | `hq/lib/catalogue-delta.mjs#replayCatalogueDelta` | 5 | 5 | 5 | 5 | **76** | DEV_ADMIN | — | M | — | DECLARATION | NATIVE_PROBE |
| 45 | STUDIO | Derive All declared function | `studio/src/replay/derive-all.mjs#deriveAll` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 46 | STUDIO | Entity Text declared function | `studio/src/replay/derive-all.mjs#entityText` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 47 | STUDIO | Event Text declared function | `studio/src/replay/derive-all.mjs#eventText` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 48 | STUDIO | Load Artifacts declared function | `studio/src/replay/derive-all.mjs#loadArtifacts` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 49 | STUDIO | Raw Count declared function | `studio/src/replay/derive-all.mjs#rawCount` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 50 | STUDIO | Search Session declared function | `studio/src/replay/derive-all.mjs#searchSession` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 51 | STUDIO | Stage Base declared function | `studio/src/replay/derive-all.mjs#stageBase` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 52 | STUDIO | Stage Graph declared function | `studio/src/replay/derive-all.mjs#stageGraph` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 53 | STUDIO | Stage Revisions declared function | `studio/src/replay/derive-all.mjs#stageRevisions` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 54 | STUDIO | Stage Schemas declared function | `studio/src/replay/derive-all.mjs#stageSchemas` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 55 | STUDIO | Stage Search declared function | `studio/src/replay/derive-all.mjs#stageSearch` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 56 | STUDIO | Stage Timelines declared function | `studio/src/replay/derive-all.mjs#stageTimelines` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 57 | STUDIO | Acceptance Label declared function | `studio/src/replay/derive.mjs#acceptanceLabel` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 58 | STUDIO | Build Receipt declared function | `studio/src/replay/derive.mjs#buildReceipt` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 59 | STUDIO | Derive declared function | `studio/src/replay/derive.mjs#derive` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 60 | STUDIO | Fidelity Breakdown declared function | `studio/src/replay/derive.mjs#fidelityBreakdown` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 61 | STUDIO | List Family Datasets declared function | `studio/src/replay/derive.mjs#listFamilyDatasets` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 62 | STUDIO | Reconstruct Stream declared function | `studio/src/replay/derive.mjs#reconstructStream` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 63 | STUDIO | Replay Wide declared function | `studio/src/replay/derive.mjs#replayWide` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 64 | STUDIO | Search declared function | `studio/src/replay/derive.mjs#search` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 65 | STUDIO | State At declared function | `studio/src/replay/derive.mjs#stateAt` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 66 | STUDIO | Stream Bytes declared function | `studio/src/replay/derive.mjs#streamBytes` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 67 | STUDIO | Stream Components declared function | `studio/src/replay/derive.mjs#streamComponents` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 68 | STUDIO | Stream Dom Join declared function | `studio/src/replay/derive.mjs#streamDomJoin` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 69 | STUDIO | Stream Roles declared function | `studio/src/replay/derive.mjs#streamRoles` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 70 | STUDIO | Stream Roles Current declared function | `studio/src/replay/derive.mjs#streamRolesCurrent` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 71 | STUDIO | Stream Text declared function | `studio/src/replay/derive.mjs#streamText` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 72 | STUDIO | With Stream Text declared function | `studio/src/replay/derive.mjs#withStreamText` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 73 | STUDIO | Build Stream Components declared function | `studio/src/replay/stream-components.mjs#buildStreamComponents` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 74 | STUDIO | Signal Summary declared function | `studio/src/replay/stream-components.mjs#signalSummary` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 75 | STUDIO | Build Stream Messages declared function | `studio/src/replay/stream-messages.mjs#buildStreamMessages` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 76 | STUDIO | Assembled Text Blocks declared function | `studio/src/replay/stream-role.mjs#assembledTextBlocks` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 77 | STUDIO | Bounded Entries declared function | `studio/src/replay/stream-role.mjs#boundedEntries` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 78 | STUDIO | Collect Docs declared function | `studio/src/replay/stream-role.mjs#collectDocs` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 79 | STUDIO | Create Growth declared function | `studio/src/replay/stream-role.mjs#createGrowth` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 80 | STUDIO | Create Stats declared function | `studio/src/replay/stream-role.mjs#createStats` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 81 | STUDIO | Descriptor Array declared function | `studio/src/replay/stream-role.mjs#descriptorArray` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 82 | STUDIO | Expand Patch declared function | `studio/src/replay/stream-role.mjs#expandPatch` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 83 | STUDIO | Feed Growth declared function | `studio/src/replay/stream-role.mjs#feedGrowth` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 84 | STUDIO | Growth Facts declared function | `studio/src/replay/stream-role.mjs#growthFacts` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 85 | STUDIO | Growth Summary declared function | `studio/src/replay/stream-role.mjs#growthSummary` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 86 | STUDIO | Is Assistant Frame declared function | `studio/src/replay/stream-role.mjs#isAssistantFrame` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 87 | STUDIO | Mapped Message Components declared function | `studio/src/replay/stream-role.mjs#mappedMessageComponents` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 88 | STUDIO | Mapped Text Components declared function | `studio/src/replay/stream-role.mjs#mappedTextComponents` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 89 | STUDIO | Message Role declared function | `studio/src/replay/stream-role.mjs#messageRole` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 90 | STUDIO | Normalize declared function | `studio/src/replay/stream-role.mjs#normalize` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 91 | STUDIO | Note Container declared function | `studio/src/replay/stream-role.mjs#noteContainer` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 92 | STUDIO | Resolve Classifier Options declared function | `studio/src/replay/stream-role.mjs#resolveClassifierOptions` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 93 | STUDIO | Scalar Stats declared function | `studio/src/replay/stream-role.mjs#scalarStats` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 94 | STUDIO | Snapshot Leaves declared function | `studio/src/replay/stream-role.mjs#snapshotLeaves` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 95 | STUDIO | Textual declared function | `studio/src/replay/stream-role.mjs#textual` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 96 | STUDIO | Token Exchange In declared function | `studio/src/replay/stream-role.mjs#tokenExchangeIn` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 97 | STUDIO | Walk Delta declared function | `studio/src/replay/stream-role.mjs#walkDelta` | 5 | 5 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 98 | STUDIO | Store integrity checks | `studio/src/store/integrity.mjs` | 5 | 4 | 5 | 4 | **76** | MEDIUM_FREQUENCY | M | — | — | CURATED_PATH | CORE |
| 99 | STUDIO | Raw stream log parsing | `studio/src/store/raw-log.mjs` | 5 | 4 | 5 | 5 | **76** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | ACCEL |
| 100 | HARNESS | Valid Event declared function | `harness/src/context/dcb-maintenance.mjs#validEvent` | 5 | 3 | 5 | 5 | **76** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 101 | HQ | Source declared function | `hq/lib/catalogue-delta.mjs#source` | 5 | 2 | 5 | 5 | **76** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 102 | STUDIO | Streaming protobuf feature mapping | `studio/src/decode/proto-features.mjs` | 5 | 5 | 5 | 5 | **75** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 103 | TIERFLOW | Visual hotpath transform | `packages/tierflow/src/hotpath.ts` | 5 | 2 | 5 | 5 | **75** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 104 | HOST | Corpus receipts and batch reconciliation | `apps/vivim-host/corpus/host-integration.mjs` | 5 | 5 | 5 | 4 | **74** | FREQUENT | H | — | — | CURATED_PATH | ACCEL |
| 105 | HQ | Unified KB query and graph view | `hq/lib/kb.mjs` | 5 | 5 | 5 | 4 | **74** | DEV_ADMIN | — | H | — | CURATED_PATH | ACCEL |
| 106 | STUDIO | Corpus knowledge indexing | `studio/src/intel/corpus.mjs` | 5 | 5 | 5 | 4 | **74** | MEDIUM_FREQUENCY | M | — | — | CURATED_PATH | ACCEL |
| 107 | STUDIO | Cross-source entity join | `studio/src/intel/join.mjs` | 5 | 5 | 5 | 4 | **74** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 108 | STUDIO | Entity similarity matching | `studio/src/intel/similarity.mjs` | 5 | 5 | 5 | 4 | **74** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 109 | STUDIO | Temporal truth timeline resolution | `studio/src/intel/temporal-truth.mjs` | 5 | 5 | 5 | 4 | **74** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 110 | STUDIO | Full stream derivation | `studio/src/replay/derive-all.mjs` | 5 | 5 | 5 | 4 | **74** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 111 | STUDIO | Raw store index reconstruction | `studio/src/store/raw-index.mjs` | 5 | 5 | 5 | 4 | **74** | MEDIUM_FREQUENCY | M | — | — | CURATED_PATH | ACCEL |
| 112 | STUDIO | World relationship graph builder | `studio/src/studio/world-relations.mjs` | 5 | 5 | 5 | 4 | **74** | USER_DEFINED | H | — | — | CURATED_PATH | ACCEL |
| 113 | STUDIO | Conversation evidence alignment | `studio/src/capture/conversation-evidence.mjs` | 5 | 4 | 5 | 4 | **74** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 114 | STUDIO | Rich conversation structure parsing | `studio/src/capture/conversation-rich.mjs` | 5 | 4 | 5 | 4 | **74** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 115 | HARNESS | DCB claim evidence reconciliation | `harness/src/context/dcb-claims.mjs` | 5 | 3 | 5 | 4 | **74** | USER_DEFINED | M | — | — | CURATED_PATH | CORE |
| 116 | STUDIO | CSV bulk evidence decode | `studio/src/decode/csv.mjs` | 5 | 3 | 5 | 4 | **73** | USER_DEFINED | M | — | — | CURATED_PATH | CORE |
| 117 | TIERFLOW | Document layout computation | `packages/tierflow/src/visual-runtime/components/layout.ts` | 5 | 2 | 5 | 5 | **73** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | ACCEL |
| 118 | TIERFLOW | Visual knowledge projections | `packages/tierflow/src/visual-runtime/knowledge/projection.ts` | 4 | 5 | 4 | 5 | **73** | FREQUENT | M | — | — | CURATED_PATH | CORE |
| 119 | HARNESS | Event provenance ledger verification | `harness/src/provenance/ledger.mjs` | 4 | 3 | 4 | 5 | **73** | FREQUENT | H | — | — | CURATED_PATH | CORE |
| 120 | MOBILE | Generative UI semantic projection | `apps/mobile-relay/generative-ui/semantic-projection.mjs` | 4 | 2 | 4 | 5 | **73** | USER_DEFINED | M | — | — | CURATED_PATH | CORE |
| 121 | HQ | Wiki graph materialization | `hq/lib/wiki.mjs` | 5 | 5 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | CURATED_PATH | ACCEL |
| 122 | MOBILE | Contact From Address declared function | `apps/mobile-relay/nlcl/index.mjs#contactFromAddress` | 5 | 5 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 123 | MOBILE | Empty World declared function | `apps/mobile-relay/nlcl/index.mjs#emptyWorld` | 5 | 5 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 124 | HARNESS | Project Dcb Graph Lineage declared function | `harness/src/context/dcb-graph-lineage.mjs#projectDcbGraphLineage` | 5 | 5 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 125 | HQ | Plan Compactions declared function | `hq/lib/context-budget.mjs#planCompactions` | 5 | 5 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 126 | HQ | Collect Candidates declared function | `hq/lib/context.mjs#collectCandidates` | 5 | 5 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 127 | HQ | Collect Doc Hits declared function | `hq/lib/context.mjs#collectDocHits` | 5 | 5 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 128 | HQ | Collect Path Hits declared function | `hq/lib/context.mjs#collectPathHits` | 5 | 5 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 129 | HQ | Collect Recent declared function | `hq/lib/context.mjs#collectRecent` | 5 | 5 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 130 | HQ | Collect Symbol Hits declared function | `hq/lib/context.mjs#collectSymbolHits` | 5 | 5 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 131 | HQ | Import Proximity declared function | `hq/lib/context.mjs#importProximity` | 5 | 5 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 132 | STUDIO | Chatgpt Chat Index Reader module computation / boundary audit | `studio/src/capture/chatgpt-chat-index-reader.mjs` | 5 | 5 | 5 | 5 | **72** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 133 | STUDIO | Chatgpt Chat Index module computation / boundary audit | `studio/src/capture/chatgpt-chat-index.mjs` | 5 | 5 | 5 | 5 | **72** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 134 | STUDIO | Replay module computation / boundary audit | `studio/src/mirror/replay.mjs` | 5 | 5 | 5 | 5 | **72** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 135 | STUDIO | Derive module computation / boundary audit | `studio/src/replay/derive.mjs` | 5 | 5 | 5 | 5 | **72** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 136 | STUDIO | Event Id module computation / boundary audit | `studio/src/replay/event-id.mjs` | 5 | 5 | 5 | 5 | **72** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 137 | STUDIO | Stream Components module computation / boundary audit | `studio/src/replay/stream-components.mjs` | 5 | 5 | 5 | 5 | **72** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 138 | STUDIO | Stream Role module computation / boundary audit | `studio/src/replay/stream-role.mjs` | 5 | 5 | 5 | 5 | **72** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 139 | STUDIO | Message Index module computation / boundary audit | `studio/src/studio/message-index.mjs` | 5 | 5 | 5 | 5 | **72** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 140 | HQ | Git commit evidence and drift analysis | `hq/lib/lake.mjs` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | M | — | CURATED_PATH | ACCEL |
| 141 | MOBILE | Resolve Context declared function | `apps/mobile-relay/nlcl/ground.mjs#resolveContext` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 142 | HOST | Context Preview declared function | `apps/vivim-host/bootstrap/web-memory.mjs#contextPreview` | 5 | 3 | 5 | 4 | **72** | FREQUENT | H | — | — | DECLARATION | NATIVE_PROBE |
| 143 | HOST | Ground Command References declared function | `apps/vivim-host/control/command-context-grounding.mjs#groundCommandReferences` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 144 | HOST | Context declared function | `apps/vivim-host/control/command-intent.mjs#context` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 145 | HARNESS | Normalize declared function | `harness/src/context/dcb-claims.mjs#normalize` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 146 | HARNESS | Reconcile Dcb Claims declared function | `harness/src/context/dcb-claims.mjs#reconcileDcbClaims` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 147 | HARNESS | Score declared function | `harness/src/context/dcb-claims.mjs#score` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 148 | HARNESS | Visit declared function | `harness/src/context/dcb-claims.mjs#visit` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 149 | HARNESS | Choose Knapsack declared function | `harness/src/context/dcb-composer.mjs#chooseKnapsack` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 150 | HARNESS | Compose declared function | `harness/src/context/dcb-composer.mjs#compose` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 151 | HARNESS | Create DCBComposer declared function | `harness/src/context/dcb-composer.mjs#createDCBComposer` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 152 | HARNESS | Normalise Request declared function | `harness/src/context/dcb-composer.mjs#normaliseRequest` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 153 | HARNESS | Path Ref declared function | `harness/src/context/dcb-composer.mjs#pathRef` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 154 | HARNESS | Score Candidate declared function | `harness/src/context/dcb-composer.mjs#scoreCandidate` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 155 | HARNESS | Strip Source declared function | `harness/src/context/dcb-composer.mjs#stripSource` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 156 | HARNESS | Verified Scope declared function | `harness/src/context/dcb-composer.mjs#verifiedScope` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 157 | HARNESS | Verify declared function | `harness/src/context/dcb-composer.mjs#verify` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 158 | HARNESS | Compute declared function | `harness/src/context/dcb-dispatch-gate.mjs#compute` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 159 | HARNESS | Prepare declared function | `harness/src/context/dcb-dispatch-gate.mjs#prepare` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 160 | HARNESS | Revalidate declared function | `harness/src/context/dcb-dispatch-gate.mjs#revalidate` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 161 | HARNESS | Valid Scope declared function | `harness/src/context/dcb-dispatch-gate.mjs#validScope` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 162 | HARNESS | Compose Dcb Flavors declared function | `harness/src/context/dcb-flavors.mjs#composeDcbFlavors` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 163 | HARNESS | Detect Dcb Flavors declared function | `harness/src/context/dcb-flavors.mjs#detectDcbFlavors` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 164 | HARNESS | Normalize declared function | `harness/src/context/dcb-flavors.mjs#normalize` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 165 | HARNESS | Prepare declared function | `harness/src/context/dcb-flavors.mjs#prepare` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 166 | HARNESS | Plan Dcb Invalidations declared function | `harness/src/context/dcb-invalidation.mjs#planDcbInvalidations` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 167 | HARNESS | Assess Dcb Definition declared function | `harness/src/context/dcb-lifecycle.mjs#assessDcbDefinition` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 168 | HARNESS | Check Iso declared function | `harness/src/context/dcb-lifecycle.mjs#checkIso` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 169 | HARNESS | Check Ref declared function | `harness/src/context/dcb-lifecycle.mjs#checkRef` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 170 | HARNESS | Create Composite Dcb Definition declared function | `harness/src/context/dcb-lifecycle.mjs#createCompositeDcbDefinition` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 171 | HQ | Census declared function | `hq/lib/context-budget.mjs#census` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 172 | HQ | Day Of declared function | `hq/lib/context-budget.mjs#dayOf` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 173 | HQ | Duration declared function | `hq/lib/context-budget.mjs#duration` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 174 | HQ | Format Tokens declared function | `hq/lib/context-budget.mjs#formatTokens` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 175 | HQ | In Window declared function | `hq/lib/context-budget.mjs#inWindow` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 176 | HQ | Render Context Budget declared function | `hq/lib/context-budget.mjs#renderContextBudget` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 177 | HQ | Resolve Ceiling declared function | `hq/lib/context-budget.mjs#resolveCeiling` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 178 | HQ | Row Total declared function | `hq/lib/context-budget.mjs#rowTotal` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 179 | HQ | Session Contexts declared function | `hq/lib/context-budget.mjs#sessionContexts` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 180 | HQ | Bundle Digest declared function | `hq/lib/context.mjs#bundleDigest` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 181 | HQ | Doc Authority declared function | `hq/lib/context.mjs#docAuthority` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 182 | HQ | Empty Bundle declared function | `hq/lib/context.mjs#emptyBundle` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 183 | HQ | Is Secret Path declared function | `hq/lib/context.mjs#isSecretPath` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 184 | HQ | Merge Ranges declared function | `hq/lib/context.mjs#mergeRanges` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 185 | HQ | Newest Commit Dates declared function | `hq/lib/context.mjs#newestCommitDates` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 186 | HQ | Path Match Bonus declared function | `hq/lib/context.mjs#pathMatchBonus` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 187 | HQ | Prepare Candidates declared function | `hq/lib/context.mjs#prepareCandidates` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 188 | HQ | Reason Label declared function | `hq/lib/context.mjs#reasonLabel` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 189 | HQ | Resolve Project declared function | `hq/lib/context.mjs#resolveProject` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 190 | HQ | Select Symbol Window declared function | `hq/lib/context.mjs#selectSymbolWindow` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 191 | HQ | Select Window declared function | `hq/lib/context.mjs#selectWindow` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 192 | HQ | Source Digest declared function | `hq/lib/context.mjs#sourceDigest` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 193 | HQ | Split Lines declared function | `hq/lib/context.mjs#splitLines` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 194 | HQ | Symbol Match Bonus declared function | `hq/lib/context.mjs#symbolMatchBonus` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 195 | HQ | To Iso declared function | `hq/lib/context.mjs#toIso` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 196 | HQ | Clamp Int declared function | `hq/lib/git-context.mjs#clampInt` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 197 | HQ | Compose Git Context declared function | `hq/lib/git-context.mjs#composeGitContext` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 198 | HQ | Default Run declared function | `hq/lib/git-context.mjs#defaultRun` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 199 | HQ | Detect Repository Duplicates declared function | `hq/lib/git-context.mjs#detectRepositoryDuplicates` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 200 | HQ | Estimate Tokens declared function | `hq/lib/git-context.mjs#estimateTokens` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 201 | HQ | Git Config From Workspace declared function | `hq/lib/git-context.mjs#gitConfigFromWorkspace` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 202 | HQ | Github State declared function | `hq/lib/git-context.mjs#githubState` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 203 | HQ | Git State declared function | `hq/lib/git-context.mjs#gitState` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 204 | HQ | Is Plain Object declared function | `hq/lib/git-context.mjs#isPlainObject` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 205 | HQ | Merge Layer declared function | `hq/lib/git-context.mjs#mergeLayer` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 206 | HQ | Normalize Remote Url declared function | `hq/lib/git-context.mjs#normalizeRemoteUrl` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 207 | HQ | Resolve Git Config declared function | `hq/lib/git-context.mjs#resolveGitConfig` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 208 | HQ | Resolve Repo Root declared function | `hq/lib/git-context.mjs#resolveRepoRoot` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 209 | HQ | Sha256 declared function | `hq/lib/git-context.mjs#sha256` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 210 | HQ | Source declared function | `hq/lib/git-context.mjs#source` | 5 | 3 | 5 | 4 | **72** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 211 | TIERFLOW | Valid Context declared function | `packages/tierflow/src/visual-runtime/command/compile.ts#validContext` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 212 | TIERFLOW | Valid Context declared function | `packages/tierflow/src/visual-runtime/command/wave3-descriptor-proof.ts#validContext` | 5 | 3 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 213 | STUDIO | Cross-event identity inference | `studio/src/intel/identity.mjs` | 5 | 2 | 5 | 4 | **72** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 214 | TIERFLOW | Decode Text declared function | `packages/tierflow/src/visual-runtime/command/compile.ts#decodeText` | 5 | 2 | 5 | 4 | **72** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 215 | HOST | Conversation journal correlation | `apps/vivim-host/conversations/studio-journal-evidence.mjs` | 5 | 4 | 5 | 4 | **71** | FREQUENT | H | — | — | CURATED_PATH | CORE |
| 216 | HARNESS | DCB policy candidate evaluation | `harness/src/context/dcb-policy-eval.mjs` | 4 | 3 | 4 | 5 | **71** | USER_DEFINED | M | — | — | CURATED_PATH | ACCEL |
| 217 | HQ | Incremental wiki invalidation | `hq/lib/freshness.mjs` | 4 | 2 | 4 | 5 | **71** | DEV_ADMIN | — | H | — | CURATED_PATH | CORE |
| 218 | TIERFLOW | Multi-surface compose boundary | `packages/tierflow/src/visual-runtime/final-wave/integration/schema-boundary.ts` | 4 | 2 | 4 | 5 | **71** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 219 | STUDIO | Captured transcript privacy classifier | `studio/src/capture/privacy.mjs` | 4 | 2 | 4 | 5 | **71** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | CORE |
| 220 | HQ | Versioned catalogue delta and replay | `hq/lib/catalogue-delta.mjs` | 5 | 5 | 5 | 3 | **70** | DEV_ADMIN | — | M | — | CURATED_PATH | CORE |
| 221 | HQ | Research evidence relation joins | `hq/lib/evidence-join.mjs` | 5 | 5 | 5 | 3 | **70** | DEV_ADMIN | — | H | — | CURATED_PATH | CORE |
| 222 | STUDIO | Decode Text Frame declared function | `studio/src/decode/decoders.mjs#decodeTextFrame` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 223 | STUDIO | Frame Size declared function | `studio/src/decode/decoders.mjs#frameSize` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 224 | STUDIO | Inflate Frame declared function | `studio/src/decode/decoders.mjs#inflateFrame` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 225 | STUDIO | Text Frame Protocol declared function | `studio/src/decode/decoders.mjs#textFrameProtocol` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 226 | STUDIO | Apply Merge Patch declared function | `studio/src/decode/delta.mjs#applyMergePatch` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 227 | STUDIO | Detect Dialect declared function | `studio/src/decode/delta.mjs#detectDialect` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 228 | STUDIO | Get Parent declared function | `studio/src/decode/delta.mjs#getParent` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 229 | STUDIO | Get Pointer declared function | `studio/src/decode/delta.mjs#getPointer` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 230 | STUDIO | Parse Dotted declared function | `studio/src/decode/delta.mjs#parseDotted` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 231 | STUDIO | Parse Pointer declared function | `studio/src/decode/delta.mjs#parsePointer` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 232 | STUDIO | Reconstruct declared function | `studio/src/decode/delta.mjs#reconstruct` | 5 | 2 | 5 | 5 | **70** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 233 | HQ | Session and tool trace ingestion | `hq/lib/session-import.mjs` | 5 | 4 | 5 | 4 | **69** | DEV_ADMIN | — | M | — | CURATED_PATH | ACCEL |
| 234 | STUDIO | Personal history extraction | `studio/src/intel/history.mjs` | 4 | 5 | 4 | 5 | **69** | USER_DEFINED | M | — | — | CURATED_PATH | CORE |
| 235 | HQ | Snapshot Hq Semantic Ids declared function | `hq/lib/catalogue-delta.mjs#snapshotHqSemanticIds` | 4 | 4 | 4 | 5 | **69** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 236 | HQ | Commit-aware source mapping | `hq/lib/git-context.mjs` | 4 | 3 | 4 | 4 | **69** | DEV_ADMIN | — | H | — | CURATED_PATH | CORE |
| 237 | HQ | Evidence declared function | `hq/lib/catalogue-delta.mjs#evidence` | 4 | 3 | 4 | 5 | **69** | DEV_ADMIN | — | M | — | DECLARATION | NATIVE_PROBE |
| 238 | HOST | Control journal conflict detection | `apps/vivim-host/control/journal.mjs` | 4 | 2 | 4 | 5 | **69** | FREQUENT | H | — | — | CURATED_PATH | ACCEL |
| 239 | HQ | Workspace refresh change planner | `hq/lib/refresh.mjs` | 4 | 2 | 4 | 5 | **69** | DEV_ADMIN | — | H | — | CURATED_PATH | ACCEL |
| 240 | HQ | Semantic ID and link drift | `hq/lib/semantic-registry.mjs` | 4 | 2 | 4 | 4 | **69** | DEV_ADMIN | — | H | — | CURATED_PATH | CORE |
| 241 | TIERFLOW | Visual command source crosswalk | `packages/tierflow/src/visual-runtime/command/crosswalk.ts` | 4 | 2 | 4 | 4 | **69** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 242 | HQ | Revision declared function | `hq/lib/catalogue-delta.mjs#revision` | 4 | 2 | 4 | 5 | **69** | DEV_ADMIN | — | M | — | DECLARATION | NATIVE_PROBE |
| 243 | STUDIO | Store compaction planning | `studio/src/store/compact.mjs` | 5 | 5 | 5 | 3 | **68** | MEDIUM_FREQUENCY | M | — | — | CURATED_PATH | ACCEL |
| 244 | HQ | Is Placeholder Thread declared function | `hq/lib/evidence-join.mjs#isPlaceholderThread` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | M | — | DECLARATION | NATIVE_PROBE |
| 245 | HQ | Join Completion Claims declared function | `hq/lib/evidence-join.mjs#joinCompletionClaims` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | M | — | DECLARATION | NATIVE_PROBE |
| 246 | HQ | Join Evidence declared function | `hq/lib/evidence-join.mjs#joinEvidence` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | M | — | DECLARATION | NATIVE_PROBE |
| 247 | HQ | Lookup Name declared function | `hq/lib/evidence-join.mjs#lookupName` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | M | — | DECLARATION | NATIVE_PROBE |
| 248 | HQ | Resolve Thread declared function | `hq/lib/evidence-join.mjs#resolveThread` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | M | — | DECLARATION | NATIVE_PROBE |
| 249 | HQ | Stored Thread Of declared function | `hq/lib/evidence-join.mjs#storedThreadOf` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | M | — | DECLARATION | NATIVE_PROBE |
| 250 | HQ | Agent Tool Coupling declared function | `hq/lib/graph.mjs#agentToolCoupling` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 251 | HQ | Change Independence declared function | `hq/lib/graph.mjs#changeIndependence` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 252 | HQ | Classify Role declared function | `hq/lib/graph.mjs#classifyRole` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 253 | HQ | Dependency Affected Systems declared function | `hq/lib/graph.mjs#dependencyAffectedSystems` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 254 | HQ | Dependents declared function | `hq/lib/graph.mjs#dependents` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 255 | HQ | Dynamic Loads declared function | `hq/lib/graph.mjs#dynamicLoads` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 256 | HQ | Escape Like declared function | `hq/lib/graph.mjs#escapeLike` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 257 | HQ | Forward Closure declared function | `hq/lib/graph.mjs#forwardClosure` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 258 | HQ | Impact declared function | `hq/lib/graph.mjs#impact` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 259 | HQ | Integration Ranking declared function | `hq/lib/graph.mjs#integrationRanking` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 260 | HQ | Migration Risks declared function | `hq/lib/graph.mjs#migrationRisks` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 261 | HQ | Minimal Connection declared function | `hq/lib/graph.mjs#minimalConnection` | 5 | 5 | 5 | 3 | **68** | DEV_ADMIN | — | H | — | DECLARATION | NATIVE_PROBE |
| 262 | STUDIO | Betweenness declared function | `studio/src/intel/graph-analytics.mjs#betweenness` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 263 | STUDIO | Cmp Id declared function | `studio/src/intel/graph-analytics.mjs#cmpId` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 264 | STUDIO | Communities declared function | `studio/src/intel/graph-analytics.mjs#communities` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 265 | STUDIO | Components declared function | `studio/src/intel/graph-analytics.mjs#components` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 266 | STUDIO | Graph Analytics declared function | `studio/src/intel/graph-analytics.mjs#graphAnalytics` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 267 | STUDIO | Merge Edges declared function | `studio/src/intel/graph-analytics.mjs#mergeEdges` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 268 | STUDIO | Normalize Nodes declared function | `studio/src/intel/graph-analytics.mjs#normalizeNodes` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 269 | STUDIO | Out Adjacency declared function | `studio/src/intel/graph-analytics.mjs#outAdjacency` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 270 | STUDIO | Pagerank declared function | `studio/src/intel/graph-analytics.mjs#pagerank` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 271 | STUDIO | To Array declared function | `studio/src/intel/graph-analytics.mjs#toArray` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 272 | STUDIO | Weight Of declared function | `studio/src/intel/graph-analytics.mjs#weightOf` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 273 | STUDIO | Build Graph declared function | `studio/src/intel/graph.mjs#buildGraph` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 274 | STUDIO | Graph Neighborhood declared function | `studio/src/intel/graph.mjs#graphNeighborhood` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 275 | STUDIO | Graph View declared function | `studio/src/intel/graph.mjs#graphView` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 276 | STUDIO | Rank Graph Nodes declared function | `studio/src/intel/graph.mjs#rankGraphNodes` | 5 | 5 | 5 | 3 | **68** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 277 | MOBILE | Index module computation / boundary audit | `apps/mobile-relay/nlcl/index.mjs` | 5 | 5 | 5 | 4 | **68** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 278 | HQ | Thread Index module computation / boundary audit | `hq/lib/thread-index.mjs` | 5 | 5 | 5 | 4 | **68** | DEV_ADMIN | — | H | — | TRACKED_PATH | NATIVE_PROBE |
| 279 | HQ | Graph Index module computation / boundary audit | `hq/lib/wiki-runtime/graph-index.mjs` | 5 | 5 | 5 | 4 | **68** | DEV_ADMIN | — | H | — | TRACKED_PATH | NATIVE_PROBE |
| 280 | TIERFLOW | Index module computation / boundary audit | `packages/tierflow/src/coordination/index.ts` | 5 | 5 | 5 | 4 | **68** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 281 | STUDIO | Index module computation / boundary audit | `studio/src/mirror/index.mjs` | 5 | 5 | 5 | 4 | **68** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 282 | STUDIO | Index module computation / boundary audit | `studio/src/profiles/sites/index.mjs` | 5 | 5 | 5 | 4 | **68** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 283 | STUDIO | Capability Index module computation / boundary audit | `studio/src/studio/capability-index.mjs` | 5 | 5 | 5 | 4 | **68** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 284 | HQ | Durable kernel revision verification | `hq/lib/kernel/store.mjs` | 5 | 4 | 5 | 3 | **68** | DEV_ADMIN | — | M | — | CURATED_PATH | ACCEL |
| 285 | HARNESS | Dcb Flavors module computation / boundary audit | `harness/src/context/dcb-flavors.mjs` | 5 | 3 | 5 | 4 | **68** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 286 | HARNESS | Dcb Lifecycle module computation / boundary audit | `harness/src/context/dcb-lifecycle.mjs` | 5 | 3 | 5 | 4 | **68** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 287 | HARNESS | Dcb Maintenance module computation / boundary audit | `harness/src/context/dcb-maintenance.mjs` | 5 | 3 | 5 | 4 | **68** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 288 | HARNESS | Dcb Observation Intake module computation / boundary audit | `harness/src/context/dcb-observation-intake.mjs` | 5 | 3 | 5 | 4 | **68** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 289 | HARNESS | Dcb Sources module computation / boundary audit | `harness/src/context/dcb-sources.mjs` | 5 | 3 | 5 | 4 | **68** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 290 | STUDIO | Dcb Context Runtime module computation / boundary audit | `studio/src/agents/dcb-context-runtime.mjs` | 5 | 3 | 5 | 4 | **68** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 291 | HQ | Thread identity lookup and join | `hq/lib/thread-resolve.mjs` | 4 | 5 | 4 | 5 | **68** | DEV_ADMIN | — | M | — | CURATED_PATH | CORE |
| 292 | HOST | Phone delivery receipt reconciliation | `apps/vivim-host/conversations/phone-delivery-acks.mjs` | 4 | 4 | 4 | 5 | **68** | FREQUENT | H | — | — | CURATED_PATH | CORE |
| 293 | HARNESS | Model routing quality scoring | `harness/src/interface/intelligent-model-router.mjs` | 4 | 2 | 4 | 5 | **68** | USER_DEFINED | H | — | — | CURATED_PATH | CORE |
| 294 | HOST | Failure/recovery diagnostic scoring | `apps/vivim-host/recovery/intelligence.mjs` | 4 | 2 | 4 | 4 | **67** | LOW_FREQUENCY | L | — | — | CURATED_PATH | CORE |
| 295 | HQ | Capability source inspection | `hq/lib/capabilities.mjs` | 4 | 2 | 4 | 4 | **67** | DEV_ADMIN | — | M | — | CURATED_PATH | CORE |
| 296 | STUDIO | Read Dataset declared function | `studio/src/replay/derive-all.mjs#readDataset` | 5 | 5 | 5 | 5 | **66** | HIGH_FREQUENCY | H | — | — | DECLARATION | KEEP_JS |
| 297 | STUDIO | Read Kv declared function | `studio/src/replay/derive-all.mjs#readKv` | 5 | 5 | 5 | 5 | **66** | HIGH_FREQUENCY | H | — | — | DECLARATION | KEEP_JS |
| 298 | STUDIO | Write Dataset declared function | `studio/src/replay/derive-all.mjs#writeDataset` | 5 | 5 | 5 | 5 | **66** | HIGH_FREQUENCY | H | — | — | DECLARATION | KEEP_JS |
| 299 | HOST | Data inventory aggregation | `apps/vivim-host/data-intelligence/catalog.mjs` | 4 | 3 | 4 | 4 | **66** | LOW_FREQUENCY | L | — | — | CURATED_PATH | CORE |
| 300 | HQ | Annotated source comment interpreter | `hq/lib/annotation-v2.mjs` | 4 | 2 | 4 | 4 | **66** | DEV_ADMIN | — | M | — | CURATED_PATH | CORE |
| 301 | TIERFLOW | Event bus normalization | `packages/tierflow/src/bus.ts` | 4 | 2 | 4 | 5 | **66** | HIGH_FREQUENCY | H | — | — | CURATED_PATH | ACCEL |
| 302 | HOST | Normalize Rust Query declared function | `apps/vivim-host/bootstrap/web-memory.mjs#normalizeRustQuery` | 4 | 2 | 4 | 4 | **65** | FREQUENT | H | — | — | DECLARATION | NATIVE_PROBE |
| 303 | HOST | Validate Ai Command Candidate declared function | `apps/vivim-host/control/command-intent.mjs#validateAiCommandCandidate` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 304 | HOST | Compile Command Program declared function | `apps/vivim-host/control/command-program.mjs#compileCommandProgram` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 305 | HOST | Validate Command Candidate declared function | `apps/vivim-host/control/intent-planner.mjs#validateCommandCandidate` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 306 | TIERFLOW | Normalize State Path declared function | `packages/tierflow/src/ui/commands.ts#normalizeStatePath` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 307 | TIERFLOW | Checked Input declared function | `packages/tierflow/src/visual-runtime/command/compile.ts#checkedInput` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 308 | TIERFLOW | Compare Pins declared function | `packages/tierflow/src/visual-runtime/command/compile.ts#comparePins` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 309 | TIERFLOW | Compile Visual Command declared function | `packages/tierflow/src/visual-runtime/command/compile.ts#compileVisualCommand` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 310 | TIERFLOW | To Exact Text declared function | `packages/tierflow/src/visual-runtime/command/compile.ts#toExactText` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 311 | TIERFLOW | Check Arguments declared function | `packages/tierflow/src/visual-runtime/command/crosswalk.ts#checkArguments` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 312 | TIERFLOW | Resolve Crosswalk declared function | `packages/tierflow/src/visual-runtime/command/crosswalk.ts#resolveCrosswalk` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 313 | TIERFLOW | Valid Capability declared function | `packages/tierflow/src/visual-runtime/command/crosswalk.ts#validCapability` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 314 | TIERFLOW | Valid Descriptor declared function | `packages/tierflow/src/visual-runtime/command/crosswalk.ts#validDescriptor` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 315 | TIERFLOW | Valid Row declared function | `packages/tierflow/src/visual-runtime/command/crosswalk.ts#validRow` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 316 | TIERFLOW | Valid Schema declared function | `packages/tierflow/src/visual-runtime/command/crosswalk.ts#validSchema` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 317 | TIERFLOW | Valid Target declared function | `packages/tierflow/src/visual-runtime/command/crosswalk.ts#validTarget` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 318 | TIERFLOW | Compile Descriptor Proof declared function | `packages/tierflow/src/visual-runtime/command/wave3-descriptor-proof.ts#compileDescriptorProof` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 319 | TIERFLOW | Descriptor Args Digest declared function | `packages/tierflow/src/visual-runtime/command/wave3-descriptor-proof.ts#descriptorArgsDigest` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 320 | TIERFLOW | Parse Catalog declared function | `packages/tierflow/src/visual-runtime/command/wave3-descriptor-proof.ts#parseCatalog` | 4 | 2 | 4 | 4 | **65** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 321 | HARNESS | Dcb Graph Lineage module computation / boundary audit | `harness/bench/dcb-graph-lineage.mjs` | 5 | 5 | 5 | 3 | **64** | DEV_ADMIN | — | M | — | TRACKED_PATH | NATIVE_PROBE |
| 322 | HQ | Lane A R2 Audit module computation / boundary audit | `hq/lib/live-graph/wave1/lane-a-r2-audit.mjs` | 5 | 5 | 5 | 3 | **64** | DEV_ADMIN | — | H | — | TRACKED_PATH | NATIVE_PROBE |
| 323 | HQ | Lane A R2 Discovery module computation / boundary audit | `hq/lib/live-graph/wave1/lane-a-r2-discovery.mjs` | 5 | 5 | 5 | 3 | **64** | DEV_ADMIN | — | H | — | TRACKED_PATH | NATIVE_PROBE |
| 324 | HQ | Lane A module computation / boundary audit | `hq/lib/live-graph/wave1/lane-a.mjs` | 5 | 5 | 5 | 3 | **64** | DEV_ADMIN | — | H | — | TRACKED_PATH | NATIVE_PROBE |
| 325 | HQ | Personal Corpus Contract module computation / boundary audit | `hq/lib/personal-corpus-contract.mjs` | 5 | 5 | 5 | 3 | **64** | DEV_ADMIN | — | M | — | TRACKED_PATH | NATIVE_PROBE |
| 326 | STUDIO | Graph Ranked module computation / boundary audit | `studio/src/export/graph-ranked.mjs` | 5 | 5 | 5 | 3 | **64** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 327 | HQ | Knowledge note search and ranking | `hq/lib/memory.mjs` | 4 | 5 | 4 | 4 | **64** | DEV_ADMIN | — | H | — | CURATED_PATH | ACCEL |
| 328 | HOST | Media library metadata scanning | `apps/vivim-host/media/library-summary.mjs` | 4 | 4 | 4 | 4 | **64** | LOW_FREQUENCY | L | — | — | CURATED_PATH | ACCEL |
| 329 | HQ | Session part aggregation | `hq/lib/session-parts.mjs` | 4 | 3 | 4 | 4 | **64** | DEV_ADMIN | — | M | — | CURATED_PATH | CORE |
| 330 | HQ | Smart source-to-wiki linking | `hq/lib/comment-wiki-bridge.mjs` | 4 | 2 | 4 | 4 | **64** | DEV_ADMIN | — | M | — | CURATED_PATH | CORE |
| 331 | HOST | Create Lexicon Store declared function | `apps/vivim-host/control/lexicon.mjs#createLexiconStore` | 4 | 4 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 332 | HQ | Document canonical fact extraction | `hq/lib/canon.mjs` | 4 | 2 | 4 | 3 | **63** | DEV_ADMIN | — | L | — | CURATED_PATH | CORE |
| 333 | MOBILE | All Preps declared function | `apps/mobile-relay/nlcl/grammar.mjs#allPreps` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 334 | MOBILE | Body Tokens declared function | `apps/mobile-relay/nlcl/grammar.mjs#bodyTokens` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 335 | MOBILE | Build Canonical declared function | `apps/mobile-relay/nlcl/grammar.mjs#buildCanonical` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 336 | MOBILE | Build Payload declared function | `apps/mobile-relay/nlcl/grammar.mjs#buildPayload` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 337 | MOBILE | Build Reading declared function | `apps/mobile-relay/nlcl/grammar.mjs#buildReading` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 338 | MOBILE | Entity From Match declared function | `apps/mobile-relay/nlcl/grammar.mjs#entityFromMatch` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 339 | MOBILE | Fill Frame declared function | `apps/mobile-relay/nlcl/grammar.mjs#fillFrame` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 340 | MOBILE | Fill Slot Value declared function | `apps/mobile-relay/nlcl/grammar.mjs#fillSlotValue` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 341 | MOBILE | Is Boundary declared function | `apps/mobile-relay/nlcl/grammar.mjs#isBoundary` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 342 | MOBILE | Is Prep Token declared function | `apps/mobile-relay/nlcl/grammar.mjs#isPrepToken` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 343 | MOBILE | Make Entity Slot declared function | `apps/mobile-relay/nlcl/grammar.mjs#makeEntitySlot` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 344 | MOBILE | Match Frames declared function | `apps/mobile-relay/nlcl/grammar.mjs#matchFrames` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 345 | MOBILE | Mention After declared function | `apps/mobile-relay/nlcl/grammar.mjs#mentionAfter` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 346 | MOBILE | Op Short Name declared function | `apps/mobile-relay/nlcl/grammar.mjs#opShortName` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 347 | MOBILE | Unquote Text declared function | `apps/mobile-relay/nlcl/grammar.mjs#unquoteText` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 348 | MOBILE | Is Quote Opener Here declared function | `apps/mobile-relay/nlcl/lexer.mjs#isQuoteOpenerHere` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 349 | MOBILE | Is Word Start declared function | `apps/mobile-relay/nlcl/lexer.mjs#isWordStart` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 350 | MOBILE | Scan Number declared function | `apps/mobile-relay/nlcl/lexer.mjs#scanNumber` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 351 | MOBILE | Scan Word declared function | `apps/mobile-relay/nlcl/lexer.mjs#scanWord` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 352 | TIERFLOW | Integer declared function | `packages/tierflow/src/visual-runtime/components/layout.ts#integer` | 4 | 2 | 4 | 5 | **63** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 353 | TIERFLOW | Invalid declared function | `packages/tierflow/src/visual-runtime/components/layout.ts#invalid` | 4 | 2 | 4 | 5 | **63** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 354 | TIERFLOW | Plain declared function | `packages/tierflow/src/visual-runtime/components/layout.ts#plain` | 4 | 2 | 4 | 5 | **63** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 355 | TIERFLOW | Resolve Layout declared function | `packages/tierflow/src/visual-runtime/components/layout.ts#resolveLayout` | 4 | 2 | 4 | 5 | **63** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 356 | TIERFLOW | Validate Layout Constraints declared function | `packages/tierflow/src/visual-runtime/components/layout.ts#validateLayoutConstraints` | 4 | 2 | 4 | 5 | **63** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 357 | TIERFLOW | Layout For declared function | `packages/tierflow/src/visual-runtime/components/wave3-canonical-map.ts#layoutFor` | 4 | 2 | 4 | 5 | **63** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 358 | HQ | Search1337x module computation / boundary audit | `hq/lib/torrent/search1337x.mjs` | 5 | 5 | 5 | 4 | **62** | DEV_ADMIN | — | H | — | TRACKED_PATH | NATIVE_PROBE |
| 359 | STUDIO | Search module computation / boundary audit | `studio/src/intel/search.mjs` | 5 | 5 | 5 | 4 | **62** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 360 | STUDIO | Classify Mime declared function | `studio/src/decode/decoders.mjs#classifyMime` | 5 | 2 | 5 | 3 | **62** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 361 | STUDIO | Decoder For Kind declared function | `studio/src/decode/decoders.mjs#decoderForKind` | 5 | 2 | 5 | 3 | **62** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 362 | STUDIO | Decode Ws Payload declared function | `studio/src/decode/decoders.mjs#decodeWsPayload` | 5 | 2 | 5 | 3 | **62** | HIGH_FREQUENCY | H | — | — | DECLARATION | NATIVE_PROBE |
| 363 | HQ | KB package ingest classification | `hq/lib/kb-intake.mjs` | 4 | 2 | 4 | 3 | **61** | DEV_ADMIN | — | M | — | CURATED_PATH | ACCEL |
| 364 | MOBILE | Best Score For Alias declared function | `apps/mobile-relay/nlcl/ground.mjs#bestScoreForAlias` | 4 | 2 | 4 | 3 | **61** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 365 | MOBILE | Score Candidate declared function | `apps/mobile-relay/nlcl/interpret.mjs#scoreCandidate` | 4 | 2 | 4 | 3 | **61** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 366 | TIERFLOW | Collect State Values declared function | `packages/tierflow/src/ui/commands.ts#collectStateValues` | 3 | 4 | 4 | 4 | **61** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 367 | TIERFLOW | Is Stored Result declared function | `packages/tierflow/src/ui/commands.ts#isStoredResult` | 3 | 4 | 4 | 4 | **61** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 368 | TIERFLOW | Map Store Code declared function | `packages/tierflow/src/ui/commands.ts#mapStoreCode` | 3 | 4 | 4 | 4 | **61** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 369 | TIERFLOW | Map Store Failure declared function | `packages/tierflow/src/ui/commands.ts#mapStoreFailure` | 3 | 4 | 4 | 4 | **61** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 370 | TIERFLOW | World Error Shape declared function | `packages/tierflow/src/ui/commands.ts#worldErrorShape` | 3 | 4 | 4 | 4 | **61** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 371 | HQ | Domain taxonomy candidate tagging | `hq/lib/taxonomy.mjs` | 4 | 2 | 4 | 3 | **60** | DEV_ADMIN | — | M | — | CURATED_PATH | CORE |
| 372 | TIERFLOW | Knowledge Presentation module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-d/knowledge-presentation.ts` | 2 | 5 | 5 | 3 | **60** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 373 | TIERFLOW | Client module computation / boundary audit | `packages/tierflow/src/visual-runtime/knowledge/client.ts` | 2 | 5 | 5 | 3 | **60** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 374 | TIERFLOW | Fixtures module computation / boundary audit | `packages/tierflow/src/visual-runtime/knowledge/fixtures.ts` | 2 | 5 | 5 | 3 | **60** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 375 | TIERFLOW | Wave3 Feed Projection module computation / boundary audit | `packages/tierflow/src/visual-runtime/knowledge/wave3-feed-projection.ts` | 2 | 5 | 5 | 3 | **60** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 376 | STUDIO | Personal Agent Self Knowledge module computation / boundary audit | `studio/src/agents/personal-agent-self-knowledge.mjs` | 2 | 5 | 5 | 3 | **60** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 377 | TIERFLOW | Valid Evidence declared function | `packages/tierflow/src/visual-runtime/command/wave4-typed-transport-adapter.ts#validEvidence` | 4 | 3 | 4 | 4 | **59** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 378 | HOST | Validate Control Catalog declared function | `apps/vivim-host/control/intent-planner.mjs#validateControlCatalog` | 4 | 2 | 4 | 4 | **59** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 379 | TIERFLOW | Digest declared function | `packages/tierflow/src/visual-runtime/command/wave4-typed-transport-adapter.ts#digest` | 4 | 2 | 4 | 4 | **59** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 380 | TIERFLOW | Identity declared function | `packages/tierflow/src/visual-runtime/command/wave4-typed-transport-adapter.ts#identity` | 4 | 2 | 4 | 4 | **59** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 381 | TIERFLOW | Semantic Digest declared function | `packages/tierflow/src/visual-runtime/command/wave4-typed-transport-adapter.ts#semanticDigest` | 4 | 2 | 4 | 4 | **59** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 382 | TIERFLOW | Validate Typed Envelope declared function | `packages/tierflow/src/visual-runtime/command/wave4-typed-transport-adapter.ts#validateTypedEnvelope` | 4 | 2 | 4 | 4 | **59** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 383 | MOBILE | Grammar module computation / boundary audit | `apps/mobile-relay/nlcl/grammar.mjs` | 4 | 2 | 4 | 5 | **59** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 384 | TIERFLOW | Wave4 Device Snapshot Adapter module computation / boundary audit | `packages/tierflow/src/visual-runtime/knowledge/wave4-device-snapshot-adapter.ts` | 5 | 5 | 5 | 3 | **58** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 385 | HARNESS | Dcb Dispatch Gate module computation / boundary audit | `harness/src/context/dcb-dispatch-gate.mjs` | 5 | 3 | 5 | 4 | **58** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 386 | STUDIO | Extract module computation / boundary audit | `studio/src/shapes/extract.mjs` | 5 | 2 | 5 | 3 | **58** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 387 | HQ | Workload-aware test selection | `hq/lib/ask.mjs` | 4 | 2 | 4 | 3 | **58** | DEV_ADMIN | — | L | — | CURATED_PATH | ACCEL |
| 388 | HOST | Bridge Evidence module computation / boundary audit | `apps/vivim-host/conversations/bridge-evidence.mjs` | 4 | 4 | 4 | 3 | **57** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 389 | STUDIO | Schemas module computation / boundary audit | `studio/src/export/schemas.mjs` | 4 | 4 | 4 | 3 | **57** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 390 | STUDIO | Evidence Partitions module computation / boundary audit | `studio/src/families/evidence-partitions.mjs` | 4 | 3 | 4 | 3 | **57** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 391 | STUDIO | Profile Evidence module computation / boundary audit | `studio/src/intel/profile-evidence.mjs` | 4 | 3 | 4 | 3 | **57** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 392 | STUDIO | Adapter Evidence Intake module computation / boundary audit | `studio/src/studio/adapter-evidence-intake.mjs` | 4 | 3 | 4 | 3 | **57** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 393 | MOBILE | Portable Digest module computation / boundary audit | `apps/mobile-relay/generative-ui/portable-digest.mjs` | 4 | 2 | 4 | 3 | **57** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 394 | HOST | Personal Semantics module computation / boundary audit | `apps/vivim-host/control/personal-semantics.mjs` | 4 | 2 | 4 | 3 | **57** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 395 | HQ | Capability Schema module computation / boundary audit | `hq/lib/capability-schema.mjs` | 4 | 2 | 4 | 3 | **57** | DEV_ADMIN | — | M | — | TRACKED_PATH | NATIVE_PROBE |
| 396 | HQ | Schema module computation / boundary audit | `hq/lib/schema.mjs` | 4 | 2 | 4 | 3 | **57** | DEV_ADMIN | — | M | — | TRACKED_PATH | NATIVE_PROBE |
| 397 | HQ | Semantic Optimizers module computation / boundary audit | `hq/lib/semantic-optimizers.mjs` | 4 | 2 | 4 | 3 | **57** | DEV_ADMIN | — | H | — | TRACKED_PATH | NATIVE_PROBE |
| 398 | TIERFLOW | Schema module computation / boundary audit | `packages/tierflow/src/visual-runtime/document/schema.ts` | 4 | 2 | 4 | 3 | **57** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 399 | TIERFLOW | Wave3 Action Semantics module computation / boundary audit | `packages/tierflow/src/visual-runtime/document/wave3-action-semantics.ts` | 4 | 2 | 4 | 3 | **57** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 400 | TIERFLOW | Wave3 Semantics module computation / boundary audit | `packages/tierflow/src/visual-runtime/render/wave3-semantics.ts` | 4 | 2 | 4 | 3 | **57** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 401 | STUDIO | Schema module computation / boundary audit | `studio/src/intel/schema.mjs` | 4 | 2 | 4 | 3 | **57** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 402 | TIERFLOW | Fixture Store module computation / boundary audit | `packages/tierflow/src/visual-runtime/revisions/fixture-store.ts` | 2 | 4 | 4 | 4 | **57** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 403 | HOST | Reconcile declared function | `apps/vivim-host/bootstrap/scheduler.mjs#reconcile` | 4 | 2 | 4 | 3 | **55** | FREQUENT | H | — | POTENTIAL | DECLARATION | NATIVE_PROBE |
| 404 | TIERFLOW | Validate Accessibility declared function | `packages/tierflow/src/visual-runtime/components/accessibility.ts#validateAccessibility` | 4 | 2 | 4 | 3 | **55** | USER_DEFINED | H | — | — | DECLARATION | NATIVE_PROBE |
| 405 | HOST | World declared function | `apps/vivim-host/control/intent-planner.mjs#world` | 3 | 4 | 4 | 4 | **55** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 406 | STUDIO | Stream Map Induce module computation / boundary audit | `studio/src/profiles/stream-map-induce.mjs` | 3 | 4 | 4 | 5 | **55** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 407 | STUDIO | Stream Map module computation / boundary audit | `studio/src/profiles/stream-map.mjs` | 3 | 4 | 4 | 5 | **55** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 408 | STUDIO | Conversation Capsule module computation / boundary audit | `studio/src/capture/conversation-capsule.mjs` | 2 | 4 | 4 | 5 | **55** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 409 | STUDIO | Conversation Ownership module computation / boundary audit | `studio/src/capture/conversation-ownership.mjs` | 2 | 4 | 4 | 5 | **55** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 410 | STUDIO | Conversation Prefix module computation / boundary audit | `studio/src/capture/conversation-prefix.mjs` | 2 | 4 | 4 | 5 | **55** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 411 | STUDIO | Conversation Runtime module computation / boundary audit | `studio/src/capture/conversation-runtime.mjs` | 2 | 4 | 4 | 5 | **55** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 412 | STUDIO | Conversation Service module computation / boundary audit | `studio/src/capture/conversation-service.mjs` | 2 | 4 | 4 | 5 | **55** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 413 | STUDIO | Conversation Thread module computation / boundary audit | `studio/src/capture/conversation-thread.mjs` | 2 | 4 | 4 | 5 | **55** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 414 | STUDIO | Stream Guard module computation / boundary audit | `studio/src/capture/stream-guard.mjs` | 2 | 4 | 4 | 5 | **55** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 415 | STUDIO | Conversation Events module computation / boundary audit | `studio/src/studio/conversation-events.mjs` | 2 | 4 | 4 | 5 | **55** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 416 | HQ | KB vocabulary binding reconciliation | `hq/lib/kb-bind.mjs` | 3 | 2 | 3 | 2 | **54** | DEV_ADMIN | — | L | — | CURATED_PATH | CORE |
| 417 | HOST | Create Bootstrap Command Worker declared function | `apps/vivim-host/bootstrap/command-worker.mjs#createBootstrapCommandWorker` | 3 | 2 | 3 | 4 | **54** | FREQUENT | H | — | — | DECLARATION | REVIEW_ONLY |
| 418 | HOST | Execute declared function | `apps/vivim-host/bootstrap/command-worker.mjs#execute` | 3 | 2 | 3 | 4 | **54** | FREQUENT | H | — | — | DECLARATION | REVIEW_ONLY |
| 419 | HOST | Action Shape declared function | `apps/vivim-host/control/command-intent.mjs#actionShape` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 420 | HOST | Canonical declared function | `apps/vivim-host/control/command-intent.mjs#canonical` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 421 | HOST | Checked Args declared function | `apps/vivim-host/control/command-intent.mjs#checkedArgs` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 422 | HOST | Provenance declared function | `apps/vivim-host/control/command-intent.mjs#provenance` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 423 | HOST | Refuse declared function | `apps/vivim-host/control/command-intent.mjs#refuse` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 424 | HOST | To Command Intent declared function | `apps/vivim-host/control/command-intent.mjs#toCommandIntent` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 425 | HOST | Valid Args declared function | `apps/vivim-host/control/command-program.mjs#validArgs` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 426 | HOST | Visit declared function | `apps/vivim-host/control/command-program.mjs#visit` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 427 | HOST | Interpret Command declared function | `apps/vivim-host/control/intent-planner.mjs#interpretCommand` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 428 | HOST | Language Pack declared function | `apps/vivim-host/control/intent-planner.mjs#languagePack` | 3 | 2 | 3 | 4 | **54** | FREQUENT | H | — | — | DECLARATION | REVIEW_ONLY |
| 429 | HOST | Valid Language declared function | `apps/vivim-host/control/intent-planner.mjs#validLanguage` | 3 | 2 | 3 | 4 | **54** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 430 | HOST | Personal Rule Store module computation / boundary audit | `apps/vivim-host/control/personal-rule-store.mjs` | 2 | 4 | 4 | 3 | **53** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 431 | STUDIO | Replay module computation / boundary audit | `studio/bin/commands/replay.mjs` | 5 | 5 | 5 | 3 | **52** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 432 | STUDIO | Proto Field Search module computation / boundary audit | `studio/scripts/proto-field-search.mjs` | 5 | 5 | 5 | 3 | **52** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 433 | STUDIO | Roadmap Source Index module computation / boundary audit | `studio/scripts/roadmap-source-index.mjs` | 5 | 5 | 5 | 3 | **52** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 434 | STUDIO | Source Index module computation / boundary audit | `studio/scripts/source-index.mjs` | 5 | 5 | 5 | 3 | **52** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 435 | STUDIO | Context module computation / boundary audit | `studio/bin/lib/context.mjs` | 5 | 3 | 5 | 3 | **52** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 436 | STUDIO | Objective Source Crosswalk module computation / boundary audit | `studio/scripts/objective-source-crosswalk.mjs` | 5 | 2 | 5 | 3 | **52** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 437 | TIERFLOW | Timeline module computation / boundary audit | `packages/tierflow/web/lib/timeline.js` | 4 | 4 | 4 | 3 | **51** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 438 | STUDIO | State Snapshot module computation / boundary audit | `studio/src/studio/state-snapshot.mjs` | 4 | 4 | 4 | 3 | **51** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 439 | OTHER | Validate module computation / boundary audit | `contracts/interop/v1/validate.mjs` | 4 | 2 | 4 | 3 | **51** | DEV_ADMIN | — | L | — | TRACKED_PATH | NATIVE_PROBE |
| 440 | HQ | Model Budget module computation / boundary audit | `hq/lib/model-budget.mjs` | 4 | 2 | 4 | 3 | **51** | DEV_ADMIN | — | M | — | TRACKED_PATH | NATIVE_PROBE |
| 441 | TIERFLOW | Normalize module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-a/normalize.ts` | 4 | 2 | 4 | 3 | **51** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 442 | TIERFLOW | Wave4 Device Semantics module computation / boundary audit | `packages/tierflow/src/visual-runtime/input/wave4-device-semantics.ts` | 4 | 2 | 4 | 3 | **51** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 443 | STUDIO | Learn module computation / boundary audit | `studio/src/intel/learn.mjs` | 4 | 2 | 4 | 3 | **51** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 444 | STUDIO | Learner Journal module computation / boundary audit | `studio/src/intel/learner-journal.mjs` | 4 | 2 | 4 | 3 | **51** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 445 | STUDIO | Statistics module computation / boundary audit | `studio/src/intel/statistics.mjs` | 4 | 2 | 4 | 3 | **51** | USER_DEFINED | H | — | — | TRACKED_PATH | NATIVE_PROBE |
| 446 | STUDIO | Match module computation / boundary audit | `studio/src/mirror/match.mjs` | 4 | 2 | 4 | 3 | **51** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 447 | STUDIO | Infer module computation / boundary audit | `studio/src/shapes/infer.mjs` | 4 | 2 | 4 | 3 | **51** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 448 | STUDIO | Adapter Learning Pipeline module computation / boundary audit | `studio/src/studio/adapter-learning-pipeline.mjs` | 4 | 2 | 4 | 3 | **51** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 449 | STUDIO | Relay Journal Correlation module computation / boundary audit | `studio/src/studio/relay-journal-correlation.mjs` | 4 | 2 | 4 | 3 | **51** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 450 | STUDIO | Uncertain Step Reconciler module computation / boundary audit | `studio/src/studio/uncertain-step-reconciler.mjs` | 4 | 2 | 4 | 3 | **51** | FREQUENT | M | — | — | TRACKED_PATH | NATIVE_PROBE |
| 451 | STUDIO | Saved Session Outcome module computation / boundary audit | `studio/src/export/saved-session-outcome.mjs` | 3 | 4 | 4 | 4 | **51** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 452 | STUDIO | Session module computation / boundary audit | `studio/src/store/session.mjs` | 3 | 4 | 4 | 4 | **51** | MEDIUM_FREQUENCY | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 453 | TIERFLOW | World Visual Bridge module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/integration/world-visual-bridge.ts` | 2 | 4 | 4 | 4 | **51** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 454 | TIERFLOW | World module computation / boundary audit | `packages/tierflow/web/world.js` | 2 | 4 | 4 | 4 | **51** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 455 | STUDIO | World Evolution module computation / boundary audit | `studio/src/intelligence/world-evolution.mjs` | 2 | 4 | 4 | 4 | **51** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 456 | STUDIO | World Bootstrap module computation / boundary audit | `studio/src/studio/world-bootstrap.mjs` | 2 | 4 | 4 | 4 | **51** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 457 | STUDIO | World Control Lineage module computation / boundary audit | `studio/src/studio/world-control-lineage.mjs` | 2 | 4 | 4 | 4 | **51** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 458 | STUDIO | World Conversation Activity module computation / boundary audit | `studio/src/studio/world-conversation-activity.mjs` | 2 | 4 | 4 | 4 | **51** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 459 | STUDIO | World Note Units module computation / boundary audit | `studio/src/studio/world-note-units.mjs` | 2 | 4 | 4 | 4 | **51** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 460 | STUDIO | World Service module computation / boundary audit | `studio/src/studio/world-service.mjs` | 2 | 4 | 4 | 4 | **51** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 461 | STUDIO | World Transfer module computation / boundary audit | `studio/src/studio/world-transfer.mjs` | 2 | 4 | 4 | 4 | **51** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 462 | STUDIO | World Url module computation / boundary audit | `studio/src/studio/world-url.mjs` | 2 | 4 | 4 | 4 | **51** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 463 | HOST | Create Web Memory declared function | `apps/vivim-host/bootstrap/web-memory.mjs#createWebMemory` | 3 | 2 | 3 | 3 | **50** | FREQUENT | H | — | — | DECLARATION | REVIEW_ONLY |
| 464 | HOST | Project declared function | `apps/vivim-host/bootstrap/web-memory.mjs#project` | 3 | 2 | 3 | 3 | **50** | FREQUENT | H | — | — | DECLARATION | REVIEW_ONLY |
| 465 | HOST | Projection Id declared function | `apps/vivim-host/bootstrap/web-memory.mjs#projectionId` | 3 | 2 | 3 | 3 | **50** | FREQUENT | H | — | — | DECLARATION | REVIEW_ONLY |
| 466 | HOST | Recent declared function | `apps/vivim-host/bootstrap/web-memory.mjs#recent` | 3 | 2 | 3 | 3 | **50** | FREQUENT | H | — | — | DECLARATION | REVIEW_ONLY |
| 467 | HOST | Safe Location declared function | `apps/vivim-host/bootstrap/web-memory.mjs#safeLocation` | 3 | 2 | 3 | 3 | **50** | FREQUENT | H | — | — | DECLARATION | REVIEW_ONLY |
| 468 | HOST | Set Opt Out declared function | `apps/vivim-host/bootstrap/web-memory.mjs#setOptOut` | 3 | 2 | 3 | 3 | **50** | FREQUENT | H | — | — | DECLARATION | REVIEW_ONLY |
| 469 | MOBILE | Icon Customize module computation / boundary audit | `apps/mobile-relay/consumer-commands/icon-customize.mjs` | 3 | 2 | 3 | 4 | **50** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 470 | MOBILE | Icon Generator module computation / boundary audit | `apps/mobile-relay/consumer-commands/icon-generator.mjs` | 3 | 2 | 3 | 4 | **50** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 471 | MOBILE | Icon Handlers module computation / boundary audit | `apps/mobile-relay/consumer-commands/icon-handlers.mjs` | 3 | 2 | 3 | 4 | **50** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 472 | MOBILE | Registry module computation / boundary audit | `apps/mobile-relay/consumer-commands/registry.mjs` | 3 | 2 | 3 | 4 | **50** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 473 | HOST | Command Worker module computation / boundary audit | `apps/vivim-host/bootstrap/command-worker.mjs` | 3 | 2 | 3 | 4 | **50** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 474 | HOST | Command Intent module computation / boundary audit | `apps/vivim-host/control/command-intent.mjs` | 3 | 2 | 3 | 4 | **50** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 475 | TIERFLOW | Ir module computation / boundary audit | `packages/tierflow/src/visual-runtime/command/ir.ts` | 3 | 2 | 3 | 4 | **50** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 476 | TIERFLOW | Wave3 Descriptor Proof module computation / boundary audit | `packages/tierflow/src/visual-runtime/command/wave3-descriptor-proof.ts` | 3 | 2 | 3 | 4 | **50** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 477 | STUDIO | Bridge Commands module computation / boundary audit | `studio/src/studio/bridge-commands.mjs` | 3 | 2 | 3 | 4 | **50** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 478 | STUDIO | Commands module computation / boundary audit | `studio/src/studio/commands.mjs` | 3 | 2 | 3 | 4 | **50** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 479 | STUDIO | Typed Prompt Ledger module computation / boundary audit | `studio/src/capture/typed-prompt-ledger.mjs` | 3 | 3 | 3 | 5 | **48** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 480 | HOST | Canonical Risk declared function | `apps/vivim-host/control/intent-planner.mjs#canonicalRisk` | 3 | 2 | 3 | 4 | **48** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 481 | HOST | Create Control Planner declared function | `apps/vivim-host/control/intent-planner.mjs#createControlPlanner` | 3 | 2 | 3 | 4 | **48** | USER_DEFINED | H | — | — | DECLARATION | REVIEW_ONLY |
| 482 | HQ | Receipts Policy module computation / boundary audit | `hq/lib/aii/receipts-policy.mjs` | 3 | 2 | 3 | 5 | **48** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 483 | STUDIO | Capabilities module computation / boundary audit | `studio/src/export/capabilities.mjs` | 3 | 4 | 4 | 3 | **47** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 484 | STUDIO | Export module computation / boundary audit | `studio/src/export/export.mjs` | 3 | 4 | 4 | 3 | **47** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 485 | STUDIO | Outcome Packet module computation / boundary audit | `studio/src/export/outcome-packet.mjs` | 3 | 4 | 4 | 3 | **47** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 486 | MOBILE | Blob Store module computation / boundary audit | `apps/mobile-relay/lib/blob-store.mjs` | 2 | 4 | 4 | 3 | **47** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 487 | MOBILE | Redis Store module computation / boundary audit | `apps/mobile-relay/lib/redis-store.mjs` | 2 | 4 | 4 | 3 | **47** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 488 | HOST | Live Projection module computation / boundary audit | `apps/vivim-host/conversations/live-projection.mjs` | 2 | 4 | 4 | 3 | **47** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 489 | HQ | Store module computation / boundary audit | `hq/lib/fsf/store.mjs` | 2 | 4 | 4 | 3 | **47** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 490 | TIERFLOW | Import Legacy module computation / boundary audit | `packages/tierflow/src/visual-runtime/document/import-legacy.ts` | 2 | 4 | 4 | 3 | **47** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 491 | STUDIO | Dcb Definition Store module computation / boundary audit | `studio/src/agents/dcb-definition-store.mjs` | 2 | 4 | 4 | 3 | **47** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 492 | STUDIO | Versioned module computation / boundary audit | `studio/src/store/versioned.mjs` | 2 | 4 | 4 | 3 | **47** | MEDIUM_FREQUENCY | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 493 | STUDIO | Conversation Tools module computation / boundary audit | `studio/src/studio/conversation-tools.mjs` | 2 | 4 | 4 | 3 | **47** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 494 | HOST | Command Intent.D module computation / boundary audit | `apps/vivim-host/control/command-intent.d.ts` | 3 | 2 | 3 | 4 | **46** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 495 | STUDIO | Web Memory Core module computation / boundary audit | `studio/src/studio/browser-companion/web-memory-core.mjs` | 3 | 2 | 3 | 3 | **46** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 496 | TIERFLOW | Generate Host Tokens module computation / boundary audit | `packages/tierflow/scripts/generate-host-tokens.mjs` | 4 | 2 | 4 | 3 | **45** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 497 | STUDIO | Learn module computation / boundary audit | `studio/bin/commands/learn.mjs` | 4 | 2 | 4 | 3 | **45** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 498 | STUDIO | Categorization Scoreboard module computation / boundary audit | `studio/scripts/categorization-scoreboard.mjs` | 4 | 2 | 4 | 3 | **45** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 499 | HOST | Phone Sessions module computation / boundary audit | `apps/vivim-host/phone-sessions.mjs` | 3 | 3 | 3 | 4 | **44** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 500 | HQ | Session Card module computation / boundary audit | `hq/lib/session-card.mjs` | 3 | 3 | 3 | 4 | **44** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 501 | HQ | Session Views module computation / boundary audit | `hq/lib/session-views.mjs` | 3 | 3 | 3 | 4 | **44** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 502 | HQ | Sessions module computation / boundary audit | `hq/lib/sessions.mjs` | 3 | 3 | 3 | 4 | **44** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 503 | STUDIO | Adapter Accepted Session module computation / boundary audit | `studio/src/studio/adapter-accepted-session.mjs` | 3 | 3 | 3 | 4 | **44** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 504 | HOST | Journal Adapter module computation / boundary audit | `apps/vivim-host/data-intelligence/journal-adapter.mjs` | 2 | 3 | 3 | 4 | **44** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 505 | STUDIO | Conversation Browser Adapter module computation / boundary audit | `studio/src/capture/conversation-browser-adapter.mjs` | 2 | 4 | 4 | 5 | **43** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | KEEP_JS |
| 506 | MOBILE | Host Revision Guard module computation / boundary audit | `apps/mobile-relay/generative-ui/host-revision-guard.mjs` | 2 | 2 | 2 | 4 | **43** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 507 | TIERFLOW | Port module computation / boundary audit | `packages/tierflow/src/visual-runtime/revisions/port.ts` | 2 | 2 | 2 | 4 | **43** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 508 | HOST | Protocol module computation / boundary audit | `apps/vivim-host/corpus/transport/protocol.mjs` | 5 | 5 | 5 | 3 | **42** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 509 | STUDIO | Knowledge module computation / boundary audit | `studio/bin/commands/knowledge.mjs` | 3 | 5 | 5 | 3 | **42** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 510 | MOBILE | Frames module computation / boundary audit | `apps/mobile-relay/nlcl/frames.mjs` | 2 | 2 | 2 | 5 | **41** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 511 | HOST | Receipt Runner module computation / boundary audit | `apps/vivim-host/receipt-runner.mjs` | 2 | 2 | 2 | 5 | **41** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 512 | HQ | Receipts module computation / boundary audit | `hq/lib/receipts.mjs` | 2 | 2 | 2 | 5 | **41** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 513 | STUDIO | Acceptance module computation / boundary audit | `studio/src/capture/acceptance.mjs` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 514 | STUDIO | Actions module computation / boundary audit | `studio/src/capture/actions.mjs` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 515 | STUDIO | Campaign module computation / boundary audit | `studio/src/capture/campaign.mjs` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 516 | STUDIO | Drive Lease module computation / boundary audit | `studio/src/capture/drive-lease.mjs` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 517 | STUDIO | Harness Runs module computation / boundary audit | `studio/src/capture/harness-runs.mjs` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 518 | STUDIO | Human module computation / boundary audit | `studio/src/capture/human.mjs` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 519 | STUDIO | Live Tap module computation / boundary audit | `studio/src/capture/live-tap.mjs` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 520 | STUDIO | Page Agent module computation / boundary audit | `studio/src/capture/page-agent.js` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 521 | STUDIO | Recorder module computation / boundary audit | `studio/src/capture/recorder.mjs` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 522 | STUDIO | Targets module computation / boundary audit | `studio/src/capture/targets.mjs` | 2 | 2 | 2 | 5 | **41** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 523 | STUDIO | Adapter Capture To Signals module computation / boundary audit | `studio/src/studio/adapter-capture-to-signals.mjs` | 2 | 2 | 2 | 5 | **41** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 524 | STUDIO | Event Feed module computation / boundary audit | `studio/src/studio/event-feed.mjs` | 2 | 2 | 2 | 5 | **41** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 525 | STUDIO | Apply module computation / boundary audit | `studio/src/profiles/apply.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 526 | STUDIO | Manage module computation / boundary audit | `studio/src/profiles/manage.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 527 | STUDIO | Migrations module computation / boundary audit | `studio/src/profiles/migrations.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 528 | STUDIO | Model Ops module computation / boundary audit | `studio/src/profiles/model-ops.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 529 | STUDIO | Profiles module computation / boundary audit | `studio/src/profiles/profiles.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 530 | STUDIO | Chatgpt module computation / boundary audit | `studio/src/profiles/sites/chatgpt.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 531 | STUDIO | Claude module computation / boundary audit | `studio/src/profiles/sites/claude.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 532 | STUDIO | Deepseek module computation / boundary audit | `studio/src/profiles/sites/deepseek.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 533 | STUDIO | Duck Ai module computation / boundary audit | `studio/src/profiles/sites/duck-ai.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 534 | STUDIO | Gemini module computation / boundary audit | `studio/src/profiles/sites/gemini.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 535 | STUDIO | Grok module computation / boundary audit | `studio/src/profiles/sites/grok.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 536 | STUDIO | Kimi module computation / boundary audit | `studio/src/profiles/sites/kimi.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 537 | STUDIO | Lmarena module computation / boundary audit | `studio/src/profiles/sites/lmarena.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 538 | STUDIO | Minimax module computation / boundary audit | `studio/src/profiles/sites/minimax.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 539 | STUDIO | Qwen module computation / boundary audit | `studio/src/profiles/sites/qwen.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 540 | STUDIO | Zai module computation / boundary audit | `studio/src/profiles/sites/zai.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 541 | STUDIO | Profiler module computation / boundary audit | `studio/src/shapes/profiler.mjs` | 3 | 3 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 542 | HARNESS | Intent module computation / boundary audit | `harness/src/interface/intent.mjs` | 3 | 2 | 3 | 3 | **40** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 543 | HQ | Skills module computation / boundary audit | `hq/lib/skills.mjs` | 3 | 2 | 3 | 3 | **40** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 544 | STUDIO | Correlate module computation / boundary audit | `studio/src/intel/correlate.mjs` | 3 | 2 | 3 | 3 | **40** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 545 | STUDIO | Intent Bindings module computation / boundary audit | `studio/src/studio/intent-bindings.mjs` | 3 | 2 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 546 | STUDIO | Relay Chat Intent module computation / boundary audit | `studio/src/studio/relay-chat-intent.mjs` | 3 | 2 | 3 | 3 | **40** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 547 | MOBILE | Control Language module computation / boundary audit | `apps/mobile-relay/control-language.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 548 | MOBILE | Relay Core module computation / boundary audit | `apps/mobile-relay/lib/relay-core.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 549 | HOST | Personal Correction module computation / boundary audit | `apps/vivim-host/control/personal-correction.mjs` | 2 | 2 | 2 | 3 | **39** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 550 | HARNESS | Journal module computation / boundary audit | `harness/src/core/journal.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 551 | HARNESS | Models module computation / boundary audit | `harness/src/core/models.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 552 | HARNESS | Pinchtab module computation / boundary audit | `harness/src/core/pinchtab.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 553 | HARNESS | Tab Lifecycle module computation / boundary audit | `harness/src/core/tab-lifecycle.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 554 | HARNESS | Tabs module computation / boundary audit | `harness/src/core/tabs.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 555 | HQ | Mission Sources module computation / boundary audit | `hq/lib/mission-sources.mjs` | 2 | 2 | 2 | 3 | **39** | DEV_ADMIN | — | H | — | TRACKED_PATH | REVIEW_ONLY |
| 556 | STUDIO | Personal Agent Bootstrap module computation / boundary audit | `studio/src/agents/personal-agent-bootstrap.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 557 | STUDIO | Personal Agent Provider Reply module computation / boundary audit | `studio/src/agents/personal-agent-provider-reply.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 558 | STUDIO | Personal Agent Service module computation / boundary audit | `studio/src/agents/personal-agent-service.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 559 | STUDIO | Personal Agent Work module computation / boundary audit | `studio/src/agents/personal-agent-work.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 560 | STUDIO | Personal Ai Baseline module computation / boundary audit | `studio/src/agents/personal-ai-baseline.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 561 | STUDIO | Personal Ai Contracts module computation / boundary audit | `studio/src/agents/personal-ai-contracts.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 562 | STUDIO | Invariants module computation / boundary audit | `studio/src/core/invariants.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 563 | STUDIO | Core module computation / boundary audit | `studio/src/families/core.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 564 | STUDIO | Resources module computation / boundary audit | `studio/src/mcp/resources.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 565 | STUDIO | Core Runtime Bridge module computation / boundary audit | `studio/src/studio/core-runtime-bridge.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 566 | STUDIO | Personal Agent Routes module computation / boundary audit | `studio/src/studio/personal-agent-routes.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 567 | STUDIO | Personal App Ai Design module computation / boundary audit | `studio/src/studio/personal-app-ai-design.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 568 | STUDIO | Personal App Blueprint module computation / boundary audit | `studio/src/studio/personal-app-blueprint.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 569 | STUDIO | Personal App Content module computation / boundary audit | `studio/src/studio/personal-app-content.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 570 | STUDIO | Personal App Provider Reply module computation / boundary audit | `studio/src/studio/personal-app-provider-reply.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 571 | STUDIO | Personal App Routes module computation / boundary audit | `studio/src/studio/personal-app-routes.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 572 | STUDIO | Personal App Service module computation / boundary audit | `studio/src/studio/personal-app-service.mjs` | 2 | 2 | 2 | 3 | **39** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 573 | TIERFLOW | Wave4 Typed Transport Adapter module computation / boundary audit | `packages/tierflow/src/visual-runtime/command/wave4-typed-transport-adapter.ts` | 3 | 2 | 3 | 4 | **38** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 574 | STUDIO | Conversation Api module computation / boundary audit | `studio/src/studio/conversation-api.mjs` | 2 | 4 | 4 | 3 | **37** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 575 | MOBILE | Theme Plan module computation / boundary audit | `apps/mobile-relay/theme-plan.mjs` | 2 | 2 | 2 | 4 | **37** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 576 | HQ | Plan module computation / boundary audit | `hq/lib/aii/plan.mjs` | 2 | 2 | 2 | 4 | **37** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 577 | HQ | Mission Intelligence module computation / boundary audit | `hq/lib/mission-intelligence.mjs` | 2 | 2 | 2 | 4 | **37** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 578 | HQ | Release Plan module computation / boundary audit | `hq/lib/release-plan.mjs` | 2 | 2 | 2 | 4 | **37** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 579 | STUDIO | Plan module computation / boundary audit | `studio/src/recipes/plan.mjs` | 2 | 2 | 2 | 4 | **37** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 580 | HQ | Wave1 Kb Export module computation / boundary audit | `hq/bin/wave1-kb-export.mjs` | 3 | 4 | 4 | 3 | **35** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 581 | STUDIO | Export module computation / boundary audit | `studio/bin/commands/export.mjs` | 3 | 4 | 4 | 3 | **35** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 582 | MOBILE | Relay module computation / boundary audit | `apps/mobile-relay/api/relay.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 583 | MOBILE | Bootstrap F01 module computation / boundary audit | `apps/mobile-relay/bootstrap-f01.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 584 | MOBILE | Bootstrap F02 module computation / boundary audit | `apps/mobile-relay/bootstrap-f02.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 585 | MOBILE | Bootstrap Pass module computation / boundary audit | `apps/mobile-relay/bootstrap-pass.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 586 | MOBILE | Bootstrap Phone module computation / boundary audit | `apps/mobile-relay/bootstrap-phone.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 587 | MOBILE | Bootstrap Prompts module computation / boundary audit | `apps/mobile-relay/bootstrap-prompts.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 588 | MOBILE | Control Chat module computation / boundary audit | `apps/mobile-relay/control-chat.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 589 | MOBILE | Control Client module computation / boundary audit | `apps/mobile-relay/control-client.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 590 | MOBILE | Control Phone module computation / boundary audit | `apps/mobile-relay/control-phone.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 591 | MOBILE | Bootstrap Adapters module computation / boundary audit | `apps/mobile-relay/generative-ui/bootstrap-adapters.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 592 | MOBILE | Catalog module computation / boundary audit | `apps/mobile-relay/generative-ui/catalog.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 593 | MOBILE | Design Composer module computation / boundary audit | `apps/mobile-relay/generative-ui/design-composer.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 594 | MOBILE | Phone Surface module computation / boundary audit | `apps/mobile-relay/generative-ui/phone-surface.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 595 | MOBILE | Grokbot module computation / boundary audit | `apps/mobile-relay/grokbot.js` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 596 | MOBILE | Ground module computation / boundary audit | `apps/mobile-relay/nlcl/ground.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 597 | MOBILE | Project module computation / boundary audit | `apps/mobile-relay/nlcl/project.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 598 | MOBILE | Symbols module computation / boundary audit | `apps/mobile-relay/nlcl/symbols.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 599 | MOBILE | Text module computation / boundary audit | `apps/mobile-relay/nlcl/text.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 600 | MOBILE | Types module computation / boundary audit | `apps/mobile-relay/nlcl/types.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 601 | MOBILE | Pairing Link module computation / boundary audit | `apps/mobile-relay/pairing-link.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 602 | MOBILE | Mission Experience module computation / boundary audit | `apps/mobile-relay/ux/mission-experience.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 603 | MOBILE | Model module computation / boundary audit | `apps/mobile-relay/ux/model.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 604 | MOBILE | Visual Bootstrap module computation / boundary audit | `apps/mobile-relay/visual-bootstrap.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 605 | MOBILE | Visual Theme module computation / boundary audit | `apps/mobile-relay/visual-theme.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 606 | MOBILE | Welcome module computation / boundary audit | `apps/mobile-relay/welcome.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 607 | HOST | Enrollment module computation / boundary audit | `apps/vivim-host/apk/enrollment.mjs` | 2 | 2 | 2 | 3 | **33** | LOW_FREQUENCY | L | — | — | TRACKED_PATH | REVIEW_ONLY |
| 608 | HOST | Pairing module computation / boundary audit | `apps/vivim-host/apk/pairing.mjs` | 2 | 2 | 2 | 3 | **33** | LOW_FREQUENCY | L | — | — | TRACKED_PATH | REVIEW_ONLY |
| 609 | HOST | Registry module computation / boundary audit | `apps/vivim-host/apk/registry.mjs` | 2 | 2 | 2 | 3 | **33** | LOW_FREQUENCY | L | — | — | TRACKED_PATH | REVIEW_ONLY |
| 610 | HOST | Capability Activation module computation / boundary audit | `apps/vivim-host/bootstrap/capability-activation.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 611 | HOST | Readiness Gate module computation / boundary audit | `apps/vivim-host/bootstrap/readiness-gate.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 612 | HOST | Runtime module computation / boundary audit | `apps/vivim-host/bootstrap/runtime.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 613 | HOST | Rust Web Reader module computation / boundary audit | `apps/vivim-host/bootstrap/rust-web-reader.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 614 | HOST | Scheduler module computation / boundary audit | `apps/vivim-host/bootstrap/scheduler.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | H | — | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 615 | HOST | Service module computation / boundary audit | `apps/vivim-host/bootstrap/service.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 616 | HOST | Worker Supervisor module computation / boundary audit | `apps/vivim-host/bootstrap/worker-supervisor.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | H | — | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 617 | HOST | Bootstrap Actions module computation / boundary audit | `apps/vivim-host/control/bootstrap-actions.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 618 | HOST | Cli module computation / boundary audit | `apps/vivim-host/control/cli.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 619 | HOST | Dispatcher module computation / boundary audit | `apps/vivim-host/control/dispatcher.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 620 | HOST | Diagnostics module computation / boundary audit | `apps/vivim-host/diagnostics.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 621 | HOST | Direct Phone module computation / boundary audit | `apps/vivim-host/direct-phone.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 622 | HOST | Linux Preflight module computation / boundary audit | `apps/vivim-host/grokbot/linux-preflight.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 623 | HOST | Run module computation / boundary audit | `apps/vivim-host/grokbot/run.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 624 | HOST | Worker module computation / boundary audit | `apps/vivim-host/grokbot/worker.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 625 | HOST | Insights module computation / boundary audit | `apps/vivim-host/insights.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 626 | HOST | Environment module computation / boundary audit | `apps/vivim-host/onboarding/environment.mjs` | 2 | 2 | 2 | 3 | **33** | LOW_FREQUENCY | L | — | — | TRACKED_PATH | REVIEW_ONLY |
| 627 | HOST | Service module computation / boundary audit | `apps/vivim-host/onboarding/service.mjs` | 2 | 2 | 2 | 3 | **33** | LOW_FREQUENCY | L | — | — | TRACKED_PATH | REVIEW_ONLY |
| 628 | HOST | Studio Port module computation / boundary audit | `apps/vivim-host/onboarding/studio-port.mjs` | 2 | 2 | 2 | 3 | **33** | LOW_FREQUENCY | L | — | — | TRACKED_PATH | REVIEW_ONLY |
| 629 | HOST | Host Adapter module computation / boundary audit | `apps/vivim-host/recovery/host-adapter.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 630 | HOST | Intelligence.D module computation / boundary audit | `apps/vivim-host/recovery/intelligence.d.ts` | 2 | 2 | 2 | 4 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 631 | HOST | Supervisor module computation / boundary audit | `apps/vivim-host/runtime/supervisor.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 632 | OTHER | Types module computation / boundary audit | `contracts/interop/v1/types.ts` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | L | — | TRACKED_PATH | REVIEW_ONLY |
| 633 | HARNESS | Dcb Synthetic Eval module computation / boundary audit | `harness/bench/dcb-synthetic-eval.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 634 | HARNESS | Cli module computation / boundary audit | `harness/src/agents/cli.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 635 | HARNESS | Fleet module computation / boundary audit | `harness/src/agents/fleet.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 636 | HARNESS | Roles module computation / boundary audit | `harness/src/agents/roles.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 637 | HARNESS | Roundrobin module computation / boundary audit | `harness/src/agents/roundrobin.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 638 | HARNESS | Swarm module computation / boundary audit | `harness/src/agents/swarm.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 639 | HARNESS | Agent module computation / boundary audit | `harness/src/dev/agent.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 640 | HARNESS | Bootstrap module computation / boundary audit | `harness/src/dev/bootstrap.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 641 | HARNESS | Cli module computation / boundary audit | `harness/src/dev/cli.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 642 | HARNESS | Workspace module computation / boundary audit | `harness/src/dev/workspace.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 643 | HARNESS | Approval module computation / boundary audit | `harness/src/interface/approval.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 644 | HARNESS | Brain module computation / boundary audit | `harness/src/interface/brain.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 645 | HARNESS | Deps module computation / boundary audit | `harness/src/interface/deps.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 646 | HARNESS | Prefs module computation / boundary audit | `harness/src/interface/prefs.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 647 | HARNESS | Router module computation / boundary audit | `harness/src/interface/router.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 648 | HARNESS | Standing module computation / boundary audit | `harness/src/interface/standing.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 649 | HARNESS | Target module computation / boundary audit | `harness/src/interface/target.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 650 | HARNESS | Compare Runner module computation / boundary audit | `harness/src/onboarding/compare-runner.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 651 | HARNESS | Feed Publisher module computation / boundary audit | `harness/src/share/feed-publisher.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 652 | HQ | Action Registry module computation / boundary audit | `hq/lib/action-registry.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 653 | HQ | Action Router module computation / boundary audit | `hq/lib/action-router.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 654 | HQ | Aliases module computation / boundary audit | `hq/lib/actions/aliases.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 655 | HQ | Handlers module computation / boundary audit | `hq/lib/actions/handlers.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 656 | HQ | Verbs module computation / boundary audit | `hq/lib/actions/verbs.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 657 | HQ | Registry module computation / boundary audit | `hq/lib/aii/registry.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 658 | HQ | Runner module computation / boundary audit | `hq/lib/aii/runner.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 659 | HQ | Scenarios module computation / boundary audit | `hq/lib/aii/scenarios.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 660 | HQ | State module computation / boundary audit | `hq/lib/aii/state.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 661 | HQ | Views module computation / boundary audit | `hq/lib/aii/views.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 662 | HQ | Approvals module computation / boundary audit | `hq/lib/approvals.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 663 | HQ | Arbor Service module computation / boundary audit | `hq/lib/arbor-service.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 664 | HQ | Automation module computation / boundary audit | `hq/lib/automation.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 665 | HQ | Autonomy module computation / boundary audit | `hq/lib/autonomy.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 666 | HQ | Canvas module computation / boundary audit | `hq/lib/canvas.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 667 | HQ | Channel Broker module computation / boundary audit | `hq/lib/channel-broker.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 668 | HQ | Channel Seats module computation / boundary audit | `hq/lib/channel-seats.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 669 | HQ | Channel Wake module computation / boundary audit | `hq/lib/channel-wake.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 670 | HQ | Cnp module computation / boundary audit | `hq/lib/cnp.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 671 | HQ | Comms Roof module computation / boundary audit | `hq/lib/comms-roof.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 672 | HQ | Components module computation / boundary audit | `hq/lib/components.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 673 | HQ | Dogfood module computation / boundary audit | `hq/lib/dogfood.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 674 | HQ | Filetypes module computation / boundary audit | `hq/lib/filetypes.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 675 | HQ | Client module computation / boundary audit | `hq/lib/fsf/client.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 676 | HQ | Slices module computation / boundary audit | `hq/lib/fsf/slices.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 677 | HQ | Git module computation / boundary audit | `hq/lib/git.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 678 | HQ | Home module computation / boundary audit | `hq/lib/home.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 679 | HQ | Cli module computation / boundary audit | `hq/lib/kernel/cli.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 680 | HQ | Lane Health module computation / boundary audit | `hq/lib/lane-health.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 681 | HQ | Lang module computation / boundary audit | `hq/lib/lang.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 682 | HQ | Machine Probe module computation / boundary audit | `hq/lib/machine-probe.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 683 | HQ | Mcp Team Tools module computation / boundary audit | `hq/lib/mcp-team-tools.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 684 | HQ | Mcp module computation / boundary audit | `hq/lib/mcp.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 685 | HQ | Mission Evaluator module computation / boundary audit | `hq/lib/mission-evaluator.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 686 | HQ | Mission Shadow module computation / boundary audit | `hq/lib/mission-shadow.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 687 | HQ | Model Inventory module computation / boundary audit | `hq/lib/model-inventory.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 688 | HQ | Model Router module computation / boundary audit | `hq/lib/model-router.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 689 | HQ | Nomenclature module computation / boundary audit | `hq/lib/nomenclature.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 690 | HQ | Pmo module computation / boundary audit | `hq/lib/pmo.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 691 | HQ | Protocol module computation / boundary audit | `hq/lib/protocol.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 692 | HQ | Realtime module computation / boundary audit | `hq/lib/realtime.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 693 | HQ | Registry module computation / boundary audit | `hq/lib/registry.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 694 | HQ | Release Status module computation / boundary audit | `hq/lib/release-status.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 695 | HQ | Revive Daemon module computation / boundary audit | `hq/lib/revive-daemon.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | H | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 696 | HQ | Revive Lane module computation / boundary audit | `hq/lib/revive-lane.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 697 | HQ | Revocations module computation / boundary audit | `hq/lib/revocations.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 698 | HQ | Seat Heartbeat module computation / boundary audit | `hq/lib/seat-heartbeat.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 699 | HQ | Seat Watch module computation / boundary audit | `hq/lib/seat-watch.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | H | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 700 | HQ | Spawn module computation / boundary audit | `hq/lib/spawn.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 701 | HQ | Task Lifecycle module computation / boundary audit | `hq/lib/task-lifecycle.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 702 | HQ | Team Automation module computation / boundary audit | `hq/lib/team-automation.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 703 | HQ | Team Dashboard module computation / boundary audit | `hq/lib/team-dashboard.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 704 | HQ | Team Panel module computation / boundary audit | `hq/lib/team-panel.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 705 | HQ | Team Protocol module computation / boundary audit | `hq/lib/team-protocol.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 706 | HQ | Telemetry module computation / boundary audit | `hq/lib/telemetry.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 707 | HQ | Toolchain Guard module computation / boundary audit | `hq/lib/toolchain-guard.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 708 | HQ | Auto module computation / boundary audit | `hq/lib/torrent/auto.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 709 | HQ | Library module computation / boundary audit | `hq/lib/torrent/library.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 710 | HQ | Poll module computation / boundary audit | `hq/lib/torrent/poll.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 711 | HQ | Qbittorrent module computation / boundary audit | `hq/lib/torrent/qbittorrent.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 712 | HQ | Releases module computation / boundary audit | `hq/lib/torrent/releases.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 713 | HQ | Wanted module computation / boundary audit | `hq/lib/torrent/wanted.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 714 | HQ | Util module computation / boundary audit | `hq/lib/util.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 715 | HQ | Vivim module computation / boundary audit | `hq/lib/vivim.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 716 | HQ | Wiki Program module computation / boundary audit | `hq/lib/wiki-program.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | H | — | TRACKED_PATH | REVIEW_ONLY |
| 717 | HQ | Rust Comment Adapter module computation / boundary audit | `hq/lib/wiki-runtime/rust-comment-adapter.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | H | — | TRACKED_PATH | REVIEW_ONLY |
| 718 | HQ | Work Orders module computation / boundary audit | `hq/lib/work-orders.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 719 | HQ | Zcode Doctor module computation / boundary audit | `hq/lib/zcode-doctor.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 720 | TIERFLOW | Cli module computation / boundary audit | `packages/tierflow/src/cli.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 721 | TIERFLOW | Config module computation / boundary audit | `packages/tierflow/src/config.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 722 | TIERFLOW | Contract module computation / boundary audit | `packages/tierflow/src/coordination/contract.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 723 | TIERFLOW | Poller module computation / boundary audit | `packages/tierflow/src/coordination/poller.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 724 | TIERFLOW | Lab module computation / boundary audit | `packages/tierflow/src/lab.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 725 | TIERFLOW | Observability module computation / boundary audit | `packages/tierflow/src/observability.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 726 | TIERFLOW | Protocol module computation / boundary audit | `packages/tierflow/src/protocol.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 727 | TIERFLOW | Registry module computation / boundary audit | `packages/tierflow/src/registry.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 728 | TIERFLOW | Security module computation / boundary audit | `packages/tierflow/src/security.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 729 | TIERFLOW | Stats module computation / boundary audit | `packages/tierflow/src/stats.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 730 | TIERFLOW | Surface module computation / boundary audit | `packages/tierflow/src/surface.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 731 | TIERFLOW | Theme module computation / boundary audit | `packages/tierflow/src/theme.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 732 | TIERFLOW | Agent module computation / boundary audit | `packages/tierflow/src/tier1/agent.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 733 | TIERFLOW | Router module computation / boundary audit | `packages/tierflow/src/tier2/router.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 734 | TIERFLOW | Accessibility module computation / boundary audit | `packages/tierflow/src/visual-runtime/components/accessibility.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 735 | TIERFLOW | Catalog module computation / boundary audit | `packages/tierflow/src/visual-runtime/components/catalog.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 736 | TIERFLOW | Wave3 Canonical Map module computation / boundary audit | `packages/tierflow/src/visual-runtime/components/wave3-canonical-map.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 737 | TIERFLOW | Wave4 Desktop Adapter module computation / boundary audit | `packages/tierflow/src/visual-runtime/device/wave4-desktop-adapter.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 738 | TIERFLOW | Wave4 Phone Adapter module computation / boundary audit | `packages/tierflow/src/visual-runtime/device/wave4-phone-adapter.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 739 | TIERFLOW | Help module computation / boundary audit | `packages/tierflow/src/visual-runtime/experience/help.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 740 | TIERFLOW | Onboarding module computation / boundary audit | `packages/tierflow/src/visual-runtime/experience/onboarding.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 741 | TIERFLOW | Recovery module computation / boundary audit | `packages/tierflow/src/visual-runtime/experience/recovery.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 742 | TIERFLOW | Wave3 Journey Fixture module computation / boundary audit | `packages/tierflow/src/visual-runtime/experience/wave3-journey-fixture.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 743 | TIERFLOW | Wave4 Onboarding Resume module computation / boundary audit | `packages/tierflow/src/visual-runtime/experience/wave4-onboarding-resume.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 744 | TIERFLOW | Master module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/integration/master.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 745 | TIERFLOW | Runner module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/integration/runner.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 746 | TIERFLOW | Studio Editing module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-b/studio-editing.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 747 | TIERFLOW | Adapters module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-c/adapters.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 748 | TIERFLOW | First Edit module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-c/first-edit.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 749 | TIERFLOW | Templates module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-c/templates.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 750 | TIERFLOW | Host Journey module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-e/host-journey.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 751 | TIERFLOW | Existing Clients module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-f/existing-clients.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 752 | TIERFLOW | Cutover Dry Run module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-g/cutover-dry-run.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 753 | TIERFLOW | Verification module computation / boundary audit | `packages/tierflow/src/visual-runtime/final-wave/lane-h/verification.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 754 | TIERFLOW | Fixture module computation / boundary audit | `packages/tierflow/src/visual-runtime/host/fixture.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 755 | TIERFLOW | Outcome module computation / boundary audit | `packages/tierflow/src/visual-runtime/host/outcome.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 756 | TIERFLOW | Port module computation / boundary audit | `packages/tierflow/src/visual-runtime/host/port.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 757 | TIERFLOW | Wave4 H Host Status Contract module computation / boundary audit | `packages/tierflow/src/visual-runtime/host/wave4-h-host-status-contract.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 758 | TIERFLOW | Action Adapter module computation / boundary audit | `packages/tierflow/src/visual-runtime/integration-2/action-adapter.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 759 | TIERFLOW | Compose module computation / boundary audit | `packages/tierflow/src/visual-runtime/integration-2/compose.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 760 | TIERFLOW | Compose module computation / boundary audit | `packages/tierflow/src/visual-runtime/integration-3/compose.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 761 | TIERFLOW | Compose module computation / boundary audit | `packages/tierflow/src/visual-runtime/integration-4/compose.ts` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 762 | TIERFLOW | Inspect module computation / boundary audit | `packages/tierflow/src/visual-runtime/render/inspect.ts` | 2 | 2 | 2 | 3 | **33** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 763 | TIERFLOW | Spatial Fallback module computation / boundary audit | `packages/tierflow/src/visual-runtime/render/spatial-fallback.ts` | 2 | 2 | 2 | 3 | **33** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 764 | TIERFLOW | Format module computation / boundary audit | `packages/tierflow/web/lib/format.js` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 765 | TIERFLOW | Palette module computation / boundary audit | `packages/tierflow/web/lib/palette.js` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 766 | TIERFLOW | Runboard module computation / boundary audit | `packages/tierflow/web/lib/runboard.js` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 767 | TIERFLOW | Topology module computation / boundary audit | `packages/tierflow/web/lib/topology.js` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 768 | TIERFLOW | Transcript module computation / boundary audit | `packages/tierflow/web/lib/transcript.js` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 769 | STUDIO | Fixtures V2 module computation / boundary audit | `studio/eval/fixtures-v2.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 770 | STUDIO | Fixtures module computation / boundary audit | `studio/eval/fixtures.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 771 | STUDIO | Run module computation / boundary audit | `studio/eval/run.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 772 | STUDIO | Config module computation / boundary audit | `studio/src/config.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 773 | STUDIO | Families module computation / boundary audit | `studio/src/families/families.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 774 | STUDIO | Autodetect module computation / boundary audit | `studio/src/intel/autodetect.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 775 | STUDIO | Bench module computation / boundary audit | `studio/src/intel/bench.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 776 | STUDIO | Discover module computation / boundary audit | `studio/src/intel/discover.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 777 | STUDIO | Platform module computation / boundary audit | `studio/src/intel/platform.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 778 | STUDIO | Priors module computation / boundary audit | `studio/src/intel/priors.mjs` | 2 | 2 | 2 | 3 | **33** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 779 | STUDIO | Jsonrpc module computation / boundary audit | `studio/src/mcp/jsonrpc.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 780 | STUDIO | Runtime module computation / boundary audit | `studio/src/mcp/runtime.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 781 | STUDIO | Catalog module computation / boundary audit | `studio/src/mirror/catalog.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 782 | STUDIO | Synthesize module computation / boundary audit | `studio/src/mirror/synthesize.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 783 | STUDIO | Template module computation / boundary audit | `studio/src/mirror/template.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 784 | STUDIO | Ws module computation / boundary audit | `studio/src/mirror/ws.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 785 | STUDIO | Registry module computation / boundary audit | `studio/src/plugins/registry.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 786 | STUDIO | Catalog module computation / boundary audit | `studio/src/recipes/catalog.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 787 | STUDIO | Choose module computation / boundary audit | `studio/src/recipes/choose.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 788 | STUDIO | Service module computation / boundary audit | `studio/src/recipes/service.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 789 | STUDIO | Patterns module computation / boundary audit | `studio/src/security/patterns.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 790 | STUDIO | Observation module computation / boundary audit | `studio/src/shapes/observation.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 791 | STUDIO | Adapter Factory Service module computation / boundary audit | `studio/src/studio/adapter-factory-service.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 792 | STUDIO | Adapter Holdout Gate module computation / boundary audit | `studio/src/studio/adapter-holdout-gate.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 793 | STUDIO | Adapter Improvement Pipeline module computation / boundary audit | `studio/src/studio/adapter-improvement-pipeline.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 794 | STUDIO | Adapter Probes module computation / boundary audit | `studio/src/studio/adapter-probes.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 795 | STUDIO | Adapter Studio Bridge module computation / boundary audit | `studio/src/studio/adapter-studio-bridge.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 796 | STUDIO | Adaptive Adapter Factory module computation / boundary audit | `studio/src/studio/adaptive-adapter-factory.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 797 | STUDIO | Agent Actions module computation / boundary audit | `studio/src/studio/agent-actions.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 798 | STUDIO | Agentic Files module computation / boundary audit | `studio/src/studio/agentic-files.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 799 | STUDIO | Ai Orchestrator module computation / boundary audit | `studio/src/studio/ai-orchestrator.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 800 | STUDIO | Ai Router module computation / boundary audit | `studio/src/studio/ai-router.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 801 | STUDIO | Ask Prompts module computation / boundary audit | `studio/src/studio/ask-prompts.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 802 | STUDIO | Atom Coverage module computation / boundary audit | `studio/src/studio/atom-coverage.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 803 | STUDIO | Atom Kit module computation / boundary audit | `studio/src/studio/atom-kit.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 804 | STUDIO | Atom Roles module computation / boundary audit | `studio/src/studio/atom-roles.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 805 | STUDIO | Bootstrap Owner Adapter module computation / boundary audit | `studio/src/studio/bootstrap-owner-adapter.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 806 | STUDIO | Ingress module computation / boundary audit | `studio/src/studio/browser-companion/ingress.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 807 | STUDIO | Capabilities module computation / boundary audit | `studio/src/studio/capabilities.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 808 | STUDIO | Claude module computation / boundary audit | `studio/src/studio/channels/claude.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 809 | STUDIO | Gemini module computation / boundary audit | `studio/src/studio/channels/gemini.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 810 | STUDIO | Chat Channel module computation / boundary audit | `studio/src/studio/chat-channel.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 811 | STUDIO | Compare Routes module computation / boundary audit | `studio/src/studio/compare-routes.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 812 | STUDIO | Compare Service module computation / boundary audit | `studio/src/studio/compare-service.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 813 | STUDIO | Drive Output module computation / boundary audit | `studio/src/studio/drive-output.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 814 | STUDIO | Four Lane Gates module computation / boundary audit | `studio/src/studio/four-lane-gates.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 815 | STUDIO | Integration Hub module computation / boundary audit | `studio/src/studio/integration-hub.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 816 | STUDIO | Orchestrator Patterns module computation / boundary audit | `studio/src/studio/orchestrator-patterns.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 817 | STUDIO | Orchestrator Routes module computation / boundary audit | `studio/src/studio/orchestrator-routes.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 818 | STUDIO | Platform X module computation / boundary audit | `studio/src/studio/platform-x.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 819 | STUDIO | Provider Performance Routes module computation / boundary audit | `studio/src/studio/provider-performance-routes.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 820 | STUDIO | Provider Performance module computation / boundary audit | `studio/src/studio/provider-performance.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 821 | STUDIO | Recipe Routes module computation / boundary audit | `studio/src/studio/recipe-routes.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 822 | STUDIO | Relay Agent module computation / boundary audit | `studio/src/studio/relay-agent.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 823 | STUDIO | State Accounts module computation / boundary audit | `studio/src/studio/state-accounts.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 824 | STUDIO | State Bind module computation / boundary audit | `studio/src/studio/state-bind.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 825 | STUDIO | State Live module computation / boundary audit | `studio/src/studio/state-live.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 826 | STUDIO | State Page module computation / boundary audit | `studio/src/studio/state-page.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 827 | STUDIO | State Vars module computation / boundary audit | `studio/src/studio/state-vars.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 828 | STUDIO | State Window module computation / boundary audit | `studio/src/studio/state-window.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 829 | STUDIO | System Secrets module computation / boundary audit | `studio/src/studio/system-secrets.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 830 | STUDIO | System Surfaces module computation / boundary audit | `studio/src/studio/system-surfaces.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 831 | STUDIO | System Wrap module computation / boundary audit | `studio/src/studio/system-wrap.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 832 | STUDIO | Systems Probes module computation / boundary audit | `studio/src/studio/systems-probes.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 833 | STUDIO | Tool Client module computation / boundary audit | `studio/src/studio/tool-client.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 834 | STUDIO | Tool Result module computation / boundary audit | `studio/src/studio/tool-result.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 835 | STUDIO | Webhooks module computation / boundary audit | `studio/src/studio/webhooks.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 836 | STUDIO | Worker Heartbeat module computation / boundary audit | `studio/src/studio/worker-heartbeat.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | POTENTIAL | TRACKED_PATH | REVIEW_ONLY |
| 837 | STUDIO | Workspace Manifest module computation / boundary audit | `studio/src/workspace-manifest.mjs` | 2 | 2 | 2 | 3 | **33** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 838 | OTHER | Prepare Host Runtime module computation / boundary audit | `tools/packaging/prepare-host-runtime.mjs` | 2 | 2 | 2 | 3 | **33** | DEV_ADMIN | — | L | — | TRACKED_PATH | REVIEW_ONLY |
| 839 | HOST | Browser Session module computation / boundary audit | `apps/vivim-host/control/browser-session.mjs` | 3 | 3 | 3 | 4 | **32** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 840 | HARNESS | Index module computation / boundary audit | `harness/src/providers/index.mjs` | 2 | 3 | 3 | 4 | **30** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 841 | MOBILE | App module computation / boundary audit | `apps/mobile-relay/app.js` | 2 | 2 | 2 | 3 | **29** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 842 | MOBILE | Sw module computation / boundary audit | `apps/mobile-relay/sw.js` | 2 | 2 | 2 | 3 | **29** | FREQUENT | M | — | — | TRACKED_PATH | REVIEW_ONLY |
| 843 | HQ | Bun Test.D module computation / boundary audit | `hq/types/bun-test.d.ts` | 2 | 2 | 2 | 3 | **29** | DEV_ADMIN | — | M | — | TRACKED_PATH | REVIEW_ONLY |
| 844 | TIERFLOW | App module computation / boundary audit | `packages/tierflow/web/app.js` | 2 | 2 | 2 | 3 | **29** | USER_DEFINED | H | — | — | TRACKED_PATH | REVIEW_ONLY |
| 845 | STUDIO | Dom module computation / boundary audit | `studio/src/capture/dom.mjs` | 2 | 2 | 2 | 5 | **29** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | KEEP_JS |
| 846 | STUDIO | Network module computation / boundary audit | `studio/src/capture/network.mjs` | 2 | 2 | 2 | 5 | **29** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | KEEP_JS |
| 847 | STUDIO | Profiles module computation / boundary audit | `studio/bin/commands/profiles.mjs` | 3 | 3 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 848 | STUDIO | Sessions module computation / boundary audit | `studio/bin/commands/sessions.mjs` | 3 | 3 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 849 | STUDIO | Ask module computation / boundary audit | `studio/bin/commands/ask.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 850 | STUDIO | Atoms module computation / boundary audit | `studio/bin/commands/atoms.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 851 | STUDIO | Bridge module computation / boundary audit | `studio/bin/commands/bridge.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 852 | STUDIO | Campaign module computation / boundary audit | `studio/bin/commands/campaign.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 853 | STUDIO | Capture module computation / boundary audit | `studio/bin/commands/capture.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 854 | STUDIO | Chat module computation / boundary audit | `studio/bin/commands/chat.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 855 | STUDIO | Companion module computation / boundary audit | `studio/bin/commands/companion.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 856 | STUDIO | Drive module computation / boundary audit | `studio/bin/commands/drive.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 857 | STUDIO | Library module computation / boundary audit | `studio/bin/commands/library.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 858 | STUDIO | Mirror module computation / boundary audit | `studio/bin/commands/mirror.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 859 | STUDIO | Personal Agent module computation / boundary audit | `studio/bin/commands/personal-agent.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 860 | STUDIO | Platform module computation / boundary audit | `studio/bin/commands/platform.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 861 | STUDIO | Providers module computation / boundary audit | `studio/bin/commands/providers.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 862 | STUDIO | Recipes module computation / boundary audit | `studio/bin/commands/recipes.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 863 | STUDIO | Relay module computation / boundary audit | `studio/bin/commands/relay.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 864 | STUDIO | Run module computation / boundary audit | `studio/bin/commands/run.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 865 | STUDIO | Selftest module computation / boundary audit | `studio/bin/commands/selftest.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 866 | STUDIO | Setup module computation / boundary audit | `studio/bin/commands/setup.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 867 | STUDIO | State module computation / boundary audit | `studio/bin/commands/state.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 868 | STUDIO | System module computation / boundary audit | `studio/bin/commands/system.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 869 | STUDIO | Tools module computation / boundary audit | `studio/bin/commands/tools.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 870 | STUDIO | Wrap module computation / boundary audit | `studio/bin/commands/wrap.mjs` | 3 | 2 | 3 | 3 | **28** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 871 | HARNESS | Cdp module computation / boundary audit | `harness/src/core/cdp.mjs` | 2 | 2 | 2 | 3 | **27** | FREQUENT | H | — | — | TRACKED_PATH | KEEP_JS |
| 872 | HARNESS | Send module computation / boundary audit | `harness/src/core/send.mjs` | 2 | 2 | 2 | 3 | **27** | FREQUENT | H | — | — | TRACKED_PATH | KEEP_JS |
| 873 | STUDIO | Personal Agent Continuity module computation / boundary audit | `studio/src/agents/personal-agent-continuity.mjs` | 2 | 2 | 2 | 3 | **27** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 874 | STUDIO | Frame module computation / boundary audit | `studio/src/studio/atoms/frame.mjs` | 2 | 2 | 2 | 5 | **27** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 875 | STUDIO | Media module computation / boundary audit | `studio/src/studio/atoms/media.mjs` | 2 | 3 | 3 | 3 | **26** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 876 | HOST | Owner Auth module computation / boundary audit | `apps/vivim-host/bootstrap/owner-auth.mjs` | 2 | 2 | 2 | 3 | **23** | FREQUENT | H | — | — | TRACKED_PATH | KEEP_JS |
| 877 | HARNESS | Dev Auto Approve module computation / boundary audit | `harness/src/interface/dev-auto-approve.mjs` | 2 | 2 | 2 | 3 | **23** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 878 | HARNESS | Launch module computation / boundary audit | `harness/src/interface/launch.mjs` | 2 | 2 | 2 | 3 | **23** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 879 | HARNESS | Grant module computation / boundary audit | `harness/src/onboarding/grant.mjs` | 2 | 2 | 2 | 3 | **23** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 880 | HQ | Dispatch Adapter module computation / boundary audit | `hq/lib/torrent/dispatch-adapter.mjs` | 2 | 2 | 2 | 3 | **23** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 881 | HQ | Wave1 Kb Read module computation / boundary audit | `hq/lib/wave1-kb-read.mjs` | 2 | 2 | 2 | 3 | **23** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 882 | TIERFLOW | Executor module computation / boundary audit | `packages/tierflow/src/tier3/executor.ts` | 2 | 2 | 2 | 3 | **23** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 883 | TIERFLOW | Wave3 Read Gateway Fixture module computation / boundary audit | `packages/tierflow/src/visual-runtime/host/wave3-read-gateway-fixture.ts` | 2 | 2 | 2 | 3 | **23** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 884 | STUDIO | Atom Dispatch module computation / boundary audit | `studio/src/studio/atom-dispatch.mjs` | 2 | 2 | 2 | 3 | **23** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 885 | STUDIO | Drive Api module computation / boundary audit | `studio/src/studio/drive-api.mjs` | 2 | 2 | 2 | 3 | **23** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 886 | STUDIO | Harness Api module computation / boundary audit | `studio/src/studio/harness-api.mjs` | 2 | 2 | 2 | 3 | **23** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 887 | STUDIO | Harness Executor Preflight module computation / boundary audit | `studio/src/studio/harness-executor-preflight.mjs` | 2 | 2 | 2 | 3 | **23** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 888 | STUDIO | Lan Pair module computation / boundary audit | `studio/src/studio/lan-pair.mjs` | 2 | 2 | 2 | 3 | **23** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 889 | MOBILE | Browser Device Key module computation / boundary audit | `apps/mobile-relay/browser-device-key.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 890 | MOBILE | Build Canvas module computation / boundary audit | `apps/mobile-relay/build-canvas.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 891 | MOBILE | Build Composer module computation / boundary audit | `apps/mobile-relay/build-composer.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 892 | MOBILE | Device Enrollment module computation / boundary audit | `apps/mobile-relay/device-enrollment.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 893 | MOBILE | Renderer module computation / boundary audit | `apps/mobile-relay/generative-ui/renderer.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 894 | MOBILE | Transport module computation / boundary audit | `apps/mobile-relay/transport.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 895 | MOBILE | Render module computation / boundary audit | `apps/mobile-relay/ux/render.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 896 | HOST | Http Status module computation / boundary audit | `apps/vivim-host/control/http-status.mjs` | 2 | 2 | 2 | 3 | **21** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 897 | HARNESS | Prov module computation / boundary audit | `harness/bin/prov.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 898 | HARNESS | Share Feed module computation / boundary audit | `harness/bin/share-feed.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 899 | HARNESS | Wh Mcp module computation / boundary audit | `harness/bin/wh-mcp.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 900 | HARNESS | Wh module computation / boundary audit | `harness/bin/wh.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 901 | HARNESS | Discover module computation / boundary audit | `harness/scripts/discover.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 902 | HARNESS | Live Smoke module computation / boundary audit | `harness/scripts/live-smoke.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 903 | HARNESS | Signin Check module computation / boundary audit | `harness/scripts/signin-check.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 904 | HQ | Arbor Hq module computation / boundary audit | `hq/bin/arbor-hq.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 905 | HQ | Autonomy module computation / boundary audit | `hq/bin/autonomy.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 906 | HQ | Console module computation / boundary audit | `hq/bin/console.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 907 | HQ | Dogfood Execution module computation / boundary audit | `hq/bin/dogfood-execution.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 908 | HQ | Dogfood module computation / boundary audit | `hq/bin/dogfood.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 909 | HQ | Hq module computation / boundary audit | `hq/bin/hq.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 910 | HQ | Revive module computation / boundary audit | `hq/bin/revive.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 911 | HQ | Spawn module computation / boundary audit | `hq/bin/spawn.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 912 | HQ | Wave1 Lane A module computation / boundary audit | `hq/bin/wave1-lane-a.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 913 | HQ | Domains 1 module computation / boundary audit | `hq/lib/actions/domains-1.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 914 | HQ | Domains 2 module computation / boundary audit | `hq/lib/actions/domains-2.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 915 | HQ | Domains 3 module computation / boundary audit | `hq/lib/actions/domains-3.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 916 | HQ | Domains 4 module computation / boundary audit | `hq/lib/actions/domains-4.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 917 | HQ | Channel Transport module computation / boundary audit | `hq/lib/channel-transport.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 918 | OTHER | W01g R001 Source Closure module computation / boundary audit | `integration/scripts/w01g-r001-source-closure.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | L | — | TRACKED_PATH | KEEP_JS |
| 919 | TIERFLOW | Server module computation / boundary audit | `packages/tierflow/src/server.ts` | 2 | 2 | 2 | 3 | **21** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 920 | TIERFLOW | Dom module computation / boundary audit | `packages/tierflow/src/visual-runtime/render/dom.ts` | 2 | 2 | 2 | 3 | **21** | HIGH_FREQUENCY | H | — | — | TRACKED_PATH | KEEP_JS |
| 921 | TIERFLOW | Dom module computation / boundary audit | `packages/tierflow/web/lib/dom.js` | 2 | 2 | 2 | 3 | **21** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 922 | STUDIO | Args module computation / boundary audit | `studio/bin/lib/args.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 923 | STUDIO | Capabilities module computation / boundary audit | `studio/bin/lib/capabilities.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 924 | STUDIO | Dev Auto Approve module computation / boundary audit | `studio/bin/lib/dev-auto-approve.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 925 | STUDIO | Query module computation / boundary audit | `studio/bin/lib/query.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 926 | STUDIO | Registry module computation / boundary audit | `studio/bin/lib/registry.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 927 | STUDIO | Relay Chat Studio module computation / boundary audit | `studio/bin/lib/relay-chat-studio.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 928 | STUDIO | Ui module computation / boundary audit | `studio/bin/lib/ui.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 929 | STUDIO | Util module computation / boundary audit | `studio/bin/lib/util.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 930 | STUDIO | Relay Chat Mvp module computation / boundary audit | `studio/bin/relay-chat-mvp.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 931 | STUDIO | Studio module computation / boundary audit | `studio/bin/studio.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 932 | STUDIO | Adapter Factory Demo module computation / boundary audit | `studio/scripts/adapter-factory-demo.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 933 | STUDIO | Adapter Fixture Site module computation / boundary audit | `studio/scripts/adapter-fixture-site.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 934 | STUDIO | Ai Chat Report module computation / boundary audit | `studio/scripts/ai-chat-report.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 935 | STUDIO | Always On module computation / boundary audit | `studio/scripts/always-on.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | POTENTIAL | TRACKED_PATH | KEEP_JS |
| 936 | STUDIO | Architecture Inventory module computation / boundary audit | `studio/scripts/architecture-inventory.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 937 | STUDIO | Bench Agent14 Systems module computation / boundary audit | `studio/scripts/bench-agent14-systems.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 938 | STUDIO | Core Local Handoff module computation / boundary audit | `studio/scripts/core-local-handoff.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 939 | STUDIO | E2e module computation / boundary audit | `studio/scripts/e2e.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 940 | STUDIO | Fidelity Survey module computation / boundary audit | `studio/scripts/fidelity-survey.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 941 | STUDIO | Fixture Site module computation / boundary audit | `studio/scripts/fixture-site.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 942 | STUDIO | Gate module computation / boundary audit | `studio/scripts/gate.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 943 | STUDIO | Harness Crash Probe module computation / boundary audit | `studio/scripts/harness-crash-probe.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 944 | STUDIO | Human Check module computation / boundary audit | `studio/scripts/human-check.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 945 | STUDIO | Induce Vs Handmap module computation / boundary audit | `studio/scripts/induce-vs-handmap.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 946 | STUDIO | Live E2e module computation / boundary audit | `studio/scripts/live-e2e.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 947 | STUDIO | Loadlab module computation / boundary audit | `studio/scripts/loadlab.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 948 | STUDIO | Local Verify module computation / boundary audit | `studio/scripts/local-verify.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 949 | STUDIO | Platform Smoke module computation / boundary audit | `studio/scripts/platform-smoke.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 950 | STUDIO | Probe Site module computation / boundary audit | `studio/scripts/probe-site.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 951 | STUDIO | Shape Atlas module computation / boundary audit | `studio/scripts/shape-atlas.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 952 | STUDIO | Transcript Intake module computation / boundary audit | `studio/scripts/transcript-intake.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 953 | STUDIO | Value Oracle Demo module computation / boundary audit | `studio/scripts/value-oracle-demo.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 954 | STUDIO | Launcher Helper module computation / boundary audit | `studio/scripts/windows/launcher-helper.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 955 | STUDIO | Resolve Start Config module computation / boundary audit | `studio/scripts/windows/resolve-start-config.mjs` | 2 | 2 | 2 | 3 | **21** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 956 | STUDIO | Build module computation / boundary audit | `studio/src/families/build.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 957 | STUDIO | Continuity module computation / boundary audit | `studio/src/intel/continuity.mjs` | 2 | 2 | 2 | 3 | **21** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 958 | STUDIO | Server Stdio module computation / boundary audit | `studio/src/mcp/server-stdio.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 959 | STUDIO | Browser Atoms module computation / boundary audit | `studio/src/studio/browser-atoms.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 960 | STUDIO | Cdp Owner Lock module computation / boundary audit | `studio/src/studio/cdp-owner-lock.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 961 | STUDIO | Http Encode module computation / boundary audit | `studio/src/studio/http-encode.mjs` | 2 | 2 | 2 | 3 | **21** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 962 | HOST | Server module computation / boundary audit | `apps/vivim-host/server.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 963 | HARNESS | Server module computation / boundary audit | `harness/src/interface/server.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 964 | HARNESS | Chatgpt module computation / boundary audit | `harness/src/providers/chatgpt.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 965 | HARNESS | Claude module computation / boundary audit | `harness/src/providers/claude.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 966 | HARNESS | Deepseek module computation / boundary audit | `harness/src/providers/deepseek.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 967 | HARNESS | Gemini module computation / boundary audit | `harness/src/providers/gemini.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 968 | HARNESS | Grok module computation / boundary audit | `harness/src/providers/grok.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 969 | HARNESS | Kimi module computation / boundary audit | `harness/src/providers/kimi.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 970 | HARNESS | Qwen module computation / boundary audit | `harness/src/providers/qwen.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 971 | HARNESS | Zai module computation / boundary audit | `harness/src/providers/zai.mjs` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 972 | HQ | Server module computation / boundary audit | `hq/lib/fsf/server.mjs` | 2 | 2 | 2 | 3 | **19** | DEV_ADMIN | — | M | — | TRACKED_PATH | KEEP_JS |
| 973 | TIERFLOW | Http module computation / boundary audit | `packages/tierflow/src/surfaces/http.ts` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 974 | TIERFLOW | Websocket module computation / boundary audit | `packages/tierflow/src/surfaces/websocket.ts` | 2 | 2 | 2 | 3 | **19** | USER_DEFINED | H | — | — | TRACKED_PATH | KEEP_JS |
| 975 | STUDIO | Client module computation / boundary audit | `studio/src/cdp/client.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 976 | STUDIO | Discovery module computation / boundary audit | `studio/src/cdp/discovery.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 977 | STUDIO | Browser Ownership module computation / boundary audit | `studio/src/chrome/browser-ownership.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 978 | STUDIO | Launch module computation / boundary audit | `studio/src/chrome/launch.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 979 | STUDIO | Act module computation / boundary audit | `studio/src/studio/atoms/act.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 980 | STUDIO | Auth module computation / boundary audit | `studio/src/studio/atoms/auth.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 981 | STUDIO | Clipboard module computation / boundary audit | `studio/src/studio/atoms/clipboard.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 982 | STUDIO | Consent module computation / boundary audit | `studio/src/studio/atoms/consent.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 983 | STUDIO | Dom module computation / boundary audit | `studio/src/studio/atoms/dom.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 984 | STUDIO | Download module computation / boundary audit | `studio/src/studio/atoms/download.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 985 | STUDIO | Form module computation / boundary audit | `studio/src/studio/atoms/form.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 986 | STUDIO | Nav module computation / boundary audit | `studio/src/studio/atoms/nav.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 987 | STUDIO | Net module computation / boundary audit | `studio/src/studio/atoms/net.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 988 | STUDIO | Read module computation / boundary audit | `studio/src/studio/atoms/read.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 989 | STUDIO | Safe module computation / boundary audit | `studio/src/studio/atoms/safe.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 990 | STUDIO | Screen module computation / boundary audit | `studio/src/studio/atoms/screen.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 991 | STUDIO | State module computation / boundary audit | `studio/src/studio/atoms/state.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 992 | STUDIO | Table module computation / boundary audit | `studio/src/studio/atoms/table.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 993 | STUDIO | Torrent module computation / boundary audit | `studio/src/studio/atoms/torrent.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 994 | STUDIO | Trace module computation / boundary audit | `studio/src/studio/atoms/trace.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 995 | STUDIO | Type module computation / boundary audit | `studio/src/studio/atoms/type.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 996 | STUDIO | Upload module computation / boundary audit | `studio/src/studio/atoms/upload.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 997 | STUDIO | Wait module computation / boundary audit | `studio/src/studio/atoms/wait.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 998 | STUDIO | Window module computation / boundary audit | `studio/src/studio/atoms/window.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 999 | STUDIO | Http module computation / boundary audit | `studio/src/studio/http.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |
| 1000 | STUDIO | Server module computation / boundary audit | `studio/src/studio/server.mjs` | 2 | 2 | 2 | 3 | **19** | FREQUENT | M | — | — | TRACKED_PATH | KEEP_JS |

## Operational interpretation and gates

1. **Always-on resource pressure:** measure continuous CPU duty, RSS, wakeups and battery before declaring any task always-on. Filename markers are weak evidence.
2. **High-frequency pressure:** benchmark active conversation capture/replay, NLP/command parsing, DCB context selection, graph search and revision transforms at realistic call rates.
3. **Burst pressure:** benchmark full imports, corpus indexing, graph rebuild, joins and compaction independently from average duty cycle.
4. **Dev/admin pressure:** keep HQ indexing/invalidation and wiki compilation workloads separate from personal Host data, permissions and phone state.
5. **Native reuse first:** inspect `native/hqkg/` and `core/programmability/` for existing implementations before writing Rust. Performance parity and correctness include rename/tombstone, ordering, digest, recovery and revocation.
6. **Proof before migration:** for any selected engine record call graph, synthetic fixture, baseline CPU/p95/peak RSS, native proof, IPC/WASM cost, error handling, identical outputs and rollback. I/O, browser sends, Host approvals, personal World durable writes and TierFlow DOM rendering remain owned by existing authorities.

**Source inspections performed:** Git tracked path census; function-name matching across six evidence batches. **Tests / benchmarks:** UNRUN. **Actual cross-repo migration comparison:** NOT RUN. **Runtime acceptance:** NOT CLAIMED.
