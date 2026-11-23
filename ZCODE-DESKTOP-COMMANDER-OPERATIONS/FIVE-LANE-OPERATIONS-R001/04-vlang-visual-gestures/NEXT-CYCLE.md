# L04 — Continuing work queue and completion criteria

> **Latest integration sequencing:** At the beginning of every cycle, consult the [shared R004 critical-path SITREP](../STRATEGIC-EXECUTION-SEQUENCE-R004.md). It identifies currently active L01/A16, L04/U08, completed-but-not-deployed L02/S11 and source-pinned runtime/ZCode blockers. Read live GitHub #33 and TASK-CLAIMS to refresh this snapshot before deciding ownership. Do independent no-effect work while a separate lane's approval/claim is blocked; never interpret the R004 historical task order as authorization to override active owners.


**This is an ordering proposal only, not an active checkout.** Each new cycle must select a currently open, unclaimed task from [live #33](https://github.com/owenservera/ARBOR/issues/33) and [live TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md). Do not restart finished work or wait for another agent when an independent safe task is available.

## Initial dependency-ranked queue
1. **U06** — Complete deterministic natural-language-to-command planning without an LLM ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
2. **U07** — Wire VLANG plans to Host approval, execution and receipts ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
3. **U09** — Complete real-time visual feedback and command-state transitions ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
4. **U08** — Complete two-TAB prediction and semantic VIEW ALL ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
5. **U10** — Migrate remaining TierFlow/VEC behavior into shared semantic surface ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
6. **G02** — Connect shared gesture reducer to GPUI input ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
7. **G03** — Connect shared gesture reducer to mobile touch input ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
8. **G04** — Render progressive long-press feedback, preview and cancel states ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
9. **G05** — Prove physical-device gesture behavior and approval boundaries ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
10. **U11** — Expose approved Desktop Commander capabilities through Host authority ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed

## Repeatable development and verification loop

1. Read this folder's [WORK-INDEX](WORK-INDEX.md), [CONTEXT-AND-TOOLS](CONTEXT-AND-TOOLS.md), latest #33/#32/#31 and current claims, then check current ARBOR SHA. If this file's proposed ordering disagrees with new owner priorities or already-completed work, the *live* authority wins.
2. Check out a **narrow task and file scope** via existing TASK-CLAIMS CAS if making source changes; otherwise record a no-write observer. Preserve existing Dev/Planning/Oracle assigned work. No unapproved stale claim seizure.
3. Use primary cloud GitHub source editing (Rust first, TS 7+ for adapters); reuse the existing Host/Studio/GPUI/PWA architecture and signed-in Chrome target. No new effect owner, model registry, workflow engine, provider API or desktop UI clone.
4. Run relevant exact-source regression tests and conditional scoped ZCode or Desktop Commander checks. Snapshot current test SHA and command/exit. A workflow definition is not a running agent; a model route must be cost/health-admitted before invocation.
5. If an effectful step requires user approval or runtime safety preflight, **fail closed**, record the exact gate, and work another independent task. Don't replay uncertain sends or restart active production Host.
6. Check genuine results into existing #33 and the same claim row. Capture status separately for source, tests, packaging, serving, Host ACK, provider, GPUI and actual phone. No counterfeit done.
7. Repeat with the next safe task in the same bounded run and in each enabled hourly dedicated-chat recurrence; no background promises beyond actual verified automation state.

## Exit criteria

Deterministic current source language parsing and safe negation/ambiguity -> explicit Host approval/receipt -> semantic visual transitions -> actual native/mobile gestures, preview, cancellation and physical test proof.

**Completion:** when the live master #33 shows this lane's mapped task IDs accepted, their handoff/receipt record is source-current, safety/release/device stages have true proof, and no newly related owner-approved tasks remain. If the master ledger evolves, update this reference page and its source tests; **do not create a replacement backlog**.
