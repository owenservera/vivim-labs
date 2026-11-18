# COPY-PASTE WAVE 2 LAUNCH — INTERNAL BUILD, TRUST & CONTRACTS

You are the **ARBOR Proposal 5 wave 2 orchestrator**. This prompt is complete; do not request another ChatGPT prompt to continue. Use the local agent/workers available to you. Read the full `.integration/proposal5/09-EXECUTION-PLAYBOOK.md`, all Proposal 5 docs/08-BETA-GATES.json, plus the seven files in `.integration/proposal5/prompts/W2/`.

## Bootstrap
Read W02 prerequisites: G1 GO + root source workspaces; patch Tauri and Host trust and wire internal data/source contracts. Do not launch this wave until its preceding serial gate returned the permitted handoff status and exact integration HEAD.

## Spawn 6 parallel lanes, separate work branches
- **A:** `.integration/proposal5/prompts/W2/A.md` — Native Rust / Tauri.
- **B:** `.integration/proposal5/prompts/W2/B.md` — Host control / World / effect receipts.
- **C:** `.integration/proposal5/prompts/W2/C.md` — Studio / signed-in browser.
- **D:** `.integration/proposal5/prompts/W2/D.md` — HQ intelligence and source evidence.
- **E:** `.integration/proposal5/prompts/W2/E.md` — mobile PWA / TierFlow visual runtime.
- **F:** `.integration/proposal5/prompts/W2/F.md` — contracts, selected Labs, independent QA.

Each subagent gets the current exact integration HEAD, own branch `work/p5-w02-<lane>`, its designated paths and access to donor sources for W0/W1 import only. Worker output MUST be a committed branch SHA, tests with real exit codes, implementation evidence, blockers and a disjoint report. No lane edits shared root lockfiles or integrates itself.

## Serial integration barrier
Wait for **all six** lane branches or explicit evidence-backed BLOCKED reports. Then run **`.integration/proposal5/prompts/W2/G.md`** in a separate integration-owner context. G reviews source, merges serially, owns root Cargo/Bun/release/CI, reruns combined tests, and publishes `integration/reports/W02-GATE.md` and exact next SHA. Never run G in parallel with lane writes.

## Autocontinue
**After G2 GO automatically launch W3.** If a security/privacy/effect writer gate is fundamentally blocked, stop with precise source and test evidence; never claim GO. **Don't broaden scope**: as-is source integration, minimum Rust/security fixes, no visual redesign or speculative Labs feature work. No external VIVIM repo/runtime dependency in installed ARBOR; one Host effect writer; no owner data cutover without explicit approval.
