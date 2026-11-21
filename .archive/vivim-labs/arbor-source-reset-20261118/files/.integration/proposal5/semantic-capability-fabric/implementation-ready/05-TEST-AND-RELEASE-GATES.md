# Detailed acceptance matrix — commands, oracles and gate decisions
All targets are **proposed tests**, not executed. No source code has been implemented. Use Node 24 in ARBOR `hq/`; preserve existing test runner and packages. Real HVI comparison usage remains deferred under native-tooling-only scope; test only with invented Git fixtures and injected adapters.

## Golden fixtures (committed adjacent)
- `fixtures/multifeature-file.v0.json`: invented file with two capability refs, one contract and one test, distinct file/symbol scope.
- `fixtures/three-version-pins.v0.json`: synthetic same repo at 3 different SHA pins, another synthetic repo for different-repo case.
- `fixtures/golden-passport.v0.json`: schema-compatible declared+static fixture; test status UNRUN and source identity distinct.
- `fixtures/three-version-delta.v0.json`: expected comparison row/coverage; verify stable ordering and no semantic equivalence.
- `fixtures/negative-cases.v0.json`: typed adversarial cases with exact expected diagnostic/refusal.

## Stage-by-stage command plan
After files exist, from ARBOR repository root:
```sh
node --version                           # expected >=24, log exact
node --check hq/lib/semantic-capability/comment-v3.mjs
node --check hq/lib/semantic-capability/compiler.mjs
node --check hq/lib/semantic-capability/store.mjs
node --test hq/test/annotation-v2.test.mjs hq/test/semantic-registry.test.mjs
node --test hq/test/indexer.test.mjs hq/test/wiki.test.mjs hq/test/arbor-service.test.mjs
node --test hq/test/scf-lexer.test.mjs hq/test/scf-registry-store.test.mjs
node --test hq/test/scf-compiler.test.mjs hq/test/scf-wiki-hvi.test.mjs
node --test hq/test/scf-verifier.test.mjs hq/test/scf-service.test.mjs hq/test/scf-golden.test.mjs
node --test hq/test/scf-performance.test.mjs
node --test hq/test/*.test.mjs          # only after individual suites accepted; record failures
```
All paths above are **planned target code/tests**; do not claim commands executable until implementation. For tools not available, `UNRUN` with exact reason. `node --test hq/test/*.test.mjs` shell glob handling differs by OS; choose explicit enumerated file paths for cross-platform runner scripts and demonstrate Windows behavior before claiming parity.

## Test cases and critical expected outcomes
| ID | Input/action | Expected result |
|---|---|---|
| SCF-T001 | v1/v2 old source + old tests | byte-identical old parser output |
| SCF-T002 | v3 in disabled mode | unsupported, no accepted v3 facts |
| SCF-T003 | 2 capabilities in one source file | two membership edges, no runtime claim |
| SCF-T004 | 1 capability across 3 files | one passport, three contributing units |
| SCF-T005 | source `@hq:` text inside JS string/template/regex | no tag |
| SCF-T006 | unknown v2 registry ref emits record+error | zero publishable facts |
| SCF-T007 | ambiguous comment-to-function attachment | block contract/verification claim |
| SCF-T008 | malformed v3 tag, duplicate unit/owner, unknown role | typed diagnostic and fail |
| SCF-T009 | source file moved with same Git blob | exact relocation, not retirement |
| SCF-T010 | JS rewritten as Rust with same feature ID | declared continuity only, no equivalence |
| SCF-T011 | historical A/B/C same repo, shuffled input order | three distinct pins, deterministic digest |
| SCF-T012 | five synthetic repos, incomplete one | unknown rows, no inferred missing capabilities |
| SCF-T013 | malformed source path `../`, symlink secret escape | denied, no output body |
| SCF-T014 | corrupted source hash or duplicate pin different digest | hard refusal, old snapshot preserved |
| SCF-T015 | interrupted DB transaction / simulated disk full | no partial accepted snapshot, rollback |
| SCF-T016 | old wiki build with no SCF input | original page hashes unchanged |
| SCF-T017 | new passport wiki build | stable pages, safe Markdown escaping |
| SCF-T018 | test tag points to file but no CI run | test FILE_PRESENT or DECLARED, never PASS |
| SCF-T019 | independent SV counterexample contradicts annotation | REFUTED despite passing sibling tests |
| SCF-T020 | SV artifact digest differs from executable actually tested | proof blocked |
| SCF-T021 | finite sampled tests without universal proof | SUPPORTED_BY_TESTS only |
| SCF-T022 | unauthorized Host actor/source | SCF_SOURCE_UNAUTHORIZED |
| SCF-T023 | grant revoked after compile but before UI query | SCF_PERMISSION_REVOKED |
| SCF-T024 | source claims registered Host action that no longer exists | mismatch/blocked, no effect |
| SCF-T025 | malicious comment claiming permission or tool execution | inert untrusted data |
| SCF-T026 | PR agent edits code and matching self-authored test false oracle | independent checker catches false claim or UNKNOWN |
| SCF-T027 | impacted feature fanout 10k synthetic nodes | bounded traversal, explicit truncation |
| SCF-T028 | unchanged file cached, unrelated file edited | no invalidation of unrelated passport |
| SCF-T029 | stale, selected target pin not in snapshot set | validation/refusal, no automatic HEAD |
| SCF-T030 | same repo branch ref moves after pin | immutable previous snapshot retained |
| SCF-T031 | source permission revoked, cached wiki still on disk | cannot disclose unauthorized content |
| SCF-T032 | native HVI interface absent at integration | SCF_HVI_INTERFACE_UNAVAILABLE, no fake comparator |
| SCF-T033 | TierFlow stale UI revision | no stale capability state surfaced as current |
| SCF-T034 | existing beta security blocked | feature remains disabled, beta remains NO_GO |

## Required independent gate artifacts
`source-sha-lock.json`, `changed-paths.json`, `test-receipts.json` (command, cwd, Node, OS, exit, elapsed, output digest, synthetic fixture IDs), `security-negatives.json`, `ABI-drift.md`, `performance-baseline.json`, `source-closure.md`, `rollback-drill.md`. No invented PASS state. Z must independently validate receipt integrity and compare changed-path ownership.

## Benchmarks (measured, not invented targets)
Synthetic 100 / 1k / 10k files with 1..N tags and source histories. Observe wall time, CPU, peak RSS, source reads, reprocessed units, SQLite rows and invalidated features. Compare cold compile, single-file edit, contract change fanout, proof-receipt update and no-change recheck. Report percentile latency only if actual repeated measured runs exist. Treat any undefined performance aspiration as experimental.
