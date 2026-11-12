# HQ source-completeness and integration gap register

**This is a research discrepancy list, NOT a competing canonical issue tracker.** All entries need HQ/Host source-linked evidence, ownership and acceptance before production.

| Gap | Current state | Desired HQ/Host contract | Verification |
|---|---|---|---|
| G01 — mixed registry identity | HQ domain.verb.object vs Host action IDs vs TierFlow UI commands / Omega legacy ops | Versioned, reviewed many-to-many identity crosswalk with no inferred equivalence | Unmatched IDs, aliases, renames and collisions exposed |
| G02 — missing universal command discovery | HQ Lane C inventory observes HQ action/CLI roots; Host catalog separate | Unified **derived** inventory including Host user commands, HQ actions, CLI/MCP, TierFlow commands, legacy-only candidates, capability status | Per-source SHA, completeness denominator, coverage and stale badges |
| G03 — user language/slot metadata | HQ registry has purpose/status but not user-facing slot types, aliases, locale, icon intents | HQ indexes Host `language.verbs`, `args`, frames, entity types, menu providers, symbolic family, icon/semantic role | Audit every action for missing role/menu metadata |
| G04 — runtime truth | `wired` / `source_wired` means source code present, not usable | Host-verified effective access by actor/device/world, evidence receipts and freshness | Compare `declared`, `source_wired`, `availableNow`, `permitted`, `verified` |
| G05 — contact discovery | No verified scoped read-only Host contact source used in Lab | Authenticated/bounded contact search with provenance, no private-data publication | Fixture vs local vs Host evidence; negative authorization cases |
| G06 — source delta and tombstones | Lab rereads on demand; cross-repo source refresh may lag | HQ graph delta subscription with revision, rename/delete/tombstone evidence | No phantom commands after removal; correct digest update |
| G07 — visual affordance registration | Icon associations are prototype view mappings | Typed semantic icon declaration / affordance metadata discoverable in HQ without stealing TierFlow rendering authority | Accessible label, aliases, SVG/icon lookup stability |
| G08 — language-to-visual parity | Lab uses Host for interpretation and adds optional visual experiments | Shared pure projection conformance across desktop, phone, HQ export and TierFlow | Golden fixtures and byte-equivalent token spans |
| G09 — compound/negated requests | Source history records prior clause loss concerns, current status unverified here | Explicit structured AST, refused/clarification outcomes; never silently erase negation | Adversarial sentence matrix |
| G10 — verification provenance | No observed live production receipt in this Lab | Read-only bounded status/effect evidence from Host with scoped permissions | Offline, stale, revoked, unknown and denied cases |

## Already existing canonical sources to reuse

- `repos-hq/hq/live-graph/wave1/lane-c/registry-adapter.mjs` and `.../commands.mjs` — HQ source evidence and action/CLI inventory, direct import in this Lab.
- `repos-hq/hq/catalogue-identity/` — identity projection, unapproved equivalences retained as candidates.
- `repos-hq/hq/lib/capabilities.mjs` — capability source/export reader, not itself a UI action catalog.
- `vivim-web-harness/apps/vivim-host/control/intent-planner.mjs` — `languagePack()` / `describe()` / `planText()` semantic and typed command reader.
- `vivim-web-harness/apps/mobile-relay/control-language.mjs` — pure per-keystroke NLCL.
- `tierflow/src/ui` — real UI revision/publisher contracts.

**Important:** any zero count when a repository is not mounted is UNKNOWN, not zero available commands.

## Round 2 additions — living knowledge lens

| Gap | Why it matters | Proposed existing owner |
|---|---|---|
| G11 — contextual wiki query SLA | HQ `kbQuery` loads a unified model; keystroke-driven queries require bounded budget, debounce, cancellation and instrumentation | HQ search/KB read owner + Lab benchmarks |
| G12 — development vs personal isolation | Live UI needs both project code truth and consented private conversation history, but HQ development SQLite is not the personal store | Host private read port + shared Rust graph compute |
| G13 — genuine wiki page provenance | Lab composes an ephemeral page, not a generated authoritative wiki document; card source/recency and links need exact revision evidence | HQ wiki/KB export and Host personal evidence |
| G14 — ambient source subscriptions | Source snapshot may go stale as a repository indexes or owner revokes a provider; no event stream yet | HQ revision/delta reader + Host revocation truth |
| G15 — compositor openness | Presentation hints (font, icon, border, color, spatial layout) are view metadata, not new command semantics or permissions | TierFlow renderer contract, Labs visual experiments |
| G16 — model-facing disclosure | Showing a personal snippet locally is not permission to send it to ChatGPT/Claude/Gemini | Host explicit per-destination context disclosure decision |