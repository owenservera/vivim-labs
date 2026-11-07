# Integration Assessment — TierFlow UI core and the vivim mobile-relay harness

**Date:** 2026-11-03  
**Scope:** Seat Weaver, lane L6, repo `vivim-ui` (`C:\0-BlackBoxProject-0\vivim-ui`). Six integration questions (U1–U6) between the TierFlow UI core (this repo) and the vivim mobile-relay / Host harness. Harness files live outside this repo and are cited as recorded evidence from the Lens evidence pack (2026-11-03), not re-verified here. Deliverable: exactly one file, `docs/integration-assessment.md`.  
**Method:** TierFlow-side citations confirmed in this session via `grep` and `Read` against the repo. No measurement was invented, no architecture was converted into a number, no state-mutating command was run, and no network access was used.  
**Statement:** This document is an assessment, not a GO/NO-GO decision; GO/NO-GO is recorded by L0.

**Vocabulary (frozen):** "Personal World" means the owner's world; "surface document" means the UI surface document. `worldId` is a legacy name for the surface document. The aliases "UI world" and "world document" are not used.

## U1 — Does GET /api/ui/worlds return 200?

**Question:** Does `GET /api/ui/worlds` (and `GET /api/ui/state`, `GET /api/ui/drafts`) return 200 on the TierFlow server?

**Answer:** No. Recorded 2026-11-03: `TIERFLOW_PORT=7799 bun run src/server.ts` → `GET /api/ui/worlds` → 404 `{"error":"not found","path":"/api/ui/worlds"}`; same result for `/api/ui/state` and `/api/ui/drafts`; legacy routes return 200 (SESSIONS-NEXT-STEPS-2026-11-03.md:50-57). Verified this session: `grep -n "handleUiHttpRequest\|UiSocketSession" src/server.ts` → no matches; `src/server.ts` imports nothing from `src/ui/store`. Handlers exist only in the surface modules (`src/surfaces/http.ts:199`, `src/surfaces/websocket.ts:156`), and `UiSocketSession` is constructed only in `tests/ui-websocket-surface.test.ts:130 and :446`; nothing constructs it or mounts the store in the server (GAP-001 open).

**Evidence:** SESSIONS-NEXT-STEPS-2026-11-03.md:50-57 (recorded run); `src/server.ts` (grep, this session, no matches); `src/surfaces/http.ts:199`; `src/surfaces/websocket.ts:156`; `tests/ui-websocket-surface.test.ts:130 and :446`.

**Status:** recorded (route response); handler absence measured this session by grep.

**Confidence:** High.

## U2 — Is the TierFlow WS-cursor loop measurably faster than the harness poll loop?

**Question:** Is there a measured p50/p95 showing the TierFlow `{epoch,seq}` cursor loop is faster than the harness apply-proposal + poll loop?

**Answer:** Could not measure — no numbers exist for either path. Harness: `apps/mobile-relay/generative-ui/phone-surface.mjs:55-70` POSTs `apply-proposal` then polls `commands/:id` up to 25×1200 ms (≤30 s); repaint polls status (72-77); pending lock line 14. TierFlow: `src/ui/realtime.ts:8-42` defines the `{epoch,seq}` cursor and `RealtimeSession` resume/gap semantics; confirmed unconsumed by the server (no `UiSocketSession` construction, no `RealtimeSession` wiring into `/ws`) — cursor logic is unit-tested only (`tests/ui-realtime.test.ts`, 14 tests, per repo sessions analysis; existence only, not a live benchmark, and no test suite proves live acceptance). `package.json` scripts define only `bench` (`bench/hotpath.bench.ts`, Rust hotpath inter-arrival p95); no `bench:ui-loop`. Required: a same-workload edit→repaint benchmark on both paths (`bun run bench:ui-loop --json` per U2 acceptance), needing (a) a mounted TierFlow server (blocked by U1) and (b) an identically timed harness phone-loop harness. Do not infer a number (GAP-009 open).

**Evidence:** `apps/mobile-relay/generative-ui/phone-surface.mjs:55-70,72-77` (recorded, harness repo); `src/ui/realtime.ts:8-42` (this session); `tests/ui-realtime.test.ts` (existence only, this session); `package.json:16` (`bench` only, this session); SESSIONS-NEXT-STEPS-2026-11-03.md:143.

**Status:** could not measure.

**Confidence:** Low.

## U3 — Authorization without a second identity system?

**Question:** Can one device credential authorize both the Harness phone and TierFlow without a second identity system?

**Answer:** Structurally yes, expressively incomplete. TierFlow: `Capability = "read" | "surface.write" | "program.write" | "exec"` (`src/security.ts:19`); policy from `TIERFLOW_CAPABILITIES` (`src/security.ts:96-108`); per-request `authorize` (`src/security.ts:130-136`); `capabilitiesFor` (`src/security.ts:142-149`). Harness: one device credential — `grantId` plus `deviceId` (SHA-256 of the device key, first 32 chars), `pending` → `active` only after owner approval on PC (`apps/vivim-host/apk/enrollment.mjs:101-130`); `apps/vivim-host/direct-phone.mjs:122-128` validates `x-vivim-grant-id` + `x-vivim-device-session` and scope membership, else 403 `DEVICE_SCOPE_DENIED`; owner approve/revoke `server.mjs:345-353`; schema `vivim.device-grant/1`. The shape maps (one device credential, per-request scope check) but (a) no documented harness scope → TierFlow capability map, and (b) the TierFlow union has no `ui.publish`, so publish/rollback cannot be expressed today (`docs/UI-CORE-SPEC.md:165`); mobile token should be `read` + `surface.write` (`docs/MOBILE-CLIENT-CONTRACT.md:21`) (GAP-002 open).

**Evidence:** `src/security.ts:19,96-108,130-136,142-149` (this session); `apps/vivim-host/apk/enrollment.mjs:101-130`; `apps/vivim-host/direct-phone.mjs:122-128`; `server.mjs:345-353`; `docs/UI-CORE-SPEC.md:165` (recorded); `docs/MOBILE-CLIENT-CONTRACT.md:21` (recorded).

**Status:** recorded.

**Confidence:** Medium.

## U4 — Does the document model cover the harness surfaces?

**Question:** Does the TierFlow document model (layers, objects, relationships, data flows) cover every harness surface type?

**Answer:** Partial. Harness catalog: `apps/mobile-relay/generative-ui/catalog.mjs:6` `TYPES = stack,grid,text,card,button,collection,profile-card,project-card,timeline,chart`; card props `title/description/sourceRef/basis/freshness` (52-58); chart items `{label,value}`, timeline/collection `{label,detail}` (59-66); `validateTree` root `{id:'root',type:'stack'}` (70-86); audience pruning `pc-private` (98-103); schema `vivim.ui-surface/1`, revisioned (90-92). TierFlow: `docs/WORLD-MODEL-SPEC.md` layers ground→structure→content→data→overlay (27-39); object `{id,layer,kind,shape,transform,props}` (43-45); relationships `contains|links|drives|mirrors`, acyclic forest (52-61); data flows bind state path → property (65-72); dynamic objects need an inbound flow (144). Mapping: card/profile/project → content-layer text/billboard static objects (provenance fields `sourceRef`/`basis`/`freshness` have no TierFlow equivalent); timeline → content objects linked by `contains`/`links` (no native order); chart → data-layer object driven by an inbound flow (direct analogue of the spec example, 106-130); text/button/stack/grid → text/billboard + `contains` hierarchy. Does not map cleanly: `sourceRef`/`basis`/`freshness` provenance, audience pruning (`pc-private`), schema mismatch (`vivim.ui-surface/1` vs TierFlow `schemaVersion`), 3D canvas vs 2D tree renderer. GAP-014: `web/world.js` (141, 132 bytes) never rendered in a browser; no WebGL2 context/screenshot/pixel assertion (SESSIONS-NEXT-STEPS-2026-11-03.md:62-65, 179-181).

**Evidence:** `apps/mobile-relay/generative-ui/catalog.mjs:6,52-66,70-86,90-93,98-103` (recorded, harness repo); `docs/WORLD-MODEL-SPEC.md:27-72,106-130,144` (recorded); `web/world.js` (recorded, unrendered); SESSIONS-NEXT-STEPS-2026-11-03.md:62-65,179-181 (recorded).

**Status:** recorded.

**Confidence:** Medium.

## U5 — One UI revision authority?

**Question:** Is there one UI revision authority, or two competing authorities?

**Answer:** Two today. Harness (GUI04 done, `docs/roadmap.json:1993-2012`): Host-owned CAS, approval, Undo; `ui_revisions` ledger; `POST /api/host/bootstrap/ui-proposal`; phone commits `baseRevision` (`apps/mobile-relay/generative-ui/phone-surface.mjs:105`); schema `vivim.ui-revision/1`. TierFlow: `src/ui/store.ts:34` `bun:sqlite`, `runtime/state.db` (:45); revisions keyed `(world_id, revision)` with `ui_current` pointer; CAS publish/rollback on `expectedRevision`; audit + idempotency in one transaction (`docs/WORLD-MODEL-SPEC.md:6`, `docs/UI-CORE-SPEC.md:119-122`); store mounted nowhere (U1) (GAP-007 open). One authority requires (a) a written ownership row — roadmap §2 assigns UI documents to TierFlow only if phase 3 returns GO, (b) demoting the other to a read-only projection with a documented handoff, (c) reconciling the two CAS bases; naming: `worldId` retained as the legacy name for the surface document (`docs/naming.md:38-40`).

**Evidence:** `docs/roadmap.json:1993-2012` (recorded, harness repo); `apps/mobile-relay/generative-ui/phone-surface.mjs:105` (recorded); `src/ui/store.ts:34,45` (this session); `docs/UI-CORE-SPEC.md:119-122` (recorded); `docs/naming.md:38-40` (recorded).

**Status:** recorded.

**Confidence:** Medium.

## U6 — Can the core run where the Host runs without a third process?

**Question:** Can the TierFlow UI core run under the Host's runtime without a third process, given the Host uses `node:sqlite`?

**Answer:** No as written. `src/ui/store.ts:34` imports `bun:sqlite`; `node -e 'require("bun:sqlite")'` → `MODULE_NOT_FOUND` (GAP-010). The Host uses `node:sqlite` (`apps/vivim-host/bootstrap/ledger.mjs:194`). Probe: `node:sqlite` works under Bun — Bun 1.4.2 exposes `DatabaseSync`, `StatementSync`, `Session`, constants, backup; CRUD/prepared/WAL succeed (roadmap §3 fact 11; GAP-010). A store port from `bun:sqlite` to `node:sqlite` is the proven-open option; `bun test tests/store-port.test.ts` does not exist. Whole Host under Bun (Studio, Chrome children) remains UNK-001; the owner default O3 is No, keep Node (not decided here).

**Evidence:** `src/ui/store.ts:34,45` (this session); `node -e 'require("bun:sqlite")' → MODULE_NOT_FOUND` (recorded, GAP-010); `apps/vivim-host/bootstrap/ledger.mjs:194` (recorded); roadmap §3 fact 11 (recorded); `tests/store-port.test.ts` absent.

**Status:** recorded (import and probe facts); UNK-001 unmeasured.

**Confidence:** Medium ("No as written"); Low for Host-under-Bun.

## What could not be measured and why

- **U2 loop latency:** no p50/p95 for the TierFlow cursor loop or the harness poll loop. A same-workload edit→repaint benchmark on both paths (`bun run bench:ui-loop --json`) requires (a) a mounted TierFlow server (blocked by U1) and (b) an identically timed harness phone-loop harness; `package.json` has no `bench:ui-loop` script.
- **U1 live 200 run:** starting the server opens `runtime/state.db` via `src/ui/store.ts:45` (`bun:sqlite`); no temp-DB env override is documented, so a live route run was not attempted.
- **UNK-001:** whole Host under Bun (Studio, Chrome children) — not measured; owner default O3 is No, keep Node.
- **UNK-002:** whether the phone renders the Personal World or only edits surfaces — not in the evidence pack.
- **Store port:** `tests/store-port.test.ts` does not exist; a `bun:sqlite` → `node:sqlite` port is unproven in this repo.

## Open owner decisions (not decided here)

- **O3 — Host under Bun:** evidence states the owner default is No, keep Node; UNK-001 (Host under Bun incl. Studio, Chrome children) remains unmeasured.
- **Revision authority post-GO:** which of the two authorities wins, conditional on phase 3 GO (roadmap §2 assigns UI documents to TierFlow only on GO); requires demoting the loser to a read-only projection with a documented handoff and reconciling the two CAS bases.
- **`ui.publish` capability:** whether to add publish/rollback to the TierFlow capability union (`src/security.ts:19`); not expressible today (GAP-002).
- **GO/NO-GO:** recorded by L0; this assessment makes no recommendation.

Other O-items (O1, O2, O4–O8) were not in the evidence pack and are not addressed here.

**Acceptance:** the file exists at `docs/integration-assessment.md`; all six questions are answered; every answer is evidence-backed or explicitly unmeasured; no test suite is claimed to prove live acceptance.
