# Spherical Naming Lab

A standalone, dependency-free Rust research experiment for escaping predictable naming tropes by comparing independent candidate lists against a broad knowledge catalogue. It has no network access, model API dependency, installation hook, or production Harness authority.

## Corpus and statistical model

- `data/anchors.tsv`: 100 curated disciplines, each with 10 concept anchors.
- `data/lenses.txt`: 10 research perspectives per anchor.
- `data/catalogue.tsv`: the materialized 10,000 distinct discipline/anchor/lens research areas, stable IDs K00001–K10000.
- `data/semantic-lexicon.tsv`: editable, inspectable semantic channels; shared terms connect otherwise dissimilar spellings.
- These are generated research contexts, **not** 10,000 independently recognized academic disciplines.
- Candidate directions combine name, functional meaning and a small role contribution. Knowledge-area directions combine anchor, discipline and lens. Sparse semantic channels and 256 signed hashed lexical features are L2 normalized. This is an explicit heuristic, **not** a calibrated LLM embedding or an objective measurement of meaning.
- Angular geodesic distance is acos(clamp(dot(a,b)/(|a||b|), -1, 1)), in degrees; normalized distance is degrees/180. Smaller is closer under this model.
- The nearest match is exact over all 10,000 directions. Random alternatives sample a top-K shortlist of distinct discipline/anchor combinations so the ten lenses do not dominate randomization.
- The radial band selects by angle. A nonzero kappa weights finite candidates by exp(kappa*cos(theta)); this is a discrete directional kernel inspired by the von Mises–Fisher density, not a continuous vMF sampler.
- The pair percentile is the empirical percentile among all eligible same-role cross-list combinations before band filtering, not a p-value. The stats command reports mean resultant length and its complement, directional variance. The random command samples uniformly from distinct unordered area pairs conditional on a band, with an explicit bounded rejection budget.
- Fixed inputs, data, algorithm, seed and settings reproduce selections on the same numerical environment. Floating-point ties or platform differences can affect near-identical rankings.

## Build and use

From this directory, with Rust installed (1.70+):

```sh
cargo test --offline --locked
cargo run --release --offline -- validate
cargo run --release --offline -- catalogue generated.tsv
cargo run --release --offline -- match examples/list-a.tsv 42 20
cargo run --release --offline -- pair examples/list-a.tsv examples/list-b.tsv 42 7 20
cargo run --release --offline -- pair examples/list-a.tsv examples/list-b.tsv 27 3 50 0 180 0
cargo run --release --offline -- random 27 10 20 100
cargo run --release --offline -- stats examples/list-a.tsv
```

The output is UTF-8 TSV, suitable for a person, another LLM, spreadsheet or later local research pipeline. Use a UTF-8 capable shell when redirecting stdout. The built-in catalogue is reproducible with the catalogue command; validate compares it byte-for-byte after line-ending normalization.

## Human + LLM naming workflow

1. Ask LLM A to make a list of names and precise functional meanings, without looking at LLM B's list.
2. Ask LLM B independently for the same role keys. The input is UTF-8 TSV, header `role<TAB>name<TAB>meaning`. Multiple different names per role are allowed.
3. Run match separately for each list: retain the globally closest research area and a seed-dependent sampled alternative, plus their distance.
4. Run pair with both lists: distinct pairs are sampled without replacement, matched on role (case-insensitive). It reports distances between names, their nearest knowledge areas, and their sampled knowledge areas, plus all area IDs, anchors and meanings.
5. Vary seed, top-K, angle band and kappa. Low kappa and wider bands encourage exploration; higher kappa favors similar directions within the allowed band. The pair band applies to name-name distance; the match band applies to candidate-area alternatives. An empty band produces an explicit error, not an invented result.
6. Discuss results rather than automatically treating a distance as good or bad. Use the original role, the paired anchors, the human's taste and language constraints to formulate a new round. Submit revised lists and repeat.

The included lists are authored demonstration fixtures, **not** evidence of independent model generation. No LLM call is made by this engine. A separate user or LLM can create its own list and request randomized pairs by invoking the CLI.

## Record feedback

Copy a real pair_id from the pair output:

```sh
cargo run --release --offline -- feedback feedback.local.tsv PAIR_ID user refine Ostium "Functional fit; avoid generic tech wording"
cargo run --release --offline -- feedback feedback.local.tsv PAIR_ID llm keep Anastomosis "Interesting biological analogy"
```

Feedback is a local append-only TSV; append from one process at a time. It is not automatically trained into the feature model. Use it to inform the next independent list and, if appropriate, review proposed edits to the public semantic lexicon. Keep private feedback outside the public repository.

## Research boundaries

Distances reflect the authored feature representation, not intrinsic novelty, cultural suitability, trademark clearance or the actual quality of a name. No legal or branding claim is made. The 10,000 rows are a structured, reproducible initial knowledge universe; vocabulary and research perspectives can be expanded in a later version with explicit dataset provenance. Optional real embeddings and independent human calibration are future research, not current capabilities.
