# Lane D — Wave 1 Round 2 execution handoff

**Status:** SAFE IMPLEMENTATION TESTED AND READY FOR INTEGRATOR REVIEW; **NATIVE A+B+C+E→HQ WIKI INTEGRATION BLOCKED**. No Wave 2 implementation/design. No existing writer, journal, source index or shared integration baseline changed.

## Executed (D-only, new files)

- **D2-01:** `contracts/validate-inputs.mjs`, actual-source schema inventory: explicit native A/B/C producer vs E consumer mismatch. `validateEGraph` inspects E's published HQKGX1 shapes and `source_commit`, `content_sha256`, `evidence_class`, `evidence` without applying Round 1's incompatible field aliases.
- **D2-02:** `lineage/traverse.mjs` recognizes E's **code → compiled-wiki-program → B semantic → C command** relationships by exact E relation and hashed graph IDs, optionally observes explicit KB links. No text co-occurrence or synthetic "VERIFIED" promotion.
- **D2-03:** `lineage/provenance.mjs`, `lineage/states.mjs`: distinct source-root Git commit (SHA-1), file bytes (SHA-256), status/semantic meaning, handler source, independent runtime/device observations (all false until independently verified), and effect permission (always false). Source anchors are *claimed/pinned*, not independently re-read file-byte attestations.
- **D2-04:** `quality/score.mjs`, `quality/defect-schema.json`: per-domain numerator/denominator, zero/unknown `NOT_MEASURED`, explicit scope and critical blockers; no misleading universal average score.
- **D2-06:** `quality/links.mjs`, `quality/privacy.mjs`: offline relative Markdown links/anchors, case collisions, traversal, malformed encodings and bounded known-pattern privacy diagnostics; not a full Markdown parser or complete DLP proof.
- **D2-07:** `test/meaning-regression.test.mjs`, fixture README: 35 explicitly synthetic negative cases plus five NL examples treated as inert/unproven, not a re-test of Lane C/Harness negation execution.
- **D2-11 spec and synthetic boundary checks:** `contracts/product-view.md`, `contracts/visual-intent-read-model.md`, `quality/product.mjs`; bounded allowlist and no inherited permission. No product deployment or Wave 3 UI writer.
- **Additional D-side read-only integration preparation:** `quality/page-manifest.mjs`, `bin/inspect-readonly.mjs` with synthetic tests. Accepts a *claimed* HQ-owned page manifest only as byte consistency, never as independent graph/runtime proof. `contracts/wiki-read-view.md` is a PROPOSAL, not an activated HQ writer interface.
- **D2-12 spec only:** `evidence/wiki-placement-input.md` explicitly defers native Rust/SQLite benchmarks and all writer migrations.

## Exact executed checks and evidence

From this directory: `node --test wiki-oracle.test.mjs test/*.test.mjs` after preserving published Round 1 source unmodified: **exit 0, 108 passed, 0 failed**, Node.js v22.16.0. 19 were Round 1 synthetic tests; 89 are new Round 2 isolated/synthetic tests. Run `node --check` over every new and legacy .mjs: exit 0. Raw logs `evidence/all-node-tests.log` and machine status `evidence/ROUND-2-RESULT.json`.

All tests are synthetic or structural. The legacy source and original handoff are untouched. No actual A/B/C owner DB snapshot, E Rust .hqj, native Cargo execution or generated integrated HQ wiki was accessible. No source root coverage denominator, independent runtime proof or Rust/SQLite performance measurements were obtained. The `bin/inspect-readonly.mjs` runner is inert and always returns `BLOCKED_NEEDS_NATIVE_RUST_HQ_WIKI_F_WITNESS` even for a coherent synthetic graph; it does not create pages.

## Actual integration dependencies and blockers

1. **A/B/C→E crosswalk**: A/B/C producer schema differences remain an upstream ownership issue; Lane F's landed-source review found no documented converter. Lane D will not synthesize or modify E's canonical graph data.
2. **Scope/reset**: owner-machine scoped old-run archive/index receipt and multi-root completeness, actual root SHAs, source byte digests must be furnished by A and checked by F.
3. **Canonical KB**: accepted/proposed B records and exact source evidence must be real, privacy-filtered and independently reconciled, not a detached annotation or synthetic linked KB.
4. **Registered CLI/NL**: C must provide a genuine operation with precise status and handler, plus independent negation/quote/clause corpus; graph/wiki may document candidates but never authorize effects.
5. **E Rust**: native Cargo import + dedicated DEV journal + where/what/talks-to/gap query, no-op, source change/deletion, corruption/collision/restart transcript, Rust comment compiler proof; E owns journal; current additive importer deletion is unresolved.
6. **HQ wiki owner**: must authorize a read-only *integrated* graph input to its existing buildWiki writer; no evidence of this active contract. D **cannot** independently generate replacement wiki pages or alter the writer while master baseline is live.
7. **F** independently reruns source+Rust+wiki and tests cross-domain truth; D’s own scores/claimed manifest NEVER self-certify.

## Native integration recipe (NOT RUN)

After Master Integration and owner approval, in a **dedicated DEV-only directory**:

```bash
# From owner-approved repos-hq root, using the canonical scoped A/B/C feeds:
node research/rust-knowledge-graph/integration/wave1-lane-e/assemble.mjs --scope DEV/scope.json --a DEV/a.json --b DEV/b.json --c DEV/c.json --compiled DEV/compiled-array.json --out DEV/wave1.jsonl --crosswalk DEV/crosswalk.json
cargo test --manifest-path research/rust-knowledge-graph/Cargo.toml --features importer
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --features importer --bin hqkg-import -- DEV/wave1.hqj DEV/wave1.jsonl
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --features importer --bin hqkg-wave1-query -- DEV/wave1.hqj where a:KNOWN_REAL_SOURCE_ID
# Only after existing HQ owner installs an accepted read-only graph view:
bun hq/bin/hq.mjs wiki --out DEV/generated-hq-wiki --force --json
# From lane-d folder: inspect ACTUAL published E exchange + HQ-built output (inert):
node bin/inspect-readonly.mjs --exchange DEV/wave1.jsonl --wiki-dir DEV/generated-hq-wiki
# Independently run Lane F acceptance and preserve raw command exits / graph + page digests.
```

**Do not run that script against any live/master-owned index or wiki destination.** D2-05/08/09/10/12 remain deferred until owner's master handoff and writer permission. The command recipe is not a claim that native source contracts currently align.

## Existing owners and safety

- A: scoped index/source evidence; B: HQ KB and semantic SQLite; C: CLI/Host command authority and NL safety; E: one HQKG importer/journal/read queries; F: independent verification; HQ wiki owner: sole Markdown/HTML writer and wiki-program config/revision writer; TierFlow: UI document; Host: consent/effects.
- No PRs, branches, Actions, force pushes, Playwright/Puppeteer, provider APIs, unsafe reset, Host effects or second semantic/wiki/journal writer.
- No modifications to `wiki-oracle.mjs`, `wiki-oracle.test.mjs`, original `HANDOFF.md`, or files outside D's owned lane. The master integrator may continue to consume its original baseline; new code stays opt-in.

**Next owner:** master integration owner/F decides after real E bridge/run and HQ writer authorization whether to invoke this Round 2 read-only diagnostic. No Wave 2 design yet.
