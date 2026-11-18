# ChatGPT strategic loop — live state (updated 2026-11-18 ~11:40Z)

**Transport (PROVEN)**: `bun C:\0-BlackBoxProject-0\vivim-web-harness\studio\bin\studio.mjs drive chatgpt --reuse-tab --prompt "..." --timeout 300 --json` (owner's ChatGPT tab, no new tabs, reply read via `studio export transcript <session> --confirm`). `chat chatgpt --target <tab>` is blocked DRAFT_PRESENT on a tab that already has a thread. Owner rule: commit+push ~1 min before each ChatGPT turn.

**Authority model (owner 2026-11-18 ~11:40Z)**: ChatGPT = PRODUCT QUEUE GENERATOR + OVERALL ORCHESTRATOR of all workstreams. Execution via this session/thread (engine tools: CreateWorkflow, AmendWorkflow, TaskStop, ResumeWorkflowRun, CronUpdate, sync bus). Owner = final authority; keeper = sole roadmap.v1.json writer; escalation only on integrity violation or both-provider outage.

## Steer log
- **#1** (02:55Z, 0dab26a): fleet briefing → P0 set (A05/A06 reconcile, A07 receipt, A03 dep), 4-lane topology, verification tiers, model split, channel roles.
- **#2** (08:17Z): A05 4-slices (Select/Execute/Verify/Proof), GOAL-3 = vertical journey lane, per-sweep flip-debt, step-5 primary/mistral fallback, GLM out, ernie probe-only.
- **#3** (09:22Z, a59f587): GOAL-3 round-1 = bounded READ-ONLY AUDIT (trace + first broken edge), keeper 2-sweep KEEPER_RECONCILIATION_STALLED escalation, A07 receipt NOT_RUN ≠ acceptance, A05.1 source-pinned, checkpoint order journey→A05.1→keeper→models.
- **#4** (10:57Z, 2ed478f): PRODUCT-FIRST OVERRIDE (owner challenge upheld). 90% product / 10% verification; scorecard = product behavior. North star: user changes their own Wiki by typing naturally — PWA shows meaning, previews change, requests approval, executes through Host, shows update + evidence. Four product deliverables: VLANG/VEC NL wiki-edit, Host authorized mutation, PWA Chat command surface, Wiki revision projection. Anti-stall: 2 rounds without product diff → redirect to next real-PWA failure. Infra frozen. WORKFLOWS UPDATED: cron f2dd9a68 prompt rewritten (product-first lanes: GOAL-3 / item=A05 / GOAL-1 / GOAL-2), arbor-life.dwf.ts frontmatter + mandate + USER persona carry the north star.

## Lane state 2026-11-18 ~11:35Z
- dwfrun-9da1a9a3 GOAL-2 lane: round 1 VERIFIED (gate 0, retest 4/4, rewalk 9/9) → keeper handoff + claim release 11:34Z; continuing to next round (2.6M tokens).
- dwfrun-e9da48fe A05 queue lane ("status flipped"): running, 3.5M tokens — the keeper flipped A05 feedable.
- GOAL-3 / GOAL-1 / goals lanes: between launches (cron relaunches per product-first spread).

## RULING #5 — ChatGPT is ORCHESTRATOR (2026-11-18T11:44Z, reviewed 7802dff)
Transcript: `.coordination/pdr/chatgpt-guidance-20261118-orchestrator.md`. Authority: product queue + orchestration; execution via manager session (engine tools); owner final; keeper sole roadmap writer.

**Product queue (48h)**: P0.1 VLANG/VEC semantic compiler (GOAL-3) · P0.2 Host authorized wiki mutation (A05 lane) · P0.3 PWA Chat command surface (GOAL-1) · P0.4 Wiki revision projection (GOAL-2) · P1.1 full vertical replay · P1.2 rename intent · P1.3 phone experience. Acceptance = real typed operation through semantics→Host→PWA with fresh-user replay; no assertion, no simulated effects, no UNRUN-as-pass.

**Executed 11:44-12:05Z**:
- cron f2dd9a68 → exact 4-slot model (GOAL-3/A05/GOAL-1/GOAL-2); occupancy requires a CLAIMED run; no fifth lane, no arg-less filler, no duplicate scopes; per-sweep state feed; mid-cycle ruling only on named triggers.
- GOAL-1 slot launched: **dwfrun-907a140d** (args goal:GOAL-1 — ruling's JSON said item:'GOAL-3'; def contract is `goal` for goal ids, `item` for queue ids; correction recorded).
- LET FINISH: dwfrun-153c8f52 (GOAL-3/VLANG+VEC, claimed, journey running) · dwfrun-e9da48fe (A05/Host, 4.2M tokens, refocus to mutation) · dwfrun-9da1a9a3 (GOAL-2/Wiki, round-1 verified gate 0/4-4/9-9, claim released to keeper).
- Stray: dwfrun-1d3667c6 'lane 2 Host' (arg-less, feed picked nothing → discovery, holds no claim) — stop queued for next sweep.
- Workflow states: roadmap PAUSED · demo ON DEMAND · build RESERVE · batch-engine VERIFY-ONLY ON DEMAND · bridge PAUSED · batch1 NEVER.

**State feed format (per sweep, bus topic orchestrator-feed)**: at_utc / head SHA / roadmap SHA / 4 slots {run, round, status, product_delta, first_broken_edge} / acceptance {product_gate, fresh_user_replay, receipt_refs, accepted_journeys} / keeper dispositions {A05, A07, flip_debt} / models {primary step-5, fallback mistral, unavailable GLM, probe-only ernie} / next_actions / ruling_required.

**Next checkpoint to ChatGPT (owner rule: commit+push ~1 min first)**: concrete semantic operation (GOAL-3), real Host effect path (A05), usable Chat preview (GOAL-1), revision-aware Wiki projection (GOAL-2) — or the exact first broken edge of each.
