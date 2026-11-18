# Proposal 5 — Autonomous Execution Playbook (7 Waves / 6 Parallel Lanes / 1 Serial Gate)

**Date:** 2026-11-12  
**Status:** proposed launchable work orders; execution NOT STARTED; beta NOT BUILT.  
**Destination:** ARBOR integration branch `integrate/proposal5-beta-r001` created by the initial orchestrator from `architecture/proposal5-beta-first-centralization`.  
**One-time objective:** physically consolidate all selected *working* VIVIM components into **one independently buildable and runnable ARBOR repository** with a clean reusable Rust graph kernel, internal operational HQ, native Node 0 minimum, current Host/Studio/mobile/TierFlow preserved, current behavior parity, and one Windows + PWA beta. **Pure migration is an acceptable outcome.** Do not add nonessential features or redesign the interface.

## Operator instructions — no repeated conversation needed

1. **Start with** [00-AUTO-RUN-ALL.md](prompts/00-AUTO-RUN-ALL.md) in an orchestration-capable local agent. It creates the integration branch, launches Wave 0 lanes A–F, waits for their committed evidence, runs gate G, and automatically proceeds to Waves 1–6 only after each gate is GO.
2. If the local agent cannot spawn its own workers, open each wave launch prompt under `prompts/W0` ... `prompts/W6`, fan out **all six A–F lanes at once** onto disjoint branches, then run G **only after all six finish**. Repeat for the next wave without asking for redesigned instructions.
3. Workers use dedicated read/write branches; they **never merge into shared integration or main**. Only gate G changes the shared integration branch; **no worker pushes to ARBOR main**.
4. The orchestrator must report and stop at a hard BLOCKED/NO_GO condition. It should *not* ask the owner to arbitrate normal code choices mid-wave. At the final real-user data migration / cutover / any publication or irreversible action, request **explicit owner approval**; dry-run and staged beta package may proceed without publishing or mutating private owner data.
5. Wave 6 produces a usable **candidate** and can cut over only with explicit approval, tested backups, rollback and clean writer handoff. Do not treat "ran all prompts" as release GO.

## Repo and source constraints

- ARBOR **contains** selected working source code from Harness, HQ JS/HQKG Rust, TierFlow and ratified Labs. **Never** depend on donor repo checkout, dev process, runtime API, symlink, Git submodule or source file from outside ARBOR after integration. Actual Chrome/provider websites, user files, Windows system and paired phone are legitimate external resources.
- Import source **preserving tested paths** initially: `src-tauri/`, `apps/vivim-host/`, `apps/mobile-relay/`, `studio/`, `harness/`, `core/programmability/`, `native/hqkg/`, `hq/`, `packages/tierflow/`. All required dependencies and assets must land internally. No wholesale history merge, private profile, token, owner corpus, runtime DB, `node_modules`, `target/` or generated log.
- Existing **Host JS effect journal remains sole writer**. Rust Node 0, graph, prediction, UI and HQ are pure/read/derived unless routed through the actual Host admission mechanism. No second effect writer, personal World writer or capability catalog.
- `hqkg` Rust from HQ: **one source-preserving canonical product copy** inside ARBOR, with build/profile/test retention; HQ development algorithms/code index also physically land under `hq/`.
- No provider APIs, Playwright or Puppeteer as product automation. Existing signed-in Chrome web app Studio behavior wins.
- Node 0: preserve user local editing, typed source-bound revisions, bounded pure native work. Don't port the R001 mutex-held per-edit scoped thread loop unchanged. No frontend/backend **semantic split**; renderer, OS, device and effect trust boundaries remain physical.
- Bun package workspace and Cargo workspace with a compatible reproducible release manifest, existing Node runtime where current HQ and Host source requires it. Do not rewrite current HQ JS to Rust just to eliminate an extra language.
- UI scope: **minimal verified wiring**, no redesign, no new view modes unless already working or trivial. Labs design-only work is not a release dependency.

## Wave matrix

| Wave | Parallel A–F mission | Serial G integration gate | Dependency |
| --- | --- | --- | --- |
| **W0 — Source Truth** | A native/Tauri; B Host/World; C Studio; D HQ; E mobile/TierFlow; F security/test/ratified Labs inventory | G0 pins the one source-lock, migration map, branch contract and acceptance inventory | Start immediately from Proposal 5 |
| **W1 — Physical Landing** | A HQKG+Rust/Tauri; B Host/World; C Studio/harness; D HQ JS; E PWA/TierFlow; F selected Labs contracts/tests/capability inventory | G1 combines all source trees, Cargo/Bun roots and dependency closure; proves no external path | G0 GO |
| **W2 — Build and Internal Wiring** | A native linker/Tauri; B Host writer/control; C Studio/browser; D internal HQ/graph; E PWA/TierFlow; F shared protocol/schema/capability index | G2 builds a coherent internal vertical slice, with patched native boundary | G1 GO |
| **W3 — Beta Journeys** | A Node 0 minimum; B command/effects; C signed-in provider; D source index/context/wiki; E paired PWA/UI; F end-to-end truth/capability tests | G3 proves critical real source-backed flows, classifies conditional features | G2 GO |
| **W4 — Red-Team Verification** | A Tauri/native security; B effect/data recovery; C provider channel; D HQ isolation/storage; E physical mobile/visual; F independent no-donor/release audit | G4 gives exact release blockers, NO false GO; GO only when hard gates demonstrated | G3 GO |
| **W5 — Blocker Closure** | A–E domain-specific only regression fixes; F packaging/acceptance/evidence closeout | G5 freezes accepted release candidate, no feature creep | G4 GO or explicit repair directive |
| **W6 — Beta Handoff** | A installer; B World/journal migration & rollback drill; C Studio smoke; D HQ/graph offline; E phone UI; F independent acceptance | G6 prepares cutover; execute only with explicit owner permission and single writer | G5 GO |

**A–F stable ownership:** A Rust/Tauri; B Host/World/control; C Studio/browser; D HQ intelligence; E mobile/TierFlow UI; F contracts/Labs/release/independent QA. **G owns** integration branch, root `Cargo.lock`, root `bun.lock`, root workspace manifests, release manifest, and cross-owner change resolution.

## Worker branching and artifact protocol

- Shared work branch: `integrate/proposal5-beta-r001`; created once from the exact Proposal 5 proposal branch.
- Worker branches: `work/p5-wNN-a-native`, `work/p5-wNN-b-host`, `work/p5-wNN-c-studio`, `work/p5-wNN-d-hq`, `work/p5-wNN-e-ui`, `work/p5-wNN-f-verify`.
- Each new wave starts from the **exact current shared integration HEAD after prior G GO**. No worker starts from a stale SHA, donor default branch or local branch with unreviewed changes.
- Each worker reports `BASE_SHA, HEAD_SHA, changed_paths, source_import_SHAs, tests_run_with_exit_codes, evidence_paths, blockers, classification` and commits results. If work is impossible, create an evidence-backed BLOCKED report instead of faking source or passing tests.
- G merges/cherry-picks **one branch at a time**, adjusts root workspace/lockfiles (exclusive writer), runs combined tests, resolves conflicts from actual source owners and emits `GO / NO_GO / REPAIR_REQUIRED` with exact SHA and next wave base. No blanket overwrite or merge solely because a worker says "done."
- A lane may not alter another lane's owned subtree without a documented handoff. For shared code, deliver a small patch proposal in lane-owned report, which G or the designated owner applies.
- On an ordinary gate failure, do not launch the next wave. Repair within the current wave and re-run G. **Exception:** W4-G may issue a finite **REPAIR_HANDOFF** to the specifically designed W5 blocker-closure wave. This is **not release GO**; W5-G must close all hard blockers and return GO_FOR_W6 before W6 starts. On fundamental NO_GO stop.

## Exclusively owned paths

| Owner | Primary write scope |
| --- | --- |
| A | `native/`, `core/programmability/`, `src-tauri/` |
| B | `apps/vivim-host/`, Host World/effect data migration scripts in owned subtree |
| C | `studio/`, `harness/`, Studio owned adapter/fixtures |
| D | `hq/`, local dev intelligence tests and source adapters |
| E | `apps/mobile-relay/`, `packages/tierflow/`, product view-only adapters |
| F | `contracts/`, `capability-spaces/` generated views, `tests/`, verification scripts, security audits |
| G | **Root** `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `package.json`, `bun.lock`, `.github/workflows/`, `integration/source-lock.json`, `integration/release-manifest.json`, merge state and final gate evidence |

The exact first source tree may require source owner approval to expand a worker's scope; record the exception rather than simultaneously editing another worker's files.

## Hard beta gates — grounded in Proposal 5

Reference `08-BETA-GATES.json` (G00–G13). Important hard stops:
1. **No outside VIVIM repository/service wiring**, verified after donor checkouts/processes are unavailable.
2. **Real Windows installer + real paired PWA**, with supported signed-in Chrome webapp conversation where owner login is available.
3. **Native security**: supported patched Tauri, origin-limited privileged command API, verified Host child process before owner token transfer, authenticated actor/device/scopes.
4. **World/effects**: one Host writer, approvals/claims/receipts, no resend after uncertain effect, backward-compatible owner data/rollback.
5. **Native Rust + internal HQ**: compiled HQKG graph, internal HQ indexing/search/context/wiki working with source revisions and no private user-data leakage.
6. **Node 0**: no editor freeze, bounded pure computations, revisioned read-only source interpretations, exact UI/Host action identity; no accidental effect.
7. **No feature creep**: current working media/voice/mission/context surfaces preserved where real; Labs research may stay disabled with an explicit source-backed limitation and owner decision.

## Output every gate must publish

A `wave-NN-GATE.md` in `integration/reports/` and updated machine-readable status:
- baseline head and merged worker head list;
- exact imports, contract IDs, one source owner, unmerged competing duplicates;
- commands executed, OS/browser/device, exit codes and resulting proof;
- P0 blockers and owner decisions;
- `GO` or `NO_GO/REPAIR_REQUIRED`, and exact next-wave base SHA;
- beta feature continuity table: working, verified narrower, preview, blocked, deferred-approved.

**Never claim a beta exists because the instructions have been written.** Only the execution agent can produce real runtime proof on the owner's Windows PC.

## Files under prompts/

- `prompts/00-AUTO-RUN-ALL.md`: one-paste control for an autonomous local orchestrator.
- `prompts/WN/LAUNCH.md`: one-paste launch of six parallel agents plus serial G.
- `prompts/WN/A.md` ... `F.md`: individually copyable self-contained lane instructions.
- `prompts/WN/G.md`: independent integration gate work order (run after A–F, not parallel).

**This playbook does not create background work from this ChatGPT chat.** The user/local orchestration workers run these prompts explicitly.
