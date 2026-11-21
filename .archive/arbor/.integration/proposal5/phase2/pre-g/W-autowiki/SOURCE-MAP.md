# W-AUTOWIKI — SOURCE-MAP
Status: PREPARED_NOT_PROMOTED; source and test preparation only. No runtime, database or display integration.

## ERA 1 — Exact pin set (observed connected GitHub, 2026-11-12)
| Repository / lane | Branch | Git HEAD |
| --- | --- | --- |
| ARBOR baseline | work/p5-pre-g-convergence-r001 | `8b23c73bf450738a76b256bf0fa17ecbf86db591` |
| ARBOR B Host | work/p5-w01-b-host | `78a5105a9c1f1880d658f350e130b1e2632d3532` |
| ARBOR C Studio | work/p5-w01-c-studio | `f950b91685927f37b48d020433aa8d2d3c271419` |
| ARBOR D HQ | work/p5-w01-d-hq | `ad55fe28c2c331841675f308f9199ee4c2add86a` |
| ARBOR E TierFlow | work/p5-w01-e-ui | `1a5cf26ce14ebce4e31d9a4373060ace61852645` |
| ARBOR F contracts | work/p5-w01-f-verify | `be458f3c4cdb7eb38d47b1de2a00d441b7cdc704` |
| repos-hq AutoWiki donor | research/autowiki-screen-mirror-20261112 | `61507b5f9bafd0211d0a556280b367574e676ce1` |
| repos-hq Rock donor | research/rock-core-storage-bounds-20261112 | `0a3c11a6df72434b46931f90b25bf1f93da09f92` |
| vivim-labs R005/R008/R009 | lab/nlp-visual-command-ui-20261111 | `67e76c5b730dcc2cd3ab3919c3ba273a6c260db8` |

All blobs below are SHA-1 Git blob object IDs observed by fetch_file, not source-content SHA-256. W records never infer a missing source from an unrelated branch.

## ERA 2 — Source ownership and readiness
| Classification | Actual branch-relative path | Exact blob SHA | Interpretation |
| --- | --- | --- | --- |
| DESIGN_ONLY | repos-hq `research/autowiki-screen-mirror/PRD-AW-001.md` | `e16aa121230adefd8ce103d0727b4ae2323202b4` | 24+ desired behaviors; not integrated |
| DESIGN_ONLY | repos-hq `research/autowiki-screen-mirror/ARCHITECTURE-AW-001.md` | `b849f3beff9da8ac2ac17fac0856882dbfd075b9` | source-authorized read path specification |
| DESIGN_ONLY | repos-hq `research/autowiki-screen-mirror/IMPLEMENTATION-ORDERS-AW-001.md` | `41bfa3c3ac3cf91d2def809005d4cbe4111abc94` | proposed owner tasks |
| SOURCE_TESTABLE | repos-hq `research/autowiki-screen-mirror/contracts/screen-context.v0.schema.json` | `88911395d0239522b5734c7084a03c5e204046de` | ScreenContext 0.1, no trusted producer |
| SOURCE_TESTABLE | repos-hq `research/autowiki-screen-mirror/contracts/autowiki-page.v0.schema.json` | `e0c44ffdc69ed75fc4fbe7ee6b40cc3f26e31329` | AutoWikiPage 0.1 |
| SOURCE_TESTABLE | repos-hq `research/autowiki-screen-mirror/contracts/autowiki-delta.v0.schema.json` | `3cef303713cba588c6545beeb4eac0636ac36a98` | AutoWikiDelta 0.1 |
| SOURCE_TESTABLE | repos-hq `research/autowiki-screen-mirror/fixtures/chat-screen-mirror.synthetic.json` | `b1f34a4556c9ab39f2366b52deff421257240320` | synthetic example, NOT live current ChatGPT screen |
| IMPLEMENTED_RESEARCH | repos-hq `research/rock-core-storage/storage-policy.mjs` | `4606b90c2635564e74c712e19771c5d244576e3e` | pure advisory planner, does not evict |
| IMPLEMENTED_RESEARCH | repos-hq `research/rock-core-storage/rock-membership.mjs` | `9023bd026a311cadf9f9d6d024562166a3176a3c` | owner-scoped bounded membership planner |
| SOURCE_TESTABLE | repos-hq `research/rock-core-storage/test/storage-policy.test.mjs` | `750592e32c6370a2bc6bb522c743ed49559397e2` | Node tests available; NOT executed in W |
| SOURCE_TESTABLE | repos-hq `research/rock-core-storage/test/rock-membership.test.mjs` | `0b30094cfc1abd6f8f05abe271d9d2b523aacb1c` | Node tests available; NOT executed in W |
| DESIGN_ONLY | vivim-labs `nlp-command-ui-lab/docs/R008-AUTOWIKI-SCREEN-STATE-MIRROR-VISION.md` | `a819245e0c13b4cf7838129340ca5e04c34f5853` | screen-flip vision |
| DESIGN_ONLY | vivim-labs `nlp-command-ui-lab/docs/R009-ROCK-CORE-STORAGE-BOUNDED-INTELLIGENCE.md` | `1c82e411503967364bfdaf25bd702733bf6f2cdc` | bounded storage vision |
| IMPLEMENTED_LAB | vivim-labs `nlp-command-ui-lab/src/context.mjs` | `c0f1a2baa074e49b1327459797f93f8a2a95a0c0` | read-only HQ KB lens from typed text, not trusted screen |
| SOURCE_TESTABLE | vivim-labs `nlp-command-ui-lab/tests/context.test.mjs` | `a7e80537662971b76746005fb0ea8d1538e0eb0d` | missing HQ and local read-only scenarios; UNRUN here |
| SOURCE_TESTABLE | vivim-labs `nlp-command-ui-lab/tests/semantic-lens.test.mjs` | `0b25d1f18f7fa5c1147ae9ac497f38815f1183fa` | negation/intent parser lab; not a permission grant |
| IMPORTED_IMPLEMENTED | ARBOR D `hq/lib/kb.mjs` | `cde5f2f57e9b6fff7651fc6af013ad76fcae32ac` | existing read-only KB, preserve its IDs |
| IMPORTED_IMPLEMENTED | ARBOR D `hq/lib/wiki.mjs` | `c85c8fbe1409273928ea5c52bc2c20f108ac4dce` | persistent wiki writer: must NOT invoke on flip |
| IMPORTED_IMPLEMENTED | ARBOR D `hq/lib/context.mjs` | `2da8285d5b6977df54467cd1cc795a63dd329f64` | source/line/digest bounded context |
| IMPORTED_IMPLEMENTED | ARBOR D `hq/lib/wave1-kb-read.mjs` | `c6bc8a8a961fc49bde0b8da27b3e280cd50bca0a` | bounded read-only semantic export; dev-only |
| IMPORTED_IMPLEMENTED | ARBOR D `hq/lib/wiki-runtime/graph-index.mjs` | `e0ef095b0a3aab6d7c64453f0db7ead947f9deab` | graph read model copies canonical IDs, no mint |
| IMPORTED_IMPLEMENTED | ARBOR B `apps/vivim-host/bootstrap/owner-auth.mjs` | `22d25dfe2484bf0db56b6c2b3970c209a0351c5f` | private owner capability, not a projection grant |
| IMPORTED_IMPLEMENTED | ARBOR B `apps/vivim-host/corpus/host-integration.mjs` | `9c2df8dc742cc88f9f5e0757380137d8492a652c` | grants and personal source receipt boundary; separate read port needed |
| IMPORTED_IMPLEMENTED | ARBOR C `studio/src/capture/conversation-thread.mjs` | `cdffb42b3e2e319f98587e636587103fef8e3d0c` | bounded currently rendered history, complete=false |
| IMPORTED_IMPLEMENTED | ARBOR C `studio/src/capture/conversation-browser-adapter.mjs` | `545f271240ce44fe37a96f2500472e9626f8400c` | exact-tab bound observer, not arbitrary scraping |
| IMPORTED_CONTRACT | ARBOR E `packages/tierflow/src/visual-runtime/knowledge/client.ts` | `9022f9f618eb555bb2367aba671e7fb702da6490` | fixture knowledge client with trusted broker |
| IMPORTED_IMPLEMENTED | ARBOR E `packages/tierflow/src/visual-runtime/knowledge/projection.ts` | `94f91eca18dcb9d853821b2cb22793b1db4f5884` | inert escaped card projection |
| IMPORTED_IMPLEMENTED | ARBOR E `packages/tierflow/src/visual-runtime/knowledge/wave3-feed-projection.ts` | `b8602cf7260f570279f007510cb538c66ec4ee2b` | scoped diff/resnapshot and source notices |
| IMPORTED_CONTRACT | ARBOR F `contracts/interop/v1/types.ts` | `e329eb1b8506938c8fc981b426c51a3582ae03c7` | SourceRef+inert view mirrors; no action authorization |
| IMPORTED_CONTRACT | ARBOR F `contracts/interop/v1/validate.mjs` | `25c0b36b11f2e6f3b003920e14e166a8f74354b7` | read guard vocabulary, not live verification |


### Additional audited donor blob pins (source-only)
| Classification | Source path | Blob SHA | Why inspected |
| --- | --- | --- | --- |
| SOURCE_AUDIT | repos-hq `research/autowiki-screen-mirror/AUTOWIKI-SOURCE-AUDIT.v0.json` | `045034fa0347240e7bf8ecafd7fb89afa87dd595` | comparative source audit, not runtime receipt |
| SOURCE_AUDIT | repos-hq `research/rock-core-storage/ROCK-SOURCE-AUDIT.v0.json` | `0838bebbecaf5bf805e453b222965a320d3e43e2` | real byte inventory gaps |
| CONTRACT | repos-hq `research/rock-core-storage/contracts/rock-storage-policy.v0.schema.json` | `06e1c0bcd1b42442adb4832632ea9ebb1d56dc8f` | policy schema, never grant |
| DESIGN_ONLY | repos-hq `research/rock-core-storage/PRD-RC-001.md` | `dc1cfdee61eb45885adc8b1ea621a6a577d562ac` | protected Rock and physical pressure |
| DESIGN_ONLY | repos-hq `research/rock-core-storage/ARCHITECTURE-RC-001.md` | `dd2fdae7bc4035a18dbaaf62eae7a06414a93d72` | no second storage authority |
| DESIGN_ONLY | vivim-labs `nlp-command-ui-lab/docs/R005-PROGRESSIVE-SPRINT.md` | `e8d8aeb663a8d3828b228ca1114f203a7bb4a6ce` | source-vs-intent/effect boundary |
| IMPLEMENTED_LAB | vivim-labs `nlp-command-ui-lab/src/semantic-lens.mjs` | `d5de11edf693eb67ffed23e1fd26e68240d4420c` | pure lexical UI, not source authority |
| IMPLEMENTED_LAB | vivim-labs `nlp-command-ui-lab/src/compiler.mjs` | `5df077d5f0d5873fbd3eb0a99afbcc5ed7f9031a` | Host result wins, no effects |
| SOURCE_IDENTITY | ARBOR D `hq/lib/kb.mjs` | `cde5f2f57e9b6fff7651fc6af013ad76fcae32ac` | `docNodeId` composes `doc:${project}:${path}`; path markers preserved without rewriting IDs |

## ERA 3 — W-owned slice and missing gates
- W implemented: `mirror.mjs` ephemeral read projection, `negative-tests.mjs` synthetic denial/staleness tests, `run-tests.mjs` Node runnable entrypoint; all within isolated W directory. No donor source copied into live product paths.
- MISSING P0: trusted runtime screen-context producer and session/world grant lease from B, exact tab binding/partial history adapter from C, D per-owner source-authorized KB query and actual four-repo scope mapping, E display/flip-back parity across desktop/PWA, F independent native contract, security and device checks.
- MISSING P1: real Rock physical byte inventories, recovery proof, write-admission/eviction enforcement in actual HQ/Host/TierFlow owners. W stores no cache to evict.
- A live W1-G integrated baseline was not used or modified; all A-F lanes above are frozen **branch sources**, not proof that their files coexist in an integrated runtime. This is **source preparation**, not product acceptance.
- R007 graph/VIEW ALL, generic screenshot scraping/OCR, personal corpus ingestion, permanent wiki writes and command effects are excluded.
