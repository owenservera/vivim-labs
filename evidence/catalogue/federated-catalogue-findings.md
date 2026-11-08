# Labs evidence — federated catalogue candidates, reusable assets and duplicate authority

**Record:** 2026-11-06 · **Factory:** 02 Catalogue of Catalogues · 03 Universal Asset Library
**Status:** DISCOVERED / SPECIFIED. Not verified capability. **Maturity max 4** (integrated) — nothing
reached environment-verified (5) or production-verified (6).

## Source pins (all read-only)

| Repo | SHA | Role |
|---|---|---|
| `owenservera/tierflow` | `ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7` | UI schema / command runtime |
| `owenservera/repos-hq` | `bfc062e6faf10171070c3d8c06aeb1ef85c47349` | canonical KB / semantics |
| `owenservera/vivim-web-harness` | `09382c61f74a304f120b05445f6dfa900b58494b` (`main` ref; worktree on `ceo/mobile-one-path-2026-11-05` @ `65495e6`) | Host / browser / Studio / Android |
| `owenservera/vivim-workstation` | `f21083bece6c1621501c197324a3924495c5347e` | ops + knowledge index |
| `owenservera/vivim-labs` | `074fd48fd86d690c6133df9ddb4bc3fe288155cd` | this repo |

Data files: `corpora/catalogue/catalogue-index.json` (9 domains), `asset-inventory.json` (32 assets),
`duplicate-authority.json` (8 collisions).

## The nine domains — canonical owners and live registries

| Domain | Canonical owner (file:line) | Live registry | Count | Maturity |
|---|---|---|---|---|
| Commands | `vivim-ui` `src/ui/commands.ts:52` | same file | 8 kinds | 3 (unit verified) |
| Components | `vivim-ui` `src/visual-runtime/document/schema.ts:12` | same file | 10 types | 3 |
| Capabilities | `repos_hq` `hq/lib/capabilities.mjs:14` | `hq/capabilities/registry/contracts.v1.json` | 16 | 2 (all `status=proposed`) |
| Providers | zcode `~/.zcode/v2/provider_config.json` + harness `harness/src/providers/index.mjs:18` | both | 16 / 8 | 3 / 2 |
| Agents | `repos_hq` `hq/registry/agentic-file-registry.v1.json:1` | same | 217 / 133 patterns | 3 |
| Models | zcode `~/.zcode/v2/provider_config.json` | `modelConfigRules.providerModelRules` | 180 | 2 (no test) |
| Algorithms | **none exists** — per-module owners | none | n/a | 3 each (module + test) |
| Assets | **none exists** | none | 32 catalogued | 2–4 per asset |
| Workflows | `repos_hq` `hq/lib/components.mjs:14` | `hq/work/orders.json` | 45 | 3 |

**Two domains have no registry at all** — Algorithms and Assets. That is the single most actionable
gap for factory 02: they are currently implicit in module layout and in scattered files.

## Duplicate authority — 8 collisions, 3 high severity

| # | Concept | Locations | Should win | Severity |
|---|---|---|---|---|
| **D5** | **CAPABILITY ID namespace collision** | `vivim-web-harness/docs/capabilities/harness-capabilities.json` (`CAP-001..246`) vs `docs/capabilities/atomic-registry.json` (`CAP-01..354`) | atomic-registry | **HIGH** |
| **D1** | UI document component types | `vivim-web-harness/apps/mobile-relay/generative-ui/catalog.mjs:7` ≡ `vivim-ui/src/visual-runtime/document/schema.ts:12` — identical 10 types | TierFlow | **HIGH** |
| **D3** | Schema id + limits | `vivim.ui-document/1` (`schema.ts:6-9`) vs `vivim.ui-surface/1` (`catalog.mjs:3,8`) — six identical limits | `vivim.ui-document/1` (surface = projection) | **HIGH** |
| D2 | Component lists | `components/catalog.ts:8` (9) ≡ `accessibility.ts:2` (9) vs `document/schema.ts:12` (10) | one list, one repo | medium |
| D4 | Provider list | `harness/src/providers/index.mjs:18` vs `scripts/capability-catalog.mjs:33-34` (stale `SEVEN_PROVIDERS`, omits grok) vs `studio/profiles/` (3) | harness profiles | medium |
| D6 | Receipt format | `repos_hq/hq/lib/receipts.mjs:26` vs `vwh.capability-receipt/1` (`capability-catalog.mjs:23`) | **both — link, never merge** | medium |
| D7 | Component registry doc vs impl | `repos_hq/docs/COMPONENT-REGISTRY.md:1` vs `hq/lib/components.mjs:12` | implementation | low |
| D8 | Workflow status | `hq/work/orders.json` (45) vs `docs/roadmap.json` (176) vs `ops/runbooks/` (4) | split ownership | low |

**D5 is the highest-value finding.** Two capability registries use overlapping `CAP-` id shapes with
different meanings. A federated catalogue that ingested both without namespace discipline would
silently collide. Any consumer resolving `CAP-xxx` across repos would get an ambiguous or wrong
row. This is exactly the failure the program doc warns about — "another independent authoritative
registry" — and it already exists in the source material.

## The live index is empty where the docs are full

Read-only query of the live HQ SQLite (`.hq/index.db`, active WAL):
`capability_node`, `map`, `package`, `contract`, `evidence`, `semantic_id`, `semantic_alias`,
`semantic_link`, `receipts` → **all 0 rows**.

The 791 capability mappings described in `docs/CAPABILITY-REGISTRY.md:27` are **not** in the live
index. Likewise `hq/lib/semantic-registry.mjs` and `hq/lib/receipts.mjs` have 0 live rows, and
`hq receipts` prints "No receipts". So the *documented* capability graph and the *queryable* one
have diverged: the authority is documented but not queryable. A Labs catalogue that linked to the
documented graph would link to something that returns nothing at runtime. **This is a convergence
blocker, and it is an owner decision (repos_hq), not a Labs fix.**

## Reusable assets — 32 catalogued, 10 highest-value

| Asset | Owner repo | Exercising test | Maturity |
|---|---|---|---|
| `components/catalog.ts` | tierflow | `components.test.ts` | 3 |
| `components/layout.ts` | tierflow | `layout.test.ts` | 3 |
| `components/accessibility.ts` | tierflow | `components.test.ts` | 3 |
| `document/schema.ts` | tierflow | `document.test.ts` | 3 |
| `ui/commands.ts` | tierflow | `ui-commands.test.ts` | 3 |
| `host/fixture.ts` | tierflow | `integration-2/real-a-h.test.ts` | **4** |
| `knowledge/fixtures.ts` | tierflow | `integration-2/real-a-h.test.ts` | **4** |
| `generative-ui/catalog.mjs` | vivim-web-harness | `generative-ui.test.mjs` | 3 |
| `harness/src/providers/index.mjs` | vivim-web-harness | `providers.test.mjs` | 3 |
| `scripts/capability-catalog.mjs` | vivim-web-harness | `capability-catalog.test.mjs` | 3 |

Each was **re-executed this session** from a scratch copy: 33/33, 55/55, 25/25, 11/11, 14/14, 25/25 —
all green. These are real, exercised assets, not empty folders.

**Empty scaffolds flagged** (a directory that is not yet an asset):
`vivim-labs/agentic_excelence/` (README + TOPICS only), `vivim-workstation/agents/handoffs/`,
`vivim-workstation/agents/roles/`, `vivim-workstation/agents/instructions/`. Four further readme-only
candidates were inspected and **dismissed** — they had real content behind them.

## An evidence index already exists — Labs must LINK, not copy

| Existing index | Rows | Note |
|---|---|---|
| `vivim-web-harness/catalog/harness-capabilities.json` + `.csv` | 246 | `vwh.harness-capabilities/1` |
| `vivim-web-harness/catalog/receipts/` + `vwh.capability-receipt/1` | — | the only contract that can move a row past `not-tested` |
| `vivim-web-harness/docs/capabilities/atomic-registry.json` | 354 | |
| `vivim-web-harness/docs/knowledge/kb-*.v1.json` | 44 / 30 / 33 / 246 | |
| `vivim-web-harness/docs/library/CROSS-REPO-SOURCE-MAP.json` | 12 repos | provenance only |
| `repos_hq/hq/registry/kb-binding.v1.json` | 44 | cross-repo bindings |
| `vivim-workstation/evidence/reports/CAPABILITY-MATRIX.md` + `ops/manifests/*.json` | — | |

**The five things Labs must link to rather than copy:**
1. `vivim-web-harness/docs/capabilities/atomic-registry.json` (354)
2. `vivim-web-harness/catalog/harness-capabilities.*` + `catalog/receipts/` — the only contract that
   can move a row past `not-tested`
3. `repos_hq/hq/registry/agentic-file-registry.v1.json` (217 records / 133 patterns)
4. TierFlow's command/IR/schema authorities — the `name@version` id space has a contract
   (`ir.ts:52`) and a schema (`hq.semantic-registry/1`) but **zero registered rows**
5. `repos_hq/hq/registry/kb-binding.v1.json` (44) + `vivim-workstation/knowledge/architecture/cross-repo-authority.md`

The Labs catalogue is therefore a **linking index over these five**, not a sixth registry.

## What this does NOT prove

- **No repo's full test suite was run** for this deliverable except where noted above — the asset
  re-execution covered 6 asset files, not whole repos. No blanket green-suite claim is made.
- **No device, browser, live-provider, Studio or Android exercise.** Hence nothing above maturity 4.
- TierFlow `tier1/2/3` runtime, `apps/vivim-host`, `apps/browser-companion`, `apps/share-web` and
  `src-tauri` were **not** asset-inventoried — a coverage gap in the inventory itself.
- `apps/share-web` looks thin but was **not proven** an empty scaffold.
- `.bak-*` agent files were excluded **by name, not diffed** — 391 excluded files were not inspected.
- The 0-row live-index findings are **snapshot-accurate only**: `.hq/index.db` has an active WAL.
- Counts are point-in-time (2026-11-06) and will drift as repos move.

**Maturity: discovered / specified.** Every row here needs an owner-approved promotion path before
it becomes authority. Labs asserts no authority over any row.
