# Oracle -> DevOps/Planning: strategic shaping signal

**Issued:** 2026-11-21 · **Origin:** Oracle North Star · **State:** PROPOSED / RECIPIENT_ACK_UNOBSERVED
**Purpose:** shape Planning's existing pipeline toward **owner speaks/types in the PWA -> software builds -> phone verifies -> fewer AI calls**. This memo is **not** a task registry or a source-code handoff. Planning owns task decomposition and schedule.

## The one highest-leverage direction
**Close the first missing durable edge between the PWA and the development workflow.** A Wiki lookup while typing is not a software-development request. The owner must be able to *explicitly* ask the PWA to change/build the product and get one authenticated, idempotent request identity that Planning and Dev can actually retrieve without the owner copying chat text between sessions.

**Observed blockers to resolve in Planning's decomposition:**
- **Evidence drift / reuse-first (2026-11-21 audit):** ARBOR first-customer tracker describes AV-01/02 progressive typing as design-only, while the actual agent-composer adapter now contains paced frame edits and first-contact TypeScript passes the interval, with fresh tests. Reconcile what is SOURCE_IMPLEMENTED vs TESTED vs VISIBLE before duplicating implementation work.
- **CI truth (2026-11-21 audit):** Recent ARBOR Actions push runs returned startup_failure with **zero jobs** (https://github.com/owenservera/ARBOR/actions/runs/44895210613). Root cause not established. Require an actual run/test receipt for release gates and shape minimal CI recovery in existing Dev workflow; a YAML workflow is not PASS.
- Remote canonical owner registry is not yet evidenced: the Labs operations-tree `OWNER-WORK/` files are NOT_FOUND on GitHub. Identify the actual canonical source and its ownership; migrate/reconcile or adopt an **existing** accessible record, without creating another shadow backlog or losing local-only history.
- Explicit PWA build-intent admission is not the same as the already implemented ambient Wiki read or as the inverse Host-authorized agent draft delivery. Reuse the existing Rust intent/LiveSystemState/Host Control contracts. Avoid creating work from partial drafts, IME composition or voice transcripts without deliberate submission.
- ChatGPT sessions cannot be assumed to wake, share context, or execute from a PWA event. Identify a supported authorized coding-agent dispatch/acknowledgment path before promising A2 autonomy. Until then, label the bridge MANUAL and provide truthful PWA status.
- Source coded, tests executed, pushed SHA, locally deployed artifact and *actual phone rendering* are different gates. Close each on the same stable URL, with rollback and source/Host receipts; preserve current Dev-owned composer/Wiki files.

**Outcome acceptance for the first Planning handoff:**
1. Owner explicitly submits one build request (typed or approved voice transcript) in PWA.
2. Rust/Host authorizes and deduplicates it into one durable, private, revision/correlation-fenced request record using the canonical work source.
3. The owner sees acknowledgment/status on the same PWA without duplicating a ChatGPT request; the relevant Planning session can obtain the identical request ID and current evidence.
4. Cancel, conflicting human edit, denied scope, stale revision, offline retry and duplicate submission are handled safely.
5. Show exactly which edge is still manual (if coding agent dispatch is not yet supported). No false completion.

## Subsequent direction, not a fresh task list
Once admission is real, shape existing work around **acknowledged Planning -> Dev bridge**, then **verified build/release -> PWA readback**, then **behavior-to-determinism with measured call savings**. Reuse Dev's working source; do not preempt or duplicate its current user/agent typing goals. Optimize the *entire* owner loop, not isolated agent counts or pages of instructions.

## What Oracle requests from Planning
At its next normal planning pass, **map this signal to existing owner IDs/bundles where possible**; propose only genuinely missing capability work. Accept, amend or reject each dependency with evidence and a pointer to the canonical Plan artifact. Return one concise acknowledgment:

```text
Oracle signal: PWA_BUILD_INGRESS
Disposition: ACCEPTED | AMENDED | DEFERRED | ALREADY_COVERED | BLOCKED
Canonical work IDs / bundle pointers: ...
First unproven edge: ...
Evidence/source SHA: ...
Next accountable role: ...
```

Oracle will review that response for alignment, **not rewrite Planning's task list**. Do not require the owner to arbitrate routine cross-agent handoffs.

## Oracle new PWA product-sequencing signal — WIKI_COMMAND_SYMBIOSIS (2026-11-21)
**Status: PROPOSED / RECIPIENT_ACK_UNOBSERVED.** Owner asks what to integrate *after* refining the two read-only AutoWiki typing milestones, to make Wiki, NL command interpretation and Host effects symbiotic. Full existing-source review and five options: [WIKI-COMMAND-SYMBIOSIS-R001.md](WIKI-COMMAND-SYMBIOSIS-R001.md). This is an additional outcome-shaping recommendation, not a competing task queue or a supersession of the separate PWA_BUILD_INGRESS meta-goal.

**Recommended narrow product result:** select one source-backed Wiki section -> choose a catalog-backed action -> resolve exact pinned target and missing args through incumbent semantic composer -> review existing Host plan -> explicitly approve one R1 edit -> see committed Wiki revision + receipt on the SAME PWA. Ordinary Wiki typing remains read-only. No mode auto-switch that executes. First close the existing human/agent AutoWiki surface without taking over Dev scope.

**Planning disposition request:** cross-map to **PV-04/PV-05/PV-06/PV-10** (existing PWA-visible outcome tracker); B06/U7, AW-012, R003 Rust lens, existing NLCL/Host/Wiki control are candidate reused dependencies. PV-08 visual programming and PV-12 deterministic recipes follow; PV-14 PWA build-intent remains a distinct explicit action. Investigate exact Wiki/HQKG identity, current grant, first-contact mode/Send fencing, selected context -> typed Host plan, and source-fenced readback. No new parser/catalog/graph/journal/permission engine. Acknowledge with accepted/amended/deferred and first evidence gap.
