# Z.ai round 1 (R01) — integration and strategic-decision checkpoint

- **Round:** R1 — "Working HQ" (charter execution sequence: correct four-project identities and clean-clone paths, prove source/index freshness, expose existing useful read models, implement a real tested improvement)
- **Date:** 2026-11-10 (user timezone Europe/Paris); implementation landed 2026-11-10T03:46:21Z; records authored ~2026-11-10T04:30Z
- **Proposer / records integrator:** Z.ai (records lane). Parallel lanes in flight at records time: always-on automation (repos-hq) and bundle creation — their final SHAs/counts are PHASE2 placeholders in this report.
- **Status:** R1 records complete except PHASE2 fields; admission GO_WITH_LIMITS (§9).

## 1. Four-repository pinned baseline

| Repo | Remote base SHA | End SHA | Change class | Tests (command + exit) | Bundle name + verify |
|---|---|---|---|---|---|
| repos-hq | `4631d19cc57a07fa532e627f21737e489d629055` | `9cce014d91187f01d5d6506bb9eeb86e0a1ae111` (advancing — automation lane in flight: `<!-- PHASE2: final R1 head SHA -->`) | HQ operating-tool repairs (identity registry, freshness parity, bin/ visibility, verify loop, wiki scoping) — no product-authority change | HQ smart test plan "full": 1938 tests / 161 files / 36.55s — **1912 pass, 12 skip, 1 todo, 13 fail** (all 13 pre-existing on pristine `4631d19c`); exit non-zero (13 fail); receipt `/tmp/smart-run.txt`. `<!-- PHASE2: automation-lane test delta + final exit code -->` | `R01-repos-hq.bundle` — `<!-- PHASE2: byte length, SHA256, git bundle verify, restore test -->` |
| vivim-web-harness | `73723103dc31b55d4c7347a8470774b27b6f18af` | `73723103dc31b55d4c7347a8470774b27b6f18af` (unchanged) | UNCHANGED (pinned checkpoint; no filler commits) | UNRUN (no changes this round) | `R01-vivim-web-harness.bundle` — `<!-- PHASE2 -->` |
| tierflow | `5c44d14bc46a8c5532635ef1fb8776cca4ddf354` | `5c44d14bc46a8c5532635ef1fb8776cca4ddf354` (unchanged) | UNCHANGED (pinned checkpoint; local alias `vivim-ui`) | UNRUN (no changes this round) | `R01-tierflow.bundle` — `<!-- PHASE2 -->` |
| vivim-labs | `75abd3aaaab1b4869455989848db660bb6a9d25c` | `<!-- PHASE2: this records commit SHA -->` | Docs-only governance records under `zai/` (this report, manifest, decisions, open items) | UNRUN (documentation-only change; no test suite executed this round per lane constraints) | `R01-vivim-labs.bundle` — `<!-- PHASE2 -->` |

**Environment:** Linux container; node v24.22.2; bun 1.4.4 (node:sqlite); git 2.47.3. Round work ran 2026-11-10 ~03:26–04:30 UTC (evidence-file timestamps; 9cce014 authored 03:46:21Z). Dependency/install reproducibility: `bun.lock` v2 under bun 1.4.4 — **guard: never run repo tooling with bun <1.4** (see incident §6 and ZAIQ-014). Protected source ownership: exactly four primary repos with independent histories (ZAI-BASE-001..004); `vivim-ui` is the local alias of GitHub `tierflow`; `vivim-workstation` is non-VIVIM and excluded; `SOFTWARE_HQ` never cloned or indexed.

Only repos-hq (code) and vivim-labs (records) changed this round; tierflow and vivim-web-harness are unchanged pinned checkpoints. This is **not** a report-only round: the tested implementation improvements and HQ operating-tool gains are in §2 and §6.

## 2. HQ operating-tool improvement and reuse

**Task attempted via HQ (query-first, per charter):** R1 needed identity resolution, strict source/index freshness proof, and useful read models over the four clean clones. What HQ actually returned at the pinned baseline, and what was repaired:

- **Task attempted via HQ:** project resolution, `hq check --strict`, `hq wiki --check`, `hq doctor`, index/find/search over the four spaces.
- **Failures found (source pins at `4631d19c`):** (1) `projects.json` held the previous owner's Windows paths (`C:\0-BlackBoxProject-0\…`) — unroutable here, so **all 5 registered spaces were lost** (incl. the non-VIVIM `vivim-workstation`); (2) `hq/bin/hq.mjs#cmdCheck`/`#cmdVerify` walked with a hardcoded 10MB cap while the indexer honors configurable `maxFileBytes` (1.5MB default) — files between 1.5MB and 10MB (observed: 6.8MB `.work/blockers/records.jsonl`, 3.2MB `source-dispositions.v1.json`) were permanently "new", so `check --strict` could never pass on a clean clone; the freshness walk also ignored `config.ignores`; (3) `hq/lib/walk.mjs#DEFAULT_IGNORES` blanket-ignored `bin` — HQ's own CLI (`hq/bin/*.mjs`), the harness `bin` and `studio/bin`, and labs `lane-d/bin` were invisible to the index; (4) repos-hq's own generated `wiki/` (written inside the repo) retriggered staleness forever.
- **HQ repairs landed (all at `9cce014`, author Z.ai <zai@vivim.invalid>):** identity re-registration (ZAID-001); freshness parity — `walkCap = flags['max-file-bytes'] || config.maxFileBytes`, shared `discoverFiles` walk, `config.ignores` passthrough (ZAID-002); per-file `db.prepare` hoisted to one round trip per space (ZAID-004); `'bin'` removed from `DEFAULT_IGNORES` with `sniffBinary` as the content-based binary gate (ZAID-003); repo-scoped `wiki/` ignore in repos-hq's `.gitignore` only, preserving tierflow's 77 tracked wiki source files (ZAID-005). Regression fixtures: `hq/test/check-freshness-parity.test.mjs` (3 tests), `hq/test/walk-bin-source.test.mjs` (4 tests).
- **Baseline versus end-of-round:** before — zero routable spaces, strict check unpassable by construction, ~60 CLI-source files unindexed, wiki self-invalidating; after — 4/4 spaces indexed (post-commit receipt `/tmp/health.json` 2026-11-10T03:46Z: **4,579 files / 71,761 symbols**, incl. repos-hq 1,655/22,760; vivim-labs 730/1,351; vivim-ui 582/7,554; vivim-web-harness 1,612/40,096), `hq check --strict` **PASS** on the clean clone, `hq wiki --check` **PASS** (~4,676 pages), `hq doctor` **all green**. Fixture-level wiki regeneration observed during test runs (`/tmp/pass1.txt`, `/tmp/pass2.txt`: 17-page demo workspace). Token/latency savings: not measured this round (honest unknown — first compounding round; formal baseline from R2). Automation (ZAID-006) in flight: `<!-- PHASE2: automation lane landing — hq refresh / watch / hooks / doctor status -->`.
- **Next-round reuse proof:** post-fix, the read model surfaces the very code the fixes touched. Read-only verification against the HQ index at records time: the `symbols` table contains `cmdCheck`, `cmdVerify` and `walkCap` (repos-hq `hq/bin/hq.mjs`) and `DEFAULT_IGNORES`, `sniffBinary`, `discoverFiles` (repos-hq `hq/lib/walk.mjs`) — i.e. an R2 agent can locate the freshness-parity and bin-unmask machinery through `hq find`/`hq search` instead of repo-wide exploration (index at peek time: 4,581 files / 71,809 symbols, reflecting the automation lane's live working tree reindexed by the in-flight watcher). `<!-- PHASE2: hq find/search command receipts + a bounded context-bundle demo answering a realistic R2 question -->`
- **Efficiency goal:** on track — R1 is the first compounding round; the compounding metrics (raw files opened, repeated searches, stale/missing edges) get their formal baseline in R2 (ZAIQ-010).

## 3. Conceptual duplicate discovery

R1-scope concepts (competing implementations discovered while working — no separate audit round):

1. **Freshness walk-cap authority** — hardcoded 10MB (`cmdCheck`/`cmdVerify` at `4631d19c`) vs configurable `maxFileBytes` (indexer). Overlap: file-universe definition. Unique value of config-driven cap: single tunable authority shared by indexer and gates. Classification: DUPLICATE (policy) → resolved: CANONICAL = `flags['max-file-bytes'] || config.maxFileBytes` (ZAID-002, IMPLEMENTED_VERIFIED). Callers/tests: `hq/test/check-freshness-parity.test.mjs` (3).
2. **Binary/artifact exclusion mechanism** — name-based blanket ignore (`DEFAULT_IGNORES` 'bin') vs content sniffing (`sniffBinary`). Overlap: keep artifacts out of the index. Unique value of sniffing: excludes actual binaries anywhere while admitting hand-written CLI source. Classification: COMPETING → resolved: CANONICAL = `sniffBinary`; name-ignores retained for `dist`/`build`/`out` (ZAID-003, IMPLEMENTED_VERIFIED). Tests: `hq/test/walk-bin-source.test.mjs` (4).
3. **Generated-output ownership/ignoring** — global `config.ignores:["wiki"]` vs repo-scoped `.gitignore`. Overlap: stop wiki self-invalidation. Unique risk of global: silently drops tierflow's 77 tracked wiki source files (hard-gate violation). Classification: COMPETING → resolved: CANONICAL = repo-scoped ignore (ZAID-005, IMPLEMENTED_VERIFIED).
4. **Reindex/wiki trigger mechanism** — manual/AI ad-hoc triggering vs HQ-native always-on (`refresh`/`watch`/`hooks`). Classification: COMPETING → proposed: CANONICAL = HQ-native automation (ZAID-006, PROPOSED, PHASE2).

Broader cross-repo conceptual dedup (Host/UI publisher/Android wrapper/chat dispatch/etc., per the charter's mandatory discovery) is deliberately deferred to R2+ rounds — R1 registered **no retirements and no orphan dispositions**; nothing was deleted. Newly found orphans: none registered this round (bin/ sources were masked, not orphaned; now visible).

## 4. Maximal single-version roadmap delta

- **Newly discovered concepts:** the four above (all within repos-hq's index/freshness/discovery/trigger machinery).
- **One selected go-forward owner per authority:** HQ CLI owns freshness gating (config-driven cap), file discovery (`walk.mjs` + `sniffBinary`), and generated-wiki scoping; automation-trigger ownership proposed HQ-native (ZAID-006).
- **Unique abilities not yet integrated:** none lost this round; tierflow's tracked wiki preserved; bin/ CLI sources newly visible. Pending integrations live in ZAIQ-003..010 (carried).
- **Updated compatibility edges:** `discoverFiles` options unchanged (callers unaffected); `check --strict` semantics now achievable; `projects.json` schema v1 unchanged (contents re-pointed).
- **V1 closure tasks vs deferred:** R1 closed the "Working HQ" gate (identity / freshness / read model). Deferred: 13 pre-existing failures triage (ZAIQ-012), context-query ranking (ZAIQ-013), test isolation guard (ZAIQ-011), automation ops (ZAIQ-015), graph parity (ZAIQ-002 remainder).
- **Exact next work order (source-owned):** (a) PHASE2: automation lane lands → record final SHA/tests, finalize ZAID-006 + ZAIQ-015; (b) PHASE2: bundle lane creates four bundles → fill R01-MANIFEST SHA256/verify/restore fields; (c) R2: triage ZAIQ-012 (fix or document per-test); (d) R2: context-query ranking/filtering (charter "semantic convergence"); (e) kernel-store test isolation guard before the next full-suite run against the live workspace.
- **Derived HQ view:** this Labs report is provenance only; HQ wiki/index views consume it read-only (no new Labs status database).

## 5. Strategic decisions and unresolved items

- **New ZAID entries** (all in [DECISIONS](../governance/DECISIONS.md)): ZAID-001 (identity re-registration), ZAID-002 (freshness parity), ZAID-003 (bin/ unmask + sniffBinary canonical), ZAID-004 (verify-loop round trip), ZAID-005 (repo-scoped wiki ignore) — all **IMPLEMENTED_VERIFIED** at `9cce014` with regression fixtures; **owner-approval status: none recorded yet — owner review of this register is pending** (charter R1 mandate was the operating authorization; no product authority, live effects or capability retirement involved). ZAID-006 (always-on automation) — **PROPOSED**, in flight: `<!-- PHASE2: final SHA + tests -->`.
- **New ZAIQ entries** (in [OPEN-ITEMS](../governance/OPEN-ITEMS.md)): ZAIQ-011 (kernel-store test can `hq reset` the real workspace — isolation guard needed), ZAIQ-012 (13 pre-existing failures, full list, triage in R2), ZAIQ-013 (context-query noise), ZAIQ-014 (bun 1.3.14 lockfile downgrade incident + guard), ZAIQ-015 (automation operations — PHASE2).
- **Round closing check (exact delta vs prior checkpoint):** New: ZAIQ-011, ZAIQ-012, ZAIQ-013, ZAIQ-014, ZAIQ-015; ZAID-001..006. Progressed (state unchanged OPEN, evidence advanced): ZAIQ-001 (four identities resolved and indexed; consumer-compatibility proof partial), ZAIQ-002 (strict freshness now provable and passing; graph parity still open). Newly blocked: none. Resolved: none (no narrative-only resolutions). Carried forward unchanged: ZAIQ-003, ZAIQ-004, ZAIQ-005, ZAIQ-006, ZAIQ-007, ZAIQ-008, ZAIQ-009, ZAIQ-010.
- **Decision debt:** ZAID-006 final SHA/tests (PHASE2); owner approvals outstanding for ZAID-001..006; graph-parity evidence (ZAIQ-002); measured perf delta for ZAID-004 (none recorded).
- **Feature preservation:** nothing dropped. repos-hq `wiki/` exclusion affects only regenerated, rebuildable output; tierflow wiki untouched; `vivim-workstation` removal is a scope policy (ZAI-BASE-001), not a capability loss.

## 6. Source/test receipts and rollback

**Exact edits (repos-hq commit `9cce014d91187f01d5d6506bb9eeb86e0a1ae111`, author Z.ai <zai@vivim.invalid>, 2026-11-10T03:46:21Z, on base `4631d19cc57a07fa532e627f21737e489d629055`):**

| File | +/− | Content |
|---|---|---|
| `projects.json` | +4/−10 | four VIVIM identities re-pointed to clean clones; `vivim-workstation` removed (ZAID-001) |
| `hq/bin/hq.mjs` | +21/−7 | `walkCap` from `flags['max-file-bytes'] \|\| config.maxFileBytes` in `cmdCheck`+`cmdVerify`; `config.ignores` passthrough; prepared statement hoisted to one round trip per space (ZAID-002/004) |
| `hq/lib/walk.mjs` | +7/−1 | `'bin'` removed from `DEFAULT_IGNORES`; binaries still excluded via `sniffBinary`; `dist`/`build`/`out` remain ignored (ZAID-003) |
| `.gitignore` | +4/−0 | repo-scoped `wiki/` ignore with scoping comment (ZAID-005) |
| `hq/test/check-freshness-parity.test.mjs` | +114/−0 | 3 regression tests |
| `hq/test/walk-bin-source.test.mjs` | +82/−0 | 4 regression tests |

**Test outcomes:** full smart-test plan "full" at 9cce014: **1938 tests across 161 files, 36.55s — 1912 pass / 12 skip / 1 todo / 13 fail** (6,821 expect() calls); exit non-zero solely due to the 13 failures, **all pre-existing** — reproduced on a pristine clone of pinned `4631d19c` (clone `/tmp/baseline-hq`; capture `/tmp/baseline-fails.txt`; exact list and capture-provenance note in ZAIQ-012). The 7 new R1 regression tests (3 parity + 4 bin-source) all pass. `<!-- PHASE2: automation-lane test delta on top of 9cce014 + final exit code -->`

**Health outcomes at 9cce014:** `hq check --strict` **PASS** on a clean clone; `hq wiki --check` **PASS** (~4,676 pages); `hq doctor` **all green**. Index receipt: 4 spaces / 4,579 files / 71,761 symbols (`/tmp/health.json`). `<!-- PHASE2: final green outputs for check/wiki/doctor after automation lands -->`

**Incidents (with evidence):**
1. **kernel-store test resets ×2:** `hq/test/kernel-store.test.mjs` executed `hq reset` against the REAL workspace, destroying the live `.hq/` DB plus the gitignored `config.json` and `wiki/` — twice during R1; config rebuilt manually both times (snapshot `/tmp/hq-backup-r1a`). → ZAIQ-011 (isolation guard required before the next full-suite run on the live workspace).
2. **bun lockfile downgrade:** bun 1.3.14 silently downgraded `bun.lock` v2→v1, breaking installs; recovered via `git checkout`; machine pinned to bun 1.4.4. Guard: never run repo tooling with bun <1.4. → ZAIQ-014.

**Negative consumer checks:** consumer surface for these edits = HQ CLI subcommands (`check`, `verify`, `index`, `wiki`) and project resolution — guarded by the 7 new regression tests plus the full suite; `discoverFiles`' options contract unchanged; no external API consumers modified. Broader negative-consumer proof beyond the suite: UNRUN this round (no deletions occurred, so no retirement-level proof was required).

**Rollback:** single-commit revert (`git revert 9cce014`) restores all repos-hq behavior (strict check reverts to permanently-failing on clean clones; bin/ sources leave the index; registry re-points to unroutable paths); index rebuildable via `hq index`; no data migrations. vivim-labs records are docs-only (revert the records commit).

**Compatibility / safety:** no provider APIs, browser automation, second databases or planner changes; no secrets, sessions or private logs added to records (public-safe evidence only; `/tmp` receipts referenced by path, not inlined). **Unverified gates:** Windows/physical-Android acceptance (not claimed); automation (PHASE2); graph parity; measured perf deltas.

## 7. Bundle checkpoint

Per [BUNDLE-PROTOCOL](./BUNDLE-PROTOCOL.md): four repository checkpoints + this manifest define the coherent R1 release set (no synthetic cross-repo commit).

| Artifact | Status |
|---|---|
| `R01-repos-hq.bundle` | `<!-- PHASE2: byte length, SHA256, git bundle verify output, restore test (or UNRUN), exact refs (base 4631d19c… → final head) -->` |
| `R01-vivim-web-harness.bundle` | `<!-- PHASE2: same fields; unchanged repo still receives a pinned checkpoint and valid full bundle when the platform permits; no filler commits -->` |
| `R01-tierflow.bundle` | `<!-- PHASE2: same fields; unchanged repo pinned checkpoint -->` |
| `R01-vivim-labs.bundle` | `<!-- PHASE2: same fields; head = records commit SHA -->` |
| `R01-manifest.json` | written; SHA256 of the final manifest: `<!-- PHASE2 -->` (fill after PHASE2 fields land) |

Restore test: UNRUN at records time (`<!-- PHASE2 -->`). **Placement/delivery rule (quoted from BUNDLE-PROTOCOL):** "Do not publish bundled history into Labs by default. Keep manifests and sanitized receipts here; distribute sensitive bundles only through approved private transfer destinations. Z.ai cloud is not an automatic private preservation authority." Also: "Never commit raw private bundles to public Labs unless separately security cleared." Gate checks 1–7 (clone at pinned SHA, record concurrent public HEAD, `git bundle verify`, restored-HEAD equality with manifest SHAs, applicable tests with pinned provenance, public/private exposure scan, source-owned integration) are executed by the bundle lane in PHASE2.

## 8. Quality scorecard and stretch findings

- **Index source coverage:** 4/4 spaces; 4,579 files / 71,761 symbols at the post-commit receipt (records-time peek incl. in-flight automation tree: 4,581 / 71,809); net +60 files from the bin/ unmask (~71.8k symbols total).
- **Concepts classified this round:** 4 (all in §3); duplicates resolved 3, proposed 1, pending (cross-repo scope) deferred to R2+ per charter.
- **Unique capabilities retained:** all (no retirements; tierflow wiki preserved; masked bin/ sources recovered).
- **Runtime imports/consumers proved:** HQ CLI subcommands + project resolution via regression tests and health runs; deeper consumer-graph proof deferred (ZAIQ-002 remainder).
- **Documentation and graph freshness:** wiki --check PASS (~4,676 pages); Rust graph/SQLite read parity UNVERIFIED this round (ZAIQ-002).
- **Automation improvements:** ZAID-006 in flight (PHASE2).
- **Costs:** full suite 36.55s wall (1938 tests); model/token costs not measured (honest unknown).
- **Unverified conditions:** automation final state; 13 pre-existing failures; context-query ranking; graph parity; device/Windows gates; ZAID-004 measured perf.
- **Stretch goals:** none attempted in R1 — mandatory gates first, per [STRETCH-GOALS](../STRETCH-GOALS.md). Groundwork relevant later: S05 (documentation freshness — wiki self-invalidation fixed), S19 (HQ compounding usefulness — baseline established this round), S17 (clean-clone bootstrap — `check --strict` now passable on a clean clone).

## 9. Next round admission

**GO_WITH_LIMITS.**

- **Conditions:** (1) PHASE2 — automation lane lands: record final SHA/tests, finalize ZAID-006/ZAIQ-015 and the manifest's repos-hq head; (2) PHASE2 — bundle lane delivers four bundles + manifest finalization (SHA256/verify/restore) with the placement rule of §7 honored; (3) ZAIQ-011 isolation guard in place before any further full-suite run against the live workspace; (4) R2 opens with ZAIQ-012 triage and ZAIQ-013 ranking work.
- **Risks:** watcher/test interplay (kernel-store reset recurrence), bun version drift (ZAIQ-014 guard), owner approvals outstanding for ZAID-001..006, 13 pre-existing failures masking new regressions if triaged lazily.
- **Next bounded work:** R2 "Semantic convergence" per charter, building on the now-working HQ read model.
- **Sign-off:** one accountable integrator must sign off the coherent four-repository checkpoint. Records integrator: Z.ai (this report). `<!-- PHASE2: coordinator/integrator sign-off + owner review after manifest finalization and bundle verification -->`
