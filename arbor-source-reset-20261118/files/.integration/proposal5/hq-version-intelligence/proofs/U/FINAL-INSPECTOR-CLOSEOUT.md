# HVI U final inspector source closeout — 2026-11-12

Status: **PREPARED_NOT_PROMOTED**. Existing U branch base was exactly `bc9e2dcc5eca44cfe8d4bcec31e9deb513c0a7d4` (no newer branch commits at implementation time). Owner waiver base `bbe7e4896eea02bcdec6f8fedb66eab885250161` is a G1 integration **candidate** and is NOT accepted main.

## Source graph and interface
- Existing TierFlow projection: `packages/tierflow/src/visual-runtime/knowledge/hvi-inspector.mjs`. Hardened text/path/secret redaction; target-relative selections refuse to recompute findings for a new target.
- Existing canonical validator: `packages/tierflow/src/visual-runtime/document/validate.ts`.
- Existing one semantic renderer: `packages/tierflow/src/visual-runtime/render/dom.ts`. New `render/hvi-inspector.mjs` builds a bounded canonical `vivim.ui-document/1` and passes it to the SAME validator and renderer. No extra visual publisher, UI server, UiCore, UiStore, graph, browser sender or effect writer.
- Existing publisher guard remains owned by `packages/tierflow/src/ui/commands.ts` `UiCore` / `src/ui/store.ts` compare-and-swap. U's read-only adapter requires `expectedRevision===currentRevision` from the consumer and fails closed when stale; it does **not** mint a publisher revision.
- HVI inputs: G1 `hq/version-intelligence/integration/comparison-report-adapter.mjs` envelope, 2–16 immutable pins, same-repo multi-SHA, target-relative coverage, evidence refs, unknown/unverified semantics, truncation, report paging.

## Feature and safety
- Feature `tierflow.hvi.inspector`: **disabled**; `liveResultRendering=false`; no remote access enabled.
- Only an explicit `synthetic=true, previewSynthetic=true` fixture path may render comparison metadata. Caller-provided `permission='granted'` NEVER enables a live report.
- States: empty, request-access, denied/revoked, progress, cancelled, stale, partial, source-report, unavailable. Read-only target/version selector projection and `expandedRowIds` for local provenance details; no Host calls, independent authorizer, cancel side effect, migration command, export action or apply path.
- No actual ARBOR/VIVIM/Labs source comparison, migration classification, personal source read, provider API, Playwright or Puppeteer.
- Read-only canonical screen-reader document and 320px/390px/1280px synthetic viewport tests. Physical PWA/TalkBack **UNRUN**.

## Tests and remaining gates
Command on a Node runtime with native TS type stripping: `node --experimental-strip-types --test packages/tierflow/tests/hvi-inspector.test.mjs`. Requires full repository checkout, Node >=22. Exits must be recorded separately if executed; source presence is **not** a PASS. Full Bun/TS typecheck, native G1 combined tests, Host per-invocation grants/revocation, real publisher mount, physical Windows/Android and beta security gates must not be claimed from U code.

H / G2 handoff: reconcile H's typed Host result with G1 report bridge, source grants and revoke on every invocation; only after live Host review may G2 intentionally enable the inspector. Z2 independently checks no duplicate publisher, stable current UiCore revision guard and denial/revocation evidence. G1's unaccepted source base must reconcile to accepted main before G2. NO main merge by U.
