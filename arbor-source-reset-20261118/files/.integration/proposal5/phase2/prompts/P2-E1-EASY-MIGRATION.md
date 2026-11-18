# CHATGPT COPY PROMPT — PHASE 2 E1: EASY LABS MIGRATION PREPARATION

You are **ARBOR Proposal 5 Phase 2 / EASY**. Work directly through connected GitHub tools, in this ChatGPT conversation. **Do not wait for W1 A–F.**

Destination: `owenservera/ARBOR`. Start from `architecture/proposal5-beta-first-centralization`; create isolated branch `work/p5-p2-easy-labs`. Source: `owenservera/vivim-labs`, pin exact SHAs of `main`, `lab/nlp-visual-command-ui-20261111`, `lab/user-io-rust-core-20261112` and any relevant staging branches. Read `.integration/proposal5/10-PHASE2-LABS-INTAKE.md`, `11-PHASE2-DECISION-MATRIX.md`, plus Proposal 5 beta gates.

**Assignment — actual GitHub source inspection and code/evidence preparation, not just a plan:**
1. Inspect `capability-observatory/lane-d/` (source, tests, schemas, refresh and dashboard), the main Observatory seed and verified extension-point evidence. Identify which files/functions can directly improve ARBOR's source status and migration truth without a new service or registry.
2. Inspect R005 `nlp-command-ui-lab/` source, including `semantic-lens.mjs`, `bridge.mjs`, `affordances.mjs`, tests and the R005 report. Select **pure, inert** semantic/negation/span/source-provenance functions or test fixtures only. Do not import a second UI shell, effect parser or catalog.
3. Inspect Chronos `wave2` pure read projections and tests; selected HQ wiki quality/privacy/provenance regression fixtures. Exclude any source that duplicates an A–F W1 owner or requires additional scheduler/runtime.
4. Produce a **copy-ready manifest** of source paths and exact destination owner/path with `PROMOTE_AS_TEST`, `PROMOTE_PURE`, `REFERENCE_ONLY` or `REJECT`, current source SHA and real test status. Create **synthetic, self-contained static fixtures/test packs** only where they can be written without modifying W1 lane-owned code.
5. Commit deliverables **only under** `.integration/proposal5/phase2/easy/` on your isolated branch. This includes `INTAKE.md`, `promotion-manifest.json`, relevant small source-preserving standalone test/fixture copies if license-safe, and `HANDOFF-G.md`.

**Non-negotiable:** Do not write to `src-tauri/`, `native/`, `apps/`, `studio/`, `harness/`, `hq/`, `packages/tierflow/`, `contracts/`, `capability-spaces/`, root manifests, shared integration/main, or the original Labs repo. Do not create an external runtime dependency. Do not claim executed tests unless actually executed.

Report branch HEAD, exact source SHAs and paths, counts of EASY items, tests actually run and precise G merge instructions. The existing W1-F worker owns integration of shared contracts; **your output is a nonconflicting prepared patch/evidence package**.

Start GitHub inspection and commit now.