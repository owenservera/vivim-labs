# P2-K acceptance — planned post-G executable tests / actual-vs-UNRUN

**Disposition:** PREPARED_NOT_PROMOTED. **Code in this branch:** source and DAG evidence, typed proposed interfaces, synthetic negative-case descriptors and a gated test driver ONLY. No active runtime, export importer or user evidence is installed. **No production, Host, Rust, device or real archive test was executed in this lane.**

## Status (2026-11-12)

| Suite / category | Exact source or staged path | Current lane result | Proof needed for promotion |
|---|---|---|---|
| GitHub source pin and file/blob inspection | Labs main 022608f5797e7b4a14ccc4c393bd3c5a4704a4be; W1 A dda546a, B 78a5105, D ad55fe2 | **OBSERVED_SOURCE** by connected GitHub | G independently matches frozen merged blobs |
| Labs existing synthetic T=0 tests (donor report says 28) | Labs personal-knowledge-lifecycle/runtime/tests/*.test.mjs | **UNRUN_CURRENT_LANE** | F records actual Node command, versions, count and failures |
| Prep manifest contract tests (3 test() cases) | K-knowledge/pre-g-contract.test.mjs | **CODE_WRITTEN, TEST_UNRUN** | Node 22+ command below; test runner status recorded |
| Prepared 26 negative cases | K-knowledge/negative-cases.json, gated by optional source-bound harness | **CODE_WRITTEN, TEST_UNRUN**; driver deliberately SKIPS without harness | F provides tests against imported actual B/A/D modules, records per-case outcomes |
| A HQKG personal lifecycle | W1 A native/hqkg/tests/personal_lifecycle.rs | **UNRUN_CURRENT_LANE** | cargo test command below, real Rust graph binary test observations |
| B Host owner/corpus/onboarding | W1 B apps/vivim-host/** tests | **UNRUN_CURRENT_LANE** | Node test command below after G and compatible source import |
| Cross-store evidence+graph crash recovery | Labs durable-ingestion.test.mjs plus B/A Host bridge | **UNRUN**, owner method names not proven | Durable Host receipt, journal/CLI marker and pending cursor after simulated crash |
| Consent, retention, erasure, storage quota, DCB invalidation | Proposed B/A/F owner tests | **BLOCKED_ACTIVATION / UNRUN** | Signed consent, deletion fence, no replay, protected floor and query denial |
| Windows + browser + phone end to end | G/W2 E/F owner environment | **UNRUN** | Tested devices, logs and exact Host account/grant evidence |
| Real ChatGPT export under explicit consent | B/F owner-local validation; never in Git | **UNRUN**, prohibited this lane | Opt-in on owner's local PC only; no user data shared or committed |

The source report at Labs personal-knowledge-lifecycle/runtime/ARCHITECTURE.md calls 28 Node synthetic tests previously passed. This is NOT a count from P2-K. No provider availability, site sign-in, archive completeness, production data writer, user-encryption guarantee or graph runtime success is claimed here.

## Minimal commands — run only in the appropriate environment

**At this isolated preparation branch checkout (safe, tests only):**

~~~sh
node --test .integration/proposal5/phase2/pre-g/K-knowledge/pre-g-contract.test.mjs
~~~

The three local metadata tests inspect JSON schema, 40 source entries, SHA-shaped source pins, DAG acyclicity and 26 synthetic negative cases. The 26 source-bound case tests are **SKIPPED by design** until a real owner harness is available. Therefore a green process with 26 skips is NOT an integrated ingestion result.

**After G plus F-approved selective import of Labs donor files into ARBOR, with no Labs checkout required by product:**

~~~sh
# Run from the selected importer test-package directory after its owner chooses an ARBOR path.
node --test apps/vivim-host/onboarding/personal-knowledge/tests/*.test.mjs

# Already imported W1 A and B source owners, after G has actually merged them.
cargo test --manifest-path native/hqkg/Cargo.toml --test personal_lifecycle
node --test apps/vivim-host/test/onboarding.test.mjs
node --test apps/vivim-host/corpus/host-integration.test.mjs
node --test apps/vivim-host/corpus/intake/host-intake.test.mjs
~~~

Paths under apps/vivim-host/onboarding/personal-knowledge/ are **proposed future selective placement** and do NOT currently exist in this branch. Run nothing from user-provided ZIP paths or provider conversations in test CI.

**After W2 F/B/A build a source-bound, synthetic-only harness, not a fake implementation:**

~~~sh
ARBOR_P2K_SYNTHETIC_HARNESS=./tests/personal-knowledge/real-owner-harness.mjs \
  node --test .integration/proposal5/phase2/pre-g/K-knowledge/pre-g-contract.test.mjs
~~~

The path real-owner-harness.mjs is a **proposed** owner-authored module (not created here). Its createSourceBoundHarness({caseId,syntheticOnly:true}) must invoke the imported B/A/D owner implementations; run(scenario) returns observed outcome and counters. The driver fails if it detects user data, outbound provider calls, cross-account writes, unauthorized commits, Rock Core deletion or unauthorized cursor advancement. F must validate that the harness uses real owner modules, not hardcoded statuses. A direct path into Labs must NOT be a release-time dependency.

## Negative security / correctness coverage

26 case IDs are recorded in negative-cases.json with synthetic input, owner, expected outcome and postcondition. The **smallest beta gate** should execute these first (at least 01–03, 04, 07–08, 12–19, 20, 22, 25–26) against real owner interfaces:

- **Consent/identity:** K-NEG-01–03; signed-in does not equal archive grant, account swap and revoke mid-batch reject; no unauthorized writes.
- **ZIP and parser:** K-NEG-04–11; traversal, symlink/duplicate, bomb, corrupt late CRC, invalid UTF-8, truncated/oversized JSON and file replacement reject before publish. Use synthetic in-memory ZIPs (Labs runtime/tests/fixtures.mjs) under controlled temporary directories.
- **Idempotency and atomicity illusion:** K-NEG-12–17; exact duplicate no-op, changed revision provenance, crash between graph commit and checkpoint, uncertain digest hold, forged receipt reject, and wrong Rust scope/epoch/bytes fail. Compare durable journal marker, checkpoint cursor, graph epoch and physical retained bytes before/after.
- **Storage/Rock Core:** K-NEG-18 and K-NEG-25; simulate almost-full disk, peak temp reserve and metadata growth. Protected receipts, grants, fences and owner-retained originals must remain intact. Compaction cannot lie about verification or convert raw data retention into silent deletion.
- **Erasure/cross-scope/privacy:** K-NEG-19–24 and 26; delete-all-derived fences active and replaying jobs, removes raw and graph descendants and invalidates caches/context, source prompt injection remains inert, model memories remain candidates, HQ dev session index does not consume private ChatGPT history, outbound destination drift blocked, and media omissions truthfully reported. Source restore must not resurrect revoked data.

## Specific expected receipts and rejection rules

1. A Host source grant must bind the World, verified account, provider, selected archive revision, operation, expiry, owner review and privacy class. Recheck before staging/commit, and on process resumption.
2. An inspection is exactly one of absent / committed(receipt) / uncertain. Batch identity and digest must match the source, owner, expected range and bytes. Unknown and uncertain are HOLD, never inferred absent.
3. Before advancing a checkpoint, Host verifies its own durable acceptance receipt **and** A's corresponding graph marker with matching opId/payloadDigest, not just a JSON verified:true field.
4. For a duplicate import, snapshot Host journal count, stored bytes and Rust graph epoch; all must remain unchanged. For changed revisions, source lineage and explicit supersession must be retained.
5. For erasure, fence new writes first, then delete/expire permitted content and derived products under B, erase graph effects under A, invalidate D/E/F read projections, inspect read denial and forbid replay. Report external previously-sent content cannot be recalled automatically.
6. When estimated peak write bytes exceed free bytes minus OS/reserve/Rock Core protected budget, do not start; evict only expressly rebuildable projections in existing storage, with reconstructible manifests. Zero/negative headroom fails closed.
7. All claims about real ChatGPT archives, Windows file handles, physical device onboarding, full corpus coverage, cross-store atomicity, or secure deletion remain UNRUN pending owner evidence.

## Promotion threshold

**NOW:** READY_FOR_W2_OWNER for selecting tests/contracts; PREPARED_NOT_PROMOTED source. **LATER:** B+A+F can independently promote *small* selected tests/typed adapters after G and owner review, recording exact G HEAD, source blob SHAs, command output, negative case results and unresolved risks. **No automatic promotion:** even an all-green synthetic suite does not authorize real-data import or device acceptance.
