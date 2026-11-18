> **UPDATED AFTER ALL FIRST FOLLOW-UPS:** use [05 — revised launch order](05-LAUNCH-ORDER-AFTER-FOLLOWUPS.md) and [04 — updated decision](04-REFRESHED-DECISION-AND-PHASE2-SLOTS.md) instead of resending the older six-prompt bundle below. A is already FREEZE_READY; only B–F need last handoff prompts. Three independent Phase2 preparation lanes may start now but must not block G.

# Proposal 5 — Pre-G Launch Pad (for concurrent ChatGPT lanes)

**Do this BEFORE the full W1-G integration prompt.** The six workers have physically landed their initial source migrations, but some focused follow-ups remain active or only prepared changes. No shared integration branch was present when checked.

Start by reading [convergence and freeze design](01-CONVERGENCE-AND-FREEZE.md), [cross-owner blocker board](02-DEPENDENCY-AND-RELEASE-BLOCKERS.md) and [machine freeze ledger](03-READY-TO-MERGE-LEDGER.json).

## Phase PREG-1 — six existing chats; no new worker threads needed

| Existing ChatGPT lane | Read and paste this finalization prompt | Expected result |
| --- | --- | --- |
| **A — Rust/Tauri** | [A final freeze](prompts/A-FINALIZE-AND-FREEZE.md) | Native source/security preparation + PREG-A source/HEAD/test ledger |
| **B — Host** | [B final freeze](prompts/B-FINALIZE-AND-FREEZE.md) | Host source/authority and A/B launch contract + PREG-B |
| **C — Studio** | [C final freeze](prompts/C-FINALIZE-AND-FREEZE.md) | Complete Studio/harness source inventory + PREG-C |
| **D — HQ** | [D final freeze](prompts/D-FINALIZE-AND-FREEZE.md) | Internal HQ source, runtime dependencies and read port + PREG-D |
| **E — PWA/TierFlow** | [E final freeze](prompts/E-FINALIZE-AND-FREEZE.md) | Secure browser-PWA enrollment and visual boundary plan + PREG-E |
| **F — Contracts** | [F final freeze](prompts/F-FINALIZE-AND-FREEZE.md) | Source truth, risk and compatibility gates + PREG-F |

Launch these concurrently, **but only after telling each chat to finish its already-running focused follow-up**. A FREEZE_READY report means that chat must stop changing its branch. IN_PROGRESS is a legitimate answer; do not treat it as ready.

## Phase PREG-2 — one serial independent referee

[H — PRE-G Referee](prompts/H-PREG-REFEREE.md): launch in a separate ChatGPT conversation after all six handoff reports are committed. This is an audit, **not a merge**. It must produce a frozen merge-input ledger and READY_FOR_G or HOLD_FOR_WORKER_FREEZE.

## Phase PREG-3 — existing G read-only preflight

[Existing ChatGPT G — Read-only pre-integration handoff](prompts/G0-PRE-INTEGRATION-HANDOFF.md): checks six immutable heads and decides whether full W1-G may start. It must not create/merge shared integration while reading this prompt. Once READY, the user may send the separately defined **full W1-G integration prompt**.

## Critical facts

- No active worker branch or ARBOR main is modified by the docs in this pre-G branch.
- All six worker branches descend from old Proposal 5 document SHA dd245d78ff3c110ba8630cad557ea0479c51a635. Current Proposal 5 architecture branch advanced to 6ab7e6976c02cac2ee71f030deda3f6541f8102f.
- At last check integrate/proposal5-beta-r001 is absent.
- A/B native boot token security and E/B browser PWA signer are unresolved **release security/product gates**. A planned patch is not an implemented fix.
- D internal HQ service is source imported but not Host mounted. C lacks an observed canonical W01-C report at current branch. Root Cargo/Bun and no-donor tests have not run.
- Current Phase 2 Labs readiness is PREPARED_NOT_PROMOTED and does not add more runtime features to W1.

**Success of this stage:** six frozen, source-proven merge inputs and a crisp dependency/security ledger so G can merge one time without being surprised by ongoing writes.
