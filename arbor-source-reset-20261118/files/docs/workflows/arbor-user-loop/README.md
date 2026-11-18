# ARBOR — User-Driven Development Loop

## ERA 1 — The simulated user

**The user is an autonomous subagent inside the existing `arbor-life` workflow.**

The subagent acts like someone using VIVIM for the first time. It receives a plain-English goal, opens the **real PWA**, and tries to achieve that goal through the interface.

It is **not** the human project owner, a developer, or a test script that knows the expected answer. It must not read source code or implementation notes before its user journey.

It should be free to explore, misunderstand, retry, and report confusing behavior. That is useful evidence.

## ERA 2 — The loop

1. **Choose a goal.** A thin feeder selects the next eligible item from the existing roadmap. Reuse what is already complete; do not restart or duplicate work.
2. **Use VIVIM.** The user subagent opens an isolated, disposable Host/PWA session and tries to complete the goal through the actual UI.
3. **Observe.** Record what the user did, what appeared on screen, what changed, and where the experience failed or became confusing.
4. **Diagnose.** A separate agent examines the observations and relevant source to find the actual cause.
5. **Fix.** Developer subagents make the smallest useful change using the existing Chat, VLang/VEC, Host, and Wiki design.
6. **Verify.** Independent checks test the change, including permissions, stale data, and failure cases. A passing API test alone is not proof of a working UI.
7. **Replay.** A fresh user subagent tries the **same original goal**, through the PWA, without being told how the developers fixed it.
8. **Evolve.** Keep evidence and regression journeys. The existing keeper alone updates the canonical roadmap, then feeds the next eligible goal.

**In one line:** user tries → system observes → developers fix → independent tests run → fresh user retries → keeper advances.

## ERA 3 — What counts as progress

Begin with three real-user goals:

1. **"Open the Wiki."** Start in Chat; reach the actual Wiki through the PWA.
2. **"Show me ARBOR progress and blockers."** Produce a useful visual answer backed by current sources, not a canned demo.
3. **"Change this Wiki section."** Understand the request, show an edit preview, require the proper approval, reject stale revisions, and confirm the change survives reload or restart.

**Rules:**

- The user subagent interacts through the real UI. Calling HTTP endpoints alone does not count as using the product.
- The user reports behavior, not code patches, and cannot approve its own success.
- Developer and verifier agents are separate from the user; the replay uses a fresh session.
- Test only in isolated, disposable sessions. Never alter a shared or production Host or send real external actions as part of a simulation.
- Respect existing Host authority, permissions, approvals, journals, and revision checks. Simulations receive only test-scoped permissions.
- Reuse existing implementation, receipts, and regression evidence. Do not create a new orchestration system, roadmap, or competing source of truth.
- Stop repeated failures after a bounded number of attempts and record a real blocker; never claim success because a script merely returned green.

**Done means:** a fresh simulated user can complete the original natural-language goal in the real PWA, see the correct result, and repeat the journey with independent supporting evidence.

## Where this fits

This document **defines the behavior**; it does not claim that the subagent or UI loop is fully implemented.

- Existing workflow: [`arbor-life` in WORKFLOWS.md](../../WORKFLOWS.md)
- Canonical queue: [roadmap.v1.json](../../product/roadmap/roadmap.v1.json)
- Product: [Host/PWA](../../../apps/vivim-host/) — Chat (`/chat`), Wiki (`/wiki`)
- Existing receipts: [roadmap receipts](../../product/roadmap/receipts/)

**Do not run this as a second loop. Add the simulated-user role to the existing `arbor-life` USE and replay stages.**
