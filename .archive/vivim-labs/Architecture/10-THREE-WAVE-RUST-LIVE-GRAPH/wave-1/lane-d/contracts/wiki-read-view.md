# D2-05 read-only integrated wiki view contract — proposed, NOT ACTIVATED

**Owner boundary:** `repos-hq/hq/lib/wiki.mjs:buildWiki` is the sole existing HQ wiki renderer and writer. This Lane D proposal cannot modify it and is not a second writer. Only the HQ wiki owner may approve integration of a graph-derived **read-only** view. E owns the Rust HQKGX1 journal; B/SQLite keeps canonical KB and semantic revisions; C/Host own command status and effects.

Proposed input (NOT an implemented HQ builder parameter):

```json
{"schema":"vivim.wave1.wiki-read-view/PROPOSED","projectId":"vivim","indexIntentRevision":1,"graphDigest":"<64hex>","sourceRootHeads":[{"id":"hq-root","commitSha":"<40hex>"}],"entities":[{"graphId":"hqg1:<64hex>","sourceId":"a:<id>","sourcePath":"hq/<path>","contentSha256":"<64hex>","semanticId":"<versioned-id>","description":"<redacted accepted description>","status":{"declaration":"declared","registry":"current","handler":"source-present","runtime":"unverified","device":"unknown","authority":"read-only"},"links":[{"target":"hqg1:<64hex>","relation":"HAS_REGISTERED_HANDLER","evidenceClass":"declared"}],"wikiPageId":"<HQ-owned stable page ID>"}],"scope":"dev-only-read-projection"}
```

Above values are **examples**, not permission grants or evidence that such an input is currently wired. The generator may accept an approved adapter instead of exactly this schema; preserve all meanings and authority states, never infer them from text matches. To validate produced pages, request a second data-only page manifest **from the same approved builder**: `graphDigest`, `writerSourceSha`, `pagePath`, `pageSha256`, `entityGraphIds[]`, `sourceRootRevision`. D computes actual generated-page hashes and checks them against the manifest; a self-declared page is not independently trusted until F reruns generation.

**No second writer**: D may examine outputs and create ephemeral JSON diagnostic reports outside the canonical wiki tree. It may not render replacement `.md` wiki pages, generate HTML, register actions/semantic IDs, activate wiki-program configurations or produce a competing data-feed registry. If E's crosswalk is absent, the result is `BLOCKED_CONTRACT_CROSSWALK`, not an invented page.

**Acceptance after master and owner consent:** one complete real code→@hq:wiki→semantic+KB→registered action→Rust query→page crosswalk; matching IDs, content hashes, source line and status evidence; no-op, stale/removal and F independent witness. Inert command syntax appears as documentation, not clickable action authority.
