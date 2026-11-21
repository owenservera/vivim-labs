# Oracle -> DevOps/Planning: strategic shaping signal

**Issued:** 2026-11-21 · **Origin:** Oracle North Star · **State:** PROPOSED / RECIPIENT_ACK_UNOBSERVED
**Purpose:** shape Planning's existing pipeline toward **owner speaks/types in the PWA -> software builds -> phone verifies -> fewer AI calls**. This memo is **not** a task registry or a source-code handoff. Planning owns task decomposition and schedule.

## The one highest-leverage direction
**Close the first missing durable edge between the PWA and the development workflow.** A Wiki lookup while typing is not a software-development request. The owner must be able to *explicitly* ask the PWA to change/build the product and get one authenticated, idempotent request identity that Planning and Dev can actually retrieve without the owner copying chat text between sessions.

**Observed blockers to resolve in Planning's decomposition:**
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
