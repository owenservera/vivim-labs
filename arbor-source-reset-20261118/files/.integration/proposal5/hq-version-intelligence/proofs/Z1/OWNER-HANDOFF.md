# Z1X → G1 owner handoff

> **LATER GOVERNING SCOPE OVERRIDE, main@539146ac741516e7d9711c63f4570585b3c3367e — NATIVE TOOLING ONLY.** The 2026-11-12 HVI `10-NATIVE-TOOLING-ONLY-SCOPE.md` explicitly overrides earlier real five-repository/361 semantic audit and X-lane requirements. Preserve historical S0/D proof as reference only. **G1 tests entirely synthetic 2/3/5 invented repos, provides a stable native callable tool API; H/U only after `GO_FOR_W2_TOOLING`; X deferred until expressly requested; Z2/G2/R certify tooling/readiness, not migration corpus.** See `SCOPE-CORRECTION-ADDENDUM.md`. Any older contradictory text below is **superseded**.

**Gate: REPAIR_REQUIRED (planning/source review only).** All source heads and immutable base are frozen in `BRANCH-SHA-MATRIX.json`. Z1 contains **no fixes** and **no** integration branch/main edits.

| Urgency | Required narrow repair | Authorized owner | Exact source/review |
|---|---|---|---|
| P0 | Verify native A manifest against native SHA256, construct C projection digest separately, bind both to pin+commit+tree, refuse silent substitution | G1 cross-seam; A/C owner review | `hq/version-intelligence/sources/{common,snapshots}.mjs`; `hq/version-intelligence/compare/index.mjs`; `hq/version-intelligence/integration/source-manifest-adapter.mjs` proposed |
| P0 | Replace misleading direct F v1 conformance with explicit **versioned v2** schemas covering real C typed objects, sourcePins, coverage arrays, evidence, limits, owner and error shape | G1 integration / F contract owner | `contracts/hq-version-intelligence/schemas/v1/{pin,manifest,comparison,migration-finding}.schema.json` historical, `schemas/v2/` proposed |
| P0 | Restore C `originPath`/`originBlob` in E display; preserve C source/target SHA, identityCandidates, errors, missing/partial distinctions | G1 / C/E owner review | `hq/version-intelligence/report/index.mjs`, `hq/version-intelligence/integration/comparison-report-adapter.mjs` proposed |
| P0 | Join D rows without promoting physical MISSING_AT_TARGET to lost capability, receipt state or retirement | G1 / D | `hq/version-intelligence/parity/atlas.mjs`, `hq/version-intelligence/integration/parity-evidence-adapter.mjs` proposed |
| P0 | Observe real Node24 integrated suite plus genuine no-donor/3 SHA replay and test blob receipts; no work on main before reviewed PR | G1 / F | `tests/hq-version-intelligence/`, `hq/test/catalogue-delta.test.mjs`, `proofs/G1/TEST-RECEIPTS.md` |
| P0 | Source grants, malicious path/symlink/submodule, secret leaks and incomplete manifest refusal | A/G1 for operator source; H/G2 for live Host authority | `sources/adapters.mjs`, `sources/snapshots.mjs`, `contracts/hq-version-intelligence/` |
| P1 | B source pin normalized repo, stable lineage and aliases, C4 legacy exports, owner decision validation; no fake equivalence | B/C/G1 | `hq/version-intelligence/identity/{index,legacy-adapters}.mjs`, `hq/catalogue-identity/EXPORT-CONTRACT.md` |
| P1 | DEFERRED: 338 original HQ source paths are historical unknowns; no current-wave semantic review, only future explicit user-triggered audit | X / HQ with Host/Rust/TierFlow owners | `proofs/D/HQ-361-TRIAGE.json`, `proofs/Z1/EVIDENCE-COVERAGE.json` |
| P1 | Decide whether any donor supervisor/admission/recovery utilities can be pure adapters without duplicate Host supervisor, and graph readers without duplicate native/hqkg graph store | Host/HQKG/TierFlow with X; no automatic code import | `proofs/D/OWNERSHIP-CONFLICTS.md` |
| Future gate | Host live per-operation actor/session/grant/revoke, PWA/world isolation; inert TierFlow read-only inspector | H/U/G2 after accepted G1 `GO_FOR_W2_TOOLING` | No H/U/X start from this referee's source-only branch |

## Exact pin / outcome
- S0: `2ef7a9f66419d681e040b459ba4762a63aa9564f`; worker base `f9823a5131e73e7562db5a4ec14e1ec481557d69`, `GO_TO_PARALLEL`.
- A: `53d4bddf4c2a8fd22f12c8d7f3949aac5fd2fe38`.
- B: `1206a343ab98a6e7abe0c19a9549c6b8c32ea9e0` (23 commits).
- C: `7f72c1415d75a7904eb5c267c2a55a9172d33cd8`.
- D: `622569e9b20f1d6dfae474594eecf544d4fe3d9d`.
- E: `d8080a4056361266702f38cf47f2a0885fe892eb`.
- F: `34f5c1568007977e0e3bf0286d4c1ec730d6af26` (16 commits).
- Source-only base parity: original `repos-hq@9b922ea049dbadc94e504b5b9432743995c020ca` → ARBOR `f9823a5131e73e7562db5a4ec14e1ec481557d69`: donor HQ 676, same-path315 (314 exact +1 patched), missing-original-path361, inspected23, UNKNOWN338.
- **G1 prerequisite:** owner must read exact `INTEGRATION-REPAIR-ORDERS.md` and all ten Z1 proof files; `REPAIR_REQUIRED` is handoff, not refusal to inspect a G1 repair PR. G1 remains solely responsible for integration fixes, runner receipts, PR review and final accept/reject. Z1 authorizes zero direct source edits/merges.

**Release:** `integration/beta-gates.json` remains `BLOCKED_NOT_BETA`. The original CI startup_failure ran zero jobs. NO runtime, Host, Windows, Android, installer, signer, grant or device acceptance.
