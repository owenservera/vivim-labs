# HVI U final test truth and cross-lane owner handoff

**Status: PREPARED_NOT_PROMOTED.** U source head **`80448f871a3658726382f09a8d57a9b03beb671d`**, starting owner-supplied U base `bc9e2dcc5eca44cfe8d4bcec31e9deb513c0a7d4`. No U-branch drift before edits. Changes are restricted to `packages/tierflow/**` and `proofs/U/**`.

## Source executed vs tests executed

- **Executed isolated V8 inspection:** 21 passed in-process JavaScript assertions on actual GitHub-fetched U JS source at `80448f871a3658726382f09a8d57a9b03beb671d`, with canonical document validator and renderer **stubbed**. Covered redaction, empty/request/progress/cancel/revoke/stale/partial, 16 synthetic pins, feature-off, bounded node labels and child counts, changed target refusal. This was **not** a Node test command, has **no OS process exit**, does not establish real validator/renderer or PWA acceptance.
- **UNRUN** `node --experimental-strip-types --test packages/tierflow/tests/hvi-inspector.test.mjs`; exit **not observed**. Committed tests cover synthetic 3-SHA same repo, five invented repos, 16 invented pins, limits, redaction, stale publisher revision, denial/revoke, inaccessible live report, phone/desktop semantic viewports.
- **UNRUN** `bun test packages/tierflow/tests/visual-runtime`; exit **not observed**.
- **UNRUN** `bun run --cwd packages/tierflow typecheck`; exit **not observed**.
- **UNRUN** local real-Host grant/revoke, G1 combined Node24/AJV, browser and physical Android TalkBack / Windows screen-reader checks; no exits.

## Current G1/H integration truth

- Latest observed G1 candidate `69447c09f420310aa84c24a1908dede91c963ec8` diverges from this U source fork. Main observed `3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298` is **not an accepted G1 integration baseline**. U does not merge or rebase.
- Latest observed H branch `d24daa9966abb222cd25210f41dcbb4758aa1209`. Its `control/hvi-capability.mjs` declares `versions.compare.pinned` with `enforcement_missing`, no executable catalog entry, and an injected local grant dependency. The output `vivim.host.hvi-safe-report/1` contains safe summary, digest and pin metadata, **not** the full `arbor.hvi.adapter.comparison-report/1` needed for target-relative detail rows. This is a typed H/G2 reconciliation task, not permission for U to query HQ or cache reports.
- U is a read-only canonical `vivim.ui-document/1` producer using existing `validateUiDocument` then `renderUiDocument`; it creates **no** UiCore/UiStore/publisher, source root, network path, approval writer, apply/merge executor or personal-World read.
- Every source report remains **synthetic-preview-only**, even if a caller supplies `permission='granted'`. Active live report feature flag is **false**. Existing canonical publisher owns revisions; this adapter refuses mismatched supplied current/expected revisions; full live publisher mounting and real Host-grant enforcement must be verified by G2.

## Next owner / blockers

1. **G1 / CI:** execute Node24 synthetic conformance, AJV and G1 security closure with actual process exits; accept G1 into main by prescribed gate only.
2. **H + Host security:** provide a real development-source grant provider, per-invocation + revoke verification, approved typed detail projection (not personal paths/text), and accepted native capability dispatch.
3. **Z2 / G2:** reconcile H safe-summary vs G1 detailed report contract; mount U in existing publisher without changing UiStore/CAS, test real canonical validation and denial/revocation, assess phone accessibility and explicitly decide whether to enable. No automatic comparisons, source migration verdicts or beta GO.

No real ARBOR, donor or Labs comparison was executed. No main merge or remote access was enabled.
