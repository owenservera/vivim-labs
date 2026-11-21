# Wave 1 Lane F — Round 2 implementation and independent evidence report

**Status: PREPARATORY IMPLEMENTATION COMPLETED; FULL INTEGRATION BLOCKED.** 2026-11-07. This is not an accepted Rust graph, product build, migration, or Wave 2 plan.

## Real code/test accomplishment

Implemented a disjoint, no-effect F-owned verifier package (`round2/`) with source/receipt attestation, strict HQKGX1 frame validation, rust-query-output comparison, schema/identity and D↔E provenance checks, synthetic compiler and deterministic NL corpora, coverage ledger, wiki differential, placement-measurement gate, DEV→PRODUCT boundary checks and pinned recipe policy. All tests are local Node, no side-effect executor.

Actual local Node v22.16.0: 140 new Round 2 `node:test` tests, including 60 synthetic compiler safety vectors, 50 HQKGX1 digest/metadata mutation cases, 120 synthetic NL utterances inspected by corpus builder, plus focused parser, source, coverage, query and evidence assertions. 12 existing Lane F regression tests also pass. Total independently executed here: **152/152 tests passing**, zero failures. Syntax-check Node modules exit 0. The aggregate verifier executed with no local native inputs, exited **2/BLOCKED**, produced seven machine findings. These are all *verifier* observations, **not** actual Rust/HQ native acceptance.

Synthetic parser benchmark: 512 fake source nodes, 145,959 JSONL bytes, five warmups and fifty parsing measurements, Node RSS sampled; stored in `evidence/checker-benchmark.json`. These timings test the F parser only. They cannot support a Rust-vs-SQLite recommendation; all canonical owner placement decisions remain provisional/deferred where unmeasured.

## Assessment against the 16 ordered objectives

| Objective | Implemented in this session | Evidence class / remaining gate |
|---|---|---|
| F2-01 | proof schema/hashes/source path and synthetic-as-native refusal | NODE_SYNTHETIC_PASSED; native custody missing |
| F2-02 | allowlisted test recipe validation; no subprocess execution | NODE_SYNTHETIC_PASSED; trusted native executable isolation unrun |
| F2-03 | HQKGX1 frame/provenance/crosswalk parser and Rust query attestor | NODE_SYNTHETIC_PASSED; native journal/query missing |
| F2-04 | A/B/C→E mismatch and E→D metadata/domain audit | STATIC_SOURCE + NODE_SYNTHETIC; actual converted feeds missing |
| F2-05 | inventory-derived design target extractor; actual 1,234 inventory source pinned | SOURCE_SEEN; complete per-target authority review still missing |
| F2-06 | 60 synthetic compiler cases and 3 grammar-families proof policy | NODE_SYNTHETIC_PASSED; Rust/native source-compiled chain missing |
| F2-07 | 120 synthetic NL examples + independent case-grade refusal | NODE_SYNTHETIC_PASSED; real Host/HQ/C observed corpus missing |
| F2-08 | coverage denominator and severity-ranked dedup ledger | NODE_SYNTHETIC_PASSED; native source/KB denominator missing |
| F2-09 | independent D→E wiki provenance/digest/chain comparator | NODE_SYNTHETIC_PASSED; original HQ-generated pages missing |
| F2-10 | measurement gate and reproducible Node checker microbenchmark | NODE_SYNTHETIC_PASSED; native Rust/SQLite p50/p95 missing |
| F2-11 | DEV/PRODUCT leakage/static privacy/source-identity checks | NODE_SYNTHETIC_PASSED; device/private KB boundary proof missing |
| F2-12 | 50 HQKGX1 mutation/refusal cases | NODE_SYNTHETIC_PASSED; source/journal mutation survival unrun |
| F2-13 | No owner-machine reset/index execution | BLOCKED_OWNER_MACHINE |
| F2-14 | No independent actual graph→wiki traversal | BLOCKED_MASTER_HANDOFF_AND_CARGO |
| F2-15 | No persisted journal lifecycle/restart/delete native execution | BLOCKED_MASTER_HANDOFF_AND_CARGO |
| F2-16 | This honest intermediate report; no final live acceptance | BLOCKED_MASTER_HANDOFF; Wave 2 INPUT DOSSIER only |

## Source contract risks that remain real

At observed Repos HQ main `04c84b1f8cc59d8aa516541b952e2a97f4ccc600`, A/B/C source declarations still have distinct schemas from E; the previous source audit pinned A/B/C/E git blobs. An in-memory schema match would not prove the producer→consumer conversion; verify A absolute source root to E repo-relative scope, B numeric project ID and source hash/semantic IDs, C `semanticId:null` and handler runtime states. E→D needs consistent `source_path/source_commit/content_sha256` vs `source_repo/source_sha` and `evidence_class` vs `link_status/evidence_ref`, plus domain-kind classification. These are not self-fixed by this read-only lane.

Original HQ command routing can score negation as its positive command; C's new guard is inert and separate. Rust E's import is additive; deletion/rename may leave stale graph nodes until owner-approved retirement semantics and native lifecycle tests establish otherwise. Do not treat absence of observed IPC/write edges as proof of absence in the product.

## Native checkpoint dependencies — strictly separate from this execution

1. **A:** actual index/run archive or isolated proof, restore-test/durable KB survival, all roots/eligibility/exclusions and source hashes, indexing intent revision.
2. **B:** private read-only canonical SQLite evidence, accepted vs proposed identities, exact link source hashes, omitted private records and truncations.
3. **C:** actual full-checkout registry export, CLI syntax status, native NO_MATCH/negation/clause/quote corpus, handler-source vs runtime receipts.
4. **E/Master:** pinned converted A/B/C feeds, compiled real Rust comment output, one source→B→C chain, native importer Cargo test result, HQKGX1 digest, dedicated DEV `.hqj`, where/what/talks-to/gap outputs, no-op/change/delete/rename/restart/invalid/collision/secret refusal raw receipts.
5. **D:** original HQ wiki regenerated from E's graph/provenance, D oracle report and actual wiki page crosslinks.
6. **F after Master final:** independent native query/journal replay, custody and performance on owner machine, causal actual-vs-target/coverage score, native Rust/SQLite placement recommendation, verified graph/wiki parity and an input-only Wave 2 dossier. **Master Integration report not available in the source heads inspected for this run.**

## Ownership / safe publication and baseline preservation

Publish only NEW `lane-f/round2/**` files. Do not alter original F `acceptance.mjs`, `cross-lane-preflight.mjs`, original `HANDOFF.md`, E importer/query/journal, any A/B/C/D file or shared Wiki/SQLite/Host/Studio/TierFlow owner. No destructive reset, effect execution, Playwright/Puppeteer, provider APIs, branch/PR/Actions/force push. Do not allow a native-run policy to execute a command before Master and owner approval. No production milestone is implied by this verifier package.
