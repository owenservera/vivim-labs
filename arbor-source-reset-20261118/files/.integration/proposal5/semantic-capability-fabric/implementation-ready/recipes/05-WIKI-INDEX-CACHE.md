# Recipe C2 — incremental HQ wiki projection and generation lifecycle
**Owner C with B store seam.** Modify `hq/lib/wiki.mjs` only at read-model binding; preserve `buildWiki` CLI, hash behavior and generated paths. NO new wiki generator or separate static site.

## Existing contract
`buildWiki({ db, root, projects, outDir='wiki', force=false, maxFilePages=5000, graphIndex=null, persistHashes=true})` loads model from SQLite and deterministically renders project/file/capability/semantic pages. Legacy `loadCapabilityModel(db)` concerns package capability scans, not feature semantic passports. Avoid confusing the two.

## Read-only optional parameter
Proposed additive parameter `capabilityPassportModel=null` (pre-validated by owner) or `passportReadPort` only if in same existing DB/scope. If null, existing wiki outputs must be **identical** to baseline hashes, including no new generated pages. With valid model, publish minimal generated pages under current wiki source-derived page IDs, e.g. `capabilities/<slug>.md` plus an index section; render exact source references and status/unknown labels. Do not create permanent pages for ephemeral UI screens.
```ts
function bindCapabilityView(baseModel, readPort, authorizedScopes, pinId) {
  const passports = readPort.list({pinId,approvedScopes:authorizedScopes,limit:boundedLimit});
  const safe = passports.map(sanitizeAndValidatePassport);
  return {...baseModel,scfPassports:safe,scfStatus:safe.length?"available":"not_indexed"};
}
```
Never accept unsafe HTML/Markdown inline in provenance or comment text; use existing wiki escaping helpers, always distinguish `testsDeclared` from `testsObserved`. On unauthorized read, omit passport contents instead of a stale cached page.

## Index and cache
- `indexProject` currently uses file size/mtime fast skip, then content hash, then symbol/import/annotation replacement transaction. SCF should subscribe to existing per-file hash changes through a bounded post-index adapter **without recomputing full workspace every keypress**. B/G can add a narrow queue hook only if source tests prove no duplicate traversals.
- Key per-file parse cache `(sha256(actual bytes),language,parserVersion,registryDigest)`; key higher aggregate `(pinId,sourceManifestDigest,ownerCatalogDigest,SCFSchemaVersion)`.
- On changed source unit: recompute that unit facts; invalidate affected feature passports and contract dependents only. On registry mutation invalidate references to changed ID; on Host catalog change invalidate action crosswalk but retain raw parsed source. On proof receipt arrival rebuild evidence projection independently from immutable source snapshot.
- Do not recompute all wiki pages for an irrelevant unchanged file; existing wiki hash state should preserve unchanged outputs.
- Accepted immutable generations persist; `WORKSPACE_EPHEMERAL` overlays cannot masquerade as commit snapshots.
- Low disk/ENOSPC => use prior accepted wiki/index snapshot if still authorized and label stale; never delete Host World originals or approved receipts.

## Acceptance
No SCF option => all preexisting `hq/test/wiki.test.mjs` goldens identical. Tagged two-feature file => two distinct passports, one file page; emitted pages sorted/digest stable. Revoked source => generated privileged detail unavailable. Untrusted comment HTML, source references and markdown escaped. 10k synthetic source units stress query pages, not O(repo size) for one edit. All measured times and memory captured, no invented target SLA.
