# ARBOR P5 / P2-K — Personal Knowledge T=0, pre-G preparation

**Disposition:** PREPARED_NOT_PROMOTED. **Owner work-order readiness:** READY_FOR_W2_OWNER (contracts and test planning only); **production ingestion:** DEFER / NOT_AUTHORIZED until explicit post-G acceptance. **Execution scope:** this directory only; no real export, provider action, runtime edit, second database or integration merge.

## Exact observed pins (GitHub API 2026-11-12)

| Evidence | Repository / branch or revision | Commit SHA |
|---|---|---|
| Execution base | owenservera/ARBOR:work/p5-pre-g-convergence-r001 | 8b23c73bf450738a76b256bf0fa17ecbf86db591 |
| Labs personal T=0 donor | owenservera/vivim-labs:main | 022608f5797e7b4a14ccc4c393bd3c5a4704a4be |
| W1 A native current read | owenservera/ARBOR:work/p5-w01-a-native | dda546a11294920d6521a0f64ab7ee62ec993c70 |
| W1 B Host current read | owenservera/ARBOR:work/p5-w01-b-host | 78a5105a9c1f1880d658f350e130b1e2632d3532 |
| W1 D HQ current read | owenservera/ARBOR:work/p5-w01-d-hq | ad55fe28c2c331841675f308f9199ee4c2add86a |
| Readiness classification | ARBOR:work/p5-phase2-readiness-docs-r001, phase2/readiness/SOURCE-TO-ARBOR.json | blob b02a7c3c8e25a825e4c9a7cf3bc8fcd9247333f3 |

These are **different isolated W1 branches**. ALREADY_IMPORTED in source-delta.json means present on that W1 owner branch, NOT present in this pre-G branch, a merged G beta or a running installation. GitHub file blobs are recorded in source-delta.json; latest W1 branch HEAD must be revalidated by G before selecting patches. Tests claimed in Labs' ARCHITECTURE.md are donor-reported, not executed in this lane.

## Verified source behavior, not aspirations

**Labs, user-selected ZIP only:** personal-knowledge-lifecycle/runtime/src/zip-reader.mjs (blob 896fdc198fa303f350a66c681e77423f759f50d2) implements read-only ZIP32/ZIP64 central directory validation, unsafe names/duplicates/symlink rejection, compression and expansion caps, streamed size/CRC verification, and SHA256 from one open handle. chatgpt-import.mjs (9bfbcd40749774a3405a8d4f3456211fe4920253) requires exact import grant, fingerprints the archive, parses a top-level conversations.json array, normalizes, stages before effects, checkpoints pending batches in injected store, inspects an authoritative sink, and reconciles committed batches after crash. json-array-stream.mjs (4f061bca5d3ee0e67646e63a6fdf9e23b244bfab) enforces UTF-8, strict object-array form, object count/byte caps. chatgpt-normalize.mjs (58b1ab985fbea2811eff3f4a2a5a70b4cb606d45) retains message/node hierarchy, roles, time, omissions, source account and canonical content revision; nontext media bytes are omitted. t0-core.mjs (39f9c5410c438ad1d5be4450cea0fdce928cb630) tracks grants and per-lane receipts in an injected transactional store.

**Labs is not an authoritative writer:** host-ledger-port.mjs (a6c68514c330af3cf60856e5cc5206514724e9db) adapts the existing Host ledger transaction/meta API. scoped-graph-proposal.mjs (6196af064d79a28e0ad3d59958264ac918d6b27c) constructs a deterministic proposal, NOT an executed Rust apply. host-archive-wiring.mjs (e8ddbb17a83fbc398d1b651e121237bd1dcd3ef9) injects required host.authorizeSource, inspectEvidenceBatch, commitEvidenceAndGraph and graphRevision methods (or a durable storage port); those names are a proposed adapter surface, **not observed as implemented on B**. fastlane-snapshot.mjs (fd59bedd5897124aecb81abd07a326555187453a) keeps profile/project and F02/F01 snapshots candidate-only and requires verified Studio receipts for bootstrap captures.

**Host W1 B:** apps/vivim-host/bootstrap/ledger.mjs (blob b40eb9ba34e49b034875936a0e7fc14964270a54) exports openLedger; its node:sqlite database offers transaction(fn), meta.get/set and other owner-owned tables. apps/vivim-host/onboarding/service.mjs (287a107eab19411a7cf0a3cadb54ea025f9377f0) has owner-reviewed F02/F01 approval and Studio scheduler; browser signed-in readiness is NOT archive consent. apps/vivim-host/corpus/intake/host-intake.mjs (1db63cf3b0cc2dca09590a203b93fc895d4c936c) and corpus/host-integration.mjs (9c2df8dc742cc88f9f5e0757380137d8492a652c) provide separately scoped manual raw corpus grants, receipt verification and read projection; they do NOT themselves implement the ChatGPT archive adapter or accepted World graph commit.

**Native W1 A:** native/hqkg/src/scoped_graph.rs (blob 586843b56eedd4a9926bb443cbbeda80d841bda4) validates namespace/revision structure but explicitly does not authenticate caller provenance. native/hqkg/src/bin/hqkg-personal.rs (8049558d6e5e625ede8d03be2504c7f947524233) accepts bounded Host-invoked scoped graph packets, epoch/byte preconditions and commit markers, with compact/erase/erase-effects commands. Actual Host process invocation, authorization, raw evidence custody and cross-store reconciliation are NOT proven.

**HQ W1 D:** hq/lib/personal-corpus-contract.mjs (blob c842fe5848d47179dc3852335747e1227e344d1d) is a pure personal intake envelope validator, not a permission authority; hq/lib/session-import.mjs (4bbd82832bd6d5bdf961ad884e6de9012bd5d464) imports OpenCode development sessions, NOT ChatGPT personal history. Development HQ indexing must not become a second personal KB or unrestricted raw-chat reader.

## File/commit-level migration DAG — proposed, NOT executed

See migration-dag.json for machine-readable nodes, edges and gates.

1. **B identity and consent** (B HEAD 78a5105; onboarding/service.mjs blob 287a107; ledger.mjs blob b40eb9b) -> exact World, authenticated ChatGPT account, import-specific SourceGrant, rights/sensitivity/retention and free-space admission. Sign-in and existing full-bootstrap approval do not authorize ZIP import.
2. **B approved local custody** (Labs 022608f / zip-reader.mjs blob 896fdc1) -> user chooses absolute local ZIP; Host binds a single file and source revision; no export scraping/provider APIs. Reject symlink, traversal, ratio bombs, changed bytes, ambiguous conversations.json and private path exposure.
3. **B parser and normalize** (Labs 022608f / json-array-stream.mjs blob 4f061bc -> chatgpt-normalize.mjs blob 58b1ab9 -> chatgpt-import.mjs blob 9bfbcd4) -> bounded normalized objects, stable account-qualified revisions, loss/omission metadata. Never transform source text into commands.
4. **B durable checkpoint** (Labs host-ledger-port.mjs blob a6c6851 -> B ledger.mjs blob b40eb9b) -> pending intent before effect, exact inspect(key,digest), no duplicate replay. Distinguish absent / committed / uncertain without inference.
5. **B evidence publication** (B host-integration.mjs blob 9c2df8d + host-intake.mjs blob 1db63c; Labs host-archive-wiring.mjs blob e8ddbb1) -> ONLY Host-verified signed durable receipt and owner grant; adapter conversion is required, not present.
6. **A graph apply** (Labs scoped-graph-proposal.mjs blob 6196af0 -> A hqkg-personal.rs blob 8049558 + scoped_graph.rs blob 586843b) -> Host invokes typed scoped graph CAS, records exact operation digest and journal marker. Rust data-plane does not authorize itself.
7. **B ledger finalize/replay** (Labs chatgpt-import.mjs and t0-core.mjs -> B ledger) -> checkpoint only after observed, current-grant-checked receipt. On crash/restart verify Host journal and graph marker, not write twice. Erasure fence supersedes old checkpoints.
8. **D/F read-only provenance and privacy projections** (D personal-corpus-contract.mjs blob c842fe5) -> expose only scoped authorized candidate/evidence references; no automatic promotion to owner fact, no secondary personal store; F verifies. **E** may later show progress from Host projection after B/A acceptance.
9. **B+A erasure and bounded retention** (B Host evidence custody + A hqkg-personal erase-effects) -> revoke capture; block concurrent commits and replay; erase requested raw, derived, indexes, graph, cached contexts and pending jobs; verify post-erase query denial; journal minimal non-content tombstone if allowed by policy. No silent external-recall claim.

### Source-grounded blockers / non-accepted edges

- The Labs importer buffers the normalized array in memory until full CRC/EOF, with stagedBytes limit of 8 MiB, maxObjects 10,000, maxBatchBytes 1 MiB, maxObjectBytes 512 KiB, and default ZIP entry 32 MiB / ZIP file 64 MiB. It is not a true bounded-disk archive ingestion of a large ChatGPT history. Further streaming or durable invisible staging requires B-owned design and crash/privacy tests, not relaxing caps blindly.
- Labs fallback host-archive-wiring.mjs maps sink.inspectBatch to host.inspectEvidenceBatch with key/world/account/provider/scope, **not the digest argument** accepted by importChatGptExport's sink contract. B must require and compare the bound batch digest; do not accept caller-asserted verified:true without an independent Host journal/signature check.
- Graph proposals and evidence writes have no proven *combined* publication transaction; uncertain must remain held until reconciled. A already has an erase-effects graph operation, but there is no audited end-to-end personal export revocation/erasure cascade.
- t0-core.mjs stores an accumulating evidence map in a Host ledger meta JSON record; quotas/compaction/tombstone durability remain owner decisions. A graph has separate capacity gates (16 MiB serialized, 20k nodes and 50k edges in CLI), not unlimited personal-history capacity.
- Official conversations.json does not cover all ChatGPT export artifacts/attachments; preserve omissions and explicit coverage, never claim full corpus.
- ZIP reader and donor tests contain synthetic behavior; malicious ZIP variants, Windows locking, resource exhaustion, and real export schema acceptance require post-G test execution.

## Rock Core and storage invariants (design constraint, not database)

Rock Core means a **policy-protected floor within existing Host authority**, not another DB, extra graph or a competing persistent journal. B owns quota/retention decisions, A owns graph compaction mechanics, F checks safety. Partition existing retained bytes conceptually into (a) protected minimum: identity/grant fences, current verified receipts, source revision and minimal provenance needed to prevent replay; (b) retained originals selected by owner, subject to explicit user policy; (c) rebuildable derived indexes/cached context and temporary media. At admission calculate usableBytes = max(0, availableBytes - OS/emergencyReserveBytes - protectedGrowthReserveBytes); require planned peak write bytes and abort/rollback headroom, not only steady-state estimate. Never silently evict protected floor, legally/owner-retained originals or deletion fences. First evict least-recently-used **rebuildable** projections with source-link manifests; then pause imports for review. Compaction must maintain authenticated evidence, affected source links, tombstones, graph CAS and recovery receipts. Per-owner limits are configurable; zero free space means fail closed, not erase history. No unlimited JSON meta map, silent rehydration, or background chat reimport without renewed consent.

## Promotion rule and explicit effort

- **READY_FOR_W2_OWNER:** this documentation, typed proposed ports and negative cases are ready for owner review after G; estimated B 1–2 engineering days for adapter design/proof + A 0.5–1 day graph boundary + F 0.5–1 day synthetic verification, assuming contracts remain compatible.
- **DEFER active import:** a full capacity-safe operational path and erasure acceptance may need several additional days plus real-user-consented local testing, especially staged large archives, Windows/device tests and secure erasure. No time estimate constitutes a production guarantee.
- **Do not cherry-pick to G automatically:** G has no dependency on this lane; W2 owners may select at most the five bounded tasks in owner-handoff.md. Branch remains prep-only and must not be used as a product-runtime import.
