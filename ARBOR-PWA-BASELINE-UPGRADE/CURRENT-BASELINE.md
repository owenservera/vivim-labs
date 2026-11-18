# Pinned source reality
ARBOR current main for R002: [7c1b01630a806c0ac701bdc21e3f4968fec91097](https://github.com/owenservera/ARBOR/commit/7c1b01630a806c0ac701bdc21e3f4968fec91097).
Original preparation baseline: [c6a2edd4ba6411d404fb7552abd8e103b7888ffc](https://github.com/owenservera/ARBOR/commit/c6a2edd4ba6411d404fb7552abd8e103b7888ffc).
Planning source review: [4c95251a709e17811e08fdebd4348fcc1828187d](https://github.com/owenservera/ARBOR/commit/4c95251a709e17811e08fdebd4348fcc1828187d). Comparison showed one intervening commit affecting desktop chat and HQ context/test modules, not reviewed PWA visual sources. Recheck HEAD before edits. No ARBOR checkout/browser/Host tests run in this planning session.

## Executable source anchors
| Path | Actual role / limitation |
|---|---|
| apps/mobile-relay/index.html; app.js | hardcoded shell, seven tabs, shared dock, renderTabs/goTab, mountBuild/mountCompose, Wiki and media |
| apps/mobile-relay/ux/model.mjs; ux/render.mjs | actual Today/World/Me models/rendering; source-backed cards and actions |
| apps/mobile-relay/control-chat.mjs; composer-assist.mjs; control-language.mjs | actual typed plan preview, NL hints, routing; don't create another interpreter |
| apps/mobile-relay/vec/ui-taxonomy/catalog.mjs; registry.mjs | 417 kinds, 34 families, 9 domains; 11 source mappings per legacy surface/document schema (heading shares text native type); 9 inspector-only and 6 world-shape-only mappings |
| apps/mobile-relay/generative-ui/catalog.mjs; compiler.mjs | 10 native visual types, strict proposal/surface validation; 64 nodes, 32 KiB, depth 7, 20 operations |
| apps/mobile-relay/generative-ui/renderer.mjs | TierFlow plan plus safe DOM rendering; Build currently only stack/grid/text/card when tierflow=true; no silent fallback |
| packages/tierflow/src/visual-runtime/document/schema.ts; validate.ts; import-legacy.ts | existing UI document, 64 nodes and bounded views/bindings, legacy import explicitly THEME_NOT_MIGRATED |
| packages/tierflow/src/visual-runtime/render/dom.ts | existing render plan and trace, semantic layout, includes ten types in source; prove parity rather than assume |
| packages/vec-ui-engines/src/catalog.mjs; registry.mjs; compiler.mjs; validators.mjs; tokens.mjs | 37 families, 461 capabilities; applicability/validation separate from installed=false and MISSING_RENDERER_ADAPTER; typed token alias support |
| apps/mobile-relay/build-composer.mjs | deterministic VEC UI adapter, reviewed global color background/accent bridge via compileUiConfiguration -> set-theme; local preview |
| apps/mobile-relay/visual-theme.mjs; theme-plan.mjs; visual-bootstrap.mjs; bootstrap-phone.mjs | separate theme presets and shell CSS projection/cache |
| apps/vivim-host/public/vivim-tokens.css; host.css; mobile-first.css | existing token/font styles; no independent new design system |
| apps/mobile-relay/build-canvas.mjs; generative-ui/phone-surface.mjs; host-revision-guard.mjs | A3 verified-receipt/readback predicates, compensating single-operation undo, TierFlow preview, authenticated blank draft; do not rewrite |
| apps/vivim-host/direct-phone.mjs; bootstrap/command-worker.mjs; bootstrap/service.mjs; ledger.mjs | validated proposal transport -> claimed command -> existing SQLite revision publisher with base CAS and readback; prior Direct field-loss fix |
| apps/vivim-host/public/pages/chat.js; wiki.js | existing desktop views; retain as actual consumers or migrate via compatibility adapter, no duplicated product functionality |

## Adjacent work
A1 live Wiki, A2 typed Wiki edit, A3 real Build, VLANG upgrade (semantic indexing, exact approval, live slots), D4 Wiki knowledge and native/HQ lanes, R4 unified NL, R5 product fixture cleanup. Read latest actual callers. Do not repeat or undo their work. Current baseline is historical evidence, not a promise all branches or local devices pass.
## Source links
[ARBOR pinned source tree](https://github.com/owenservera/ARBOR/tree/c6a2edd4ba6411d404fb7552abd8e103b7888ffc) · [VLANG package](https://github.com/owenservera/vivim-labs/tree/main/ARBOR-VLANG-UPGRADE)
Catalog snapshot is generated from declared source types/IDs only; no owner data, private credentials or executable ARBOR source copied into Labs.

## R002 — source advance (7c1b01630a80)
See [R002-SOURCE-ADVANCE.md](R002-SOURCE-ADVANCE.md): ten changed paths over two commits; new Wiki exact approval, receipt-derived desktop template and corresponding UI/tests. Visual runtime/catalog unchanged; live UI tap and bootstrap receipt gaps remain. Original preparation and previous planning review are historical evidence; current pinned head 7c1b01630a806c0ac701bdc21e3f4968fec91097. Runtime tests UNRUN.


R003 — Current inspected ARBOR main: 7031105ce179151adc9be3fe482b7f4cbb68d55c. U1–U6 committed. U6 live source-card renderer and exact anchor retrieval integrated into both owner surfaces. Existing composer-assist already provides two-TAB and scoped/latest-input draft assistance. See R003-U7-CONVERGENCE.md for source reality and runtime limitations. Earlier pins historical; tests UNRUN.
