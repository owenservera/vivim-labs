# L02 — Complete canonical work index (living links; 2026-11-23 snapshot)

**Mission:** Native GPUI, shared PWA, source-pinned packaging, source-to-device proof. **19 open canonical tasks** at this snapshot. This is a **read-only mirror of task titles and deep links**, not an alternative backlog or permission to edit already claimed files.

**On each cycle:** reopen [ARBOR master #33](https://github.com/owenservera/ARBOR/issues/33), compare these IDs and checkbox states to live canonical source; check [TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md); reconcile collisions and pick a *currently open, actually claimable* item. Completed status is determined by #33, not this historical list. [Runbook](NEXT-CYCLE.md) · [links/tools](CONTEXT-AND-TOOLS.md) · [owner startup](CHATGPT-LAUNCH.md).

## Complete task coverage

| ID | Canonical deliverable | Source |
|---|---|---|
| **S02** | Build one immutable release from a single verified source SHA | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S03** | Restore required Rust runtime and complete native packaging | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S04** | Complete lockfiles and reproducible package manifests | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S05** | Validate Host, GPUI and PWA together in an isolated runtime | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S06** | Safely promote release without changing phone URL or pairing | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S07** | Prove PC composer changes appear on phone | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S08** | Prove phone composer changes appear on PC | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S09** | Handle concurrent edits, revisions, IME and interruptions | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S10** | Synchronize Wiki revisions and conversation state across devices | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S11** | Verify production service worker and refresh safeguards | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S12** | Resolve CI startup failures and execute real jobs | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S13** | Verify Host supervision, crash recovery and rollback | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **S14** | Capture matching two-device screenshots and Host receipts | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **U01** | Validate every native GPUI menu and window-control interaction | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **U02** | Complete Developer screenshots, diagnostics and privacy receipts | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **U03** | Verify native GPUI/mobile layout and editor parity | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **U04** | Complete keyboard, IME, multiline and focus behavior | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **C04** | Prove Rust representation and visual/interaction parity | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **C05** | Integrate approved capabilities without duplicate browser or backend | [#33](https://github.com/owenservera/ARBOR/issues/33) |

## Dependency clusters

- **Immutable release & device proof:** S02–S14; follow the actual #33 status and receipts before sequencing effects
- **Native usability:** U01–U04; follow the actual #33 status and receipts before sequencing effects
- **Studio capture to Rust/native:** C04–C05; follow the actual #33 status and receipts before sequencing effects

## Cross-lane dependencies

- L01 provides a verified provider turn before L02 may claim chat parity.
- L04 produces semantic and gesture behavior; L02 owns native/phone adapter acceptance.
- L05 owns independent evidence/CI audit, not live deployment.

## End-to-end acceptance

One immutable build and deployed SHA with rollback; same Host-backed state and Wiki revision on PC/phone in both directions; physical screenshots, input, IME and source/receipt correlation; no changed phone URL.

**Evidence ladder:** CLAIMED → CODED → PUSHED → TESTED → PACKAGE/SERVED if relevant → HOST_ACK → PROVIDER / GPUI / PHONE verified as applicable. Earlier stages do not imply later stages. No item closes from this index alone.

## Other planned work

The existing [Planning catalogue](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/MASTER-PLAN-R001.md) has 37 research/prep candidates (AW/PL/PY/GP/VF), which are **cross-references**, not 37 additional deliverables. The existing [owner-work sources](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/OWNER-WORK/ONE-UI-TRACKER-LOOPS-R003-EXECUTION-SPEC.md) describe older acceptance cycles; interpret them through the newer canonical native GPUI+PWA directive. Preserve historical work and its source proven receipts; do not blindly relaunch old queues.
