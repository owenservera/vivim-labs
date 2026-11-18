# Lane A handoff — B, C, E, F, and G1

**Status:** PREPARED_NOT_PROMOTED. **Base:** `f9823a5131e73e7562db5a4ec14e1ec481557d69`. S0 gate `GO_TO_PARALLEL` from `2ef7a9f66419d681e040b459ba4762a63aa9564f`. Never treat source-only fixture green as release/device/Host authority acceptance.

- **B identity:** consume `pin.pinId`, `pin.repoIdentity`, `pin.resolvedCommitSha` and `manifest.entries[].blobSha`. Do not treat path as global identity; `hq/catalogue-identity` owns semantic crosswalk decisions. Same blob across distinct repos is not same capability.
- **C N-way comparator:** consume two or more complete offline `SourceManifest` objects. Incomplete scans must yield `UNOBSERVABLE`, not deletions. `manifest.manifestDigest` is deterministic; independent verification via `manifestDigestOf`. Same repo 3 commits supported.
- **E reports:** display `complete/partial`, `omittedMinimum`, `errors`, `parseStatus`, `excludedReason`, and exact revision; never serialize `readPinnedBlob` results or leak credential text.
- **F conformance:** use `fixtures.mjs` for v1/v2/v3, page order, Git blob SHA-1, symlink escape metadata, LFS, binary and secret exclusions. Add multi-repo source tests and malicious adapter pages. Full Node24/CI acceptance belongs G1.
- **G1 integrator:** supply explicit authorized Git-object adapter and combine exports under integrator ownership; do not add `gh`/provider-browser effects. Run `node --test hq/version-intelligence/sources/snapshots.test.mjs` plus F's independent tests on Node24, record exact commands/exit, prove offline replay from persisted manifests.
- **H Host owner (future wave):** bind real actor/session/live development source grants to authorize; inspect revocation on every request; keep source projection read only.

**Non-blocking integration handoffs:** No in-lane GitHub network adapter is implemented deliberately. Source grants, trusted recursive tree page provenance, Git SHA-256 repo support and durable offline manifest persistence need named owner acceptance. No new graph/store/scheduler/catalogue, mutable HEAD shortcut, source effect writer or browser sender introduced. Existing beta gates remain blocked.

**Lane A initial source commit:** `18a5bae8afa915bb8fd5f6817774c17918a5a582`. **Executed source-only harness:** 14/14 synthetic assertions using V8 emulated Node APIs; no process exit. **Still UNRUN:** actual `node --test`, Node24 compatibility, Host grants, Rust, Windows/Android, CI.
