# L03 — Planning proposal crosswalk (supporting work, not new master tasks)

This table links the **16 planning candidate IDs** most closely related to L03. The original Planning catalogue contains **37 unique candidates across five lanes**. These are not additional accepted ARBOR #33 deliverables, nor do they confer source ownership. Read live [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33) and [Labs TASK-CLAIMS](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md) before performing a step.

**Source:** [Planning master catalogue (21 November 2026)](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/MASTER-PLAN-R001.md). Status below is the *catalogue snapshot*; refresh it each cycle. `PAUSED_OWNER` tasks remain paused unless an explicit later owner decision and corresponding claim refresh authorizes the scope.

| Candidate | Priority / catalogue state | Dependencies | Prepared work / intended test |
|---|---|---|---|
| **PL-004** | P0 / SCOPED | none | Disposition Oracle PWA_BUILD_INGRESS shaping signal against existing task IDs, no new dispatcher |
| **AW-001** | P0 / SCOPED | PL-003 | Inspect staged R003 Cargo manifests, five Rust modules, HQKG adapter and 37 authored tests; run cargo test/fmt/clippy in authorized cloud toolchain |
| **AW-002** | P0 / SCOPED | PL-003 | Audit existing core/programmability and native/hqkg Cargo feature/dependency directions and exact R003 module registration |
| **AW-003** | P0 / SCOPED | AW-002 | Crosswalk 7 wiki hub aliases to existing Host wiki:page identities and HQKG node IDs |
| **AW-004** | P0 / SCOPED | AW-003 | Trace existing Host actor grants, source allowlists and exact HQKG query_scoped seeds/exclusions |
| **AW-005** | P0 / SCOPED | AW-004 | Define request sequence, actor grant, wiki revision/digest, graph epoch and watermark fences for read projections |
| **AW-007** | P0 / SCOPED | AW-006 | Build claim-to-English fidelity matrix for existing R003 narrative.rs: negation, tense, units, actors, unknowns and disputes |
| **AW-008** | P1 / SCOPED | AW-003,AW-007 | Prepare canonical seven-hub page, link and truthful-empty-state contracts using existing Wiki graph |
| **AW-009** | P1 / SCOPED | AW-005,AW-008 | Use existing context_lens.rs for ambient/selected/pinned pages and Overview/Explain/Trace depth |
| **AW-010** | P0 / SCOPED | AW-004,AW-005 | Identify exact existing owner-scoped Host read port and transport for Wiki/HQKG projection |
| **AW-013** | P0 / SCOPED | AW-004,AW-005,AW-010 | Write negative test specification for redacted links, private graph neighbors, raw drafts, revoked source, developer-only trace |
| **AW-014** | P1 / SCOPED | AW-001,AW-004 | Bound HQKG projection by existing scoped query depth, tokens, item cap and completeness |
| **AW-015** | P0 / SCOPED | AW-001,AW-002,AW-003,AW-004,AW-010,AW-013 | Prepare one Rust-first R003 Dev handoff: source pins, tests, insertion points, rollback and collision checks |
| **PY-001** | P1 / SCOPED | PL-004 | Design explicitly submitted PWA build/change intent from existing composer using Rust admission rather than ambient Wiki lookup |
| **PY-002** | P1 / BLOCKED_SOURCE | PL-001,PY-001 | Locate existing canonical durable work request writer; do not introduce a replacement database |
| **PY-003** | P1 / SCOPED | PY-001,PY-002 | Specify idempotency, deny, cancel, conflicting edits and offline replay using Host effect authority |

## Required intake behavior

1. Resolve candidate to existing #33/OWNER-WORK task **before** adding work to this lane's execution queue. If the connection is ambiguous, treat it as a dependency/preparation item; do not mint a new product ID.
2. Consult all already prepared Planning packages: [bundle index](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/BUNDLE-INDEX.md), [source reuse map](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/SOURCE-REUSE-MAP-R001.md), and [acceptance/handoff](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/ACCEPTANCE-AND-HANDOFF-R001.md).
3. Existing `OFFERED_TO` remains with the sender until the receiver writes `ACCEPTED_BY`. Source edits require an actual nonconflicting claim, and no historic source PASS substitutes for current source tests.
4. When working on a candidate, preserve its original ID, source references, prior receipts and disposition; report it in the corresponding [canonical task WORK-INDEX](WORK-INDEX.md) only when accurately mapped. No second backlog and no inferred product deployment.
