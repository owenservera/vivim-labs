# R006 Host grant source audit — exact reuse and remaining gap

**ARBOR main inspected:** `7ae24ff78a797581b510fe7a78105bbe301c0de4`, source file hashes available on GitHub. This is code inspection, NOT a live runtime grant lookup.

## Actual observed authority boundaries

| ARBOR source | Actual trust contract | Safe reuse | Missing for R006 |
|---|---|---|---|
| `apps/vivim-host/server.mjs`, `handleControl`, `developmentStillOwner` | Authenticated desktop-owner token or owner UI session and mutation/CSRF rules | Reuse as **actor/session validity** and pre/postflight authorization | Does **not** return complete HQKG entity/source allowlist |
| `apps/vivim-host/control/development-read-port.mjs`, `createDevelopmentReadPort` | Owner-only approved source roots and HVI repository selection; rechecks `stillOwner()` | Reuse existing owner-only **development** read authorization; never expose raw source path to phone | No native personal Wiki/HQKG grant envelope or namespace entity grant |
| `apps/vivim-host/control/hvi-source-grant.mjs` | Existing ledger meta grant with expiry/revocation for selected **GitHub repositories**, not graph entities | Reuse vocabulary/pattern: expiry, revocation tombstone, re-check every read | **Do not reinterpret HVI repository grant** as permission to a personal Wiki graph node |
| `apps/vivim-host/direct-phone.mjs`, `authorizeDevice` | Paired-device `control:read`/`control:execute` scopes, revocable through enrollment | Reuse existing device permission and Host-owned read routing | Device control:read is not a blanket owner-private development/HQKG entity grant |
| `native/hqkg/src/scoped_graph.rs`, `prefix` | Deterministic development/personal namespace prefix, provenance and revision validation | Reuse exact namespace-scoped native graph representation | Documentation and code do **not** authenticate the human/agent caller; prefix is not ACL |
| `native/hqkg/src/context_retrieval.rs`, `query_scoped` | Bounded native BFS excludes source-owner supplied entity IDs and enforces prefix | Reuse; R006 builds a verified **complete** exclusion set for this incumbent function | Query code cannot invent authoritative Host per-actor allowlist |
| `hq/lib/context.mjs`, `allowedSourceRefs` | Optional caller-verified index source-filter narrows development context | Reuse as source narrowing **after** authorization | Does not grant source access on its own |
| `.integration/proposal5/phase2/pre-g/K-knowledge/proposed-ports.d.ts`, `HostSourceGrant` | Proposed typed source grant | Design reference only | Not proven installed/executable |

## Verified design conclusion

**There is no source-verified production adapter that transforms current Host actor/device grant into a complete personal Wiki/HQKG exact graph entity + source-ref grant.** Existing Host owner identity, HVI repository scope, device `control:read` and native graph namespace checks are different permissions. They must not be silently collapsed into one.

R006 is therefore a valid native **preparation** module but **not safe to activate** with a fake `CurrentHostGrantSource`, a browser-provided allowed list, a manually set `exclusions_complete=true`, or a broad prefix. The code fails closed unless the actual Host authority supplies the required complete current sets.

## Minimal next owning Dev change, no competing subsystem

1. Reuse the current Host owner session or device grant and current scoped source index **as owners**; request an exact source-bound permit read for the current Wiki page/entity set from the source that owns those records.
2. In the existing Host control/read path, compile a time/revision-bounded, per-actor **existing-source projection**: `actor_id`, `grant_revision`, `scope_prefix`, `focus_entity`, exact `allowed_entities`, exact `allowed_source_refs`. Only use current already-authorized values, not a new general-purpose ACL engine. If real values are not available, return **UNAVAILABLE/403** rather than inventing a grant.
3. Let the native helper `derive_complete_coverage` enumerate current graph IDs, validate those exact permitted entities and derive all denials; no new graph or database.
4. Recheck the original Host grant before and after read and immediately before UI publish; revocation may happen without graph epoch increment. The native Rust R004/R005 readback guard is additional, not a replacement for Host reauthorization.
5. Unit and integration tests must include wrong actor, no source grant, private neighbor, revoked phone or owner grant, stale Wiki revision, changed grant with unchanged graph epoch, source mismatch and unchanged visible PWA URL after owner-approved integration.

**Ownership:** This is a Dev integration prerequisite, not permission for Planning to edit `apps/vivim-host/server.mjs`, `direct-phone.mjs` or ARBOR product source. No GPUI execution is authorized by this package.
