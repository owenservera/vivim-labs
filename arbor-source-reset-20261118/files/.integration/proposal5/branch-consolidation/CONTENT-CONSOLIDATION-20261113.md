# ARBOR non-conflicting branch-content consolidation — 2026-11-13

## Summary
Source baseline `main`: `8a0c6fe66776a4978c650d69ccc2f9558024117f`.
Selective staging head before report: `b529ec809881b74d6df7fddf5727c061e5721e3d`.
Exactly **472 new files** were added: 446 under `.integration/`, 13 under `apps/`, 11 under `packages/`, 1 under `studio/`, and 1 under `.github/`.
Full recursive Git tree comparison: **0 modified existing blobs, 0 deleted existing blobs**, no truncated trees.

## Integration decision
Merged only new paths absent from main; existing paths (whether in product source or research) were never overwritten. Where candidate branches supply different versions of an existing file, retained main's version. Candidate branch histories remain recoverable through `archive/pre-cleanup-20261113`.

### Key new source (added, not runtime-admitted)
- `apps/mobile-relay/vec/`: deterministic compiler, UI adapter, taxonomy catalogs and registry; associated test files.
- `packages/vec-ui-engines/`: standalone catalog/compiler/registry/token/validator code and tests.
- `apps/vivim-host/public/mobile-first.{html,js,css}`: new mobile-first UI assets, plus dedicated test.
- `.integration/proposal5/`: docs, source registries, implementation-ready contracts, synthetic test/reference code, proofs, foundation and Phase 2 research.

### Intentionally NOT imported because existing source paths differ
- `apps/mobile-relay/build-canvas.mjs`, `build-composer.mjs`, `sw.js`, `app.js`, `index.html`, `test/service-worker.test.mjs`.
- `apps/vivim-host/direct-phone.mjs`, `server.mjs`; existing `studio/README.md`.
- Existing HVI Host `control/hvi-capability.mjs`, `control/hvi-native-guide.json`, `test/hvi-capability.test.mjs`.
- Existing TierFlow `visual-runtime/render/hvi-inspector.mjs` and `tests/hvi-inspector.test.mjs`.
- Existing `.integration/proposal5/README.md` and `.integration/proposal5/phase2/README.md`.
- VEC PWA source edits duplicating above filenames.
No source-path collision was automatically resolved; related new modules may require subsequent deliberate wiring to function.

## Truth / tests
This merge is **additive source consolidation**, not release acceptance. GitHub tree existence and blob SHA comparisons verified only. Node, Bun, Cargo, Windows, device/PWA and integration tests: **UNRUN**. No claim of runtime-ready mobile-first Host or integrated VEC in active product.

## Recovery
All old tips are preserved in `archive/pre-cleanup-20261113` with `.integration/proposal5/branch-consolidation/BRANCH-ARCHIVE-20261113.json` and supplemental mobile-first manifest. Obsolete remote branch deletion is not part of this merge and remains separately blocked by GitHub connector capabilities.
