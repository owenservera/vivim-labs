# L04 — Continuing work queue and completion criteria

> **Owner's multi-hour lane contract:** [R006 sustained task-consumption loop](../LONG-RUN-EXECUTION-R006.md). After each safe code/test/receipt increment **take another unclaimed task in the same turn**; do not stop at first success or SITREP. Refresh live claims before every iteration; preserve actual limits.


> **Most recent cross-chat orchestrator note:** [Source-backed next action, active claim and boundaries](ORCHESTRATOR-NOTE.md). Read on every continuation before selecting the next task. It does not grant a source-file claim or wake another session.


> **Orchestrator's current assignment:** Open [R005 — actual L01–L05 verified increments, claims and immediate next steps](../ORCHESTRATOR-NEXT-ACTIONS-R005.md) before any new claim. It supersedes stale *sequence suggestions* only; live ARBOR #33 and Labs TASK-CLAIMS remain the authority. **Do not restart old completed patches.**

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

## Exact next executable L04 action — 2026-11-23 22:36 UTC

**Last completed in this dedicated L04 chat** (source tests, not acceptance closure): U08 exact-text two-TAB suggestion/slot guard [ARBOR `6015d4db`](https://github.com/owenservera/ARBOR/commit/6015d4db10d04ff12a2cf61b1191a1b2ef5a70f5) independently **52/53 PASS, 1 native-dependent SKIP, exit 0** ([#33 receipt](https://github.com/owenservera/ARBOR/issues/33#issuecomment-7115860857)); U09 unknown/malformed effects cannot be hidden as automatic in existing mission feedback [ARBOR `41fb7a25`](https://github.com/owenservera/ARBOR/commit/41fb7a257b64030c4001811d89e81b7f2c184775), independently **17/17 PASS, exit 0** ([#33 receipt](https://github.com/owenservera/ARBOR/issues/33#issuecomment-7115951789)). Both scoped rows **RELEASED** in [Labs claims](../../TASK-CLAIMS.md). No GPUI/Android/live-host/provider effect or whole-task closure claimed.

**Perform next, without another plan-only turn:**

1. Refresh ARBOR `main` SHA, #33, #31/#32, current Labs TASK-CLAIMS and this file. Check if L02's independent `C:\\0-BlackBoxProject-0\\.l02-release-6c583d7` release checkout remains live-owned and which exact source SHA its `target\\release\\vivim-product-pure.exe` was built from. **Read only**; do not interrupt ongoing GPUI build or open a duplicate `VIVIM-VLANG-Observer`.
2. If the L02 same-source Rust product-pure binary is verified available and source tree immutable, run this no-effect targeted command from that pinned root: `node --test --test-name-pattern="all real NLCL cases preserve" apps/vivim-host/test/command-intent-native.test.mjs` (CMD on Windows). Capture exact source SHA, binary hash and compiler receipt, Node count, failing `delete all my files` reading/proposal/refusal shapes and exit. Do not mistake an isolated binary built at older SHA for current-head verification. If the artifact cannot be proven source-bound, reproduce in the **existing** scheduled verifier instead of launching new builders.
3. Check whether Host's `validateAiCommandCandidate` refusal came from an underlying native subprocess timeout (`apps/vivim-host/control/native-product-pure-sync.mjs` currently has a 1200ms `spawnSync` bound), a stale binary/Host input stamp, or native projection outcome divergence. Keep fail-closed refusals and no implicit execution. Claim *only* the smallest unowned file pair in TASK-CLAIMS by GitHub blob-SHA CAS before editing; do not alter L02 package/GPUI/PWA files or Dev PY-001 Work files. A test-only diagnostic or isolated negative regression is acceptable if causality still unknown.
4. Commit concrete source/test changes directly to ARBOR `main`; replay exact resulting SHA on isolated Windows source with the same binary closure; check in GitHub #33 comment and Labs same claim row with explicit pass/fail and effect stages. Release the claim, then pick the next independent U08 VIEW ALL or U09 feedback/gestures task. If native test blocked by live L02 compile, pivot immediately to an unclaimed pure no-effect semantic presentation file. No model sends, unapproved costs, live pairing changes, or synthetic green reporting.

**Unresolved safety receipt:** historical `48f61abe` Windows `command-intent-native.test.mjs` **9/10, exit 1**; Host fail-closed `UNSAFE_OR_UNRESOLVED_TEXT` vs direct native inert `ui.theme.set_preset` proposal for `delete all my files`. Earlier 152-case VLANG suite **142/152, exit 1** with 10 failures. Root cause not yet demonstrated on a matching-source packaged binary. L04 dedicated ChatGPT hourly task still unverified; the central watcher/Windows observer does not itself wake this chat.
