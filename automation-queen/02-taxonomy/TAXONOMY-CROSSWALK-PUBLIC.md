# TAXONOMY-CROSSWALK-PUBLIC.md — manifest fileType × 217-record agentic-file registry (AUTQ-01)

**Scan:** `autq01-20261107T2203Z` · Lane E. **Inputs:** manifest fileType column (2,861 rows) and the 217-record agentic-file registry (7 repos, 19 observed types, 133 catalog patterns of which 12 observed / 121 unobserved; 143 mapped / 74 unmapped records). **Corpus count** = copy receipts (930 COPIED) joined to the manifest fileType; fan-out rows are reference-only by design, so corpus < manifest for every type.

## 1. Crosswalk table

| Manifest fileType | Manifest rows | Corpus rows | Registry family / role (observed patterns) | Registry coverage verdict |
| --- | --- | --- | --- | --- |
| agent | 235 | 105 | F01 Agent entry & bootstrap (AGENTS.md ×26, BOOTSTRAP.md ×20, WORKSPACE.md ×4, AGENT.md ×3) + F02 Provider-native instruction rules (CLAUDE.md ×3, GEMINI.md ×1, `.opencode/agents/*.md` ×3) + F03 identity (0 observed) | Partial — entry/instruction defs covered; role/identity patterns unobserved |
| tool | 438 | 221 | F08 capability (TOOLS.json ×3, TOOL-REGISTRY.json ×1, TOOLS.md ×1) | Weak — registry tracks tool *registries*, not tool implementations |
| docs | 576 | 256 | F04 Context & retrieval (README.md ×45, CONTEXT.md ×1) + F19 Collaboration & handoffs (HANDOFF.md ×12) + F12 State/status & telemetry (SITREP.md ×20) | Good for doc artifacts; no map for spec/runbook subtypes |
| test | 282 | 113 | F17 Testing & validation — 0 observed records | None — registry has no test pattern |
| other | 779 | 141 | Mixed; includes F10 config observed records (claude.json ×4, gemini.json ×2, agent.json ×1) and the 74 unmapped registry records | Bucket — needs reclassification (see §3) |
| policy | 86 | 9 | F05 Law, policy & guardrails (POLICY.md, RULES.md, …) — 0 observed | None observed |
| model-router | 73 | 26 | No registry family; nearest F10 config (provider config files observed) | None |
| skill | 78 | 8 | F09 Skills & reusable methods (SKILL.md ×52, BEST-PRACTICES.md ×15) | Strong — the best-covered type |
| ui | 74 | 0 | No registry family | None |
| workflow | 47 | 37 | Nearest catalog F14 Work inventory & scheduling / F15 Planning & decomposition — 0 observed | None observed |
| executor | 26 | 0 | Nearest catalog F14/F16 Execution & operations — 0 observed | None observed |
| MCP | 25 | 7 | F10 config (provider/client config JSON) + F08 capability | Partial — server *configs* map; server *implementations* do not |
| trigger | 24 | 1 | Nearest F12 (state/status) or F13 continuity — no trigger pattern observed | None observed |
| schedule | 21 | 0 | Nearest F14 — 0 observed | None observed |
| prompt | 19 | 0 | Nearest catalog F06 Goals & product intent — 0 observed | None observed |
| supervisor | 36 | 6 | No direct pattern; nearest F01 entry + F16 operations | None observed |
| hook | 12 | 0 | Nearest F16 — 0 observed | None observed |
| *(extension-typed: mjs 8, md 2, ts 1, ps1 1, yaml 1, json 1)* | 14 | *(included in typed rows above where copied)* | — | Taxonomy hygiene defect |
| *(missing fileType)* | 16 | 0 | — | Taxonomy hygiene defect |

Totals: manifest 2,861 (typed 2,831 + extension-typed 14 + missing 16) · corpus 930.

## 2. What the crosswalk shows

1. **The 217-record registry is instruction/config-centric.** All 19 observed types are `.md` instruction/context/collaboration artifacts or provider/client config JSON. Its strongest families are skills (67 records), agent entry & bootstrap (53), context & retrieval (46) — i.e., it catalogs *how agents are instructed*, not *how automation runs*.
2. **The manifest's operational types have no registry counterpart.** workflow, trigger, schedule, hook, executor, test, ui, model-router, supervisor — 585 manifest rows — land on catalog families with zero observed records (121 of 133 catalog patterns unobserved). The registry's own interpretation states observed files are not standards and unobserved catalog patterns are not automatically required.
3. **Both vocabularies are partial and complementary.** The manifest classifies runtime automation code the registry never saw; the registry classifies instruction files the manifest types as agent/docs/skill. A merged taxonomy needs: (a) the registry's 21-family spine for instruction artifacts, (b) the manifest's 17 operational types for code, (c) a reclassification pass for the 793 rows that are `other`/extension-typed/missing.
4. **Corpus skew is by design.** Fan-out rows (1,538) are reference-only, so corpus counts track the first-pass deep-dive repos only (930 copies). The corpus is therefore instruction/doc-heavy (docs 256, tool 221, agent 105) and contains no ui/prompt/schedule rows because those types concentrated in fan-out repos or were secret-blocked/cap-truncated.

## 3. Follow-ups recorded (links, not new authority)

- Reclassify 793 rows (other 779 + extension-typed 14) and backfill 16 missing fileTypes — extends the G1 limits, not a new scan.
- Extend the registry with operational families (workflow/trigger/schedule/executor/test) or declare the two vocabularies officially separate — decision for the taxonomy owner; QUEEN-BLUEPRINT #18 carries the backfill.
- No registry record was modified: the registry file was read once, read-only, as the one permitted original-repo read.
