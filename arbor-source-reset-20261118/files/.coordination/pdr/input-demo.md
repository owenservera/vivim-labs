# PDR INPUT — arbor-demo lane (round 2, 2026-11-14)

Consensus input only — not decisions. Evidence verified live this session (route sweep + HTTP probes).

## A. Complete route/asset inventory (verified)

Server: `apps/vivim-host/server.mjs` (bun, port `VIVIM_HOST_PORT||43187` :984). Boot: `bun apps/vivim-host/server.mjs`. Host LIVE 200 at sweep time.

1. **Static ASSETS exact-string map** (server.mjs:73-99; MIME :61-71; handler :383-388 — NO catch-all, unknown `/demo/*` → 404):
   - `GET /demo` → demo/demo.html (cockpit = dev page, page 3)
   - `GET /demo/controller.js`, `GET /demo/demo.css`
   - `GET /demo/fixture/*.json` — 7 fixture files
   - `GET /demo/feedback/inbox.json` — journal snapshot
2. **Dynamic demo routes:** `POST /demo/feedback` (server.mjs:348-369) — read-modify-write of inbox.json, no lock = THE concurrent-writer drop defect. Sole direct disk write in server.mjs.
3. **Existing product/API routes (reuse-first inventory — no new machinery needed):**
   - `GET /api/host/status` :379 (journal + `bridge/commands.jsonl` reads)
   - `GET /api/host/bootstrap/{status,projection,ui-revision}` :445-453
   - `GET /api/host/data-intelligence/{summary,registry}` :554-555
   - `GET /api/host/apk/{releases,download}` :608-614
   - `GET /api/host/control/{catalog,world,language,lexicon,recent,plans/:id}` :305+772 — NLCL action-catalog surface
   - `POST /api/host/control/{interpret,plan,execute}` :781-791 — EXISTING NLCL runtime (in-memory)
   - `POST /api/host/bootstrap/*` mutations map :397-408 (11 routes); onboarding family :818-966
4. **Client:** public/demo/controller.js — exactly 3 URL shapes: GET `/demo/fixture/index.json` (:115), GET `/demo/fixture/<step>` (:117; fail-closed same-origin guard :81), POST `/demo/feedback` `{round,approveAll,verdicts}` (:600).
5. **Journal:** public/demo/feedback/inbox.json — JSON array; entry schema `{round, approveAll, verdicts:[{id,verdict}|{itemId,screen,decision,notes,tags}], receivedAt}` (214 lines); poll-round.mjs validates (:15-21 `isRoundEntry`), read-only.
6. **Tests:** `apps/vivim-host/test/demo-{shell-assets,shell-tokens,fixtures,iframe-smoke}.test.mjs` — 25/25 green (verified post-recovery).
7. **Parked drafts** (docs/product/proposals/chat-wiki-baseline/): `state.mjs.draft` (5 exports: loadState/saveState/loadWiki/saveWiki/applyWikiEdit); `chat.html.draft` (fetches GET `/demo/state` :62, POST `/demo/chat-send` :73 — nonexistent); `wiki.html.draft` (fetches GET `/demo/wiki-data` :38 — nonexistent).
8. **Roadmap data:** docs/product/roadmap/roadmap.v1.json — buildQueue 3 items, schema `{itemId,order,status,note}`, item 1 `CHAT-BOX-BASELINE`.
9. **NLCL backbone (reuse-first):** apps/vivim-host/control/{command-intent.mjs (interpretControl), action-catalog.v1.json, dispatcher.mjs, intent-planner.mjs, journal.mjs}.

## B. Draft-evolution spec (convergence target for the parked drafts)

- **state.mjs.draft — survives as projection core.** 5 exports unchanged; add: journal read (JSONL per-line), file-surface projection (P2 Stage A), gate-round validation (P6, additive).
- **chat.html.draft — evolve:** GET `/demo/state` → GET `/demo/state.json` (ASSETS addition); POST `/demo/chat-send` → `POST /demo/mutate {schemaVersion:1, actor:"owner-browser"|"loop-agent"|"keeper", kind:"chat-send", text}` (P5 envelope + keeper r2 actor/schemaVersion fields, adopted); intent-IR chip rendering retained; 1.5s poll retained. NLCL binding per D1 — no demo-side interpreter.
- **wiki.html.draft — evolve:** GET `/demo/wiki-data` → GET `/demo/wiki.json` + GET `/demo/file-surface.v1.json` (both ASSETS additions; 1s poll RETAINED — P2 Stage A render path intact); `data-wiki-id` attributes retained for applyWikiEdit targeting; wiki edits → `POST /demo/mutate {kind:"wiki-edit", edit}`; roadmap render via read-through (D2).
- **Both pages: same-origin relative fetches ONLY** (promotion discipline, P7).

## C. Append-only JSONL migration plan (P5)

**Touchpoints (4):**
1. server.mjs `POST /demo/feedback` → generalized `POST /demo/mutate`: `appendFile` one line to `public/demo/journal.jsonl` (single write per entry, entries <4KB). `POST /demo/feedback` kept as alias for one release (controller.js :600 body unchanged), then retired.
2. ASSETS: `/demo/feedback/inbox.json` → `/demo/journal.jsonl` (MIME `application/jsonl`, text/plain fallback).
3. controller.js :600 — POST body shape unchanged; journal read path switches to per-line parse.
4. poll-round.mjs — `isRoundEntry(v)` SURVIVES unchanged (entry schema identical); file read switches JSON.parse-whole → per-line parse + blank-line skip.

**One-time migration:** inbox.json array → journal.jsonl lines (script, preserves receivedAt order; original retained as inbox.json.bak until gate).
**Concurrency:** single bun process serializes HTTP writes; loop-plane direct appends = single `appendFile` calls <4KB (line-atomic in practice); NO read-modify-write anywhere.
**Gate entries:** `{round:"owner-gate-<batch>", itemId, decision:approve|revise, receivedAt}` — additive validation, same file, per P6. **Record/execute split (keeper r2, adopted):** the journal entry is the RECORD only; the keeper lane is the sole writer executing the roadmap.v1.json mutation (one-writer rule) — my entry schema already carries itemId + decision + batch for unambiguous execution.

## D. Design points MISSING from synthesis v1 (demo lane)

- **D1 — NLCL binding for chat-send (largest gap).** P1 says NLCL-first but never states WHERE interpretation runs. Reuse-first ruling needed: mutate `kind:chat-send` routes through the EXISTING Host control runtime (`POST /api/host/control/{interpret,plan,execute}`, apps/vivim-host/control/*, action-catalog.v1.json). The wiki capabilities = action-catalog ADDITIONS registered via roadmap — NOT a second interpreter in the demo. Without this binding, "NLCL-first" silently becomes a sidecar interpreter = the duplicate-system failure mode. **Capability set (keeper r2, adopted):** 12 day-1 incl. `deletePage` as SOFT delete — right call for a reprogrammable surface (journal-traceable undo).
- **D2 — roadmap.v1.json serving for the wiki.** ASSETS copy (self-contained but drift risk) vs small read-through route `GET /demo/roadmap` (single source of truth on disk). Recommend read-through.

## E. P7 placement — demo lane implementation surface + break-assessment

**Under Option A (product routes day 1):**
- MOVES to product routes: the mutate POST (endpoint `/api/mutate` per keeper r2 naming, adopted — consistent with /api/host/* convention), state/wiki/file-surface reads, the page assets (chat/wiki html+js+css). Drafts' same-origin discipline SURVIVES — it is relative-URL, not /demo-specific; pages fetch same-origin product paths.
- STAYS in /demo: fixtures (7), journal.jsonl + poll-round.mjs, controller.js + cockpit (dev page = machine-artifact surface per P4).
- Re-planning (honest): code diff ≈ identical A vs B (same handlers, different pathname strings). The cost is COORDINATION: product routes + page-asset tree sit outside my sanctioned owned paths (apps/vivim-host/public/demo/**) → oracle-authorized scope expansion + host-control-keeper route registration per touch → the 3-min-cadence iteration becomes oracle-window-gated for any server.mjs touch. Mitigation: batch route authorization (one window covering the batch-1 route set). Tests: 25 parcel tests re-base to product paths (~1 sweep, mechanical). Estimate: +1 coordination window up front, then parity; not a stall.

**Under Option B (demo-namespace first):** zero new coordination. Risk = /demo pages drifting from future product routes. Mitigation = promotion contract fixed in the PDR BEFORE code (product URLs + route names), promotion lands as a BATCH-1 EXIT GATE (same batch, not deferred), same-origin relative-fetch discipline makes promotion a path-constant change.

**Demo lane recommendation:** B with the promotion-at-batch-1-close contract (one code copy, one planned migration — anti-duplicate risk mitigated); A is workable with batched route authorization. Owner rules.

## F. Round-2 keeper deltas (bus df2b24 + c22343) — adoption record (r2)

| Delta | Verdict | Note |
|---|---|---|
| Capability count restored to 12 incl. deletePage soft-delete | ACCEPT | journal-traceable undo fits a reprogrammable surface |
| RDCB gains dependsOn [WIKI-BASELINE] | ACCEPT | generator references file-surface records; no RDCB before wiki baseline |
| gate-decision = journal record + keeper executes roadmap.v1.json mutation (one-writer rule) | ACCEPT | fixes a gap in my C plan — journal is record only; keeper sole writer |
| Envelope gains actor + schemaVersion | ACCEPT | actor replaces writer-in-round-field; cleaner AI-use-native audit trail |
| Endpoint /api/mutate if owner rules P7-A | ACCEPT | consistent with /api/host/* convention; /demo/mutate under B |

All five deltas folded into sections B, C, D1, E above (marked "keeper r2, adopted").
