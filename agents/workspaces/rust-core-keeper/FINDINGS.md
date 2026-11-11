# Rust core keeper — running findings and handoffs

Observations/handoffs, **not a second roadmap, capability registry, authorization ledger or current-runtime status authority**. Seeded 2026-11-11 from existing records only; no broad investigation, source revalidation or Cargo gates rerun. Historic findings are revision-bound, not presumed live. Workflow `dwfrun-0b167562-78c5-4276-bc9f-a89a5f0c8072` remains ongoing per supplied checkpoint; no result predicted.

## Authorities and evidence keys

- Product priorities: [Harness roadmap](https://github.com/owenservera/vivim-web-harness/blob/main/docs/ROADMAP.md) and [machine ledger](https://github.com/owenservera/vivim-web-harness/blob/main/docs/roadmap.json); link their existing items, do not copy a competing backlog here.
- Local recorded evidence: [STRATEGY](STRATEGY.md), [WORKPLAN](WORKPLAN.md), [STATUS](STATUS.md), append-only [LOG](LOG.md). Source references below are inherited anchors, not newly inspected source.
- Captured local revisions (STRATEGY:11 / STATUS:55): **HQ** `a1d6ada4c2b7c17ed57c4ba77c24f95c4d5d286c`; **Harness** `59288b36d56f68c8073caa0a75a72d43f9bf9a1d`; **TierFlow** (`vivim-ui`) `50822f11ee63c3d80f631f8fea7dcb9b19bffc38`; **Labs** `34b6ca77ce91c93fe90feaa590a7f0de8d0ebcaa`. No fetch/current-head or installed-runtime claim. HQ Rust paths mean `research/rust-knowledge-graph/`.
- **M** = existing keeper master-integration memory checkpoint, read via the matching memory-index lines: `rust-core-keeper-master-integration-20261111.md`, sections Latest recovery checkpoint / Current workflow shape / Success and boundaries. This is a secondary recorded observation, not current-run telemetry; private session content is not copied here.

## Standing maintenance rule

Capture important discoveries **when found**, with the next unused stable `RCK-F###` ID; never recycle IDs. Retain dated disconfirmations, changed evidence and superseding links under the same item rather than silently deleting history. Record exact revision/blob and actual command exits when available; otherwise mark evidence gaps. Route cross-owner items with evidence through an authorized channel. **Recorded ≠ sent ≠ acknowledged ≠ acted on**: attach separate delivery, reply and implementation/acceptance evidence. This documentation mandate sends nothing. Update linked authorities through their owners, never by treating this tracker as permission or a replacement registry.

## Entries

### RCK-F001 — HQKGX1 parser record-order hypothesis
- **Date:** 2026-11-11. **Category:** risk. **State:** hypothesis.
- **Statement:** Recorded source review suggests tombstone→node same-ID and edge-after-tombstone may evade the intended monotonic phase checks; no failing regression was executed in that review.
- **Source/revision evidence:** HQ captured revision above; STRATEGY:39; WORKPLAN:5–22; `src/import.rs:215–238,280–293`; reverse-order-only coverage `tests/import.rs:184–198`.
- **Impact:** Potential accepted-exchange/apply inconsistency; not a reproduced defect or remote exploit claim.
- **Affected platform/owner:** HQKG / Rust keeper; HQ exchange producer coordination with hq-data-keeper.
- **Next action:** After current-source recheck and crate clearance, run a valid-digest tombstone→same-ID-node regression plus edge-after-tombstone case; fix only if reproduced and admitted.
- **Status:** Pending reproduction; current workflow outcome unknown. **Canonical issue/contract:** `repos-hq.hqkg-exchange/1`; WORKPLAN §1; no issue link recorded.
- **Delivery:** Recorded here; not sent, acknowledged or acted on by this tracker turn.

### RCK-F002 — Foreign observability exclusion is crate-wide
- **Date:** 2026-11-11. **Category:** handoff. **State:** source-only.
- **Statement:** M records `p1-rust-causal-observability` as excluding other Rust writers across the whole crate, not only `src/observability.rs`; recovery-test writes also need clearance. Old dirty-work observations do not prove current ownership/liveness or release.
- **Source/revision evidence:** M Current workflow shape; HQ `hq/aii/build-plan.json:78–81` as recorded there (checkpoint did not supply a fresh commit pin); STRATEGY:13,27 and STATUS:56 record earlier 174-line foreign modification; M additionally records untracked `examples/observability_probe.rs`. LOG:14 later records repair verification, not exclusion release.
- **Impact:** Single-file scoping could still collide with a foreign crate lane.
- **Affected platform/owner:** Entire HQKG crate / parent coordinator, Rust keeper and causal-observability owner.
- **Next action:** Parent obtains current exclusion/release evidence and reconciles accepted diff/test scope before admitting any Rust source or test writer; preserve foreign work.
- **Status:** Clearance unverified; no release inferred. **Canonical issue/contract:** Existing HQ AII build-plan assignment; no separate issue link recorded.
- **Delivery:** This exclusion handoff not sent here; no acknowledgement/action evidence recorded.

### RCK-F003 — HQ JS context is not Rust context parity
- **Date:** 2026-11-11. **Category:** finding. **State:** source-only.
- **Statement:** Recorded integration review distinguishes HQ JS source-line context composition from Rust graph BFS retrieval, including different token-estimator proxies. Substitution cannot claim semantic or budget parity merely from matching output sizes.
- **Source/revision evidence:** M Success and boundaries (no fresh source pin there); STRATEGY:23,35,49 at captured HQ revision; Rust `src/context_retrieval.rs:20–82`. Exact JS estimator formula/line was not retained in these seed records; do not invent it.
- **Impact:** Risk of lost source-line verification, relevance/freshness behavior or misleading budget/performance claims.
- **Affected platform/owner:** HQ JS context / hq-data-keeper; Rust retrieval / Rust keeper; chosen Host consumer owner.
- **Next action:** Pin both actual implementations and estimator formulas; add a synthetic differential fixture covering source-line checks, relevance, exclusions, freshness, omissions and budgets before admitting replacement.
- **Status:** Parity unproven. **Canonical issue/contract:** Existing HQ `hq/lib/context.mjs`; Harness W04 roadmap journey; no issue link recorded.
- **Delivery:** Recorded; not sent or acknowledged; no parity implementation accepted here.

### RCK-F004 — Keep native programmability and TierFlow distinct
- **Date:** 2026-11-11. **Category:** finding. **State:** source-only.
- **Statement:** Harness native programmability and TierFlow aggregation are distinct useful components, not demonstrated HQKG adoption or automatic prerequisites for its integration. Preserve their unique value; no retirement proposed.
- **Source/revision evidence:** Captured Harness/TierFlow revisions above; STRATEGY:25–26,47; Harness `src-tauri/Cargo.toml:18`, `src-tauri/src/main.rs:145–146,178`, `core/programmability/GRAPH-INTEGRATION.md:15–32`; TierFlow `core/Cargo.toml:1–15`, `core/src/main.rs:1–19`, `src/hotpath.ts:22`.
- **Impact:** Avoids false adoption claims and collapsing independent compiler/visual-event semantics into a second engine or rewrite.
- **Affected platform/owner:** Harness native-program owner; TierFlow visual/runtime owner; Rust keeper.
- **Next action:** Coordinate a source/policy/action-ID/risk/approval semantic crosswalk if a real consumer needs it; retain components until differential evidence supports a shared slice.
- **Status:** Complementary implementations retained; runtime wiring not revalidated. **Canonical issue/contract:** Harness existing `GRAPH-INTEGRATION.md`, `command-program/1` and `native-program/1` crosswalk.
- **Delivery:** Recorded; not sent or acknowledged; no consolidation action taken.

### RCK-F005 — Host invocation boundary remains a decision
- **Date:** 2026-11-11. **Category:** decision. **State:** source-only.
- **Statement:** Rust's central computational remit does not select a Host ABI/invocation adapter or transfer data/effect authority. First existing W04 consumer and authorized insertion point were unresolved in recorded planning.
- **Source/revision evidence:** STRATEGY:34–35,41,51; WORKPLAN:32–38; captured Harness revision, `docs/ROADMAP.md:11,41,58,139–141`; M Current workflow shape requires discovery before implementation-hole admission.
- **Impact:** Assuming Tauri-from-Node, personal/development graph convergence or a new service could cross authority and compatibility boundaries.
- **Affected platform/owner:** Harness Host/World and phone owners, hq-data-keeper, Rust keeper, parent coordinator.
- **Next action:** At the existing workflow admission gate, name one real consumer, authenticated adapter, invocation contract, revision/revocation behavior and strongest runnable parity/permission gate; leave unavailable device/IPC gates explicit.
- **Status:** Await discovery/admission evidence; no selected boundary claimed. **Canonical issue/contract:** Harness W04 roadmap/ledger; existing workflow hole “Admit exact implementation scopes and consumer gates”.
- **Delivery:** Recorded; not sent or acknowledged; no Host integration accepted here.

### RCK-F006 — Trusted mutation APIs and atomicity need explicit contracts
- **Date:** 2026-11-11. **Category:** decision. **State:** source-only.
- **Statement:** Recorded review distinguishes guarded public puts from deliberately raw change-set apply and caller-constructed Exchange values. Caller staging in the CLI is not proof of atomic library apply or universal source ownership fencing.
- **Source/revision evidence:** Captured HQ revision; STRATEGY:40,51; WORKPLAN:24–30; `src/lib.rs:168–181,224–232,272–278`; `src/graph_change_set.rs:198–216,367–413`; `src/scoped_graph.rs:71–95,154–174`; `src/import.rs:19–39,336–413`; existing tests `tests/import.rs:45–57,162–179` inspect the untouched original.
- **Impact:** Ambiguous trust boundaries, partial-mutation expectations and reserved-key/ownership semantics; no remote exploit established.
- **Affected platform/owner:** Rust API/adapter owners, HQ importer caller and hq-data-keeper; parent resolves contract scope.
- **Next action:** Decide trusted-internal versus untrusted-entrypoint guarantees, then test late failure against the actual supplied graph, scoped extra reserved keys, imported-record overwrite/removal and replay preservation under cleared scope.
- **Status:** Contract decision and focused tests pending. **Canonical issue/contract:** Existing HQKG mutation/owner-fencing contracts; WORKPLAN §2; no issue link recorded.
- **Delivery:** Recorded; not sent or acknowledged; no hardening change authorized by this entry.

### RCK-F007 — Baselines apply only to captured revisions and trees
- **Date:** 2026-11-11. **Category:** finding. **State:** verified (historical recorded gate evidence only; not rerun).
- **Statement:** The exact clean baseline verifies only its captured HQ revision/tree. Later repair evidence is a separate checkpoint; neither establishes current workflow, consumer, physical-device or production acceptance.
- **Source/revision evidence:** STATUS:19–36; [exact baseline log](baseline-20261111T005731588684Z-evidence.log): HQ `a1d6ada4c2b7c17ed57c4ba77c24f95c4d5d286c`, unchanged clean crate hashes/status, fmt/check/test-importer/build-importer each 0; 225 passed / 0 failed / 11 ignored. LOG:14 subsequently records fmt/check/test-importer/clippy-importer/build-importer each 0, 236 passed / 0 failed / 11 ignored, focused 11 passed; observability blob `4696d63adf0c528c81f182388f4d1467f6a5d619`, probe blob `fc754e06d8067eae3b79652149aad0e4c200d181`. Later record does not give a new whole-tree commit pin here.
- **Impact:** Prevents extending old green results to foreign edits, a different tree or uninspected settled workflow commands.
- **Affected platform/owner:** Rust keeper, workflow coordinator and independent acceptance reviewer.
- **Next action:** Attach actual exits/totals and exact tree/blob pins to the next returned workflow checkpoint; revalidate only through its admitted gates. Preserve old evidence rather than relabel it current.
- **Status:** Historical baselines retained; current campaign acceptance unknown. **Canonical issue/contract:** Existing baseline evidence; LOG records HQ-065 PASS receipt `rcpt_423298bd289612f0f65be9f1` for the separate causal repair.
- **Delivery:** Baseline entry not sent here. LOG:14 records prior `vivim.rust-causal-attribution/1` publication to hq-data-keeper, bus message `20261111T020102113Z-369a97`; this proves a recorded prior send only, not recipient acknowledgement/action or this tracker's delivery.

### RCK-F008 — Mixed-owner deletion and recovery are compatibility decisions
- **Date:** 2026-11-11. **Category:** risk. **State:** source-only.
- **Statement:** Recorded review flags cross-source incident-edge deletion and deliberately different legacy HQJ1-tail recovery versus HQD1 refusal. Uniform strict refusal is a policy change, not an established missing guard.
- **Source/revision evidence:** Captured HQ revision; STRATEGY:42,51; WORKPLAN:40–44; `src/lib.rs:304–324,1049–1065`; `tests/core.rs:48–61`; existing importer/durable coverage linked in STRATEGY.
- **Impact:** Silent ownership or journal-compatibility changes could discard unique behavior; restart unit tests do not prove Windows power-loss safety.
- **Affected platform/owner:** Rust durability/import owners; HQ source adapter owners; parent for compatibility decisions.
- **Next action:** Agree mixed-source cascade and legacy recovery policy, then run bounded failure/restart/CAS fixtures after crate clearance; keep physical crash/filesystem guarantees unproven until separately evidenced.
- **Status:** Policy and additional evidence pending. **Canonical issue/contract:** Existing HQJ1/HQD1 and HQKG source-ownership contracts; WORKPLAN §4.
- **Delivery:** Recorded; not sent or acknowledged; no recovery-policy change made.

### RCK-F009 — Ongoing integration run is not accepted implementation
- **Date:** 2026-11-11. **Category:** handoff. **State:** source-only.
- **Statement:** Supplied checkpoint says existing run `dwfrun-0b167562-78c5-4276-bc9f-a89a5f0c8072` is ongoing. M records interruption/recovery of that same run and uninspected settled baseline steps; settlement, design or broad strategy completion is not passing gates or completed consumer integration.
- **Source/revision evidence:** User's current mandate; M Latest recovery checkpoint and Current workflow shape; existing draft `C:/0-BlackBoxProject-0/repos_hq/.zcode/workflow-drafts/Rust-centered-VIVIM-master-integration.dwf.ts` as recorded in M (draft/run not queried this turn).
- **Impact:** Prevents duplicate campaigns, predicted outcomes and premature integration claims.
- **Affected platform/owner:** Parent workflow coordinator, Rust keeper, independent integration reviewer and admitted consumer owners.
- **Next action:** Reconcile this tracker with the next actual checkpoint from the same run; append dated confirmations/disconfirmations and exact gates, preserving earlier observations. Do not start a replacement run from this tracker task.
- **Status:** Ongoing per supplied checkpoint; results awaited. **Canonical issue/contract:** Existing run and its admission hole, not a new roadmap item.
- **Delivery:** Recorded; not sent, acknowledged or acted on by this tracker turn.

### RCK-F010 — Current owner program-level authority decision
- **Date:** 2026-11-11. **Category:** authority decision. **State:** explicit direct-user grant; no product execution claimed.
- **Exact owner quote:** 'UPDATE YOUR SENIORITY: YOU ARE PROGRAM LEVEL - YOU HAVE FULL INGESTION AUTHORITY AND PROGRAM EXECUTION AND REAL WORLD TESTING - YOU SHOULD NOT HAVE ANY BLOCKERS - IF YOU DO LET ME KNOW AND I WILL REMOVE THEM'.
- **Decision:** Program-level Rust capability/integration remit, full ingestion authority, program execution and real-world testing granted by current owner. Supersedes old planning-only restrictions and blanket docs freezes within authorized scope. Active foreign writes require coordination/protection and exact overlap resolution, not redundant owner approval.
- **Limits:** Authority is not technical availability. Credentials/private-source restrictions, provenance/classification and approved ingestion still matter; no assumed consent for unrelated third-party data. Host effect enforcement, approvals and receipts must not be bypassed. External publication/destructive actions outside the grant require confirmation.
- **Evidence:** Current direct user instruction, not inferred from claim/inbox/workflow. Definition read without edits. ROLE updated and existing matching authority feedback memory reused, not duplicated.
- **Completion:** Supplied goal runtime says strategy complete; master integration `dwfrun-0b167562-78c5-4276-bc9f-a89a5f0c8072` remains unfinished. No workflow query, implementation completion or execution acceptance claimed here.
- **Blockers / smallest remedy / continuing work:** No Agent/SendMessage dispatch tool exposed in current session; search did not expose Agent. Restore supported background dispatch for subsequent execution; bounded recording continued meanwhile. RCK-F002 is historical conflict evidence requiring current coordination, not assumed live exclusion/release. Devices/credentials/providers unprobed; no failure or availability invented. Escalate concrete blockers with exact evidence, smallest remedy and work continuing.
- **Delivery:** Actual Bun sync send exited 0: `20261111T021409042Z-b0a24a`, FROM rust-core-keeper TO `*`, kind `status`, topic `rust-core-program-authority`. Requested relevant seats flag exact overlaps/blockers with evidence, smallest remedy and continuing work. Delivery only, NOT wake, acknowledgement, acceptance or action. No recipient reply observed this turn. This supersedes the original tracker-turn no-send restriction for this explicitly authorized broadcast only.
- **Scope / gates:** Only authorized ROLE/STATUS/FINDINGS, append-only LOG, existing feedback authority memory and bounded MEMORY pointer plus authorized bus write. No source/config/tools/model edits, provider/device effects, commits/pushes. All execution gates UNRUN. Historical four-repository pins and duplicate/unique dispositions retained, not freshly verified; no retirement. No bundles: Git writes prohibited in this mandate.
- **Next action / rollback:** Continue the existing unfinished integration through coordinated source-grounded scopes and actual gates, not a replacement campaign. Rollback only this authority prose/memory/index and LOG append, preserving historic evidence/foreign work; bus delivery remains durable.

## Original tracker-creation turn (historical)

Only this file and an appended LOG entry are authorized writes. No Rust/source, agent config, shared ledger, bus or memory changes; no commits/pushes. No four-repository bundle checkpoints created: Git writes are prohibited and this turn changes documentation only. Rollback: remove this newly created file and only this turn's appended LOG entry; preserve all prior records and foreign work. Structural validation is read-only and does not validate the source hypotheses or ongoing workflow.
