# R003 — owner queue and drop-in code handoff
**Disposition:** implementation PREPARED IN LABS, not applied to ARBOR product source. Dev ChatGPT owns current PWA goals and active ARBOR files. This package is a read-only future integration candidate.
**Source observed:** ARBOR main commit `446a33766fb11a437670260d9b206f00dafedb47` (2026-11-21 retrieval). Refresh main before any product patch. Another agent can advance it.

## Six workstreams, exact next gates

| Ordered lane | Verified existing code / source | Unfinished work | This package's handoff |
| --- | --- | --- | --- |
| 1. Human typing -> AutoWiki | apps/mobile-relay/app.js; first-contact search flow; core/programmability/src/wiki_read.rs. First-customer tracker reports 180ms input debounce, actual native WASM ABI test, Android repaint UNRUN | Live phone rendering + exact source-fence proof; owner Dev ChatGPT is implementing | **Do not edit** Dev's code. Consume its stabilized draft-to-wiki event. |
| 2. Live System Fabric | core/programmability/src/live_system.rs; Host live change feed and source revision guards | Complete coverage of other live regions; real device observation; no shadow state | Use existing source epoch, scope and wiki revision; context_lens.rs rejects stale input and revocation |
| 3. AutoWiki + VLANG context | native/hqkg/src/context_retrieval.rs; core/programmability/src/wiki_read.rs; wiki-semantics.mjs; Rust search; HQ context | Canonical ID crosswalk + secure graph context retrieval on phone + verified English explanation + link relevance | bundle_projection.rs + owned_context.rs + narrative.rs + HQKG adapter + acceptance tests |
| 4. Agent-visible typing and UI | core/programmability/src/agent_composer.rs; agent-composer-bridge.mjs; first-contact-composer.ts with typingIntervalMs in current source | Physical Android typing visibility, authoritative touch targets and real action receipt | One focus projection for both human and Host-authorized agent edits; no separate agent wiki path |
| 5. Rust convergence | core/programmability/Cargo.toml; native/hqkg/Cargo.toml; native GPUI. Tauri remains archive | Migrating additional JS decision logic without Rust dependency cycles or duplicating Host effects | Drop-in nested core module + feature-gated HQKG adapter; no new graph engine |
| 6. Closed-loop quality | Rust semantic code, provenance classes and existing development tracker; VLANG SCF/SV program remains partially design | Real claim identity/evidence mapping and reviewed quality-to-VLANG handoff | quality_feedback.rs emits inert, source-revisioned review candidates; no self-edit |

## Complete staged Rust crate

```
AUTOWIKI-FOUNDATION-R001/
  Cargo.toml
  src/lib.rs                       # planned rename to autowiki/mod.rs in ARBOR
  src/context_lens.rs              # focus, pin, depth, input and source fences
  src/bundle_projection.rs         # current authorized graph cards/links
  src/owned_context.rs             # ergonomic owned receipt -> borrowed projector
  src/narrative.rs                 # grounded English sentences, explicit uncertainties
  src/quality_feedback.rs          # earliest-stage reviewed diagnostics
  tests/preintegration_acceptance.rs # common human/agent use and revocation
  integration/hqkg_context_bridge.rs # ARBOR-only feature-gated adapter
```

The crate does NOT reimplement HQKG query_scoped, user/agent draft submission, Host Control effects, approval, live event transport, page persistence or the VLANG compiler. Graph source permission and verified-fact status are attested by their existing owners.

## Integration patch: two native homes, one graph

1. On refreshed ARBOR main and after the Dev ChatGPT has finished its active PWA integration, copy **the staged src/lib.rs as** `core/programmability/src/autowiki/mod.rs`; copy the five module source files into that same `autowiki/` directory. All staged sibling module imports use `super::`, intentionally supporting this move. Add `pub mod autowiki;` to the existing `core/programmability/src/lib.rs`. The module is std-only, so no new Cargo dependency.
2. Copy `integration/hqkg_context_bridge.rs` to `native/hqkg/src/autowiki_context_bridge.rs`. Add the line `#[cfg(feature="native-program")] pub mod autowiki_context_bridge;` to its existing `src/lib.rs`, since native/hqkg already has optional `vivim-programmability-core` via feature native-program. This prevents adding a dependency from core back into HQKG.
3. The source-owning Host/graph adapter must construct `LensRead` from **current actor permissions**: exact allowed entity IDs, exact allowed source refs, exact entity->page IDs, source scope, wiki revision and graph epoch. Do not trust client-provided values. Retain empty/ambiguous outcomes when no canonical mapping exists.
4. Call the existing `hqkg::context_retrieval::query_scoped` with trusted scope, ordered seed IDs, exclusions, watermark, and `read_budget(depth)` limits. The adapter only copies its already-selected items/edges; it performs no traversal.
5. Run `project_authorized_context`, then feed approved current `SentenceClaim` values (with independent semantic-verification status) to `narrative::realize`. HQKG provenance alone establishes an indexed occurrence, **not the truth of an application capability**. Do not elevate source annotations to Verified without independent semantic evidence.
6. Expose only a read-only, revision-fenced result through an EXISTING owner-authorized Host read port. Do not invent a public REST path. The PWA consumes it via the Dev ChatGPT's stabilized live composer/wiki interface, keeping the same URL and one textarea. Browser displays strings with textContent, never HTML.
7. On empty or stale context, display a truthful empty/status state. Never retry writes, start a command, call a provider, or switch to a non-scoped cache. A trace view needs source-owner developer authorization, not a client toggle.
8. Route `quality_feedback::review_candidates` into the existing owner review / task-creation path only after a separate explicit owner decision. It must never mutate VLANG or Wiki sources automatically.

## Build/test commands (NOT EXECUTED IN THIS PREP ROUND)
```sh
cargo test --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --all-targets
cargo fmt --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --check
cargo clippy --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path core/programmability/Cargo.toml --lib autowiki
cargo test --manifest-path native/hqkg/Cargo.toml --features native-program --lib
```
The standalone acceptance tests use fabricated public IDs, never synthetic product-state assertions. Real Host API, live source mapping, physical Windows/Android behavior and GitHub integration tests remain **UNRUN**. Do not mark READY TO DEPLOY until they pass.

## Highest-priority unresolved code decisions (verification, not product decisions)
- The exact canonical equivalence between Host Wiki page IDs (e.g. `wiki:page:`) and current HQKG graph node IDs.
- Where the Host's approved entity/source allowlists are exposed to the existing HQKG query-scoped adapter; prefix matching alone is not sufficient per-item authorization.
- Whether an independently verified VLANG/semantic claim receipt is already available to populate the narrative's Verified field. If not, truthful source-based cards may render, but assertive English is gated.
- The first actual full Rust build of these staged modules. Current ChatGPT cloud environment has no cargo/rustc and cannot fetch binaries; tests are present but **unexecuted**.
- Phone receives no special access to owner-private source-index routes. A permitted user-context read must follow the existing scoped Host read path.
