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

## Seven round goals

R0 clean-clone baseline; R1 HQ identity+index; R2 joined capability/roadmap/source/evidence; R3 HQ documentation and verified original retirements; R4 competing Host/mobile/TierFlow subsystems; R5 composable HQ developer views and agent workflow; R6 fresh-clone verification and executable backlog.

Every round reports four pinned repository heads, four independently verified bundles, one integration manifest, evidence, risk/rollback and strategic decisions. Stop destructive actions when gates fail, not audit/research progress.
