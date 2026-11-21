# Wire contract, canonical hashing, error/refusal matrix
## ABI versions
- Parse: `arbor.scf.parse/0.1`; Compiler snapshot: `arbor.scf.snapshot/0.1`; Fact: `arbor.semantic-capability-fact/0.1`; Passport: `arbor.scf.capability-passport/0.1`; Delta: `arbor.scf.semantic-delta/0.1`; Diagnostic: `arbor.scf.diagnostic/0.1`.
- SV reuse: `arbor.sv.claim/0.1`, `arbor.sv.proof-receipt/0.1`, `arbor.sv.explanation/0.1` EXACT; don't define parallel proof schema.
- Existing `hq.annotation/2`, `hq.semantic-registry/1`, `vivim.semantic-wiki-ir/0.1`, and HVI VersionComparison are input *adapters*, not formats to overwrite.

## Required pin and fact fields
`Pin {repository,commitSha,treeSha,requestedRef?,completeness,scope,manifestDigest}`, no branch names as immutable identity. `Fact {subject,predicate,object,source:{path,blobSha,lines,unitRef?,scope},pin,claimStage,staticStatus,evidence[],digest,coverage}`. Text/comment cannot set `TEST_OBSERVED` or `RUNTIME_VERIFIED`.

## Canonical JSON digest v0.1
1. Reject non-JSON values (undefined, NaN, cyclic, functions, infinity), unsafe strings, unknown object keys at wire boundary. Normalize UTF-8 to NFC at allowed text fields, normalize source paths to repo-relative POSIX and refuse `..`, absolute drive, NUL. Do NOT normalize user-authored source bytes before hashing them.
2. Serialize object keys recursively sorted by UTF-8 byte order; set-like arrays sorted by documented canonical key (tag identity/record ID); ordered arrays such as source stack/event trace retain original order. Preserve integer/string types; no locale sort, timestamps or OS paths in content-addressed digest.
3. Use `sha256(Buffer.from(canonicalJson(value),"utf8"))`; prefix digest names by schema and input domain to prevent cross-domain confusion. Git tree/blob SHA is kept verbatim from Git metadata and verified against Git source adapter.
4. `snapshotDigest` covers exact pin, manifest, registry digest, approved owner catalog digest, parser/SCF ABI versions and sorted facts/diagnostics/coverage. Runtime receipts are separate with own source/build/spec anchors; they never rewrite past Git snapshot digest.
5. Duplicate `(repo,commitSha,path)` with different blob/digest => `SCF_DIGEST_CONFLICT` HARD FAIL; ambiguous stable unit/semantic ID across owners => refuse equivalence.

## Typed diagnostics (single source of error truth)
`{schema:"arbor.scf.diagnostic/0.1",code,severity:"error"|"warning"|"info",stage:"lexer"|"attach"|"registry"|"compile"|"storage"|"proof"|"projection"|"authority",sourcePin?,path?,span?,ref?,message,blocking,recoverable}`.
Hard-failure codes: `SCF_SOURCE_UNAUTHORIZED`, `SCF_SOURCE_ESCAPE`, `SCF_SECRET_REF`, `SCF_INVALID_PIN`, `SCF_DIGEST_CONFLICT`, `SCF_ID_CONFLICT`, `SCF_ANNOTATION_UNKNOWN`, `SCF_ATTACHMENT_AMBIGUOUS`, `SCF_OWNER_MISMATCH`, `SCF_SPEC_UNAPPROVED`, `SCF_PROOF_ARTIFACT_MISMATCH`, `SCF_RESOURCE_LIMIT`, `SCF_STORAGE_INCOMPLETE`, `SCF_HVI_INTERFACE_UNAVAILABLE`, `SCF_PERMISSION_REVOKED`.
Soft/unknown: `SCF_TAG_UNSUPPORTED_VERSION`, `SCF_SOURCE_PARTIAL`, `SCF_NOT_RUN`, `SCF_TEST_UNOBSERVED`, `SCF_RUNTIME_UNKNOWN`, `SCF_SCOPE_UNCLASSIFIED`, `SCF_SOURCE_UNANNOTATED`, `SCF_STATIC_CLAIM_UNPROVEN`. Never coerce to negative capability from partial data.

## Confidence and evidence
Use orthogonal `declared:{yes,no}`, `parsed:{valid,invalid,unknown}`, `registry:{current,proposed,deprecated,unknown}`, `static:{observed,contradicted,not_checked}`, `test:{receipt_valid,refuted,unrun}`, `runtime:{observed,not_observed,forbidden,unknown}`. SV verdict on each property is an additional independent dimension, never a scalar score. A `REFUTED` property cannot be masked by another passing property.

## Reader output rules
Human Wiki may render a summary but every sentence that claims working/tested/available must bind exact evidence. Incomplete manifest = `incomplete`, missing source = `unobserved`, never `retired`. Unknown refs in v2 records are not publishable even if parser emitted them. Host source grant must be checked at *read time* for stored sensitive passports and excerpts.
