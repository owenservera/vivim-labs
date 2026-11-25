# Native intelligence intake R001 — 2026-11-25

Research only. Extend incumbent NLCL/VLANG/HQKG/VEC/Studio/AutoWiki; no new production parser, graph, executor, scheduler or registry. ARBOR source pin: [a91ebbf0](https://github.com/owenservera/ARBOR/tree/a91ebbf0dad49389959ceee0d631b5d744c46acd). Existing [claims](../../../../ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md) and #33 remain authoritative.

## Pinned upstream source intake
Each entry is a source pointer, not a vendored dependency. Licenses below are GitHub metadata hints; inspect exact license files and dependency licenses before copying/admission. Do not copy private data.

| Project / immutable source | Candidate mechanism | Destination | Disposition |
|---|---|---|---|
| [Stitch](https://github.com/mlb2251/stitch/tree/350804b7b35807c78bd21c313785ae5152ae2985) (MIT) | Corpus-guided abstraction, argument holes, compression | VEC/native_program | First extraction: typed anti-unification, bounded and inert |
| [egglog](https://github.com/egraphs-good/egglog/tree/d89bc58adc9b9a87a8f571c5b9ad75d686d226e6) (MIT) | Relational fixpoints + equality saturation | HQKG/VEC | Compare pure, scoped rules against current typed DAG |
| [egg](https://github.com/egraphs-good/egg/tree/73975c9819efb4af2f3c6c1b44fa53f8c5742574) (MIT) | Cost extraction, rewrite equivalence | VEC | Inspect extract.rs; only pure expressions, never reorder effects |
| [Salsa](https://github.com/salsa-rs/salsa/tree/8b2c16b2e35724509d7ef8ff5a65ba4071569834) (Apache-2.0 metadata) | Dependency tracking and revision invalidation | HQKG/AutoWiki | Compare existing invalidate.rs, graph_change_set.rs |
| [Datafrog](https://github.com/rust-lang/datafrog/tree/116493370b21c888cc4b3d498a03a535ba2428c0) (Apache-2.0 metadata) | Semi-naive relational inference | HQKG | Compare bounded closure |
| [Ascent](https://github.com/s-arash/ascent/tree/e52c84b4419c56976413de1cbf15d56c8c136a5c) (MIT) | Rust-native Datalog rules | HQKG | Alternative to Datafrog, not a second graph |
| [FST](https://github.com/BurntSushi/fst/tree/5907b4739793b3d5d7061eaa3f85274e09769d6a) (Unlicense metadata) | Compact lexicon and prefix search | NLCL | Benchmark catalog lexicon; no change to grant semantics |
| [Aho-Corasick](https://github.com/BurntSushi/aho-corasick/tree/6c0abf5681bfc30bb9d8f7f52b68a350b436fffa) (Unlicense metadata) | Multi-pattern matching | NLCL | Candidate span generation, no implicit intent |
| [ICU4X](https://github.com/unicode-org/icu4x/tree/a14c711d82c2dcffb96dd138e6e09643d221a00d) (license review needed) | Locale-aware text processing | NLCL | English/Spanish/Catalan segment/normalize tests |
| [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation/tree/2368a49833e4b6dd0e933a8485dd96599e72c870) (license review needed) | Grapheme and word boundaries | NLCL/GPUI | Preserve source spans and quoted literals |
| [Differential Dataflow](https://github.com/TimelyDataflow/differential-dataflow/tree/f9ccd506f45f0802bebee17c806bcbba19c221d5) (MIT metadata) | Incremental joins and retractions | HQKG | Scale-only comparison after lightweight baseline |

## Other literature and follow-up candidates
- [DreamCoder](https://arxiv.org/abs/2006.08381) — program-library learning; borrow symbolic abstraction, not neural inference.
- Stitch paper — official [POPL listing](https://popl23.sigplan.org/details/POPL-2023-popl-research-papers/41/Top-Down-Synthesis-For-Library-Learning).
- [Egglog paper](https://doi.org/10.1145/3591239) — equality saturation plus Datalog.
- [SLOG](https://aclanthology.org/2023.emnlp-main.194/) — structural generalization; inspect dataset license before ingest.
- [Grammatical Framework](https://www.grammaticalframework.org/) — abstract/concrete syntax split; research only.
- [Neurosymbolic NLP survey](https://doi.org/10.3389/frai.2026.1797587) — research reference; not evidence of ARBOR quality.
- [Chumsky](https://docs.rs/chumsky/latest/chumsky/) — parser recovery; old zesterer/chumsky repository reports archived, locate canonical maintenance before adoption.
- Further candidates: Logos, pest, rust-analyzer, Tantivy, Roaring, petgraph, Z3, tree-sitter, weighted finite-state transducers, chart parsing, feature unification, anti-unification, typed HTN planning, truth maintenance. Unpinned; no dependency approval.

## Extraction sequence
1. Inspect Stitch pattern_args.rs/compression.rs and egg extract.rs (done, exact pins above). Implement independently authored typed sequence anti-unification, not a copy of their source.
2. Compare against exact-match memoization on real approved read-only program traces from the existing typed DAG; preserve scope, revisions and capability signatures. No private traces in public Labs. Unit checks are not live acceptance.
3. Compare semi-naive fixed-point and incremental invalidation with incumbent HQKG, including deletion/revocation and graph partitioning.
4. Benchmark lexicon candidate generation and multilingual source-span normalization; reuse control_language/catalog.
5. Build a holdout harness using real authorized first-party commands and unseen compositions; report exact source, correctness, abstention, latency, CPU/RAM, zero inference calls. No synthetic promotion.
6. Offer candidates to the current core-IP owners through the existing claim/receipt protocol; no production edits in this lab.

## First Rust experiment
See typed_abstraction.rs. Compile in isolation: rustc --edition=2021 --test typed_abstraction.rs -o typed_abstraction_test && ./typed_abstraction_test. It is a bounded, pure research kernel; no grants, execution, persistence, or external effects. No claims of native production integration or upstream whole-repo vendoring.
