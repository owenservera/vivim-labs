# DevOps & Planning master task catalogue — R001

**21 November 2026. Planning proposals only; NOT a replacement OWNER-WORK ledger or active checkouts.**

**Observed pins:** ARBOR main `e2ef16ec221fb752fd50a1e5808d7e9a7a8e3e42`, Labs main `b0eb56e41d76ba785eff91858eb82584b08a1162` before this pass. Refresh before implementation. Owner has not authorized editing ARBOR application source from Planning. Dev owns both live PWA typing milestones and product integration. The GPUI preparation scope remains PAUSED by prior owner directive.

## Design law

One Rust fabric / existing HQKG graph / one Host effect and permission authority / existing durable Wiki journal / existing PWA composer and Dev release workflow. R003 is a read-only projection into existing modules, NOT another graph, database, journal, parser, compiler, command router, chat worker or mobile app. Ambient typing never writes a Wiki entry or creates work. The existing Lab R003 package has 37 **authored but UNRUN** Rust tests, not 37 passed tests. Physical Android is a separate optional acceptance surface, not a native-code prerequisite.

## Planning status vocabulary

- `SCOPED`: Planning candidate, **unclaimed**, not finished; priorities are suggested, not canonical OWNER-WORK ranking.
- `BLOCKED_SOURCE`: no authorized remote canonical task/request source proved; cannot call it complete.
- `PAUSED_OWNER`: explicitly paused. Do not check it out until resumed.
- No task below is Dev's current PWA implementation. Those appear under external dependencies only.

**37 Planning task candidates:** 5 Governance; 16 AutoWiki R003; 8 PWA development pipeline; 3 Native GPUI; 5 Verification.

## Next ten Planning-only candidates, in recommended working order

| Order | Candidate | Prep action | Dependency / blocker |
|---:|---|---|---|
| 1 | **PL-003** | Refresh ARBOR and Labs main heads, Dev-owned source paths, code symbols, task claims and active file collision matrix before each packet | none |
| 2 | **PL-004** | Disposition Oracle PWA_BUILD_INGRESS shaping signal against existing task IDs, no new dispatcher | none |
| 3 | **AW-001** | Inspect staged R003 Cargo manifests, five Rust modules, HQKG adapter and 37 authored tests; run cargo test/fmt/clippy in authorized cloud toolchain | PL-003 |
| 4 | **AW-002** | Audit existing core/programmability and native/hqkg Cargo feature/dependency directions and exact R003 module registration | PL-003 |
| 5 | **AW-003** | Crosswalk 7 wiki hub aliases to existing Host wiki:page identities and HQKG node IDs | AW-002 |
| 6 | **AW-004** | Trace existing Host actor grants, source allowlists and exact HQKG query_scoped seeds/exclusions | AW-003 |
| 7 | **AW-010** | Identify exact existing owner-scoped Host read port and transport for Wiki/HQKG projection | AW-004,AW-005 |
| 8 | **AW-005** | Define request sequence, actor grant, wiki revision/digest, graph epoch and watermark fences for read projections | AW-004 |
| 9 | **AW-006** | Crosswalk VLANG/SCF verifier receipts to R003 claim verification state and source evidence | AW-003 |
| 10 | **AW-013** | Write negative test specification for redacted links, private graph neighbors, raw drafts, revoked source, developer-only trace | AW-004,AW-005,AW-010 |

## Complete task catalogue (proposal keys, not canonical OW IDs)

### Governance

| ID | Priority | State | Dependencies | Preparation / scaffold deliverable | Acceptance evidence |
|---|---|---|---|---|---|
| **PL-001** | P0 | BLOCKED_SOURCE | none | Locate and reconcile canonical OWNER-WORK history, writer, task IDs and source-of-truth semantics; never create empty replacement ledger | Existing requests, IDs and status transitions preserved without invented check-ins |
| **PL-002** | P0 | SCOPED | none | Exercise task-claim CAS checkout, conflicting writes, OFFERED_TO and ACCEPTED_BY contracts in a no-effect checklist | One active executor; rollback and stale claim reconciliation documented |
| **PL-003** | P0 | SCOPED | none | Refresh ARBOR and Labs main heads, Dev-owned source paths, code symbols, task claims and active file collision matrix before each packet | Exact base SHAs, REUSE/REPAIR/NEW/UNKNOWN table and accountable source owners |
| **PL-004** | P0 | SCOPED | none | Disposition Oracle PWA_BUILD_INGRESS shaping signal against existing task IDs, no new dispatcher | One recorded ACCEPTED/AMENDED/DEFERRED response with first unproven edge |
| **PL-005** | P1 | BLOCKED_SOURCE | PL-001 | Map proposal IDs to original OW IDs and preserve local historical events without rewriting or doubling registry | Original event identity, source revision, ownership and count reconciliation |

### AutoWiki R003

| ID | Priority | State | Dependencies | Preparation / scaffold deliverable | Acceptance evidence |
|---|---|---|---|---|---|
| **AW-001** | P0 | SCOPED | PL-003 | Inspect staged R003 Cargo manifests, five Rust modules, HQKG adapter and 37 authored tests; run cargo test/fmt/clippy in authorized cloud toolchain | Command, version, exit and test count; otherwise UNRUN with cargo absence |
| **AW-002** | P0 | SCOPED | PL-003 | Audit existing core/programmability and native/hqkg Cargo feature/dependency directions and exact R003 module registration | No dependency cycle; no parallel Wiki or graph; ready patch locations |
| **AW-003** | P0 | SCOPED | AW-002 | Crosswalk 7 wiki hub aliases to existing Host wiki:page identities and HQKG node IDs | Stable typed IDs; ambiguous/missing IDs refused, no second registry |
| **AW-004** | P0 | SCOPED | AW-003 | Trace existing Host actor grants, source allowlists and exact HQKG query_scoped seeds/exclusions | Current trusted grant producer and revoked/cross-user negative cases |
| **AW-005** | P0 | SCOPED | AW-004 | Define request sequence, actor grant, wiki revision/digest, graph epoch and watermark fences for read projections | Old/out-of-order/revoked results never repaint; same-revision conflict denied |
| **AW-006** | P0 | SCOPED | AW-003 | Crosswalk VLANG/SCF verifier receipts to R003 claim verification state and source evidence | Indexed occurrence not mistaken for independently verified capability |
| **AW-007** | P0 | SCOPED | AW-006 | Build claim-to-English fidelity matrix for existing R003 narrative.rs: negation, tense, units, actors, unknowns and disputes | Each factual sentence traceable; no false behavioral or temporal claims |
| **AW-008** | P1 | SCOPED | AW-003,AW-007 | Prepare canonical seven-hub page, link and truthful-empty-state contracts using existing Wiki graph | All seven hubs reachable, stable source-linked sections and no invented copy |
| **AW-009** | P1 | SCOPED | AW-005,AW-008 | Use existing context_lens.rs for ambient/selected/pinned pages and Overview/Explain/Trace depth | Human and agent share one read path; stale previews superseded, typing read-only |
| **AW-010** | P0 | SCOPED | AW-004,AW-005 | Identify exact existing owner-scoped Host read port and transport for Wiki/HQKG projection | No public owner-private index; trusted scope and revision carried end-to-end |
| **AW-011** | P1 | SCOPED | AW-006,AW-007 | Prepare reviewed quality-feedback candidate ingress to existing VLANG and task writer | Earliest-stage typed diagnostic; no autonomous self-edit or shadow queue |
| **AW-012** | P1 | SCOPED | AW-010 | Map Wiki commands and NO_MATCH behavior to existing Host action registry and Rust control grammar | Read-only previews inert, unregistered actions refused; effects still Host-approved |
| **AW-013** | P0 | SCOPED | AW-004,AW-005,AW-010 | Write negative test specification for redacted links, private graph neighbors, raw drafts, revoked source, developer-only trace | No sensitive information in previews, snippets, titles, logs or trace |
| **AW-014** | P1 | SCOPED | AW-001,AW-004 | Bound HQKG projection by existing scoped query depth, tokens, item cap and completeness | No unbounded traversal, no hidden truncation, measured performance if executable |
| **AW-015** | P0 | SCOPED | AW-001,AW-002,AW-003,AW-004,AW-010,AW-013 | Prepare one Rust-first R003 Dev handoff: source pins, tests, insertion points, rollback and collision checks | Labs main receipt; OFFERED_TO only until actual Dev ACK |
| **AW-016** | P1 | SCOPED | AW-015 | Specify PWA/GPUI semantic parity contract against Dev's existing human/agent typed input when available | Same verified projection/source revision or explicit unavailable status; GPUI not assumed implemented |

### PWA development pipeline

| ID | Priority | State | Dependencies | Preparation / scaffold deliverable | Acceptance evidence |
|---|---|---|---|---|---|
| **PY-001** | P1 | SCOPED | PL-004 | Design explicitly submitted PWA build/change intent from existing composer using Rust admission rather than ambient Wiki lookup | Owner deliberate action only; no partial draft/IME interpreted as task |
| **PY-002** | P1 | BLOCKED_SOURCE | PL-001,PY-001 | Locate existing canonical durable work request writer; do not introduce a replacement database | One request ID and exact durable source accessible to PWA/Planning |
| **PY-003** | P1 | SCOPED | PY-001,PY-002 | Specify idempotency, deny, cancel, conflicting edits and offline replay using Host effect authority | Duplicate PWA submissions and reconnects produce one accepted outcome |
| **PY-004** | P1 | SCOPED | PY-002,PY-003 | Specify PWA status readback through existing Rust LiveSystemState and Host read ports | Same durable request ID; honest pending/blocked/failed/verified state |
| **PY-005** | P1 | SCOPED | PL-004,PY-002 | Research supported ChatGPT coding agent dispatch/ack protocol and permission boundary | Real supported dispatch or explicit MANUAL; GitHub file is not an active bus |
| **PY-006** | P1 | SCOPED | PL-002,PY-005 | Map atomic Planning OFFERED_TO Dev ACCEPTED_BY ownership with task claim file | One executor and source-based handoff receipt with refusal/retry states |
| **PY-007** | P1 | SCOPED | PY-004,PY-006 | Define verified build/test/package/serve/HTTPS/visible result/rollback receipt chain | Same stable URL plus exact pushed SHA, test, runtime and UI proof |
| **PY-008** | P2 | SCOPED | PY-007 | Define measured manual intervention, deterministic Rust reuse and AI-call efficiency counters | Report only real receipts and known costs; no invented baseline |

### Native GPUI

| ID | Priority | State | Dependencies | Preparation / scaffold deliverable | Acceptance evidence |
|---|---|---|---|---|---|
| **GP-001** | P2 | PAUSED_OWNER | PL-003 | Stage version-accurate GPUI input change observation, IME handling and latest-wins Wiki search in Labs only | Read-only native update; no Search/Send; no ARBOR edit |
| **GP-002** | P2 | PAUSED_OWNER | GP-001 | Map native composer RegionKind::Input and authorized agent ACK to existing Host control path; not phone.composer impersonation | Native source revision readback and human preemption with Host authorization |
| **GP-003** | P2 | PAUSED_OWNER | GP-002 | Prepare actual Windows GPUI visual test and PWA parity evidence, distinct from optional Android | Source-backed native observed output not a mock or HTTP 200 |

### Verification

| ID | Priority | State | Dependencies | Preparation / scaffold deliverable | Acceptance evidence |
|---|---|---|---|---|---|
| **VF-001** | P0 | SCOPED | PL-003 | Define separate SOURCE, TEST, PACKAGE, SERVE, AUTH_ACK, HTTPS, GPUI, PWA and PHONE evidence gates | Each gate PASS/FAIL/UNRUN with exact source, exit and observed artifact |
| **VF-002** | P1 | SCOPED | VF-001 | Scaffold screenshot/visual readback correlation to source SHA, session, Wiki revision and Host receipt | Actual observed frame and revision, or clearly UNRUN; private data redacted |
| **VF-003** | P1 | SCOPED | VF-001,PY-007 | Prepare same-URL PWA release rollback plan, native binary resolver checks and correct Origin/CSRF probes | One reversible source-pinned release, dirty checkout preserved |
| **VF-004** | P1 | SCOPED | VF-001,AW-013 | Author negative matrix: revoked consent, wrong actor, stale epoch, IME, human-agent race, no Send, redaction | Fixture-safe test intent with source-owner denial receipt |
| **VF-005** | P1 | SCOPED | VF-002 | Reuse bounded first-contact observability events for source-linked diagnostic review, not a new journal | No raw prompt/draft/credential log; diagnostic not effect proof |

## External dependencies: NOT in Planning Next Ten

| Deliverable | Owning executor | Existing code/evidence | Planning observes |
|---|---|---|---|
| Human typing -> first-contact AutoWiki | ChatGPT Dev | `apps/mobile-relay/app.js`; Rust `wiki_read.rs`; first-contact 180ms search reports | New source/tests/real browser proof, not Planning's product edit |
| Authorized agent typing -> same AutoWiki | ChatGPT Dev | `first-contact-composer.ts`, `agent-composer-adapter.ts`, Host `agent-composer-bridge.mjs`, Rust `agent_composer.rs` | Current approved Host ACK and visible source; no copy of composer |
| Apply R003 to ARBOR | ChatGPT Dev *after explicit acceptance* | Labs staged `DESIGN/AUTOWIKI-FOUNDATION-R001` | Offer source-pinned, verified Rust package; no implied recipient ACK |
| Release/runtime and optional real phone verification | ChatGPT Dev and separately approved local runtime executor | Existing First Customer Tracker and Dev release playbook | Source/build/deployment/device evidence classified separately |

## Best first work packet

**AW-R003-READ-READINESS:** in small separately claimed steps, do PL-003 -> AW-001/AW-002 -> AW-003/AW-004 -> AW-005/AW-010/AW-013. The first user-visible goal of this packet is a **source-scoped, revision-fenced, read-only context projection contract** ready for Dev, not a speculative full Wiki engine. Then handle AW-006/007/008/009 for verified English, seven hubs and ambient lens. Finish AW-015 for handoff.

## Claim and update policy

Read [shared task claims](../TASK-CLAIMS.md) before touching any files. Check out one cohesive Planning packet with file scope, source pin, UTC time and planned acceptance. Use GitHub contents SHA compare-and-swap. Status in this document is candidate scope only and does not imply the claim. Check in actual test command/exit, prepared files and commit hash; sender records OFFERED_TO:CHATGPT-DEV, receiver records ACCEPTED_BY before execution ownership changes. Read [source reuse map](SOURCE-REUSE-MAP-R001.md), [verification scaffold](ACCEPTANCE-AND-HANDOFF-R001.md) and [tracker reconciliation](TRACKER-RECONCILIATION-R001.md). Missing canonical OWNER-WORK means no new OW ID, no invented history and no automatic promotion to executable owner work.

## Research anchors

- Labs R003: `../DESIGN/AUTOWIKI-FOUNDATION-R001/R003-EXECUTION-HANDOFF.md`, `src/lib.rs` and five sibling modules, feature-gated HQKG adapter.
- ARBOR current: `core/programmability/src/{wiki_read,live_system,agent_composer}.rs`; `native/hqkg/src/context_retrieval.rs`; Host control read ports; mobile relay composer.
- Product tracker: [ARBOR first-customer tracker](https://github.com/owenservera/ARBOR/blob/main/docs/FIRST-CUSTOMER-TRACKER.md).
- Oracle [steering signal](../ORACLE/TO-PLANNING.md).
