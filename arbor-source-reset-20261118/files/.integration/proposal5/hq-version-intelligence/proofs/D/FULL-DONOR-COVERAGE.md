# HVI Lane D — full original HQ donor coverage

**Status: PREPARED_NOT_PROMOTED.** S0 is `GO_TO_PARALLEL`; original HQ pinned `9b922ea049dbadc94e504b5b9432743995c020ca`; target pinned `f9823a5131e73e7562db5a4ec14e1ec481557d69`.

## Fixed Git blob census
- Donor `hq/**`: **676** tracked blobs; **315** same mapped path; **314** identical; **1** patched (`hq/lib/walk.mjs`); **361** not at original path.
- All **361** missing-path rows have exact Git source revision and blob SHA. **23/361 (6.37%)** were individually inspected as source contents; the other **338** remain `UNKNOWN_UNREVIEWED`.
- This is **100% missing-path enumeration**, **6.37% contents inspected**, **0 accepted runtime verification**. Physical path absence is NOT missing capability. There are no automatic tombstones.
- Archived logs, source tests and prior handoffs are not executed tests. Node 24/Bun/Cargo/Windows/Android and Host grant tests are UNRUN in this audit.

## Families
| Family | Missing paths | File contents inspected | UNKNOWN_UNREVIEWED |
|---|---:|---:|---:|
| `hq/test` | 144 | 3 | 141 |
| `hq/live-graph` | 75 | 6 | 69 |
| `hq/convergence` | 38 | 1 | 37 |
| `hq/wave1-rust-graph` | 29 | 5 | 24 |
| `hq/work` | 25 | 1 | 24 |
| `hq/aii` | 21 | 0 | 21 |
| `hq/catalogue-identity` | 14 | 4 | 10 |
| `hq/audits` | 6 | 0 | 6 |
| `hq/supervisor` | 4 | 3 | 1 |
| `hq/AGENT-PROTOCOL.md` | 1 | 0 | 1 |
| `hq/CONTRACT.md` | 1 | 0 | 1 |
| `hq/config` | 1 | 0 | 1 |
| `hq/git` | 1 | 0 | 1 |
| `hq/integration` | 1 | 0 | 1 |

## Direct source observations and owner constraints
- `hq/catalogue-identity/crosswalk.mjs` exports `stableRecordId` and `buildCrosswalk`; source adapters and Labs projections remain source-only. Lane **B** owns canonical identity adaptation, not a second identity registry.
- `hq/supervisor/supervisor.mjs` imports `node:child_process`, exports `launchLane`/`dispatchTasks`. Host already has `apps/vivim-host/bootstrap/worker-supervisor.mjs` and `apps/vivim-host/runtime/supervisor.mjs`; Host owner must rule on pure admission/recovery reuse and reject duplicate effects.
- `hq/live-graph/wave1/master/build-graph.mjs` assembles graph inputs from original HQ and may spawn processes. ARBOR Rust `native/hqkg` is the existing graph owner. Read/compile portions are candidates, not accepted replacement graph installations.
- `hq/wave1-rust-graph/lane-b/round-2/bounded-reader.mjs`, `sqlite-inspector.mjs` and `knowledge-lifecycle.mjs` have executable read-only diagnostics. Their unique semantic queries must be compared with HQ/Native before any import.
- Original 144 `hq/test/**` paths need owner-sensitive triage. Inspected `automation.test.mjs` uses Node; `kb.test.mjs` and `agent-tool-grants.test.mjs` use Bun. Existing retained `hq/test/semantic-registry.test.mjs` was inspected but **not** counted in the 361 missing rows.
- `hq/work/lane-a3-handshake.mjs` imports channel-broker for a live provider turn; no automatic provider sender/effect migration to HQ.

See `HQ-361-TRIAGE.json` for complete row-level uncertainty, proof refs and owner candidates. No personal data, tokens, profile content or runtime sessions were ingested.
