# Proposal 5 — PRE-G Round 2 decision, refreshed after first follow-ups

**Review date:** 2026-11-12. **Review method:** GitHub branch heads, recent commits, source-owned worker reports and corresponding compatibility patches. **No integration branch or executable combined beta exists as of review.**

## Executive decision

**YES: run exactly three parallel Phase 2 preparatory ChatGPT lanes before G.** They are strictly isolated, with write scope limited to their own new `.integration/proposal5/phase2/pre-g/` deliverable directories on individual branches. They prepare source-selected tests, adapter work orders and permission-failure harnesses; **they do not land more runtime code into the existing W1 workers or G integration branch**. This keeps beta critical path unchanged.

The previous blanket "send PREG-A..F" prompt pack is now stale: **A has already declared FREEZE_READY and committed PREG-A. Do not resend A or reawaken it.** B–F have completed their first follow-up work, but most have not committed the final short freeze report. Read the latest worker branch and execute focused one-time closure instructions in this round. Source-generated reports are not proof of runtime acceptance.

## Current latest worker source state

| Lane | Latest GitHub HEAD observed | First follow-up assessment | Final next action |
| --- | --- | --- | --- |
| A Rust/Tauri | `dda546a11294920d6521a0f64ab7ee62ec993c70` | `integration/worker-reports/PREG-A.md` exists, **FREEZE_READY**. 99 exact imported source blobs; A security plan only, not source fix. | **NO ADDITIONAL PROMPT. FREEZE A.** |
| B Host | `c12f4a27a8c700c9df4cba17a9ff96efd5897748` | `W01-B-FOLLOWUP.md` documents bootstrap handshake and World authority; no native launch code applied. | One short freeze/Host cross-owner report; do not rewrite Host before integration. |
| C Studio | `4bca551d58a1972cc02a8b20b772e1e2417b874d` | 4 source-import commits, `studio/` and `harness/` present; no canonical migration report found. | One final **inventory/report-only** prompt, freeze. |
| D HQ | `aa6284812432cbc375bbe6b1f94bb867a0e72cfa` | W01D follow-up implemented HQ-owned source hardening and auth-scoped dev intelligence read port; Node/Windows/Host integration unrun. | One short PREG-D with modified source blob map and Host integration contract, freeze. |
| E PWA/TierFlow | `4ed530f1702e9ce3f592e2ae1a7080fa36e5f93d` | browser WebCrypto/IndexedDB signer, compatibility tests, exact B-owned Host static allowlist patch and W02-E report committed; G integration absent. | One short PREG-E reporting new code and tests; **do not redo signer**. |
| F Contracts | `7538ebc706f98c15530fe39ba3a40b479448b37f` | source/fixtures/interop and test-only Host catalog; V8 synthetic assertions recorded, Node/Cargo/Windows unrun. | One short freeze plus merged-source test plan, no new generic ontology. |

Worker HEADs are observed at audit, not a substitute for each lane's own final frozen SHA. Follow-up patches remain isolated; the agreed integration branch `integrate/proposal5-beta-r001` was absent when rechecked.

## Two beta-critical cross-owner findings with changed risk

**S1 native process-identity and owner capability:** still a blocker. A/B have compatible *designs*, but Tauri remains pinned to an old family, remote privilege scope is too broad, and authenticated child launch is not yet implemented or exercised. Do not ask independent Labs workers to fix S1; G merges original source, W2 A/B implement jointly.

**S2 browser-PWA enrollment:** initial source gap is no longer simply "no browser signer". E's new implementation uses WebCrypto P-256, nonextractable IndexedDB-stored key, and fail-closed lost-key handling. E identified one required **B-owned** Host static asset allowlist addition in `apps/vivim-host/direct-phone.mjs`, supplied as `integration/worker-reports/W02-E-B-HOST-STATIC.patch`. E also identified an active device session bearer replay surface that requires **B/G/F policy and threat-model review**, not unsanctioned E Host edits. Synthetic compatibility checks are not physical/browser Host acceptance.

## Why Phase 2 now

After G source merge, any tests and cheap adapter interfaces prepared on separate branches will be quicker to import. But new runtime implementation now would produce merge churn, mix ownership and risk turning beta integration into a parallel feature project.

Run **three preparation lanes only**:
1. **P2-T — Executable readiness and differential regression test packs.** Source-grounded Observatory, pure voice, R005 semantic/UI and Chronos fixture verification; create test modules under dedicated pre-G Phase2 staging path, not shared `tests/`.
2. **P2-K — Personal Knowledge T=0 migration engineering.** Deep read current Labs ZIP/normalizer/checkpoint implementation plus W1 B/D/A source. Produce exact typed ports, import DAG, privacy/consent/cancellation/receipt negative tests in isolated staging; no user-data ingestion, no new persistence, no Host live source edits.
3. **P2-C — Native Node0/Chronos + visual read interoperability.** Compare Labs R001/Chronos and R005 with imported A/D/E/F contracts, identify minimum pure reusable source/test adapters and eliminate duplicates. Prepare compilation-ready isolated contract/fixture changes only, no second Rust scheduler or TierFlow publisher.

**Do not run H-category high-value feature development yet.** R007, AutoWiki per-screen mirroring, full Rock Core compactor, generalized app builder, Automation Queen and 100 predictive ideas offer value, but starting them here would directly compete with G/W2 capacity. H-category work stays a source-grounded ranked backlog, not code promotion.

## G critical path

B,C,D,E,F final FREEZE_READY → independent read-only freeze audit → G source merge. Phase2 T/K/C do **not** need to finish for G to begin and must never block G unless they find a concrete current security/data-loss defect with proof. G may import their selected fixtures **only after** core six imports are internally consistent, under explicit owner assignment, preferably W2.

Status vocabulary: `SOURCE_PRESENT`, `SOURCE_PATCHED`, `TEST_EXECUTED`, `HOST_WIRED`, `DEVICE_ACCEPTED`, `PREPARED_NOT_PROMOTED`, `UNRUN`. Do not confuse these.

## Recommended immediate sequence

- Skip A; send refreshed one-time prompts to **B,C,D,E,F** in their existing ChatGPT conversations.
- Concurrently start **P2-T, P2-K, P2-C** in three new ChatGPT conversations on three separate branches forked from this pre-G documentation branch.
- Have one independent ChatGPT referee collect frozen SHA/report for all six, including the already frozen A; don't make the referee depend on Phase2 output.
- Start G once all six source inputs are frozen. Include the phase2 readiness ledger as an optional post-merge W2 handoff, not as a prerequisite to G.
