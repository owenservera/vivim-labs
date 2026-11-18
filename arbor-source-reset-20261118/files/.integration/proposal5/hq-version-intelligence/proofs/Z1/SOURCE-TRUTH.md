# Z1X — pinned source truth and migration completeness
The target and donor pins are immutable historical source snapshots, not the current branch names. S0 workerBase `f9823a5131e73e7562db5a4ec14e1ec481557d69` is the common ancestor; Z1 must not fork latest main. On inspection, current main was `cbe768031e0797040a09c084b7b8bc12f07fbe0a` and had not accepted these HVI worker sources.

| Metric | Count | Truth boundary |
|---|---:|---|
| Donor repos-hq `hq/**` files | 676 | frozen `repos-hq@9b922ea049dbadc94e504b5b9432743995c020ca` |
| At same target path | 315 | 314 exact Git blob SHA and 1 patched `hq/lib/walk.mjs` |
| Absent at original path | 361 | not proof of feature loss |
| All missing paths enumerated in S0 and D | 361 | row-by-row SHA match, no duplicate |
| Content-inspected path rows | 23 | 6.37% of 361 |
| Not reviewed | 338 | `UNKNOWN_UNREVIEWED`, including most missing tests, graph and historical files |
| Proven semantic equivalence of absent paths | 0 | none promoted to equivalence by the referee |
| Runtime verified missing paths | 0 | no Host/device receipts |

## Donor C4 identity map
B `DONOR-MAP.json` tracks 14 pinned `hq/catalogue-identity` donor paths:
- Seven preserved **exact Git blobs**: `EXPORT-CONTRACT.md`, `cli.mjs`, `crosswalk.mjs`, `labs-adapter.mjs`, `source-adapters.mjs`, `test/benchmark.mjs`, `test/crosswalk.test.mjs`.
- Two reviewed source adaptations: `README.md`, `observer-export.mjs`.
- Five historical donor artifacts deliberately not ported: `HANDOFF.md`, `evidence/cli-smoke.json`, `evidence/node-test.tap`, `evidence/performance.jsonl`, `evidence/source-census.json`. Old TAP/bench evidence must not be assigned to new product test execution.
- B adds `hq/version-intelligence/identity/{index.mjs,legacy-adapters.mjs,test/identity.test.mjs}`. These are pure projection/adapter code, not a competing graph or source identity authority.

## Labs and research
D tracked only **22 selected pinned source candidates**: **11 RESEARCH_IMPLEMENTED, 7 DESIGN_ONLY, 1 DESIGN_AUDIT_ONLY, 1 FIXTURE_OR_AUDIT, 1 TARGET_SOURCE_PRESENT, 1 STAGED_CONTRACT**. These include capability observatory, personal-knowledge/Host intake candidates, Chronos, semantic visual compiler/R005, Rock Core, AutoWiki, interpretation X-ray and Node0 work. None received product-installation evidence. Personal-knowledge evidence must remain exclusively under Host actor/consent/World authority; no personal corpora are part of Z1.

## Specific owner refusals
- Old HQ supervisor calls `node:child_process`; Host already owns process supervision. Reuse only independently reviewed pure admission/recovery, no parallel worker launcher or effect authority.
- Old `hq/live-graph` and `hq/wave1-rust-graph` have possibly useful diagnostics but native `native/hqkg` remains the graph owner; do not import another graph store/journal.
- Existing `hq/lib/catalogue-delta.mjs` and `hq/lib/semantic-registry.mjs` remain canonical historical/semantic contracts; B shims must not alter their stored schema.
- Studio signed-in browser sender, Host effect writer/grant authority and TierFlow visual publisher are all exclusive; the version inspector is development read-only.

The machine-readable `EVIDENCE-COVERAGE.json` carries all **361 pinned rows** and the 22 Labs candidates with explicit uncertainty.
