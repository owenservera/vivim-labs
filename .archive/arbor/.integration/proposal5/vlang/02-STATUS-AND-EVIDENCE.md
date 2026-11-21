# Four-era maturity inventory — do not confuse design with running software

These are **maturity classifications**, not chronological periods. Machine source is `machine/component-register.json`. A source file present in main is not automatically integrated or device-tested. No executable tests were run in this strategic documentation turn.

## IMPLEMENTED — concrete source exists (not newly verified by tests)
- HQ v1 `hq/lib/protocol.mjs`: annotation policy/linter and nine descriptive tags.
- HQ v2 `hq/lib/annotation-v2.mjs`: nine typed tags, real-comment source parsing and registry diagnosis. `@hq:semantic/3` is NOT accepted here.
- HQ `semantic-registry.mjs`: stable typed IDs and review lifecycle.
- HQ `indexer.mjs`, `schema.mjs`, `arbor-service.mjs`: incremental development indexing, SQLite and bounded authorized source read.
- HQ `wiki.mjs`, `wiki-program.mjs`, `comment-wiki-bridge.mjs`: existing Wiki projection, owner-confirmed program configuration and separately bounded Rust-comment bridge.
- Research SWC v0.1 and Pre-G/PG02 crosswalk: *source/research/preparation existed on other branches*, not promoted as SCF product. These must not be silently counted as deployed.
All the above need actual current-branch runner and owner integration checks for runtime/device claims.

## PRE-DEV PREPPED — near-code designs, fixtures and handoffs, not implementations
- Draft [PR #13](https://github.com/owenservera/ARBOR/pull/13): SCF + SV blueprint, 44-document kit, 32 target source actions, 15 worker prompts, 34 planned acceptance scenarios, 5 synthetic fixture files; planned tests recorded as UNRUN.
- Draft [PR #19](https://github.com/owenservera/ARBOR/pull/19): umbrella Semantic Engineering & Verification Program, role boundaries, long-horizon backlog and R002 convergence.
- SCF agent coding protocol, SC-ABI, source/owner rules, verifier contracts, red-team negatives, HVI optional adapter and rollback plans are **specified**, not installed.
- HVI native-tooling-only governing scope is **merged policy**; HVI implementation/acceptance is separately governed and cannot be inferred from repo docs or parallel worker commits.

## DISCUSSED BUT NOT IMPLEMENTED — actual product gaps
- V3 opt-in typed tags and parser integration; source/symbol attachment; new reviewed registry kinds.
- SCF signed/pinned semantic facts, incremental invalidation, capability passports and controlled SQLite migrations.
- Independent Semantic Verifier code, trusted oracle/spec approval and anti-weakening controls; bound proof receipts and checked explanations.
- Existing Wiki/HVI enrichment from accepted SC-ABI, agent native discoverability/diagnostics, Host source-grant read and TierFlow inspector.
- Complete independent Z/serial G tests, realistic rollback drill, revocation/remote UI/device security acceptance.

## VISION — future outcomes, not release commitments
- Self-describing files, capability-first Wiki and browsable topology of the whole owned codebase.
- N-way architecture time machine; reliable migration/parity explanations from independently checked lineage.
- Automatic change impact, precise context bundles for coding agents and provenance-preserving code reviews.
- VIVIM screen semantic mirror with permission-scoped truth, live receipt freshness and safe Unknown.
- Generalized proof-carrying SDK, cross-language semantics and model-based verification for bounded pure programs.

## Numerical snapshot
Components in machine register: 30. By four-era class: 12 implemented/source-research entries; 5 pre-dev prepared; 9 discussed but not implemented; 4 vision. These are inventory row counts, **not completion percentages**, product functionality scores or test totals. Detailed maturity is provided separately by `evidence_state`.

## Confidence and tests
Observed directly in this review: ARBOR main file presence and selected source content, both draft branch SHAs, existing HVI scope and observed HVI branch heads. Imported from earlier SCF audit, not reverified now: repos-hq SWC branch, Labs R010, Pre-G/PG02 historical pins and their reported tests. **Tests executed this SITREP: zero.** Node24 tests, synthetic HVI fixtures, SV negative proof, permissions, rollback and beta security remain UNRUN for this review.
