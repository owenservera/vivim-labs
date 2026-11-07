# Semantic kernel: ten executable optimization algorithms

**Status:** implemented pure algorithms, 33 conformance cases (30 original + 3 defect-regression cases) and measured on Windows/Bun 1.4.2 as of 2026-11-02. Experimental library: NOT YET integrated into production HQ CLI/indexing, KB writer, wiki, TierFlow or VIVIM Host.

**Source:** [hq/lib/semantic-optimizers.mjs](../../../hq/lib/semantic-optimizers.mjs)
**Tests:** [hq/test/semantic-optimizers.test.mjs](../../../hq/test/semantic-optimizers.test.mjs)
**Benchmark:** [scripts/bench-semantic-optimizers.mjs](../../../scripts/bench-semantic-optimizers.mjs)

Run synthetic benchmark locally: bun scripts/bench-semantic-optimizers.mjs --size 1000 --runs 5

The microbenchmark fixture is synthetic, writes no tracked baseline files, and reports per-algorithm median/p95 timing. The harness does not report memory, so no heap figure is claimed here. Numbers in "Measured on the owner machine" below are real Windows/Bun 1.4.2 runs.

Run tests locally: bun test hq/test/semantic-optimizers.test.mjs

## Catalog

| ID | Export | Method | Cost | Consumer |
|---|---|---|---|---|
| A01 | buildAliasIndex | Unicode NFKC normalization, stable collision-preserving index | O(n log n) | HQ schema/lexicon |
| A02 | resolveAlias | exact-first, optional prefix, ambiguity is not guessed away | O(n + m log m) | HQ/VIVIM Ω adapters |
| A03 | rankEvidence | bounded fixed weighted score, redaction, stable tie sorting | O(n log n) | KB/context |
| A04 | packContext | greedy marginal topical coverage per token | O(k*n*t) | context candidate strategy |
| A05 | reconcileClaims | structurally canonical value grouping; preserve contradictions | O(n log n) | KB evidence intake |
| A06 | traceImpact | cycle-safe reverse BFS over dependencies | O(V+E), plus sort | source impact |
| A07 | orderActionPlan | Kahn dependency sort with cycle/missing-dependency blocking | O(V²+E) worst case | typed plan preview |
| A08 | allocateTasks | prioritized constrained agent-capacity assignment **proposals** | O(T*A log A) | HQ coordination |
| A09 | coalesceObservations | opt-in, barrier-separated last-revision-wins | O(n) | HQ bus observer path |
| A10 | detectDrift | expected-vs-actual hash, version, existence comparison | O(n log n) | schema/source integrity |

n = input rows, k = selected items, t = topics, V/E = graph nodes/edges, T = tasks, A = agents. Context packing is a heuristic and is not guaranteed globally optimal; ranking scores are descriptive, not sources of authority. Inputs must already respect upstream sensitivity filtering.

## Architectural contract

- Pure: no timers, filesystem, DB writes, network, Chrome, send, permission or approval side effects.
- Source comment @hq: annotations remain HQ protocol v1; these functions do not add a new parser or redefine VIVIM Ω symbolic/NLCL interpretation.
- HQ+KB is the canonical shared knowledge/semantic kernel, but does not own runtime provider effects.
- A08 only suggests assignments: it never opens a task claim or routes an executable command.
- A09 coalesces only explicit **coalescible: true** observations of three known kinds within uninterrupted segments. Messages requiring ACK, approval, receipts and effect events are barriers, not compacted. Oversized input blocks.
- A07 only orders **descriptions** of intended actions, never executes an action, grants a scope or retries a send.
- Existing HQ context knapsack, graph query, realtime coordinator and read-only wiki remain the runtime. Their differential tests must pass before integrating any alternative optimization.

## Proposed wiring order

1. HQ registry/parser v2: A01/A02 alias indexing for registered semantic IDs and A10 source-link drift diagnostics. Preserve existing parser and add opt-in versioned grammar tests.
2. HQ knowledge: A03/A05 under provenance and contradiction storage. Differential-test A04/A06 against existing HQ context and graph implementations.
3. HQ coordination: A08 task proposal UI and A09 observer-event compaction through the existing sync bus. Bus owner must preserve durable claims, messages, ACK and receipts.
4. Wiki V1: A07 ordering for typed configuration plan preview; bounded scoped writer stays separate from the read-only wiki console.
5. TierFlow/VIVIM: versioned adapters, independent tests and owner-authorized Host/Studio/Android effect path. No TierFlow dependency in VIVIM.

## Conformance, measurements and limitations

The test file contains 33 deterministic cases covering the ten algorithms, ambiguity, collisions, cycle blocking, capacity checks, source drift, contradictions, secret paths, receipt barriers, plus three regression cases for the defects fixed on 2026-11-02 (selection order invariance, secret-path superset, malformed-input tolerance).

Measured on the owner machine (Windows, Bun 1.4.2, synthetic fixture, size 1000, runs 5, median / p95 in ms):

| Algorithm | median ms | p95 ms |
| --- | ---: | ---: |
| A01 indexAliases | 2.193 | 2.803 |
| A02 resolveAlias | 0.205 | 0.451 |
| A03 rankEvidence | 2.899 | 15.570 |
| A04 packContext | 75.402 | 84.352 |
| A05 reconcileClaims | 4.312 | 5.452 |
| A06 traceImpact | 2.542 | 22.779 |
| A07 orderActionPlan | 3.468 | 3.801 |
| A08 allocateTasks | 14.254 | 32.173 |
| A09 coalesceObservations | 3.006 | 17.592 |
| A10 detectDrift | 1.188 | 2.009 |

A04 is the outlier: the greedy loop is O(k·n·t) and re-scans the candidate list per pick.

### Comparison against the existing HQ implementation

| ID | Existing HQ counterpart | Verdict | Reason |
| --- | --- | --- | --- |
| A01 buildAliasIndex | none (taxonomy vocabulary maps terms, not semantic ids) | not comparable | different input domain; no HQ alias index exists |
| A02 resolveAlias | none (search/find are FTS5 queries) | not comparable | HQ has no alias-to-target resolver to compare against |
| A03 rankEvidence | `hq/lib/search.mjs` FTS5 BM25 rank | not comparable | fixed weighted relevance/quality/freshness score vs corpus-derived BM25; no shared ranking function |
| A04 packContext | `hq/lib/context.mjs` exact 0/1 knapsack (32-token units) | not comparable for selection | greedy topical coverage vs exact knapsack over score; comparable only for the shared invariants (budget respected, secret candidates excluded) |
| A05 reconcileClaims | none (canon keeps contradictions visible, no value-level grouping) | not comparable | HQ has no claim-value reconciliation pass |
| A06 traceImpact | `hq/lib/graph.mjs` `impact()` reverse BFS | comparable in shape | both are bounded, cycle-safe reverse traversals; HQ keys nodes by indexed file path, A06 by arbitrary id, so an equivalence proof needs one id mapping |
| A07 orderActionPlan | none | not comparable | HQ has no plan topological sort |
| A08 allocateTasks | `scripts/sync.mjs` claims (bus owner) | not comparable | A08 emits proposals only; bus claims are durable owner state |
| A09 coalesceObservations | `hq/lib/realtime.mjs` coordinator debounce | not comparable | HQ coalesces run requests per (kind, project); A09 compacts observation events per subject with last-revision-wins |
| A10 detectDrift | `hq/lib/lake.mjs` `drift()` (working-tree vs index) | not comparable | A10 compares pinned digest/version contracts; lake drift compares git state to index hashes |
| redaction rule | `isSecretPath` in realtime/canon/filetypes/kb-intake, plus `context.mjs` | equal after the 2026-11-02 fix | the optimizer rule is now a strict superset of every HQ module's rule |
| null / malformed input | HQ modules assume shaped input | better after the 2026-11-02 fix | the optimizer now fails closed instead of throwing |

### Defects fixed on 2026-11-02 (each with a failing case before the fix)

1. **A04 selection depended on input order** when two candidates shared an id and a value/token ratio. The tie-break compared ids only, so equal ids fell back to input order. Fixed with a canonical total tie-break; regression case `A04 selection is invariant when candidate ids collide`.
2. **The optimizer redaction rule admitted `.envrc`**, which every HQ module excludes (`name.startsWith('.env')`). Fixed by rebuilding the rule as a per-segment superset of the HQ rule; regression case `redaction refuses every secret path the HQ rule refuses`.
3. **Malformed index rows and null array inputs threw** (`resolveAlias` on a row without `alias`, every export on `null`). Fixed with a `list()` guard and an alias type guard; regression case `malformed index rows and null inputs never throw`.

Still open, not fixed: `coalesceObservations` uses `output.push(...segment.values())`. A single segment of 100 000 events was measured on Bun 1.4.2 and Node 24 and did not overflow the argument limit, so no failure is provable at the documented bound; the pattern remains a latent hazard.

Next execute:
- bun test hq/test/semantic-optimizers.test.mjs
- bun test --timeout 30000 hq/test/
- bun run typecheck
- differential tests before promoting any optimizer into the runtime selectors, graph or coordinator

Do not infer that the library's existence proves end-to-end language execution, KB migration, wiki reprogrammability or bus integration. Source: [ARCHITECTURE.md](ARCHITECTURE.md), [CONTRACT-AND-ROADMAP.md](CONTRACT-AND-ROADMAP.md).
