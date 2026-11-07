# Breaker findings triage — BREAK lane (read-only review)

Source of truth for this review: the two findings files plus the two confirmation sources I actually read
(`hq/lib/model-router.mjs`, `plugins/vivim-team/scripts/grant-protocol.mjs`). Findings whose file is
`hq/lib/team-panel.mjs`, `hq/bin/hq.mjs` or `plugins/vivim-team/hooks/team-state.sh` were NOT read, so they are
marked unverified — I cannot personally confirm them. No source files were edited; only this report was written.

Scope: 15 panel-router findings (F01–F15) + 8 protocol-plugin findings (F01–F08) = 23 total.

Confirmed real (read the source): 13 — of which 5 already fixed, 8 still open.
Unverified (source not read): 10.
Open findings fixable now with a small code edit: 8.

Highest-value remaining fix: panel F03 — same-provider sibling of a 2056 model is kept healthy (provider-level
budget-block contagion). It is visible in live `hq route-model` output (M3 2056 budget-blocked, 22 agents kept on
M3.1-Flash) and matches the module's own motivation (five lanes lost at once to one cap).

---

## Panel-router findings — hq/lib/model-router.mjs / hq/lib/team-panel.mjs / hq/bin/hq.mjs

| id | finding | severity | real? | fixable-now? | owner-action |
|----|---------|----------|-------|--------------|--------------|
| F01 | A 2056 line without the upgrade sentence is capped, not budget-blocked | high | Yes — confirmed FIXED by reading model-router.mjs: `planCredit` (line 113) matches `\(2056\)`/token-plan phrases and is checked before `CAP_RE` (line 114), so short `(2056)` strings now return `budget-blocked`. | No (already fixed) | None. Add a regression test for the short `(2056)` string and error_code `2056`. |
| F02 | `minReliability` is never applied | high | Yes — confirmed: `scoreCandidate` never reads `policy.minReliability`; `classifyHealth` hardcodes `DEFAULT_POLICY.minReliability` (line 129), so 1 completion + 9 errors still returns `healthy` with a "low reliability" reason. | Yes | Apply `policy.minReliability` in `scoreCandidate` (reject below floor) and pass `policy` into `classifyHealth` instead of the hardcoded default. |
| F03 | Same-provider sibling of a 2056 model is kept healthy | high | Yes — confirmed: health is keyed per `provider/model`; `planRoutes` keeps any `healthy` pin regardless of a sibling's 2056 (line 363). | Yes | Add provider-level budget-block contagion: when a provider returns 2056/token-plan-exhausted, suppress same-provider siblings or factor provider health into the keep rule. |
| F04 | A wrapped `tools:` line hides Glob and Grep | high | Unverified — `team-panel.mjs` was not read. | Unknown | Read `team-panel.mjs` continuation-line parsing; fix multi-line/`\r\n` tools so Glob/Grep are not silently dropped. |
| F05 | Eight inherit agents sent to one free model the router watched die | high | Unverified — `hq/bin/hq.mjs` was not read. | Unknown | Read `hq/bin/hq.mjs` `routeTask` fan-out; add a per-model agent cap so inherit agents are not piled onto one free lane. |
| F06 | Pinned model with `thoughtLevel: medium` is "healthy" to router, "will not launch" to panel | high | Yes — confirmed: `planRoutes` never reads `thoughtLevel`; keep rule only checks `health.state === 'healthy'` (line 363). | Yes | Consult thought-level/launch data before keeping a pin so a model the panel says won't launch is not described as a healthy pin. |
| F07 | Every route is priced at `$0.0000` per call | medium | Yes — confirmed: `agentTask = agent.task ?? task ?? DEFAULT_POLICY.task` (line 332) and `DEFAULT_POLICY.task` is `undefined`, so `estInput`/`estOutput` are 0 and `rawCostUsd` is 0 for every candidate (line 172). | Yes | Default task token estimates or skip pricing/rationale when no task is known; do not quote `$0.0000`. |
| F08 | Trailing comment makes `thoughtLevel: max` look illegal | medium | Unverified — `team-panel.mjs` was not read. | Unknown | Read `team-panel.mjs`; strip trailing comments from scalar frontmatter values. |
| F09 | `nonexistent-tool` evidence always says Glob | medium | Unverified — `team-panel.mjs` was not read. | Unknown | Read `team-panel.mjs`; make `evidence` name the actual missing tool. |
| F10 | `inherit` with no `thoughtLevel` is a clean audit | medium | Unverified — `team-panel.mjs` was not read. | Unknown | Read `team-panel.mjs`; decide whether inherit-without-thoughtLevel should be a defect (suite currently requires the hole). |
| F11 | Unclosed frontmatter block reported as a different file shape | medium | Unverified — `team-panel.mjs` was not read. | Unknown | Read `team-panel.mjs`; distinguish unclosed frontmatter from no frontmatter and preserve the name. |
| F12 | Task counts drop the open-task number when `blocked` is written first | medium | Unverified — `team-panel.mjs` was not read. | Unknown | Read `team-panel.mjs`; broaden `parseTaskCounts` patterns (`N blocked open tasks`, comma forms). |
| F13 | Empty catalog blames `inherit` with a broken sentence and proposes nothing | medium | Yes — confirmed: `planRoutes` empty-catalog skip reads "no eligible alternative in the catalog (inherit is inherits the session model)" (line 349), a duplicated/garbled reason. | Yes (trivial wording) | Emit a distinct message when the catalog is empty versus when a named model is missing. |
| F14 | `KNOWN_TOOLS` is unused, so an unknown grant is a clean file | low | Unverified — `team-panel.mjs` was not read. | Unknown | Read `team-panel.mjs`; either consult `KNOWN_TOOLS` or remove it. |
| F15 | The cost sort never runs, so "higher cost tier" is wrong | low | Yes — confirmed: `routeTask` labels any eligible loser with a higher tier "not chosen — higher cost tier" even when the tier gap is unpriced-vs-priced, not price (lines 210–212). | Yes (trivial wording) | Tie the rejection reason to the actual cause (price vs unpriced vs tier). |

## Protocol-plugin findings — plugins/vivim-team/scripts/grant-protocol.mjs (+ hooks)

| id | finding | severity | real? | fixable-now? | owner-action |
|----|---------|----------|-------|--------------|--------------|
| F01 | YAML block-list skills are invisible, so every --apply appends another grant | high | Yes — confirmed FIXED by reading `grant-protocol.mjs`: `fmGet` collects indented `- item` block-list values (lines 59–66), so block-list skills are read and merged; second `--apply` = 0 changes was proven (held section). | No (already fixed) | None. Add a block-list regression test. Note: a block-list `tools:` value reaches `parseTools` as an array (`value.trim()` would throw) — a latent crash worth guarding, not one of the listed findings. |
| F02 | `--apply` is not idempotent: rewrites skills and plants an empty backup every time | medium | Yes — confirmed FIXED: early `return` when `!changed.length` (lines 225–228) means no backup dir and no skill-file rewrite when nothing changed. | No (already fixed) | None. Keep the early return; it also covers the skill-body writes. |
| F03 | Unclosed frontmatter fence swallows the body into the frontmatter | high | Yes — confirmed FIXED: `splitFrontmatter` returns `fm: [], body: lines` when unclosed (lines 46–48), so the original body is preserved and only wrapped in a new empty block; `hadFrontmatter` is now moot. | No (already fixed) | None. Verify unclosed files round-trip with body intact. |
| F04 | Folded / non-flow tools values are parsed as tool names, and duplicate keys are half-updated | high | Yes — confirmed: `tools: >` yields `>` as a tool name (`fmGet` returns the raw rest; `parseTools` keeps `>` → `[>, ...ctx tools]`). Duplicate keys: `findIndex`/`fmSet` rewrite only the first `tools:`/`skills:` line, leaving later grants intact. | Yes | Leave non-scalar/folded values alone (or fail loud); rewrite every duplicate key or refuse the file. |
| F05 | `tools: []` is rewritten into a Context7-only allow-list | high | Yes — confirmed FIXED: `tools: []` → `parseTools` returns `[]`; the `tools.length > 0` guard skips the rewrite (explicit F05 comment, lines 125–139). | No (already fixed) | None. Regression-test `tools: []` stays `[]`. |
| F06 | Leading-whitespace skills key ignored and a second skills key appended | medium | Yes — confirmed (misdescribed, bug is real): `fmGet` matches indented keys via `trimStart`, but slices the RAW head (`head.slice(key.length + 1)`), producing a `: [other]`-corrupted value; `fmSet` replaces the first line rather than appending a second key. | Yes | Slice from the trimmed head for both scalar and block values; keep exactly one key per field. |
| F07 | Missing jq makes the hook emit invalid JSON and still exit 0 | medium | Unverified — `team-state.sh` was not read. | Unknown | Read `plugins/vivim-team/hooks/team-state.sh`; fail open with valid JSON or non-zero when jq is absent. |
| F08 | The hook cannot block a prompt by its own work, and a missing state file fails open | low | Unverified — `team-state.sh` and `hooks.json` were not read. | Unknown | Read the hook and `hooks.json`; confirm missing/invalid state fails open as designed and cannot block a prompt beyond the 2s host cap. |

## Summary

- 23 findings total. 13 confirmed real by reading the source; 5 of those are already fixed in the current files (panel F01; protocol F01, F02, F03, F05). 8 remain open and real. 10 are unverified because their source files (`team-panel.mjs`, `hq/bin/hq.mjs`, `team-state.sh`) were not read.
- All 8 open confirmed findings are fixable now with small, scoped edits — none require a new design or owner decision.
- Highest-value remaining fix: panel F03 (same-provider sibling contagion after a 2056), high severity and currently visible in live output.
