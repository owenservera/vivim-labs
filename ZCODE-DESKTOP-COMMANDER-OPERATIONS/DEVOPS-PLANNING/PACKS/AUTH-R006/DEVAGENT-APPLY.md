# R006 DevAgent exact source integration

**Precondition:** Dev accepts this Labs handoff via existing `TASK-CLAIMS.md` and refreshes ARBOR `main` (last read `7ae24ff78a797581b510fe7a78105bbe301c0de4`). R003/R004 existing HQKG adapter and `read_authorized_context` must be reconciled first.

## 1. Copy only one new native helper into incumbent HQKG

```text
Labs:
  ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/PACKS/AUTH-R006/native_grant_coverage.rs

ARBOR target:
  native/hqkg/src/autowiki_grant_coverage.rs
```

Register beside the existing R003/R004 native bridge in `native/hqkg/src/lib.rs`:

```rust
#[cfg(feature = "native-program")]
pub mod autowiki_grant_coverage;
```

Do not add a new dependency, database, graph or permissions engine. The existing feature `native-program` already depends on `vivim-programmability-core`; the new module uses incumbent HQKG `Graph`, incumbent scoped retrieval and existing Rust query admission. TypeScript/Host may only bind the real actor grant at the trusted boundary, never perform another graph BFS.

## 2. Use the actual owner-grant authority

Implement `CurrentHostGrantSource` only through the existing Host Control owner/session permission authority. Before returning `CurrentHostGrant`, verify:

- Authenticated owner actor identity, current session/grant revision and exact source-scope prefix.
- Exact granted graph ID list and exact granted source-reference list, after any scoped source policy/consent checks. Do not infer access from a prefix, graph node title or phone-supplied ID.
- Current grant revocation state and whether the caller is still authorized to perform a read. Return `None` on unavailable/denied/ambiguous authority.
- The grant is independently read again during R006 preflight/postflight. Same source grant revision is necessary but not sufficient if a grant store reports revocation without incrementing; compare the full grant and re-check live validity.
- **Never** insert `CurrentHostGrant` into PWA JSON, logs, network response or a Wiki sentence. Source-bound checks belong inside the trusted Host/HQKG process.

The provided `StaticSource` / `FlippingSource` are **test-only fixtures**, not production permission sources.

## 3. Connect one existing HQKG read

`derive_complete_coverage(graph, host, actor_id)` scans current `Graph::nodes()`, checks every allowed entity exists and has only currently allowed source refs, then generates exact exclusions of every other in-prefix graph node. Only after the complete scan does `CompleteGraphCoverage::read_scope` set `exclusions_complete=true`.

`read_with_current_grant(graph, host, actor_id, request, depth, trusted_scope_id, trusted_wiki_revision)` then calls the **R004 existing bridge** and `query_scoped` once, compares the current graph epoch and re-reads Host grant before returning the bounded `LensProjection`. It does not publish browser UI or execute a Host effect. The source-authorized Host Wiki revision and actor scope remain separate from the graph epoch and grant revision.

After returning, before actual PWA/native display, the Host consumer still needs the R004 `readback_guard` plus a final current grant check. A grant can be revoked between query completion and UI paint without HQKG epoch advancement.

## 4. Refusal/performance policy

- No grant, scope prefix not explicit, actor mismatch, unauthorized/ghost entity, sourceless or private-sourced allowed node, >65,536 scanned graph nodes, >2,048 allowed IDs/sources, graph change, or changed current grant => **REFUSE**. No fallback to prefix-only read.
- The existing graph scan is temporary correctness-first code. If it becomes expensive, replace with **an existing authoritative HQKG namespace index** subject to the same complete coverage proof; do not create a second index or grant store.
- No need to involve Android, ADB, browser automation, provider sessions or any outside Model API to validate native correctness.

## 5. Tests and rollback

Copy [native_grant_coverage_test.rs](native_grant_coverage_test.rs) into `native/hqkg/tests/autowiki_grant_coverage.rs`. Run the commands from [VERIFICATION.md](VERIFICATION.md); record real exit and compiler versions.

If integration fails, remove only the new module/registration and the admitted Host binding with Dev's explicit coordination; preserve current HQKG, Wiki journal, PWA composer, actor permissions and all concurrent main commits.
