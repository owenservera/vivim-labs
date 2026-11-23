# L05 — Continuing work queue and completion criteria

**This is an ordering proposal only, not an active checkout.** Each new cycle must select a currently open, unclaimed task from [live #33](https://github.com/owenservera/ARBOR/issues/33) and [live TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md). Do not restart finished work or wait for another agent when an independent safe task is available.

## Initial dependency-ranked queue
1. **Z02** — Establish ongoing parallel AutoWiki, VLANG, visual and safety improvement loops ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
2. **Z03** — Resolve model admission and 429 limits safely ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
3. **Z04** — Enable actual screenshot-based visual evaluation ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
4. **Z07** — Align ZCode evidence with GitHub without duplicating source work ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
5. **R04** — Repair GitHub Actions runner/startup infrastructure ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
6. **R06** — Unify source, build, runtime, test and device evidence ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
7. **R07** — Close outstanding native plugin and security packaging gates ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
8. **R01** — Inventory and reconcile local-only ARBOR changes ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
9. **R03** — Resolve 57-branch archive/consolidation task ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed

## Repeatable development and verification loop

1. Read this folder's [WORK-INDEX](WORK-INDEX.md), [CONTEXT-AND-TOOLS](CONTEXT-AND-TOOLS.md), latest #33/#32/#31 and current claims, then check current ARBOR SHA. If this file's proposed ordering disagrees with new owner priorities or already-completed work, the *live* authority wins.
2. Check out a **narrow task and file scope** via existing TASK-CLAIMS CAS if making source changes; otherwise record a no-write observer. Preserve existing Dev/Planning/Oracle assigned work. No unapproved stale claim seizure.
3. Use primary cloud GitHub source editing (Rust first, TS 7+ for adapters); reuse the existing Host/Studio/GPUI/PWA architecture and signed-in Chrome target. No new effect owner, model registry, workflow engine, provider API or desktop UI clone.
4. Run relevant exact-source regression tests and conditional scoped ZCode or Desktop Commander checks. Snapshot current test SHA and command/exit. A workflow definition is not a running agent; a model route must be cost/health-admitted before invocation.
5. If an effectful step requires user approval or runtime safety preflight, **fail closed**, record the exact gate, and work another independent task. Don't replay uncertain sends or restart active production Host.
6. Check genuine results into existing #33 and the same claim row. Capture status separately for source, tests, packaging, serving, Host ACK, provider, GPUI and actual phone. No counterfeit done.
7. Repeat with the next safe task in the same bounded run and in each enabled hourly dedicated-chat recurrence; no background promises beyond actual verified automation state.

## Exit criteria

Authenticated exact source/claim/modeled cost and run liveness; CI jobs actually allocated and executing; signed release evidence separately sourced from package/serving/device; no phantom workers or paid fallback.

**Completion:** when the live master #33 shows this lane's mapped task IDs accepted, their handoff/receipt record is source-current, safety/release/device stages have true proof, and no newly related owner-approved tasks remain. If the master ledger evolves, update this reference page and its source tests; **do not create a replacement backlog**.
