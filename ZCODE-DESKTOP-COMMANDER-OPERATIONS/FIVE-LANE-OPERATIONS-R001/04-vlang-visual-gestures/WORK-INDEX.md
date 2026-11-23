# L04 — Complete canonical work index (living links; 2026-11-23 snapshot)

**Mission:** Rust NL planning, Host-approved commands, TierFlow visual feedback, gestures. **10 open canonical tasks** at this snapshot. This is a **read-only mirror of task titles and deep links**, not an alternative backlog or permission to edit already claimed files.

**On each cycle:** reopen [ARBOR master #33](https://github.com/owenservera/ARBOR/issues/33), compare these IDs and checkbox states to live canonical source; check [TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md); reconcile collisions and pick a *currently open, actually claimable* item. Completed status is determined by #33, not this historical list. [Runbook](NEXT-CYCLE.md) · [links/tools](CONTEXT-AND-TOOLS.md) · [owner startup](CHATGPT-LAUNCH.md).

## Complete task coverage

| ID | Canonical deliverable | Source |
|---|---|---|
| **U06** | Complete deterministic natural-language-to-command planning without an LLM | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **U07** | Wire VLANG plans to Host approval, execution and receipts | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **U08** | Complete two-TAB prediction and semantic VIEW ALL | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **U09** | Complete real-time visual feedback and command-state transitions | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **U10** | Migrate remaining TierFlow/VEC behavior into shared semantic surface | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **U11** | Expose approved Desktop Commander capabilities through Host authority | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **G02** | Connect shared gesture reducer to GPUI input | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **G03** | Connect shared gesture reducer to mobile touch input | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **G04** | Render progressive long-press feedback, preview and cancel states | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **G05** | Prove physical-device gesture behavior and approval boundaries | [#33](https://github.com/owenservera/ARBOR/issues/33) |

## Dependency clusters

- **VLANG and semantic feedback:** U06–U11; follow the actual #33 status and receipts before sequencing effects
- **Shared gesture acceptance:** G02–G05; follow the actual #33 status and receipts before sequencing effects

## Cross-lane dependencies

- L03 consumes U07 Host action semantics and Work approval.
- L02 owns actual GPUI/phone rendering and gesture adapter acceptance.
- L05 independently checks exact-SHA VLANG/safety suite and evidence.

## End-to-end acceptance

Deterministic current source language parsing and safe negation/ambiguity -> explicit Host approval/receipt -> semantic visual transitions -> actual native/mobile gestures, preview, cancellation and physical test proof.

**Evidence ladder:** CLAIMED → CODED → PUSHED → TESTED → PACKAGE/SERVED if relevant → HOST_ACK → PROVIDER / GPUI / PHONE verified as applicable. Earlier stages do not imply later stages. No item closes from this index alone.

## Other planned work

The existing [Planning catalogue](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/MASTER-PLAN-R001.md) has 37 research/prep candidates (AW/PL/PY/GP/VF), which are **cross-references**, not 37 additional deliverables. The existing [owner-work sources](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/OWNER-WORK/ONE-UI-TRACKER-LOOPS-R003-EXECUTION-SPEC.md) describe older acceptance cycles; interpret them through the newer canonical native GPUI+PWA directive. Preserve historical work and its source proven receipts; do not blindly relaunch old queues.
