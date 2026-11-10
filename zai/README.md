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

## Round lifecycle

1. Clone exactly four approved repositories at fixed full SHAs; verify public-release clearance before downloading any public-sensitive history.
2. Discover conceptual overlaps, unique components, orphans and competing authorities across **all four**, including distinct code and vocabulary.
3. Choose best of breed or compositional target with explicit evidence and risk gates; log strategic decisions as **proposals** until owner-approved.
4. Implement only the authorized bounded round, preserve unique functionality, verify consumers and rollback.
5. Produce four bundle checkpoints, tests, manifest and decision/open-item deltas. Even unchanged repositories must appear in the manifest.
6. Record remaining risks and the next executable integration queue. HQ index/roadmap generation is the target, never a new independent status store in Labs.

**Public safety:** Do not put secrets, owner sessions, raw private logs, credentials, browser profiles or confidential archive content in these files, example fixtures, bundles, reports or paths. PRIVATE archives belong exclusively in an owner-approved private destination and must not be mirrored here.
