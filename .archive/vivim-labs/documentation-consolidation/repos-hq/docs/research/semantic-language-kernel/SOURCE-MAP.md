# Audited source-map — authoritative references and historical leads

**Verification:** paths marked **inspected** were fetched from GitHub main in this conversation on 2026-11-02. Historical leads are source locations verified in earlier repository inspection or known design discussions; they require reinspection before code changes. URLs always point to original repositories; local paths are below.

| Family | Repo file / URL | What to recover | Evidence |
| --- | --- | --- | --- |
| HQ inline parser | [repos-hq/hq/lib/protocol.mjs](https://github.com/owenservera/repos-hq/blob/main/hq/lib/protocol.mjs) | v1 annotation kinds, deterministic comments scanner and lint contract | inspected |
| HQ agent rules | [repos-hq/hq/AGENT-PROTOCOL.md](https://github.com/owenservera/repos-hq/blob/main/hq/AGENT-PROTOCOL.md) | coding agent obligations and current comment syntax | inspected earlier |
| HQ knowledge | [repos-hq/hq/lib/kb.mjs](https://github.com/owenservera/repos-hq/blob/main/hq/lib/kb.mjs) | unified read model over docs, decisions, CANON, sessions, notes, wiki | inspected earlier |
| HQ intake | [repos-hq/hq/lib/kb-intake.mjs](https://github.com/owenservera/repos-hq/blob/main/hq/lib/kb-intake.mjs) | package validations, proposed-only incoming authority, hashes | inspected earlier |
| HQ wiki | [repos-hq/docs/WIKI-CONSOLE.md](https://github.com/owenservera/repos-hq/blob/main/docs/WIKI-CONSOLE.md) | read-only typed registry, browser/CLI command consistency | inspected |
| HQ rendering | [repos-hq/hq/lib/wiki.mjs](https://github.com/owenservera/repos-hq/blob/main/hq/lib/wiki.mjs) | deterministic wiki construction | inspected earlier |
| HQ source of truth | [repos-hq/docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md](https://github.com/owenservera/repos-hq/blob/main/docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md) | shared kernel decision, migration gates | inspected |
| HQ bus | [repos-hq/docs/SYNC-BUS.md](https://github.com/owenservera/repos-hq/blob/main/docs/SYNC-BUS.md) | coordination, claims, cursors, inbox | historical lead — check current HQ agent updates |
| HQ bus code | [repos-hq/scripts/sync.mjs](https://github.com/owenservera/repos-hq/blob/main/scripts/sync.mjs) | messaging owner implementation | historical lead |
| Ω reference design | [vivim-omega/docs/NCLL-AND-SELF-KNOWLEDGE.md](https://github.com/owenservera/vivim-omega/blob/main/docs/NCLL-AND-SELF-KNOWLEDGE.md) | mind, NLCL pure, director, 17 symbols, typed IR | inspected |
| Ω pure implementation | [vivim-omega/plugins/vivim-nlcl-pure/src/interpret.ts](https://github.com/owenservera/vivim-omega/blob/main/plugins/vivim-nlcl-pure/src/interpret.ts) | deterministic interpreter | inspected earlier |
| Ω language entry | [vivim-omega/plugins/vivim-nlcl-pure/src/index.ts](https://github.com/owenservera/vivim-omega/blob/main/plugins/vivim-nlcl-pure/src/index.ts) | exports, tokenizer, grammar, grounded WorldModel | inspected earlier |
| Ω reprogramming | [vivim-omega/plugins/vivim-director/src/index.ts](https://github.com/owenservera/vivim-omega/blob/main/plugins/vivim-director/src/index.ts) | teach/rules as data, not codegen | inspected earlier |
| VOMEGA semantics | [VOMEGA/README.md](https://github.com/owenservera/VOMEGA/blob/main/README.md) | additional semantic-OS lineage; resolve exact parser and commands from its index | inspected earlier |
| VIVIM Host dispatch | [vivim-web-harness/apps/vivim-host/control/dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) | plan digest, owner approval, exclusive effect claim, journal | inspected |
| VIVIM action recipes | [vivim-web-harness/studio/docs/architecture/AGENT-RECIPE-RUNTIME-V1.md](https://github.com/owenservera/vivim-web-harness/blob/main/studio/docs/architecture/AGENT-RECIPE-RUNTIME-V1.md) | existing Studio typed recipe runtime, effect fences | inspected earlier |
| VIVIM plugin design | [vivim-web-harness/docs/plugins/PLUGIN-TAXONOMY-AND-PLACEMENT.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/PLUGIN-TAXONOMY-AND-PLACEMENT.md) | contribution types vs execution placement | inspected |
| VIVIM KB types | [vivim-web-harness/docs/knowledge/kb-types.v1.json](https://github.com/owenservera/vivim-web-harness/blob/main/docs/knowledge/kb-types.v1.json) | 44 typed design definitions, source/authority/freshness | inspected earlier |
| VIVIM KB spec | [vivim-web-harness/docs/architecture/CORE-KNOWLEDGEBASE-TECHNICAL-SPEC-V1.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/architecture/CORE-KNOWLEDGEBASE-TECHNICAL-SPEC-V1.md) | Host-owned prototype, now superseded as deployment target | inspected earlier |
| TierFlow protocol | [tierflow/src/protocol.ts](https://github.com/owenservera/tierflow/blob/main/src/protocol.ts) | Intent / Plan / JobSpec / RunMeta / BusEvent | inspected earlier |
| TierFlow separation | [tierflow/docs/PROJECT-AGNOSTIC-DOGFOOD-AND-ADAPTERS.md](https://github.com/owenservera/tierflow/blob/main/docs/PROJECT-AGNOSTIC-DOGFOOD-AND-ADAPTERS.md) | universal tiers vs HQ-specific adapter | inspected earlier |

## Local checkout mapping

- HQ: `C:\0-BlackBoxProject-0\repos_hq` **or** `C:\0-BlackBoxProject-0\repos-hq` (probe actual folder; do not assume).
- TierFlow: `C:\0-BlackBoxProject-0\tierflow`.
- VIVIM: `C:\0-BlackBoxProject-0\vivim-web-harness`.
- Ω: `C:\0-BlackBoxProject-0\vivim-omega`.
- VOMEGA: `C:\0-BlackBoxProject-0\VOMEGA`.

## Historical discussion recovery (not independently re-fetched)

Previous conversations distinguished (1) original VIVIM `NLCL → ActionPlan → ExecutionKernel` and permission gate, (2) September Ω 17 glyph language/mind/director, (3) VOMEGA semantic compiler and visual feedback investigations, (4) October Host universal text control and action contract, and (5) November HQ kernel/writable wiki design. Dates and former implementation placement are historical until verified against code. Treat this map as the investigation entrypoint, not permission to rewrite historic systems.
