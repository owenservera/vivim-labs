# Pipeline contracts — proposed design; validate against existing source

## Canonical pipeline
`SourceCatalog -> SourceGrant -> DiscoveryReceipt -> ImportJob -> ContentUnit -> EvidenceAnchor -> CandidateClaim -> Reconciliation -> ScopedGraphChangeSet -> HostCommitReceipt -> QueryProjection -> DCBManifest -> AuthorizedSendReceipt`

Do not add a second store. Reuse Host-owned evidence storage/World, HQ Rust kernel and TierFlow projections; crosswalk all proposed fields against existing source schemas.

## Minimum contracts
### SourceGrant
`grantId, ownerId, sourceId, accountScope, allowedDataClasses, timeRange, allowedOperations, disclosureDestinations, grantedAt, revokedAt, policyVersion`.
Granular scopes: metadata discovery, content ingest, ongoing updates, retention, external delivery; no grant implied by current browser login. Exact provider destination must be checked at send time.

### ImportJob / checkpoint
`jobId, sourceId, grantId, mode(backfill|incremental), sourceRevision, cursor, batchDigest, stage, attempt, status, inputCount, acceptedCount, rejectedCount, updatedAt, failureClass`.
Idempotency by source namespace + stable provider ID/locator + native revision/content hash; reuse existing Rust apply ledger and Host journal. Resuming cannot replay a revoked grant.

### ContentUnit / EvidenceAnchor
`unitId, sourceIdentity, parentUnitId, nativeObjectId, kind, format, contentDigest, locator, observedAt, occurredAt, importedAt, sensitivity, authority, offsets, provenanceChain, retentionPolicy`.
Preserve raw original locally where granted; extracted chunks must retain offsets, order, media and source hierarchy. Never commit secrets/raw personal evidence to Labs.

### Candidate knowledge
`claimId, claimType, subjectRef, predicate, object, evidenceRefs, extractionMethod, confidence, validFrom, validTo, assertedAt, observedAt, state(candidate|accepted|disputed|superseded|retracted), disclosureClass`.
Distinguish assertion by third party / model / owner from independent validation. Derived preferences and identity merges require higher thresholds and review where consequential.

### Graph materialization
Use domain-scoped identities and exact revisions from `typed_dag.rs` and `scoped_graph.rs`. Stage transforms; reject foreign namespace, stale input, malformed provenance, dangling edges, unsafe sensitive edges. Host commits accepted data to its durable personal store; avoid two independent writers.

### Context delivery
DCB is a read-only *projection*: `purpose, task, audience, destination, principal, budget, includedEvidence, omissions, freshness, provenanceDigest, sensitivity, manifestDigest`. Host enforces disclosure separately at time of browser send, binds destination/thread ID to approved send receipt. On graph change or consent revocation, invalidate cached manifests.

## Quality and safety
- Source ingestion is data, never commands; prompt injection must not cross into policy.
- Backfill must be bounded and pausable; storage estimates and low-disk protection before starting archives.
- Dedup exact bytes != semantically same assertion; retain multiple independent attestations.
- Sensitive third-party and cross-account relationships need scoped sharing labels.
- Revocation supports `stop-capture`, `disconnect-keep-history`, and `delete-all-derived` as distinguishable UX choices. Explicit deletion cascades to indexes, graph, cached DCBs and replay.
- Outbound delivery and external already-sent messages cannot be silently clawed back; disclose limitation.
- Acceptance: deterministic replay, duplicate imports, truncated/corrupt archives, renamed files, stale revisions, crash mid-batch, conflicting assertions, account swaps, consent revocation mid-run, cross-domain leakage, exfiltration attempt embedded in source, no device acceptance without device evidence.

## Truth/evidence matrix to produce before merge
Map every owner-producer-transform-consumer edge to `repo, file:line, SHA, schema, test, runtime receipt`. Categorize KEEP / CONNECT / CONSOLIDATE / MIGRATE / RETIRE / INVESTIGATE. Missing evidence is UNKNOWN, never presumed working.
