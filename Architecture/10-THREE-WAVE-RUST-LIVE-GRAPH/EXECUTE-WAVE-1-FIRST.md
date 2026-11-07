# WAVE 1 — EXECUTE BEFORE DESIGNING WAVE 2
Date: 2026-11-07
Status: launch-authoritative Wave 1 execution charter (planned; no implementation success claimed)
Campaign: one logical Rust graph for VIVIM code, KB, semantic commands, CLI and generated wiki.
Dependencies: ../07-INTEGRATION-PHILOSOPHY.md, ../09-UNIFIED-INTELLIGENCE-COMMAND-WIKI.md, README.md

## Owner decision
**Do NOT author a final Wave 2 plan until Wave 1 has built and run executable code on actual local source and produced independent integration evidence.** Older Wave 2 drafts in this campaign are hypotheses and explicitly non-authoritative; replace them only after Wave 1 has run.

The six Wave 1 lanes must work in parallel on separate owned surfaces, join in one Rust journal, and yield a live-derived **actual vs intended** and **blind spot** report. No plan-only substitution. Any unsupported source or blocked test must be made visible.

## Mission outcome
On the local machine, run existing HQ code intelligence (A), KB/semantic identity (B) and language/command/CLI (C) through a versioned, validated data-only adapter into **existing Rust HQKG**. The generated wiki is the **derived product and quality test** of their convergence, not a fourth independent data source. Persist into a dedicated development graph journal and offer working read-only CLI queries. Prove it with a source-backed, non-synthetic vertical slice. Then examine what the running graph does *not* know; write that to a structured ledger used to design Wave 2.

Target first query answers:
- WHERE: repo/path/symbol/source SHA, graph node ID, documentation page.
- WHAT: type/role, purpose, inputs/outputs, available commands, accepted knowledge, known limitations.
- TALKS TO: static imports/calls and *separately tagged* observed runtime/event communications; no conflation.
- CAN DO: actual CLI syntax, registered action, handler, status wired/declared/proposed, permission owner.
- WRITES: SQL/KB/config/Host/Visual source-native ownership, with evidence or unknown.
- GAP: expected target, observed real state, missing/wrong/unknown, evidence, proposed integration.
- CHANGE: version, freshness, affected entities and stale projections.

## Six parallel implementation lanes
| Lane | Owns | Specific implementation (reuse first) | Required evidence | Makes next step easier |
|---|---|---|---|---|
| **A — Scoped Reset, Indexing Intent & Multi-Root Code Intelligence** | Scoped old-run reset + one logical VIVIM project / multiple-source-root adapter (no HQ engine rewrite) | Inspect and archive old run; safely reset only prior-run disposable indexing state, preserve durable KB/kernel; scope multiple source roots as one project, capture versioned indexing intent (why/what to discover); then index files, symbols, imports and @hq annotations with source hashes and exclusions | One logical project with N declared/verified roots and one queryable indexing intent; reset receipt, no data loss, source coverage by root, static edge confidence | Graph can discover implementation and dependencies without legacy-run contamination |
| **B — KB, Semantic IDs & Authority** | Narrow HQ KB/semantic read adapter | Query docs, accepted/proposed decisions, CANON terms, kernel revision where available, semantic IDs/aliases/evidence; preserve privacy and authority, join to source IDs where verified | 1+ real KB/decision, 1+ semantic ID, provenance and accepted/proposed distinct; no owner auto-promotion | Commands/wiki can resolve against shared knowledge rather than guesses |
| **C — Commands, NL & CLI Inventory** | Read-only command/action extractor/mapper | Enumerate existing `hq`/Rust CLI help and dispatch, action registry, action router, callable handlers, schemas/effects/actors, declared/wired status; register source-linked operation nodes | 1+ real CLI command with exact syntax and source handler, 1+ action/intent route, NO_MATCH test; effects marked read-only vs source-owned | Subsequent integrations inherit one inspectable control vocabulary |
| **D — Wiki as Integration Product & Quality Oracle** | Existing HQ wiki builder plus integration-level validation (no second wiki writer) | Generate wiki views **from the combined A + B + C model**, including source locations, semantic meaning, real CLI/action mappings, call/dependency links and evidence. Measure missing pages, dead links, source-only content, unknown meanings and unwired commands as defects in A/B/C integration. Prepare test harness concurrently, run final quality evaluation after E. | Wiki must regenerate deterministically and reveal where A/B/C are correct or disconnected; verified source→knowledge→command→wiki trace; no standalone fourth-data-stream certification | Turns improvements to A/B/C into immediately visible, testable, usable documentation |
| **E — Rust HQKG Live Projection & Runner** | Integration-owned HQKGX1 adapter, Rust importer CLI, dedicated dev journal and query CLI | Implement bounded deterministic **source-owned** projection from lanes A+B+C, using the existing HQKGX1 importer, stable canonical `hqg1` IDs, source hash, journal save, no-op unchanged and query helpers; expose unified read model for D's generated wiki; do not import D as a separate authority | Real Rust executable links code, KB, commands and CLI and can drive regenerated wiki output; repeat/no-op, corruption, collision and restart tests | The graph becomes the testable substrate and the wiki's source of integrated relationships |
| **F — Integration Testing, Actual/Target & Blind Spots** | Labs verification, gap schema and report generator; no cross-lane writes | Independently execute E on local real source, compare planned target entities from Architecture docs and observatory, inspect unknown coverage, missing edges, duplicate owners, CLI/wiki/drift, schema/transport and dataflow holes | Report from actual graph query, raw commands/exit codes/SHAs, ranked gap ledger with evidence and severity; distinguish untested from absent | Wave 2 is designed from real failure modes, not theoretical architecture |

## Hand-off interfaces (avoid a big upfront contract program)
- A/B/C/D each emit bounded **data-only, stable-ID candidate records + provenance + explicit unknowns**, and a test fixture sourced from real, non-sensitive material.
- E is **sole final HQKGX1 assembly/integration owner**, mapping validated records to Rust node/edge kinds; only the Rust journal writer persists graph.
- F reads the assembled graph/CLI outputs and target docs. It does not self-certify Lane E.
- Every lane HANDOFF names output location, tested commands, source revision, output digest, omitted fields, error/failure behavior, and successor lane consumption.
- Share source file contracts early but **do not** create a competing semantic registry, transform engine, or separate graph storage.

## DEV vs PRODUCT
**DEV:** full repo coverage, graph inspector, index diagnostics, verbose ingest status, mock/synthetic tests clearly labeled, source-level CLI and wiki visibility, gap ranks. Run only against local approved files and a dedicated dev journal; preserve existing HQ DB and browser profile.

**PRODUCT SHAPE:** identical node/edge/operation IDs and semantic meaning; user-scoped and privacy-filtered sources, minimal diagnostics, approved adapters, bounded resources, safe read-only knowledge/command discovery, and Host-authorized effects only. No DEV source-crawling privilege, research fixture, lab state, experimental handler or raw provider access is a product dependency.

Wave 1 only needs DEV fully runnable. PRODUCT gets an explicit buildable target/profile specification and release contamination tests where possible; do not falsely claim a product release.

## Execution timeline (event/order, not hours)
**Phase 0 — MANDATORY SCOPED RESET**: follow [WAVE-1-INDEX-RESET-AND-INTENT.md](WAVE-1-INDEX-RESET-AND-INTENT.md): inspect/archive previous run, preserve durable KB and multi-repo indexer, safely retire/reset old run-owned index state, establish ONE logical VIVIM project with MULTIPLE in-scope folders/repositories and explicit versioned indexing intent. Then record fresh heads, execute the initial index under the new scope, and run HQ/Cargo tests. Never blindly invoke `hq reset --yes`.
**Phase 1**: A, B and C build real source read adapters in parallel; D builds independent wiki acceptance cases and quality checks in parallel but does not certify output until A+B+C+E are connected.
**Phase 2**: E validates and imports A+B+C into Rust; D generates the wiki from this integrated model and measures missing links/meaning/commands; E and D resolve only their authorized integration defects.
**Phase 3**: F independently runs end-to-end/negative tests, system inventory and comparison; requests only narrowly owned fixes.
**Phase 4**: integration owner publishes final evidence bundle and an **observation-driven Wave-2 input dossier**. Wave 2 design begins after results are reviewed.

## Required final runtime demo
A short reproducible command recipe from repos-hq root, including steps for existing HQ index/wiki, projection generation, Rust import, and queries. The demonstrated graph inputs MUST come from real registered source/code, KB/semantic, and command/CLI records. The wiki MUST be regenerated as a **derived output** from those inputs, and show at least one verified cross-domain chain. Missing/incorrect wiki content is an A/B/C/E integration quality failure or an explicit unsupported feature, not a missing fourth input domain.
Proof transcript includes:
1. `where <capability>`: repo file/symbol/head/KB source.
2. `what <capability>`: knowledge description, command/engine/writer status.
3. `talks-to <capability>`: crosslinks tagged as static/imported/observed/not-known.
4. `gap <capability>`: desired target vs source observed vs runtime tested with provenance.
5. Re-run unchanged: import is no-op, stable digest, no new journal snapshot.
6. Source edit or safe KB test revision: modified result after reindex/refresh, no unrelated loss.
7. Conflict/secret/stale input: clear refusal; existing graph remains valid.
8. Restart: reloaded graph yields equivalent query results.

These CLI words are the desired user-facing façade, **not existing command names**; Lane E must document actual new/old syntax and prove it.

## Integration gate / definition of done
**PASS** only when:
- Native Cargo and HQ tests/commands run on the real target environment, with logs and exact exit codes.
- Actual Rust graph contains linked nodes for the **three foundational domains**: code (A), KB/identity (B), CLI/actions/commands (C); generated wiki output (D) is source- and graph-linked, not accepted as a separate canonical input.
- At least one source→knowledge/semantic→command chain is traversable in Rust and **accurately expressed in the generated wiki**; a wiki page generated only from files without B/C semantics does not count.
- Coverage denominator names every in-scope source root under ONE logical VIVIM project, explicit indexing intents and exclusions. Incomplete scanning = partial, not complete. Prior-run state cannot contaminate fresh observations.
- No second semantic truth, KB writer, journal writer or Host effect authority.
- Actual-vs-target machine-readable report and human wiki/Markdown report are generated from **the graph**.
- Blind spot register classifies missing, observed-but-unwired, duplicate, ambiguous, stale, unsafe, untested, and out-of-scope. Top gaps have exact source or refusal evidence.
- Dev and product variants are documented separately; end-user profile never requires the DEV crawler.
- Six HANDOFFs + integration receipt; refresh/no-op, negative-input and restart evidence.

**BLOCKED** if the scoped reset cannot be proven safe, project/intent/root scope remains ambiguous, any foundational A/B/C real data domain is absent, the Rust graph is not actually run, the wiki does not demonstrate A+B+C wiring, or the end-to-end report is not machine-generated. **PARTIAL** only for explicitly allowed bounded coverage with clear denominator; no claim entire codebase scanned from a sample.

## Wave 2 MUST be informed by Wave 1
After acceptance produce `WAVE-2-INPUT-DOSSIER.md` using observed:
1. Actual indexed/eligible/excluded code and KB coverage (per repo; counts with denominators).
2. Nodes/edges by semantic kind, source, freshness and evidence strength.
3. Cross-domain link rate; orphan engines/commands/wiki pages; ambiguous/duplicate identities.
4. Unwired and unimplemented actions (vs actions known executable), CLI syntax gaps and real NL ambiguity.
5. Missing input/output/write/dataflow relationships, transport/schema gaps, owner conflicts.
6. Native runtime performance (throughput, refresh lag, memory, journal size), failures and recovery.
7. What worked immediately, what required a dev-only bridge, what cannot ship in user product.
8. Candidate Wave 2 tasks ranked by measured downstream unlock and severity.
9. Unknowns and failed tests preserved verbatim; no silent assumptions.
10. A proposed 6-lane Wave 2 **only after** execution, with each assignment tracing to a graph-derived finding.

## Runtime and repo rules
Use fresh heads and GitHub main only, non-force CAS, no PRs/branches/Actions, disjoint lane ownership, **owner-authorized scoped reset of the old index run only**, no blanket `hq reset --yes`, no Playwright/Puppeteer, no provider APIs or browser profile replacement. Source-native canonical writers remain. All expected output paths must be verified before linking them publicly.

## Source implementation pointers
- `repos-hq/hq/lib/indexer.mjs`, `schema.mjs`, `graph.mjs`, `kb.mjs`, `semantic-registry.mjs`, `action-registry.mjs`, `action-router.mjs`, `wiki.mjs`, `wiki-program.mjs`.
- `repos-hq/hq/bin/hq.mjs`, `research/rust-knowledge-graph/src/lib.rs`, `engine.rs`, `import.rs`, `src/bin/hqkg-import.rs`.
- `research/agentic-knowledge-graph/scripts/export-hqkg.mjs` and `research/graph-kernel-integration/scripts/export-unified-hqkg.mjs` are existing **research** exporters, **not** live HQ code/KB/command/wiki exporters.
