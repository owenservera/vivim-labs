# R001-04 — PRD: Integration, Verification, Release Gates & Next Engineering Handoff

Date: 2026-11-12
Status: CODE COMMITTED / TESTS UNRUN / FULL RUST↔UI↔HOST E2E BLOCKED.

## Verified source baselines
- UI Labs R005/R006 source: \`lab/nlp-visual-command-ui-20261111\` at \`20f393c953e2b6d83201d18fee2f2dd56deeaa68\`.
- Host baseline: \`main\` \`cae97afe848d79957c9d6c698a90996a758bfd56\` before R001 branch.
- HQ baseline: \`main\` \`7efb52b251fd56c9257a3184ac39dc19d84340b5\`.
- Isolated source patch: Harness \`work/native-user-io-r001-20261112\` (inspect current branch head; it advances as R001 files are committed).

## Code landed (source presence verified, not runtime verified)
| Path | Concrete responsibility | Current status |
| --- | --- | --- |
| Harness \`core/programmability/src/user_io.rs\` | Typed event, trusted source snapshots, four independently scoped Rust reactors, deterministic signal order, revisioned projection, in-memory explicit-accept scoring, Tab CAS and inert Submit, strict JSON adapter, unit tests | SOURCE_IMPLEMENTED / BUILD_UNRUN |
| Harness \`core/programmability/src/lib.rs\` | Exports \`pub mod user_io;\` | SOURCE_IMPLEMENTED / BUILD_UNRUN |
| Harness \`src-tauri/src/main.rs\` | Registers read-only \`user_io_event\` command with a native Mutex state instance, no catalog/Host personal sources by default | SOURCE_IMPLEMENTED / WINDOWS_UNRUN |
| Harness \`core/programmability/examples/user_io_r001.rs\` | Synthetic \`full\` + 2 Tabs demonstration using fixture IDs and accepted pattern | SOURCE_IMPLEMENTED / RUN_UNRUN |
| Harness \`core/programmability/USER-IO-R001.md\` | Local run instructions, source boundaries and limitations | COMMITTED |
| Labs \`user-io-rust-core/rounds/R001-*.md\` | Full R001 engineering PRD suite | COMMITTED |

## Round 1 verification matrix (no test is claimed passed)
### Native compile
- \`cargo test --manifest-path core/programmability/Cargo.toml user_io -- --nocapture\`
- \`cargo test --manifest-path core/programmability/Cargo.toml\` (regression)
- \`cargo run --manifest-path core/programmability/Cargo.toml --example user_io_r001\`
- \`cargo check --manifest-path src-tauri/Cargo.toml\` on Windows (native deps/environment)
- Confirm \`src-tauri/tauri.conf.json\` capabilities allow only intended app webview to invoke native endpoint; do not expose trusted personal sources through untrusted origins.

### Behavioral
- One DraftChanged -> four independent computations; evidence carries input revision, named reactor.
- Same inputs lead to deterministic evidence/convergence regardless of subscriber finish order.
- \`full\` -> native template candidate -> currently trusted project ID -> next-steps focus ranked from accepted stats.
- Tab 1 binds exact template+project, Tab 2 focus, Submit returns an inert marker; no Host actions.
- Wrong revision, repeated sequence, missing project/template, active IME, negation, invalid wire fields -> refusal.
- Project/catalog source refresh invalidates old choices; no stale project suggested from prior source snapshot.
- No owner source installed -> no project or canonical command suggestion. Fixture is clearly identified as synthetic.
- Privacy/no effect: no permission mint, browser send, persistent graph write or private-memory export.

### Future E2E cutover gates
- Connect trusted, actor-scoped Host source read adapter for current command catalogue and known project context; no client-supplied "trusted" values.
- Compare Host \`intent-planner.mjs\` semantic output against Rust on shared golden corpus, especially negation, compound clauses, aliases, action risk, arguments and personal rules.
- Connect UI Labs/TierFlow adapter read-only; capture visible Tab interaction and IME/caret from Windows desktop.
- Connect authenticated paired PWA to same Rust process without extra semantic planning and run real device tests.
- Integrate reusable Rust HQKG/typed DAG, namespace isolation, data lineage and invalidation without path-copied crates or a competing graph.
- Replace 4 threads-per-event with bounded shared native scheduler/engine registry; cancellation, priority and p95 measurement.
- Add more modalities: voice final, touch/select, corrections, offline drafts, screen reader, explicit approvals; no false claim that R001 source already covers these.
- Bridge real submitted plans to **existing Host admission/journal/effects/receipts**, preserving current authority and uncertain-send recovery; no unauthorized side effect from speculative computation.
- Prove recovery, synchronized revisions, source revocation, and no duplicate effects before removing JS parser paths.

## Definitions of status
- DESIGN: planned contract without source code.
- SOURCE_IMPLEMENTED: code added, neither build nor runtime proven.
- BUILD_PASSED: actual successful native toolchain compilation/test log.
- INTEGRATION_VERIFIED: existing Host/graph/renderer connected with observed behavior.
- DEVICE_ACCEPTED: Windows packaged build and physical phone UX verified.
- PRODUCTION_GO: all required security, behavioral, recovery and performance gates passed with owner acceptance.

Never promote a status because a test file exists or a source field says \`verified\`.

## Known gaps / defects requiring follow-through
- **G001** Default native Tauri state intentionally has no real source catalog/project; two-Tab demo only works with source-injected Rust fixture. Correctly shows no known candidate in default product invocation.
- **G002** Tauri event command is registered but current Host HTML/JavaScript page is not connected; native source proof does not establish live browser I/O.
- **G003** Prototype uses one thread per reactor per event; needs shared bounded scheduler and cancel/coalesce policy before production.
- **G004** Source proof is structural only; no authenticated Host catalog, DCB, personal graph or HQ index read-port yet.
- **G005** No cargo/rustc toolchain in this authoring environment, so compilation, native tests and Windows ABI checks are unrun.
- **G006** Only text drafts and two-Tab selection exist in native code; no full speech/haptics/assistive/surface-parity protocol.
- **G007** Future Host effect bridge must preserve existing journal claims and one-send semantics.
- **G008** Current test corpus is synthetic; need differential against current NLCL/Host/TierFlow registries and actual owner-machine sources.
- **G009** The native R001 adapter holds one Mutex around a full pure computation cycle; should move work outside lock and use committed CAS snapshot under a bounded executor before high-frequency use.
- **G010** Source update currently invalidates all sessions rather than graph-selective dependencies; refine using shared graph invalidation in next round.
- **G011** No personal persistent learning/forgetting; only in-memory explicit Tab accept counts in the code.
- **G012** Security trust of Tauri-origin calls and per-actor session binding must be proven before any private source adapter is enabled.

## Roadmap integration constraints
Harness source branch isolated from UI Labs and HQ concurrent work. Do not merge/retire JS paths merely because native module compiles. Prefer source-derived native adapters with matching owner identity. Rust graph-node engine in HQ is a shared computational foundation but must first be safely packaged/imported. TierFlow controls presentation and guarded UI publishing. The phone PWA is first-class; APK is not a R001 blocker.

## Acceptance artifact and handoff format
For each gate: \`gateId, scope, repo, baseSha, headSha, file:line, command, environment, exitCode, observedOutput, proofKind, status, responsibleOwner, rollback\`. No secret/private content in public Labs. User-facing status should separate unrun tests from failed tests.

## Immediate next local integration order
1. Fetch isolated Host branch and run the pure crate tests; fix compile errors before feature expansion.
2. Run synthetic example and Tauri cargo check; capture exact output/errors.
3. Install a trusted read-only source projection from current Host action catalog and current project World under authenticated actor context; no user-supplied source authority.
4. Connect only the Lab/Host UI **read-only** and test real \`full\` + Tab + Tab; preserve current JS as default until parity.
5. Backport only verified native semantic primitives to shared Rust graph engine/typed DAG integration; avoid forks.
6. Expand to bounded async graph reads and verified source revisions; then plan production rollout after extensive security & device testing.

**Release verdict now: SOURCE CHECKPOINT COMPLETE, NATIVE BUILD UNRUN, PRODUCT CUTOVER BLOCKED.**
