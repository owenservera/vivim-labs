# R005 — copy-ready four-application integration packages (Labs only)
Source pin ARBOR cc4f001fe835b1ceaa20e8cb45fa0353081f41ab; Craft e64d7cb73d7f39323b7040a984e0f9b3f7981146. Implements R004 P0/P3/P4/P5 preparation: target inventory with actual canonical IDs and source blobs, revision-bound reviewed surface coverage, noncallable typed operation proposals, change invalidation and existing Studio source index. No external dependencies (Node 22+). No ARBOR modifications, no runtime permissions, no inferred account binding. Unit inputs test contracts only, not live measurements.

## Workflow
1. Run existing Craft persistent investigation process in authorized scope. Export immutable bundle; use existing Labs importer with native verifier and artifact index. Keep all actual private data outside Git.
2. From this directory: node cli.mjs template /private/atlas.json /private/review.json
3. Fill review.surfaces with entries containing surface (see targets.mjs), status observed/partial/restricted/not-applicable, refs [{evidenceId,position}] (position null or omitted only for observation evidence, numeric for a specific control). A human reviewer classifies surfaces; no label guessing or automatic semantic identity. Leave unknown surfaces omitted. Do not change atlasDigest; recapture requires a new review.
4. node cli.mjs compile /private/atlas.json /private/review.json /private/proposal.json
5. Prepare an operator-owned JSON array of {siteId,atlasPath,previousAtlasPath?,reconciliationPath?}; node cli.mjs index /private/entries.json /private/studio-index.json. Set VIVIM_CRAFT_INDEX_FILE on existing Studio service when its owner approves; existing GET /api/v1/dev/craft-intelligence/targets and /targets/{siteId} already consume this index. Do not restart a busy production runtime.
6. node cli.mjs diff /private/old-proposal.json /private/new-proposal.json /private/delta.json
7. node --test compiler.test.mjs

## Integration placement (future ARBOR patch, not applied)
- targets.mjs: merge source identity projection into existing integration-target inventory/semantic registry; no second registry. Facebook is a social provider candidate. Existing provider profiles are source references only.
- compiler.mjs: copy pure proposal compiler into Studio development intelligence, reuse existing Craft route projection for ingress; extend existing API only after source-owner review. The SHA of the atlas is local content integrity, not an authenticated source receipt. A reviewer classification is not an independent verification, a binding, or an approval.
- studio-index.mjs: use unchanged in development tooling. No ARBOR route change necessary.
- operation proposal keys/inputFields/outputShape are design vocabulary, NOT official semantic IDs or JSON Schema. Next step bind to existing HQ IDs, strict typed schemas, trusted account grants and live Studio readback; preserve learned adapter read-only restriction. No arbitrary selectors or learned sends. Graph and AutoWiki projections must consume scoped qualified evidence through existing owners.
- Port pure normalization/coverage/delta only after Rust differential tests, license check and real capture parity. Keep existing Node compatibility.
- Full application completeness cannot be inferred from counts, review marks, or one account. Source metadata and operation proposals never authorize execution.
