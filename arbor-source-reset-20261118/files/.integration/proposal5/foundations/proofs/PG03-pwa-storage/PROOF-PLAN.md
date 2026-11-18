# PG03 — executable post-G proof plan (no active patches here)

## Preparation-level evidence
1. Verified A–F GitHub branch refs against the foundation shared contract; all PREG A–F report blobs were fetched and declared FREEZE_READY **source-only**.
2. Verified current foundation 80c29fbc0d45afbce0fadde5388a8f912cdd2762, absent integrate/proposal5-beta-r001, exact source file blobs recorded in SOURCE-DELTA.json.
3. Examined actual E key/session/SW/control code, B direct Host + grant/dispatcher/journal, A launcher/Tauri config, D SQLite/wiki/kernel, HQ pure Rock policy, Labs ChatGPT ZIP/scoped graph and P2-K/T handoffs. Inspected donor Rock test source. No local binaries or original user data supplied.
4. GitHub-only mode: runnable synthetic-contracts.test.mjs validates fixture classifications; **UNRUN** here. It does NOT import actual source modules, assert permission enforcement or exercise a phone.

## Required post-G steps (only owner may implement; source path is pinned frozen *input*, re-pin accepted G HEAD)
**B/E trust surface:** in apps/vivim-host/direct-phone.mjs add precise static signer route with correct MIME and verify manifest icon+service-worker permitted paths. Reconcile every E app.js/module import with allowed paths; prove no private/static spill. Add independent B request signature+fresh nonce (method, canonical path+query, SHA-256 body, audience, grant/session, time, idempotency/operation ID) with atomic nonce tracking, rejected on altered body, replay, stale/grant revoke and no partial write. Implement E signing only against server-defined transcript; protect lost-key and origin-bound registration. Probe Origin checks for ALL enrollment POSTs and x-forwarded/Tailscale trusted proxy. Keep approvals in B Host and Studio sender in C.
**A/B launcher:** src-tauri/src/main.rs verify spawned child/port/nonce ownership before passing owner capability and tighten src-tauri/capabilities/default.json exact IPC URL; non-attested occupied 43187 must get zero owner token. Ensure bootstrap owner bearer cannot leak via logs, debug, redirects or foreign HTTP listener.
**A/G packaging:** after G source landing remove compulsory resources/vivim.apk and conditionalize VIVIM_BUNDLED_APK runtime forwarding; retain packaged Node, Host runtime and valid PWA web assets. Verify actual tools/packaging/prepare-host-runtime.mjs availability and all transitive dependencies in accepted G; no debug APK or placeholder manifest as release proof.
**D/B/A Rock and private data:** instrument read-only filesystem inventory by volume and owner, include allocated SQLite DB/WAL/SHM, FTS, kernel, journals, source vault, user DCB and caches. Propose via pure planRockStorage from HQ research without new DB; only an existing owner may reclaim after sourceAvailable/readable verified, no active grant/lease, protected exclusions and re-measured scratch/reserve. Do not call HQ .hq/index.db wholly disposable because schema includes notes/receipts. Do not auto VACUUM INTO/restore under ENOSPC. Treat personal ZIP import per P2-K as independently owner-consented and gated.
**E/F device:** PWA real HTTPS install, secure context/IndexedDB, pairing with P-256, key loss/revocation/session timeout, SW offline, stale source, IME, accessibility, QR, network uncertainty, no-APK Windows install. F documents exact devices, command exits and evidence.

## Proposed commands (ALL UNRUN IN PG03)
At isolated proof checkout (fixture integrity **only**, not product acceptance):
~~~sh
node --test .integration/proposal5/foundations/proofs/PG03-pwa-storage/synthetic-contracts.test.mjs
~~~
At HQ Rock donor checked-out pinned SHA (source compatibility, independently reproduce):
~~~sh
node --test research/rock-core-storage/test/storage-policy.test.mjs research/rock-core-storage/test/rock-membership.test.mjs
~~~
After G acceptance and selected source import, using actual module paths:
~~~sh
node --test apps/vivim-host/test/*.test.mjs
node --test apps/mobile-relay/*.test.mjs
cargo test --manifest-path native/hqkg/Cargo.toml
cargo test --manifest-path core/programmability/Cargo.toml
node --test .integration/proposal5/phase2/pre-g/K-knowledge/pre-g-contract.test.mjs
~~~
Paths/wildcards are discovery aids; F must enumerate present tests on accepted G tree and adjust commands. Real Windows build must execute the accepted G release packaging command (not asserted here). Android acceptance requires owner-device evidence, not headless desktop approximation.

## Storage decision trace / safe dry-run
- Measure each physical volume and all assigned files. For new writes, check freeBytes - reserveBytes >= incomingBytes + temporaryBytes; account for peak WAL/SHM/FS temp and other app consumers.
- Classify per STORAGE-CLASSIFICATION.json; lock by owning writer and inventory source+revision+consent+hash+proven rehydrate operation.
- Invoke HQ Rock pure plan on immutable metadata (never private content). If state read_only_protected, reject growth; if reclaim_required, **propose**, do not execute. Validate independent owner retention and recovery; remeasure free bytes AFTER any explicitly approved deletion.
- Simulate crash before temp rename, ENOSPC after WAL begin, permission loss, inaccessible source, expired lease and concurrent writer before authorized compaction. Verify original DB/revisions/receipts preserved and idempotent retry only for pure operations.
- Only after real F evidence can owner promote. No storage deleting/VACUUM/writes occurred here.

**Truth labels:** SOURCE_OBSERVED, SYNTHETIC_FIXTURE_UNRUN, NODE_UNRUN, CARGO_UNRUN, WINDOWS_UNRUN, ANDROID_UNRUN, SECURITY_NOT_ACCEPTED, PREPARED_NOT_PROMOTED.
