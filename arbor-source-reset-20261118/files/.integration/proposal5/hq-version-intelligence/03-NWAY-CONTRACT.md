# Normative HVI contract: pinned source evidence, N-way semantics and authority
## Interfaces / modules (the work items own implementations)
- `hq/version-intelligence/sources` A: `resolvePin(source: {repo, ref|sha, provider, approvedScope}) -> PinnedSource`; `snapshotAt(pin, options) -> immutable SourceManifest`; `verifyPin`, `readPinnedBlob` with bounded bytes.
- `hq/catalogue-identity` B: port pure original C4 crosswalk; `hq/version-intelligence/identity` provides adapter to stable source-scoped/canonical IDs, overlap candidates, source completeness and disputed equivalence.
- `hq/version-intelligence/compare` C: `comparePinned({pins: PinnedSource[2..N], targetPinId, pairwise, scope, budgets, identityDecisions?}) -> VersionComparison`; N-way source matrix and deterministic deltas. No optional implicit HEAD.
- `hq/version-intelligence/parity` D: source/capability/runtime reachability and donor-to-target lineage evidence; classify physical absence vs semantic migration.
- `hq/version-intelligence/report` E: deterministic JSON and Markdown exports, interactive-view projection without approval or effect.
- `contracts/hq-version-intelligence` / `tests/hq-version-intelligence` F: versioned schemas and conformance.

## Immutable pinned source
`PinnedSource {pinId, provider, repoIdentity, requestedRef, resolvedCommitSha:40hex, rootTreeSha:40hex, subPath?, manifestDigest, acquiredAt, completeness:complete|partial|blocked|missing, scope:development, sourceGrantRef?, evidenceRef}`. The **commit SHA** is identity for the revision; `requestedRef` is provenance only. `main` is never used as a comparison key after resolve. Same repo at three SHAs yields **three distinct pins**. Different repo roots never collapse on identical folder names.

`SourceManifest`: stable ordered entries `{path, mode, blobSha, bytes?, kind, symlinkTarget?, parseStatus, excludedReason?}`; optional symbols,exports,imports,tests,claims,contracts and capability records carry `{repo,commitSha,path,lines,blobSha}`. Integrity: source bytes hashed; digest over canonical sorted manifest. Explicit truncated/unsupported/blocked flags. Scan never treats Git dirty worktree as the pinned commit.

## States
Source: `EXACT_BLOB`, `SAME_PATH_CHANGED`, `EXACT_RELOCATION`, `SPLIT_CANDIDATE`, `COMBINE_CANDIDATE`, `SIMILARITY_CANDIDATE`, `MISSING_AT_TARGET`, `NEW_AT_TARGET`, `UNOBSERVABLE`.
Capability: `PRESENT_EQUIVALENT_CONFIRMED`, `PRESENT_CHANGED`, `IMPORTED_NOT_WIRED`, `SOURCE_WIRED_UNVERIFIED`, `RUNTIME_VERIFIED`, `DESIGN_ONLY`, `RESEARCH_IMPLEMENTED`, `INTENTIONALLY_EXCLUDED`, `BLOCKED`, `UNKNOWN`.
Evidence: `source_blob`, `source_symbol`, `contract`, `fixture`, `executed_test`, `host_receipt`, `device_receipt`, `owner_decision`. Neither docs nor compiler comments equal installed handlers. A `SIMILARITY_CANDIDATE` may never be silently promoted to equivalence or retirement.

## Comparison semantics
Choose one immutable target snapshot, generate **target-relative** coverage for every donor and optional **all-pair** relations. Compare pinned file identities first, relocated exact blobs next, then typed symbols/import/contract/capability identities, then explicit equivalence decisions with evidence. Record `unobserved` rather than `missing` for incomplete scans. Missing relative path ≠ missing capability. Git rename heuristics alone ≠ semantic equivalence. Digest equality ≠ runtime equivalence. Missing source ≠ tombstone; only explicit owner-confirmed tombstone with receipt. Keep historical records and signed/hashed decision provenance; refuse same revision different digest.

## Reproducibility, performance and privacy
- Do not execute donor code, install donor dependencies, run untrusted builds or access hidden files just to scan. Read only approved Git object snapshots / authorized source roots / allowlisted GitHub source adapter. Never read credentials, personal corpora or provider browser state.
- No donor checkout or runtime process required to **use an already-ingested snapshot offline**. Fetching a *new* donor Git revision is an explicitly authorized developer-source read, not a hidden runtime coupling.
- Bound repo count, total entries, file bytes, depth, workload and report pagination; cancel cooperatively. Report `truncated=true` and exact counts rather than pretend complete.
- For same identity across snapshots, lineage is a graph **projection** within existing HQ SQLite/HQKG design, not a new operational graph database.
- Host validates every read using real actor/session + live grant; the diagnostic CLI is local/operator-only. No model-inferred permissions. HQ returns analysis/preview, never commits code, sends browser messages, grants privileges or writes personal World.
- Typed JSON errors for `PIN_NOT_FOUND`, `SOURCE_UNAUTHORIZED`, `PIN_MOVED`, `SCOPE_ESCAPE`, `INCOMPLETE_SCAN`, `DIGEST_CONFLICT`, `ID_CONFLICT`, `EQUIVALENCE_UNPROVEN`, `RESOURCE_LIMIT`.
- Make schema adapters explicit rather than silently changing `vivim.catalogue-export/1` or `hq.semantic-registry/1`; preserve backward compatibility.

## Acceptance report fields
`{schema, targetPinId, sourcePins[], pairwise?, coverage, summary, sourceDeltas[], capabilityDeltas[], identityCandidates[], authorityConflicts[], unported[], intentionallyExcluded[], unobserved[], runtimeProof[], mergePlan[], evidenceIndex[], limits, errors, reproducibilityDigest}`; each finding has stable id, source revision(s), 1+ evidence refs, certainty, owner and decision state. Separate `test_executed` from `test_file_present`.
