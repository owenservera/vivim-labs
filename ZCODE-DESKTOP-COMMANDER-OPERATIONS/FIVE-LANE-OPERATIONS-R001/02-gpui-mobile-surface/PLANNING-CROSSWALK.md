# L02 — Planning proposal crosswalk (supporting work, not new master tasks)

This table links the **6 planning candidate IDs** most closely related to L02. The original Planning catalogue contains **37 unique candidates across five lanes**. These are not additional accepted ARBOR #33 deliverables, nor do they confer source ownership. Read live [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33) and [Labs TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md) before performing a step.

**Source:** [Planning master catalogue (21 November 2026)](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/MASTER-PLAN-R001.md). Status below is the *catalogue snapshot*; refresh it each cycle. `PAUSED_OWNER` tasks remain paused unless an explicit later owner decision and corresponding claim refresh authorizes the scope.

| Candidate | Priority / catalogue state | Dependencies | Prepared work / intended test |
|---|---|---|---|
| **AW-016** | P1 / SCOPED | AW-015 | Specify PWA/GPUI semantic parity contract against Dev's existing human/agent typed input when available |
| **PY-004** | P1 / SCOPED | PY-002,PY-003 | Specify PWA status readback through existing Rust LiveSystemState and Host read ports |
| **PY-007** | P1 / SCOPED | PY-004,PY-006 | Define verified build/test/package/serve/HTTPS/visible result/rollback receipt chain |
| **GP-001** | P2 / PAUSED_OWNER | PL-003 | Stage version-accurate GPUI input change observation, IME handling and latest-wins Wiki search in Labs only |
| **GP-002** | P2 / PAUSED_OWNER | GP-001 | Map native composer RegionKind::Input and authorized agent ACK to existing Host control path; not phone.composer impersonation |
| **GP-003** | P2 / PAUSED_OWNER | GP-002 | Prepare actual Windows GPUI visual test and PWA parity evidence, distinct from optional Android |

## Required intake behavior

1. Resolve candidate to existing #33/OWNER-WORK task **before** adding work to this lane's execution queue. If the connection is ambiguous, treat it as a dependency/preparation item; do not mint a new product ID.
2. Consult all already prepared Planning packages: [bundle index](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/BUNDLE-INDEX.md), [source reuse map](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/SOURCE-REUSE-MAP-R001.md), and [acceptance/handoff](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/ACCEPTANCE-AND-HANDOFF-R001.md).
3. Existing `OFFERED_TO` remains with the sender until the receiver writes `ACCEPTED_BY`. Source edits require an actual nonconflicting claim, and no historic source PASS substitutes for current source tests.
4. When working on a candidate, preserve its original ID, source references, prior receipts and disposition; report it in the corresponding [canonical task WORK-INDEX](WORK-INDEX.md) only when accurately mapped. No second backlog and no inferred product deployment.
