# L01 — Complete canonical work index (living links; 2026-11-23 snapshot)

**Mission:** Signed-in provider session correctness and Studio capture/interaction proofs. **19 open canonical tasks** at this snapshot. This is a **read-only mirror of task titles and deep links**, not an alternative backlog or permission to edit already claimed files.

**On each cycle:** reopen [ARBOR master #33](https://github.com/owenservera/ARBOR/issues/33), compare these IDs and checkbox states to live canonical source; check [TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md); reconcile collisions and pick a *currently open, actually claimable* item. Completed status is determined by #33, not this historical list. [Runbook](NEXT-CYCLE.md) · [links/tools](CONTEXT-AND-TOOLS.md) · [owner startup](CHATGPT-LAUNCH.md).

## Complete task coverage

| ID | Canonical deliverable | Source |
|---|---|---|
| **A01** | Reconcile the historical uncertain ChatGPT turn without resending | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A02** | Resolve the existing busy Studio session safely | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A03** | Deploy the isolated ChatGPT channel into the live runtime | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A04** | Verify a new ChatGPT prompt reaches the correct composer | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A05** | Verify exactly one send and the correct conversation/thread | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A06** | Read back a genuine completed ChatGPT reply | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A07** | Project the verified reply through the existing Host relay | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A08** | Render and acknowledge the reply on the physical phone | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A09** | Deploy and validate the mobile AI chooser and conversation history | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A10** | Connect native GPUI composer to actual AI provider conversations | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A11** | Display messages, replies, loading states, errors and thread switching in GPUI | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A12** | Prove a second prompt continues the same ChatGPT conversation | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A13** | Complete real Gemini and Qwen message delivery | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A14** | Resolve Claude, Z.ai and DeepSeek readiness blockers | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A15** | Verify each provider's actual supported capabilities | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **A16** | Validate cancellation, offline, restart and no-duplicate-send behavior | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **C01** | Complete authenticated ChatGPT.com interactive-surface capture | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **C02** | Finish composer/message/streaming parity matrix | [#33](https://github.com/owenservera/ARBOR/issues/33) |
| **C03** | Validate dialogs, attachments, menus and interaction feedback | [#33](https://github.com/owenservera/ARBOR/issues/33) |

## Dependency clusters

- **Provider delivery:** A01–A12; follow the actual #33 status and receipts before sequencing effects
- **Additional providers:** A13–A16; follow the actual #33 status and receipts before sequencing effects
- **Browser capture:** C01–C03; follow the actual #33 status and receipts before sequencing effects

## Cross-lane dependencies

- L02 owns real native/phone rendering and delivery parity; provider effects require fresh owner approval.
- L05 independently verifies live receipt and source-pinned Studio tests.

## End-to-end acceptance

New owner-submitted turn -> correct signed-in composer/thread -> exactly one confirmed send -> settled reply -> Host receipt -> GPUI and physical-phone rendered ACK. Browser-state inspection alone is not sufficient.

**Evidence ladder:** CLAIMED → CODED → PUSHED → TESTED → PACKAGE/SERVED if relevant → HOST_ACK → PROVIDER / GPUI / PHONE verified as applicable. Earlier stages do not imply later stages. No item closes from this index alone.

## Other planned work

The existing [Planning catalogue](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/MASTER-PLAN-R001.md) has 37 research/prep candidates (AW/PL/PY/GP/VF), which are **cross-references**, not 37 additional deliverables. The existing [owner-work sources](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/OWNER-WORK/ONE-UI-TRACKER-LOOPS-R003-EXECUTION-SPEC.md) describe older acceptance cycles; interpret them through the newer canonical native GPUI+PWA directive. Preserve historical work and its source proven receipts; do not blindly relaunch old queues.
