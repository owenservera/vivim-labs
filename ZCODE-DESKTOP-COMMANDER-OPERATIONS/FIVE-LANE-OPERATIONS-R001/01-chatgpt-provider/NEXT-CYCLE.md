# L01 — Continuing work queue and completion criteria

> **Most recent cross-chat orchestrator note:** [Source-backed next action, active claim and boundaries](ORCHESTRATOR-NOTE.md). Read on every continuation before selecting the next task. It does not grant a source-file claim or wake another session.


> **Orchestrator's current assignment:** Open [R005 — actual L01–L05 verified increments, claims and immediate next steps](../ORCHESTRATOR-NEXT-ACTIONS-R005.md) before any new claim. It supersedes stale *sequence suggestions* only; live ARBOR #33 and Labs TASK-CLAIMS remain the authority. **Do not restart old completed patches.**

> **Latest integration sequencing:** At the beginning of every cycle, consult the [shared R004 critical-path SITREP](../STRATEGIC-EXECUTION-SEQUENCE-R004.md). It identifies currently active L01/A16, L04/U08, completed-but-not-deployed L02/S11 and source-pinned runtime/ZCode blockers. Read live GitHub #33 and TASK-CLAIMS to refresh this snapshot before deciding ownership. Do independent no-effect work while a separate lane's approval/claim is blocked; never interpret the R004 historical task order as authorization to override active owners.


**This is an ordering proposal only, not an active checkout.** Each new cycle must select a currently open, unclaimed task from [live #33](https://github.com/owenservera/ARBOR/issues/33) and [live TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md). Do not restart finished work or wait for another agent when an independent safe task is available.

## Initial dependency-ranked queue
1. **A01** — Reconcile the historical uncertain ChatGPT turn without resending ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
2. **A02** — Resolve the existing busy Studio session safely ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
3. **A03** — Deploy the isolated ChatGPT channel into the live runtime ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
4. **A04** — Verify a new ChatGPT prompt reaches the correct composer ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
5. **A05** — Verify exactly one send and the correct conversation/thread ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
6. **A06** — Read back a genuine completed ChatGPT reply ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
7. **A07** — Project the verified reply through the existing Host relay ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
8. **A08** — Render and acknowledge the reply on the physical phone ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
9. **C01** — Complete authenticated ChatGPT.com interactive-surface capture ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
10. **C02** — Finish composer/message/streaming parity matrix ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
11. **C03** — Validate dialogs, attachments, menus and interaction feedback ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
12. **A09** — Deploy and validate the mobile AI chooser and conversation history ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
13. **A10** — Connect native GPUI composer to actual AI provider conversations ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed

## Repeatable development and verification loop

1. Read this folder's [WORK-INDEX](WORK-INDEX.md), [CONTEXT-AND-TOOLS](CONTEXT-AND-TOOLS.md), latest #33/#32/#31 and current claims, then check current ARBOR SHA. If this file's proposed ordering disagrees with new owner priorities or already-completed work, the *live* authority wins.
2. Check out a **narrow task and file scope** via existing TASK-CLAIMS CAS if making source changes; otherwise record a no-write observer. Preserve existing Dev/Planning/Oracle assigned work. No unapproved stale claim seizure.
3. Use primary cloud GitHub source editing (Rust first, TS 7+ for adapters); reuse the existing Host/Studio/GPUI/PWA architecture and signed-in Chrome target. No new effect owner, model registry, workflow engine, provider API or desktop UI clone.
4. Run relevant exact-source regression tests and conditional scoped ZCode or Desktop Commander checks. Snapshot current test SHA and command/exit. A workflow definition is not a running agent; a model route must be cost/health-admitted before invocation.
5. If an effectful step requires user approval or runtime safety preflight, **fail closed**, record the exact gate, and work another independent task. Don't replay uncertain sends or restart active production Host.
6. Check genuine results into existing #33 and the same claim row. Capture status separately for source, tests, packaging, serving, Host ACK, provider, GPUI and actual phone. No counterfeit done.
7. Repeat with the next safe task in the same bounded run and in each enabled hourly dedicated-chat recurrence; no background promises beyond actual verified automation state.

## Exit criteria

New owner-submitted turn -> correct signed-in composer/thread -> exactly one confirmed send -> settled reply -> Host receipt -> GPUI and physical-phone rendered ACK. Browser-state inspection alone is not sufficient.

**Completion:** when the live master #33 shows this lane's mapped task IDs accepted, their handoff/receipt record is source-current, safety/release/device stages have true proof, and no newly related owner-approved tasks remain. If the master ledger evolves, update this reference page and its source tests; **do not create a replacement backlog**.
