# Shared task check-in / check-out claims (ownership overlay, not a backlog)

**Purpose:** one GitHub-visible owner-claim surface for ChatGPT Dev, DevOps/Planning, and Oracle. The canonical task descriptions, ordering, and statuses remain in OWNER-WORK when accessible, otherwise in verified ARBOR/Labs trackers. This file is **not** a replacement task ledger. A task absent here is **UNCLAIMED/UNKNOWN**, not automatically available to overwrite.

## Required protocol

1. **READ:** Before listing or starting tasks, fetch fresh `main` for this file, actual task definition and relevant source. Separate task **delivery owner** from dependency **observer**. Every task must have one active executor, or be explicitly UNCLAIMED/BLOCKED. An active Dev task must NEVER be counted as a Planning next-to-do.
2. **CHECK OUT:** Write one claim row below by updating this file through the GitHub contents API with the latest content SHA (optimistic concurrency). Include task ID, executor, scope/files, source head, claim timestamp UTC, claim state `ACTIVE`, and next acceptance. Re-fetch the committed row before editing owned paths. If a competing claim/write or unclear task owner exists, stop and reconcile; don't auto-steal.
3. **WORK / CHECK IN:** Update the same row to `PREPARED`, `READY_FOR_HANDOFF`, `CODED`, `TESTED`, `BLOCKED`, or `DONE` only with evidence; include GitHub commit, test provenance and pending acceptance. On `BLOCKED` retain claim explicitly or release it explicitly. An agent can't self-certify phone verification from a source commit.
4. **HANDOFF:** Sender records `OFFERED_TO:<role>` (still sender-owned). Receiver must fetch latest state, explicitly `ACCEPTED_BY:<role>` and atomically change executor. Until acceptance, sender owns the handoff; no implicit cross-chat delivery. Owner may override with an explicit recorded decision.
5. **RELEASE:** Once a task has closed/handed off, set `RELEASED` with evidence and zero active executor. Do not delete history from the row: append a short latest-event pointer/commit link. Reclaimed/reopened tasks get a new claim event in the same task row, not a second task.
6. **STALE:** Timestamp alone never transfers ownership. A stale or vanished chat is `NEEDS_RECONCILIATION`; Oracle may flag it but not seize or reassign it. GitHub file atomic update prevents lost claim writes, not simultaneous activity on other source files.

## Claim register

| Task / authoritative reference | Active executor | Scope / source pin | Claim state | Latest check-in / evidence | Pending / handoff |
| --- | --- | --- | --- | --- | --- |
| _No claims recorded yet. Historical assignment in prose is not an active checkout._ | — | — | UNCLAIMED/UNKNOWN | — | Reconcile existing Dev and Planning responsibilities before new claims |

## SITREP selection rule

**Last five** = latest five GitHub-backed task check-ins, with timestamps and evidence; if fewer than five exist, show only real entries and label the remainder unavailable. **Next ten** = next ten actionable tasks **owned/claimable by this particular agent**, ordered from authoritative tracker; exclude Dev-owned active work, and place blocked external dependencies separately in a compact `Waiting on Dev / Oracle / owner` section. Each row: ID, delivery owner, claim state, brief completed proof, next action, blocker, last check-in UTC. Never turn OW chronology, planning wishlists or code evidence into pretend claim history.

## Concurrency and recovery

- One task executor, one edit scope at a time. Parallel agents may work separate tasks and scopes; overlapping source files require a recorded owner-to-owner agreement before coding.
- Use GitHub contents SHA compare-and-swap for this file. On 409/422: refresh, reconcile, retry; never force update. Direct main, no branches or PRs unless owner authorizes.
- Exact agent/chat identity might not be available across sessions; record role plus an explicit short session token supplied by the agent, never invent stable cross-chat routing.
- Do not automatically launch agents, spend models, or run local desktop software. GitHub is the shared check-in surface, not a notification or active task bus.
- No historical claim events are backfilled without evidence. Oracle audits collisions; Planning sequences own prep tasks; Dev checks out only its product execution tasks.
