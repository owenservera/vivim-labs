# VIVIM Application-Surface Intelligence — architecture and standardization

## 1. Design objective

Turn Craft's evidence-backed observation of a webapp into source-qualified knowledge that tells Studio/Harness **which surfaces exist, which controls and states have evidence, what actions are only inferred, what source changed, and which capabilities remain missing**. Do not confuse a discovered control with a verified authorized operation. Provide compounding development leverage without another registry, graph, journal, provider API or effect owner.

## 2. Current data path (implemented here)

```
Craft MCP/GUI session (own private Chromium; permitted target only)
  ├─ web_site_capture → <capture-dir>/capture.json, README.md, PNGs
  ├─ web_page_states_capture → <states-dir>/states.json, PNGs
  └─ core_evidence_export → <evidence>.json (format=v1)
                                  ↓
Labs src/cli.mjs + src/adapter.mjs (NO browser/CDP)
  1. Read bounded JSON (32 MiB bundle max, 2500 records/unknowns)
  2. Verify Craft evidence content-ID (fail closed on mismatch)
  3. Verify exact operator-supplied target origin, reject mixed origins
  4. Read optional artifact only if absolute real folder equals Craft evidence result
  5. Whitelist structural counts, UI roles, status transitions, and narrow optional labels
  6. Emit read-only atlas.json / coverage.md to a fresh developer output folder
```

**Not implemented here:** ingestion by ARBOR Studio, upload to HQKG or any form of learned adapter promotion. The source output is a *proposed read model*, not a new authoritative product store.

## 3. Target IR / stable identifiers

The current JSON output version is `vivim.application-surface-atlas/0.1`. It includes `target` (operator-declared `siteId`, exact `origin`, `ownedTarget`), `source` (Craft bundle identity, optional exact SHA-256 of bytes, counts), `evidence` (verified Craft IDs and allowed qualifiers only), `observations` (page, state and walkthrough summaries), `controls` (bounded structural roles, filtered optional label) and `unknownIds` (IDs only), and a hardcoded inert `authority` block.

**Identity rules:** `craft.evidenceId` is *a content digest*, not a VIVIM semantic operation ID; `siteId + origin` scopes an investigation, and a future `surfaceId` must be independently versioned and owner-reviewed. A selector, role label, route, account name or screenshot file path must never mint a product action ID. Distinct concepts: target, application surface, evidence record, candidate operation, provider binding, Host account/grant, and effect receipt.

**Source integrity vs permissions:** Craft evidence ID authenticates content consistency only. It is not a signature or authorization token. The `ownedTarget` flag is a local user/operator declaration and never an actual Host grant. Real Studio intake requires its existing Core-qualified capture-verification procedure. No newly imported record receives a fake Studio evidence class such as `WINDOWS_LIVE_ACCEPTED`.

**Coverage semantics:** `complete` in Craft means complete over *that tool's local inspection horizon* (e.g. captured widths or first N menus), not a proof that the whole webapp surface was inspected. No control existence proves operation semantics; an unexplored feature is `unknown`, not absent. A screenshot can show a UI state but not guarantee exact side-effect behavior.

**Privacy:** source bundle and capture folders may contain content or paths derived from target pages. The importer does not copy raw `result`, full URLs, conversations, CSS, HTML or images. Output is developer-only, not automatically uploadable to GitHub. Label inclusion requires `--owned-target --include-labels` and short free-text labels are redacted against common secret/PII patterns. Operator must review output before sharing; this is not a comprehensive DLP engine.

## 4. Later ARBOR ingestion (handoff, not implemented)

The future ARBOR adapter **should not** call `studio/src/studio/adapter-evidence-intake.mjs` with fabricated Studio `captureId`/`sessionId` or approve its own `verifyCapture`. Better pattern:

1. ARBOR Dev Intelligence exposes an explicit *untrusted external evidence* intake that verifies bundle hash and source permission separately. It treats `sourceProducer: craft-anything` as external and cannot claim Studio live-capture acceptance.
2. Map only the subset of findings with exact match to existing chat features into proposed adapter probes. For richer surfaces create a distinct **read-only Atlas projection**, not a second adapter store.
3. Associate proposals with current site profile revision, existing Studio/Harness source location, and exact captured evidence reference; perform held-out runtime checks before promoting a learned read-only adapter through the **existing** factory.
4. Reuse current HQKG journal/index for approved metadata-only nodes, with correct owner namespace. Do NOT send raw Craft records through `hq/lib/kg-export.mjs` as though they came from HQ source indexing; do NOT use `hqkg-web-feed`'s visit-only API for arbitrary UI observations.
5. Keep Host approval/receipt and signed-in browser effects exclusive to the incumbent authority; generic planner and Rust pure kernels may compute candidate previews but cannot approve/send.
6. PWA-first: Studio and the mobile Wiki read a revision-aware projection; GPUI is an optional client, never the only developer viewport.

## 5. Enrichment lifecycle

`source-observed → normalized-inert → mapped-candidate → reviewed → held-out-verified → read-only-preview → Host-wired (separately approved) → live-effect-verified`

Every transition must preserve its own evidence and never borrow a stronger status from the preceding phase. Negative cases: stale revisions, mixed origins, cross-account source, contradictory selectors, partial captures, missing screenshot artifacts, revocation, rerun, and send uncertainty.

## 6. Why this is compatible with Rust convergence

The Labs bridge is isolated Node 22+ with no packages. It does not modify ARBOR's heavily edited Rust kernel, HQKG importer, Studio channels, mobile relay, VEC, or Host. The *next* migration target is purely deterministic normalization and contradiction resolution after a real bundle demonstrates stable field semantics. Native Rust should never embed Craft's browser actor or become a second Host effect authority.

## 7. Hard limitations of the current implementation

- The app atlas currently handles bounded screenshot/count metadata, structural outline controls, state labels/transition categories, and walkthrough action/status summaries. Full site schemas, route graphs, responsive token catalogs, screenshots and actual learned execution recipes are future read models, not implemented.
- Imported artifact files are locally digested but are not attested by a Craft-signed manifest; the recorded folder match prevents simple swaps, not adversarial filesystem tampering.
- The `--owned-target` flag is not a source-authorization verification. Upstream investigation permission is a separate human/operator responsibility.
- No Rust `craft_anything_evidence` cross-implementation hash vector executed yet: do not mark release-ready without that parity suite.