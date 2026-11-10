# Strategic decision register — campaign evidence only

**Purpose:** Explain *where and why* Z.ai makes architecture or implementation selections. Append new decision entries; do not rewrite prior rationale. Owner approvals and operational status live in the existing authoritative system, not here.

## Required entry template

### ZAID-YYYYMMDD-NNN — Short concept title

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

## Initial policy decisions (owner-provided, not Z.ai architecture inventions)

- **ZAI-BASE-001:** Exactly four primary VIVIM repositories; `vivim-ui` is local folder alias for GitHub `tierflow`.
- **ZAI-BASE-002:** Existing HQ indexing, Rust graph and wiki generation are the single cross-repo development intelligence backbone.
- **ZAI-BASE-003:** Harness retains live effects, Host, one Android app and current product roadmap authority pending verified migration.
- **ZAI-BASE-004:** Labs archives/evidence are public-safe only; no separate live capability/roadmap authority.
- **ZAI-BASE-005:** Prefer composed best-of-breed solutions and preserve unique capabilities before retiring duplicate code.

These are scope policies; future ZAID decisions must carry candidate evidence, authorization and tests.

## Round 1 (R1 — Working HQ) decisions, 2026-11-10

Numbering starts at ZAID-001: no prior ZAID records exist (ZAI-BASE-001..005 above are owner-provided policies). R1 implementation landing commit: `repos-hq` `9cce014d91187f01d5d6506bb9eeb86e0a1ae111` (author Z.ai <zai@vivim.invalid>, 2026-11-10T03:46:21Z), on pinned baseline `4631d19cc57a07fa532e627f21737e489d629055`. ZAID-006 is in flight in a parallel lane; its final SHA and test counts are PHASE2 placeholders — do not treat them as final.

### ZAID-001 — Re-register the four VIVIM identities onto clean-clone paths

- **Round / date / proposer:** R1 / 2026-11-10 / Z.ai
- **Status:** IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** HQ's project registry must resolve exactly the four canonical VIVIM spaces (ZAI-BASE-001) at routable local paths so every HQ consumer (index, check/verify freshness, wiki, find/search, context assembly) can operate on this clean-clone workspace. Affected users: Z.ai operating agent and all HQ CLI read surfaces.
- **Candidates with immutable provenance:** (a) `repos-hq@4631d19cc57a07fa532e627f21737e489d629055:projects.json` — five entries, every path Windows-style `C:\0-BlackBoxProject-0\…` (repos-hq, vivim-labs, vivim-ui, vivim-web-harness, vivim-workstation); unroutable on this Linux workspace, i.e. all 5 spaces lost to local HQ operations. (b) `repos-hq@9cce014d91187f01d5d6506bb9eeb86e0a1ae111:projects.json` — four entries: `repos-hq` → `.`, `vivim-labs` → `/home/z/my-project/repos/vivim-labs`, `vivim-ui` → `/home/z/my-project/repos/tierflow` (local alias of GitHub `tierflow`), `vivim-web-harness` → `/home/z/my-project/repos/vivim-web-harness`. Consumer: HQ project resolution across index/check/wiki/context.
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

### ZAID-006 — Always-on HQ automation: refresh / watch / hooks [IN FLIGHT — PHASE2]

- **Round / date / proposer:** R1 / 2026-11-10 / Z.ai (parallel automation lane)
- **Status:** PROPOSED
- **Decision class:** SELECT_CANONICAL
- **Concept/intent and affected users:** Eliminate manual/AI triggering of reindex + wiki regeneration. Proposed machinery (all HQ-native): `hq refresh` (incremental reindex + stale-only wiki regen, `refresh.log`), `hq watch` daemon (debounced `fs.watch` with polling fallback, pause file `.hq/watch.pause`, pid tracking), `hq hooks` (git post-commit/post-checkout/post-merge auto-refresh), and automation status surfaced through `hq doctor`. Affected users: the operating agent every round; freshness/wiki gates.
- **Candidates with immutable provenance:** (a) `repos-hq@4631d19cc` — no native automation: reindex/wiki regeneration triggered manually or by AI turns (status quo being eliminated). (b) repos-hq automation lane, IN FLIGHT at records time — working-tree files `hq/lib/automation.mjs` (modified), `hq/test/automation.test.mjs` (modified), `hq/test/watch-automation.test.mjs` (new), `hq/bin/hq.mjs` (modified); observed live: watcher alive, 4 spaces, args `watch --debounce 400 --wiki-debounce 4000` (`/tmp/health.json`, 2026-11-10T03:59Z). <!-- PHASE2: final commit SHA + immutable provenance for landed files -->
- **Overlap:** Both candidates determine when reindex + wiki regen happen.
- **Distinct value:** Candidate (b) makes triggering continuous, debounced, observable (`doctor`) and pausable (`.hq/watch.pause`), instead of dependent on an agent remembering to run commands; stale-only wiki regen bounds regeneration cost.
- **Alternatives considered:** external cron/systemd timers (orchestration authority outside HQ's observability — conflicts with the charter's HQ-native operating system); git hooks alone (miss non-git file changes); keep manual/AI triggering (status quo — repeated stale index incidents).
- **Best-of-breed rubric + hard-gate evaluation:** Test evidence: <!-- PHASE2: automation lane test counts + exits -->. Hard gates to verify at landing: no index/wiki write-authority change (same writers, different trigger), watcher must not fight full test-suite runs (see ZAIQ-011/ZAIQ-015 pause protocol), no provider/live effects introduced. Unknowns: supervision across machine restarts; hook drift.
- **Decision and rationale (proposed):** HQ-native always-on automation is the single go-forward trigger authority for reindex + wiki regeneration; manual/AI triggering remains only as fallback.
- **Impact:** Freshness becomes continuously maintained; `refresh.log`/`doctor` add operational observability. Risk to control: concurrent test runs resetting/rebuilding `.hq` (ZAIQ-011, ZAIQ-015).
- **Implementation tasks and source-owned work orders:** <!-- PHASE2: automation lane commit SHA and file list -->
- **Verification and rollback:** <!-- PHASE2: test receipts, watcher pause-protocol verification, rollback path -->
- **Open items linked:** ZAIQ-015 (automation operations), ZAIQ-011 (test-workspace isolation guard).
- **Owner authorization evidence:** None yet — decision is PROPOSED; in-flight implementation observed but not landed or verified at records time.
- **Supersedes / superseded by:** Would supersede manual/AI-triggered reindex+wiki (no prior ZAID).
