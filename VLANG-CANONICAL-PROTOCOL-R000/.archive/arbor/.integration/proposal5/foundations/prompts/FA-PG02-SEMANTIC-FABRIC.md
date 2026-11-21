# ARBOR Proposal 5 — FA-PG02 / Semantic Wiki, Deterministic Language, VIEW ALL and AutoWiki Convergence

**EXECUTE NOW using connected GitHub.** Read `.integration/proposal5/foundations/prompts/00-SHARED-PRE-G-CONTRACT.md`, `FOUNDATIONAL-ARCHITECTURE.md`, `FOUNDATIONAL-PRD.md`, `SOURCE-ATLAS.v1.json`, `NATIVE-MACHINERY-AND-CONTRACTS.md`, `WAVE-MANIFEST.v1.json`.

**Repository:** `owenservera/ARBOR`.
**Create:** `work/p5-fa-pg02-semantic-fabric-r001` from the **current pinned** `work/p5-foundational-architecture-prep-r001` HEAD.
**Exclusive writes:** `.integration/proposal5/foundations/proofs/PG02-semantic-fabric/` ONLY.
**Tasks:** `FA-PG02-01..FA-PG02-08`. **Completion:** `PREPARED_NOT_PROMOTED`; no G/product code changes or actual wiki/personal memory writes.

## FIRST — pin true source and read actual code

Current source refs at prompt authoring; **verify heads again before writing**:

- ARBOR D HQ `work/p5-w01-d-hq@ad55fe28c2c331841675f308f9199ee4c2add86a`: `hq/lib/{annotation-v2,semantic-registry,action-registry,action-router,wiki,wiki-program,comment-wiki-bridge,kb,arbor-service}.mjs`. Important: `comment-wiki-bridge.mjs` implements `hq.comment-program/0.1` and `vivim.command-intent/0.1`; **this is not the same grammar as `hq.annotation/2`**.
- ARBOR B Host `work/p5-w01-b-host@78a5105a9c1f1880d658f350e130b1e2632d3532`: `apps/vivim-host/control/{intent-planner,command-intent,command-program,lexicon,personal-rule-store,dispatcher}.mjs`, `action-catalog.v1.json` (v6, 19 source catalog actions).
- ARBOR E UI `work/p5-w01-e-ui@1a5cf26ce14ebce4e31d9a4373060ace61852645`: `packages/tierflow/src/visual-runtime/command/{ir,compile,crosswalk}.ts`, `apps/mobile-relay/{control-language,generative-ui/semantic-projection}.mjs` and actual render/revision sources.
- ARBOR A pure native `work/p5-w01-a-native@dda546a11294920d6521a0f64ab7ee62ec993c70`: `native/hqkg/src/{engine,typed_dag,scoped_graph}.rs` and `core/programmability/src/lib.rs`. These are pure compute, not execution rights.
- ARBOR F interop `work/p5-w01-f-verify@be458f3c4cdb7eb38d47b1de2a00d441b7cdc704`: `contracts/interop/v1/` type/schema fixtures.
- **Existing S semantic** `work/p5-preg-semantic-wiki-r001@d7e59a389a6ec5278595e317dfa904a801783c5b`: `.integration/proposal5/phase2/pre-g/S-semantic/` actual `inert-adapter.mjs`, `inert-adapter.test.mjs`, `semantic-ir.json`, `semantic-ir.schema.json`, copied `research/semantic-wiki-compiler/semantic-wiki-compiler.mjs`, `COMMAND-WIKI-CROSSWALK.md`, `RUN-LEDGER.json`.
- **Existing V visual** `work/p5-preg-visual-language-r001@7ad5b3c49aefdf1fdd8b212e5b7eea0e7b92338e`: `.integration/proposal5/phase2/pre-g/V-visual/` `PROTOTYPE/projection.mjs`, `realtime-projection.schema.json`, `VISUAL-TRUTH-MATRIX.md`, `OWNER-HANDOFF.md` and actual samples.
- **Existing W AutoWiki** `work/p5-preg-autowiki-r001@827d21d1c33aa8597c600b45179452abc35c7af9`: `.integration/proposal5/phase2/pre-g/W-autowiki/` `mirror.mjs`, `negative-tests.mjs`, `run-tests.mjs`, `CONTRACTS.md`, `STORAGE-GUARDS.md`, `RUN-LEDGER.json`. **Refresh HEAD: this has advanced since a prior source atlas.**
- **Existing N0 native event** `work/p5-preg-n0-io-r001@cde3b6e5521d5b17215ab10088cb8090f8b87e6f`: `.integration/proposal5/phase2/pre-g/N0-io/IO-CONTRACT.json`, `synthetic-proof.mjs`, reference Rust/TS types.
- **Labs actual source** `owenservera/vivim-labs` `lab/nlp-visual-command-ui-20261111`: `nlp-command-ui-lab/src/{semantic-lens,compiler,bridge,context,affordances}.mjs`, R005–R010 docs and tests. *Do not import the research webserver as a new product compiler.*
- **HQ SWC original** `owenservera/repos-hq`, `research/semantic-wiki-compiler-20261112`: `research/semantic-wiki-compiler/{semantic-wiki-compiler.mjs,contracts/semantic-wiki-ir.v0.schema.json,test/semantic-wiki-compiler.test.mjs}`. S may already contain byte-identical source; prove before suggesting import.

## Mission — converge one **meaning model**, not the interpreters/owners

1. **Semantic language inventory:** distinguish HQ `hq.annotation/2`, HQ old source comments, Rust `@hq:wiki` comment-program, HQ `hq.wiki-program/1`, SWC `vivim.semantic-wiki-ir/0.1`, Host Omega/NLCL action grammar, Host command DAG, E typed visual command IR, V visual projection schema, W AutoWiki and N0 event envelope. Extract real names/fields/refs/producers, not invented conceptual equivalents.
2. **Compiler truth:** compare copied S compiler against original HQ SWC at exact source file blob; determine which source verification is caller-supplied vs authenticated by actual HQ walker. Produce exact `SemanticWikiIR`→HQ existing KB/wiki read projection crosswalk. No comment or wiki node creates Host executable handler.
3. **Owner action binding:** map semantic `@hq:action` versioned ID vs B catalog exact action ID, arg/risk/readiness and `planText` original-text validation. Known read-only action may be a candidate; unknown, owner mismatch, untested or stale remains inert. Do not hardcode a fictional `full update` action.
4. **Single visual trace:** compare S semantic frames, V `projection.mjs`, E TierFlow `visual-runtime`, N0 events, F `SourceRef`; propose **minimal** typed source/actor/world/catalog/input revision intersection, UTF-16 spans, producer states `ran|denied|skipped_budget|stale|unavailable|failed`, evidence and readiness axes. Preserve original text/caret/IME and privacy across draft updates.
5. **Parallel I/O and stale guard:** one input revision fans into independent pure source/wiki/grammar/decision/context producers, and only matches from current source/world/policy revision may be reduced to the view. Rust remains compute owner; Host remains effect owner. No invented always-on "super-router" or independent graph.
6. **Screen mirror:** map W `mirror.mjs` trusted ScreenContext and source refs into actual D HQ service read plus B personal grants. Show current project first but retain provenance/freshness; no wiki Markdown generated on every focus and no raw personal chat copied to development index.
7. **Predictive 2-Tab proof:** design a *source-backed read-only* `full [project] [focus]` example from a **real registered source/query** if available; if not, show missing capability and expected owner requirements. No semantic "completeness" claims for unsupported verbs.
8. **Lab/ARBOR duplicate ledger:** classify R005–R010 pieces that are already in S/V/W/N0 and those not present; avoid a second language compiler, action router, UI renderer or wikistore. Record exact source gaps rather than a re-import roadmap.

### One bounded executable staging proof

Implement **if source-compatible** an inert, data-only `semantic-visual-crosswalk.mjs` + focused Node test within your owned `proofs/PG02-semantic-fabric/` using real S, V and W typed structures or a strict subset. Required assertions:
- typed action annotation/ref provenance preserved; unknown/ambiguous mapping fails;
- current text/source/world revisions required for a visual trace;
- stale or wrong-owner producer/ScreenContext result **refused or omitted explicitly** (no denied-node existence leaks);
- `effectPerformed:false`, `executable:false`, no Host dispatch;
- negated `don't send` cannot be represented as an allowed provider-send;
- wiki source freshness reported independently of intent and Host approval;
- exposed UTF-16 span offsets never split astral Unicode at selection and don't mutate original text.

**Important:** do not fabricate real source permissions or success receipts in synthetic fixtures. If a dependency cannot be imported from the worker branch (S/V/W are isolated branches), write a portable *test-only* adapter and document exactly how to run it against read-only source checkouts; mark Node integration **UNRUN** if unexecuted.

## Required output (all within exclusive subtree)

- `README.md`; `SOURCE-DELTA.json` with semantic S/V/W/N0/D/B/E current branch/actual blob, owner and disposition; `SEMANTIC-ABI-CROSSWALK.json`; `CAPABILITY-COMMAND-CROSSWALK.json` (owner action and allowed source statuses); `PROOF-PLAN.md`; `NEGATIVE-ACCEPTANCE.md`; `OWNER-HANDOFF.md` (≤5 tasks); `RUN-LEDGER.json`.
- Optional `semantic-visual-crosswalk.mjs`, `test/*.test.mjs`, `fixtures/*.json` if executed or honestly `UNRUN`.
- Clear acceptance split: **source comment parser / machine wiki / local phrase grammar / user display / actual Host action** are five different gates.

## Completion / stop

Commit only `proofs/PG02-semantic-fabric/`, verify 0 out-of-scope files, state actual Node/Bun/visual tests **versus UNRUN**, report exact head, `PREPARED_NOT_PROMOTED`, and STOP. G original integration remains entirely independent.
