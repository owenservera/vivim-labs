# DevAgent exact integration runbook — R003/R004

**Preparation artifact only.** Dev owns product integration; Planning does not patch ARBOR. Re-fetch `ARBOR/main` first (latest inspected `7ae24ff78a797581b510fe7a78105bbe301c0de4`). Read `TASK-CLAIMS.md` for concurrent Dev work. Treat all paths as insertion proposals until compared with the new head.

## 0 — Confirm source owners and collision fence

Read existing `core/programmability/src/{lib.rs,wiki_read.rs,live_system.rs,agent_composer.rs}`, `native/hqkg/src/{lib.rs,context_retrieval.rs}`, Host `control/{live-system-read-port,development-read-port,agent-composer-bridge}.mjs`, and the current PWA composer sources. `development-read-port.mjs` is owner-only and must not be exposed to the phone. **Do not modify Dev-owned PWA source in this preparation handoff.**

## 1 — Install core projection modules (after Dev acknowledges)

Use the existing core library `core/programmability/` (crate `vivim-programmability-core`). Create `core/programmability/src/autowiki/` and place the staged R003 [src/lib.rs](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/src/lib.rs) as `autowiki/mod.rs`. Place *all eight* staged sibling modules in the same directory:

```text
autowiki/context_lens.rs
autowiki/bundle_projection.rs
autowiki/owned_context.rs
autowiki/narrative.rs
autowiki/quality_feedback.rs
autowiki/page_alias_projection.rs         # R004
autowiki/readback_guard.rs               # R004
autowiki/query_admission.rs              # R004
```

Add `pub mod autowiki;` in existing `core/programmability/src/lib.rs`. These modules depend only on std and one another; there is no core -> HQKG dependency. **Do not convert** this into a second standalone production crate. Existing Root Cargo/feature resolutions take precedence over this documentation.

## 2 — Install one native HQKG bridge without a new graph traversal

Use **existing** `native/hqkg/` and the existing optional core dependency under `native-program`. Copy staged [R003 integration/hqkg_context_bridge.rs](../../../DESIGN/AUTOWIKI-FOUNDATION-R001/integration/hqkg_context_bridge.rs) to `native/hqkg/src/autowiki_context_bridge.rs`. **Append**, not separately `mod`, [authorized_query_extension.rs](authorized_query_extension.rs). The extension relies on the original bridge's `project_authorized_context` and imported `LensRead`/`LensProjection` symbols, so do not duplicate imports.

Register in `native/hqkg/src/lib.rs`:

```rust
#[cfg(feature = "native-program")]
pub mod autowiki_context_bridge;
```

The existing `query_scoped` is the only BFS. The new Rust query admission validates trusted complete exclusions, exact seed membership, scope ID syntax, current graph epoch and bounded budgets. HQKG then performs the actual read; existing R003 projection performs per-item exact authorization again. *A mere prefix filter or client-provided completeness flag is NOT sufficient.*

## 3 — Source-owner inputs: exact trust boundary

**Trusted Host inputs required before calling read_authorized_context:**

- Current source epoch and actual graph epoch, from authoritative Host/HQKG read; deny if changed.
- Current owner actor grant, authorized graph entity IDs, authorized source-reference IDs and exact committed Wiki page IDs.
- Complete set of excluded in-prefix graph node IDs for the current graph epoch. This needs native graph/source-owner traversal or a pre-existing authoritative permission index; neither has been proven to exist. If coverage is unknown, set completeness false and refuse, rather than silently broadening scope.
- Focus entity already matched by existing semantic resolver, not guessed from raw words; use `page_alias_projection::resolve_exact_hubs` to crosswalk to the exact source-owned `wiki:page:` identifiers.
- Current Host Wiki revision **independently read from source**, not from the PWA expected revision. Pass independent `trusted_scope_id` and `trusted_wiki_revision` into the extension. This fixes a stale-result tautology.

### Function bridge

```text
Host current source/actor grant
 -> TrustedQueryScope (exact scope/epoch/exclusions), LensRead (exact IDs, refs)
 -> read_authorized_context(graph, grant, request, depth, trusted_scope_id, trusted_wiki_revision)
 -> native hqkg::context_retrieval::query_scoped (one bounded BFS)
 -> existing R003 project_authorized_context / per-item denial
 -> lens read ticket + source-attested current graph epoch
 -> readback_guard::accept_readback
 -> existing owner-authorized Host Wiki read projection
 -> Dev-owned existing PWA composer/AutoWiki read-only view
```

**Do not** create another HTTP endpoint, Wiki database, command router, Host journal or permission engine. Resolve the existing Host read port and source grant ownership before applying this final hop.

## 4 — Prevent out-of-order visual truth

On each source-admitted draft, call the existing `context_lens::Lens::apply` and obtain current resolved page selection. Call `readback_guard::issue_read` before the async source read; on completion call `accept_readback` against current Lens and fresh **source-attested** graph epoch. Reject source epoch/scope, Wiki revision, input sequence, page or depth mismatch. A human edit supersedes an agent’s pending read; revocation clears private preview. No Send, provider call or graph write on typing.

## 5 — Verified English + seven hubs

Use existing `narrative::realize` only with independently verified, current, visible `SentenceClaim` and exact authorized evidence refs. Reuse `quality_feedback::review_candidates` as **inert** reviewed candidate diagnostics; never auto-create owner tasks or modify VLANG. Page aliases `wiki.home` through `wiki.development` are display aliases only; require exact mapping to committed source pages, otherwise show truthful empty/unavailable.

## 6 — Test, package, release handoff

Run [TEST-RECIPES.md](TEST-RECIPES.md) in a real cloud checkout with Cargo. First standalone staging tests, then after Dev product integration core and HQKG tests; install `native_hqkg_authorized.rs` into `native/hqkg/tests/autowiki_authorized.rs`. Verify feature-gated native binary linking, Host grant refusal, no-leak and actor race. Only then use the Dev-owned [source-to-live playbook](../../CHATGPT-DEV/RELEASE-AND-INTEGRATION-PLAYBOOK.md) for unchanged URL, native resolver/Origin/HTTPS and actual UI observation. Android is not a native/core gate; report it separately.

## Rollback

Before product integration record precise Dev-owned baseline SHA. Revert only added `autowiki/` modules and two lib.rs module registrations plus the Host consumer **only after coordinating concurrent edits**. Preserve existing Wiki journal, HQKG graph, PWA composer, Dev source commits and all grants; never force push/reset another agent's work.
