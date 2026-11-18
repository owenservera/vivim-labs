# HVI A — source contract and B/C/E/F interface

Schema: `arbor.hvi.source/1` and `arbor.hvi.manifest/1`. Node ESM exported from `hq/version-intelligence/sources/snapshots.mjs`, with adapter in `adapters.mjs`.

```js
const adapter = createGitObjectAdapter({ resolveRef, readCommit, listTree, readBlob });
const authorize = async ({operation, scope}) => liveDeveloperGrantCheck(operation, scope); // H owns real Host check
const pin = await resolvePin({
  repo: 'owenservera/repos-hq', provider: 'github', ref: 'main',
  approvedScope: 'development', sourceGrantRef: 'opaque-developer-grant'
}, {adapter, authorize});
const manifest = await snapshotAt(pin, {adapter, authorize});
// A moving main does not change manifest.pin.resolvedCommitSha.
await verifyPin(pin, {adapter, authorize}); // checks moved main -> PIN_MOVED
const bytes = await readPinnedBlob(pin, 'hq/lib/git.mjs',
  {adapter, authorize, manifest, maxBytes: 1500000});
```

**Adapter method payloads:**
- `resolveRef({repoIdentity,ref}) -> {commitSha}` or null. Only called during explicit resolve/verification, **never during snapshot**.
- `readCommit({repoIdentity,commitSha}) -> {sha,treeSha}`. Exact commit and tree checked against immutable pin before snapshot/blob read.
- `listTree({repoIdentity,commitSha,rootTreeSha,cursor,pageSize}) -> {verified:true,entries,nextCursor?,truncated?}`. Flattened Git tree entries with `{path,type:'blob'|'commit'|'tree',mode,sha,size?}`; adapter must tie every verified page to supplied commit/root tree. Tree directory records ignored; Git link mode 160000 and symlink 120000 retained.
- `readBlob({repoIdentity,commitSha,blobSha,maxBytes}) -> Uint8Array|Buffer`; validated by git blob SHA-1 and byte bound. Strictly no fallback reads.

**Pin identity** is `sha256(canonical({provider,repoIdentity,commitSha,subPath}))`; normalized GitHub owner/repo is case insensitive; same repo at three commits gives three different keys. `requestedRef` and `acquiredAt` are provenance, excluded from identity/digest. Resolve requires explicit `approvedScope:'development'` and fresh `authorize` callback approval. A pin has `manifestDigest:null` until a snapshot. Snapshot returns `manifest.pin` with the actual digest and completeness.

**Manifest** has sorted `entries` with `path,mode,blobSha,bytes,kind,parseStatus,excludedReason,symlinkTarget`, plus `pinId,repoIdentity,resolvedCommitSha,rootTreeSha,subPath,observedEntries,omittedMinimum,completeness,truncated,errors,manifestDigest`. Digest is SHA-256 of canonical semantic manifest payload, excluding wall clock and optional pin provenance. `partial` when page/entry budgets run out; no inferred deletions from partial manifests. Unsupported type/path, duplicate path, page conflicts, bad blob and tree drift fail closed.

**Safety:** no network adapter in this lane; no donor scripts executed; no current working-tree read; no symlink resolution; no LFS dereference. Inspection of content is opt-in (`inspectContent`) and bounded, with status `uninspected` otherwise. Sensitive-looking paths never undergo byte reads. A cloned adapter alone grants no Host access; caller must pass a live authorization implementation at **each operation**. Paths are opaque untrusted source data; no interpolation of source text in errors.

**Errors:** `PIN_NOT_FOUND`, `PIN_MOVED`, `SOURCE_UNAUTHORIZED`, `SCOPE_ESCAPE`, `INCOMPLETE_SCAN`, `DIGEST_CONFLICT`, `ID_CONFLICT`, `RESOURCE_LIMIT`. Structured exceptions have `code` and a non-sensitive `detail`.

**Known coverage limits:** Git object SHA-1 only (40 hex), verified *flattened* tree pages supplied by adapter, no implicit GitHub token or local source-root access, no actual Host permission implementation, and no full semantic parity or runtime verification. G1 tests integrated source adapters; H owns Host live grants; F owns independent conformance.
