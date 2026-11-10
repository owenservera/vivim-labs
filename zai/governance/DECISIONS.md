# Strategic decision register — campaign evidence only

**Purpose:** Explain *where and why* Z.ai makes architecture or implementation selections. Append new decision entries; do not rewrite prior rationale. Owner approvals and operational status live in the existing authoritative system, not here.

## Required entry template

### ZAID-NNN — Short concept title

- **Round / date / proposer:**
- **Status:** PROPOSED | OWNER_APPROVED | REJECTED | SUPERSEDED | IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL | COMPOSE_BEST_OF_BREED | KEEP_INDEPENDENT | KEEP_AS_ADAPTER | RETIRE_AFTER_MIGRATION | ARCHIVE_HISTORICAL | DEFER_PENDING_PROOF
- **Concept/intent and affected users:**
- **Candidates with immutable provenance:** repo, full commit SHA, path, symbol, consumer
- **Overlap:** features shared by candidates
- **Distinct value:** what each implementation does that alternatives do not
- **Alternatives considered:** including doing nothing
- **Best-of-breed rubric + hard-gate evaluation:** cite actual tests, unknowns and source ownership
- **Decision and rationale:** identify go-forward responsibility and components to carry over
- **Impact:** existing authority, migration plan, compatibility, footprint and dependencies
- **Implementation tasks and source-owned work orders:** IDs/link only; no shadow task state
- **Verification and rollback:** evidence links and explicit unverified gates
- **Open items linked:** ZAIQ-* IDs from OPEN-ITEMS.md
- **Owner authorization evidence:** link to approved issue/record; do not assume approval
- **Supersedes / superseded by:**

**ID scheme (decided in R1):** ZAIDs are sequential (`ZAID-001`, `ZAID-002`, …), mirroring the ZAIQ scheme of [OPEN-ITEMS](./OPEN-ITEMS.md). DECISION-PROTOCOL step 4 mandates only: "**Propose:** append a ZAID to [DECISIONS](./DECISIONS.md) and any ZAIQ to [OPEN-ITEMS](./OPEN-ITEMS.md)" — no ID format is prescribed, so this register's original `ZAID-YYYYMMDD-NNN` template heading was ambiguous against the sequential entries actually written; the sequential scheme is canonical from R1 onward (judgment recorded in rounds/R01-REPORT.md §5) and the template heading above has been aligned to it.

## Initial policy decisions (owner-provided, not Z.ai architecture inventions)

- **ZAI-BASE-001:** Exactly four primary VIVIM repositories; `vivim-ui` is local folder alias for GitHub `tierflow`.
- **ZAI-BASE-002:** Existing HQ indexing, Rust graph and wiki generation are the single cross-repo development intelligence backbone.
- **ZAI-BASE-003:** Harness retains live effects, Host, one Android app and current product roadmap authority pending verified migration.
- **ZAI-BASE-004:** Labs archives/evidence are public-safe only; no separate live capability/roadmap authority.
- **ZAI-BASE-005:** Prefer composed best-of-breed solutions and preserve unique capabilities before retiring duplicate code.

These are scope policies; future ZAID decisions must carry candidate evidence, authorization and tests.

## Round 1 (R1 — Working HQ) decisions, 2026-11-10

Numbering starts at ZAID-001: no prior ZAID records exist (ZAI-BASE-001..005 above are owner-provided policies). R1 implementation landing commits: `repos-hq` `9cce014d91187f01d5d6506bb9eeb86e0a1ae111` (author Z.ai <zai@vivim.invalid>, 2026-11-10T03:46:21Z), on pinned baseline `4631d19cc57a07fa532e627f21737e489d629055`; and `repos-hq` `bdfe2644576071a595c0f3fd9a52e76118ccfcb9` (author Z.ai <zai@vivim.invalid>, 2026-11-10T04:41:09Z), directly on `9cce014`, landing ZAID-006 (always-on automation). Both entries below are finalized.

### ZAID-001 — Re-register the four VIVIM identities onto clean-clone paths

- **Round / date / proposer:** R1 / 2026-11-10 / Z.ai
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** HQ's project registry must resolve exactly the four canonical VIVIM spaces (ZAI-BASE-001) at routable local paths so every HQ consumer (index, check/verify freshness, wiki, find/search, context assembly) can operate on this clean-clone workspace. Affected users: Z.ai operating agent and all HQ CLI read surfaces.
- **Candidates with immutable provenance:** (a) `repos-hq@4631d19cc57a07fa532e627f21737e489d629055:projects.json` — five entries, every path Windows-style `C:\0-BlackBoxProject-0\…` (repos-hq, vivim-labs, vivim-ui, vivim-web-harness, vivim-workstation); unroutable on this Linux workspace, i.e. all 5 spaces lost to local HQ operations. (b) `repos-hq@9cce014d91187f01d5d6506bb9eeb86e0a1ae111:projects.json` — four entries: `repos-hq` → `.`, `vivim-labs` → `<workspace>/repos/vivim-labs`, `vivim-ui` → `<workspace>/repos/tierflow` (local alias of GitHub `tierflow`), `vivim-web-harness` → `<workspace>/repos/vivim-web-harness` (workspace-absolute paths genericized for public safety). Consumer: HQ project resolution across index/check/wiki/context.
- **Overlap:** Both are v1-schema project registries claiming the VIVIM names; both list the four core repos.
- **Distinct value:** Candidate (b) is routable on this machine, preserves `vivim-ui` as tierflow's local alias (zai README; repos-hq lane README), and drops the non-VIVIM `vivim-workstation` space.
- **Alternatives considered:** keep the Windows-path registry (all five spaces stay unroutable — R1 impossible); re-add `vivim-workstation` (violates the four-repo scope, ZAI-BASE-001); clone into different local names (breaks the documented `vivim-ui` alias convention); do nothing (HQ unusable locally).
- **Best-of-breed rubric + hard-gate evaluation:** Correctness/test evidence — post-change index run resolved all four spaces (receipt `/tmp/health.json`: 4 projects indexed 2026-11-10T03:46Z); `hq check --strict` PASS on the clean clone; `hq doctor` all green. Architecture/authority fit — registry stays a repos-hq-owned tracked input; no second authority created. Security/source ownership — no new content classes, no provider/effect surfaces. Unknowns: none material for identity resolution itself.
- **Decision and rationale:** Canonical identity set = the four VIVIM repos at local clean-clone paths; `vivim-ui` remains tierflow's local alias; `vivim-workstation` removed as non-VIVIM. Candidate (b) is the go-forward registry.
- **Impact:** No authority change; enables every downstream R1 verification (freshness, read model, wiki). Consumers resolve by project name, so no consumer breakage; the removed fifth space was non-VIVIM by policy.
- **Implementation tasks and source-owned work orders:** repos-hq commit `9cce014` — `projects.json` (+4/−10). No shadow task state; this register + R01-REPORT are the record.
- **Verification and rollback:** Verified by the R1 index/freshness/health receipts cited above and the full-suite run at 9cce014 (1912 pass / 13 pre-existing fail; receipt `/tmp/smart-run.txt`). Rollback: `git revert 9cce014 -- projects.json` restores the old registry (but re-strands the local workspace). Unverified gate: owner review of this record.
- **Open items linked:** ZAIQ-001 (progressed — identities resolved and indexed; consumer-compatibility proof partial), ZAIQ-002 (progressed — strict freshness now provable).
- **Owner authorization evidence:** None recorded as a separate approval. Charter R1 mandate ("correct four-project identities and clean-clone paths") is the operating authorization; owner review of this register pending. No product authority, live effects or capability retirement involved.
- **Supersedes / superseded by:** Supersedes the registry contents of `projects.json` at `4631d19c` (same file, same schema).

### ZAID-002 — Freshness parity: check/verify honor the indexer's configurable walk cap

- **Round / date / proposer:** R1 / 2026-11-10 / Z.ai
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** `hq check` / `hq verify` freshness walks must observe the same file universe the indexer observes, so strict freshness gates can pass on a clean clone. Affected users: any agent or gate relying on `hq check --strict`.
- **Candidates with immutable provenance:** (a) `repos-hq@4631d19c:hq/bin/hq.mjs#cmdCheck` / `#cmdVerify` — hardcoded 10MB walk cap, and the freshness walk ignored `config.ignores`. (b) `repos-hq@9cce014d91187f01d5d6506bb9eeb86e0a1ae111:hq/bin/hq.mjs#cmdVerify` (async function, line ~981) and `#cmdCheck` (async function, line ~3410) — `const walkCap = Number(flags['max-file-bytes'] || config.maxFileBytes)` (lines ~1000 / ~3425) with `extraIgnores` passthrough, walking via the same `discoverFiles` (`repos-hq@9cce014:hq/lib/walk.mjs#discoverFiles`, line ~78) the indexer uses. Consumers: `hq check`, `hq verify`, strict gates.
- **Overlap:** Both candidates walk project files and report per-space freshness status.
- **Distinct value:** Candidate (b) makes the indexer's configurable `maxFileBytes` (1.5MB default) the single cap authority and honors `config.ignores`; candidate (a) permanently reported files between 1.5MB and 10MB as "new" (observed: 6.8MB `.work/blockers/records.jsonl`, 3.2MB `source-dispositions.v1.json`), so `check --strict` could never pass on a clean clone.
- **Alternatives considered:** raise the hardcoded cap above 10MB (diverges again on the next config change); disable strict mode (defeats the gate); lower the indexer cap to 10MB (removes over-cap source from the index); do nothing (R1 freshness gate unpassable).
- **Best-of-breed rubric + hard-gate evaluation:** Correctness/test evidence — new regression fixture `repos-hq@9cce014:hq/test/check-freshness-parity.test.mjs` (3 tests, all passing) plus `hq check --strict` PASS on a clean clone; full-suite receipt 1912 pass / 13 pre-existing fail. Architecture — one cap authority (`flags['max-file-bytes'] || config.maxFileBytes`) shared by check/verify/indexer. Hard gates — no security, source-ownership or behavioral-compatibility violation; behavior change is intentional (over-cap files no longer misreported as new). Unknowns: no isolated before/after latency benchmark recorded.
- **Decision and rationale:** The config-driven cap + shared `discoverFiles` walk is canonical; check/verify and the indexer now walk the same universe.
- **Impact:** `hq check --strict` becomes a trustworthy gate on clean clones; freshness semantics change for files between the old and new caps (previously always "new").
- **Implementation tasks and source-owned work orders:** repos-hq commit `9cce014` — `hq/bin/hq.mjs` (+21/−7), `hq/test/check-freshness-parity.test.mjs` (+114, 3 tests).
- **Verification and rollback:** 3/3 parity tests pass; `hq check --strict` PASS on the clean clone at 9cce014; full-suite receipt as above. Rollback: `git revert 9cce014` (single commit; the strict gate reverts to permanently-failing behavior). Unverified gate: owner review.
- **Open items linked:** ZAIQ-002 (progressed — strict freshness proven; graph parity still open).
- **Owner authorization evidence:** None recorded as separate approval; charter R1 mandate ("prove source/index freshness") is the operating authorization. Owner review pending.
- **Supersedes / superseded by:** Supersedes the hardcoded-cap freshness-walk behavior of `4631d19c`.

### ZAID-003 — Unmask bin/ source directories; content sniffing is the canonical binary gate

- **Round / date / proposer:** R1 / 2026-11-10 / Z.ai
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** The HQ index/read model must see hand-written CLI source living in tracked `bin/` directories while still excluding compiled binaries — by content, not directory name. Affected users: wiki/context consumers that document the CLI surface; any agent navigating via `hq find`/`hq search`.
- **Candidates with immutable provenance:** (a) `repos-hq@4631d19c:hq/lib/walk.mjs#DEFAULT_IGNORES` (line ~14) — blanket `'bin'` entry: every tracked `bin/` directory (repos-hq `hq/bin` incl. `hq/bin/hq.mjs` itself, vivim-web-harness `harness/bin` + `studio/bin`, vivim-labs `lane-d/bin`) was invisible to the index. (b) `repos-hq@9cce014d91187f01d5d6506bb9eeb86e0a1ae111:hq/lib/walk.mjs#DEFAULT_IGNORES` (line 14, `'bin'` removed) with `#sniffBinary` (line 27) as the binary gate inside `#discoverFiles`; `dist`/`build`/`out` remain name-ignored. Consumers: `discoverFiles` → indexer, check/verify, wiki, context.
- **Overlap:** Both candidates aim to keep build artifacts out of the index.
- **Distinct value:** Candidate (b) admits hand-written `.mjs` CLI source (previously ~60 files unindexed) while still excluding actual binary output regardless of directory name; candidate (a) was blind to the exact CLI surface (`hq/bin/*.mjs`) that the wiki and context tools document.
- **Alternatives considered:** keep the blanket ignore (index stays blind to CLI source); whitelist specific `bin/` paths per repo (fragile per-repo configuration drift); move CLI sources out of `bin/` (breaks existing entrypoints and contracts); do nothing.
- **Best-of-breed rubric + hard-gate evaluation:** Correctness/test evidence — new regression fixture `repos-hq@9cce014:hq/test/walk-bin-source.test.mjs` (4 tests, all passing); net +60 indexed files, bringing the four-space post-commit index to ~71.8k symbols (71,761 symbols / 4,579 files, receipt `/tmp/health.json`). Hard gates — no security or source-ownership change; compatibility preserved (entrypoint paths unchanged; `dist`/`build`/`out` still ignored). Unknowns: none material.
- **Decision and rationale:** `sniffBinary` (content sniffing) is the canonical binary exclusion; `'bin'` removed from `DEFAULT_IGNORES`; name-based ignores retained only for genuinely generated output directories.
- **Impact:** Index, wiki and context surfaces now cover the CLI source of all four repos; generated binaries still excluded. No consumer breakage — discovery only grows.
- **Implementation tasks and source-owned work orders:** repos-hq commit `9cce014` — `hq/lib/walk.mjs` (+7/−1), `hq/test/walk-bin-source.test.mjs` (+82, 4 tests).
- **Verification and rollback:** 4/4 tests pass; post-commit index receipt shows the unmasked files/symbols; full-suite receipt 1912 pass / 13 pre-existing fail. Rollback: `git revert 9cce014` (restores blanket `'bin'` ignore). Unverified gate: owner review.
- **Open items linked:** None new.
- **Owner authorization evidence:** None recorded as separate approval; charter compounding mandate (repair CLI read surfaces when they impair assigned work) is the operating authorization. Owner review pending.
- **Supersedes / superseded by:** Supersedes the `DEFAULT_IGNORES` membership of `4631d19c`.

### ZAID-004 — Verify-loop optimization: one prepared-statement round trip per space

- **Round / date / proposer:** R1 / 2026-11-10 / Z.ai
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** `hq check`/`hq verify` freshness loops must not recompile a SQL prepared statement per file; hoist to one round trip per space. Affected users: anyone running freshness checks over large spaces (4,500+ files per full pass).
- **Candidates with immutable provenance:** (a) `repos-hq@4631d19c:hq/bin/hq.mjs#cmdCheck`/`#cmdVerify` — `db.prepare` invoked per file inside the walk loop. (b) `repos-hq@9cce014d91187f01d5d6506bb9eeb86e0a1ae111:hq/bin/hq.mjs#cmdCheck`/`#cmdVerify` — per-file prepared statement hoisted to a single round trip per space (part of the same +21/−7 hunk as ZAID-002). Consumers: same as ZAID-002.
- **Overlap:** Identical freshness semantics; both are implementations of the same check/verify loop.
- **Distinct value:** Candidate (b) avoids N statement compilations per space (one per file → one per space).
- **Alternatives considered:** keep per-file prepare (slower, no semantic difference); cache statements globally across commands (unneeded complexity).
- **Best-of-breed rubric + hard-gate evaluation:** Correctness/test evidence — covered by the ZAID-002 parity fixture (3 tests) and the full-suite receipt (1912 pass / 13 pre-existing fail); behavior unchanged. Hard gates — no authority, security or compatibility impact. Unknowns (honest): no isolated before/after timing benchmark was recorded; the gain is structural (fewer round trips), not a measured number.
- **Decision and rationale:** One prepared statement per space is the go-forward verify-loop implementation.
- **Impact:** Lower per-check overhead; no observable behavior change.
- **Implementation tasks and source-owned work orders:** repos-hq commit `9cce014` — `hq/bin/hq.mjs` (same hunk as ZAID-002).
- **Verification and rollback:** Covered by check-freshness-parity tests + full suite. Rollback: `git revert 9cce014`. Unverified gate: measured performance delta (none recorded).
- **Open items linked:** None new.
- **Owner authorization evidence:** None recorded as separate approval; owner review pending.
- **Supersedes / superseded by:** Supersedes the per-file prepare loop of `4631d19c`.

### ZAID-005 — Wiki self-invalidation: repo-scoped wiki/ ignore for repos-hq only

- **Round / date / proposer:** R1 / 2026-11-10 / Z.ai
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** repos-hq's own generated `wiki/` output (written inside the repo by the HQ wiki generator) must not retrigger index staleness forever, without ignoring tierflow's TRACKED `wiki/` (77 legitimate source files, `git -C tierflow ls-files wiki/` = 77). Affected users: freshness gates, wiki regeneration, anyone cloning repos-hq.
- **Candidates with immutable provenance:** (a) global `config.ignores: ["wiki"]` in repos-hq's HQ config — wrongly excludes tierflow's tracked `wiki/` from the index (77 source files lost). (b) repo-scoped: add `wiki/` to repos-hq's `.gitignore` only — `repos-hq@9cce014d91187f01d5d6506bb9eeb86e0a1ae111:.gitignore` (+4: the `wiki/` entry with the scoping comment "Scoped to this repo: tierflow's wiki/ is tracked source and stays indexed"); `config.ignores` left empty. Consumers: file discovery (`discoverFiles`), freshness walk (now honoring `config.ignores` passthrough per ZAID-002), wiki generator output.
- **Overlap:** Both candidates stop repos-hq's generated wiki output from invalidating freshness.
- **Distinct value:** Candidate (b) is precise — only the generating repo's output is excluded; other repos' tracked wiki content stays indexed. Candidate (a) is a blunt instrument that silently drops 77 tierflow source files (a hard-gate violation: silent capability loss).
- **Alternatives considered:** global `config.ignores` (rejected — collateral loss); move wiki output outside the repo (breaks wiki tooling expectations and repo-relative paths); do nothing (`check --strict` permanently stale → R1 gate unpassable).
- **Best-of-breed rubric + hard-gate evaluation:** Correctness/test evidence — `hq check --strict` PASS on the clean clone at 9cce014 and `hq wiki --check` PASS (~4,676 pages); tierflow wiki still indexed (77 tracked files remain in the index universe). Hard gates — no silent feature loss (tierflow wiki preserved); no authority change. Unknowns: none material.
- **Decision and rationale:** Repo-scoped `.gitignore` entry in repos-hq; `config.ignores` stays empty. Generated output is excluded exactly where it is generated.
- **Impact:** repos-hq's own `wiki/` no longer indexed (it is regenerated output, rebuildable from the index); tierflow's tracked `wiki/` unaffected.
- **Implementation tasks and source-owned work orders:** repos-hq commit `9cce014` — `.gitignore` (+4/−0).
- **Verification and rollback:** strict-check and wiki-check receipts above; rollback: `git revert 9cce014 -- .gitignore`. Unverified gate: owner review.
- **Open items linked:** ZAIQ-005 (tierflow wiki/.hq-tool consumers must still be proven before any retirement).
- **Owner authorization evidence:** None recorded as separate approval; charter R1 freshness mandate is the operating authorization. Owner review pending.
- **Supersedes / superseded by:** Supersedes the unscoped self-invalidation behavior of `4631d19c` (no ignore at all).

### ZAID-006 — Always-on HQ automation: refresh / watch / hooks

- **Round / date / proposer:** R1 / 2026-11-10 / Z.ai (parallel automation lane)
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** Eliminate manual/AI triggering of reindex + wiki regeneration. Landed machinery (all HQ-native, at `bdfe264`): `hq refresh` (lock-protected incremental reindex + stale-only wiki regen + strict gate + `.hq/refresh.log` audit line per space), `hq watch` daemon (debounced `fs.watch` with polling fallback, pause file `.hq/watch.pause`, pid tracking, `--once` deterministic cycle), `hq hooks` (git post-commit/post-checkout/post-merge auto-refresh, idempotent, hooks marked `# vivim-hq-hooks v1`), and automation status surfaced through an informational `hq doctor` section. Affected users: the operating agent every round; freshness/wiki gates.
- **Candidates with immutable provenance:** (a) `repos-hq@4631d19cc` — no native automation: reindex/wiki regeneration triggered manually or by AI turns (status quo being eliminated). (b) `repos-hq@bdfe2644576071a595c0f3fd9a52e76118ccfcb9` (author Z.ai <zai@vivim.invalid>, 2026-11-10T04:41:09Z, directly on `9cce014`) — landed files: `hq/lib/refresh.mjs` (new, +350/−0), `hq/lib/freshness.mjs` (new, +37/−0 — the one freshness walk shared by check/verify/watch/refresh), `hq/test/watch-automation.test.mjs` (new, +378/−0, 10 tests), `hq/bin/hq.mjs` (+308/−62 — `refresh`/`watch`/`hooks` subcommands + doctor Automation section), `hq/lib/automation.mjs` (+89/−17), `hq/test/automation.test.mjs` (+7/−4, 6 tests), `README.md` (+34/−0 — pause-protocol documentation); 7 files, +1203/−83. Observed live after landing: watcher alive (4 spaces, `watch --debounce 400 --wiki-debounce 4000`) under the untracked workspace mini-service `hq-keeper` (supervised child, self-healing restart); hooks 3/3 installed in each of the four repos.
- **Overlap:** Both candidates determine when reindex + wiki regen happen.
- **Distinct value:** Candidate (b) makes triggering continuous, debounced, observable (`doctor`) and pausable (`.hq/watch.pause`), instead of dependent on an agent remembering to run commands; stale-only wiki regen bounds regeneration cost.
- **Alternatives considered:** external cron/systemd timers (orchestration authority outside HQ's observability — conflicts with the charter's HQ-native operating system); git hooks alone (miss non-git file changes); keep manual/AI triggering (status quo — repeated stale index incidents).
- **Best-of-breed rubric + hard-gate evaluation:** Test evidence — `hq/test/watch-automation.test.mjs` 10/10 pass (deterministic `--once` cycles, pause protocol, duplicate-daemon guard, lock deferral); `hq/test/automation.test.mjs` 6/6 pass; R1 parity fixtures still green (`check-freshness-parity.test.mjs` 3/3, `walk-bin-source.test.mjs` 4/4). Full suite at `bdfe264` (single fully-captured run, receipt `/tmp/r1-automation-tests.log`): **1948 tests / 162 files / 99.55s — 1923 pass / 12 fail / 12 skip / 1 todo**, exit non-zero solely from the 12 failures — all members of the known 13-test pre-existing baseline set (ZAIQ-012; the "A2 cross-process race" entry passed this run, confirming that entry as flaky-family); **zero new regressions** versus the `9cce014` receipt (1912 pass / 13 fail). Activation receipts (`/tmp/r1-automation-activation.log`): index 4,584 files / 71,948 symbols; wiki 4,682 pages regenerated incrementally (6 written / 4,676 unchanged); `check --strict` PASS; `wiki --check` PASS; `doctor` exit 0 with Automation section; hooks 12 installed (3 hooks × 4 repos); smoke: watch-triggered refresh 3594 ms (files=1, pages=4, reason=watch) and post-checkout hook refresh 6605 ms (files=1, pages=4, reason=hook:post-checkout). Hard gates verified at landing: no index/wiki write-authority change (same writers, new trigger); watcher/test interplay controlled by the pause protocol (ZAIQ-011/ZAIQ-015); no provider/live effects introduced. Unknowns: supervision across machine restarts (keeper is a dev mini-service, not a system service); hook drift over time.
- **Decision and rationale:** HQ-native always-on automation is the single go-forward trigger authority for reindex + wiki regeneration; manual/AI triggering remains only as fallback. Landed and verified at `bdfe264`.
- **Impact:** Freshness becomes continuously maintained; `refresh.log`/`doctor` add operational observability. Risk to control: concurrent test runs resetting/rebuilding `.hq` (ZAIQ-011, ZAIQ-015) until the isolation guard lands.
- **Implementation tasks and source-owned work orders:** repos-hq commit `bdfe264` — new `hq/lib/refresh.mjs` (+350), `hq/lib/freshness.mjs` (+37), `hq/test/watch-automation.test.mjs` (+378); modified `hq/bin/hq.mjs` (+308/−62), `hq/lib/automation.mjs` (+89/−17), `hq/test/automation.test.mjs` (+7/−4), `README.md` (+34); 7 files, +1203/−83.
- **Verification and rollback:** Test receipts: full suite at `bdfe264` as cited in the rubric (`/tmp/r1-automation-tests.log`); the run used the safe full-suite protocol (real `.hq/` and `wiki/` parked aside via rename, sacrificial copy installed from the `/tmp/hq-backup-r1a` snapshot, restored afterwards) so `kernel-store` could not damage the live workspace (ZAIQ-011). Watcher pause-protocol verification: `watch-automation.test.mjs` fixtures cover pause-file behavior (watcher stays alive and skips refreshes while paused, one full catch-up pass on resume) and hook silence while paused; live operation confirmed the daemon honors `.hq/watch.pause` and that hooks append to `.hq/hooks.log`. Activation + smoke receipts as cited in the rubric. Rollback: `hq hooks remove` (uninstalls only marked hooks) → stop the watcher (kill the pid recorded in `.hq/watch.pid`, or stop the keeper supervisor) → `git revert bdfe264` (single commit; returns to manual triggering; `9cce014` parity fixtures unaffected). Unverified gates: machine-restart supervision; owner review.
- **Open items linked:** ZAIQ-015 (automation operations — pause protocol and keeper supervision live; machine-restart persistence open), ZAIQ-011 (test-workspace isolation guard — reset mechanics documented, guard pending), ZAIQ-012 (12 of 13 pre-existing failures re-confirmed at `bdfe264`; entry 1 flaky-passed).
- **Owner authorization evidence:** None recorded as separate approval; implementation landed and verified at `bdfe264` (receipts in this entry). The charter's compounding-HQ-native-operating-system mandate is the operating authorization. Owner review of this register pending; no product authority, live effects or capability retirement involved.
- **Supersedes / superseded by:** Supersedes manual/AI-triggered reindex+wiki as the default trigger (manual commands remain as fallback; no prior ZAID).

---

## Round 2 (R2 — Semantic convergence) decisions, 2026-11-10

**Numbering note (integrator):** ZAID-007..011 below finalize the L10 records-lane drafts (`/tmp/r2-lane-10/decisions-drafts.md`) with each lane's landed receipts. ZAID-012..017 are lane R2-L5's six concept decisions, renumbered from the lane report's internal draft numbers (L5 drafted them as ZAID-008..013; the canonical register had already assigned 008–011 to L1/L2/L3/L6, so L5's entries paste in as 012–017: concept A→ZAID-012, B→013, C→014, D→015, E→016, F→017). ZAID-018 is the automation-resilience lane (R2-L9) hardening pair. Lane-commit → landed-SHA mapping: L1 `e633e8d` (clone `/tmp/r2-lane-01`) → `74c1f31`; L2 `a15b6c1` (`/tmp/r2-lane-02`) → `0d58260`; L3 (`/tmp/r2-lane-03`) → `dc0784c`; L4 `3d74f9e` (`/tmp/r2-lane-04`) → `0a7b2ff`; L6 (`/tmp/r2-lane-06`) → `2d274c7`. Owner-approval status: PENDING for every R2 entry — these records are non-authoritative evidence.

### ZAID-007 — Wire the VIVIM product ledger into HQ's joined read views (crosswalk edges, explain joins, KB query layer)

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L4)
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** COMPOSE_BEST_OF_BREED
- **Concept/intent and affected users:** ZAIQ-006 — connect distinct atomic requirements, imported capabilities, journeys, product ledger and semantic vocabulary in existing HQ views without ID collision. Affected users: the operating agent (R2+ discovery: one `vivim explain`/`kb query` answers requirement→capability→journey→code chains), plus every `hq kb query` consumer.
- **Candidates with immutable provenance:** (a) `repos-hq@bdfe2644576071a595c0f3fd9a52e76118ccfcb9:hq/lib/vivim.mjs#syncVivim` — loads `journey-capability-crosswalk.v1.json` + `source-file-catalog.json` into `roles`, hashes them into the digest, then never reads them; `project_id` hardcoded NULL. (b) `repos-hq@bdfe264:hq/bin/hq.mjs#cmdVivim(explain)` — single-record field dump, no relationships, no code join. (c) `repos-hq@bdfe264:hq/lib/kb.mjs` — unified read model over doc/decision/canon/session/note/wiki + query-only imported capability documents; VIVIM records invisible. (d) `repos-hq@0a7b2ffc4ff09ac6e1e46d669f432ad23e11b824` (landed; lane commit `3d74f9e142fea07efd5ef3573b9eb8eeb06f182b`) — the landed composition: crosswalk edges in journey payloads, `explainVivimRecord` joins across `vivim_records`/`files`/`symbols`, VIVIM ledger as a namespaced query-only KB layer.
- **Overlap:** all candidates project the same source-owned harness registries into HQ.
- **Distinct value:** (d) is the only one that answers cross-entity questions; (a)–(c) each hold one disconnected fragment.
- **Alternatives considered:** a second relationship/graph store (violates charter: no second graph/database); adding vivim nodes to `kb graph` (rejected this round: changes graph output shape and the query-only invariant; relationships exposed via `vivim explain` instead); registering VIVIM capabilities as `semantic_id` rows (deferred: the semantic registry has no production ingestion path and a global `name@version` namespace would collide with native ids; the `vivim:` KB namespace is collision-free today).
- **Best-of-breed rubric + hard-gate evaluation:** tests — 12 new tests + 79 adjacent green with zero new regressions (lane receipts `/tmp/r2-lane-04-smart-final.txt`); live-store proof — 999/999 capability source-file refs resolve to indexed files; crosswalk 25 journeys → 964 capability / 271 task links materialized. Hard gates — no authority change (read-only projections; product authority stays in the harness space), no security surface (secret redaction path untouched), behavioral compatibility preserved (no-vivim-rows stores byte-identical; pinned test contracts unchanged; graph/export output unchanged). Unknowns: crosswalk matchPolicy is lexical discovery evidence, not execution proof — surfaced verbatim, never promoted.
- **Decision and rationale:** go-forward = (d). Existing tables/mechanisms only; native source ids preserved under a `vivim:` namespace in the KB and verbatim in payloads.
- **Impact:** `vivim sync` result JSON gains `crosswalk` (additive); `vivim explain` output extended, schema `hq.vivim-explain/2`; `kb query` gains an additive result kind; `kb health --json` gains `vivim.records`. No consumer in-tree breaks (verified).
- **Implementation tasks and source-owned work orders:** repos-hq commit `0a7b2ff` — `hq/lib/vivim.mjs` (+197/−7), `hq/lib/kb.mjs` (+99/−3), `hq/bin/hq.mjs` (+34/−4), `hq/test/semantic-convergence.test.mjs` (+345, 12 tests); 4 files, +669/−13.
- **Verification and rollback:** receipts in lane report `/tmp/r2-lane-04-report.md` §5; rollback = `git revert 0a7b2ff` (no schema migration; full-replace resync drops new payload keys).
- **Open items linked:** ZAIQ-006 (progressed — see its R2 delta), ZAIQ-013 (partially progressed: exact-id and requirement-title queries now rank the authoritative record first), ZAIQ-010 (progressed — cross-entity questions answerable with zero raw-file reads).
- **Owner authorization evidence:** none recorded; charter R2 mandate is the operating authorization. Owner review pending.
- **Supersedes / superseded by:** none.

### ZAID-008 — Test-workspace isolation: no test may resolve the operator's real `.hq`

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L1; draft L10)
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** no test — in particular `hq/test/kernel-store.test.mjs`, whose final case spawns `hq reset --yes` at `REPO_ROOT` — may resolve, reset or destroy the operator's real `.hq/` store, gitignored `config.json`, `wiki/` or home receipt file (the R1 destructions are documented in ZAIQ-011). Affected users: every agent running the full suite on the live workspace; CI/automation operators.
- **Candidates with immutable provenance:** (a) status quo at `bdfe264` — `scripts/test-smart.mjs` only SELECTS files and invokes `bun test` at the repo root (no sandboxing); `kernel-store.test.mjs` final case runs `hq reset --yes` against the real workspace via `findRoot()`'s upward walk. (b) `repos-hq@74c1f31f0a45c2c9985775aa78a9e2e23b8e2e02` (landed; lane commit `e633e8d` in `/tmp/r2-lane-01`): two complementary mechanisms — **`HQ_TEST_SANDBOX`** (resolution override honored by `findRoot()` in `hq/bin/hq.mjs`: when set, the sandbox IS the workspace, the upward walk never runs; per-spawn, test side) and **`HQ_TEST_FENCE`** (destructive-reset fence: `scripts/test-smart.mjs` exports `HQ_TEST_FENCE=os.tmpdir()` to the whole `bun test` child; `cmdReset()` refuses with exit 2 *before any rmSync* when the resolved root is outside the fence; runner side, fail-closed); `kernel-store.test.mjs` reset-survival case relocated to a `makeSandboxWorkspace()` temp workspace (kernel DB at `<sandbox>/kernel/kernel.db`, hash taken after lock release + WAL checkpoint — which also fixes pre-existing failure ZAIQ-012 entry 8); `hq/test/workspace-isolation.test.mjs` (new, 249 lines, 7 regression tests incl. an incident replay against a decoy "real" workspace whose recursive tree digest must stay byte-identical); README documents both vars and drops the hazard warning. (c) alternative evaluated and rejected: fence on *resolution* (not just reset) — would break sanctioned tests that read the real workspace read-only (day-one-context comms case); documented scope decision: the fence guards `hq reset` only, the one command that rmSync's pre-existing state; other workspace writers are individually sandboxed by their tests via `HQ_TEST_SANDBOX`.
- **Overlap:** all candidates govern where tests resolve `.hq`.
- **Distinct value:** (b) is defense-in-depth: per-spawn resolution override (test side) + runner-wide destructive fence (runner side) — a naive future test spawning `hq reset` with cwd at the real workspace and no sandbox is still refused.
- **Alternatives considered:** keep the R1 park/restore protocol (manual, error-prone, already failed twice pre-protocol); mark kernel-store test skip-on-live (loses coverage); do nothing (ZAIQ-011 recurrence).
- **Best-of-breed rubric + hard-gate evaluation:** `workspace-isolation.test.mjs` 7/7 in both full-suite runs (lane receipts `/tmp/r2-lane-01-report.md`); the L2 pristine-clone confirmation runs and the R2 final full-suite receipt at `1a115ac` (`/tmp/r2-suite-final.txt`, 1977 pass / 5 fail — all 5 documented pre-existing) ran with the fence active. Full-suite receipts were captured in clones (verify-clone at `1a115ac`) — the guard plus 7/7 regression tests are the live-workspace safety proof; no live full-suite run was needed this round (the R1 park/restore protocol is retired). Hard gates: no source-ownership change; no behavioral change to production `hq reset` outside test env vars. Unknowns: assumption stated in the lane — the operator's real workspace is never under `os.tmpdir()`.
- **Decision and rationale:** canonical = (b). `HQ_TEST_SANDBOX` + `HQ_TEST_FENCE` + per-test sandboxing; the R1 park/restore protocol is retired (documented in README).
- **Impact:** full-suite runs become safe against the live workspace; the ZAIQ-011 self-replenishing trap (cold proof cache always selecting kernel-store) is defused rather than worked around.
- **Implementation tasks and source-owned work orders:** repos-hq commit `74c1f31` — `hq/test/kernel-store.test.mjs` (+68/−), `hq/bin/hq.mjs` (+25/−), `scripts/test-smart.mjs` (+7/−), `hq/test/workspace-isolation.test.mjs` (+249, new), `README.md` (+4/−); 5 files, +327/−26.
- **Verification and rollback:** `bun test hq/test/workspace-isolation.test.mjs` → 7/7 pass (in-suite and full-run receipts cited); rollback = `git revert 74c1f31` (single commit).
- **Open items linked:** ZAIQ-011 (→ RESOLVED_WITH_EVIDENCE), ZAIQ-015 (watcher/test interplay), ZAIQ-012 entry 8 (fixed by the same commit).
- **Owner authorization evidence:** none recorded; charter compounding mandate (stop recurring damage) is the operating authorization. Owner review PENDING.
- **Supersedes / superseded by:** supersedes the unguarded test-workspace resolution behavior of `bdfe264` (and earlier), and retires the R1 park/restore manual protocol.

### ZAID-009 — Pre-existing-failure triage: per-test disposition, deterministic comms-roof join, offline spawn model context

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L2; draft L10)
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** dispose of the 13 pre-existing test failures (ZAIQ-012) per-test with evidence; fix the real defects, re-target the intent-outdated tests, document the environment-dependent remainder. Affected users: every agent reading suite results (13 known failures masked new regressions); comms-roof/channel-evidence consumers.
- **Candidates with immutable provenance:** (a) status quo at `bdfe264` — 13 documented failures (ZAIQ-012 list), reproduced identically on a pristine clone of pinned `4631d19c` (plus the 14th, DEC-MOBILE-001, surfaced in the L2 pristine baseline). (b) `repos-hq@0d582603b4b70ea3f0d09d16b8a2a1c5c049ea71` (landed; lane commit `a15b6c1` on a pristine clone of `bdfe264`): **comms-roof determinism fix** — `probeChannelEvidence`'s `readDir` in `hq/lib/comms-roof.mjs` now sorts the directory listing (raw `readdirSync` order is filesystem-dependent: hash order on Linux overlayfs, sorted on NTFS; the join test observed `items[0]` = R2 CLAIMED instead of R1 VERIFIED) — a production determinism defect, test unchanged; **day-one-context re-targeting** — owner commit `2bf032a` consolidated AGENTS.md (day-one essentials inline, full four-channel contract in `docs/AGENT-BUS.md`): tests re-targeted to both homes, all original protections preserved with current wording (no weakening); **offline spawn model context** — entries 9–13's root cause: `spawn.mjs cmdNew` calls `acquireModelContext()` (owner-machine files `~/.zcode/provider_config.json`, `~/.zcode/provider-depot/catalog.json`, `~/.zcode/cli/db/db.sqlite`, or the `HQ_SPAWN_PROVIDER_CONFIG`/`HQ_SPAWN_DEPOT_CATALOG`/`HQ_SPAWN_USAGE_DB` overrides) before any runner/CLI launch; exit 10 `MODEL_CONTEXT_UNAVAILABLE` on hosts without them; tests now provide the context offline. (c) documented-environment-dependence disposition for entries 2, 6, 7 (+ DEC-MOBILE-001): owner-machine host state (`%APPDATA%\npm\...\zcode-acp-server\dist\cli.js` or `HQ_ACP_SERVER`, `~/.agents/skills/agent-bootstrap/sessions.mjs` or `HQ_SESSIONS_TOOL`, `%ProgramFiles%\ZCode\...` or `HQ_ZCODE_CLI`; the global `~/.agents/AGENTS.md` instruction file; the built phone-surface artifact) — not legitimately creatable in this sandbox (writing stubs at those paths would fake the surfaces the tests exist to verify); required host state documented; env-var seams exist for a future harness.
- **Overlap:** all candidates govern the disposition of the same failure set.
- **Distinct value:** (b)+(c) distinguish real production defects (entry 3), intent-outdated tests (entries 4–5), host-state dependence (2/6/7), offline-context dependence (9–13) and the flaky family (entry 1) — fixing the fixable, documenting the rest with exact required host state.
- **Alternatives considered:** leave all 13 failing (masks regressions — rejected); skip-list them (loses coverage); fix only tests without touching production code (impossible for entry 3 — the nondeterminism was in production).
- **Best-of-breed rubric + hard-gate evaluation:** pristine-clone receipts: baseline `/tmp/r2-lane-02-baseline.txt` (1948 tests, 1921 pass / 14 fail) → final `/tmp/r2-lane-02-final.txt` (1929 pass / 6 fail). Entries 12–13 re-confirmed on the pristine clone as the R1 close condition required, then fixed (offline context). R2 final full-suite receipt at `1a115ac` (`/tmp/r2-suite-final.txt`): **1995 tests / 167 files / 41.04s — 1977 pass / 5 fail / 12 skip / 1 todo**; the 5 = entry 1 (flaky race family), entries 2/6/7 (environment-dependent), DEC-MOBILE-001 (environment-layout; pre-existing at `bdfe264`, L2 baseline receipt line 1365). No change to the join invariant itself (a receipt joins ONLY by requestId; only a real join counts). Hard gates: no security/ownership change. Unknowns: entry 1's race window remains (narrowing is a separate bounded task); entries 2/6/7 + DEC-MOBILE-001 pass on the owner's machine by construction.
- **Decision and rationale:** canonical = (b) + (c): per-entry disposition with evidence — entries 3, 4, 5, 8, 9, 10, 11, 12, 13 FIXED (9 of 13; entry 8 by ZAID-008's commit); entry 1 FLAKY (documented); entries 2, 6, 7 environment-dependent (documented required host state); DEC-MOBILE-001 environment-layout (pre-existing, dispositioned alongside).
- **Impact:** suite failure count 14→5, all 5 dispositioned with documented causes; new regressions become visible immediately.
- **Implementation tasks and source-owned work orders:** repos-hq commit `0d58260` — `hq/lib/comms-roof.mjs` (+4), `hq/test/day-one-context.test.mjs` (+34/−10), `hq/test/spawn-detach.test.mjs` (+36/−3), `hq/test/spawn.test.mjs` (+37/−4); 4 files, +94/−17.
- **Verification and rollback:** receipts cited above; rollback = `git revert 0d58260` (returns entries 3–5, 9–13 to failing).
- **Open items linked:** ZAIQ-012 (→ RESOLVED_WITH_EVIDENCE), ZAIQ-011 (guard used during the pristine confirmations).
- **Owner authorization evidence:** none recorded; charter round mandate (R2 fixes actual code gaps; R1 admission condition) is the operating authorization. Owner review PENDING.
- **Supersedes / superseded by:** supersedes the nondeterministic `probeChannelEvidence` iteration and the unretargeted day-one/spawn tests of `bdfe264`.

### ZAID-010 — Context-assembly ranking: measured relevance improvement over the R2 baseline

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L3; draft L10)
- **Status:** IMPLEMENTED_VERIFIED (with an honest boundary: the 10-question protocol's primary-inclusion metric is unchanged — see ZAIQ-016)
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** `hq context <task>` bundles and `hq search` must rank files that answer the task above off-domain noise. Affected users: every agent consuming context bundles (charter bounded-context objective); ZAIQ-013.
- **Candidates with immutable provenance:** (a) ranking at `bdfe264` — `hq/lib/context.mjs#composeContext` source-selection mix (fts/symbol/path/recent) as observed in the L10 bundle captures: context hit@5 0.10, search hit@5 0.10 on the 10-query bench. (b) `repos-hq@dc0784cdb6b429c4ffa8b583ae0612cff8fcce6f` (landed) — measured ranking changes to the search/context read models (commit-message metrics, same `.hq` store both runs, ground truth locked before the baseline, `scripts/context-bench.mjs`): context hit@5 0.10→0.30, context noise@10 0.91→0.89, context gt recall 0.25→0.30; search hit@5 0.10→0.20 (two previously-empty pages now return best-match OR results, one gaining its ground-truth file in top-5; empty pages scored 0 noise by convention, so the aggregate search noise metric rises 0.78→0.97 even as hit@5 doubles — recorded honestly). (c) alternatives evaluated in-lane: trace-first assembly, project scoping, source-kind reweighting (bench-loser variants; receipts `/tmp/r2-lane-03-baseline.json` / `/tmp/r2-lane-03-after.json`).
- **Overlap:** all candidates assemble deterministic context bundles from the same index.
- **Distinct value:** (b) is the only variant with a locked-ground-truth measured improvement; L4's ZAID-007 additionally ranks exact-id and requirement-title queries to the authoritative vivim record first.
- **Alternatives considered:** do nothing (ZAIQ-013 stays open — rejected; R1 admission condition); widen the token budget (does not fix ranking, raises cost).
- **Best-of-breed rubric + hard-gate evaluation:** measured against the L10 BEFORE baseline (1/6 primary-answer inclusion, ~15–25% on-domain): bench AFTER as in (b); 10-question protocol re-run at `1a115ac` (L10b, `/tmp/r2-lane-10/efficiency-end.md`): coverage held 9/10 FULL / 0 FAILED / 0 raw reads, read-latency median 0.14→0.058 s, but primary-answer inclusion stayed 1/6 — sharpened D1 diagnosis: in 5/6 bundles the primary file is *selected by the ranker then cut by the budget pass* (omitted with reason=budget) → the residual defect is the packing/budget policy, not the ranker. Hard gates: determinism preserved (same task → same bundle; `hq verify` intact); no second context authority. Unknowns: bench vs protocol divergence is query-set-dependent (protocol Q5 search improved to #1 while Q6 trace regressed #1→#13 — L3 effects are query-dependent).
- **Decision and rationale:** canonical = (b)'s scoring on top of the existing `composeContext`; the residual budget-cut omission is scoped to a new open item (ZAIQ-016) rather than claimed fixed.
- **Impact:** better top-5 relevance on representative task queries; empty-query search no longer returns nothing; the protocol's remaining gap is precisely diagnosed for R3.
- **Implementation tasks and source-owned work orders:** repos-hq commit `dc0784c` (search/context read-model ranking; bench `scripts/context-bench.mjs` with locked ground truth); receipts `/tmp/r2-lane-03-{baseline,after}.json`, `/tmp/r2-lane-03-{baseline,after}-run.txt`.
- **Verification and rollback:** bench before/after receipts cited; rollback = `git revert dc0784c`.
- **Open items linked:** ZAIQ-013 (PROGRESSED — bench metrics attached, protocol boundary recorded), ZAIQ-016 (new — budget/packing policy), ZAIQ-010 (protocol is the reusable measurement), ZAID-007 (joint effect).
- **Owner authorization evidence:** none recorded; charter R2 mandate ("semantic convergence") is the operating authorization. Owner review PENDING.
- **Supersedes / superseded by:** supersedes the source-selection scoring of `composeContext` at `bdfe264`.

### ZAID-011 — Toolchain guard: bun >= 1.4 and lockfile-format assertion at tool entry

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L6; draft L10)
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** repo tooling must refuse to run under bun < 1.4 and must assert the `bun.lock` v2 format before tooling proceeds, so the silent-downgrade incident (bun 1.3.14 rewrote `bun.lock` v2→v1, breaking installs; recovered via `git checkout`; machine on bun 1.4.4) cannot recur silently. Affected users: every operator/agent running HQ or Labs tooling on any machine.
- **Candidates with immutable provenance:** (a) status quo at `bdfe264` — no enforced guard in repos-hq; `vivim-labs` `package.json` engines advisory only. (b) `repos-hq@2d274c748bd6ee2e3b1aa16cb6b6e5e1ca62a3f9` (landed): `hq/lib/toolchain-guard.mjs` (new, 133 lines) — shared guard, refuses bun < 1.4.0 (exit 2, message names required + detected version) and a `bun.lock` with the legacy v1 text signature (exit 3, remedy: `git checkout bun.lock`); numeric version compare on [major, minor, patch]; wired at `hq/bin/hq.mjs` entry (+8) AND `scripts/test-smart.mjs` entry (+5, same shared module); `hq/test/toolchain-guard.test.mjs` (new, 200 lines, 13 tests); README documents the guard. (c) alternative evaluated: CI-only check (rejected — the incident happened locally; the guard must be at tool entry).
- **Overlap:** all candidates prevent under-version toolchain execution.
- **Distinct value:** (b) is enforced and fail-closed (vs advisory engines); shared module = one implementation for both entry points.
- **Alternatives considered:** advisory engines only (status quo — already bypassed once); documentation-only guard (R1 interim); do nothing.
- **Best-of-breed rubric + hard-gate evaluation:** guard tests 13/13 pass in-suite (`hq/test/toolchain-guard.test.mjs`); every subsequent full-suite receipt (incl. the final `1a115ac` run) executed with the guard active — demonstrably silent under a compliant toolchain; install reproducibility: `bun install` on bun 1.4.4 reproduces from `bun.lock` v2 (L2/L6 pristine-clone runs each installed 264 packages cleanly). Hard gates: fail-closed, no silent mutation of `bun.lock`; no behavioral change under a compliant toolchain. Unknowns: CI matrix coverage (none exists).
- **Decision and rationale:** canonical = (b): enforced guard at every tool entry (CLI + smart runner) via the shared module.
- **Impact:** the bun-downgrade incident class is eliminated at entry; fresh-clone operators get an actionable refusal instead of a corrupted lockfile.
- **Implementation tasks and source-owned work orders:** repos-hq commit `2d274c7` — `hq/lib/toolchain-guard.mjs` (+133, new), `hq/test/toolchain-guard.test.mjs` (+200, new), `hq/bin/hq.mjs` (+8), `scripts/test-smart.mjs` (+5), `README.md` (+2); 5 files, +348.
- **Verification and rollback:** `bun test hq/test/toolchain-guard.test.mjs` → 13/13; rollback = `git revert 2d274c7`.
- **Open items linked:** ZAIQ-014 (→ RESOLVED_WITH_EVIDENCE), ZAIQ-019 (new — D5 doctor node-version display stays open, deliberately out of this guard's scope).
- **Owner authorization evidence:** none recorded; charter compounding mandate + R1 admission risk list is the operating authorization. Owner review PENDING.
- **Supersedes / superseded by:** supersedes the advisory-only toolchain constraint of `bdfe264`.

### ZAID-012 — UI publisher/revision stores: no dual writers; cutover deferred behind the D6 fence proof (ZAIQ-004)

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L5)
- **Status:** PROPOSED
- **Decision class:** DEFER_PENDING_PROOF (with KEEP_INDEPENDENT semantics for the interim: Harness bootstrap ledger remains the sole live publisher; TierFlow UiStore remains unmounted)
- **Concept/intent and affected users:** which store may commit Founder Workspace UI revisions. Affected: phone/PC users of the Build surface; Harness Host; TierFlow visual runtime.
- **Candidates with immutable provenance:** (a) `vivim-web-harness@7372310:apps/vivim-host/bootstrap/ledger.mjs#openLedger` (uiRevisions binding line 404; insert line 167; activate via `UPDATE … active`), `…/bootstrap/service.mjs#applyUiProposal` (line 622), `…/bootstrap/runtime.mjs#createBootstrapRuntime`; consumers `server.mjs`, `receipt-runner.mjs`, `direct-phone.mjs`, `command-worker.mjs`; tests `ui-revision.test.mjs` et al. (b) `tierflow@5c44d14:src/ui/store.ts#openUiStore` (line 1163) + `src/ui/commands.ts#UiCore` (line 254); consumers tests only (`impact src/ui/store.ts`: 9 dependents, none production); `src/ui/{realtime,surfaces}` unmounted (GAP-001 404s). Internal design pins: Mission D audit (`tierflow@60c55e0…`, `harness@0741420…`, `hq@36f689a…`) and ADVANCE-6A (`tierflow@8dab853…`, `harness@09382c6…`).
- **Overlap:** SQLite revision histories keyed `(world_id, revision)` with active pointer, publish/undo, idempotency; CAS publication in both designs.
- **Distinct value:** (a) grant-checked writer + live phone/PC path + Host approvals/receipts; undo-reactivation (with the recorded active-vs-max collision hazard). (b) world-scoped `ui_current` pointer, WAL/FULL, retention/audit/store locks, typed command port, closed `/api/ui/*`+WS epoch contracts — all test-covered, none routed.
- **Alternatives considered:** cut over now (BLOCKED: no durable fence exists in either — ADVANCE-6A conflict #3; canonical 2D vs spatial schema migration BLOCKED; I5 GO absent); dual-write during transition (forbidden — Mission D conflict #4); retire the Harness writer (forbidden — ZAI-BASE-003 + negative-consumer proof absent); do nothing unrecorded (leaves ZAIQ-004 DECISION_REQUIRED open).
- **Best-of-breed rubric + hard-gate evaluation:** capability coverage: both stores complete for their scope (Harness live path; TierFlow durability semantics); correctness: Harness live tests + historical TierFlow suites (1,046 pass at FINAL-WAVE pin — not rerun at head; UNVERIFIED at current SHA); architecture/authority: single-writer invariant satisfied today (one live writer, one unmounted store); security: grant checks live-side only; TierFlow publishes nothing. Hard gates: no dual writers (satisfied by decision), no silent capability loss (both retained), behavioral compatibility untouched.
- **Decision and rationale:** Harness bootstrap ledger is the sole live UI publisher until a separately authorized, owner-approved cutover passes the ADVANCE-6A transition table. TierFlow UiStore stays unmounted and independently tested. Never dual-write, never infer one revision history from the other, never reuse an issued revision number after rollback. Cutover preconditions (the proof still missing): durable cross-implementation fence/epoch (absent in both), 100% active-caller classification with runtime traces at every ingress, `vivim.ui-surface/1` ↔ `vivim.ui-document/1` bijection proof at 320/360/1280 with a11y/privacy/rollback, lossless World→canonical migration (currently BLOCKED), failure-injection matrix F01–F28, physical device + owner gates (G5/G6/G7).
- **Impact:** no code change now; blocks any "cleanup" that would touch `generative-ui`, `src/ui`, or the Host bootstrap routes; retirement of legacy writers is a later, separately approved caller-verified task.
- **Implementation tasks and source-owned work orders:** none opened by this decision (design exists: ADVANCE-6A/6B); future work orders must cite this ZAID and gate D6.
- **Verification and rollback:** verification = the ADVANCE-6A preflight assertions + F01–F28 matrix + I6 gates; rollback = the ADVANCE-6A §9 rollback drill. Unverified gates: everything listed above; explicitly no current-head test rerun was performed by this lane.
- **Open items linked:** ZAIQ-004 (→ proposed delta below), ZAIQ-008 (progressed for this concept).
- **Owner authorization evidence:** none yet; owner approval required before any cutover execution. This entry proposes the *decision*, not the cutover.
- **Supersedes / superseded by:** none (first decision record for this concept; ZAI-BASE-003 remains the governing policy).

### ZAID-013 — One wiki generator: HQ `wiki.mjs`; tierflow `wiki/` = stale projection to retire only after ZAIQ-005 proof; harness catalog generators operational until HQ parity

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L5)
- **Status:** PROPOSED
- **Decision class:** SELECT_CANONICAL (component dispositions: tierflow wiki/ RETIRE_AFTER_MIGRATION; harness tools/docs KEEP_AS_ADAPTER)
- **Concept/intent and affected users:** derived documentation/index generation across repos. Affected: all HQ consumers (wiki views, freshness gates) and every reader of tierflow's tracked `wiki/`.
- **Candidates with immutable provenance:** (a) `repos-hq@bdfe264:hq/lib/wiki.mjs` (+`wiki-program.mjs`, `wiki-runtime/graph-index.mjs#hqg-exchange`, `comment-wiki-bridge.mjs`; CLI `hq wiki`); tests `watch-automation.test.mjs` 10/10; R1 receipts ~4,682 pages. (b) `tierflow@5c44d14:wiki/**` (77 tracked files; generated 2026-11-02 by installed `.hq-tool` bundle v1.0.0 — 46 files/1,453,578 bytes, no `sourceSha` — vs HQ v1.1.1 57 files; covers 54 files/1,089 symbols/0 commits vs 635 current blobs; Mission D T25 PROJECTION/STALE). (c) `vivim-web-harness@7372310:tools/docs/reindex.mjs` + `check-docs.mjs` + `generate-work-queue.mjs` (regenerates `docs/capabilities/{atomic-registry,source-symbol-index,source-file-catalog}.json`, `document-catalog.json` 423 active blob-pinned entries). (d) `vivim-labs@316fa5f:documentation-consolidation/**` — historical relocation archive (137/137 exact-blob reconciled; 1,097 retained).
- **Overlap:** (a)/(b) are the same generator at different versions; (a)/(c) both index files/symbols into derived artifacts with freshness expectations.
- **Distinct value:** (a) cross-repo scope, per-page hashes, graph-linked views, automation; (c) product capability/criticality catalogs tied to the roadmap with blob pinning — a class HQ does not yet emit; (b) none (stale copy).
- **Alternatives considered:** keep tierflow wiki as hand-maintained source (rejected: it is generated output; hand-editing generated pages is the known anti-pattern — Mission D T25 "rebuild after verifying CLI, do not hand-edit"); have HQ emit harness catalogs now (premature: parity undefined); delete tierflow wiki/ today (forbidden: ZAIQ-005 proof chain incomplete).
- **Best-of-breed rubric + hard-gate evaluation:** HQ wiki: live tests + freshness gate + automation receipts (HIGH). Tierflow wiki: provenance verified stale (HIGH confidence in staleness; consumers UNVERIFIED — ZAIQ-005). Harness reindex: operational, consumed by its own gates (MED — parity boundary undefined). Hard gates: no silent loss (77 files retained until proof); no second generator authority.
- **Decision and rationale:** `hq/lib/wiki.mjs` is the single wiki generator (confirms ZAI-BASE-002 with evidence). Tierflow's checked-in `wiki/` is marked generated/stale and may be retired only after: verified `.hq-tool` install/rebuild chain (pack-single → verify-bundle → install-single) at compatible versions, a recorded regenerated diff, enumerated consumer migration, and rollback proof. Harness `tools/docs` stays as an operational exception until HQ emits equivalent capability/catalog views.
- **Impact:** protects tierflow wiki/ from premature deletion; protects HQ wiki from being re-implemented; leaves harness tooling intact.
- **Implementation tasks and source-owned work orders:** ZAIQ-005 proof chain (existing); no new work orders from this entry.
- **Verification and rollback:** verification = ZAIQ-005 closure evidence; rollback = restore tracked wiki/ from git (trivial, since no deletion may happen before proof).
- **Open items linked:** ZAIQ-005, ZAIQ-008.
- **Owner authorization evidence:** none needed beyond existing policy; deletion steps require the G-lane supervisor per Mission H.
- **Supersedes / superseded by:** none; operationalizes ZAID-005's scoping for the tierflow side.

### ZAID-014 — Graph authority: HQ SQLite dependency graph is the live read authority; Rust HQKG stays the compute sidecar behind the exchange; harness test-graph stays repo-local

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L5)
- **Status:** PROPOSED
- **Decision class:** SELECT_CANONICAL (with KEEP_AS_ADAPTER for the Rust sidecar and KEEP_INDEPENDENT for the harness test graph)
- **Concept/intent and affected users:** code-graph facts for dependency traversal, ranking and context. Affected: every `hq deps/impact` consumer and the wiki graph views.
- **Candidates with immutable provenance:** (a) `repos-hq@bdfe264:hq/lib/graph.mjs` (191 ln; `deps`, `impact`, `projectGraph`; test `graph.test.mjs`; live CLI verification during this lane). (b) `repos-hq@bdfe264:research/rust-knowledge-graph/src/lib.rs` (+modules; historical receipts: cargo test 52 passed; import 60,424 nodes/121,388 edges; **not built at head — liveness UNVERIFIED**). (c) `repos-hq@bdfe264:hq/lib/live-graph/wave1/lane-a.mjs` (+`lane-a-r2-{audit,discovery}.mjs`; consumers `hq/bin/wave1-lane-a.mjs`, `live-graph-wave1-a.test.mjs`). (d) `repos-hq@bdfe264:hq/lib/wiki-runtime/graph-index.mjs` (consumer: `hq/bin/hq.mjs`). (e) `vivim-web-harness@7372310:test/graph.mjs` (310 ln; `wh test --affected`).
- **Overlap:** (a)–(d) compute dependency-graph facts over the same repos; (e) computes file import edges for one repo.
- **Distinct value:** (a) zero-build live SQL read model; (b) traversal/ranking compute, recovery journal, reconcile/invalidate, web-memory; (d) SHA↔node↔semantic linkage; (e) affected-test selection.
- **Alternatives considered:** promote Rust to graph authority (dead: DE-01/DS-01 — threshold 10x unmet, single disputed measurements 0.398x vs 3.664x); delete the Rust crate (forbidden: authorized research + exchange producer); merge the harness graph into HQ (different purpose — test selection, not knowledge).
- **Best-of-breed rubric + hard-gate evaluation:** division of labor is pre-agreed by three governing docs (ROUND-2-DESIGN "Rust owns graph compute/journal, HQ SQLite retains canonical/source-index/FTS/KB"; GRAPH-KERNEL-UNIFICATION "Canonical owner: Repos HQ"; owner-matrix "no second graph authority"). This entry adds the current-head liveness facts: (a) live-verified; (b) UNVERIFIED (no `target/`); (e) live via harness tooling.
- **Decision and rationale:** confirm the existing division: HQ SQLite (via graph.mjs) = live read authority; Rust HQKG = non-authoritative compute sidecar, consumed only through `repos-hq.hqkg-exchange/1`; harness keeps its per-repo test-selection graph. No promotion of Rust without DS-01's 10x proof.
- **Impact:** none on current behavior; guards against re-litigating DE-01.
- **Implementation tasks and source-owned work orders:** none new; R6 fresh-clone acceptance must decide whether rebuilding the Rust sidecar is mandatory (see risk).
- **Verification and rollback:** verification = graph parity tests (ZAIQ-002, still open); rollback = N/A (no change).
- **Open items linked:** ZAIQ-002 (graph parity), ZAIQ-008.
- **Owner authorization evidence:** policy ZAI-BASE-002 already covers the backbone; this records the classification evidence.
- **Supersedes / superseded by:** none.

### ZAID-015 — Channel/bridge transports stay independent: HQ seats for AI sessions, Harness for browser/provider/phone; the DEAD-ENDS register is the boundary

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L5)
- **Status:** PROPOSED
- **Decision class:** KEEP_INDEPENDENT
- **Concept/intent and affected users:** agent/seat intent transport with journals, approvals, receipts. Affected: HQ seat users; Harness phone/Studio/provider users.
- **Candidates with immutable provenance:** (a) `repos-hq@bdfe264:hq/lib/channel-{transport,broker,seats,wake}.mjs` (+`seat-watch.mjs`, `seat-heartbeat.mjs`; CLI `hq seat …`, `hq channel watch`; tests incl. the ZAIQ-012 #2/#3 environment-dependent family). (b) `vivim-web-harness@7372310:harness/src/**` (core CDP/journal/send; interface server/router/standing/approval; providers; agents; provenance/ledger.mjs), `studio/src/studio/{browser-companion/ingress,bridge-commands}.mjs`, `studio/src/capture/conversation-service.mjs`, `apps/mobile-relay/{api,lib,transport,control-phone}*.mjs`, `apps/vivim-host/bootstrap/command-worker.mjs`.
- **Overlap:** seats/sessions, request journals with idempotency, approvals, receipts, wake/dispatch.
- **Distinct value:** (a) cross-repo AI-seat orchestration over OpenCode/zcode ACP with event-driven wake → bus; (b) real CDP control of the single dedicated Chrome (loopback 9333), provider profiles, send-once journal, `wh approve` interactive approvals, phone relay pairing, Studio `expectedRevision` CAS.
- **Alternatives considered:** use `wh` as the agent send path (DE-03 dead); harness bridge worker as agent channel (DE-04 dead); `relay-chat-mvp` without `--studio-url` (DE-05 dead); ANet as ChatGPT channel (DE-06 dead); treat Studio `confirm:true` as approval (DE-15 dead). Merge the families (rejected: violates the standing invariant "one browser owner, one effect owner, one logical KB writer").
- **Best-of-breed rubric + hard-gate evaluation:** the 15-entry dead-end register with per-item evidence paths is the strongest available boundary evidence; reopen conditions are recorded per item. Security hard gate: the DE-15 reopen condition (a real Studio approval record) is unimplemented — so no agent channel may reuse the Studio endpoint today.
- **Decision and rationale:** no transport merges. HQ may render harness journal/receipt state as read-only views only (evidence-join). Any future agent channel must satisfy DE-15's reopen condition before reusing Studio's endpoint.
- **Impact:** none now; prevents wasted work on known-dead merges.
- **Implementation tasks and source-owned work orders:** none new; ZAIQ-012 #2/#3 remain the verification debt for HQ seats.
- **Verification and rollback:** verification = ZAIQ-012 #2/#3 closure (bridge entry point + receipt join on this host); rollback = N/A.
- **Open items linked:** ZAIQ-012, ZAIQ-008.
- **Owner authorization evidence:** not required (status-quo confirmation with evidence).
- **Supersedes / superseded by:** none.

### ZAID-016 — Session/ledger/receipt stores stay domain-separated; joins only as HQ read-only views

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L5)
- **Status:** PROPOSED
- **Decision class:** KEEP_INDEPENDENT
- **Concept/intent and affected users:** durable session/claim/receipt records per domain. Affected: HQ dev-intelligence consumers; Harness Host/Studio/phone; Labs governance.
- **Candidates with immutable provenance:** HQ: `hq/lib/sessions.mjs` (`SESSION_SCHEMA`; import/absorb + views), `hq/lib/kernel/store.mjs` (durable kernel, survives `hq reset`), `hq/lib/spawn.mjs#SPAWN_LEDGER`, `hq/lib/receipts.mjs` + `channel-broker.mjs#RECEIPT_SCHEMA` + `evidence-join.mjs`. Harness: `apps/vivim-host/bootstrap/ledger.mjs#openLedger` (Personal World: claims/observations/…/ui_revisions/receipts), `studio/src/store/session.mjs#KeyValueStore` + `conversation-service.mjs` (captured sessions), `apps/vivim-host/phone-sessions.mjs`, `receipt-runner.mjs` + `docs/receipts/`. TierFlow: `docs/SESSION-LEDGER.md` (historical halt record), `src/visual-runtime/revisions/wave4-d-session-reconcile.ts` (test adapter). Labs: `zai/governance/*` (evidence only).
- **Overlap:** append-oriented ledgers with ids/status/evidence; several record conversations; several record measured activity.
- **Distinct value:** each owns a domain no other can cover (see Concept E); the merge directions are recorded dead ends (DE-11 Host-as-KB-writer; DE-10 second global ledger).
- **Alternatives considered:** Host ledger as KB (DE-11 dead); a second global TODO/status ledger (DE-10 dead); a cross-repo session store (rejected: violates source ownership).
- **Best-of-breed rubric + hard-gate evaluation:** domain separation is enforced by contract docs (kernel-store contract; SHARED-KB-KERNEL-ARCHITECTURE) and registers. Kernel wiring (GAP-025/DE-09) remains in flight under L4/ZAID-007 — not duplicated here.
- **Decision and rationale:** no store merges; cross-domain joins only as HQ read-only views (`hq receipts`/evidence-join); the Host bootstrap ledger remains Personal World, never KB; Labs governance remains non-authoritative evidence.
- **Impact:** none now; guards DE-10/DE-11 from reopening.
- **Implementation tasks and source-owned work orders:** none new (L4 owns kernel wiring).
- **Verification and rollback:** verification = ZAIQ-011 guard (kernel-store isolation) + ZAID-007 (L4); rollback = N/A.
- **Open items linked:** ZAIQ-011, ZAIQ-008.
- **Owner authorization evidence:** not required (status-quo confirmation with evidence).
- **Supersedes / superseded by:** none.

### ZAID-017 — Per-repo test selection and work queues stay source-owned; HQ stays the cross-repo view layer

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (lane R2-L5)
- **Status:** PROPOSED
- **Decision class:** KEEP_INDEPENDENT
- **Concept/intent and affected users:** affected-test selection and work-order queues. Affected: CI/dev loops in each repo.
- **Candidates with immutable provenance:** test runners: `repos-hq@bdfe264:scripts/test-smart.mjs` (proof cache; ZAIQ-011 trap), `vivim-web-harness@7372310:test/{graph,select}.mjs` + `harness/src/build/review.mjs` (`wh test --affected`; WH-TEST-AFFECTED-DESIGN.md), `tierflow@5c44d14` `bun test tests/` + `bench/`. Work orders: `repos-hq@bdfe264:hq/lib/work-orders.mjs` (consumers `hq/bin/autonomy.mjs`, `hq/bin/hq.mjs`), `vivim-web-harness@7372310:docs/agents/{WORK-QUEUE.md,generate-work-queue.mjs#generateQueue}`, `tierflow@5c44d14:VIVIM-VISUAL-RUNTIME/**` (historical).
- **Overlap:** dependency-graph-based test selection; queue generation from capability maps.
- **Distinct value:** each is tuned to its repo's roots/languages; HQ's is the only one with cross-repo impact data; the harness's is the only one with a product capability map; TierFlow's waves are immutable history.
- **Alternatives considered:** one shared test runner (rejected: charter forbids an alternative planner; roots/languages differ; per-repo tooling is not a second authority); a shared work-order queue (rejected: DE-10).
- **Decision and rationale:** per-repo tooling stays source-owned; HQ renders cross-repo views only; TierFlow wave docs are historical work orders and never imply current production GO.
- **Impact:** none.
- **Open items linked:** ZAIQ-011 (repos-hq runner trap).
- **Supersedes / superseded by:** none.

---

### ZAID-018 — Watch-backend self-healing + wiki hash-cache integrity (automation hardening pair)

- **Round / date / proposer:** R2 / 2026-11-10 / Z.ai (automation-resilience lane R2-L9)
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** the always-on automation (ZAID-006) must survive backend failures without burning CPU or poisoning its derived state, and `wiki --check` must never damage the cache it verifies. Affected users: the watcher daemon (all four spaces), every `wiki --check` consumer, the freshness gates.
- **Candidates with immutable provenance:** (a) status quo pre-fix: **R2 incident** — the daemon's recursive `fs.watch` hit inotify ENOSPC (`fs.inotify.max_user_watches=8192` exhausted while walking `node_modules`), emitted an async `'error'` that the R1 code swallowed in a no-op handler; the runtime's internal retry loop then burned one full core for hours (the D7 ~91% CPU finding, root-caused) while the space had zero watch coverage. Separately: `wiki --check`'s force build into a throwaway dir upserted input hashes for content never written to the canonical wiki (poisoning the hash cache so later incremental regens skipped stale pages); the incremental generator never deleted orphaned pages (removed files left pages + rows forever); a zero-delta reindex unconditionally bumped `projects.indexed_at`, churning project wiki pages while the zero-delta watch path schedules no wiki settle. (b) `repos-hq@3d5eee6dbad2e2ec4a4457e10f3f8f6ecf5f6c9` — watch self-healing: an async watcher error now closes the broken watcher, counts `stats.backends.errored`, emits a `watch-error` event and engages the snapshot-poll fallback for every space (sync throws already did); `stats.backends = {native, errored, polling}` exposes backend state; `SNAPSHOT_IGNORES` gains `'wiki'` (a poll backend cannot feed back into its own wiki regeneration); `cmdWatch` records a red watch timing receipt + `.hq/watch.log` audit line on watch-error, the daemon start line records `wikiDebounce`, and the 5s pause detector is deliberately reffed (in pure-poll mode it is the only keepalive holding the event loop); `hq/test/watch-backend.test.mjs` (new, 4 fixtures: async error self-healing, sync throw fallback, wiki churn ignored under polling, healthy native coverage); `hq/CONTRACT.md` + README document the backend contract. (c) `repos-hq@1a115ac3b042c9bad21eafdd5b9a33166c63d9fc` — wiki hash-cache integrity: `buildWiki` gains `persistHashes` (default true; the check path and `--out` exports never persist — check builds can never poison); canonical builds walk the generator-owned wiki dir and delete any file the current page set does not declare (orphans deleted, emptied dirs removed best-effort; result schema gains `'removed'`); `indexed_at` bumps only when the index actually churned (files added/changed/removed, head/branch moved, counts changed — zero-delta reindexes do not churn); regression fixtures in `wiki.test.mjs` (cache poisoning, orphan cleanup) + `indexer.test.mjs` (zero-delta no-bump + real-change bump).
- **Overlap:** both commits govern automation resilience of the ZAID-006 machinery.
- **Distinct value:** (b)/(c) make failures loud, self-healing and state-safe: broken backend → poll fallback + receipts; check builds → never write cache; orphans → deleted; zero-delta → no churn.
- **Alternatives considered:** raise `max_user_watches` system-wide (owner sysadmin territory, not portable); suppress the async error (the R1 behavior — the incident); delete orphans manually (recurring burden).
- **Best-of-breed rubric + hard-gate evaluation:** `watch-backend.test.mjs` 4/4 + `wiki.test.mjs`/`indexer.test.mjs` regression fixtures pass (final full-suite receipt at `1a115ac`: 1977 pass, zero new regressions); live verification: post-restart watcher (pid 13714, started 06:40:00Z) measured at ~3.8% CPU (D7 fixed-as-measured; the ENOSPC self-healing path itself was never exercised live — no ENOSPC event was logged after the restart, recorded honestly); `hq check --strict` PASS and `hq wiki --check` PASS at close with 0 index drift at rest. Hard gates: no trigger-authority change; poll fallback is read-path parity, not a second writer. Unknowns: the ENOSPC self-heal path is fixture-proven, not live-proven.
- **Decision and rationale:** canonical = (b)+(c): fail-loud + fallback + cache-integrity semantics for the automation machinery.
- **Impact:** the watcher can no longer silently burn a core with zero coverage; `wiki --check` is side-effect-free; watcher restarts no longer redden the wiki gate.
- **Implementation tasks and source-owned work orders:** repos-hq commits `3d5eee6` (`hq/lib/automation.mjs` +35, `hq/bin/hq.mjs` +22, `hq/test/watch-backend.test.mjs` +154 new, `hq/CONTRACT.md` +16, `README.md` +2; 5 files, +215/−14) and `1a115ac` (`hq/lib/wiki.mjs` +67, `hq/lib/indexer.mjs` +21, `hq/bin/hq.mjs` +13, `hq/test/{wiki,indexer}.test.mjs` +100 combined; 5 files, +193/−8).
- **Verification and rollback:** receipts cited; rollback = `git revert 1a115ac 3d5eee6` (independent commits).
- **Open items linked:** ZAIQ-015 (PROGRESSED — D7 root-caused and fixed-as-measured, D8 improved: watch.log now emits paused/resumed + wikiDebounce), ZAID-006 (the machinery this hardens).
- **Owner authorization evidence:** none recorded; charter always-on-automation mandate is the operating authorization. Owner review PENDING.
- **Supersedes / superseded by:** supersedes the swallowed-async-error watcher and the cache-writing check path of `bdfe264`.
