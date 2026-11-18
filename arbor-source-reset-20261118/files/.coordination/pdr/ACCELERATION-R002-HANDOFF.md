# ARBOR orchestration desk — Acceleration R002 handoff (2026-11-14)

**OWNER DIRECTIVE: ROADMAP UPDATED, WORK NOT YET EXECUTED.** Do not continue the old stopped roadmap run by default. Re-pin current `main`, consume inbox and follow the existing owner-batch authorization protocol before issuing effectful work. This handoff **points to** the canonical roadmap, it is not an additional queue.

## Read in order

1. `docs/product/roadmap/roadmap.v1.json` → `acceleration`, `buildQueue` (A00–A08), new L2 acceptance/write-scope fields and `archivedBuildBatches`.
2. `docs/product/roadmap/ACCELERATION-R002.md` → acceleration flywheel, lanes, safety and measures.
3. `docs/product/roadmap/ROADMAP.md` → human projected inventory, 3 L0 streams, active L1-DEMO-LOOP and existing 89 L2 entries.
4. `scripts/roadmap/acceleration-queue.mjs --check` and `--ready` → **read-only** deterministic checks. `node --test scripts/roadmap/acceleration-queue.test.mjs` for Node process tests when a local checkout exists.
5. Existing contracts `.coordination/pdr/PDR-batch1-exit.md`, `apps/vivim-host/control/action-catalog.v1.json`, `apps/vivim-host/server.mjs` and `scripts/batch-evidence/`. Never copy these into a parallel engine.

## The immediate first three concurrent lanes

**Lane B starts A00** (sole roadmap writer): fix the two flawed Wiki verification assumptions, keep separate API/source vs browser-render checks, preserve historical receipts, reconcile the *claims* in the contradictory release ledgers against actual run evidence. Do not falsely mark release accepted or rewrite historical records.

**Lane C starts A01:** build one command that launches an isolated real Host and checks /chat, /wiki, /demo and the typed API routes; record exact Node/Bun/OS versions, command, exit status, stdout/stderr and source SHA. No actual signed-in provider or device effect.

**Lane A starts A02:** derive chat and Wiki projections from the canonical Host journal/state with durable revisions; make approved writes survive reload and restart; prove both screens agree without a new store, graph, interpreter or effect owner.

Next ready work is unlocked only by accepted dependencies, not by unreviewed source landing. A03 after accepted A00; A04 after A02; A06 after A01+A02; A05 after A01+A02+A03; A07 after A05+A06; A08 after A02+A04.

## What to report to owner and oracle

For each item: exact commit, changed paths and owning lane; actual test exits vs UNRUN; independent receipt; live product behavior demonstrated; residual blockers; next ready frontier; and time/cost saved on the *next* task or NOT_YET_MEASURED. All changes must use Host effect permissions and fresh-head safeguards. No provider APIs, no live provider claim from mock source, no 100-agent fleet and no APK milestone.

The default target is not a new design doc: **make one typed instruction change an accurate, durable second-screen intelligent Wiki and turn a subsequent failure into a verified repair.**

## Execution status refresh — 2026-11-14

A00/A01/A02 are now `in-progress` in canonical `roadmap.v1.json` with source landed. Verification and owner acceptance remain UNRUN/PENDING; A03–A08 remain blocked/queued. The immediate local commands are in `docs/product/roadmap/ACCELERATION-R002.md` §8. No stopped workflow was automatically resumed, no other branch was merged, and no Host/private source effects were performed by this GitHub update.
