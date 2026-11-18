# ARBOR Acceleration R002 — W1 Lane C handoff

**Branch:** `work/accel-r002-w1-c-wiki-visual` (isolated; no merge).
**Source/main pin at branch creation:** `2a42f1af025db8aacb965326b11c73da671ffb3f`.
**Result SHA:** See branch HEAD (a commit cannot record its own SHA in its contents).
**Acceptance:** **SOURCE_IMPLEMENTED / FOCUSED_NODE_TESTS_EXECUTED / HOST_BROWSER_UNRUN / INDEPENDENT_ACCEPTANCE_PENDING**.
**Accelerator speedup:** `NOT_YET_MEASURED`.

## Source-inspected contracts

Read the parallel README and C order; canonical acceleration roadmap and `roadmap.v1.json`; PDR handoff and batch-1 exit; actual Host `server.mjs`, product journal, chat and Wiki source; `apps/mobile-relay/vec/compiler.mjs`; seed Wiki/file-surface; and read-only route/evidence tests.

- `/api/wiki`: revisioned, journal-replayed Host Wiki projection; `/api/state`: product revision/chat state.
- `/api/roadmap`: keeper-owned read-through; `/api/file-surface`: generated **`files[]`** registry, schema `vivim.file-surface/1`; `/api/journal`: data-root Host product journal records.
- `/api/mutate`: *sole* existing Host Wiki mutation route. Same-origin browser Origin and `x-vivim-control: 1`; `schemaVersion:1`, `actor:owner-browser`, `kind:wiki-edit`, nested `edit.action:set-section` and `expectedRevision`. A response is not accepted without an exact acknowledgement and revisioned readback.
- VEC `vivim.vec-compile/0.1` currently has no installed Wiki adapter; `unsupported/LEGACY_OWNER_NOT_MIGRATED` means **no executed/persisted compiler**. A04 remains a typed *read-only projection seam*, not an activated NL compiler.

## Five exclusively owned changed paths

1. `apps/vivim-host/public/pages/wiki.js`: poll/source-backed projections, individual source retries with last-good preservation, per-source unchanged-snapshot render suppression; actual `files[]` shape; seed/verified status and source metadata; bounded section diff/preview; freshness check before preview and before POST; explicit same-origin Host acknowledgement and readback; refusal, conflict and uncertain-network draft recovery.
2. `apps/vivim-host/public/pages/wiki.html`: accessible two-screen Wiki dashboard, source and revision markers, edit-review controls and static VEC read-only contract seam. External same-origin JS/CSS only, no inline script/style.
3. `apps/vivim-host/public/pages/wiki.css`: responsive compact/wide presentation, visible status/refs/focus and review comparison.
4. `apps/vivim-host/test/wiki-visual-contract.test.mjs`: isolated Node source + mock-DOM VM tests of actual client script, source projections, no-repaint, previews, successful acknowledgement/readback and 400/403/409/network refusal.
5. `docs/acceleration/w1/C.md`: this bounded execution handoff.

No edits to `server.mjs`, Host journal, VEC compiler, roadmap, tests owned by other lanes, `main`, or integration ledgers. No provider APIs, new graph, interpreter, publisher, or privileged effects.

## Actual checks / environment

| Command / observation | Actual exit | Truth |
|---|---:|---|
| `node --check apps/vivim-host/public/pages/wiki.js` | 0 | JS syntax checked (local authored source) |
| `node --test apps/vivim-host/test/wiki-visual-contract.test.mjs` | 0 | 5/5 tests passed, 0 failed, 94.95ms, Node v22.16.0 / Linux 6.18.57 x86_64 |
| `timeout 18s chromium --headless=new ... --screenshot=... http://127.0.0.1:44587/wiki` | 124 | Attempted against isolated synthetic read API; Chromium startup hung (DBus errors), **no screenshot** |
| `timeout 10s chromium --headless=new ... --dump-dom about:blank` | 124 | Independent blank-browser startup also hung; browser rendering **NOT PROVEN** |
| `node --test apps/vivim-host/test/product-routes.test.mjs` | UNRUN | Original full repository/Host imports not available in local scoped checkout; do not borrow read-only tests |
| Real Host /wiki route and interactive browser rewalk | UNRUN | No real Host UI/runtime acceptance |
| Node 24, Bun, Windows, paired Android/PWA, CI | UNRUN | Node 22 was actually installed, Bun unavailable; no live device/browser acceptance |

### Bounded UI evidence

The actual Wiki client was executed with a controlled minimal DOM and isolated synthetic API fixture **inside Node VM**, not a real browser. The five executed tests cover: Wiki revision and product state, `files[]` provenance, roadmap verification/acceptance fields; retained last-good state and unchanged-snapshot no repaint; old/new line diff and `expectedRevision`; explicit `x-vivim-control` ack followed by fresh Wiki read; and stale 409, invalid 400, forbidden 403 and uncertain network responses that **do not claim save** or discard the draft. This is source/DOM-model evidence, **not visual screenshot/device evidence**. No fabricated image or unrun exit.

## Security / dependencies / cross-lane handoff

- HTML/CSS/JS external assets remain compatible with Host strict `script-src 'self'; style-src 'self'` CSP. DOM text is inserted through `textContent`, not trusted HTML. Real publication and interpretation remain entirely outside this lane.
- Wiki edits require explicit human preview and confirm; a fresh `/api/wiki` revision is compared at both stages; Host remains the only writer; 400/403/409/network uncertainty preserve draft and show refusal. No optimistic saved label, no blind retry after ambiguous POST.
- Roadmap `A02` is **in-progress; verification UNRUN, ownerAcceptance PENDING** at the inspected pin. `A04` is **queued/blocked by A02**. Lane C does not flip gates. The next read-only ready frontier remains A00+A01+A02 until independent acceptance.
- **Lane A:** deliver independently verified durable Host journal/CAS/authorization, and ensure explicit Wiki-edit ack/revision compatibility after its changes.
- **Lane B:** run isolated real Node24 Host acceptance, route/asset and POST/readback rewalk, and inspect actual browser/Chromium availability; use the fresh Lane C branch after integration.
- **Lane D / keeper:** independently confirm source-vs-executed-vs-accepted truth and preserve UNRUN states; owner integrator may later serially integrate reviewed A/B/C/D/E on a fresh pin.
- **A04 owner:** once A02 independently accepted, connect the existing NLCL/VEC trusted adapter/Host proposal flow to this read-only projection; do not create a browser-side interpreter or publisher.
- **Remaining blocker:** authentic rendered mobile/wide-screen browser and physical device evidence, actual Host Node24 acceptance, A02 independent acceptance. `NOT_YET_MEASURED` downstream acceleration.

**Disposition:** `PREPARED_NOT_PROMOTED` (source + focused Node tests; not accepted, not merged).
