# SCF near-code launch order
**Full integration work order for one agent:** [MASTER](prompts/00-MASTER-IMPLEMENT-ALL.md). Copy the exact agent starter below and point it to the existing draft PR #13; separate lanes optional but independence gate required.

## One-agent starter (copy into another ChatGPT conversation)
```text
ARBOR PROPOSAL 5 — IMPLEMENT SCF + SV FROM ALREADY FINISHED ENGINEERING BLUEPRINT.
Use connected GitHub. Read COMPLETE master work order:
https://github.com/owenservera/ARBOR/blob/work/p5-semantic-capability-fabric-r001/.integration/proposal5/semantic-capability-fabric/implementation-ready/prompts/00-MASTER-IMPLEMENT-ALL.md
Read all surrounding implementation-ready contracts, module recipes, source pin/ABI lock, synthetic fixtures, test matrix, verifier proposal, native HVI tooling-only governing scope. DO NOT REDESIGN. First reconcile actual current main, HVI accepted source/PRs, existing SCF draft PR #13, exact SHAs. Execute S0 and follow work-orders.json dependencies/owned paths. Build actual source code, tests and proof receipts in scoped branches, run real Node24 synthetic tests, use independent review before G1/G2 and serial PR merges only with verified tests. No real ARBOR vs donors/Labs migration comparison, no provider APIs/personal data/effect authority, no false runtime/beta GO. Report exact commit SHAs, commands and exits vs UNRUN, blockers and final readiness.
```

## Parallel/sequential launch plan
`S0 → [A,B,F parallel] → [C,D,E parallel] → Z1 → G1 → [H,U parallel] → Z2 → G2 → R`.
**Cannot launch H/U until G1 actually accepted**; cannot call G1 accepted while HVI native tool missing/UNRUN. Real donor comparison is out of scope.

| Step | Role | Complete prompt |
|---|---|---|
| S0 | sequential | [01-S0-BASE-AND-ABI-LOCK.md](prompts/01-S0-BASE-AND-ABI-LOCK.md) |
| A | parallel | [10-A-COMPILER.md](prompts/10-A-COMPILER.md) |
| B | parallel | [10-B-REGISTRY-STORE.md](prompts/10-B-REGISTRY-STORE.md) |
| F | parallel | [10-F-TEST-ORACLES.md](prompts/10-F-TEST-ORACLES.md) |
| C | parallel | [20-C-PASSPORT-HVI-WIKI.md](prompts/20-C-PASSPORT-HVI-WIKI.md) |
| D | parallel | [20-D-SEMANTIC-VERIFIER.md](prompts/20-D-SEMANTIC-VERIFIER.md) |
| E | parallel | [20-E-AGENT-SERVICE.md](prompts/20-E-AGENT-SERVICE.md) |
| Z1 | sequential | [30-Z1-INDEPENDENT-REFEREE.md](prompts/30-Z1-INDEPENDENT-REFEREE.md) |
| G1 | serial-integrator | [40-G1-SERIAL-INTEGRATION.md](prompts/40-G1-SERIAL-INTEGRATION.md) |
| H | parallel | [50-H-HOST-NATIVE-READ.md](prompts/50-H-HOST-NATIVE-READ.md) |
| U | parallel | [50-U-TIERFLOW-INSPECTOR.md](prompts/50-U-TIERFLOW-INSPECTOR.md) |
| Z2 | sequential | [60-Z2-PRODUCT-REFEREE.md](prompts/60-Z2-PRODUCT-REFEREE.md) |
| G2 | serial-integrator | [70-G2-PRODUCT-INTEGRATION.md](prompts/70-G2-PRODUCT-INTEGRATION.md) |
| R | sequential | [80-R-FINAL-SYNTHETIC-READINESS.md](prompts/80-R-FINAL-SYNTHETIC-READINESS.md) |

## Operational rule
Worker files are owned by the explicit paths in `work-orders.json`; branch names are reserved but not yet created. Each worker reports `baseSha,branchHeadSha,changedPaths,sourcePins,testsExecuted,testsUnrun,securityResult,gate`. No agent should treat documentation-only status as already implemented. Any new HVI interfaces from moving `main` require S0 ABI reconciliation and explicit owner handoff, never guessing.
