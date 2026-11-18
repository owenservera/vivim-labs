# HVI B — G1 integration handoff

**Gate: PREPARED_NOT_PROMOTED**. This is source/proof on an isolated branch, not integration into ARBOR main and not runtime verification.

## Immutable scope
- BASE_SHA: f9823a5131e73e7562db5a4ec14e1ec481557d69 (S0 GO_TO_PARALLEL).
- SOURCE_PINS: repos-hq main 9b922ea049dbadc94e504b5b9432743995c020ca; ARBOR base f9823a5131e73e7562db5a4ec14e1ec481557d69; Labs main 022608f5797e7b4a14ccc4c393bd3c5a4704a4be. Full 11-ref set in S0/SOURCE-REVISIONS.json.
- WORK_BRANCH: work/p5-hvi-w1-b-identity-r001. G1 must fetch actual branch tip and review commit ancestry and per-file diff before serial merge. No stale SHA reset.
- CHANGED_PATHS: hq/catalogue-identity/**; hq/version-intelligence/identity/**; .integration/proposal5/hq-version-intelligence/proofs/B/**. No other paths authorized.
- DONOR_BLOBS: proof DONOR-MAP.json enumerates all 14 files: 7 byte-identical, 2 source-adapted, 5 intentionally excluded historic items.

## Source implementation
1. Ported donor C4 source crosswalk, Labs input adapter, source adapter, CLI and original crosswalk Node tests/benchmark. Original C4 schema and source-scoped IDs remain intact.
2. Added pure immutable pinned identity module. One repo at N versions uses distinct repo+commit+source row IDs and a separate lineage key; cross-repo canonical claims remain review candidates; no automatic equivalence, promotions, tombstones or Host effects.
3. Added explicit C4 and vivim.catalogue-export/1 compatibility adapters while leaving existing catalogue-delta/semantic-registry code and schema unchanged.
4. Observer export hardened to fail on mismatched snapshot digest or malformed allowlist. No source text or secret references in observer output.
5. Authored new negative and N-way identity synthetic tests in identity/test/identity.test.mjs, plus original donor test/crosswalk.test.mjs and 10k record benchmark source.

## G1 verification order (mandatory)
1. Verify pinned workerBaseSha, S0 gate, donor blob provenance and exact exclusive path diff. Do not promote old donor test logs.
2. Stage/merge B into integrator branch only, preserving donor source and existing HQ catalogue/semantic schemas. Integrate A/C/D/E/F serially under their own boundaries; never import missing A module to make B tests pass.
3. In genuine ARBOR-only Node 24 checkout run: node --test hq/catalogue-identity/test/crosswalk.test.mjs ; node --test hq/version-intelligence/identity/test/identity.test.mjs ; node hq/catalogue-identity/test/benchmark.mjs. Record command, OS, Node version, stdout and exact process exits. Review repeated-name 10k bounded work.
4. Run F conformance and combined A/B/C identity replay tests, including same repo three immutable SHAs, same declared cap ID differing owners/code, digest fork, Labs no-allowlist deny, incomplete donor no tombstone, hostile paths and no donor checkout offline replay.
5. Verify runtime Host grant/revoke and production-mount boundaries separately through H/G2. A readable source module or passing synthetic test is not device/runtime acceptance.

## Specific risks and owner handoffs
- B identity requires A exact PinnedSource/SourceManifest input and C source-delta semantics; map schema fields in G1 explicitly. A and C APIs may differ because parallel workers have not landed. Do not silently change their contracts.
- B legacy C4 data intentionally lacks cryptographic owner-signature verification. Reviewed equivalence is an annotation; F/G1 must reject forged decision receipts before claiming trusted semantic parity.
- Original C4 crosswalk's repeated-title candidate grouping bounds output but not worst-case duplicate loop work. F/G1 10k benchmark should measure and harden if a resource limit is violated.
- Operator-only CLI writes caller-selected local output; verify G1 path permissions and never expose it as a Host endpoint.
- D/X own semantic classification of the S0 361 absent HQ paths. This B lane restores the crosswalk package but does not certify 361 missing capabilities as recovered.
- G1 owns root manifests, lockfile repairs, integration and CI. Source-lock/beta-gates remain BLOCKED_NOT_BETA; absent Cargo.lock/bun.lock, CI startup_failure and Windows/Android acceptance remain separate barriers.

## Test evidence and disposition
- TESTS_WITH_COMMAND_AND_EXIT: none. No authenticated local ARBOR checkout was available from this connected-GitHub worker; live Node/Bun/Cargo commands were not executed.
- UNRUN_TESTS: complete list and exact commands in TEST-LEDGER.json. GitHub blob inspection and recursive-tree comparison were source checks, not test execution.
- SECURITY/PRIVACY: read-only identity, deny-by-default Labs observer, no personal data/provider/browser state, no second registry/store/graph/executor.
- BLOCKERS: runnable checkout/Node24 missing in worker; cross-lane integration, owner receipt trust, high-load benchmark and live Host permission all pending.
- PROPOSED_DISPOSITION: PREPARED_NOT_PROMOTED. NEXT_OWNER_HANDOFF: Z1 independently validates scope/schema/security; G1 alone stages and tests/merges after Z1 approval.
