# VIVIM Pocket Wave 5 — Lane 5A transport handoff

**Status (2026-11-06):** **SOURCE COMMITTED / ISOLATED NODE TESTED; NOT WIRED, NOT HOST-VERIFIED, NOT DEVICE-VERIFIED.** Integration 5 is **BLOCKED**. This is a lane-owned, non-HTTP, Host-side transport primitive, not a production upload endpoint, authorization decision or durable HQ ingest receipt.

**Work order:** [repos-hq/WAVE-5/LANE-5A.md](https://github.com/owenservera/repos-hq/blob/main/CHATGPT-SPRINT/POCKET-CAMPAIGN-2026-11-05/WAVE-5/LANE-5A.md). **Next owner:** [Integration 5](https://github.com/owenservera/repos-hq/blob/main/CHATGPT-SPRINT/POCKET-CAMPAIGN-2026-11-05/WAVE-5/INTEGRATION-5.md).

## 1. Exact repository baselines and concurrent work

- Initial read: `vivim-web-harness/main=f9568a5b8a82fd6d1c6da8cac221b6205b2cc323`; `repos-hq/main=e573c798cbeba53f09775be9c75ca5829646efb2`.
- Rechecked before first write: harness `3a6afe69ef49f646d9b524a4f4aa51a46c78a809`. That intervening Integration 4 commit touched **only** `studio/android/build.ps1`, `build.sh` and `src/inc/vivim/app/PocketStore.java`; those are read-only for Lane 5A and were not altered.
- Rechecked immediately before this handoff: harness `d1fed0e68f90cf2e0212079da6ff2e0f8c805471`; HQ `252c8ebd3d66c25fbb72e4e797a1342a54089e51`. Concurrent additions include Wave 5B's `apps/vivim-host/corpus/intake/` and Wave 5C/5D HQ work. Subsequent Android `PocketHome.java`/`PocketOwnerActions.java` and 5D evidence changes were observed and left untouched.
- One GitHub create-file write was rejected by a real `409` expected-head conflict while 5B was editing `host-intake.mjs`; the head and per-file diff were reread, then the test-evidence commit was retried successfully. No force push or overwriting of concurrent work.
- The tree still had **no `INTEGRATION-4-RESULT.md`** at last verification. Wave 4 physical/Host acceptance remains unproven.
- Canonical sources read, without modification: HQ `docs/architecture/MOBILE-ONE-AUTHORITATIVE-PATH.md` blob `8a9f07b03981273f6fa09afdd70752ae30b3cd4d`; HQ `contracts/personal-corpus/README.md` blob `57f15f01493b8796c8321cdf78ba46dfec4a43aa`; HQ `hq/lib/personal-corpus-contract.mjs` blob `c842fe5848d47179dc3852335747e1227e344d1d`; HQ `fixtures.v1.json` blob `a1fbab64905612e3a206be3108b27b908ba25fe6`; HQ `schema.v1.json` blob `7390bb6e1942402c0d73cc1a8c3a3a5c0a94b5f1`; harness `studio/android/POCKET-ROUND3.md` blob `20fb3e7bf86dfc7e20dcfcc52cd3543145883f28`.

## 2. Lane-owned deliverables and exact commits

All files are new and confined to `apps/vivim-host/corpus/transport/*`. No shared Host routes, Android sources, APK builder, mobile relay, HQ contracts, provider handler or KB authority were changed.

| Path | Commit provenance | Verified final Git blob |
| --- | --- | --- |
| `protocol.mjs` | Created `4938bcdb23821e308a56767a83a8405438b6598b`; limits aligned `b916dc5e3a7589a4e54a6f885018808ed326d418`; mixed-World fix `f496b0e51f06aa0f6e474a026d0a143bfffa7106` | `b6a6976a8b100b69d73309a69bed7d6578cbbd3b` |
| `protocol.test.mjs` | Created `3d6f06484bc3cc230cb306d845e7b4c010cc8316`; bounds `b0424b39b52dfa6dbe4b55f5e9b84830680418d9`; mixed-World negative test `39835f1e0d063bd37072694738da9972f66d5c89` | `94e55bed8219fd505a1ec49a0cc5fb65d94c63c2` |
| `hq-contract.test.mjs` | `0998644b3d36c89c202726bfbde48333385a1192` | `ebafe7253a2925c5d6b7caba2468ba0b3d6d943b` |
| `test-results-2026-11-06.tap` | `845e1230a53af61ea83ac0da204d7b45b6cedacc` | `9bd2bc4b100ba32cec76937c2fd11c2acde3e35b` |

The final source and test blobs were fetched back from GitHub and matched the files executed in the isolated Node working tree.

### Implemented behavior

`createCorpusTransferProtocol(...)` uses injection of the existing Host's trusted `authorizeDevice`, `approveTransfer`, canonical HQ `validateCapture`/`canonicalCapture`, receiver `persistAccepted` and read-only `lookupAccepted`. The module **cannot create its own device grants, grant approval, perform provider sends, write World state, or promote KB content**.

- New version marker `vivim.corpus-transfer/1`; immutable `(worldId,eventId)` identity, canonical metadata digest, original-byte SHA-256 checks and replay-conflict rejection. Mixed-world batches are rejected because cursor operations identify one eventId inside its single approved World.
- Authenticated live enrolled device/grant and explicit `corpus:upload` scope required for **every** operation; exact `owner`-approved batch hashes, event IDs, byte budgets, expiration and grant revocation checked independently of a capture envelope. Approval callback results are never accepted from a request body.
- A client must invoke `issueScope` separately; there is **no reconnect hook or automatic transmission**. `inspect` yields a recoverable byte cursor, `pushChunk` accepts only exact-cursor chunks or identical fully-previously-committed retries. Altered/reordered chunks fail.
- Chunk writes are flushed to staging *before* committing the ledger cursor. Restart recovery truncates an uncommitted staging tail; missing committed prefixes fail closed. One exclusive lock prevents stale concurrent ledger writers; crashed locks fail closed pending operator intervention.
- `finalize` verifies whole raw digest and first persists `accepting` before calling the Host receiver. An ACK is marked `accepted` only after an injected receiver returns a claimed durable receipt with matching world/event/metadata digest/original digest. If receiver state is uncertain, it never calls the receiver again automatically; explicit `reconcile` consults trusted read-only receipt lookup.
- Current limits: **10 events/batch; 32 MiB total approved original bytes; 25 MiB/asset; 256 KiB/chunk; 24 h max consent scope; 16 MiB max transport ledger file**. Bounded streaming SHA-256 read avoids loading full media into transport memory.

## 3. Executable evidence, accurately scoped

Executed in the isolated local working directory with **Node `v22.16.0`**, against Git-blob-verified source. Syntax checks were run for each of the three `.mjs` modules. Raw stdout is committed alongside source in [test-results-2026-11-06.tap](test-results-2026-11-06.tap).

```sh
node --check apps/vivim-host/corpus/transport/protocol.mjs
node --check apps/vivim-host/corpus/transport/protocol.test.mjs
node --check apps/vivim-host/corpus/transport/hq-contract.test.mjs
node --test apps/vivim-host/corpus/transport/*.test.mjs
```

**Observed final TAP (real execution, isolated synthetic receiver):**
```text
1..12
# tests 12
# suites 0
# pass 11
# fail 0
# cancelled 0
# skipped 1
# todo 0
```

The eleven passing tests include an in-process synthetic **10-capture/10-receipt cycle** with zero duplicate accepts; byte-identical replay; immutable ID/content conflicts; unauthorized, expired and revoked scope; SHA tampering; strict upload cursor and retry; 10-event/32-MiB/25-MiB/256-KiB budgets; distinct Worlds; restart resumption after an orphan tail; simulated receiver response loss after persistence with explicit reconcile; crash-left lock and lost committed prefix.

**SKIP — do not count as a pass:** `hq-contract.test.mjs` intentionally requires an actual sibling `repos-hq` checkout or `VIVIM_REPOS_HQ_PATH`. This isolated container did not have that checkout. It will import exact HQ validator and exact shared fixture bytes *when available*, not an independently-authored approximation. The unit tests instead inject a synthetic corpus validator, synthetic approval broker and synthetic fsyncing Host receiver. **No live Host process, enrolled device session, actual Android phone, SDK34 build, HQ persistence adapter, or production-credible Host ACK was exercised.** This is real Node execution, **not a Host acceptance receipt**.

**Required cross-repo rerun** on a checked-out pair:
```sh
VIVIM_REPOS_HQ_PATH=/absolute/path/to/repos-hq node --test apps/vivim-host/corpus/transport/*.test.mjs
node --test apps/vivim-host/corpus/transport/protocol.test.mjs
```

## 4. Integration 5 patch proposal (read-only cross-lane files)

This is a concrete **proposal**, not an applied patch. The concurrent 5B implementation `apps/vivim-host/corpus/intake/host-intake.mjs` currently exposes `createHostCorpusIntake(...).acceptBatch({batchId,grantRef,session,items:[{envelope,bytes}]})`, requires a distinct device scope `corpus:intake`, a Host-approved `corpus:raw-intake` grant, and HMAC-signed batch receipts. Its limits are 10 items/32 MiB; Lane 5A was aligned to them. **Do not use the transport's `corpus:upload` as evidence of 5B intake authorization**. A separate Host-only grant check is mandatory.

Integration-owned changes needed, in order:

1. Mount the minimal authenticated corpus routes in the existing Host/Direct handler only: explicit owner scope issue, status, binary chunk, finalize, explicit reconcile. Authenticate the existing `enrollment.validateGrant({grantId,sessionId})` on each request and require an independently approved `corpus:upload` scope; recheck revocation and owner review immediately before final Host acceptance. Never attach a listener to reconnection events. Treat capture envelopes, local IDs and client approval fields as untrusted.
2. Reuse HQ's exact `validateCorpusCapture` and `canonicalCorpusJson` at the Host boundary. Keep immutable local `policy.grantRef:null` as captured: 5B currently requires non-null `policy.grantRef`, which would force illegal mutation of original immutable metadata. Pass the authorized Host grant **out of band** and bind it to original immutable envelope digest, approved owner, World, device, event ID and hash. Use a separately versioned Host acceptance record, not a rewrite of `vivim.corpus-intake/1`.
3. Build `persistAccepted` on the single existing 5B Host raw-record authority and **verify signed receiver receipt** before translating to 5A's `{durable:true,worldId,eventId,digest,sha256,receiptId}`. Never accept a transport-generated receipt or treat bytes-on-disk as a Host approval. **Implement `lookupAccepted` as a verified, read-only durable receipt lookup** for after-crash reconciliation; 5B currently exposes only `acceptBatch`. A batch may need one approved scope per 10 events; preserve exact per-event receipts.
4. 5B's current `validateOriginalEnvelope` accepts only `stored` + `original`, whereas HQ canonical fixtures also carry `referenced` and `metadata_only`. These may be processed as separate immutable metadata intake events with no invented bytes. No implicit promotion into KB/World, no source rights inferred, and no provider effects. If this is deferred, explicitly reject non-stored input rather than claiming imported.
5. Enforce the existing Host's owner approval, rights/sensitivity/third-party review and one World writer; add live signed-device tests for revoked grant while uploading, concurrent Host workers, restart before/after bytes and ACK, 10 durable records, cross-repo canonical fixture compatibility, deletion/retention and backup/recovery interactions. Retest after Integration 4 finishes; do not ship a second phone or send path.

## 5. Remaining risks, performance and non-claims

- **No production Host adapter/routes**, Android UI affordance or per-device HTTPS auth wiring has been integrated. A forged response from a defective `persistAccepted` adapter would not prove durable acceptance. Integration 5 must verify the Host's signed receipt and persistent store, not just the `durable:true` field.
- The transfer staging ledger has no finalized retention/garbage-collection policy; orphan raw bytes, expired scopes and stale locks require a bounded Host-owned policy. Its 16-MiB JSON ledger and fsync-on-each-256-KiB-chunk are conservative correctness-first limits, **not measured Host throughput or storage budgets**. A crash with a pre-receiver `accepting` marker remains **uncertain**, fail-closed, until explicit owner/operator reconciliation.
- POSIX directory `fsync` is attempted; Windows directory sync is intentionally omitted due to OS variance. Actual NTFS/Tailscale Host restart, power-loss and durable ACK semantics have **not** been verified.
- Corpus owner evidence never grants transfer or authorizes downstream actions. No data is automatically submitted on reconnect, and transport does not make knowledge claims.
- **Context7 consulted:** `/websites/nodejs_latest-v22_x_api`, Node.js v22 API documentation for `fs.openSync`, `fs.fsyncSync`, `fs.renameSync`, `crypto.createHash`; installed test runtime `v22.16.0`. Filesystem-specific crash durability on Windows and the production grant/receipt adapters remain **unverified**.

| Gate | Result |
| --- | --- |
| SOURCE COMMITTED | **YES** — exact commit and blob IDs above |
| ISOLATED NODE TESTED | **YES** — 11/11 runnable pass; 1 HQ fixture check SKIPPED |
| CROSS-REPO CANONICAL FIXTURE EXECUTED | **NO** |
| SHARED HOST/APK INTEGRATED | **NO** — Integration 5 only |
| ANDROID COMPILED (SDK34) | **NO** |
| DEVICE VERIFIED | **NO** |
| LIVE HOST VERIFIED / durable ACK | **NO** |
| INTEGRATION 4 and 5 physical release gates | **BLOCKED** |

**Next action:** Integration 5 should first reconcile 5A/5B's differing consent, immutable envelope and receipt interfaces, then run the full signed-device/Host and canonical HQ fixture matrix. Do not mark a release gate as passed based on the isolated tests above.
