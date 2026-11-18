# AUTOPSY — why hours of lanes produced no codebase change (2026-11-18 ~13:10Z)

Owner: "I still see no codebase changes after hours." Verified by git: since 0dab26a (session start) the only product deltas are the GOAL-1 wiki regression pair + 2 one-line edits (committed pre-steer by lane work) + a05-cycle.mjs (a driver script, infrastructure) + tests. Zero changes to VLANG/VEC semantics, Host mutation behavior, chat surface, or wiki projection. The A05 lane spent **4.2M tokens over 4h** and produced `Changed: (none)`-class outcomes plus test-suite maintenance on its own new driver.

## Root causes (each with evidence)

**RC-1 — Feature/defect inversion (primary).** The LIFE round contract is USE → OBSERVE → FIX, and FIX only fires when OBSERVE finds FAILED journey steps. Journeys now mostly pass (GOAL-1 fixed the wiki nav), so `OBSERVE skipped: journey clean` → FIX has nothing to fix → rounds close with receipts and no diff (GOAL-2 handoff 2026-11-18T11:34Z: `Changed: (none)`). The four product deliverables (semantic compiler, Host mutation, chat preview, wiki revision display) are FEATURES — a passing journey never mandates building them. The loop can only react to defects; features need a build mandate.

**RC-2 — Slot drift; args are advisory.** dwfrun-907a140d launched with `goal:GOAL-1`, but its Feed phase logged `picked A06 (build) | settled=10` — the feeder ignored the goal restriction and consumed a queue item; the lane then spent its round on A06 infrastructure (fixer executing 39m30s on one step, 1.4M-token test writer maintaining a05-cycle tests). Slot assignment is not enforced by the def.

**RC-3 — No browser inside subagents; "fresh-user replay" has never been real UI.** Every journey step 1 logs: `LocalToolSearch select:agent.browsers.list,agent.tabs.list -> No matching deferred tools found; anti-hang rule -> programmatic curl mode`. All "user journeys" are curl probes (`mode: programmatic-partial` recorded honestly in receipts). The evidence standard (fresh-user browser replay) is structurally unreachable inside workflow subagents today.

**RC-4 — Environmental gate noise.** `scripts/acceleration/live-host-check.mjs:147` asserts Node 24; the machine runs Node v26.3.0 → `Observed v26.3.0; Node24 required` → exit 1 in every receipt's live check. Verification rounds repeatedly burn on this instead of product. (Reproduce: `bun scripts/acceleration/live-host-check.mjs`.)

**RC-5 — Token burn in infrastructure.** A05 lane: fixer trio (318k/994k/202k) + test writer (1.4M) + re-tester (358k) + verifier (632k) — the largest single consumer is a test writer maintaining tests for a driver script the same run created. Self-referential infra work.

**RC-6 — Raw product material exists and nobody builds on it.** The stopped GOAL-1/A06 journey verified real behavior: chat send POST 200; NLCL `status` → `vivim.status.show` (`feedback.status: ok`, canonical `/show`); `/api/wiki` serves pages/sections (About/HowTo); `/api/journal` records entries. The product's edges work at the API level; the missing increments are: typed semantic intent → plan preview (VEC), approved mutation (Host), command surface with preview (PWA), revision-driven display (Wiki).

## What must change (candidate, for ChatGPT's ruling)

1. Round contract must mandate a product increment per round: each lane carries a concrete build backlog slice (file-pinned), not a defect-reaction mandate. A round that ends without a diff to named product paths is a FAILED round, not a receipt.
2. Slot enforcement: the def's feeder must refuse to claim anything else when `goal`/`item` is bound; drift = stop.
3. Evidence standard: either provision the browser backend into subagents (bridge, one-time) or define the honest programmatic contract (curl parity + replay) and label browser evidence UNRUN until real UI is reachable.
4. Kill RC-4 immediately (version assert → allow Node ≥24, or pin the check to the Host's own runtime).