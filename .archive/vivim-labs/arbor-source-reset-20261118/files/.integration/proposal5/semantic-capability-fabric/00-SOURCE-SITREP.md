# Semantic Wiki / inline meta-language — actual source sitrep
**Read-only source audit date:** 2026-11-12. These are observed source/interface states, not actual combined runtime tests.

| Layer | Source | State | Observed behavior and gap |
|---|---|---|---|
| v1 inline protocol | `repos-hq/hq/lib/protocol.mjs` donor main `9b922ea0`, same file imported to ARBOR | SOURCE_PRESENT | 9 v1 descriptive tags; `renderAgentProtocol` tells agents >20-line file to start with @hq:context; linter diagnoses missing headers, orphan @hq:see and missing owner on todo. Policy is not every first-party file |
| v2 typed source annotations | `hq/lib/annotation-v2.mjs`, blob `7e94dfbd...` | SOURCE_PRESENT | 9 tags: type, capability, effect, evidence, contract, emits, consumes, action, test. Real-comment lexer, injected registry, source refs; unknown IDs report diagnostics. No independently authenticated source pin in parser |
| indexed source | `hq/lib/indexer.mjs`, blob `4f5cf53c...` | SOURCE_PRESENT | incremental SQLite files/symbols/imports/annotations and inferred consumer analysis. Not a complete declarative capability-passport compiler |
| deterministic wiki | `hq/lib/wiki.mjs`, blob `c85c8fbe...` | SOURCE_PRESENT | local deterministic Markdown/browser pages from index; 2,481 lines at audit. Generated read model, not installed Host action |
| semantic IDs + wiki-program | `semantic-registry.mjs`, `wiki-program.mjs`, `comment-wiki-bridge.mjs` | SOURCE_PRESENT | versioned semantic IDs and source links, owner-reviewed version/rollback for wiki views; Rust @hq:wiki compiler uses a different grammar, not SWC |
| SWC research v0.1 | repos-hq branch `research/semantic-wiki-compiler-20261112` @ `7cb2fadd88c5a08c0ec7e084410e150a5b6e0f3d` | RESEARCH_IMPLEMENTED | `research/semantic-wiki-compiler/semantic-wiki-compiler.mjs` blob `83c2eabd...`, `vivim.semantic-wiki-ir/0.1`; consumes real v2 parser and supplied owner catalog; `read_only_semantic_wiki_projection`, no effect |
| proposed comment semantic/3 | same research branch, `META-CODING-LANGUAGE-SWC-001.md` | DESIGN_ONLY | proposed utterance, frame, slot, constraint, relation, visual, wiki-topic, example, disclosure, migration; not parsed by v2 |
| Labs R010 | vivim-labs branch `lab/nlp-visual-command-ui-20261111` @ `67e76c5b730dcc2cd3ab3919c3ba273a6c260db8` | DESIGN/RESEARCH | expands wiki+deterministic composer; does not accept product/runtime |
| ARBOR S-semantic prep | `work/p5-preg-semantic-wiki-r001` @ `d7e59a389a6ec5278595e317dfa904a801783c5b` | PREPARED_NOT_PROMOTED | byte-identical compiler/parser copy, synthetic IR and inert adapter; not merged as live writer |
| ARBOR PG02 fabric | `work/p5-fa-pg02-semantic-fabric-r001` @ `021864846783d28679b29e563490d4bb100ac079` | PREPARED_NOT_PROMOTED | semantic ABI crosswalk and synthetic tests; Host/real device unrun |
| HVI migration/version intelligence | `.integration/proposal5/hq-version-intelligence/` on ARBOR main @ `f9823a5131e73e7562db5a4ec14e1ec481557d69` | DOCS_MERGED, IMPLEMENTATION_NOT_VERIFIED | multiple immutable source pins and N-way compare work orders; should consume SC-ABI when implemented, not be replaced |

## Verified links
- HQ language: https://github.com/owenservera/repos-hq/blob/research/semantic-wiki-compiler-20261112/research/semantic-wiki-compiler/META-CODING-LANGUAGE-SWC-001.md
- HQ architecture: https://github.com/owenservera/repos-hq/blob/research/semantic-wiki-compiler-20261112/research/semantic-wiki-compiler/ARCHITECTURE-SWC-001.md
- HQ research compiler: https://github.com/owenservera/repos-hq/blob/research/semantic-wiki-compiler-20261112/research/semantic-wiki-compiler/semantic-wiki-compiler.mjs
- Labs R010: https://github.com/owenservera/vivim-labs/blob/lab/nlp-visual-command-ui-20261111/nlp-command-ui-lab/docs/R010-SEMANTIC-WIKI-COMPILER-VISION.md
- ARBOR semantic language baseline: https://github.com/owenservera/ARBOR/blob/work/p5-preg-semantic-wiki-r001/.integration/proposal5/phase2/pre-g/S-semantic/LANGUAGE-BASELINE.md
- HVI normative contract: https://github.com/owenservera/ARBOR/blob/main/.integration/proposal5/hq-version-intelligence/03-NWAY-CONTRACT.md

## Unsolved cross-cutting problem
Today the same capability may appear as a source comment, repo registry, Host action, test title, Labs experiment, Rust graph node and TierFlow visual projection without one compile-time **evidence-backed correlation** across revisions. Free-form textual matching carries risk and expense. Build one opt-in feature membership+lineage ABI; require the coding agent to maintain simple semantic declarations; use existing indexes to *verify and derive* topology. This does not erase the need to inspect old unannotated code or verify runtime.
