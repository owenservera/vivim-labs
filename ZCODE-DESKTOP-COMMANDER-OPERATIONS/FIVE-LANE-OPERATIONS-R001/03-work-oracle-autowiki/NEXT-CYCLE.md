# L03 — Continuing work queue and completion criteria

> **Most recent cross-chat orchestrator note:** [Source-backed next action, active claim and boundaries](ORCHESTRATOR-NOTE.md). Read on every continuation before selecting the next task. It does not grant a source-file claim or wake another session.


> **Orchestrator's current assignment:** Open [R005 — actual L01–L05 verified increments, claims and immediate next steps](../ORCHESTRATOR-NEXT-ACTIONS-R005.md) before any new claim. It supersedes stale *sequence suggestions* only; live ARBOR #33 and Labs TASK-CLAIMS remain the authority. **Do not restart old completed patches.**

> **Latest integration sequencing:** At the beginning of every cycle, consult the [shared R004 critical-path SITREP](../STRATEGIC-EXECUTION-SEQUENCE-R004.md). It identifies currently active L01/A16, L04/U08, completed-but-not-deployed L02/S11 and source-pinned runtime/ZCode blockers. Read live GitHub #33 and TASK-CLAIMS to refresh this snapshot before deciding ownership. Do independent no-effect work while a separate lane's approval/claim is blocked; never interpret the R004 historical task order as authorization to override active owners.


**This is an ordering proposal only, not an active checkout.** Each new cycle must select a currently open, unclaimed task from [live #33](https://github.com/owenservera/ARBOR/issues/33) and [live TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md). Do not restart finished work or wait for another agent when an independent safe task is available.

## Initial dependency-ranked queue
1. **U05** — Prove continuously refreshed persisted-source AutoWiki ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
2. **O01** — Connect real ChatGPT conversation ingestion to Oracle ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
3. **O03** — Persist knowledge with provenance and revisions ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
4. **O04** — Demonstrate meaningful personal knowledge in AutoWiki ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
5. **O05** — Complete incremental updates, deduplication, corrections and deletion ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
6. **O02** — Run all seven personal discovery passes on authorized real data ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed
7. **O06** — Validate onboarding and private corpus handling on both devices ([canonical task](https://github.com/owenservera/ARBOR/issues/33)); evidence and claim ownership must be refreshed

## Repeatable development and verification loop

1. Read this folder's [WORK-INDEX](WORK-INDEX.md), [CONTEXT-AND-TOOLS](CONTEXT-AND-TOOLS.md), latest #33/#32/#31 and current claims, then check current ARBOR SHA. If this file's proposed ordering disagrees with new owner priorities or already-completed work, the *live* authority wins.
2. Check out a **narrow task and file scope** via existing TASK-CLAIMS CAS if making source changes; otherwise record a no-write observer. Preserve existing Dev/Planning/Oracle assigned work. No unapproved stale claim seizure.
3. Use primary cloud GitHub source editing (Rust first, TS 7+ for adapters); reuse the existing Host/Studio/GPUI/PWA architecture and signed-in Chrome target. No new effect owner, model registry, workflow engine, provider API or desktop UI clone.
4. Run relevant exact-source regression tests and conditional scoped ZCode or Desktop Commander checks. Snapshot current test SHA and command/exit. A workflow definition is not a running agent; a model route must be cost/health-admitted before invocation.
5. If an effectful step requires user approval or runtime safety preflight, **fail closed**, record the exact gate, and work another independent task. Don't replay uncertain sends or restart active production Host.
6. Check genuine results into existing #33 and the same claim row. Capture status separately for source, tests, packaging, serving, Host ACK, provider, GPUI and actual phone. No counterfeit done.
7. Repeat with the next safe task in the same bounded run and in each enabled hourly dedicated-chat recurrence; no background promises beyond actual verified automation state.

## Exit criteria

Explicitly authorized Work/knowledge ingestion -> provenance/identity/grants -> stable CAS and Wiki readback -> correction/deletion and revocation proof -> meaningful same-source output on native and mobile.

**Completion:** when the live master #33 shows this lane's mapped task IDs accepted, their handoff/receipt record is source-current, safety/release/device stages have true proof, and no newly related owner-approved tasks remain. If the master ledger evolves, update this reference page and its source tests; **do not create a replacement backlog**.

## L03 independent no-write verification — 2026-11-23 22:27 UTC

- ARBOR latest search-listed `main` commit `d0e5d8a6b6a5d5b39c5aef93b1bb5aa92614cd96` (search result, not a local checkout). GitHub source read at this ref: `core/programmability/src/wiki_read.rs` blob `9d33b4399931855383b79b6c661487a1b6737b8b`; `wiki_full_source_validation_contract.rs` blob `03a9d6e07899a90fc828a0148eaa81123896ed2a` (default-branch read).
- Deterministic source inspection: 4/4 structural predicates true (full-source validator called, function defined, search cap 256, digest computed after validation). **STATIC ONLY**; Cargo tests UNRUN, exit N/A: connected GitHub provides source, execution container cannot resolve github.com, no clean immutable source checkout. ZCode runId UNRUN; runtime/GPUI/phone UNVERIFIED. No personal data, sends or Host effects.
- Labs claims blob `67ce27a8663b54b4f4e9f3f7611fd5385fec2411`: U05 `wiki-full-source-validation-20261123` ACTIVE and SOURCE UNTESTED; Dev PY-001 ACTIVE. **No source checkout or source edits attempted; no claim released or stolen.**
- **Exact next executable action:** L03 active U05 owner must fetch current ARBOR main and Labs claims; run `cargo test --manifest-path core/programmability/Cargo.toml --test wiki_full_source_validation_contract --offline` from clean isolated exact-SHA source with a Rust toolchain; record count, exit and SHA in the *existing U05 row* and #33, then RELEASE that claim. If source changed, reconcile first. After release, CAS claim a disjoint O05 source-backed Wiki correction/deletion/readback test path only if not occupied. Do not initialize Work, ingest corpus or restart Host.
