# Source-backed command-space census — Round 2

**Date:** 2026-11-11. **Evidence level:** current public GitHub source inspected, isolated Labs tests performed; **no execution on owner's Windows Host or HQ index**. Do not confuse code-source records with verified production access.

## Known real registrations at inspected public source

| Real source | File / exact source blob SHA | Observed | Not proven |
|---|---|---|---|
| Current Host typed catalog | `owenservera/vivim-web-harness/apps/vivim-host/control/action-catalog.v1.json` blob `c423d5933c28ca9f534028aeaffd7929f767c269` | Catalog v6, **19** source-described typed actions across appearance/control/devices/machine/network/personalization/privacy/system | A usable email-send command, contact lookup, device grants, current effective availability, Windows E2E |
| TierFlow UI core vocabulary | `owenservera/tierflow/src/ui/types.ts` blob `32d869d1528ff2fa2e42fee9ca091e5544d32eb6` | **8** typed `UiCommandKind` values, including world/draft/publish/rollback/state | Host-mediated effect parity, UI publisher gate readiness, actor permissions |
| HQ source inventory | `owenservera/repos-hq/hq/live-graph/wave1/lane-c/registry-adapter.mjs` blob `d2951ef7920c9ae5dc61a7b64b289d2c4b6ec1cf` | Existing `inventory()` computes HQ action records and CLI roots from `hq/lib/actions/*` + dispatcher source, with digest/source evidence | Full current action/CLI counts until this function runs against the actual installed checkout; runtime availability |
| HQ unified KB | `owenservera/repos-hq/hq/lib/kb.mjs` + `hq/lib/search.mjs` | Existing `kbQuery()` combines wiki, documents, decisions, terms, session, note, capability, product records; existing FTS5 content search | Personal authorization, current index population, freshness or permission |

**Remote source-read is not a production command-space audit.** Our sandbox does not mount the owner's PC repositories or `.hq/index.db`; the local source census therefore reports **UNKNOWN**, not zero commands available. Run the report on the actual four-repo checkout to populate evidence.

## Actual local source-audit command

From the Labs checkout, with the three repos available and configured by environment variables:

```powershell
cd C:\0-BlackBoxProject-0\vivim-labs\nlp-command-ui-lab
bun run census
bun run census:full
```

`census` enumerates per-source family, action count, readiness distribution, language metadata, argument coverage, display-glyph projection, SHA/digest evidence and missing owner integrations. `census:full` displays every discovered action descriptor. It **never sends, applies or approves** a command, and by default persists no data. The output is a local source observation, not a guarantee that every platform effect entrypoint is discoverable.

## Remaining first-class gaps

1. **Product breadth:** Host's typed control catalog is a narrow user-facing command subset. The broader Studio/browser capability set, provider account routes, Windows/Android activities, HQ MCP/skills, TierFlow programs and historical registries are not yet joined into a provenance-checked *complete* crosswalk.
2. **Same ID vs same operation:** `domain.verb.object`, `ui.*`, Omega `*.op@1` and Host dotted IDs need an approved equivalence map with version, source and authority; string matching is disallowed.
3. **Menu metadata:** many typed actions lack `entityTypes`, `sourceId`, resolver port, locale aliases, icon/visual roles, native affordance categories and relation semantics. We must index these as evidence, not add them as a new authoritative registry in Labs.
4. **Availability:** code-discovered / wired / verified / permitted-for-this-actor / available-on-this-device / performed are distinct. The existing Host must issue those facts, and publisher/receipt evidence must be current.
5. **Graph/wiki linkage:** `command→handler→file→symbol→test→wiki section→capability` joins are patchy. The lab's realtime evidence panel makes missing edges visible; HQ remains the semantic graph owner.
6. **Personal graph:** the local HQ development SQLite is **not** the universal private user corpus. Reuse the shared Rust graph compute plus Host personal evidence read API when accepted; do not ingest private user histories into HQ just to make the demo work.
7. **Freshness and UX:** one current local catalog read isn't an authenticated subscription. Need versioned deltas, event invalidation, bounded parse latency and cancellation tests across PC/PWA.

## Wave order (scope the work to this lab)

1. Finish syntax/lexicon/visual affordance prototype and source audits in Labs (this round).
2. HQ issues one read-only, versioned catalogue projection covering relevant canonical registries, completeness by provider and identifier-mapping gaps. Do not create a second registry.
3. Host issues a scoped, read-only `context/contacts/command-availability` projection, with opt-in personal scope and no implicit model disclosure.
4. TierFlow consumes the same view in the existing renderer and authenticated publisher, retaining one authoritative World/UI revision writer.
5. Verify desktop/PWA: latency, caret, Tab, accessibility, stale request cancellation, negative/compound text and production receipt truth. Only then consider promotion.