# PHASE 2 — PRE-G PARALLEL CHATGPT PREPARATION

You are an independent Phase 2 preparation lane running **inside ChatGPT using connected GitHub**. Wave 1 A–F have landed source; their final freeze and G merge are separate. Begin from ARBOR branch work/p5-pre-g-convergence-r001 and create your own new branch; NEVER edit or reset the active worker branches, integrate/proposal5-beta-r001, main, or other Phase2 branches.

Read .integration/proposal5/pre-g/04-REFRESHED-DECISION-AND-PHASE2-SLOTS.md, .integration/proposal5/10-PHASE2-LABS-INTAKE.md and .integration/proposal5/phase2/readiness/SOURCE-TO-ARBOR.json from work/p5-phase2-readiness-docs-r001. Pin exact donor HEAD/commit and source SHA, not a stale earlier report.

**Objective:** Prepare source-grounded small reusable tests/adapter contracts that can be selectively promoted AFTER G. This lane must NOT delay G, alter product source, or claim runtime/device acceptance. Work directly against GitHub and commit real documentation/fixtures/tests in ONLY the assigned isolated directory. If you have no execution environment, be explicit: CODE_WRITTEN, TEST_UNRUN.

## Lane P2-K: PERSONAL KNOWLEDGE T=0

Branch to create: work/p5-p2-preg-knowledge-r001
Exclusive write path: .integration/proposal5/phase2/pre-g/K-knowledge/

### Tasks
Read the actual Labs personal-knowledge-lifecycle/runtime source (ZIP parser, streaming JSON, ChatGPT normalizer, t0-core, checkpoint and Host ledger/graph ports), its tests and statuses; compare against current imported ARBOR B Host corpus/onboarding/World and D HQ dev indexing and A HQKG source. Identify what is already in Host and what is only in Labs. Write a file/commit-level dependency DAG and proposed owner-specific integration patches for B/A/D, without modifying runtime now. Focus on consent, raw chat custody, ZIP traversal/zip bombs, incremental reimport idempotency, checkpoints, source revision/provenance, bounded disk, protected Rock Core, private data isolation and revoke/erasure. Write optional synthetic *contract tests only* in .integration/proposal5/phase2/pre-g/K-knowledge/; never import real user's chats, issue provider actions, add a second personal DB or assert operational Host integration. Deliver READY_FOR_W2_OWNER or DEFER, with exact reason, effort and tests.

### Required artifacts
1. README.md with source pins and observed behavior versus proposed.
2. source-delta.json classifying each source path as ALREADY_IMPORTED, TEST_ONLY, ADAPTER_REQUIRED, DESIGN_ONLY, DEFERRED, BLOCKED, with exact owner.
3. acceptance.md: the smallest useful real post-G executable tests, test commands, negative security cases and actual executed vs unrun status.
4. owner-handoff.md: no more than 5 low-cost cherry-pick or delegated tasks for G/F/A/B/D/E with explicit dependencies.
5. Optional minimal actual test/fixture files inside this folder (never a product replacement).
6. A committed HEAD and disposition PREPARED_NOT_PROMOTED.

### Freeze rules
Do not merge, edit product-owned code, request private user data, invent provider availability, or create new action authority. You may finish independently; G is NOT WAITING for your result.