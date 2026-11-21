# SEMANTIC-SOURCE-MAP — ARBOR Proposal 5 / S-semantic

**Verdict: PREPARED_NOT_PROMOTED.** Inspected as source; no runtime, release, authorisation, registry or wiki-writer changes. This is an **inert, synthetic** candidate.

## Exact source revisions

| Owner | Resolved immutable commit | Relationship |
| --- | --- | --- |
| ARBOR pre-G base | `8b23c73bf450738a76b256bf0fa17ecbf86db591` | isolated branch ancestry; no A–G writes |
| repos-hq SWC research | `7cb2fadd88c5a08c0ec7e084410e150a5b6e0f3d` | research compiler and current HQ module blobs |
| vivim-labs R005/R010 | `67e76c5b730dcc2cd3ab3919c3ba273a6c260db8` | lab visual prototype and vision |
| ARBOR D HQ | `ad55fe28c2c331841675f308f9199ee4c2add86a` | independent HQ source import (donor `9b922ea049dbadc94e504b5b9432743995c020ca`) |
| ARBOR B Host | `78a5105a9c1f1880d658f350e130b1e2632d3532` | exclusive catalog/intent/dispatcher authority |
| ARBOR F verification | `be458f3c4cdb7eb38d47b1de2a00d441b7cdc704` | inert interop ABI, tests; **not** a Host runtime |
| ARBOR E UI | `1a5cf26ce14ebce4e31d9a4373060ace61852645` | TierFlow UI IR, separate from semantic compiler |
| Proposal 5 Node0/beta | ARBOR pre-G `04-NODE0-SECURITY-AND-INTERACTION.md` blob `d911b9d60d5de0141069b8a6d34d05f6cc8352cc`; `05-FULL-BETA-RELEASE-CONTRACT.md` blob `5f2c185e973b68b268b006d0120541e3b75c1d01` | pure input/projection, Host effects only |

## Inspected source map

Statuses: **EXISTING** (implemented code at source), **SOURCE_IMPORTED** (physically present on isolated W1 owner branch), **RESEARCH_IMPLEMENTED** (runnable research implementation, not accepted product), **DESIGN_ONLY**, **ADAPTER_REQUIRED**, **BLOCKED** (acceptance/proof missing). Multiple classifications can apply in separate locations. Blob IDs are Git blobs, *not* execution evidence.

| Module / exact path | Blob SHA | Classification and exact boundary |
| --- | --- | --- |
| HQ `hq/lib/annotation-v2.mjs` | `7e94dfbd4fbca8ef0fdb9e0610053364ce2c38c8` | EXISTING, SOURCE_IMPORTED in D and **byte-identical portable copy here**. Pure `parseAnnotationV2`, 9 v2 typed tags, comment-aware lexer, schema `hq.annotation/2`, error diagnostics; caller must refuse error-marked records |
| HQ `hq/lib/extract.mjs` | `2b0696542b0027f474b57c5692f939f326d24ae4` | EXISTING, SOURCE_IMPORTED in D; symbol/import and v1 comment extraction, **not** SWC's imported parser |
| HQ `hq/lib/semantic-registry.mjs` | `ec982381c77147119f62e702a162a203df8e7d4e` | EXISTING, SOURCE_IMPORTED in D; semantic versioned IDs, owner promotion and aliases; ADAPTER_REQUIRED for trusted read-only `lookup`; not a second registry |
| HQ `hq/lib/wiki.mjs` | `c85c8fbe1409273928ea5c52bc2c20f108ac4dce` | EXISTING, SOURCE_IMPORTED in D; deterministic incremental **writer/materializer**, **not imported here** |
| HQ `hq/lib/wiki-program.mjs` | `97cde5d6dbb14fc421bfa5eddb11236e9b71c745` | EXISTING, SOURCE_IMPORTED in D; owner-confirmed preview/config activation, NOT auto-activated from comments |
| HQ `hq/lib/comment-wiki-bridge.mjs` | `996b0f75362a66b823d95b321754f2ff17275325` | EXISTING, SOURCE_IMPORTED in D; `hq.comment-program/0.1` validate/preview path; **distinct** from `vivim.semantic-wiki-ir/0.1` |
| HQ `hq/lib/action-registry.mjs` | `275ccefd76f69f365d13f7f7bb21cb91f0228369` | EXISTING, SOURCE_IMPORTED in D; `wired/declared` HQ dev taxonomy; not Host catalog risk/readiness |
| HQ `hq/lib/action-router.mjs` | `f7911158a18d73029850013a76fc91207b69ab51` | EXISTING, SOURCE_IMPORTED in D; exact-token weighted simple scorer, may return ranked matches, **not** Host admission |
| HQ `research/comment-program-rust/src/lib.rs` | `2e1aa1bd9bac2a07d4dc39716f4600fdb399838a` | RESEARCH_IMPLEMENTED; Rust `@hq:wiki` compiler produces inert wiki-program config; ADAPTER_REQUIRED for semantic candidate; Cargo acceptance BLOCKED/UNRUN here |
| HQ `research/semantic-wiki-compiler/semantic-wiki-compiler.mjs` | `83c2eab7921c84dd55f0503518e7fc7e4a466657` | RESEARCH_IMPLEMENTED, **byte-identical portable copy here**. Imports only `../../hq/lib/annotation-v2.mjs` (relative topology preserved). `compileSemanticWiki` emits inert IR |
| HQ `research/semantic-wiki-compiler/contracts/semantic-wiki-ir.v0.schema.json` | `a05a98cb354f73f78ddb1906590503e759ab699b` | RESEARCH_IMPLEMENTED, **byte-identical `semantic-ir.schema.json`**; JSON Schema 2020-12; no modifications |
| HQ `research/semantic-wiki-compiler/test/semantic-wiki-compiler.test.mjs` | `d92f10a21b12a0a5c145e4e20497edaa1fdb2c3f` | RESEARCH_IMPLEMENTED, **byte-identical portable tests**; local Node re-execution UNRUN |
| HQ SWC architecture, PRD and v3 language proposal | `ccfd7bd2fb92e048cf2799c68bf078c421641e39`, `0c3e2c1aa727434e5004575ee0806fd6c19e8011`, `817c8691dc130f76a0b30cc0674d3ebc4830baf8` | DESIGN_ONLY for `@hq:semantic/3`, curated lexicon, hot grammar and source→UI integration; no v3 parser is installed |
| Labs `nlp-command-ui-lab/src/semantic-lens.mjs` | `d5de11edf693eb67ffed23e1fd26e68240d4420c` | RESEARCH_IMPLEMENTED R005 lexical/negation/quote preview, not Host parser |
| Labs `nlp-command-ui-lab/src/compiler.mjs` | `5df077d5f0d5873fbd3eb0a99afbcc5ed7f9031a` | RESEARCH_IMPLEMENTED R005 visual-only prototype. `tests/semantic-lens.test.mjs` `0b25d1f18f7fa5c1147ae9ac497f38815f1183fa` inspected; UNRUN here |
| Labs R005 / R010 docs | `e8d8aeb663a8d3828b228ca1114f203a7bb4a6ce`, `1affe8871bce006342a07fa0ede9446733577b88` | R005 research tests claimed 28/28 and synthetic benchmark 100/100 **in donor report**, not rerun here; R010 DESIGN_ONLY expanded semantic compiler vision |
| B `apps/vivim-host/control/action-catalog.v1.json` | `c423d5933c28ca9f534028aeaffd7929f767c269` | SOURCE_IMPORTED in B. `vivim.control-action-catalog/1` version **6**, **19 source-declared actions**. Full text SHA-256 `57a665e245fe40ffa6701f6986414dbc0359ea8435fb234005c5fe22f7be5398`; B owns every action |
| B `apps/vivim-host/control/intent-planner.mjs` / `command-intent.mjs` | `3d1bbcd5ad80ffa6276ea4e5d2b5f0cfbd8cf4ab` / `52828ce955027b9e083a8afa5625cb0dbc1b730f` | SOURCE_IMPORTED in B. Host NLCL original-text planner; `vivim.command-intent/0.1`; separate from HQ SWC |
| E `packages/tierflow/src/ui/types.ts` | `32d869d1528ff2fa2e42fee9ca091e5544d32eb6` | SOURCE_IMPORTED in E. UI schema v1 objects/relationships/`UiCommand`, **not semantic language grammar** |
| F `contracts/interop/v1/protocol.schema.json` | `d13c0e6d5065400224bf7a4bbf89aa9a92f4156c` | SOURCE_IMPORTED / ARBOR-authored F inert `arbor.interop/1` source/evidence/revision projection; ADAPTER_REQUIRED, not new semantic or execution owner |

## Actual import topology and portability

1. SWC's only runtime import is `../../hq/lib/annotation-v2.mjs`. Port layout here reproduces exactly `research/semantic-wiki-compiler/semantic-wiki-compiler.mjs` → `hq/lib/annotation-v2.mjs`; both Git blobs equal donor files. Node builtins are only used by the **separate local adapter/test**, not by SWC.
2. `parseAnnotationV2` imports **nothing**, while `extract.mjs`, semantic-registry, wiki/bridge/program and Host planner have their own dependency/DB/authority closures. Copying those closures into S would create a competing runtime and is rejected.
3. Sample IR is compiled from `fixtures/status.example.mjs` with actual v2 source lines 2–6 and synthetic registered IDs. Its Host action metadata comes from an **observed B catalog action**, but it is **not** evidence that the synthetic comment is present in B. See `fixtures/EXAMPLE-SOURCE.md`.
4. **Important schema gap:** SWC v0 records a single 1-based annotation `line`, not columns/end offsets, and the `sourceId` and `contentHash` are caller-supplied. The local `inert-adapter.mjs` verifies hash agreement with the supplied source bytes and refuses unsupported v3 mentions, but it cannot establish repository authenticity or real Host acceptance. No schema edits have been made to hide that gap.
5. Do not claim that SWC's acceptance of a registry `lookup` implies owner promotion; its parser treats `deprecated` as a warning. Production must require **current, owner-pinned** semantic IDs and a trusted complete catalog snapshot.
