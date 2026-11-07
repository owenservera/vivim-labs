# Lane A — deterministic reader, validator and serializer protocol
**Status:** Design contract, NOT installed provider instructions or a deployed validator.

## Normative read pipeline
1. Pin the source: resolve the file's (repo, path, Git blob SHA) from `FRONTMATTER-217-WORK-MATRIX.json`; verify matching source bytes and matrix revision. Reject `E_SOURCE_SHA_MISMATCH` or `E_SOURCE_REVISION_STALE`.
2. Choose the format by exact observed type and path, **never** by Markdown extension alone. 52 `SKILL.md` and 3 `.opencode/agents/*.md` are conditional native YAML candidates; 151 other Markdown entries are sidecar-first; 11 JSON files keep the original JSON body. Reject wrong routing as `E_ADAPTER_MISMATCH`.
3. Parse the inert sidecar with strict duplicate-key detection, UTF-8, size limits and JSON Schema draft 2020-12. Unknown metadata is a refusal, not silently stripped (`E_UNKNOWN_METADATA_FIELD`). No YAML untrusted tags, aliases or arbitrary object construction.
4. Build section links with unchanged `section:NNN:NNN` identity, original heading and line. A null classification stays null until the full original source span and family-specific meaning are reviewed (`E_UNEVIDENCED_SECTION_CLASSIFICATION`). Reject `E_SECTION_ID_DUPLICATE`.
5. Apply **current** external owner policy, principal/task/project scope, destination and disclosure checks as hard filters. Historical files, donor snapshots and sidecars never grant permission. Private source bodies remain in Repos HQ (`E_PRIVATE_SCOPE_DENIED`). Source metadata alone cannot authorize effects (`E_OWNER_AUTHORITY_CONFLICT`).
6. Optionally emit provider-native **non-effect** metadata only after a target-local loader/version receipt proves exact acceptance and current owner policy allows modifications. OpenCode's `permission` and Skills' experimental `allowed-tools` are not generated from source metadata. Unknown keys rejected (`E_NATIVE_KEY_UNSUPPORTED`).
7. Produce only a versioned, normalized, non-authorizing IR projection. No automatic provider send, new live file, tool invocation, owner grant, or in-flight prompt rewrite.

## Canonicalization and invalidation
- Identity is `fileId` + pinned `sourceRepository/sourcePath/sourceBlobSha`; `targetId` is separate from mutable display names.
- Normalize JSON deterministically: schema's declared field order; nested keys sorted; UTF-8; no incidental whitespace; preserve order for semantically ordered arrays, sort sets only when declared set-valued; no absolute local paths.
- Record `sha256` of normalized IR **excluding** `changeControl.normalizedDigest`. Hash rules must be checked by E on equivalent serializations.
- DCB/LRU key: `sourceBlobSha + schemaVersion + adapterRevision + policyRevision + graphEpoch + principal + destination + project/role/task scope`. Recheck current policy and source freshness on cache hits **and** misses. Revocation, new SHA, changed graph epoch or expired evidence invalidates. Deny stale cache as `E_STALE_POLICY_CACHE`.

## State gates
`metadata-proposal` ≠ `designed` ≠ `parser-validated` ≠ `live-observed`. A design commit proves none of the last two. `live-observed` requires independent runtime test receipt and a separately verified owner approval receipt (`E_LIVE_WITHOUT_RECEIPT`).

## Refusal codes
`E_REQUIRED_FIELD`, `E_UNKNOWN_METADATA_FIELD`, `E_SOURCE_SHA_MISMATCH`, `E_SOURCE_REVISION_STALE`, `E_ADAPTER_MISMATCH`, `E_NATIVE_KEY_UNSUPPORTED`, `E_UNEVIDENCED_SECTION_CLASSIFICATION`, `E_SECTION_ID_DUPLICATE`, `E_OWNER_AUTHORITY_CONFLICT`, `E_PRIVATE_SCOPE_DENIED`, `E_NONAUTHORIZING_CONSTANT`, `E_LIVE_WITHOUT_RECEIPT`, `E_UNSAFE_SOURCE_PATH`, `E_STALE_POLICY_CACHE`.

## Verification boundary / handoff
Fixtures in `fixtures/A-contract-cases.json` define 3 expected-positive and 12 expected-negative cases, **not executed test receipts**. Lane E must run real JSON Schema validation, adversarial parsers and semantic checks independently; B/C/D must not claim loader compatibility or current authority based on filenames. All 168 unclassified headings remain unclassified pending source-span review; all 217 adapter routes are design-only. Native provider load, Android/Windows use and any Host effect remain untested.
