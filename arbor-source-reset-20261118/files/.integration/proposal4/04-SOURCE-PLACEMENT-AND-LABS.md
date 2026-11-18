# Proposal 4 — Source Placement, Transformations and Labs Graduation

**Purpose:** concretely explain what moves, what stays, what becomes an adapter, and what should not be rewritten during the first master integration.  
**Status:** preliminary source-backed architecture map; every runtime claim requires exact source SHA and tests during execution.

## 1. Disposition vocabulary

- **BIND:** keep the existing implementation and add a typed source/operation port into ARBOR.
- **PROMOTE:** reuse an existing tested engine/library as a canonical product dependency.
- **ADAPT:** refactor boundaries and protocol shape but preserve the implementation and behavior.
- **CONVERGE:** consolidate multiple representations or routes into a single contract, with differential tests before retiring any old owner.
- **REPLACE-LATER:** keep current behavior now; eventually change language/runtime only if needed, with shadow comparison.
- **GRADUATE:** turn a Labs concept or prototype into its proper product module after reviewed acceptance.
- **RETAIN-EXTERNAL:** keep useful independent upstream development, use pinned build-time/read port.
- **ARCHIVE:** document, stop shipping or integrating after proven consumers are moved.
- **REVIEW:** no sound status or authority evidence yet; do not import blindly.

## 2. Canonical product component placement

| Existing source | ARBOR system module / backbone port | First-integration disposition and transformation |
| --- | --- | --- |
| Harness `apps/vivim-host/server.mjs` | `bridges/host-control`, `kernel/authority-port`, `surfaces/desktop` | **BIND + ADAPT.** Keep current Host serving authoritative World/plan/effect data; unify authenticated read/submit and projection contracts, avoid a mandatory JS parser hop for all native semantic events. No immediate wholesale Rust rewrite. |
| Harness `control/action-catalog.v1.json` and `dispatcher.mjs` | `modules/interaction`, `modules/diagnostics`, `kernel/authority-port` | **BIND.** The Host catalog is the source owner of actually admitted effect/action IDs. Derive public capability descriptors from it; never create a second action catalog with different risk or readiness labels. |
| Harness `control/journal.mjs`, recovery and receipts | `kernel/authority-port`, `bridges/host-control`, `tests/authority` | **RETAIN FIRST.** This remains sole effect writer while ARBOR integrates. Rust may shadow-read and compare; migrating the writer is a separate later one-writer release gate, not a first-integration prerequisite. |
| Harness `control/intent-planner.mjs`, Omega/NLCL / command validator | `modules/interaction/bindings`, `kernel/composition` | **CONVERGE.** Bind existing semantic frames to one source-anchored IR, preserve negation/quote/risk via differential fixtures. Do not replace with Rust prefix matching until parity is proven. |
| Harness `core/programmability/` | `kernel/model` + `kernel/composition` | **PROMOTE PURE LOGIC.** Reuse safe typed transformation validators and output contracts; align identities/version handling, avoid competing schemas. |
| Harness R001 Rust `user_io.rs` draft PR | `modules/interaction`, `kernel/runtime`, tests | **GRADUATE SPEC/TESTS; REPLACE-LATER concurrency.** Its two-Tab and refusal fixtures are helpful; its mutex-held per-keystroke threaded architecture is not target production implementation. |
| Harness `apps/vivim-host/direct-phone.mjs`, pairing controls | `modules/device-pairing`, `surfaces/web/transports`, Host bridge | **BIND + ADAPT.** Existing owner/device trust and signed sessions remain, then converge into exact native actor-bound protocol after acceptance. No second paired phone product. |
| Harness `apps/mobile-relay/`, Host public UI | `surfaces/web`, `modules/conversations`, `modules/interaction` | **CONVERGE** only one responsive presentation/application identity. Retain tested UX/screens and correct transport controls, retire duplicated semantic state and action catalogs when compared. |
| Harness `studio/src/`, `harness/` | `surfaces/browser-worker`, `bridges/studio-browser`, `modules/conversations` | **BIND + ISOLATE.** Signed-in Chrome/CDP adapter, exact thread/send-once/capture and uncertain effects. No provider APIs, no Playwright/Puppeteer. |
| Harness media library/byte-range stream/cache | `modules/media` + existing Host capability port | **BIND.** Preserve secure path, auth, 206/416/range/caching contracts. Do not rewrite streaming to claim architectural purity. |
| Harness onboarding + Studio setup | `modules/onboarding` + `modules/knowledge` | **BIND + COMPOSE.** Existing Chrome session and source access become typed first-run program nodes with source/consent proofs. |
| HQ `research/rust-knowledge-graph` `engine.rs` | `kernel/graph` | **PROMOTE.** Reuse `ComputeEngine`, generation pinning, scheduler primitives; one canonical computation library with test and release agreement. Do not create a second graph engine. |
| HQ `typed_dag.rs`, `scoped_graph.rs`, `context_retrieval.rs` | `kernel/graph` + `kernel/composition` | **PROMOTE/ADAPT** trusted pure DAG with authenticated source revisions and scope, preserving Development/Personal namespaces and no effect authority. |
| HQ `hq/lib/semantic-registry.mjs`, action/index census | `bridges/hq-development`, `tools/intake`, `kernel/model` | **BIND** stable source semantic identity and version crosswalk, but do not turn HQ's developer registry into a second product action/permission writer. |
| HQ incremental indexer, KB and wiki | `bridges/hq-development`, `modules/knowledge`, `modules/workspaces` | **RETAIN-EXTERNAL.** Development intelligence stays in HQ. Product's personal AutoWiki uses compatible graph/view recipes over owner World. Source-driven projects can be read with proper scope. |
| HQ wiki-program/comment bridge | `modules/knowledge`, `modules/diagnostics`, `kernel/composition` | **BIND/ADAPT** versioned pure wiki config program patterns; only owner-approved config writes through canonical port; no new wiki database. |
| HQ interpretation surface census (43 facets, 31 selected source producers) | `modules/interaction`, `modules/observability`, `tests/contracts` | **BIND AUDIT INPUT**, not production coverage. Generated runtime manifest must report actual ran/skipped/timed-out and exact source bindings. |
| TierFlow `src/visual-runtime`, `src/ui` | `surfaces/web/visual-runtime`, module `views/` descriptors | **RETAIN-EXTERNAL + ADAPT.** Pin built renderer and typed command schemas. Use TierFlow as projection/edit experience, never a second effect authority or independent product app. |
| TierFlow optional `core` Rust and independent server | `kernel/graph` review / TierFlow upstream | **REVIEW** unique algorithmic gains; no duplicate shipping Rust scheduler, visual server or .NET shell by default. |
| Existing `owenservera/vivim` Cosmic Desktop | `surfaces/web` selected view assets only | **REVIEW, NO WHOLESALE IMPORT.** Browser singleton kernel, direct model-provider ideas and mock local DB are not the system truth; salvage only useful source/license-safe UI after comparison. |
| Existing `owenservera/vivim-platform` | migration inventory only | **REVIEW.** Its multi-service/server/API-first code is a historical donor, not proof those processes are needed in ARBOR. Keep old dependencies/secrets out of the new Git history. |

## 3. Labs work graduation — user intent vs release proof

| Labs domain/source | Declared status | First ARBOR landing | Real transformation |
| --- | --- | --- | --- |
| `user-io-rust-core/node-0-prime/` | Architecture direction; native spike in draft PR, not compiled | `modules/interaction`, `kernel/runtime`, `tests/authority` | Extract canonical session/evidence/effect separation; implement bounded event fan-out, typed source stamps and multi-subscriber trace; reject mutex/thread-per-event as production strategy |
| `nlp-command-ui-lab/` R004/R005/R006/R007 | R005 source experiment and synthetic tests; R006 prediction concepts; R007 VIEW ALL proposed | `modules/interaction/views`, `surfaces/web/composer` | Use display semantics, source anchors, prediction slots, layers and geometry over the one core trace; UI does not invent authorization |
| `personal-knowledge-lifecycle/` including runtime | Explicit DESIGN/RESEARCH scaffold with source code requiring audit | `modules/onboarding`, `modules/knowledge`, `modules/context` | Connect ChatGPT-first capture, evidence lineage, candidate extraction and opt-in World memory lifecycle; keep consent/disclosure separate |
| `Architecture/` one Rust graph / command/wiki | Conceptual target with mixed source maturity | `kernel/graph`, `kernel/composition`, `modules/knowledge` | Convert accepted architecture to typed ports and tests, not duplicate graph store or HQ runtime |
| `vivim/voice-to-tasks/` | Owner Day-0 goal; source verification pending | `modules/voice-to-tasks`, `modules/interaction` | Treat voice as typed input and task as separately accepted World effect; no voice-specific action owner |
| `chronos-toolkit/` | Research/staging; native execution verification required | `modules/scheduling` | Bind deterministic time/schedule transforms into same graph; Host remains effect/notification owner |
| `capability-observatory/` | Seed-only tracker; no full production verification | `tools/intake`, `modules/diagnostics`, `tests/contracts` | Reuse source/proof vocabulary for derived capability inventory and runtime introspection; no second product registry |
| `intelligence/algorithms/`, `engines/` | Isolated research | `modules/personalization`, `kernel/graph` | Promote only benchmarked pure algorithms, not wholesale independent intelligence service |
| `vivim/PLUGIN-SYSTEM-ARCHITECTURE.md`, `extension-points/` | Candidate architecture; 153 possible surfaces not 153 verified hooks | `modules/extensions`, `kernel/composition` | Reuse verified extension port concepts and Lab security taxonomy; defer generic marketplace/loader |
| Labs Dynamic Context Bundles, memory, context, Zero Intent Graph | User-established strategic context; mixed implementation | `modules/context`, `modules/knowledge`, `modules/personalization` | DCB as versioned read/selection program with source/permission proofs; graph computation shared; personal knowledge World-owned |
| Labs AutoWiki and storage-bound Rock Core | Desired product behavior; code/run proof incomplete | `modules/knowledge` / `modules/workspaces` views; `configuration/resource-budgets` | Per-screen source-derived wiki from the same accepted graph, bounded hot/warm/cold caches; no parallel wiki store |
| Labs missions, self-healing, automation/agent programs | User-established intended UX, mixed research | `modules/missions`, `modules/automation` | Reuse existing Host mission state and effect journal, compose agents/pure recovery logic; no independent autopilot grant |
| Labs app-builder / compositional UI concept | Owner direction, not verified product feature | `modules/app-builder` plus `surfaces/web` | Visual authoring of the *same typed program graph* and user-approved configuration, not a new app execution engine |
| Labs media, performance, evidentiary corpora | Mixed tests/research | `modules/media`, `tests/performance` | Keep only license-safe/source-verified fixtures, authentic media contract tests; never dump personal media into Git |
| Labs Z.ai R001/R002 bundles | Past assumptions need exact branch/merge proof | destination modules **only after audit** | Map each tested source patch by owner and source SHA; no blanket assumption they are already integrated |

**Ratification constraint:** a concept the user selected can be an architectural acceptance criterion while still having `DESIGN` implementation status. Source-present modules are not automatically verified in Windows or phone. Record these separately in a reproducible migration ledger.

## 4. Structural transformations — what actually changes

### M01. One capability vocabulary replaces the frontend/backend contract split

Old: `frontend performs button behavior` + `backend endpoint` + `separate agent skill`.  
New: one `CapabilityRef` with source-owner ID, typed ports, risk/scope, versioned implementation binding, view/projection and test obligations.

**Compatibility:** bind old API/JS functions behind port adapters; don't reimplement behavior until test parity.

### M02. One semantic interpretation frame across natural language and visual controls

Old: independent Host NLCL, mobile shortcuts, TierFlow inert typed command, Labs semantic lens.  
New: a source-anchored multi-facet interaction frame feeding one typed program compiler with current source references. Provisional visual hints can be local; submitted semantics must be source-consistent.

**Compatibility:** golden tests and shadow comparison. Existing Host planner stays authoritative for current actions during migration, not automatically replaced by the Rust R001 prefix recognizer.

### M03. One compute engine, existing graph identity

Old: HQKG research + native validator + TierFlow Rust hot path.  
New: one versioned HQKG-derived native graph/typed DAG/scheduler, adapters for current engines, no second graph store and no implicit effect execution.

**Compatibility:** source reuse, engine generation pinning, namespace/authority rules, performance tests. No automatic copying of private development index into World.

### M04. One UI reality, multiple rendering projections

Old: Host public UI, mobile relay UI, TierFlow standalone view, Cosmic Desktop history and Labs visual experiments.  
New: one responsive shell and composable views over the same semantic/source model. Desktop/PWA differ in transport and device affordances only.

**Compatibility:** stage read-only UI projection first, preserve existing working screens until acceptance.

### M05. One configuration/change contract

Old: UI preferences, Host rules, feature switches and wiki config in different shapes.  
New: actor-scoped, source-owned config proposals with revision CAS and preview/diff. No settings YAML can grant authority.

**Compatibility:** migrate values through explicit namespaced settings crosswalk; preserve user choices, do not overwrite with defaults.

### M06. One effect-authority gate, even during integration

Old: Host JS grants/dispatcher/journal remain canonical.  
New: everything compiles proposed effects into that one Host gate and receipt stream. Native Rust may own the eventual implementation, but one source writer at a time.

**Compatibility:** exact journal/claim parity and crash recovery; no dual-writer phase. The first integration does **not** require rewriting all Host effects into Rust.

### M07. One source-driven wiki and diagnosis surface

Old: HQ generated wiki and source intelligence, UI concept AutoWiki, per-screen contextual discussions.  
New: same graph/source/capability identity drives user AutoWiki, project inspector, program explanation, VIEW ALL and diagnostics. No separately edited personal wiki "truth".

**Compatibility:** HQ remains dev-intelligence read authority; personal source materializations come from Host World, with retention quotas.

### M08. One consumer product, without mandatory extra processes

Old: multiple local app launchers, sidecars, JS servers and test-only shells.  
New: minimal necessary processes: resident native Host/Tauri, one isolated signed-in Chrome browser worker, paired PWA client. During the first adapter phase, the existing Node Host remains as **sole effect/data authority**, and its bridge is explicit rather than hidden.

**Compatibility:** remove an old process only after its consumers and state transitions are moved and tested; no speculative forced retirement.

## 5. Required migration ledger

For each source port or Lab component, record:

```yaml
source_repo:
source_branch_sha:
source_path:
source_symbol_or_contract:
source_owner:
implementation_status: # DESIGN | SOURCE_PRESENT | TESTED | WIRED | ACCEPTED
runtime_consumers:
source_data_class:
effect_authority:
proposed_module_id:
target_ports:
disposition: # BIND | PROMOTE | ADAPT | CONVERGE | REPLACE-LATER | ...
compatibility_adapter:
golden_tests:
security_tests:
rollout_gate:
retirement_condition:
evidence_url:
owner_decision:
```

No guesses about source authorizations or Test/GO. A capability descriptor is only allowed to say `available_now` after current source-owner evidence.

## 6. Confidential source protection

ARBOR is currently private, but this is not license to commit user conversations, Chrome profiles, provider session tokens, saved credentials, unredacted memory or old Git history. Source references/fixtures must be synthetic or explicitly sanitized. Build-outputs and dependencies remain ignored. Migration from old private `vivim` and `vivim-platform` is *selected source copying with audit*, not entire history import.

## 7. What Proposal 4 deliberately leaves external

- HQ's development-indexing authority and generated dev docs.
- TierFlow's upstream generic renderer development.
- Labs experimentation, capability tracking and archive.
- Provider websites and their signed-in browser user sessions (Host controls permitted access; does not own providers).
- Any separately authorized personal cloud sync research not accepted as Day-0 product architecture.

These are **not separate definitions of VIVIM capability semantics**. They integrate through typed source ports and a shared observable system model when current permissions allow it.
