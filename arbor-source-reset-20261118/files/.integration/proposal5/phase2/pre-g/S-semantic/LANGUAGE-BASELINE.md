# LANGUAGE-BASELINE — five distinct languages, no substitution

| Layer | Source and grammar identity | What it can actually do at inspected source | It cannot replace |
| --- | --- | --- | --- |
| HQ comment v1/v2 and simple scorer | `protocol.mjs` PROTOCOL_VERSION 1: explanatory 9 tags; `annotation-v2.mjs` `hq.annotation/2`: 9 typed tags; `action-router.mjs`: exact token weights **object 3, verb 2, domain 2, purpose 1** | parse real comments, consult semantic IDs, deterministically rank existing HQ actions, return `NO_MATCH` at zero score | typed natural-language slot/negation planner, Host executor or wiki program |
| Host NLCL + intent | B `apps/vivim-host/control/intent-planner.mjs`, `command-intent.mjs`, `apps/mobile-relay/control-language.mjs`, catalog `vivim.control-action-catalog/1` version **6** and `vivim.command-intent/0.1` | interpret original owner text against exact actions/language slots/risks/readiness; produces Host plans and refusal/clarification; B dispatcher still owns effects | HQ code comment compiler, generic internal wiki or a second effect writer |
| TierFlow visual UI IR | E `packages/tierflow/src/ui/types.ts`, `UI_SCHEMA_VERSION=1`, `UiCommand`, object/layer/flow/relationship, world revisions | describe/manipulate UI world as typed data, with separate revision guard/publisher | user intent grammar, semantic action ID owner or Host grant |
| HQ research SWC v0.1 | `research/semantic-wiki-compiler/semantic-wiki-compiler.mjs` → `vivim.semantic-wiki-ir/0.1`, consumes actual `hq.annotation/2` | compile type/capability/effect/contract/evidence/action links from source comments and **explicit** B catalog binding into read-only IR; no execution | Host NLCL, existing `hq/lib/wiki.mjs` writer, current semantic registry, Rust runtime |
| Incremental semantic language (proposal) | `META-CODING-LANGUAGE-SWC-001.md`, opt-in proposed `@hq:semantic/3`; Labs R010 vision | **DESIGN_ONLY** curated utterance/frame/slot/negation/typed operator/lexicon/source-aware UI and Rust hot grammar | automatically installed v3 tags, broad open-ended LLM intent understanding, authorisation or an automatic 90% success claim |

## Related, but separate

- Rust `research/comment-program-rust/` compiles its **different** `@hq:wiki` grammar to `hq.comment-program/0.1` and `hq.wiki-program/1`. It must not be treated as a native implementation of SWC v0.1.
- `hq/lib/comment-wiki-bridge.mjs` validates the Rust wiki-program configuration against current semantic/action registry. `hq/lib/wiki-program.mjs` owns data-only preview and *owner-confirmed* activation, not source comments.
- Labs R005 `src/semantic-lens.mjs` / `src/compiler.mjs` produce `vivim.labs.visual-interpretation/1` lexical/negation/quoted-word previews with Host refusal precedence. Labs' 28/28 tests and 100/100 synthetic probes are donor claims, not ARBOR device acceptance.
- F `arbor.interop/1` provides source/revision/evidence mirrors, not a replacement semantic IR. Node0 beta minimum accepts **inert draft** and **Host-exclusive committed actions**, not simultaneous command compilers.

## Version negotiation / safe migration

1. Keep v1 and v2 semantics unchanged; v2 unknown/deprecated IDs need a stricter owner-backed acceptance check before an IR is promoted.
2. SWC v0.1 schema stays byte-identical to HQ (`semantic-ir.schema.json`); document absent end columns, immutable Git origin proof, and catalog-signature fields rather than silently adding unvalidated keys.
3. v3 is an **opt-in, owner-reviewed additive grammar** after negative corpus tests and an explicit compatibility/version rejection path; no reinterpreting existing comments with new semantics.
4. Host must recompute exact user text, catalog/world revisions and actor context. Neither a high score, predicted slot, UI wire nor a semantic comment proves intent or authorisation.
5. Measure eligible task frequencies and negative safety/held-out cases before any claim that a deterministic language covers most actions. A compiler that is source-backed is not automatically a proven user-language engine.
