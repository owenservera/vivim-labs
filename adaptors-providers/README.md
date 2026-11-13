# VIVIM Studio — adaptors-providers
**Round:** R001 landing scaffold · **Status:** LABS_PROTOTYPE / NOT WIRED · **Date:** 2026-11-13

## Product goal
VIVIM should let the owner interact with their own digital services **from VIVIM**: discover accounts, view a unified inbox and feed, read/search past material, compose, preview and explicitly approve a message or other action, and see a verifiable provider result. Facebook Messenger is the first concrete **outbound messaging** reference scenario. Other providers follow the same family-of-capabilities design; a provider tile does not imply verified browser access or legal/technical support.

## Navigation
- [Catalog](catalog/providers.json) — machine-readable first wave, 32 provider/product candidates.
- [Stream envelope](contracts/stream-envelope.v0.schema.json) — identity, provenance, observed state, cursor and permissions.
- [Host action intent](contracts/host-action-request.v0.schema.json) — **request only**; never a browser action, grant or send receipt.
- [UI surfaces](contracts/studio-component-map.json) — UI vocabulary, states and Host/TierFlow binding targets.
- [Landing prototype](ui/index.html) — mobile-first exploratory UI, synthetic thread only, SEND DISABLED.
- [Coverage and decisions](PROVIDER-STREAM-MATRIX.md) — stream classes, priority, restrictions and Facebook flow.
- [Product handoff](WORK-ORDERS.md) — phase-ordered product engineering tasks and acceptance.
- [Source pins](evidence/source-pins.json) — exact commits inspected; source presence is not release proof.

## Intended user journeys
**1. Facebook / Messenger**: choose Messages → connect existing Facebook session through Host → explicit grant for thread discovery and read → list threads → select recipient/account/thread → see cited thread contents and freshness → draft → preview destination and exact bytes → owner approval → Host-controlled browser send once → authoritative receipt and reconciliation. A page read, typed composer or optimistic UI is **not** proof of delivery.

**2. Facebook content**: choose Posts, Comments, Reactions, Photos or Friends → source permission → supported user export or explicitly verified bounded browser read → progress, preview and provenance → local knowledge/query views. No silent friend graph harvesting.

**3. Other services**: reuse the same inbox/feed/library/timeline/data-stream building blocks; each provider adapter declares supported modes, boundaries and evidence independently. Do not fake equivalence between a read-only export and a live two-way channel.

## Ownership and boundaries
- **Labs:** candidate taxonomy, portable data contracts, *synthetic* UI/fixture and tests.
- **vivim-web-harness / Studio:** existing Chrome profile and signed-in browser actor, capture factory, source grants, Host action approval, provider sends, World/evidence/journal/receipts and product roadmap. No new browser actor or direct third-party API.
- **repos-hq / shared Rust:** existing personal-scoped graph, indexing, query and deterministic computations; NOT a second graph or persistent provider store.
- **tierflow:** declarative, read-only views and approved command projections; one publisher.
- **apps/mobile-relay:** existing single PWA/phone surface; no second Android app.

Never store cookies, passwords, tokens, live conversations, attachments, personal friend lists or recipient information in Labs/Git. Even if this repository's visibility changes, materials here remain publication-safe.

## Current truth
R001 is a **landing scaffold only**: provider entries are unverified candidates; screenshots/preview represent fixtures. The prototype neither authenticates to Facebook nor executes a message send. Actual outbound messaging requires per-provider evidence, permissions, terms review, signed-in desktop Chrome, bind-to-thread safety, independent receipt tests and real-user acceptance. A mobile browser alone cannot inherit a PC Chrome sign-in.

## Run prototype
Serve the repository root with any static HTTP server and open `/adaptors-providers/ui/` (the browser fetches `../catalog/providers.json`). Example: `python -m http.server 8080`. No dependencies or network calls to providers. The UI displays a **synthetic** Messenger thread. Run `node --test adaptors-providers/tests/*.test.mjs` from the repo root to validate the catalog and static handoff.

## Names
Retain the owner-requested directory spelling `adaptors-providers`. Distinguish **provider** (Facebook), **product** (Messenger), **surface** (facebook.com/messages), **account** (owner-scoped signed-in principal), **stream** (thread/message/post/etc.) and **transport** (export, file, observed web). A product may have multiple sources, accounts and streams.