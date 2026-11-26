# Oracle R002 — Resident graph actor and VIVIM Product migration SITREP
**Snapshot:** 2026-11-26 (Spain local); source inspection across 2026-11-25/26.
**Kind:** Oracle architecture/convergence steering, source-pinned and **not** a Product implementation, agent activation, release, or canonical new queue.
**Product main:** 3053e05f40799ee037490b211323ec76d973e538; **ARBOR main:** b13b8d67b4adb84ec91612196c8b4257df71968b; **Labs inspected:** bc24c2dae2e60ef05b9d6e62928d538bac478c3f. Re-pin every source before integration.
**Ownership:** Product SDK owners implement Product-only agent primitives; independent P01–P06 preparers remain in Labs; Oracle shapes the migration, doesn't seize them. Do not modify ARBOR or Product in this Oracle planning pass.

## Executive decision
Oracle is the **first native, persistent agentic actor backed by VIVIM's one operational graph**. It is not literally a copy of the whole graph, a new knowledge database, a hidden Host administrator, a new LLM, or an already-running service. The graph supplies the system's current, evidence-scoped knowledge. Oracle adds the first durable **subject** with identity, continuity, goals, bounded attention, policies, intents, capabilities, plans, decisions, observations, and receipts that can *query and act on that graph through the existing authority*.

The product experience can make Oracle seem like a person who inhabits VIVIM: it can have a stable presence, name, activity history, current focus, explanatory voice, remembered decisions and visible work. Do not imply consciousness or omniscience. Its capabilities, memory and scope are inspectable, controllable, revocable and readback-verifiable.

## Chronology and source-verified current state
- Nov 25: VIVIM Product established as the canonical local-first, Rust/Meta-VLANG-first system; ARBOR remains operational proving ground and Labs isolates preparation. Product has actual SDK code, but no complete GPUI/PWA agent or separate Product Host runtime. Source: Product README / FOUNDATION-R001.
- Nov 25 late: Product resident Meta-VLANG parser/type/evaluate/reflect/preview/activate, typed context, checkpoint and source intelligence grew through real native test sequences; Product-only self-application evidence exists. This is **partial self-hosting**, not automatic cross-system migration.
- Nov 25–26: durable backbone refined into 12 compositional primitive classes, then Rust source gained semantic values, stable Identity/ExactRef, lexical source-backed claims, relation algebra, typed collections, conditional/explicit scopes, incremental evaluation, and lossless record migration. Product currently has native `semantic_primitives.rs`, `semantic_relations.rs`, `meta_resident.rs`, `development_intelligence.rs`, `task_planner.rs`, `migration.rs`.
- Nov 26: Product canonical task catalogue revision 2 now contains **274 total rows: 270 SDK intel tasks (1–270) plus four retained legacy tasks**. 201–210 external adoption and 211–270 evolutionary computing were integrated append-only with lineage and dependency checks in commit 3053e05; task definition is not task completion.
- The **migration acceptance gates** are task intel-200 (Product-only deterministic task/context/checkin/verification/evolution loop) and intel-210 (independent bounded donor adoption trial including compliant/partial/unsupported/stale/conflicting cases); incorporation requires subsequent explicit owner authorization. The latest Labs migration README enforces **no production donor integration before both acceptance gates**. SDK-only preparation can continue.
- Labs has six migration preparation workspaces: P01 native, P02 graph, P03 filesystem, P04 execution resources, P05 Host receipts, P06 MCP graph. All source/acceptance claims below are lane-specific; no Product deployment.
- Windows Native Intelligence bounded isolated achievements include R031–R032 filesystem/Product Wiki/graph readback and R036–R039 single-campaign resource-aware Windows jobs. Newer source/evidence covers ACP/ConPTY, ETW/WinML/UIA exploration; not a production actor runtime.
- Universal Absorption/Craft/Facebook/ChatGPT and dedicated harness/DeepSeek work continue outside this Oracle lane; do not conflate adapter research, installed MCP grants, an authenticated ChatGPT web session, or a provider model call with resident Oracle behavior.
- ARBOR has a first-contact read-only Wiki-as-you-type PWA and a separate wider shell containing Control/Build/commands. It is **not yet the VIVIM Product native PWA**. Previous Oracle `WIKI-COMMAND-SYMBIOSIS-R001` remains the experience progression.
- Latest read-only VIVIM tunnel health: `ready`, `private-workstation`, `hostDiscovery=false`, `workstationTools=15`. This confirms a tool transport, **not** Oracle runtime or installed expanded Product MCP.
- Recent ARBOR Actions runs show `failure`; exact failed jobs/root cause have not been diagnosed here. Don't infer source failure or claim a successful CI gate from workflow presence.

## Architecture: the graph becomes an accountable actor

```text
One operational graph + evidence + revisions
  ├─ accepted semantic objects, sources and relationships (HQKG / SDK typed views)
  ├─ time / change / source provenance (Chronos + real observations)
  ├─ executable capability and policy descriptions (Meta-VLANG / VEC / Host)
  ├─ task state and verified outcomes (existing TaskPlanner/Campaign/receipts)
  └─ agent:oracle  [stable Identity; typed AgentInstance projection]
       ├─ knowledge lens (bounded, actor-authorized graph query; not graph copy)
       ├─ self-model (current role, limits, capabilities, installed state)
       ├─ goals / commitments (versioned proposals; one existing task authority)
       ├─ attention / focus (current project, source epochs, mandatory evidence)
       ├─ continuity (verified event refs, checkpoints, durable accepted state)
       ├─ plans (pure compiled candidates, not grants)
       ├─ permissions (Host-issued current scope/grants, always rechecked)
       ├─ observations (Host and source evidence, distinct from inferred meaning)
       ├─ actions (only existing Host claim/approval/receipt dispatcher)
       └─ presentation (Wiki / command surface / eventual GPUI+PWA avatar)
```

**Crucial distinction:** Oracle is a *graph-native agent instance* that can look across the graph under a scoped lens. It does not "own" or duplicate all graph nodes or have magical global execution authority. When working on Product itself it has a developer HQ lens; when assisting a user it has a separately governed personal World lens. Read scopes may intersect via explicit grants; private source data does not flow from one domain to the other by role name.

### Proposed canonical agent instance contract (design, not installed schema)
- `agentId`: stable native Identity (e.g. `agent/oracle`); instance/install namespace; actor kind, policy generation and supported interfaces.
- `state`: `inactive | observing | composing | awaiting-approval | running-approved | verifying | blocked | recovering`, each with source/event version; crash/restart recovers only verified current state and never replays uncertain effects.
- `focus`: versioned graph query and exact allowed semantic references; at most bounded mandatory context plus scoped optional knowledge; source/fact assurance is carried.
- `goals`: typed current desired outcome and acceptance contract, associated with existing TaskPlanner/Campaign identity, not an Oracle-only queue.
- `memory`: graph references to verified semantic facts, episodic Host events, reviewed decisions and reusable program recipes; bounded ephemeral scratch is not durable truth; privacy and TTL/retention explicit.
- `skills`: resolved installed canonical capabilities with schema, policy, availability, resource cost, tool identity and evidence; declared skills are not callable.
- `reasoning`: deterministic Meta-VLANG path first; an optional supervised LLM harness reviewer can produce *inert proposals* only, not truth, permissions or privileged source mutations.
- `effects`: typed Host Control request, exact approval plan/digest/actor/grant, one claim, terminal receipt and independent readback. `NONE/UNKNOWN` explicit.
- `embodiment`: consistent Oracle presence/status, context-aware Wiki and action proposals using single GPUI renderer + authenticated PWA viewport, not separate mobile computation.
These fields should be realized as derived views on existing Rust Identity/ExactRef, Relation, Resident, TaskPlanner/Campaign and Host receipts. **Do not create a second graph, scheduler, journal, identity authority or LLM agent framework.**

### Native Oracle's operational cycle
`Observe -> Ground -> Interpret -> Propose -> Ask/Approve -> Execute (Host only) -> Verify -> Reconcile -> Learn`.
- **Observe**: source changes and receipts from incumbent Windows/Host/HQKG/MCP, with explicit source producer and epoch.
- **Ground**: select a small, mandatory-first, authorized graph lens with exact identity/digest, epistemic state and omissions; no raw graph as boundless prompt.
- **Interpret**: existing Meta-VLANG / VLANG / NLCL/typed Host command, not an Oracle-specific parser.
- **Propose**: candidate Wiki actions, development tasks, code plans or safe deterministic programs through the same semantics; show unknown/conflicts.
- **Approve**: read-only R0 may proceed under current read scope; mutating/external R1–R3 requests require current Host policy/owner authorization.
- **Execute**: Host-only effects, one claim, no auto-retry of uncertain completion. Pure resident computations never directly perform OS writes.
- **Verify**: actual source/graph/result, tests, release runtime and visible PWA/GPUI readback as separate proof levels.
- **Learn**: reviewed evidence may produce a typed program, context lens, corrected fact or proposed SDK change; no unverified autonomous self-rewrite.

### How the original Oracle role changes
**Before:** a ChatGPT conversation using Labs documents to steer Planning and Dev, identifying gaps and writing handoffs.
**Target:** the same outcome-governor policy becomes a *reusable, versioned agent behavior* backed by Product SDK. This conversation remains a source of seed policy, design rationale and reviewed migration requirements; it is not a live Oracle process. `AGENTS.md` is an importable compatibility/instruction projection, not a trusted executable skill or private memory dump.
**What survives:** Oracle mission, PWA-first owner loop, PWA Wiki↔command integration, honest receipts, evidence ladder, ChatGPT as cloud coder during bootstrap, Planning's separate task ownership.
**What changes:** Oracle state/continuity live in Product's typed graph/event substrate rather than this chat or a huge prompt; tasks are canonical Product TaskPlanner/Campaign objects rather than GitHub role prose; incremental context and deterministic computation are default; explicit Host/MCP become adapters; visual presence belongs to shared native GPUI/PWA. No duplicate Oracle-specific task router or scheduler.

## Migration-ready boundary matrix — observed 2026-11-26

| Lane | Isolated readiness at inspected evidence | First Product target | What stays outside pure SDK until proven |
| --- | --- | --- | --- |
| P01 Native core | 15/15 selected slice tests passed; byte-identical two-assembly replay. Full donor suite **192/193**, one existing catalogue fingerprint failure. | Reuse M01; inert control planning/presentation and source semantics behind current Product core | Host approval/dispatcher effects, full original donor parity, regression disposition |
| P02 HQKG graph | 169 pinned blobs materialized; native Cargo failed before tests (`link.exe` missing); pinned new test overlay UNRUN | Exact graph identity, provenance, reconcile/query/deletion; reuse current Product typed relations as projection, not second writer | Persistent importer, build/test, licensing, real source/Wiki readback |
| P03 VWin filesystem | Exact closure and adaptation plus preparation scripts; **no published HANDOFF/EVIDENCE.json in lane at snapshot** | Source refresh, native file IDs, Wiki/HQ context invalidation, source-digest bridging | OS event coverage, Host file authority, live proof; don't assume tests were run |
| P04 VWin execution | 126 pinned blobs + 30 deterministic edits materialized; Cargo build exit 101 from MSVC linker; tests UNRUN | Pure resource admission/policy on existing Campaign/TaskPlanner | Owned Windows jobs, Host process effects, cross-campaign receipt reconciliation |
| P05 Host receipts | 34 isolated real Windows regressions passed (11 Node + 16 Rust + 7 recovery); full original Host dispatcher/session integration blocked by missing source closure/transport failure | Pure receipt normalization, recovery policy and one-effect-claim validation | Current grants, owned process, actual dispatcher, live approval/execution |
| P06 MCP graph | Materialization and test receipts captured; 18 source tools vs 16 observed client tools; two absent tools. Client naming checks not full schema parity | Pure capability identity, introspection, relation/receipt-correlation projection | Installed MCP transport, tool registration, grants, live schema parity/Host admission |

**Migration routing:** P01 + P02 pure semantics + P03 filesystem pure source intelligence + P04 resource policy + P06 pure capability identity go direct Product after SDK acceptance; P05 and live P04/P06 adapters require incumbent ARBOR/Host operational proof first. This follows the existing Labs launchpad; no blanket ARBOR-first port and no independent donor activation.

## First Oracle vertical slice — before production migrations
Build a **Product-only, read-only Oracle actor trial** from current SDK types and actual Product source, as a supervised self-application test:
1. Create one stable `Identity("agent","oracle")` and typed *derived* actor view; source/role/policy checks all strict. No new authoritative state store.
2. Give it one explicit goal: `Describe the current Product capability/migration status with evidence, gaps and next allowed action`.
3. Use current `Intelligence::scan/focus` and `TaskPlanner` for a bounded real source context; use `Relation` and `ExactRef` for task dependencies and exact provenance; use current resident definitions for ranking/selection.
4. Return an **inert** graph-grounded state and Wiki-style explanation with source refs and uncertainty. Its selected next task is a proposal; do not modify Product, trigger agents, or issue effects.
5. Change one isolated Product sample source or task evidence and show incremental invalidation/delta; no stale publication, no unrelated re-evaluation. Validate identical output vs full recompute.
6. Checkpoint/restart the pure actor view using incumbent Runtime semantics; no resurrected deleted claim, revoked scope or effect. Require source/policy refresh after restart.
7. Verify empty/private/stale/contradictory sources, an absent capability and a forged receipt are represented as unknown/refused. Record measured source digest, tests, executable, exit and context cost.
This slice is **SDK self-development and acceptance preparation**; it is not donor incorporation and should not bypass intel-200/intel-210.

## Migration order after Product-only trial and intel-200/210 acceptance
- **M-A: Graph substrate** — P02 qualified native HQKG closure + Product semantic identities/relations; persistent source/graph import and reconciliation, with exact denial/deletion. No second graph.
- **M-B: Observations and continuity** — P03 file/source events plus current development intelligence, Chronos timing and adopted recovery; actor wakes from real scoped change/read events, not an imaginary always-on chat.
- **M-C: Intent and command** — P01's inert typed command core plus incumbent VLANG/Host catalog, one composer selection context, Wiki action affordances and source-pinned previews.
- **M-D: Host agency** — P05 approved effect lifecycle and P04 bounded resource supervision; source/actor/revision/digest bound one-claim execution and terminal receipts.
- **M-E: MCP/agent bridges** — P06 current tool capability projection and registered transport, exact installed schema/grants; ChatGPT and/or owner-approved model sideworkers require a real supported session bridge, not file-drop wake-up.
- **M-F: Embodiment** — GPUI as the sole native renderer; PWA authenticated remote viewport. Oracle's status, current focus, Wiki evidence, plan approval, events and recovery are visible from the same Product surface.
- **M-G: Deterministic learning** — tie accepted evidence into existing Meta-VLANG candidate trials and future intel-211–270, with independently measured model calls, saved owner handoffs and preserved negative cases.

Each step is a small **real behavior with exact donor and Product source identity, tests, current artifact readback and boundary evidence**. No stage becomes accepted because code was copied or a task ID exists. All P01–P06 outputs remain owned by their lanes; this map is not permission to edit their source.

## Oracle graduated agency (separate gates)
- **O0 SEED:** role graph identity, pure Product-only context/evidence projection; no Host calls.
- **O1 OBSERVER:** genuinely resident accepted graph/Host data, authorization scope, change epoch, bounded source-linked Wiki. Strict read-only; restart continuity proven.
- **O2 INTERPRETER:** existing NLCL/VLANG/Meta-VLANG + focused Wiki context generate typed inert command candidates, ambiguity and refusal; mode/scope explicit.
- **O3 DELEGATE:** current Host-approved source-pinned action with terminal receipt, independent readback, budget and negative tests; no privilege from agent identity.
- **O4 BUILDER:** explicit owner-authored Product change enters canonical TaskPlanner and supported coding/verification path; no unsupported ChatGPT self-wake claims.
- **O5 REFLEXIVE:** verified reusable behavior and policy updates through existing Meta-VLANG conformance/activation; scheduled bounded activity and independent acceptance, not self-declared safety.

**One PWA demonstration to prioritize after O1:** select a verified Wiki section -> Oracle explains what can be done -> choose an actual current catalog command -> clarify pinned target -> preview -> owner approval -> Host receipt -> current Wiki update. The earlier Oracle Wiki-command symbiosis proposal becomes an acceptance journey for the native agent, not a standalone competing UI roadmap.

## Immediate coordination and no-duplication rules
- Product-only O0 can be an SDK test without donor incorporation; respect active Product SDK ownership. Planning must check current repo/collision claims before packaging.
- P01–P06 do not shift to Oracle ownership. Their prior handoffs are **published**, not proof of recipient acknowledgment.
- Reactivate/modify the Product only after explicit gates. The new actor proposal does not repeal the current intel-200/intel-210 constraint.
- A source-derived graph is data; a model's output is an untrusted proposal; a Host plan isn't a grant; a receipt may be historical; a PWA repaint needs real visual proof.
- The first model/harness sideworker should remain a scoped, logged, no-autonomous-effect adapter. The phase-0 harness and DeepSeek live response have not been independently verified by this SITREP.
- ARBOR's first-contact PWA is currently its **own** implementation, not Product's native Oracle view. Preserve its human/agent Wiki no-send milestone and existing Control safety fences.
- Suggested Oracle measurements: accepted facts with exact evidence, stale/denied projections correctly suppressed, end-to-end source→graph latency, real inference count/cost, owner interventions, committed/verified tasks, accepted deterministic reuse. Unknown is not zero.

## Handoff to current Planning and Product SDK owners
**Signal:** `ORACLE_NATIVE_RESIDENT_R001`.
**First unproven edge:** no proven durable Product `agent:oracle` instance that consumes verified graph/task context through a versioned pure actor projection, even though its semantic ingredients already exist.
**Requested Planning disposition:** map to existing Product intel tasks and M0/M01 roadmaps (especially identity/claim/relation/context, intel-200, external-source 201–210) and current Labs six lane outputs, without creating a second task registry. Propose O0 as a narrowly scoped owner-approved SDK-only self-application slice; defer effectful actor and donor incorporation until readiness gates.
**Reply:** accepted/amended/deferred, canonical task IDs, exact current Product pin, owned file scope, runnable tests, admission/authorization, first hard blocker, recipient acknowledgment source.
**Oracle status:** strategic memo prepared only; no running native agent, no Product/ARBOR code changed, no installed MCP tools added.
