# DevOps/Planning bundle index — verified links, not a task ledger

This file contains **pointers to known pre-integration artifacts**. It does not supersede the canonical OW request/update registry. At the 2026-11-21 GitHub read, the operations-tree `OWNER-WORK/` registry and older `DESIGN/PRE-CODE/` and `code/` locations were **not in Labs remote main**. Do not invent their contents or mark their OW requests complete.

## Current execution pack — 23 November 2026

- **[Source-pinned agent execution pack R001](PACKS/EXECUTION-20261123-R001/README.md)**: five nonoverlapping ready-to-pick-up packages for AI conversations (#32), same-source release/two-way phone parity (#31), native GPUI acceptance, VLANG/AutoWiki/Oracle, and ZCode/Desktop Commander verification. **[Agent responsibilities](PACKS/EXECUTION-20261123-R001/AGENT-HANDOFF.md)** · **[acceptance recipes](PACKS/EXECUTION-20261123-R001/ACCEPTANCE.md)** · **[machine manifest](PACKS/EXECUTION-20261123-R001/manifest.json)**. Last verified observer 215/215 at older SHA `7b9e7dbd`; new source observer manually triggered, result pending at publication. **Publication is not agent acceptance, source deployment or physical device PASS.** Always refresh main and `TASK-CLAIMS` before pickup.

## Master Planning catalogue (new source-backed proposal view)

- [MASTER-PLAN-R001](MASTER-PLAN-R001.md): **37 proposed Planning-only task packets** (5 Governance, 16 AutoWiki R003, 8 PWA-to-development pipeline, 3 GPUI paused, 5 verification). This is a **candidate catalogue**, not canonical OW task history or a second active claim system; Planning Next Ten are proposals, unclaimed until TASK-CLAIMS checkout. Existing Dev PWA/ARBOR product tasks are excluded.
- [SOURCE-REUSE-MAP-R001](SOURCE-REUSE-MAP-R001.md): source-verified map of actual Rust Wiki/HQKG/LiveSystemState/Host/PWA/GPUI seams, prohibiting duplicate stores/engines and flagging unknowns.
- [ACCEPTANCE-AND-HANDOFF-R001](ACCEPTANCE-AND-HANDOFF-R001.md): exact R003 test commands, source/permission/epoch negative gates, source-to-live ladder, delivery fields and UNRUN states.
- [TRACKER-RECONCILIATION-R001](TRACKER-RECONCILIATION-R001.md): absent remote OWNER-WORK reconciliation; do not mint OW IDs or silently replace local historical registry.

**Observed source pins:** ARBOR `e2ef16ec221fb752fd50a1e5808d7e9a7a8e3e42`, Labs `b0eb56e41d76ba785eff91858eb82584b08a1162` before Planning commits. **No R003 Rust compilation in this round**, cargo unavailable in cloud. GitHub Planning artifact push ≠ Dev acceptance or user-visible runtime proof.

## R006 native HQKG complete grant coverage — current offer

- **AW-GRANT-COVERAGE-R006** [Code pack](PACKS/AUTH-R006/README.md) | [Rust native graph grant builder](PACKS/AUTH-R006/native_grant_coverage.rs) | [8 authored native integration tests](PACKS/AUTH-R006/native_grant_coverage_test.rs) | [Dev integration](PACKS/AUTH-R006/DEVAGENT-APPLY.md) | [verification](PACKS/AUTH-R006/VERIFICATION.md) | [Host grant source audit](PACKS/AUTH-R006/HOST-GRANT-SOURCE-AUDIT.md).
- **Reuse** actual `hqkg::Graph::nodes`, `Graph::epoch`, node source refs, existing R004 `TrustedQueryScope`, and incumbent `query_scoped`. No second grant database, graph/index or Host effects engine. Complete per-prefix exclusions only after a bounded scan, exact actor/source grant, valid focus, and grant+epoch postflight; refuse incomplete/oversized scopes.
- **Evidence:** 8 Rust integration `#[test]` markers authored, not executed. R003-R005 existing standalone crate now **82** authored tests, R004 separate native HQKG tests **2** authored. Cargo/rustc and live Host grants **UNRUN**; actual Host-grant producer unknown. Dev ACK **NOT OBSERVED**. GPUI remains paused.

## R005 semantic proof + human Wiki read pipeline — offer

- **AW-SEMANTIC-PRECODE-R005**: [DevAgent R005 source+code pack](PACKS/SEMANTIC-R005/README.md) | [existing-source/proposed SV proof schema](PACKS/SEMANTIC-R005/SOURCE-TRUTH-AND-ABI.md) | [exact integration plan](PACKS/SEMANTIC-R005/DEVAGENT-INTEGRATION.md) | [acceptance tests](PACKS/SEMANTIC-R005/ACCEPTANCE.md).
- **Actual source staged:** `DESIGN/AUTOWIKI-FOUNDATION-R001/src/{semantic_evidence,page_assembly,read_pipeline}.rs`, registered in existing `src/lib.rs`; uses R003 narrative and R004 readback/identity guards. No duplicate verifier/graph/page store.
- **Critical discovery:** current ARBOR Semantic Verifier is **design only / NOT IMPLEMENTED**. Default proof reader None; code cannot call a hypothetical production checker. Source occurrence and test-only evidence do not establish runtime capability. Semantic graph epoch/revision is independent of Wiki journal revision.
- **Evidence:** 82 authored standalone Rust tests total (37 R003 +20 R004 +25 R005); native HQKG tests 2 additional authored; static export checks PASS. Cargo/rustc build and runtime **UNRUN** in this container. **OFFERED_TO:CHATGPT-DEV; receiver ACK NOT_OBSERVED**.

## R003/R004 DevAgent code prep — current offer

- **AW-R003-PRECODE-R004:** [R003 DevAgent code-pack entry](PACKS/R003/README.md) | [exact cloud integration runbook](PACKS/R003/DEVAGENT-APPLY.md) | [Rust/native tests](PACKS/R003/TEST-RECIPES.md) | [authorized HQKG query extension](PACKS/R003/authorized_query_extension.rs) | [native integration tests](PACKS/R003/native_hqkg_authorized.rs).
- **Staged in existing R003 Rust crate:** [exact source-owned page alias projection](../DESIGN/AUTOWIKI-FOUNDATION-R001/src/page_alias_projection.rs), [stale readback/digest guard](../DESIGN/AUTOWIKI-FOUNDATION-R001/src/readback_guard.rs), [complete source-grant query admission](../DESIGN/AUTOWIKI-FOUNDATION-R001/src/query_admission.rs), and [registered modules in src/lib.rs](../DESIGN/AUTOWIKI-FOUNDATION-R001/src/lib.rs). Existing five R003 Rust modules untouched.
- **Evidence:** 57 statically counted authored standalone Rust tests (37 original + 20 new), 2 additional staged native HQKG integration tests; module export and source-fence static checks PASS. Rust cargo/fmt/clippy/ARBOR compilation/runtime **UNRUN** (no cargo/rustc in this ChatGPT cloud container). ARBOR product code **UNCHANGED**.
- **Ownership:** Planning sender; **OFFERED_TO:CHATGPT-DEV**, receiver ACK **NOT OBSERVED**. Dev must re-pin ARBOR and obtain current genuine Host actor/source scope and graph-exclusion coverage; never infer permission from query prefix or current expected revision. GPUI preparation remains owner-paused.

## Available on connected GitHub main

| Scope | Available artifact | Owner/collision boundary | Evidence state |
| --- | --- | --- | --- |
| **OW-017: AutoWiki top-level pages / dynamic context / readable English / VLANG quality** | [AutoWiki Foundation R001 + R002/R003](../DESIGN/AUTOWIKI-FOUNDATION-R001/README.md), [exact R003 Rust integration handoff](../DESIGN/AUTOWIKI-FOUNDATION-R001/R003-EXECUTION-HANDOFF.md), [OW-017 disposition](../DESIGN/SOURCE-DISPOSITION/OW-20261121-017.md) | Planning prepares Rust code in Labs; Dev integrates in ARBOR after current-source/collision check | PREPARED / PUSHED TO LABS; 37 authored tests, **UNRUN**, no ARBOR integration |
| **OW-007: user typing -> AutoWiki** | ARBOR `docs/FIRST-CUSTOMER-TRACKER.md`, `apps/mobile-relay/app.js`, `core/programmability/src/wiki_read.rs` | **Dev ChatGPT owns active PWA code**; Planning observes and prepares follow-on contracts | First-contact implementation reported; physical Android confirmation UNRUN at inspected source |
| **OW-009 / OW-010 / OW-014: agent-visible typing** | ARBOR `docs/integration/OW-20261121-LIVE-FABRIC-STATUS.md`, `core/programmability/src/agent_composer.rs`, `apps/vivim-host/control/agent-composer-bridge.mjs`, `apps/mobile-relay/first-contact-composer.ts` | **Dev ChatGPT owns active Rust/Host/PWA implementation**; Planning prepares downstream wiki parity and testing | Source present; actual on-phone source/revision/receipt proof still separate |
| **OW-011/012/013/016: broader design and activity fabric** | Source-specific ARBOR components and previous Labs pointers may exist elsewhere; verify every path individually | Do not silently take over parallel owners | Not reconciled by this role bootstrap |

## Deliberately unverified historical pointers

Earlier planning docs referenced `../OWNER-WORK/NOW.md`, `../OWNER-WORK/UPDATES.jsonl`, `../DESIGN/PRE-CODE/INTEGRATION-PLAYBOOK.md`, `../code/agent-native-composer/SPRINT.md`, local ignored `.private/VLANG-PWA-RUNTIME-INTEGRATION`, and a deployment note. They were **not verified in the connected Labs GitHub operations tree**. A local-only path is not a shareable Dev bundle. Re-read through approved context access only when truly required; don't silently start a Desktop Commander coding session.

## How to add a bundle
1. Resolve the owner's existing OW ID; use the task's actual current `ARBOR/main` SHA and source symbols.
2. Check Dev and Oracle ownership, then make a narrowly scoped Labs package following [BUNDLE-TEMPLATE.md](BUNDLE-TEMPLATE.md).
3. Execute cloud tests when available, record exact PASS/FAIL/UNRUN, commit directly to Labs main, and fetch back the committed result.
4. Add exactly one pointer here, with dependency, code ownership and evidence class. **A pushed bundle is not an adopted Dev handoff.**

## GESTURES-R002 — bidirectional Wiki hold/preview/cancel (2026-11-21)

- [Prepared gesture design/Dev handoff](../DESIGN/gestures/README.md) — 12 source-pinned bounded work packages, [machine manifest](../DESIGN/gestures/manifest.v1.json), G0–G9 physical PWA proof matrix; ARBOR source pin cf2e5c8.
- Proposed PWA-visible review outcome **PV-15**, supports existing PV-01/04/05/06/08/10 and reuses TierFlow A039, VEC, Rust LiveSurface, Activity Fabric, First Contact Wiki and Host permissions. **PREPARED / no ARBOR source changes / tests UNRUN / Dev ACK not observed.**

## GESTURES-R003 — shared GPUI/PWA Rust precode (2026-11-22)

- [R003 shared contract](../DESIGN/gestures/R003-CANONICAL-CONTRACT.md), [pure Rust reducer](../DESIGN/gestures/src/gesture_interaction.rs), [Dev bridge](../DESIGN/gestures/R003-DEV-INTEGRATION.md), [G0–G11 gates](../DESIGN/gestures/R003-ACCEPTANCE.md) and [evidence](../DESIGN/gestures/R003-EVIDENCE.md). This **supersedes R002's PWA-only framing**, while reusing its previous groundwork and IDs.
- **15 trackable GEST items**, with GEST-04 PRECODE_STAGED_UNRUN and GEST-13–15 native GPUI/phone parity additions. 17 Rust tests AUTHORED / UNRUN (rustc/cargo unavailable). **No ARBOR source changes, no Dev acceptance, no deployment/physical verification.**
