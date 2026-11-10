# Z.ai Fullstack — VIVIM Four-Repository Convergence

This is a **public-safe campaign evidence and handoff workspace**, not a fifth VIVIM repository, canonical knowledge store, roadmap ledger, or alternate authority. Source of truth is each owning repository at a pinned commit; HQ owns generated cross-repo intelligence. All claims require repository, commit, path and evidence class.

## Repositories

| Repository | Folder here | Authority |
|---|---|---|
| `owenservera/repos-hq` | [repos-hq](./repos-hq/README.md) | Existing index, SQLite/Rust graph, context, documentation generation, orchestration |
| `owenservera/vivim-web-harness` | [vivim-web-harness](./vivim-web-harness/README.md) | Host, Android, effects, authorization, current product roadmap |
| `owenservera/tierflow` | [tierflow](./tierflow/README.md) | Reusable visual runtime, schemas and adapters |
| `owenservera/vivim-labs` | [vivim-labs](./vivim-labs/README.md) | Research, public-safe archives, evaluation evidence |

The user's local directory `vivim-ui` corresponds to GitHub `tierflow`: one project, not two. `SOFTWARE_HQ` is separate/private and **must never be cloned into this campaign, indexed as VIVIM, or exposed here**.

## Working files

- [Campaign charter](./CAMPAIGN-CHARTER.md) — scope, conceptual deduplication, maximal roadmap requirements.
- [Strategic decisions](./governance/DECISIONS.md) — append-only decision records and revision evidence.
- [Open items](./governance/OPEN-ITEMS.md) — unresolved issues and pending decisions, triaged at every round.
- [Decision protocol](./governance/DECISION-PROTOCOL.md) — how competing implementations are evaluated and authorized.
- [Stretch goals](./STRETCH-GOALS.md) — ambitious, gated nonessential objectives.
- [Round template](./rounds/ROUND-TEMPLATE.md) — required per-round reporting fields.
- [Bundle protocol](./rounds/BUNDLE-PROTOCOL.md) — four checkpoint bundles plus one manifest.

## Operating model — HQ first, productive from the first round

**`repos-hq` is Z.ai's actual operating tool, not merely another code repository to audit.** Use the existing HQ CLI, SQLite index, source adapters, Rust graph, context assembler and wiki generator to discover, route, implement, verify and hand off work. Fix and extend these tools when missing capabilities block engineering. Do not build parallel notebooks, status stores, indexes or generators in Labs.

**No discovery-only Round 0.** The first round must deliver tested code improvements. Inspect only what the bounded task requires; leverage Mission A–K evidence and previously generated HQ outputs. Start with working four-repo HQ project/path resolution, source freshness and useful read views.

## Round lifecycle

1. Clone exactly four approved repositories at pinned full SHAs; establish a minimal safety/command baseline only as needed to begin implementation.
2. **Ask HQ first** for source ownership, dependencies, candidate implementations, relevant context and verification. When HQ is wrong, stale or missing data, repair the existing HQ machinery and test its improved answers as part of the round.
3. Implement bounded high-value source integration or cleanup; discover conceptual overlaps and unique functionality **while working**, not in a separate exhaustive audit phase.
4. Choose evidence-backed canonical/composite solutions, preserve unique components and record decisions as proposals until owner-approved.
5. Re-index changed source, regenerate affected HQ documentation/read views, measure freshness and record what **the next round can now answer without repo-wide exploration**.
6. Deliver actual code/test improvements plus four bundle checkpoints, a manifest, decision/open-item deltas and a short next-action queue. Unchanged repos remain pinned without filler commits.

**Compounding requirement:** each round must improve both VIVIM and the efficiency of the next round's HQ-assisted work. By Round 4, agents should normally navigate by HQ queries and scoped context, drilling into raw files only to confirm source, correct HQ gaps or perform edits. HQ-derived answers always carry source SHA and verification status. Never blindly trust a stale index.

**Public safety:** Do not put secrets, owner sessions, raw private logs, credentials, browser profiles or confidential archive content in these files, example fixtures, bundles, reports or paths. PRIVATE archives belong exclusively in an owner-approved private destination and must not be mirrored here.
