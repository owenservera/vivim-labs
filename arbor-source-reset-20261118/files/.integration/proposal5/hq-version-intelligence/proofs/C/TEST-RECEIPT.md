# HVI C — executed checks and source-bound proof
Gate: PREPARED_NOT_PROMOTED. Branch: `work/p5-hvi-w1-c-compare-r001`.
Worker base (S0 `GO_TO_PARALLEL`): `f9823a5131e73e7562db5a4ec14e1ec481557d69`.
S0 evidence branch HEAD: `2ef7a9f66419d681e040b459ba4762a63aa9564f`.
Tested comparator blob: `5203df02605b82951dfa2d5dbee9f0f179e0ecfe`.
Tested fixture blob: `85f949bcfbf8701d5a5aa1feabf79fd1867161f4`.
Source commit: `2261aae59ea5c4bbdf0261acdc25bf98ad110c34`.
Test commit: `8d7fab7b03f247977b210bc2cc5dbf4ccafd832a`.
No edits to `main` or non-owned paths.

## Actual executed commands (ephemeral Linux container, Node v22.16.0)
- `node --check /mnt/data/hvi-c/compare/index.mjs` — exit **0**, syntax accepted.
- `node --test /mnt/data/hvi-c/compare/index.test.mjs` — exit **0**, TAP **9 tests, 9 pass, 0 fail, 0 skipped**; duration 80.164261 ms for final run. The tests cover T02, T03, T05, T06, T12, T13, T17, T19, T20 subsets. Do NOT infer other T01–T24 cases passed.
- `node --input-type=module` with inline synthetic 10,000-entry manifests per pin and `budgets.maxFindings=1000` — exit **0**, 20,000 input entries, 10,000 observed findings, 1,000 emitted findings, `limits.truncated=true`, 156 ms wall-clock, 90 MiB RSS in the measured process.
- `git hash-object` for each local tested file — exit **0**, exact blobs above; connected GitHub `fetch_file` independently returned those same blob IDs on the worker branch.
- Connected GitHub source inspection (not executable tests): donor/library implementation and S0 pinned proofs; read of `integration/source-lock.json` and `integration/beta-gates.json`.

## Tests UNRUN (not passes)
- Node 24 CI/integrated checkout tests, G1 combined A/B/C/D/E/F conformance, actual Lane A Git snapshot acquisition, Lane B identity approval adapter.
- `bun test`, `cargo test`, Windows/MSVC, Android physical devices, Host grant/revoke, network and source-closure pipeline, native loopback/security acceptance.
- Actual five-repository corpus scan, complete HQ 361-path semantic classification, production performance and end-to-end runtime parity.

## Evidence limits and privacy
Input: pinned **synthetic** manifests only. No donor checkout, browser session, private corpus, network source acquisition, or production handler invoked.
The module uses `node:crypto` for canonical digests and no filesystem/network/writes. Paths resembling credential/profile artifacts are refused, and outputs contain Git revision/blob provenance rather than file bytes. No source-only hit becomes `RUNTIME_VERIFIED`; no missing file creates a tombstone.
The bounded test deliberately truncates findings; its counts are not a completed migration audit.
Existing beta gate remains `BLOCKED_NOT_BETA`.

## Inspected source blobs on immutable worker base
- `hq/lib/catalogue-delta.mjs`: `6849e6df26bd136ee50ae08a260d8ab9e9445ac7` — two-snapshot C4 delta/replay, not this N-way repo comparator.
- `hq/aii/fixtures/HQ-114-compare-registered-repos.mjs`: `800f7119cad59228577b029772bd98592ec9bc2a` — seeded overlap fixture, not a live repository scan.
- `hq/lib/graph.mjs`: `f824b0b9d940bbde9b80ee5d51db2e0eb21cf6a3` — existing graph projection preserved.
- `hq/lib/semantic-registry.mjs`: `ec982381c77147119f62e702a162a203df8e7d4e` — existing semantic owner registry preserved.

Disposition: `PREPARED_NOT_PROMOTED`. No beta GO; no runtime certification.
