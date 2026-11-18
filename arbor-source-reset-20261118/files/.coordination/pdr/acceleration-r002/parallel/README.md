# ARBOR Acceleration R002 — W1 five-lane parallel execution

Owner instruction: five independent ChatGPT GitHub-connected execution chats, one lane each. This folder is work-order source, NOT a competing roadmap or authority. Canonical machine queue: docs/product/roadmap/roadmap.v1.json. Baseline before this handoff: main 60510e4868dde794bcca6e24baaa998cae78b8fc (2026-11-14); A00/A01/A02 source landed but Node24/real-browser/physical-device tests UNRUN. A03/A04/A06 dependencies must NOT be called accepted until their prerequisites are independently verified.

## Mandatory read order for every worker
1. This README plus the worker's assigned A.md, B.md, C.md, D.md or E.md in this folder.
2. docs/product/roadmap/ACCELERATION-R002.md and roadmap.v1.json (items A00–A08, owner acceptance and hard dependencies).
3. .coordination/pdr/ACCELERATION-R002-HANDOFF.md, .coordination/pdr/PDR-batch1-exit.md.
4. Source files listed in the specific work order; inspect real code before editing.

## Five source-disjoint lanes

| Lane | Goal | Exclusive writable source ownership | Default branch |
|---|---|---|---|
| A | A02 Host journal authority + crash/revision safety | apps/vivim-host/server.mjs; chat/product-journal.mjs; test/product-journal.test.mjs; test/product-routes.test.mjs | work/accel-r002-w1-a-host-authority |
| B | A01 real Node24/Host acceptance and deterministic receipt | scripts/acceleration/live-host-check.mjs; live-host-check.test.mjs; .github/workflows/acceleration-r002-w1-b.yml | work/accel-r002-w1-b-runtime-proof |
| C | A02→A04 source-backed second-screen Wiki and visual UX | apps/vivim-host/public/pages/wiki.js, wiki.css, wiki.html; test/wiki-visual-contract.test.mjs | work/accel-r002-w1-c-wiki-visual |
| D | A00 independent evidence and release-truth | scripts/batch-evidence/WIKI-BASELINE.mjs; scripts/roadmap/evidence-truth.mjs and evidence-truth.test.mjs; test under scripts/batch-evidence/ | work/accel-r002-w1-d-evidence-truth |
| E | A03 feedback→work-order deterministic compiler (preparatory until A00 accepted) | scripts/feedback-loop/** | work/accel-r002-w1-e-feedback-compiler |

Each lane may create ONLY its own evidence report under docs/acceleration/w1/{A,B,C,D,E}.md. All other files are READ-ONLY, including roadmap.v1.json, test-manifest.json, shared work orders, .coordination status/gates, files owned by other lanes, and release manifests. No cross-lane cherry-picks or main merges; report integration needs to the handoff.

## Execution and integrity

- Work ONLY on the assigned branch created from a common main SHA; if branch exists confirm it matches and do not reset another worker's work.
- Code-first: implement useful behavior/tests immediately, not merely notes or scaffolding. Reuse Host, NLCL, VEC, HQKG, existing test runner and approval contracts; no duplicate interpreter, graph store, publisher or queue.
- No provider APIs, Playwright/Puppeteer, privileged provider effects, new remote login, manual fake receipts, real user data reads, or unapproved automation.
- Actual executed Node, Bun, OS/browser tests must carry command, observed exit status, environment and exact SHA; otherwise mark UNRUN. GitHub source checks do not count as runtime tests.
- No task is accepted because source landed, a generated pass claim exists or a worker says 'done'. Owner gate/keeper sole-writer rules remain. Lane D is independent verifier but cannot unilaterally flip queue.
- Use fresh branch head for every write. Do not touch another lane's files even for a convenient test fix; document the integration request.
- Never delete historical receipts; never rewrite unverified beta/device states to green. Security and authority are hard invariants.
- Each lane commits in its own branch, reports exact commit SHA and changed paths, test commands and actual exit codes/UNRUN, remaining blockers, and a one-paragraph integration handoff.

## Integration sequencing (later, not performed by these parallel lanes)

Review source/evidence for A+B+C+D+E independently. Merge or cherry-pick serially only after ownership/evidence review, rebase/re-pin as needed, and run combined Node24 tests. A00/A01/A02 need independent acceptance before activating A03/A04/A06 effects. The feedback compiler E can be built/tested *read-only* early; it may NOT write roadmap or execute build jobs until A00 is accepted. C can implement and verify visual contracts, but cannot claim full A04 NL→VEC accepted until A02 is accepted and genuine compiler tests run.

## Owner-facing completion grid

A: fresh revision/authorization, durable replay, refusal tests. B: exact Node24/Host exits and artifacts. C: browser-safe second-screen projection, change preview, stale refusal and visual proof. D: no false green, proof semantics and release conflict kept blocked. E: deliberate failure yields deterministic deduplicated source-pinned work order without any autonomous Host effect.
