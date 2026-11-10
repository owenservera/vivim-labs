# Campaign charter — maximal single-version VIVIM

## North star

**Produce the maximal coherent version of capabilities already present across the four repos, with one go-forward implementation for each authoritative responsibility and no loss of useful non-overlapping functionality.** Maximal means complete knowledge and an integration plan, **not** shipping every experiment or rewriting functioning code.

## Hard constraints

- Exactly four primary repos; maintain independent Git histories. HQ is the agent workspace and one existing cross-repo index/documentation backbone.
- Harness retains current product roadmap authority, Host/browser effects, approvals and receipts; one existing phone app `apps/mobile-relay`.
- TierFlow retains a domain-neutral visual runtime; any publisher cutover must be separately authorized and acceptance-tested.
- Labs supplies only evidence, research and safe archives; this workspace logs campaign decisions as **non-authoritative evidence**, not product status.
- No provider API substitution, Playwright/Puppeteer, second graph/database/docs generator or alternative production planner.
- No non-trivial deletion until negative consumer proof, tested replacement/rollback and appropriate provenance/preservation.
- No local Windows/Android physical acceptance claims from cloud tests.

## Mandatory conceptual overlap discovery

Detect competing implementations even when languages, filenames, dependencies or contracts differ. For each concept record: intent, candidate repo:path:symbol@SHA, interfaces, callers, runtime reachability, overlapping behavior, unique capabilities, security/authority, state persistence, tests, performance, recovery, constraints and evidence quality.

Decisions: `SELECT_CANONICAL`, `COMPOSE_BEST_OF_BREED`, `KEEP_INDEPENDENT`, `KEEP_AS_ADAPTER`, `RETIRE_AFTER_MIGRATION`, `ARCHIVE_HISTORICAL`, `DEFER_PENDING_PROOF`.

Classifications: `CANONICAL`, `COMPLEMENTARY`, `ADAPTER`, `PROJECTION`, `COMPETING`, `DUPLICATE`, `ORPHANED`, `HISTORICAL`, `UNVERIFIED`.

Score by capability coverage (25), correctness/test evidence (20), architecture/authority fit (20), maintenance/reuse (15), security/recovery (10), performance/footprint (10), while **security, source ownership and behavioral compatibility are hard gates**. Present proposed composite when candidates contain different valuable strengths. Unverifiable tests score as unknown, never pass.

## The maximal single-version roadmap

Generate **one HQ-indexed derived roadmap view** (not a competing database) linking: canonical concept → all competing implementations → overlap/difference matrix → selected strategy → unique features to absorb → unselected component disposition → exact dependency/consumer chain → test and acceptance gates → next bounded work order. Preserve namespaced native source IDs. Include separate near-term V1 closure and longer-horizon capability tracks; neither may silently erase an orphan.

Every source file must receive an evidence-qualified disposition: integrated / pending orphan / redundant after proof / historical / operational keep / unknown. Every unresolved decision must appear in the round open-item delta. No silent feature loss and no silent status upgrade.

## Compounding HQ-native operating system

**HQ is the engine Z.ai works through every round, and continuously improves.** The current HQ SQLite index, Rust knowledge graph, semantic registry, source adapters, context assembler, work routing, receipts and wiki generator are the mandatory default for discovery and reporting. They must become increasingly sufficient for doing real development without exhaustively rereading all four repositories. No independent Z.ai crawler/database/orchestrator as an alternative authority.

**Productive work starts in Round 1; there is no audit-only Round 0.** Improve HQ while delivering immediately useful functionality. Each round should:
- Query HQ first and log baseline query coverage, source freshness and confidence.
- Repair missing source adapters, wrong four-project identities, missing consumers, graph edges, CLI read surfaces or stale documentation when they impair the actual assigned integration work.
- Implement/test bounded source improvements; index changes and add regression fixtures that prevent having to rediscover the same facts.
- Store durable **source-linked** decisions, negative-consumer evidence, integration crosswalks, tests and receipt pointers through existing HQ mechanisms; Labs stores only campaign review/provenance.
- Demonstrate that the next round can answer more questions with fewer raw-file reads, lower context/token overhead and fewer repeated exploratory searches, without sacrificing accuracy.

**Round 4 objective:** expect an agent to select a task, ask HQ for a bounded context/evidence bundle, see all candidate implementations and their unique features, follow dependencies/owners and run targeted tests with minimal broad source exploration. If this fails, treat the failing HQ query/adapter/index as an actionable defect and improve it, rather than declaring HQ production-ready.

## Execution sequence

- **R1 — Working HQ:** correct four-project identities and clean-clone paths, prove source/index freshness, expose existing useful read models and implement a real tested improvement.
- **R2 — Semantic convergence:** wire source-owned capability, requirement, code, journey, roadmap and evidence relationships into existing HQ views while fixing actual code and integration gaps.
- **R3 — Documentation convergence:** generate shared docs through HQ; verify consumers and remove safe redundant originals; improve documentation freshness feedback during engineering.
- **R4 — HQ-driven architecture convergence:** use HQ as the primary work interface to compare competing Host/Android/TierFlow implementations and land evidence-backed integrations with minimal rediscovery.
- **R5 — Operational refinement:** better context assembly, task routing, dependency-aware tests, provenance and developer-facing views through existing HQ interfaces.
- **R6 — Fresh-clone acceptance:** restore four Git bundles, rebuild HQ from clean sources and verify reproducible intelligence, docs, safe tests and an executable remaining queue.

Every round reports four pinned repository heads, independently verified bundles, one manifest, actual changed behavior/test results, HQ efficiency deltas, rollback gates and strategic decisions. Stop destructive actions when gates fail, **not** all non-destructive engineering progress.
