# VISUAL-TRUTH-MATRIX — verified source identity versus lab research
Status: **PREPARED_NOT_PROMOTED**. Audit date: 2026-11-12. All files in this lane are read-side, synthetic, inert. Git identities are pinned in [SOURCE-PINS.json](SOURCE-PINS.json). No product has been edited.

## Source-backed behaviors and graduation decision

| R005 entry (exact export / test) | Observed semantics | Source eligibility | Critical non-claim |
| --- | --- | --- | --- |
| `semantic-lens.mjs:scanLexical(text)`; `semantic-lens.test.mjs` Unicode/quotes | Regex token role hints, non-overlapping original **UTF-16** spans, quote literals; **slices at 1000 characters** | Reuse source-preservingly as opt-in *hint producer* behind bounded source adapter | Not a universal grammar; must not silently lose clauses >1000 or equate regex tokenization with source identity |
| `detectNegation(text)`; negation tests | Recognizes `don't`, `do not`, `never` and punctuation boundary, first negative only | Reuse only to emit explicitly provisional scope/evidence; extend upstream with clause-scope tests before richer claims | First hit is not all negations; never invert to a positive action |
| `projectHints(text)`; source hint test | Known project-name lexical hints | Read-only fallback | A name is not a grounded HQ/Host source reference |
| `commandQuery`, `commandMenu`, `resolveTypedReference`; slash tests | Explicit leading `/` discovery from supplied inventory, namespace-separated, no execution | Source-preserving import behind Host/HQ/TierFlow *read* snapshots | Registered action is not available, authorized, or executed |
| `semanticSummary(text,preview)`; understanding/permission test | Signals, lexical vs Host-planner provenance, always `executable:not_assessed` | Presentation projection only | Candidate/ready understanding is not Host effect grant |
| `meaningDelta(before,after)`; delta tests | Coarse display delta for safety/action/slot/outcome | Presentation-only micro feedback | Not graph-level semantic diff |
| `replaceSuggestion(source,menu,item)`; stale-span test | Exact substring replacement, one-step undo; rejects mismatched query; 1000-char ceiling | Reuse with **additional** input revision and composition CAS guard | Does not provide safe multi-device/caret/IME reconciliation alone |
| `affordances.mjs:sigilFor`, `affordanceFor`, `decorateActions`; sigil tests | Deterministic namespace sigils, icon/line/risk hints | Pure rendering decoration | `liveVerified` not established by source observation; do not inflate unknown to available |
| `compiler.mjs:annotate`, `visualCompile`; compiler tests | Visual preview, Host `refused` and `clarification_required` precedence; fictional contacts; `executionPerformed:false` | Preserve only pure visual/contract behavior; **do not vendor planner/fixture-source I/O into Rust** | Compiled visual preview not Host planner nor native semantic authority |
| `bridge.mjs:collectInventory`, `getContacts`; bridge tests | Local file census and fictional demo contact lookup | Research evidence; replace reads with authentic bounded owner ports after G | Filesystem crawler and fictional identity are not actual runtime data |
| `public/app.js`; R005 sprint record | Quiet/Assisted/X-ray toggles; Tab/arrow/Esc, undo and lexical overlays in loopback demo | UI behavior reference, **not** mobile/device-accepted product renderer | No browser acceptance; do not create competing product composer |
| `scripts/semantic-bench.mjs`; R005 bench | 100 synthetic requests checked for spans, negation and non-effects | Diagnostic baseline only | R005 docs report 28/28 tests and 100/100 probes; **not rerun by this lane** |

Source source-tested = repository contains code + associated source tests; not necessarily independently run in this task. These matrix claims come from the inspected source files and R005 sprint report, not from production execution.

## Design-only versus observed

| Source | Status | What may graduate now |
| --- | --- | --- |
| R006 `R006-100-PREDICTIVE-REALTIME-COMPOSER-CAPABILITIES.md` | **DESIGN_ONLY** 100 proposals; predictive completion, two Tabs, habit learning, prewarm | Only synthetic test scenarios; no implemented Rust/HQ producer coverage claim |
| R007 `R007-TOTAL-INTERPRETATION-VIEW-ALL-AND-HQ-HANDOFF.md` | **DESIGN_ONLY** 10-layer view, 43-facet reported audit, candidate hypergraph, full-coverage manifest | Projection schema, overlapping candidates, negative tests, VIEW ALL *data mask* prototype |
| R010 `R010-SEMANTIC-WIKI-COMPILER-VISION.md` | **DESIGN + separate HQ compiler proof** | Source ID and explanation metadata references only; V never becomes semantic compiler |
| Labs Native I/O parallel event fabric and R001 UI boundary | **DESIGN**, with separately observed native Rust R001 source | Fan-out contract + source/revision guards; no native stream or cross-surface cutover proof |
| `vivim-web-harness/core/programmability/src/user_io.rs` | **IMPLEMENTED SOURCE**: R001 `UserIoCore::apply`, `apply_json`, `InputEvent::{Draft,Tab,Submit}`, `Projection`, four reactor evidence variants | Later owner can adapt current read-only single snapshot; richer word-level multi-producer wire transport **not implemented** |
| ARBOR E TierFlow command IR/compiler and inert renderer/revisions | **IMPORTED SOURCE** frozen in E branch, not in pre-G base | Post-G E mapping of `semanticGraph` to *inert* TierFlow visual elements |
| ARBOR B Host planner/catalog/dispatcher | **IMPORTED SOURCE** frozen in B branch | Host-owned interpretation and risk labels; only dispatcher may admit effects |
| ARBOR D HQ semantic registry/annotation v2 | **IMPORTED SOURCE** frozen in D branch | Owner-confirmed semantic IDs, source links and aliases; nothing grants execution |
| ARBOR F `contracts/interop/v1` | **IMPORTED CONTRACT** frozen in F branch | Reuse `SourceRef`, `InertViewProjection` and risk enum, preserving owner boundaries |

## Exact source pins (blob = Git file identity)

| Source | Path | Blob SHA | Branch commit SHA (prefix) |
| --- | --- | --- | --- |
| ARBOR B frozen | `apps/vivim-host/control/intent-planner.mjs` | `3d1bbcd5ad80ffa6276ea4e5d2b5f0cfbd8cf4ab` | `78a5105a9c1f` |
| ARBOR B Host catalog | `apps/vivim-host/control/action-catalog.v1.json` | `c423d5933c28ca9f534028aeaffd7929f767c269` | `78a5105a9c1f` |
| ARBOR B dispatcher | `apps/vivim-host/control/dispatcher.mjs` | `723865482b8a0f98ab3214b2ec6a83aaea629c43` | `78a5105a9c1f` |
| ARBOR D metadata | `hq/lib/semantic-registry.mjs` | `ec982381c77147119f62e702a162a203df8e7d4e` | `ad55fe28c2c3` |
| ARBOR D annotations | `hq/lib/annotation-v2.mjs` | `7e94dfbd4fbca8ef0fdb9e0610053364ce2c38c8` | `ad55fe28c2c3` |
| ARBOR E command IR | `packages/tierflow/src/visual-runtime/command/ir.ts` | `a6e581476af9d5b091b1939591ad5d5ba8a32a45` | `1a5cf26ce14e` |
| ARBOR E command compile | `packages/tierflow/src/visual-runtime/command/compile.ts` | `4cc5b63bea409171e2b4b0a29398c87907cd35ea` | `1a5cf26ce14e` |
| ARBOR E render | `packages/tierflow/src/visual-runtime/render/wave3-semantics.ts` | `5843f12213a2b82077ea1ffaed084a013c01b123` | `1a5cf26ce14e` |
| ARBOR E revision | `packages/tierflow/src/visual-runtime/revisions/port.ts` | `59cb23a5571a02d5cc5bd719af04cbca8b696cd8` | `1a5cf26ce14e` |
| ARBOR E mobile semantic | `apps/mobile-relay/generative-ui/semantic-projection.mjs` | `0a10271b605df29f067951e655e7233b909b012a` | `1a5cf26ce14e` |
| TierFlow canonical upstream | `src/visual-runtime/command/ir.ts` | `a6e581476af9d5b091b1939591ad5d5ba8a32a45` | `50822f11ee63` |
| ARBOR F interop types | `contracts/interop/v1/types.ts` | `e329eb1b8506938c8fc981b426c51a3582ae03c7` | `be458f3c4cdb` |
| ARBOR F interop schema | `contracts/interop/v1/protocol.schema.json` | `d13c0e6d5065400224bf7a4bbf89aa9a92f4156c` | `be458f3c4cdb` |
| Labs R005 lexical | `nlp-command-ui-lab/src/semantic-lens.mjs` | `d5de11edf693eb67ffed23e1fd26e68240d4420c` | `67e76c5b730d` |
| Labs R005 compiler | `nlp-command-ui-lab/src/compiler.mjs` | `5df077d5f0d5873fbd3eb0a99afbcc5ed7f9031a` | `67e76c5b730d` |
| Labs R005 bridge | `nlp-command-ui-lab/src/bridge.mjs` | `2daeb4edac5c0122ac2752771d214f6ddd23be09` | `67e76c5b730d` |
| Labs R005 affordances | `nlp-command-ui-lab/src/affordances.mjs` | `8f010ef7d0e1ac444e9855055f6f4881fc36c56a` | `67e76c5b730d` |
| Labs R005 tests | `nlp-command-ui-lab/tests/semantic-lens.test.mjs` | `0b25d1f18f7fa5c1147ae9ac497f38815f1183fa` | `67e76c5b730d` |
| Labs R005 bench | `nlp-command-ui-lab/scripts/semantic-bench.mjs` | `f1555e2e42ad0fce376eceb5815d24cc24a9adba` | `67e76c5b730d` |
| Labs R005 record | `nlp-command-ui-lab/docs/R005-PROGRESSIVE-SPRINT.md` | `e8d8aeb663a8d3828b228ca1114f203a7bb4a6ce` | `67e76c5b730d` |
| Labs R006 | `nlp-command-ui-lab/docs/R006-100-PREDICTIVE-REALTIME-COMPOSER-CAPABILITIES.md` | `a8300777bc4676f83eef789fe07984e3302e49f6` | `67e76c5b730d` |
| Labs R007 | `nlp-command-ui-lab/docs/R007-TOTAL-INTERPRETATION-VIEW-ALL-AND-HQ-HANDOFF.md` | `cba4d05b853ae4de0cc69a0caee6fb7692fe867e` | `67e76c5b730d` |
| Labs R010 | `nlp-command-ui-lab/docs/R010-SEMANTIC-WIKI-COMPILER-VISION.md` | `1affe8871bce006342a07fa0ede9446733577b88` | `67e76c5b730d` |
| Labs IO event fabric | `user-io-rust-core/PARALLEL-EVENT-FABRIC-ARCHITECTURE.md` | `54d942eca63cd339adb8d7a959d3badf32afd0c1` | `61d9fc15c551` |
| Labs IO native UI | `user-io-rust-core/rounds/R001-03-NATIVE-OUTPUT-AND-UI-BOUNDARY-PRD.md` | `0cde2b1c648f5529950dc7423ce4e287cd4c6214` | `61d9fc15c551` |
| Labs IO migration | `user-io-rust-core/USER-IO-CONTRACT-AND-MIGRATION.md` | `3928de100a6a98183c1c6e0e8cf9c002497c5bd4` | `61d9fc15c551` |
| Native Rust R001 | `core/programmability/src/user_io.rs` | `d8a53bb2a9216721323790ca96b9517af6528bec` | `fa6dea9576c8` |
| Native Rust exports | `core/programmability/src/lib.rs` | `39a5142908956a1149a7a81a45b9f460e96222db` | `fa6dea9576c8` |

## Explicit unsupported grounding

- The pre-G base branch **does not include** workers A–F; their source is read on isolated, pinned worker refs. Do not import it here.
- B's `intent-planner.mjs` references `../../mobile-relay/control-language.mjs` but that file is **not present at this path on the pinned standalone B branch**. G must resolve cross-lane import topology and run imports before this integration can be considered wired.
- No accepted authoritative producer manifest, actual personal DCB definitions, actor-scope Host permission checks, live World/HQ graph revision, or native stream exists in V.
- No real private corpus; no provider send; no DOM scraping. `PROTOTYPE/fixtures.mjs` contains invented strings, not user conversations.
- Current R001 Rust carries bounded template/project/focus evidence, **not** general multi-clause token spans, real authorization, memory writes or accessible hyperedges.
- The prototype preserves candidate source identities and a shared immutable human expression. Its safety is bounded to pure data transformation; runtime security/privacy/audience enforcement remains future A/B/E/F work.
