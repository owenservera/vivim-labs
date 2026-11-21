# DevOps / Planning SITREP — 2026-11-21 (GitHub bootstrap R004)

**Role state:** planning/pre-integration only; no background agent, live task router, active session bridge or automatic cross-thread synchronization. See [AGENTS](AGENTS.md) -> [PROFILE](PROFILE.md) -> [BUNDLE-INDEX](BUNDLE-INDEX.md); [conversation lessons](CONVERSATION-LESSONS.md) are selective reading.

## Source observation and limits
- Observed ARBOR `main` commit: `446a33766fb11a437670260d9b206f00dafedb47` during this bootstrap. **Not a branch lock; repin before implementing.**
- Labs `main` is receiving these role-profile docs directly; use a fresh HEAD before every bundle.
- Connected Labs GitHub has `DESIGN/AUTOWIKI-FOUNDATION-R001` and `DESIGN/SOURCE-DISPOSITION/OW-20261121-017.md`.
- GitHub operations paths **not found** during this read: `OWNER-WORK/`, `DESIGN/PRE-CODE/`, `code/`, `CONTEXT-MAP.md`. These may be local/ignored; their historical references are **not remote artifacts**. No new OW ledger or invented statuses.
- Shared roles: Dev [AGENTS](../CHATGPT-DEV/AGENTS.md), Oracle [AGENTS](../ORACLE/AGENTS.md). These are documented roles, not proof that another session has accepted a handoff.

## First-customer truths (different sources, different evidence)
| Outcome | Observed code/report | Not yet established by this profile |
| --- | --- | --- |
| User types; AutoWiki reacts | ARBOR `docs/FIRST-CUSTOMER-TRACKER.md` reports first-contact 180ms auto-search, Rust WASM build/ABI result, and local Direct HTTP checks. Separate general Wiki tab has Search/Enter. | Physical Android repaint on unchanged URL; current deployed revision/evidence |
| Agent types visibly; same AutoWiki reacts | Rust `agent_composer.rs`, Host `agent-composer-bridge.mjs`, PWA `first-contact-composer.ts` with `typingIntervalMs` in inspected main | Live owner-approved agent delivery + phone visual frames + source/receipt correlation |
| Context bundles and readable AutoWiki | [R003 prepared Rust modules](../DESIGN/AUTOWIKI-FOUNDATION-R001/README.md) and [handoff](../DESIGN/AUTOWIKI-FOUNDATION-R001/R003-EXECUTION-HANDOFF.md), with 37 staged tests statically counted | Compiler/test execution, merging into ARBOR, canonical page/graph mapping, approved reader scope, full prose/command quality |
| VLANG quality feedback | Staged `quality_feedback.rs` is inert and owner-reviewed by contract | Authoritative SCF/SV claim receipts, live review ingress, verified code feedback loop |

**No production source was edited or compiled by this Planning bootstrap.** Changes are documentation/profile pointers in Labs `main`; no local computer or ZCode was used. Historical build/test results in ARBOR must not be promoted to a current Android PASS.

## Recommended next planning slice (not a second backlog)
1. Refresh Dev ChatGPT's latest ARBOR commits and the owner source of truth.
2. Confirm exact Wiki page ID -> HQKG entity ID crosswalk, source authorization, and Host read port. If unresolved, prepare a narrow *read-only* Rust adapter test contract against current code; no second graph.
3. Stage claim-to-English sentence fidelity tests and a no-leak negative suite only where Dev has not already begun implementation.
4. Hand the already-prepared R003 package to Dev as a **proposal** with base pins and specific UNRUN tests; do not imply acknowledgment or inclusion in ARBOR.

## Fresh-agent operating reminder
Read only role AGENTS, this SITREP, relevant bundle, actual ARBOR source and source receipts; inspect other folders only as necessary. Respect direct GitHub main, narrow paths and one effect authority. If a necessary file is local-only, mark **REMOTE_SOURCE_UNAVAILABLE** rather than silently using Desktop Commander for code work.
