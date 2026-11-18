# Baseline source-path crosswalk / migration investigation
These are **mechanical path-prefix measurements** against Git trees, not semantic/behavioral migration status. Absence at mapped path means triage, not lost feature. A file at the mapped path may be non-executable. The first live HVI dogfood must improve this to semantic lineage and runtime reachability.

| Original source subtree | Target prefix in ARBOR | Donor files | Present same mapped path | Byte identical | Absent mapped path |
|---|---|---:|---:|---:|---:|
| repos-hq `hq/` | `hq/` | 676 | 315 | 314 | 361 |
| vivim-web-harness `apps/` | `apps/` | 241 | 222 | 217 | 19 |
| vivim-web-harness `studio/` | `studio/` | 958 | 379 | 379 | 579 |
| vivim-web-harness `harness/` | `harness/` | 132 | 122 | 122 | 10 |
| vivim-web-harness `core/` | `core/` | 7 | 7 | 7 | 0 |
| vivim-web-harness `src-tauri/` | `src-tauri/` | 14 | 13 | 12 | 1 |
| vivim-web-harness `tools/` | `tools/` | 47 | 1 | 0 | 46 |
| vivim-web-harness `schemas/` | `schemas/` | 6 | 0 | 0 | 6 |
| tierflow `src/` | `packages/tierflow/src/` | 86 | 85 | 85 | 1 |
| tierflow `tests/` | `packages/tierflow/tests/` | 80 | 80 | 80 | 0 |
| tierflow `web/` | `packages/tierflow/web/` | 12 | 12 | 12 | 0 |
| tierflow `runtime/` | `packages/tierflow/runtime/` | 4 | 2 | 2 | 2 |
| tierflow `VIVIM-VISUAL-RUNTIME/` | `packages/tierflow/VIVIM-VISUAL-RUNTIME/` | 273 | 0 | 0 | 273 |

**Caution:** Multiple donor paths may map to a common target and count same target twice; the table is not a count of distinct migrated functionality. `vivim-labs` has no approved canonical path-prefix mapping; source and research evidence must be classified via semantic crosswalk. The 273 visual-runtime files absent at this *assumed* path may be staged under another topology or intentionally excluded; do not call them lost until investigated.

## Specific HQ gaps to cross-reference
1. `hq/catalogue-identity` 14 files: CLI, pure crosswalk, source adapters, Labs observer, tests, benchmark/contract/evidence; research implemented, not promoted.
2. `hq/live-graph/wave1` 75 and `hq/wave1-rust-graph` 29: determine replacement in native/hqkg vs useful read model/test harness; DO NOT copy a duplicate graph.
3. `hq/supervisor` 4: investigate bounded admission/recovery design versus Host canonical supervision; do not create second effect executor.
4. `hq/test` 144 absent: split tests applicable to ARBOR dev intelligence from legacy media/agent/automation, evaluate coverage, avoid importing secret fixtures.
5. `hq/convergence` 38, `hq/aii` 21, `hq/work` 25, historical audit docs: classify archival/tracker/generated/adaptable/live.
6. Labs: capability observatory, Chronos, personal knowledge, user IO, R005–R010, SWC, AutoWiki, Rock Core and interpretation X-ray; pin main/research branch, mark source vs design and owner.

## Mandatory output per candidate
`origin_repo`, `origin_sha`, `origin_path`, `origin_blob`, `target_repo`, `target_sha`, `target_path[]`, `target_blob[]`, `identity_basis`, `source_lineage`, `capability_id`, `source_type`, `tests`, `runtime_evidence`, `decision`, `certainty`, `missing_reason`, `owner`, `merge_recommendation`, `evidence_refs[]`, `coverage`.
