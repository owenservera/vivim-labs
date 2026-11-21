# R003 DevAgent Rust code-prep packs — AW-R003-PRECODE-R004

**State: staged code + static source audit, NOT compiled, integrated, deployed or accepted by Dev.** This Labs-owned package builds on the [existing AutoWiki Foundation R003](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/R003-EXECUTION-HANDOFF.md), instead of recreating its graph, wiki, compiler or Host authority. The two PWA typing milestones remain **ChatGPT Dev owned**. GPUI prep remains **PAUSED**.

## Source pins and current owner state

- **ARBOR `main` observed:** `7ae24ff78a797581b510fe7a78105bbe301c0de4` (21 Nov 2026); re-fetch immediately before product edits because Dev continues modifying the PWA.
- **Labs:** these packets live in `vivim-labs/main`; inspect current `TASK-CLAIMS.md` before integration. Planning checkout `AW-R003-PRECODE-R004` owns Labs prep, not ARBOR runtime files.
- **Current collisions:** Dev checkout `PWA-FIRST-WIKI-MOUNT-R002` owns `apps/mobile-relay/app.js` and relevant tests; separate Dev UTF-8 regression claim concerns PWA adapter/test. Do not modify those paths from this package.
- **Existing R003 baseline:** 37 authored Rust tests. Added R004 modules contain 18 more tests, totaling **55 statically counted staging tests**, plus **2 proposed native HQKG integration tests**. Neither count means tests passed.

## Three implementation-ready packets

| Pack | Actual code / integration | Dependent task keys | Dev scope after explicit acceptance |
|---|---|---|---|
| **A — Exact Wiki identities** | [page_alias_projection.rs](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/src/page_alias_projection.rs) and existing R003 `bundle_projection.rs` | AW-003/008 | Rust core only; source-owner feeds canonical Host `wiki:page:` IDs and actual HQKG IDs |
| **B — Authorized bounded native read** | [query_admission.rs](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/src/query_admission.rs), [authorized_query_extension.rs](authorized_query_extension.rs), [native HQKG tests](native_hqkg_authorized.rs) | AW-004/010/013/014 | Rust core policy + **existing** HQKG query_scoped and trusted Host read port |
| **C — Stale-result fencing / same-lens acceptance** | [readback_guard.rs](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/src/readback_guard.rs), existing `context_lens.rs`, existing narrative/quality modules | AW-005/006/007/009/015 | Rust core, current Host source owner and Dev's existing PWA read-only renderer, not another composer |

**Module registration in Labs is already staged** in the existing [R003 Rust crate root](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/src/lib.rs). All new policy modules are std-only; the native extension relies on the existing HQKG `native-program` feature and existing core dependency.

## Receiver instructions

Read [DEVAGENT-APPLY.md](DEVAGENT-APPLY.md) in order, then [TEST-RECIPES.md](TEST-RECIPES.md). Copy only exact modules from Labs into the existing ARBOR module homes after refreshing source and accepting the owner checkout. **Do not copy this entire Labs repository as a new crate or create a second graph runtime.**

## Critical blockers that are not code-creation work

1. Host current source owner must produce **real canonical wiki IDs** and exact entity IDs; display aliases are not durable identities.
2. Host must independently establish the complete disallowed in-prefix graph IDs for the current graph epoch (or decline retrieval). A mere `exclusions_complete=true` without source-owner proof is forbidden.
3. Host must independently supply current actor grant, trusted Wiki revision and authorized source references. An expected revision from browser request is NOT proof.
4. Existing independently verified VLANG/SCF claim receipts are required before narrative code can promote capability statements to verified English. Until then render honest unverified/unknown states.
5. Exact Host read-port entrypoint for scoped Wiki+HQKG projection remains a Dev integration decision; do not create a public owner-private development endpoint.

## Test classification

**Static evidence:** GitHub source-read and export checks performed on eight Rust module declarations and 55 `#[test]` markers. **Rust `cargo test`, `cargo fmt`, `cargo clippy`, module-level ARBOR build, Host integration, PWA browser, GPUI Windows, Android and deployment are UNRUN.** Current ChatGPT cloud container has Node 22 but no Cargo or rustc. Source code, unit tests and proposed native integration tests are not a passing release.

## Completion criteria for this Planning handoff

- Current source hashes and file collision boundaries included.
- Exact Rust insertion paths and feature gates specified.
- Actual authored code, validation tests, privacy refusal cases, rollback and command matrix available to Dev.
- Separate receiver `ACCEPTED_BY:CHATGPT-DEV` check-in needed before ARBOR integration. Until then this is an offered, **unaccepted** Labs prep bundle.
