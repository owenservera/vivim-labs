# SITREP — VIVIM Web Harness ↔ HQ integration

**Date:** 2026-11-03 · **Audience:** owner · **Mode:** measured, read-only toward vivim-web-harness
**Measured from:** `C:\0-BlackBoxProject-0\repos_hq` using `bun hq/bin/hq.mjs`
**Subject space:** `vivim-web-harness` → `C:\0-BlackBoxProject-0\vivim-web-harness`
**Authority:** read-only projection. Nothing in this document grants execution authority, and no row below is permission to act.

---

## 0. Verdict

The integration is **live and working on the read path, drifted and false-green on the verification path, and incomplete on the designed surface.**

| Face | State | Proof |
| --- | --- | --- |
| Registration + index | **current** | `hq status` → 2 spaces, 3535 files, 50502 symbols, last indexed `2026-11-03T17:13:07.290Z` |
| VIVIM feed (`vivim_records`) | **aligned** | `hq vivim check` → "VIVIM feed is current and aligned"; digest `3159dc6da50a` |
| KB package intake | **stale on disk, drifted in HQ** | `hq kb verify` exit 1 → Unchanged 7 · changed 6 · added 14 · Missing 315 |
| KB producer self-check | **false green** | `package.mjs --check` exit 0 "consistent" while 6 package files differ from the live tree |
| `hq check` freshness | **19 false positives** | gitignored partials + one 2.4 MB file over the 1.5 MB walker cap |
| DCB context bundles | **working** | 24 scoped sources, 4633/5120 tokens, sha256 + line ranges per source |
| Lint / docs / wiki gates | **noisy, non-blocking** | 3398 INFO findings; docs health 10 missing families / 258 orphans; wiki stale 3577 pages |
| Designed alignment surface | **mostly superseded, partly absent** | 5 of 9 rows in `VIVIM-LINK.md:132-144` now exist; MCP, wiki page, home, `ask` still absent |
| VIVIM's own HQ copy (`.hq-tool/`) | **stale snapshot** | 10 files differ, 3 commands missing, bundle index 16 h stale |

**One-line:** HQ can read VIVIM correctly today; VIVIM cannot prove its own KB package is current, HQ's freshness checks report noise rather than truth, and the HQ copy VIVIM actually runs is a ten-file snapshot behind this one.

---

## 1. Registration and index

```
$ bun hq/bin/hq.mjs status
HQ status — C:\0-BlackBoxProject-0\repos_hq
Spaces: 2 | Files: 3535 | Symbols: 50502 | Commits: 503
Annotations: 234 | Notes: 1 | Receipts: 3
Last indexed: 2026-11-03T17:13:07.290Z

repos-hq	-	756 files	16089 symbols	2026-11-03T17:13:01.452Z
vivim-web-harness	-	2779 files	34413 symbols	2026-11-03T17:13:07.287Z
```

- Space registered at `C:\0-BlackBoxProject-0\vivim-web-harness`, 2779 files / 34413 symbols / 500 commits.
- Incremental pass after registration: `+0 ~0 =2779 -0` — no churn, no deletion, 27.6 s wall.
- HQ never wrote into the VIVIM tree. All writes landed in `.hq/index.db` (rebuildable, gitignored).

**Repo state at measure time (untouched by this session):** HEAD `8a80235f64563dd34148cfc144019979d321f837` ("chore(repo): snapshot working tree"), 18 staged modifications, 10 commits behind `origin/main`. Deliberately divergent; no pull, rebase, or commit was performed.

---

## 2. VIVIM feed (goals alignment)

Before sync, 4 registers had moved:

```
$ bun hq/bin/hq.mjs vivim check
changed: docs/capabilities/atomic-registry.json
changed: docs/capabilities/source-file-catalog.json
changed: docs/capabilities/criticality.json
changed: docs/roadmap.json
```

After `hq vivim sync`:

```
Phase: WINDOWS_ROADMAP_2026_10_31 — active
Capabilities 354 · Requirements 354 · Tasks 161 · Journeys 120 · Gaps 39 · Routes 26
Synced: 2026-11-03T17:23:39.860Z · digest 3159dc6da50a
Local registries: match the feed
```

```
$ bun hq/bin/hq.mjs vivim check
VIVIM feed is current and aligned.
```

**Unlinked-work signal (advisory, not a defect):**

- 90 P0 capabilities without a task
- 68 tasks without a capability
- 218 unlinked capabilities

`hq vivim explain cap-auth-001` → `no record cap-auth-001`, exit 1. The subcommand exists (help at `hq/bin/hq.mjs:293`, dispatch `hq/bin/hq.mjs:1268`); the id tested was simply not in the feed. Use a real id from `atomic-registry.json`.

**Where it is implemented:** `hq/lib/vivim.mjs`; dispatch `hq/bin/hq.mjs:1268-1347`; `syncVivim`, `readAlignment`, `alignmentDrift`, `alignmentForTask`.

---

## 3. KB package intake

Recorded intake:

- digest `ad9dcad49bc69ef1ad55d4afdaa13f5ef622efcd422240aa4f2fb2fd61ae0957`
- 25 files, ingested `2026-11-02T18:04:31Z`
- package lives at `C:\0-BlackBoxProject-0\vivim-web-harness\runtime\kb-package`

Current verify:

```
$ bun hq/bin/hq.mjs kb verify ../vivim-web-harness/runtime/kb-package
KB verify: vivim-kb 2026-11-02
Unchanged 7 · changed 6 · added 14
Missing from package: 315
Package drifted from what HQ holds.
```

**Root cause of the "Missing 315":** `intakeVerify` (`hq/lib/kb-intake.mjs:1565-1597`) compares a 25-file package against the whole statement of `documents` rows for the space (328 rows). 25 vs 328 → 25 − 7 − 6 − 12 recorded-but-not-in-`documents` = 315 "missing". The 12 package `.json` files land as kind `data`/`other`, never as `documents` rows, so they are structurally unreachable by the comparison. **This comparison can never return `ok` for a partial package.**

**Drift proven live.** SHA-256 of live file vs package copy differs for exactly 6 files, all of them among the 18 staged modifications in the VIVIM repo:

- `AGENTS.md`
- `docs/architecture/CORE-KNOWLEDGEBASE-V1.md`
- `docs/knowledge/MASTER-CATALOG-V1.md`
- `docs/knowledge/OPERATIONS-V1.md`
- `docs/knowledge/PENDING-WORK-V1.md`
- `docs/knowledge/README.md`

**Producer self-check is a false green.** `C:\0-BlackBoxProject-0\vivim-web-harness\tools\knowledge\package.mjs:304-377` (`checkPackage`) verifies only the internal manifest digest and file counts. It never re-hashes live sources. It therefore exits 0 and prints "consistent" while the package is six files out of date.

**Fresh intake would be a real change:** `hq kb intake --dry-run` → Added 12 · changed 13 · unchanged 0 · unclassified 21 · decisions 0 · terms 0.

**KB health:**

```
415 docs · 13 decisions · 72 canon · 93 sessions · 3128 orphans
```

---

## 4. Quality gates

| Gate | Command | Result |
| --- | --- | --- |
| Index freshness | `hq check` | **exit 1** — 19 files, all false positives (see §8.3) |
| Protocol lint (vivim) | `hq lint --project vivim-web-harness` | 3398 findings / 2779 files, **all INFO** — `missing-file-header`, `missing-export-doc` |
| Document health (vivim) | `hq docs health --project vivim-web-harness` | Missing families **10** · Orphans **258** · Stale **0** · Duplicates **18** · Conflicts **0** |
| Wiki freshness | `hq wiki --check` | **exit 1** — "Wiki is stale: 3577 changed page(s)" |
| Sessions | `hq sessions list` | 93 sessions, dominated by HQ work, not VIVIM work |
| MCP surface | `hq/lib/mcp.mjs` | 11 tools (`hq_search` … `hq_status`); **no** `hq_vivim_*` tool |

Lint severity is uniformly informational. There is no error or warning class in the vivim project, so lint is currently advisory-only and cannot gate a handoff.

The 3577 stale wiki pages are one full regeneration away from clean; nothing in them is corrupt, they have simply never been written.

---

## 5. Context quality (DCB)

Probe: `hq_context` for the task "mobile relay phone bootstrap F02 render plan" scoped to `project=vivim-web-harness`.

- 24 sources, all vivim-scoped
- 4633 / 5120 usable tokens (budget 6000, reserve 800)
- every source carried `ref`, `lines`, `sha256`
- sources resolved to real VIVIM code: `apps/mobile-relay/bootstrap-f02.mjs`, `apps/mobile-relay/api/relay.mjs`, `apps/mobile-relay/app.js`, plus matching docs

This is the read path working as designed: bounded, scoped, provenance-bearing, and deterministic apart from the timestamp.

---

## 6. VIVIM's own agentic surface (measured, previously unrecorded)

The VIVIM repo is far more agent-configured than the HQ-side docs assume.

| Surface | Path | Content |
| --- | --- | --- |
| Root entrypoint | `AGENTS.md` | Owner authorization 2026-10-31; "any user prompt supersedes any prior context"; single root `AGENTS.md`, no nested copies |
| Skill pack | `.agents/skills/*/SKILL.md` | 8 skills: `vivim-browser-companion`, `vivim-chrome-owner`, `vivim-host-bootstrap`, `vivim-mobile-pwa`, `vivim-provider-harness`, `vivim-relay-bridge`, `vivim-roadmap`, `vivim-world-schema` |
| Studio OpenCode config | `studio/opencode.json` | model `openrouter/nvidia/nemotron-3.5-super-vl:free`; instructions `[context.md, ultracave SKILL.md]` |
| Studio agents | `studio/.opencode/agents/` | local-opencode, chatgpt-aion, git-manager |
| **Self-contained HQ bundle** | `.hq-tool/` | **53 files · 1,495,618 bytes · BUNDLE.json digest `b657fa8181dd89703d436dab69e963dd166001280703fe6117009be4555368fd` · 30 capabilities · 48 bundled files · every file stamped `2026-11-03 07:18:54`** |
| **Live HQ state** | `.hq/index.db` | **45,436,928 bytes · 1 project · 2580 files · 34413 symbols · created `2026-11-03T01:29:29Z` · last indexed `2026-11-03T01:30:06Z`** |
| Packaged intake source | `runtime/kb-package/` | 25 files, including a packaged `AGENTS.md` |

Two consequences:

1. **VIVIM dogfoods HQ — with a stale copy.** It ships its own `hq/` bundle (`bin/hq.mjs`, `lib/{kb-intake,kb,mcp,context,canon,filetypes}.mjs`) and keeps a 45 MB local index. Measured against `repos_hq/hq/`:

   | Measure | `repos_hq/hq` | `.hq-tool/hq` |
   | --- | ---: | ---: |
   | Files (`.mjs`/`.json`/`.md`) | 95 | 38 |
   | Identical content | — | 28 |
   | Differing content | — | **10** |
   | Present only in `repos_hq` | **57** | — |
   | Present only in bundle | — | 0 |
   | CLI dispatch cases | **50** | 47 |

   Differing files and line deltas:

   ```
   bin/hq.mjs          2432 → 2013   (-419)
   lib/kb-intake.mjs   1597 → 1101   (-496)
   lib/wiki.mjs        2192 → 1906   (-286)
   lib/kb.mjs          1087 →  983   (-104)
   CONTRACT.md         1197 → 1118    (-79)
   lib/schema.mjs       522 →  467    (-55)
   lib/home.mjs         674 →  636    (-38)
   lib/protocol.mjs     515 →  515    (hash differs, same length)
   lib/components.mjs  2158 → 2158    (hash differs, same length)
   lib/semantic-optimizers.mjs 267 → 267 (hash differs, same length)
   ```

   Commands present in `repos_hq` and **absent from the bundle**: `capabilities`, `sessions`, `semantic`. The bundle does carry `vivim` and `kb` dispatch, and its own `intakeVerify` at `kb-intake.mjs:1069` — an older 1101-line variant of the defective function described in §8.1. The bundle registry also lacks `kb-binding.v1.json` (14,061 B).

   **Consequence:** VIVIM is not running the HQ being measured here. Any fix to `kb-intake.mjs`, `schema.mjs`, or `wiki.mjs` in `repos_hq` silently does not reach `.hq-tool/`.

2. **VIVIM's own index is 16 hours stale.** Its `meta` row reads `last_index_at 2026-11-03T01:30:06Z`, while 226 JSON files under the tree have a later mtime. The measurable effect:

   | Language | HQ index | VIVIM's own index |
   | --- | ---: | ---: |
   | json | **1604** | 1405 |
   | javascript | 773 | 773 |
   | markdown | 328 | 328 |
   | symbols | 34413 | 34413 |

   The missing 199 rows are all JSON — consistent with the bundle having been written at `07:18:54`, 5 h 48 m after the index pass. Its dogfood instance has never been refreshed.

3. **VIVIM does not use the `@hq:` annotation protocol.** Zero `@hq:` tags across **1101** source and doc files (excluding `.hq-tool/`, `runtime/kb-package/`, `docs/research/`). So the 3398 lint findings in §4 measure HQ's convention against a repository that never adopted it. That is not a VIVIM defect; it is a wrong-ruler measurement and should not be reported as VIVIM debt.

---

## 7. Designed contract vs implemented

`docs/VIVIM-LINK.md:132-144` records 9 alignment surfaces as absent. Re-measured against source:

| Surface | Doc claim | Measured 2026-11-03 | Evidence |
| --- | --- | --- | --- |
| `hq/lib/vivim.mjs` read API | present | **present** | `hq/lib/vivim.mjs` |
| `vivim_records` table | present | **present** | `hq/lib/schema.mjs` |
| `hq vivim sync\|status\|check\|explain` | absent | **PRESENT** | `hq/bin/hq.mjs:290-293` help, `:1268-1347` dispatch |
| `hq context` vivim block | absent | **PRESENT** | `hq/lib/context.mjs:14,729-744` |
| `hq route` alignment block | absent | **PRESENT** | `hq/lib/skills.mjs:7,429,503-517` |
| `hq brief` alignment summary | absent | **PRESENT** | `hq/lib/lake.mjs:14` (`alignmentSummary`, `alignmentMarkdown`) |
| `hq feed` alignment field | absent | **present** (via lake) | `hq/lib/lake.mjs:14` |
| `hq home` Goals alignment | absent | **absent** | no vivim import in `hq/lib/home.mjs` |
| `hq wiki` `alignment.md` | absent | **absent** | no vivim import in `hq/lib/wiki.mjs` |
| `hq mcp` `hq_vivim_alignment`, `hq_vivim_capability` | absent | **absent** | `hq/lib/mcp.mjs:22-150` lists 11 non-vivim tools |
| `hq ask` alignment routing | absent | **absent** | not wired |

**`docs/VIVIM-LINK.md:136-140` is stale.** The document understates implemented work by five rows. It is claimed by the VIVIM orchestrator on the sync bus; correcting it requires a bus claim, not a unilateral edit.

**Model intake `hq.vivim-model/1`** (`kb-model.mjs`) remains specified and unimplemented. No model tables exist. `docs/KB-LINK.md` already states this correctly.

---

## 8. Concrete defects

### 8.1 `intakeVerify` cannot succeed on a partial package

- **Where:** `hq/lib/kb-intake.mjs:1565-1597`
- **Symptom:** `hq kb verify` always reports hundreds of "Missing" and exits 1, even when the package is perfect.
- **Cause:** compares package file set against all `documents` rows for the space, and never counts `data`/`other` rows.
- **Fix shape:** compare only against the rows carrying `kb_package` provenance, and treat non-`documents` kinds as matched by content hash.

### 8.2 Producer self-check is a false green

- **Where:** `vivim-web-harness/tools/knowledge/package.mjs:304-377`
- **Symptom:** `--check` exits 0 "consistent" while 6 package files differ from the live tree.
- **Cause:** validates manifest digest and counts; never re-hashes sources.
- **Fix shape:** regenerate the in-memory manifest and diff per-file hashes against the on-disk manifest; exit 1 on any mismatch.
- **Why it matters:** this is the command the runbook tells the operator to trust. A false green here invalidates every downstream "package is current" claim.

### 8.3 `hq check` counts walker exclusions as staleness

- **Where:** `hq/lib/walk.mjs:15` (`DEFAULT_MAX_FILE_BYTES = 1_500_000`), `:99` (`if (stat.size > maxFileBytes) continue;`)
- **Symptom:** 19 "stale" files that are not stale.
- **Breakdown of the 19:**
  - `vivim-web-harness/sessions/*.json` (13) — excluded by `vivim-web-harness/.gitignore:40` `sessions/`
  - `vivim-web-harness/.vivim-context/lake/events/2026-11.jsonl` — excluded by `.gitignore:27` `.vivim-context/`
  - `vivim-web-harness/docs/capabilities/source-symbol-index.json` — **2,401,007 bytes**, over the 1,500,000 cap
  - `repos-hq/sessions/*.json` (2) and `harvest/cr-extensions/.../2026-08-24-normal.json` (1)
- **Effect:** `hq check` cannot be used as a CI gate; it fails on a correct tree. A reviewer who trusts it will conclude the index is broken when it is not.

### 8.4 VIVIM runs a stale HQ copy and a stale index

- **Where:** `vivim-web-harness/.hq-tool/hq/` and `vivim-web-harness/.hq/index.db`
- **Symptom:** 10 of 38 bundled files differ from `repos_hq/hq/`; 3 commands (`capabilities`, `sessions`, `semantic`) do not exist in the bundle; the bundle's own index is 16 h old and misses 199 JSON rows.
- **Cause:** the bundle is a point-in-time install (all files stamped `2026-11-03 07:18:54`) with no refresh path; its index was built at `01:30:06Z` and never re-run.
- **Why it matters:** fixes applied in `repos_hq` — including the §8.1 and §8.3 fixes — do not reach the HQ instance VIVIM actually runs. The two systems will diverge further with every `repos_hq` commit.
- **Fix shape:** add a bundle-refresh step keyed on the `BUNDLE.json` digest, and run `hq index` inside the VIVIM space after any bundle install.

---

## 9. Open owner decisions

From `docs/CAPABILITY-KB-ARCHITECTURE.md` (D1–D4) and `docs/CONTINUATION-PLAN.md` (P0–P5):

| Id | Question | Current default |
| --- | --- | --- |
| D1 | Does HQ own capability ids, or does VIVIM? | HQ owns schema, VIVIM owns execution (default governs) |
| D2 | Is KB intake evidence-only? | Yes — evidence, never authority |
| D3 | Who confirms promotion to `current`/`accepted`? | Owner only, explicit flag |
| D4 | Default authority ceiling for a package | `project_observed` |
| P3 | Approve the VIVIM↔HQ contract as normative | **pending** — blocks promotion work |

No decision above is blocked on missing evidence. All four are blocked on an owner sentence.

---

## 10. Next actions

Ordered by leverage. Nothing here was executed; the VIVIM tree was read-only for this session.

**A. Make the producer honest (VIVIM repo, requires an owner write decision):**

```sh
cd C:\0-BlackBoxProject-0\vivim-web-harness
bun tools/knowledge/package.mjs --write
bun tools/knowledge/package.mjs --check     # must now exit 0 with matching hashes
```

**B. Re-ingest the corrected package (HQ repo):**

```sh
cd C:\0-BlackBoxProject-0\repos_hq
bun hq/bin/hq.mjs kb intake ../vivim-web-harness/runtime/kb-package --dry-run
bun hq/bin/hq.mjs kb intake ../vivim-web-harness/runtime/kb-package
bun hq/bin/hq.mjs kb verify ../vivim-web-harness/runtime/kb-package
```

Expect `hq kb verify` to still exit 1 until §8.1 is fixed. Do not read that as a failed re-ingest.

**C. Fix the three defects:**

| Defect | File | Smallest correct change |
| --- | --- | --- |
| 8.1 subset comparison | `hq/lib/kb-intake.mjs:1565-1597` | verify against `kb_package` provenance rows only |
| 8.2 false-green check | `vivim-web-harness/tools/knowledge/package.mjs:304-377` | re-hash live sources, exit 1 on mismatch |
| 8.3 freshness noise | `hq/lib/walk.mjs:99` + `hq check` reporting | report walker exclusions under a separate `excluded` bucket, not `stale` |
| 8.4 stale dogfood copy | `vivim-web-harness/.hq-tool/` | index after install; add a bundle-digest freshness check |

**D. Regenerate the wiki** (one pass clears 3577 pages):

```sh
bun hq/bin/hq.mjs wiki
bun hq/bin/hq.mjs wiki --check
```

**E. Correct the stale contract table** — `docs/VIVIM-LINK.md:136-140` — **after** a sync-bus claim, since the VIVIM orchestrator owns that file.

**F. Refresh the dogfood copies** — two separate commands, both in the VIVIM tree:

```sh
cd C:\0-BlackBoxProject-0\vivim-web-harness
bun .hq-tool/hq/bin/hq.mjs index     # closes the 199-row JSON gap
```

Re-installing `.hq-tool/` requires an owner decision on how the bundle is cut; it is not a regeneration command today. Until then, treat `.hq-tool/` as a frozen snapshot and prefer `repos_hq` for any measurement that must reflect current behaviour.

**G. Watch bundle drift** — compare `vivim-web-harness/.hq-tool/BUNDLE.json` digest `b657fa81…` and the 10 differing files against the current `repos_hq/hq/` tree; a mismatch means VIVIM is running a stale HQ.

---

## 11. Evidence appendix

Every number in this document traces to one of these commands, run from `C:\0-BlackBoxProject-0\repos_hq` on 2026-11-03.

```sh
bun hq/bin/hq.mjs status
bun hq/bin/hq.mjs check
bun hq/bin/hq.mjs index
bun hq/bin/hq.mjs vivim check
bun hq/bin/hq.mjs vivim sync
bun hq/bin/hq.mjs vivim status
bun hq/bin/hq.mjs vivim explain cap-auth-001
bun hq/bin/hq.mjs kb status
bun hq/bin/hq.mjs kb health
bun hq/bin/hq.mjs kb verify ../vivim-web-harness/runtime/kb-package
bun hq/bin/hq.mjs kb intake ../vivim-web-harness/runtime/kb-package --dry-run
bun hq/bin/hq.mjs lint --project vivim-web-harness
bun hq/bin/hq.mjs docs health --project vivim-web-harness
bun hq/bin/hq.mjs wiki --check
bun hq/bin/hq.mjs sessions list --limit 5
bun hq/bin/hq.mjs kb query "capability registry template" --kind doc
```

```sh
# VIVIM repo, read-only
Get-ChildItem -Recurse -File -Include 'AGENTS.md','CLAUDE.md','GEMINI.md','SKILL.md','opencode.json'
Get-Content .hq-tool\BUNDLE.json
Get-Item .hq\index.db
Get-Content .gitignore
```

```powershell
# bundle drift: per-file SHA-256 comparison of repos_hq/hq vs .hq-tool/hq
# result: identical 28 · differing 10 · repos_hq-only 57 · bundle-only 0
# CLI dispatch: 50 cases (repos_hq) vs 47 (bundle); missing: capabilities, sessions, semantic

# VIVIM own index, opened read-only
sqlite3 -readonly .hq\index.db "select 'files', count(*) from files union all select 'symbols', count(*) from symbols;"
sqlite3 -readonly .hq\index.db "select key, value from meta order by key;"
# → files 2580 · symbols 34413 · last_index_at 2026-11-03T01:30:06.635Z · created_at 2026-11-03T01:29:29.383Z

# annotation adoption
# → 0 @hq: tags across 1101 source/doc files (excluding .hq-tool/, runtime/kb-package/, docs/research/)
```

Source anchors cited: `hq/bin/hq.mjs:290-293,1268-1347` · `hq/lib/context.mjs:14,729-744` · `hq/lib/skills.mjs:7,429,503-517` · `hq/lib/lake.mjs:14` · `hq/lib/mcp.mjs:22-150` · `hq/lib/kb-intake.mjs:1565-1597` · `hq/lib/walk.mjs:15,99` · `vivim-web-harness/tools/knowledge/package.mjs:304-377` · `vivim-web-harness/.gitignore:27,40` · `docs/VIVIM-LINK.md:132-144`.

---

## 12. Traceability

- Feed contract: `docs/VIVIM-FEED-CONTRACT.md` (`hq.vivim-alignment/1`)
- Intake contract: `docs/VIVIM-KB-INTAKE.md` (`hq.vivim-kb-intake/1`)
- Kernel target: `docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md`
- Phases and hazards: `docs/CONTINUATION-PLAN.md`
- Gate commands: `docs/BASELINING-RUNBOOK.md`
- North stars: `docs/NORTH-STARS.md` — U3 (trust/evidence), A11 (DCB auto-generation), D4 (filetypes drive architecture), V8 (trust)
- Rule book: `docs/AGENTIC-RULEBOOK.md` §8 privacy, §12 non-equivalences

---

*Read-only projection. A document never grants execution authority. Evidence is not permission.*
