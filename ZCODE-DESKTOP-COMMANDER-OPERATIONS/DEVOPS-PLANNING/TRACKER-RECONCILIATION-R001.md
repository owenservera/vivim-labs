# Reconciliation of Planning proposals with canonical task history

## Source-of-truth limitation

The remote `ZCODE-DESKTOP-COMMANDER-OPERATIONS/` directory in Labs GitHub main does **not** include the historical local `OWNER-WORK/` directory. Older role files mention `OWNER-WORK/NOW.md`, `REQUESTS.jsonl`, `UPDATES.jsonl`, but no current remote contents or task history were accessible. `TASK-CLAIMS.md` **is** accessible and is exclusively a claim/handoff overlay. No new canonical OW ID may be inferred from proposal numbering.

## This work's disposition

- [MASTER-PLAN-R001](MASTER-PLAN-R001.md) is a **Planning-only proposal catalogue**: 37 scoped/researched future preparation packets, never 37 active claims or new OW IDs.
- It references existing Dev ownership, source modules, and task definitions but is not an active task/router service or alternate durable owner ledger.
- Planning may check out one coherent packet by CAS update of `TASK-CLAIMS.md`, and then update this plan with evidence if that packet's preparation really progresses.
- The current `PLAN-MASTER-20261121-R001` claim identifies this documentation pass, not permission to start GPUI implementation or alter ARBOR application source.

## Once canonical history is accessible

1. Confirm actual owning writer and path; obtain only authorized data. Do not publish private/ignored local work records wholesale.
2. Read original request IDs and append-only update event IDs/timestamps at the original source. Compare to ARBOR first-customer product tracker and claims, without overwriting or reconciling from guesswork.
3. Link each proposal key to an existing OW ID only on exact semantic scope, owner and source evidence. No match => propose genuinely new canonical task to existing writer, no automatic minted OW number.
4. Preserve every original event and all concurrency state. Append new cross-references through existing canonical writer, then verify historical event count, identity uniqueness and untouched records.
5. Retain proposal links only as supporting Planning analysis; do not maintain two competing mutable status ledgers.
6. If still inaccessible, report **canonical registry unverified** and leave these as proposals rather than pretending they are assigned or complete.

## Ownership and consumer rules

- ChatGPT Dev owns active PWA typing and ARBOR integration. Never put those tasks in Planning's Next Ten. They remain external dependencies.
- Oracle supplies strategic shaping but does not assign or steal work. Planning records ACCEPTED/AMENDED/DEFERRED with source when reviewed.
- The sender owns a handoff until receiver ACK appears in TASK-CLAIMS. GitHub file presence is neither a notification nor an active worker.
- A paused GPUI task may remain a proposal but requires explicit owner resumption before checkout.
- Direct GitHub main, Rust-first, no default Desktop Commander/ZCode source coding. Local runtime verification remains a separately authorized activity.
