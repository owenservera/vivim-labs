# ARBOR Acceleration R002 — W1 Lane D: A00 independent evidence handoff

**Lane:** D — A00 Wiki verifier and release-truth refusal  
**Branch:** \`work/accel-r002-w1-d-evidence-truth\` (no main merge)  
**Common baseline:** \`2a42f1af025db8aacb965326b11c73da671ffb3f\`  
**Observed main tip during review:** \`2a42f1af025db8aacb965326b11c73da671ffb3f\`  
**Source commit before this handoff:** \`2076345480ee4393b36b09067bfd8c8fea0ed23c\`  
**Final branch commit:** see exact branch head in owner-facing report; this report is itself the final commit, avoiding a self-referential SHA.  
**Disposition:** **SOURCE_REVIEW_COMPLETE / INDEPENDENT_RUNTIME_ACCEPTANCE_UNRUN / NO_GO_FOR_ACCEPTANCE**.

## 1. Scope and implemented changes

Only Lane D's five permitted paths were written:

1. \`scripts/batch-evidence/WIKI-BASELINE.mjs\`: strict JSON-array /api/journal parsing, strict raw JSONL integrity checks against /demo/journal.jsonl, fail-closed source/API preflight, external JS/CSS evidence distinct from browser DOM, guarded isolated loopback live edits; negative HTTP probes for forged origin, missing explicit action header, forged actor, missing revision and stale revision; no-mutation verification; exactly-one acknowledged append; revision/state/API consistency; unchanged committed seed; explicit UNRUN labels for browser, restart, physical phone, session capability and keeper acceptance.
2. \`scripts/batch-evidence/wiki-evidence-regression.test.mjs\`: synthetic isolated transport fixtures and negative regressions for API shape, corrupt JSONL, empty journal, missing assets, error routes, forged-origin acceptance, broken stale guard, source seed mutation and successful bounded API projection.
3. \`scripts/roadmap/evidence-truth.mjs\`: read-only comparison of all three unchanged source ledgers; explicit gate-by-gate contradictions, zero-job CI/startup failures, G12 self-contradiction, physical-device gap and divergent merge SHA; a consistent green claim still cannot grant release permission; robust rejection of malformed ledgers.
4. \`scripts/roadmap/evidence-truth.test.mjs\`: current-ledger conflicts, zero CI job refusal, G12 contradiction, internally consistent but unverified green-claim refusal and malformed/duplicate gate IDs.
5. \`docs/acceleration/w1/D.md\`: bounded reviewer and keeper handoff (this file).

No roadmap.v1.json, product Host, other lane, release ledger, original receipt, workflow, or main branch was changed. No duplicate authority, interpreter, graph, or publisher was introduced.

## 2. Source-inspected evidence and contradictions

- \`integration/beta-gates.json\`: **UNBLOCKED**, promoted G01–G13 \`ACCEPTED\`, \`mainConsolidation.betaGo=true\`, \`toolchainTests=EXECUTED\`, \`deploymentAuthorized=true\` even though the same record says \`ciConclusion=startup_failure\`, \`ciJobs=0\`. The G12 evidence string explicitly says the Windows runner was **not executed**.
- \`.integration/proposal5/08-BETA-GATES.json\`: **PROPOSED_UNRUN**, all G00–G13 have \`status=UNRUN\`, \`owner_accepted=false\`, \`observed_evidence=null\`.
- \`integration/release-manifest.json\`: **BLOCKED_NOT_BUILT**, \`securityAccepted=false\`, \`deviceAccepted=false\`, \`noDonorBuildAccepted=false\`; Windows and PWA \`UNRUN\`; \`releaseAllowed=false\`; \`mainConsolidation.ciConclusion=startup_failure\` and zero jobs. Its \`mergeCommit=dc2ae811...\` differs from promoted ledger \`7586f28d...\`.
- GitHub API independent observation: run [43334607434](https://github.com/owenservera/ARBOR/actions/runs/43334607434) concluded \`startup_failure\`; \`/jobs\` returned \`total_count=0\`. Run [43233248631](https://github.com/owenservera/ARBOR/actions/runs/43233248631) also concluded \`startup_failure\`. These are **not** test exits or executed acceptance.
- \`apps/vivim-host/server.mjs\`: Wiki edits currently require explicit same-origin \`Origin\`, \`x-vivim-control: 1\`, typed actor and \`expectedRevision\`; /api/journal returns a JSON array, /demo/journal.jsonl returns raw JSONL; /wiki loads external assets. No independent session-bound security capability is established by those headers.
- \`apps/vivim-host/chat/product-journal.mjs\`: replay projects Wiki and chat from the Host data-root journal. The read path currently silently skips malformed JSONL lines. Lane D detects this in evidence via raw journal rather than claiming that Host corruption behavior is repaired. Any fail-closed Host hardening is Lane A's exclusive source scope.
- \`apps/vivim-host/public/pages/wiki.js\`: rendering is asynchronous browser JS; response HTML alone cannot prove rendered DOM. Source/API checks are \`SOURCE_AND_API_ONLY\`, not browser/device E2E.

All original ledger claims, receipts, queue statuses and owner-acceptance fields remain untouched.

## 3. Actual execution, environment and test status

**Observed environment:** isolated tool container, Linux x64; \`node --version\` = \`v22.16.0\` (**exit 0**); \`git --version\` = \`2.47.3\` (**exit 0**). The runtime is **not** Node24 or the Windows Host. No ARBOR checkout was mounted (\`test -d /mnt/data/ARBOR\` **exit 1**). \`git ls-remote https://github.com/owenservera/ARBOR.git HEAD\` **exit 128**: DNS failure (\`Could not resolve host: github.com\`). Connected GitHub source read/write works separately, but does not execute Node processes.

| Requested command | Status | Literal exit |
|---|---|---|
| \`node --test scripts/batch-evidence/wiki-evidence-regression.test.mjs scripts/roadmap/evidence-truth.test.mjs\` | **UNRUN** — no executable local checkout | N/A |
| \`node scripts/roadmap/evidence-truth.mjs --check\` | **UNRUN** — no executable local checkout | N/A |
| \`node scripts/batch-evidence/WIKI-BASELINE.mjs\` (static) | **UNRUN** — no executable local checkout | N/A |
| \`ARBOR_WIKI_EVIDENCE_ISOLATED=1 ARBOR_WIKI_EVIDENCE_BASE_URL=http://127.0.0.1:<ephemeral-port> node scripts/batch-evidence/WIKI-BASELINE.mjs --live\` | **UNRUN** — requires Lane B's real isolated Host | N/A |
| Real browser DOM render, restart replay, Node24/Windows, physical PWA and independent keeper acceptance | **UNRUN** | N/A |

The new tests are committed and reproducible but **must not be reported as passing** until a runner actually executes them. GitHub file/diff verification is a source review, not test execution. No fabricated exit statuses or browser/device receipts.

## 4. Independent A00 acceptance recommendation

**NO_GO_FOR_ACCEPTANCE** for A00 at this handoff. Accept the scoped source work for integration review only. Release/beta promotion is **NO_GO**; neither a green-looking source claim nor a clean source diff is sufficient. Current blocker categories: contradictory G00–G13 gate states; source claims of executed tests versus actual CI zero-job startup failure; missing independent Node24, real browser, restart, security and device receipts; incomplete source-backed session authorization proof.

**Lane A request:** verify revision refusal, same-origin/actor limits, forged session/capability threat model, replay across actual Host restart, and corruption behavior at the Host journal authority. Do not imply that a forged \`Origin\` and actor string prove session-bound authorization.

**Lane B request:** run the exact two Lane D Node suites and \`--check\` under an isolated Node24 Host, plus \`WIKI-BASELINE.mjs --live\` using only Lane B's ephemeral dataRoot. Publish command, OS/Node24, exit codes, stdout/stderr, source SHA and failing receipts. Resolve CI \`startup_failure\` so a job process actually starts. Do not merge or mark A01 accepted on source-only evidence.

**Lane C request:** supply real browser DOM and external-asset evidence for the second screen, read-only against the isolated Host, without treating this verifier's regex/source checks as visual acceptance.

**Keeper request:** preserve the contradictory historical ledgers and receipts. Inspect all lane SHAs and run independent acceptance before changing canonical roadmap state. A00 \`verification=UNRUN\` and \`ownerAcceptance=PENDING\` remain authoritative until a real keeper receipt; A03 does not unlock yet. A04 similarly awaits independent A02 acceptance. Do not mark beta release accepted.

## 5. Next ready frontier / acceleration

Read-only canonical queue observation: **A00, A01, A02** are in progress with unaccepted verification; A03–A08 remain dependent/queued. No effectful queue flip, owner authorization, release approval, or cross-lane merge was performed. Subsequent-work speedup: **NOT_YET_MEASURED**.
