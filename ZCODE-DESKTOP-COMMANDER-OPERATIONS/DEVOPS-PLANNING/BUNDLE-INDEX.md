# DevOps/Planning bundle index — verified links, not a task ledger

This file contains **pointers to known pre-integration artifacts**. It does not supersede the canonical OW request/update registry. At the 2026-11-21 GitHub read, the operations-tree `OWNER-WORK/` registry and older `DESIGN/PRE-CODE/` and `code/` locations were **not in Labs remote main**. Do not invent their contents or mark their OW requests complete.

## Master Planning catalogue (new source-backed proposal view)

- [MASTER-PLAN-R001](MASTER-PLAN-R001.md): **37 proposed Planning-only task packets** (5 Governance, 16 AutoWiki R003, 8 PWA-to-development pipeline, 3 GPUI paused, 5 verification). This is a **candidate catalogue**, not canonical OW task history or a second active claim system; Planning Next Ten are proposals, unclaimed until TASK-CLAIMS checkout. Existing Dev PWA/ARBOR product tasks are excluded.
- [SOURCE-REUSE-MAP-R001](SOURCE-REUSE-MAP-R001.md): source-verified map of actual Rust Wiki/HQKG/LiveSystemState/Host/PWA/GPUI seams, prohibiting duplicate stores/engines and flagging unknowns.
- [ACCEPTANCE-AND-HANDOFF-R001](ACCEPTANCE-AND-HANDOFF-R001.md): exact R003 test commands, source/permission/epoch negative gates, source-to-live ladder, delivery fields and UNRUN states.
- [TRACKER-RECONCILIATION-R001](TRACKER-RECONCILIATION-R001.md): absent remote OWNER-WORK reconciliation; do not mint OW IDs or silently replace local historical registry.

**Observed source pins:** ARBOR `e2ef16ec221fb752fd50a1e5808d7e9a7a8e3e42`, Labs `b0eb56e41d76ba785eff91858eb82584b08a1162` before Planning commits. **No R003 Rust compilation in this round**, cargo unavailable in cloud. GitHub Planning artifact push ≠ Dev acceptance or user-visible runtime proof.

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
