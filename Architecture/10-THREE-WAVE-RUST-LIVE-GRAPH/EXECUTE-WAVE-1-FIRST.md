# WAVE 1 — EXECUTE BEFORE DESIGNING WAVE 2
Date: 2026-11-07
Status: launch-authoritative Wave 1 execution charter (planned; no implementation success claimed)
Campaign: one logical Rust graph for VIVIM code, KB, semantic commands, CLI and generated wiki.
Dependencies: ../07-INTEGRATION-PHILOSOPHY.md, ../09-UNIFIED-INTELLIGENCE-COMMAND-WIKI.md, README.md

## Owner decision
**Do NOT author a final Wave 2 plan until Wave 1 has built and run executable code on actual local source and produced independent integration evidence.** Older Wave 2 drafts in this campaign are hypotheses and explicitly non-authoritative; replace them only after Wave 1 has run.

The six Wave 1 lanes must work in parallel on separate owned surfaces, join in one Rust journal, and yield a live-derived **actual vs intended** and **blind spot** report. No plan-only substitution. Any unsupported source or blocked test must be made visible.

## Mission outcome
On the local machine, run existing HQ index/KB/command/wiki read models through a versioned, validated data-only adapter into **existing Rust HQKG**. Persist into a dedicated development graph journal and offer working read-only CLI queries. Prove it with a source-backed, non-synthetic vertical slice. Then examine what the running graph does *not* know; write that to a structured ledger used to design Wave 2.

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
| **A — Code & Dependency Ingestion** | Narrow HQ index read/export adapter, not HQ core rewrites | Read registered projects, files, symbols, resolved and unresolved imports, @hq annotations, Git head/source hash, source exclusions; produce stable records in graph-ready source format | Coverage by repo/language, 3+ source references, edge confidence marked static; freshness and ignored-source counts | Graph can automatically find the real code, implementations and inter-component imports |
| **B — KB, Semantic IDs & Authority** | Narrow HQ KB/semantic read adapter | Query docs, accepted/proposed decisions, CANON terms, kernel revision where available, semantic IDs/aliases/evidence; preserve privacy and authority, join to source IDs where verified | 1+ real KB/decision, 1+ semantic ID, provenance and accepted/proposed distinct; no owner auto-promotion | Commands/wiki can resolve against shared knowledge rather than guesses |
| **C — Commands, NL & CLI Inventory** | Read-only command/action extractor/mapper | Enumerate existing `hq`/Rust CLI help and dispatch, action registry, action router, callable handlers, schemas/effects/actors, declared/wired status; register source-linked operation nodes | 1+ real CLI command with exact syntax and source handler, 1+ action/intent route, NO_MATCH test; effects marked read-only vs source-owned | Subsequent integrations inherit one inspectable control vocabulary |
| **D — Automatic Wiki & Programs** | Read-only generated wiki export/adapter | Build/check existing wiki without rewriting its canonical renderer; ingest generated pages and links; map wiki-program preview, config versions, comment/wiki bindings and semantic IDs | 1+ real generated wiki page linked to actual source/semantic or command node; stable deterministic rebuild; no activation/effect | The graph is explorable as a documentation and programming workbench |
| **E — Rust HQKG Live Projection & Runner** | Integration-owned HQKGX1 adapter, Rust importer CLI, dedicated dev journal and query CLI | Implement bounded deterministic **source-owned** projection from lanes A–D (not static research graph), use existing HQKGX1 importer, stable canonical `hqg1` IDs, source-hash, atomic journal save, no-op unchanged, query helpers; no new Rust graph registry | Real Rust executable imports and queries code+KB+command+wiki objects with source links; tests repeat/no-op, corruption, collision, restart | The graph itself becomes the testable substrate for next wave |
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
**Phase 0**: record fresh main SHAs, local environment, available CLI/build tools, existing registered repo/index coverage. Run initial HQ/Cargo tests; do not reset.
**Phase 1**: run lanes A–D in parallel; every lane provides actual read data and records gaps.
**Phase 2**: E validates, combines and imports real A–D exports; produces journal and runnable query interface.
**Phase 3**: F independently runs end-to-end/negative tests, system inventory and comparison; requests only narrowly owned fixes.
**Phase 4**: integration owner publishes final evidence bundle and an **observation-driven Wave-2 input dossier**. Wave 2 design begins after results are reviewed.

## Required final runtime demo
A short reproducible command recipe from repos-hq root, including steps for existing HQ index/wiki, projection generation, Rust import, and queries. The demonstrated outputs MUST come from real registered source/KB/command/wiki records; if one domain is unavailable, say BLOCKED instead of substituting a synthetic success.
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
- Actual Rust graph contains nodes for all four requested domains: code, KB, CLI/action/command, wiki, with source-linked edges.
- One cross-domain traversal (source→command→wiki or source→semantic→wiki) is proven, not merely four disconnected inventories.
- Coverage denominator names every registered repo and exclusions. Incomplete scanning = partial, not complete.
- No second semantic truth, KB writer, journal writer or Host effect authority.
- Actual-vs-target machine-readable report and human wiki/Markdown report are generated from **the graph**.
- Blind spot register classifies missing, observed-but-unwired, duplicate, ambiguous, stale, unsafe, untested, and out-of-scope. Top gaps have exact source or refusal evidence.
- Dev and product variants are documented separately; end-user profile never requires the DEV crawler.
- Six HANDOFFs + integration receipt; refresh/no-op, negative-input and restart evidence.

**BLOCKED** if any of four real data domains absent, graph isn't actually run, or end-to-end report isn't machine-generated. **PARTIAL** only for explicitly allowed bounded coverage with clear denominator; no claim entire codebase scanned from a sample.

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
Use fresh heads and GitHub main only, non-force CAS, no PRs/branches/Actions, disjoint lane ownership, no reset of HQ index, no Playwright/Puppeteer, no provider APIs or browser profile replacement. Source-native canonical writers remain. All expected output paths must be verified before linking them publicly.

## Source implementation pointers
- `repos-hq/hq/lib/indexer.mjs`, `schema.mjs`, `graph.mjs`, `kb.mjs`, `semantic-registry.mjs`, `action-registry.mjs`, `action-router.mjs`, `wiki.mjs`, `wiki-program.mjs`.
- `repos-hq/hq/bin/hq.mjs`, `research/rust-knowledge-graph/src/lib.rs`, `engine.rs`, `import.rs`, `src/bin/hqkg-import.rs`.
- `research/agentic-knowledge-graph/scripts/export-hqkg.mjs` and `research/graph-kernel-integration/scripts/export-unified-hqkg.mjs` are existing **research** exporters, **not** live HQ code/KB/command/wiki exporters.
