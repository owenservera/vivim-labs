# Source compilation and incremental Wiki/Indexer topology
## Compiled flow
```text
CODING AGENT edits ordinary first-party source
  └─ minimal v1/v2 comments + optional future v3 ref tags
       ↓ exact commit SHA / tree hash / Host-approved development scope
HQ INDEXER + safe walker (existing): files, imports, exports, symbols, docs
       ↓ (no source execution; size/path/symlink/secret limits)
HQ annotation v1/v2 lexer + opt-in v3 parser (one parser pipeline)
       ↓ strict real comment boundaries, file-vs-symbol anchors, diagnostics
SEMANTIC REGISTRY + explicit owner-catalog/contract crosswalk (existing)
       ↓ typed referential integrity, no new owner minted
SCF compiler: declared claims JOIN static observations & test/receipt evidence
       ↓ immutable Semantic Capability ABI with provenance + uncertainty
Existing HQ SQLite / native HQKG read projection / version-pinned HVI snapshots
       ├── capability passports + machine query
       ├── existing incremental Wiki generator human pages
       ├── HVI N-way deltas / loss reports / source lineage
       ├── agent context packs and change-impact previews
       └── TierFlow/UI and AutoWiki inert, bound read-only projections
                      (Host checks real current permissions at use time)
```

## Minimum incremental algorithm
1. Commit pin created: record exact source tree and approved scope; no implicit moving `main` accepted. For an uncommitted local edit, treat it as `WORKSPACE_EPHEMERAL` with distinct digest; never relabel as Git commit.
2. Index delta: for changed tracked first-party files, parse only changed bytes; cache `(parserVersion,blobSha,language,registryDigest)` outputs. Invalidated source units only; unchanged unit ids/edges retained.
3. Resolve semantic IDs once from current registry projection; stale/deprecated/unknown/alias conflicts flag typed diagnostic; no update to registry authority.
4. Compare declared memberships with **inferred** imports, exports, symbol calls, contracts and test mappings, reporting mismatches with exact source span.
5. Invalidate downstream dependents only when public contract/semantic identity/owner/status changed. Bounded breadth-first graph walk with explicit truncation and freshness.
6. Compile immutable capability passport objects keyed by `(repoSha,unitId,capabilityId,ABI schema,digests)`. Preserve previous snapshot; no destructive replacement.
7. Update wiki pages, architecture view and HVI snapshots **via existing wiki/indexer and version engine**; no competing writer. Wiki pages are read-only and diffable.
8. Runtime receipt arrival updates evidence projection only, **not the source commit**. Consent revocation invalidates access/cached disclosure on every read.
9. On low disk/ENOSPC keep last verified snapshot and emit `incomplete`; never discard Host World originals/approval/receipt logs to make space.

## Agent development hooks (proposal)
- On file creation: template only fills known registered IDs; if unknown mark pending classification in PR metadata, do not hallucinate.
- On save: IDE local nonblocking parser feedback on changed unit, no full git walk every keystroke.
- On commit/PR: deterministic index+semantic check, source link/digest and contract/type crosswalk. Return diagnostics and semantic diff; branch cannot claim new capability as runtime without observations.
- In CI: compile pinned full manifest, run available static and actual tests; compare with preceding accepted ABI; high-risk breaking changes require owner review. Emit signed/hashed artifact/receipt referencing test exits.
- On app open: use already compiled, verified snapshot and exact current Host permission/read scope; no synchronous full-repo compilation in UI hot path.

## Performance and resource bounds
Aim for `O(changed_source_units + affected_dependents)` after initial census. Parsing bounded by file byte cap, graph traversal by node/edge/depth, paginated wiki/report query and revision cache size. No fixed performance SLA can be asserted before benchmark. Rust native pure compute is optional acceleration under one existing HQKG engine, not new source of identity.

## Security
Comments, Markdown, Git paths and retrieved documents are untrusted inputs. No code `eval`, shell interpretation, dynamic provider APIs or unreviewed remote execution. Reject malformed tags, path traversal, symlink escape, secret references, hostile instructions embedded in comments, graph cycles exceeding bounds, hash collision/corruption. Source reads require exact Host-approved development-root grants where product integration needs them.
