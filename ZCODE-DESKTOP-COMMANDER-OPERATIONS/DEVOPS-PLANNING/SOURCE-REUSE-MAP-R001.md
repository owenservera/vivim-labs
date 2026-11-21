# Source reuse and collision matrix — R001

Observed ARBOR source pin: `e2ef16ec221fb752fd50a1e5808d7e9a7a8e3e42`, following `4623d533` and `446a3376`; Labs R003 staged under `DESIGN/AUTOWIKI-FOUNDATION-R001`. Read of files at one commit is not runtime proof. Repin before changing any ARBOR source. **No ARBOR application code is owned by Planning in this round.**

| Capability | Existing source observed | Decision | Outstanding proof, risk or boundary |
|---|---|---|---|
| Existing Wiki read | ARBOR `core/programmability/src/wiki_read.rs`: `WikiRead`, `read`, `search_view`, revision/digest and source search | **REUSE**; current read projection | Host page identity to graph ID; actual current read route |
| Native graph retrieval | ARBOR `native/hqkg/src/context_retrieval.rs`: `query_scoped`, `ScopedQuery`, `ContextBundle` | **REUSE** existing bounded graph traversal | Trusted exact ID grants/seed selection; prefix-only scope is insufficient for arbitrary private items |
| Native graph scoping | ARBOR `native/hqkg/src/managed_context.rs`, `scoped_graph.rs`, `semantic_source.rs` | **REUSE** existing provenance and source validity | Revocation, graph epoch, visibility and per-item permission producer |
| Rust core module registration | ARBOR `core/programmability/src/lib.rs` | **REPAIR only by Dev** after accepted handoff | Add candidate `autowiki/mod.rs` without backward HQKG dependency |
| HQKG registration | ARBOR `native/hqkg/src/lib.rs` has `native-program` gated module patterns | **REPAIR only by Dev** | Add candidate `autowiki_context_bridge.rs` behind same feature; avoid cycle |
| Staged read-only projection | Labs R003 `src/{lib,context_lens,bundle_projection,owned_context}.rs`, `integration/hqkg_context_bridge.rs` | **REUSE already staged**; first validate | 37 test cases reported static, Rust compilation UNRUN; normalize semantics to real grants |
| Staged English and review candidate | Labs R003 `src/{narrative,quality_feedback}.rs` | **REUSE**; do not create second realizer | Claim verification is not the same as indexed file occurrence; approval route UNKNOWN |
| Semantic identities | ARBOR `apps/vivim-host/chat/wiki-semantics.mjs`, Rust `wiki_target.rs`, existing HQ semantic registry | **REUSE**; aliases only where needed | `wiki.home` etc are planned display IDs, not proven canonical persistent graph IDs |
| Live event projection | ARBOR `core/programmability/src/live_system.rs`: `LiveSystemState`, `RegionKind::Input`, `ChangeEvent` | **REUSE** source/view revisions | Observation is not an authority to execute or publish graph facts |
| Native agent proposal | ARBOR `core/programmability/src/agent_composer.rs`: `prepare_draft`, revision guard, writable action ID | **REUSE** source policy | Existing action ID `pwa.composer.draft.set` applies to phone; do not impersonate GPUI |
| Host-approved agent path | ARBOR `apps/vivim-host/control/agent-composer-bridge.mjs`: observeWithFence, stage/execute/acknowledge | **REUSE**, Dev-owned current integration | Real selected field, owner approval and source ACK; no simulated authorization |
| First-contact browser composition | ARBOR `apps/mobile-relay/{first-contact-composer,agent-composer-adapter}.ts` | **REUSE**, Dev-owned | React to real input, composition/IME, human preemption and approved progressive typing |
| Existing browser Wiki | ARBOR `apps/mobile-relay/app.js`, `wiki-source-view.mjs` plus native/WASM | **REUSE**, Dev-owned | Differentiate general Wiki Search button from first-contact auto-search |
| Host LiveSystemState bridge | ARBOR `apps/vivim-host/control/live-system-read-port.mjs` | **REUSE** Host read and revision generation | Current actor/private Wiki read must be denied unless Host authorizes |
| Owner development source | ARBOR `apps/vivim-host/control/development-read-port.mjs` | **REUSE owner-only source**, not phone API | Never expose owner-private HQ index to unpaired phone, build separate content store or bypass grant |
| Host control/action registry | ARBOR `apps/vivim-host/control/{dispatcher.mjs,action-catalog.v1.json}`, Rust `control_*`, `context_grounding.rs` | **REUSE** Host preview/approval/effect policy | Wiki-read command labels are not executable grants; NO_MATCH on unknown |
| Current first-contact observability | ARBOR `docs/FIRST-CONTACT-LIVE-OBSERVABILITY.md`, existing Host data-root diagnostic events | **REUSE** metadata-only stages | No raw draft, source snippets, secrets; events not visual/permission proof |
| GPUI native | ARBOR `native/gpui-desktop/src/{main,lib}.rs`, `README.md` | **REUSE**, planning prep PAUSED | Existing Wiki opens from Search, native agent target unproven; GPUI cannot be treated as HTML rendering |
| Build/package acceptance | ARBOR `docs/FIRST-CUSTOMER-TRACKER.md`, Labs `CHATGPT-DEV/RELEASE-AND-INTEGRATION-PLAYBOOK.md` | **REUSE** exact evidence gates | Actual Dev code, native binary resolution, Origin, HTTPS and real visual acceptance are independent |
| Check-in/check-out | Labs `TASK-CLAIMS.md`; historical local `OWNER-WORK/*` named in older docs | **REUSE** claims overlay, DO NOT create new task state ledger | Canonical remote OWNER-WORK unavailable; no forged task history |

## Non-duplication and source authority

1. Only HQKG traverses the graph; R003 receives authorized selected nodes and edges and returns a bounded lens. No new graph DB or duplicated source journal.
2. Only Host grants effect authority. Rust draft proposals and Wiki previews are inert. The same existing human/authorized-agent composer input must drive the same read-only Wiki lens.
3. Do not promote annotations/provenance into independently verified capabilities. VLANG/SCF verifier evidence is a separate, currently partially unverified source.
4. No raw source grants, private graph neighbors, passwords, prompt bodies, revoked titles or trace payloads in PWA results/diagnostics.
5. Do not silently replace TypeScript DOM glue with Rust pixel claims. Core product semantics Rust-first; browser DOM events and native GPUI are two distinct adapters.

## Audited references

- [R003 code/integration handoff](../DESIGN/AUTOWIKI-FOUNDATION-R001/R003-EXECUTION-HANDOFF.md)
- [R002 source crosswalk](../DESIGN/AUTOWIKI-FOUNDATION-R001/SOURCE-CROSSWALK-R002.md)
- [Seven-hub identity spec](../DESIGN/AUTOWIKI-FOUNDATION-R001/PAGE-TAXONOMY.md)
- [Oracle shaping](../ORACLE/TO-PLANNING.md)
- [ARBOR first-customer tracker](https://github.com/owenservera/ARBOR/blob/main/docs/FIRST-CUSTOMER-TRACKER.md)
