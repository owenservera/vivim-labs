# Source-neutral snapshot and connector contract — NO GIT DEPENDENCY

## Architectural principle
**Snapshot-based intelligence, not Git-based intelligence.** HVI's comparator already consumes deterministic manifests and projects source/evidence parity. Source acquisition must not make Git part of the customer product.

### Common normalized snapshot
A normalized source descriptor is `{sourceId, displayName, kind, locatorHint?, grantRef?, capabilityMetadata?}`.
A source capture is `{schema:"arbor.hvi.source-neutral-snapshot/1", sourceId, snapshotId, versionType, versionLabel?, observedAt, contentDigest, completeness, truncated, exclusions, provenance, entries}`.
A snapshot entry is `{normalizedPath, kind, size?, digestAlgorithm, contentDigest, semanticMetadata?, evidenceRefs?}`. `versionType` is one of `git-commit-verified`, `provider-revision-verified`, `archived-snapshot`, `local-captured-snapshot` or `unknown`; the last must be reported as unknown. For a verified Git snapshot retain separate optional `resolvedCommitSha`, `rootTreeSha`, and `gitBlobSha` **only if verified**. Never fill those from file timestamps or SHA256 content digests.

Stable snapshot IDs are computed over canonical scoped metadata and ordered entries; source provenance and authorization receipts are attached separately so source contents aren't altered by grant renewals. Content SHA256 is standard for all connector-neutral entries. A Git-object reader may additionally verify Git SHA-1/SHA-256 depending on repository object format.

### Acquisition adapters
- **Native folder/files:** OS file APIs already inside the Host/Tauri app; explicit chosen roots; bound scope, no traversal or symlink escape; capture immutable content digests and snapshot time. Does not promise repository history.
- **Archive/import:** user-selected ZIP/TAR or exported manifest; detect and constrain archive traversal and zip bombs, verify bytes; classify as archived snapshot unless version metadata is independently verified.
- **Existing saved snapshots:** derived, integrity-checked, cached data; record original grant/source and confidence.
- **Authorized connected source:** use a supported already signed-in application workflow or approved connector; no demand that a customer install a developer CLI. Respect platform capabilities, account permissions and revocation.
- **Optional developer version connector:** can read Git objects using an embedded implementation or an already-present `git` program, if the developer explicitly opts in. It is **not** shipped as a mandatory dependency or blocker for all customers. `resolveRef/readCommit/listTree/readBlob` still serve this adapter's specialized Git commit semantics.

### Bridging to existing HVI
1. Inspect `hq/version-intelligence/sources/{common.mjs,adapters.mjs,snapshots.mjs}`, `integration/source-manifest-adapter.mjs` and `compare/index.mjs` and retain versioned Git pin contract unchanged for genuine Git sources.
2. Provide an explicit **source-neutral -> compare-manifest bridge** instead of spoofing `arbor.hvi.manifest/1` with fabricated 40-hex SHAs, `provider:"github"` or fake `gitBlobSha`. If current comparator only accepts Git pinned manifests, introduce a genuine versioned `snapshot-manifest/2` adapter or source-neutral comparator envelope, with conformance tests. Do not weaken digest/pin validation for Git snapshots.
3. Distinguish `IDENTICAL_CONTENT`, `MODIFIED_CONTENT`, `EXACT_RELOCATION`, `MISSING_AT_CAPTURE`, `UNKNOWN` and `UNOBSERVABLE`; do not claim lineage/ancestry when snapshots lack it.
4. No new graph database, comparison engine, scheduling system, agent dispatcher or Host effect writer.

### Native comparison capability
The user says “Compare these two folders,” “Compare my project export against last month's snapshot,” or “Compare these Git revisions” (developer). The same agent discovers **available connectors**, asks for source access only when necessary, captures immutable snapshots and invokes HVI. No Git installation prompt.

### Conformance
- Test using ordinary files and an archive **with Git entirely absent from PATH**, and without any `.git` directories.
- Verify repeated snapshots/digests, moved/changed/omitted files, partial capture, revocation, malicious paths/archives and secrets.
- Test optional verified Git revisions separately without imposing Git on the ordinary file-source user.
- Explicitly label file-snapshot differences as unverified runtime/semantic equivalence.
- All reports identify their `versionType`, source digest and limitations.

**Anti-requirement:** Never treat `simple-git`/`isomorphic-git` and a `real-git-adapter.mjs` as a default product dependency; never tell customers to install Git to use HQ.
