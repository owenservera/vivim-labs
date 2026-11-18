# ARBOR 3-PAGE BASELINE — PDR (batch-1 exit record)

Owner-gated decision record, written by oracle at batch-1 exit (2026-11-14). Supersedes nothing; extends the round-robin ledger `.zcode/workflows/arbor-3page-refinement.v1.md` (which holds the full negotiation history).

## 1. Product frame

Three pages on the ONE production Host (`apps/vivim-host`), served at real product routes (P7-A ruling, owner 2026-11-14). AI-use-native from day 1: the same endpoints serve the human browser and the machine; machine-readable state is primary; the demo workstream drives all three pages through the same endpoints with no screenshots for routine work (snapshots only for double-checking complex upgrades).

- **Page 1 — chatbox (`/chat`)**: NLCL-first. Every message → intent IR → dispatcher → deterministic state mutation. Day-1 capability set = 12 (sendMessage, readState, executeIntent, readJournal, getCapabilities + createPage, editSection, movePage, linkPages, readWikiSection, listPages, deletePage[soft-delete/archive]). Interpretation runs in the EXISTING Host control runtime (`apps/vivim-host/control/*`, action-catalog) — never a demo-side interpreter.
- **Page 2 — realtime wiki (`/wiki`)**: renders roadmap.v1.json + journal live (source of truth, render-only; one-writer rule: keeper thread is sole writer of roadmap.v1.json) AND hosts the vlang file-surface special section. Two-stage surface layer: Stage A (batch 1) = deterministic parser over a seeded corpus → `file-surface.v1.json` (schema `vivim.file-surface/1`); Stage B (later batches) = HQ indexer gains the shared comment grammar. Honest label: format + static registry until L1-VLANG-VEC.
- **Page 3 — dev page (`/demo`)**: existing cockpit + build-loop panels; machine artifacts live here (journal.jsonl, fixtures, poll-round.mjs, receipts).

## 2. Closed-loop contracts (converged, oracle-owned)

1. roadmap.v1.json (ordered batch, itemIds) → demo builds (lane-autonomous workflow) → build receipts → **keeper independent verify** (mode=verify, second implementation) → implemented-exclusion evidence → queue flip recorded→executed (keeper sole writer) → demo re-walk over product routes → next batch.
2. Owner gates: per-batch exit only (self-iterating canon). Deterministic NLCL-assertable checks run owner-free with receipts.
3. Machine drive: ONE typed write endpoint (`/api/mutate`, envelope `{kind, actor, schemaVersion, ...}`; kinds: chat-send | wiki-edit | gate-decision). Reads via ASSETS/API routes. Loop-plane writes (queue, acceptance records) stay file contracts owned by loop threads.
4. Journal: append-only JSONL (`demo/journal.jsonl`), entry schema `{round, approveAll, verdicts[], receivedAt}` — gate entries `{round:"owner-gate-<batch>", itemId, decision}` recorded→executed|failed by keeper.
5. RDCB v1: format + generator only; bundles reference file-surface records + NLCL action ids; digest stable across runs; no execution claims.
6. Transport/coordination: oracle = coordinator + design authority; lanes lane-autonomous post-ack; bus + SendToSession per comms discipline; ChatGPT = strategic input via oracle entry point (transport pending proof).

## 3. Batch-1 evidence (owner-approved 2026-11-14, bus 84ef66)

dwfrun-e163906c (demo, arbor-batch1.dwf.ts) completed all 7 stages; exit-gate re-walk: /chat, /wiki, /api/state, /api/wiki, /api/roadmap, /api/journal, /api/file-surface = 200, POST /api/mutate = 200, POST /demo/feedback alias = 200. Receipts: `.coordination/artifacts/demo/receipt-r2-{journal-migration,product-routes,nlcl-binding}.json`. D1 binding proven live (chat-send → `vivim.control-language/1` intent IR). RDCB v1: 3 bundles in `data/demo-baseline/rdcb/`. Keeper verify pass triggered (bus 107b81). Lane-split adopted: demo builds, keeper verifies.

## 4. Effort baseline (self-reducing principle, owner directive)

Batch-1 cost the owner: 1 interactive decision (the exit gate). Batch-2 target: 1 auto-approved gate, oracle interventions ≤ batch-1. Measured each batch; a batch that does not reduce effort is wrong by design.

## 5. Open items at write time

- Host incident #2 (post-settle death, 3/3 probes 000) — demo owns boot+diagnose (FIRST-RUN doctrine).
- Queue flip recorded→executed by keeper under 84ef66; RDCB-BASELINE itemization per demo exit report (3 bundles, receipts on disk).
- Keeper independent verify receipts → implemented-exclusion evidence.
- Model flap: cron wakes inherit workspace default (platform limit); owner pin = durable fix.
- ChatGPT transport: GitHub bridge kit proven but not stood up for ARBOR; queue file `.coordination/pdr/chatgpt-questions.md` live.