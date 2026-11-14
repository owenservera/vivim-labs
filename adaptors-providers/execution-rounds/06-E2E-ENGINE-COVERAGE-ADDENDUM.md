# R002 E2E engine coverage addendum — Capture Studio → Host World → HQ/KB → Rust → TierFlow/PWA
**Date:** 2026-11-14  
**State:** ADDENDUM / REQUIREMENT EXPANSION, not implemented or verified.  
**Ownership:** Labs orchestration only. No ARBOR code change is authorized by this document.  
**Compatibility:** Preserve the six W0 A–F work orders already running. Apply this addendum starting in G0 handoff and W1+ work preparation; don't revise in-flight branches.

## Product completion invariant
"Provider Hub mock complete" means a realistic provider observation can enter the actual ARBOR internal Source/Capture Studio seam, be independently admitted as evidence by existing Host World/ownership, become an incremental privately scoped knowledge/index/graph projection using existing HQ + Rust machinery, be queried and displayed through the existing Studio / TierFlow / PWA, and produce an inert exact-target action proposal with approval and durable mock-receipt semantics. **Do not pass by wiring JSON directly to UI or merely adding one GraphPort interface.**

```text
[Signed-in web app OR approved owner export]   future live source; no provider API
          |
[Studio Capture: SourcePort / selector intelligence / accepted sessions]  fake transport in mock
          |
[Host intake + identity + source grant + provenance + receipt + privacy/retention]
          |
[Normalized versioned events and attachments/metadata / source revisions]
          |                           \
[HQ semantic indexing + KB query]  [Native Rust scoped graph/algorithms]
          |                           /
[Source-linked context, search, recall, explain, timeline, UI projections]
          |
[Studio + one TierFlow publisher + existing paired PWA]
          |
[Draft / exact target / Host plan / owner approval / fake action receipt]
```

## Existing evidence / gaps (re-pin against current ARBOR at G0)
- `studio/src/studio/adapter-factory-service.mjs` learned profiles intentionally permit READ features only and `sendPermission=false`. The hub may reuse learning for **read observations** but cannot promote it to a sender. Candidate read selectors require provider-specific current evidence and holdout verification.
- `studio/src/studio/adapter-accepted-session.mjs` derives bounded accepted DOM-turn read events; its metadata expressly lacks verified native account or thread identity. Those identities cannot be guessed from DOM selectors or URLs.
- `apps/vivim-host/conversations/studio-journal-evidence.mjs` correlates journal evidence; it is not a second effect writer.
- `apps/vivim-host/control/dispatcher.mjs` is source-owned plan/approve/claim/effect/receipt authority. Provider Hub must not add direct send endpoints or self-issued grants.
- `native/hqkg/src/scoped_graph.rs` checks scoped structure and provenance fields but does NOT authenticate owner access. The actual Host identity/grant must bind reads and graph writes/projections.
- Existing `hq/` operational indexing, source search, wiki and semantic registry are **development-domain machinery** by default. Do not inject personal provider bodies into public development wiki, indiscriminately reuse dev namespace or store private data in dev Git logs. Reuse bounded algorithms and contract-verified personal scope, or expose separate secure personal projections through Host.
- Existing `apps/mobile-relay` and `packages/tierflow` own mobile and presentation; do not create a parallel PWA or publisher.
- ARBOR Proposal 5 says ARBOR should contain all selected code internally. Donor projects and Labs must not become production dependencies.

## Required engineering work, if inspection finds the capability absent
### Capture Studio / source intelligence
1. **Surface contracts:** adapter descriptor for account/product/stream kinds, capabilities, accessible modes (export, signed-in browser read, effect verified/unavailable), provenance, source epoch, and bounded cursors.
2. **Source acquisition:** page/chunk capture with bounded polling and explicit stop/abort; owner-selected export importer; provider-specific multipart/media/attachment metadata; no raw credentials. E2EE gap and unsupported modes are first-class statuses.
3. **Adapter learning and repair:** reuse existing accepted saved-session evidence and read-only factory; measure selector drift, quality and holdout performance; never learn send selectors into an executable actor.
4. **Correct identity:** verify principal/account/source and exact thread identities independently from page URL; account switch invalidates read cursors, drafts and evidence.
5. **Capture lifecycle:** initial snapshot, incremental delta, tombstone, edits, duplicates, reordering, partial/unknown, error, rate limit, revoked source, offline, recovery, cancel and resumable import.
6. **Fixture parity:** fake StudioSourcePort emits *the same* versioned observation envelopes that future accepted Capture Studio port will emit; execute this with actual ARBOR code, not a separate fake demo.

### Host World / data trust
1. Grant/purpose/retention/owner scope at intake; no silent source discovery or reusing AI-chat session grants for Facebook.
2. Append-only source evidence and origin/account/content digest, revision and receipt; link provider-native IDs as claims, never as unconditional identity.
3. Durable dedup and delete/tombstone propagation; support source revocation and downstream search/cache invalidation.
4. Exact-target action intent binds provider/account/thread/recipient/text/attachments/revisions to current Host plan; owner approves through existing journal; claimed-without-receipt remains UNCERTAIN.
5. Synthetic receipts carry MOCK_ONLY/MOCK_RECEIPT and cannot be mistaken for live observed or recipient-delivered.

### HQ / Knowledge Base / personal context
1. Explicitly inventory **KB as user-facing personal knowledge** and **HQ as ARBOR's development intelligence**; establish the existing personal corpus/Host source owner before choosing code.
2. Introduce/adapt bounded typed transforms: provider content → accepted personal source references → local text/media indexing → source-linked search and answer/context retrieval; keep raw personal content in owner-protected data domain.
3. Ensure time-aware and entity/thread relationship queries (participants, events, message chronology, media, duplicate exports, reactions, cross-provider related-but-unmerged identities) and precise source citations.
4. Incremental reindexing on edits/deletes/revoke/source epoch; expired or revoked content never resurrects through cache/wiki/graph/context bundles.
5. One semantic index story with source coverage/freshness; no second full-text/embedding database by default. Test deterministic baseline first, optional local semantic search only if current architecture and memory budgets allow it.
6. Do NOT write personal social data to HQ's development wiki, git metadata, CI artifacts, public reports, or fixtures.

### Native Rust / HQKG
1. Reuse ARBOR's actual `native/hqkg` scoped graph/typed DAG and proven native engines, not a new parallel graph.
2. Identify measured hot paths appropriate for native computation: high-volume normalization, dedup/content hashing, timeline merge, incremental indexing, graph projection, query ranking; optimize only when a benchmark shows benefit.
3. Provide one bounded typed JS↔Rust protocol, golden fixtures and deterministic parity for ID/version/tombstone/source scope; a fake Rust port is only a preliminary test layer.
4. Model graph nodes/edges with private owner namespace, provenance, revocable source context and no authorization inference from `scoped_graph` structural validation.
5. Include cancellations, backpressure, memory limits, source revision CAS, replay and crash/refusal parity; avoid adding a second journal or native process scheduler with separate authority.
6. If native build/runtime unavailable in GitHub-only environment, report SOURCE_PREPARED and exact Cargo test UNRUN; never mark native integration accepted based on mocked GraphPort alone.

### TierFlow / PWA / conversation composer
1. Existing Studio and paired PWA both consume source-backed provider/read models through the same internal Host routes; no direct fixture imports in UI.
2. Offer account/source picker, grants, inbox, feed, media, thread, search, citation/freshness/status, progress/errors, draft/edit/preview, approval and receipt/recovery views.
3. Preserve one TierFlow revision/publisher guard and one Host command path; respect phone-auth and offline behavior. Offline drafts may persist locally; no offline queued effect replay.
4. Natural-language requests to read/search and draft messages resolve to typed plans; models/source text cannot activate a grant or effect.
5. Synthetic end-to-end demonstrator must run on ARBOR source through fake transport and fake World/Action/Rust adapters with explicit MOCK_ONLY marking. Then progressively replace fake ports with verified existing owners.

## Wave handoff mapping (apply once W0 agents have finished)
| Wave | Mandatory extension | Owning lane / gate |
|---|---|---|
| **G0** | Reconcile W0 truth: exact Capture Studio, personal KB/HQ, native Rust, Host source owners; identify what exists, missing and must be upgraded. | G0 records engine gap ledger, no retroactive edits to W0 agents |
| **W1** | Schemas for evidence/delta/tombstone/source identity; one hub service; mock World/Action/Rust/source ports match future owner contract. | A, B, C, D, E, F |
| **W2** | Add source capture lifecycle and incremental ingest; source-to-Host provenance/retention; local normalized KB and native Rust bridge design; UI consumes **same** routed hub. | A identity, B streams, C read/index projections, D/E UI, F jobs; G2 integrates |
| **W3** | Realistic fake Studio capture/exports for Meta, E2EE/attachments/identity drift; graph and source provenance scenarios. | A–D pack owners, E journeys, F adversity |
| **W4** | The other six providers emit identical accepted observation envelopes and knowledge/graph proposals, respecting product-specific gaps. | A–F pack owners, G4 consistency |
| **W5** | E2E source intake → knowledge/index → Rust graph → unified search/UI → action-plan mock receipt; cross-surface parity. | A/B/C/D/E/F; G5 verifies whole route |
| **W6** | Independent tests measure actual source/Host/KB/native/PWA closure and crash/revoke/tombstone semantics; inventory all live/device UNRUN. | A–F and G6 |

**Collision note:** Existing lane write scopes are narrow. A running W0 worker must not be retasked. G0 should produce a source-backed upgrade/change map, after which the W1–W6 integrators can amend lane-scoped work orders or issue a narrow extra lane/next round **before** anybody edits shared ownership paths. No parallel spontaneous changes to HQ, native core or Host root. Do not pretend these extensions are already included in every existing worker prompt.

## E2E acceptance scenarios (hard, no screen-only passing)
- **Capture→search:** fake Facebook post with exact account, content revision and source evidence enters Studio accepted source port → Host grant/admission → personal KB source-index → query returns source-cited post in PWA.
- **Messaging→action:** fake Messenger inbound thread → exact recipient and source-bound draft → current Host plan/approval → fake Host receipt; wrong thread/account or claimed-without-confirmation refused/UNCERTAIN.
- **Export→dedup:** selected user export for Instagram/Facebook twice → only one accepted logical unit; changed content version updates source revision, graph/search accurately reflect it.
- **Revocation→purge:** revoke Telegram or Gmail source grant while cached → source-linked search, graph projections and context hide revoked content, no stale resurrection.
- **Native parity:** replay the same 1,000 synthetic mixed-provider events through JS and the accepted Rust transform (if actually wired); identical typed result digest and bounded runtime.
- **Offline→reconnect:** paired phone displays known stale source state and drafts locally; reconnect must not replay a send or mint a receipt.
- **Cross-owner attack:** wrong World/account source data refused by Host, knowledge index, Rust scoped graph and UI query.
- **Provider drift:** accepted read selector becomes invalid; learned adapter returns degraded/read-only and requires held-out evidence; never upgrades to a send capability.
- **Attachment constraints:** large/partial media metadata and unsupported decryption surface explicit gaps, no fabricated content and no public data leak.
- **Unknown delivery:** fake browser action claimed then simulator crashes; Host receipt remains UNCERTAIN, even if UI saw a composer clear event.

## Evidence requirements for G2–G6
One source-pinned table per engine: path/owner/existing methods, proposed adapters, actual patches, executed command/exit, synthetic vs real, native tests, consent/revoke/identity cases, and UNRUN device/live acceptance. **A mock pass proves only a mock pass.**

## Repository restriction for this addendum
This file is published solely to `vivim-labs`. It grants NO authorization to modify ARBOR and does not alter any active W0 lane prompt or branch. Future ARBOR changes happen only when subsequent ChatGPT worker prompts are deliberately launched under approved repo policy.
