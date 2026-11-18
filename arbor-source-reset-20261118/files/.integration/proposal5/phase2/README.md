# Proposal 5 — Phase 2 Labs Intake: Parallel ChatGPT Launch Pad

**Status:** planned and sourced; Phase 2 worker prompts prepared, not executed.

Read [the detailed assessment](../10-PHASE2-LABS-INTAKE.md) and [decision matrix](../11-PHASE2-DECISION-MATRIX.md).

## Run now alongside A–F, in three separate ChatGPT conversations

| ChatGPT workstream | Scope | Prompt |
| --- | --- | --- |
| **P2-E1 — Easy Migration** | Observatory, verified hook crosswalk, pure R005 UI/source semantic fixtures, small Chronos read-only fixtures, HQ wiki quality tests | [Copy prompt](prompts/P2-E1-EASY-MIGRATION.md) |
| **P2-C1 — Core Dev/Migration** | ChatGPT T=0 knowledge intake, native Node 0 source-truth, Chronos Rust, approved voice-to-task boundary | [Copy prompt](prompts/P2-C1-CORE-MIGRATION.md) |
| **P2-H1 — High-Value Pending** | R007, advanced prediction, AutoWiki, Rock Core, app builder, automation, personal DCB and extension research | [Copy prompt](prompts/P2-H1-HIGH-VALUE.md) |

After the three parallel chats commit their **isolated** branches, use [P2-G — Integration Handoff](prompts/P2-G-INTEGRATION-HANDOFF.md) as a **fourth, serial ChatGPT conversation**. It may recommend promotion but must not modify current Wave 1 branches or merge into the active integration branch without the existing master G's approval.

**Why staging matters:** Wave 1 A–F are already importing the same product owners. Phase 2 can inspect and prepare everything safely now; source promotion is serialized at W1-G/W2-G to avoid duplicate code or conflicting writes.

**No outside VIVIM wiring:** future selected Labs code physically lands under the existing ARBOR owner, not an external Labs runtime dependency.

**Beta rule:** extra Labs features are optional; migrate working/tested code cheaply, defer research-only ambitions. Source tests are not automatically device/runtime acceptance.
