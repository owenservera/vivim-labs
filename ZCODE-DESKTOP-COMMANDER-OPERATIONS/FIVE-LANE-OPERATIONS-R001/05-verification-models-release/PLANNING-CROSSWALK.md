# L05 — Planning proposal crosswalk (supporting work, not new master tasks)

This table links the **11 planning candidate IDs** most closely related to L05. The original Planning catalogue contains **37 unique candidates across five lanes**. These are not additional accepted ARBOR #33 deliverables, nor do they confer source ownership. Read live [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33) and [Labs TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md) before performing a step.

**Source:** [Planning master catalogue (21 November 2026)](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/MASTER-PLAN-R001.md). Status below is the *catalogue snapshot*; refresh it each cycle. `PAUSED_OWNER` tasks remain paused unless an explicit later owner decision and corresponding claim refresh authorizes the scope.

| Candidate | Priority / catalogue state | Dependencies | Prepared work / intended test |
|---|---|---|---|
| **PL-001** | P0 / BLOCKED_SOURCE | none | Locate and reconcile canonical OWNER-WORK history, writer, task IDs and source-of-truth semantics; never create empty replacement ledger |
| **PL-002** | P0 / SCOPED | none | Exercise task-claim CAS checkout, conflicting writes, OFFERED_TO and ACCEPTED_BY contracts in a no-effect checklist |
| **PL-003** | P0 / SCOPED | none | Refresh ARBOR and Labs main heads, Dev-owned source paths, code symbols, task claims and active file collision matrix before each packet |
| **PL-005** | P1 / BLOCKED_SOURCE | PL-001 | Map proposal IDs to original OW IDs and preserve local historical events without rewriting or doubling registry |
| **PY-006** | P1 / SCOPED | PL-002,PY-005 | Map atomic Planning OFFERED_TO Dev ACCEPTED_BY ownership with task claim file |
| **PY-008** | P2 / SCOPED | PY-007 | Define measured manual intervention, deterministic Rust reuse and AI-call efficiency counters |
| **VF-001** | P0 / SCOPED | PL-003 | Define separate SOURCE, TEST, PACKAGE, SERVE, AUTH_ACK, HTTPS, GPUI, PWA and PHONE evidence gates |
| **VF-002** | P1 / SCOPED | VF-001 | Scaffold screenshot/visual readback correlation to source SHA, session, Wiki revision and Host receipt |
| **VF-003** | P1 / SCOPED | VF-001,PY-007 | Prepare same-URL PWA release rollback plan, native binary resolver checks and correct Origin/CSRF probes |
| **VF-004** | P1 / SCOPED | VF-001,AW-013 | Author negative matrix: revoked consent, wrong actor, stale epoch, IME, human-agent race, no Send, redaction |
| **VF-005** | P1 / SCOPED | VF-002 | Reuse bounded first-contact observability events for source-linked diagnostic review, not a new journal |

## Required intake behavior

1. Resolve candidate to existing #33/OWNER-WORK task **before** adding work to this lane's execution queue. If the connection is ambiguous, treat it as a dependency/preparation item; do not mint a new product ID.
2. Consult all already prepared Planning packages: [bundle index](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/BUNDLE-INDEX.md), [source reuse map](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/SOURCE-REUSE-MAP-R001.md), and [acceptance/handoff](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/ACCEPTANCE-AND-HANDOFF-R001.md).
3. Existing `OFFERED_TO` remains with the sender until the receiver writes `ACCEPTED_BY`. Source edits require an actual nonconflicting claim, and no historic source PASS substitutes for current source tests.
4. When working on a candidate, preserve its original ID, source references, prior receipts and disposition; report it in the corresponding [canonical task WORK-INDEX](WORK-INDEX.md) only when accurately mapped. No second backlog and no inferred product deployment.
