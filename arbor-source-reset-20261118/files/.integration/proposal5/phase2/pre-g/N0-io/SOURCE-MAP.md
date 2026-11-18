# PRE-G N0-IO — SOURCE MAP (R001, source-only)

**Disposition:** PREPARED_NOT_PROMOTED. This is a read-only inventory plus isolated synthetic staging. The destination base does **not** contain Lane A/B/E/F imports until G merges them. Do not infer in-ARBOR runtime availability from a worker branch.

## Immutable inspected branch heads (2026-11-12)

| Source | Exact ref | Observed Git commit |
|---|---|---|
| Labs native I/O | owenservera/vivim-labs:lab/user-io-rust-core-20261112 | 61d9fc15c55144c4f0b79afa248981c2b0ca9bd6 |
| Web Harness native R001 | owenservera/vivim-web-harness:work/native-user-io-r001-20261112 | fa6dea9576c83496d95562b98b7a815fda4aec88 |
| ARBOR pre-G base | owenservera/ARBOR:work/p5-pre-g-convergence-r001 | 8b23c73bf450738a76b256bf0fa17ecbf86db591 |
| ARBOR native owner A | owenservera/ARBOR:work/p5-w01-a-native | dda546a11294920d6521a0f64ab7ee62ec993c70 |
| ARBOR Host owner B | owenservera/ARBOR:work/p5-w01-b-host | 78a5105a9c1f1880d658f350e130b1e2632d3532 |
| ARBOR UI/phone owner E | owenservera/ARBOR:work/p5-w01-e-ui | 1a5cf26ce14ebce4e31d9a4373060ace61852645 |
| ARBOR verification owner F | owenservera/ARBOR:work/p5-w01-f-verify | be458f3c4cdb7eb38d47b1de2a00d441b7cdc704 |

## Source file Git blob SHA-1 pins (direct reads or directory census)

These are Git blob identifiers returned by direct connected GitHub file reads **or** source directory listings; a directory SHA pin is not proof that every file body was inspected. The architectural and implementation claims below are grounded in targeted direct source reads, while supporting manifests/catalog paths are cross-checked through the directory census. Every path belongs to the repository/ref column above.

| Ref | Actual file path | Blob SHA |
|---|---|---|
| Labs | user-io-rust-core/PARALLEL-EVENT-FABRIC-ARCHITECTURE.md | 54d942eca63cd339adb8d7a959d3badf32afd0c1 |
| Labs | user-io-rust-core/USER-IO-CONTRACT-AND-MIGRATION.md | 3928de100a6a98183c1c6e0e8cf9c002497c5bd4 |
| Labs | user-io-rust-core/PREDICTIVE-INTERACTION-FAST-PATH.md | 358e6898fe8dbdf5732bd9778a8ce8071e1e91ee |
| Labs | user-io-rust-core/node-0-prime/N0P-02-TARGET-ARCHITECTURE-AND-ADRS.md | a71393fb5fdca89dbbfb0a4ab7e177e892beafd9 |
| Labs | user-io-rust-core/node-0-prime/N0P-04-SOURCE-TRUTH-AND-SECURITY.md | 72c71c11b916af1202d80b3b64a514b544503835 |
| Labs | user-io-rust-core/rounds/R001-01-NATIVE-EVENT-FABRIC-PRD.md | 69432c2e5aa097e05e71af984277f918603a2835 |
| Labs | user-io-rust-core/rounds/R001-03-NATIVE-OUTPUT-AND-UI-BOUNDARY-PRD.md | 0cde2b1c648f5529950dc7423ce4e287cd4c6214 |
| Labs | user-io-rust-core/rounds/R001-05-SOURCE-TRUTH-DELTA.md | 4044674c2df6bb07d9fd72ae3cd8a18b001428db |
| Harness R001 | core/programmability/src/user_io.rs | d8a53bb2a9216721323790ca96b9517af6528bec |
| Harness R001 | core/programmability/src/lib.rs | 39a5142908956a1149a7a81a45b9f460e96222db |
| Harness R001 | core/programmability/USER-IO-R001.md | ee703b54b314c040a5ce831153bd2252b84db24a |
| Harness R001 | src-tauri/src/main.rs | fe7b3ff7c0b5451f336c873cc71a11c7966a4bbd |
| A | core/programmability/src/lib.rs | fe42cb6bf535c91d3ba05911977999a9d8f8c254 |
| A | core/programmability/Cargo.toml | 9c3a8e997897a8aa4079ecbef420adc1af41c090 |
| A | src-tauri/src/main.rs | 4b68350ac90cf87968dba6876036d0a74658d58b |
| A | src-tauri/Cargo.toml | 3985b0749263a336fa5f8049f8ad239132151a84 |
| A | native/hqkg/src/engine.rs | fbb1100999fd229fb5a4352a78bbb0d77b643988 |
| A | native/hqkg/src/typed_dag.rs | 24d4ef22ab76f1c8832f5c426b29860c74fac2a5 |
| A | native/hqkg/src/invalidate.rs | 271b86b48bb79aa8895d712e4335bdf442cd5bfb |
| A | native/hqkg/Cargo.toml | b90c06fa3035ad7b8b7cdd2e97593c3e4fd2b19a |
| A | integration/worker-reports/PREG-A.md | 3b4bf897bcb2e255456ef46404a8840bff5b535a |
| B | apps/vivim-host/control/dispatcher.mjs | 723865482b8a0f98ab3214b2ec6a83aaea629c43 |
| B | apps/vivim-host/control/action-catalog.v1.json | c423d5933c28ca9f534028aeaffd7929f767c269 |
| B | apps/vivim-host/control/intent-planner.mjs | 3d1bbcd5ad80ffa6276ea4e5d2b5f0cfbd8cf4ab |
| B | apps/vivim-host/control/journal.mjs | b9739a00c8345a0513dba32ddc0361e96bb9b16f |
| B | apps/vivim-host/direct-phone.mjs | 6bb604cf7a64e4623c28bc86495f39a1f40f1626 |
| B | apps/vivim-host/control/cli.mjs | d88284146ed829f4c7f6ab9fc0d10ce7ba9719e9 |
| B | integration/worker-reports/PREG-B.md | b3e4947347211bd4446890e09985103afe2cd37a |
| E | apps/mobile-relay/control-client.mjs | 59ee97e51f6fddbd99fe3ef9adfd03ed5170d02d |
| E | apps/mobile-relay/app.js | 9591deb81f1c76bddbcf7db631143af3a0c3410e |
| E | packages/tierflow/src/visual-runtime/command/compile.ts | 4cc5b63bea409171e2b4b0a29398c87907cd35ea |
| E | integration/worker-reports/PREG-E.md | a2944217c2a42c377c8aa3ee2aa4efe9d255295a |
| F | contracts/interop/v1/types.ts | e329eb1b8506938c8fc981b426c51a3582ae03c7 |
| F | contracts/interop/v1/protocol.schema.json | d13c0e6d5065400224bf7a4bbf89aa9a92f4156c |
| F | contracts/interop/v1/validate.mjs | 25c0b36b11f2e6f3b003920e14e166a8f74354b7 |
| F | integration/worker-reports/PREG-F.md | a42e1162fd87ed3c74bd7dd6a1d484af91dfb855 |

The Labs node-0-prime/ and rounds/ directory listings were separately inspected; their directory Git tree identifiers are 3dcaa0a281991c86e07cbe94f8f4739e92968c8d and 80df5a82d92c31e6b4e9b4a81d31b75930f3f334 respectively. Other files in those directories were not read in full and are not claimed inspected.

## Actual entrypoint and destination matrix

| Surface / flow | Current source fact | State on independent owner branch | Missing W2 integration/proof |
|---|---|---|---|
| Windows keystrokes/editor | A src-tauri/src/main.rs boots Node Host and exposes pure validate_native_program. Harness R001 adds user_io_event Tauri command, unconsumed by live UI. | A = IMPORTED_OLD_SOURCE; Harness R001 = IMPLEMENTED_EXPERIMENT_ONLY | A-verified webview caller and trusted process origin; bounded first-hop, no owner-token leak, real COM/IME/source tests |
| Windows native semantic result | R001 user_io.rs returns one final synchronous read-only projection after four pure reactors; no incremental stream. | IMPLEMENTED_EXPERIMENT_ONLY | Revisioned partial output stream, cancellation, bounded queue; never block glyph/caret paint |
| Phone PWA inputs | E control-client.mjs makes authorized /mobile/api/control/ calls through B direct-phone.mjs; not a call to native Rust. | E and B = IMPORTED_SEPARATELY | B authenticated session-to-PC native transport; session/origin/grant verification, replay protection, no device-to-Rust fake direct call |
| CLI | B control/cli.mjs uses the Host control path; A HQKG command line tools exist for development/research. | IMPORTED_SEPARATELY | A/B verified local IPC actor scope; CLI cannot impersonate interactive owner approval |
| Voice | Labs VoicePartial/VoiceFinal protocol is DESIGN; F imported synthetic voice-to-tasks fixtures, not microphone-to-Rust ingress. | F = TEST_FIXTURE_ONLY | E/A voice capture, consent, partial coalescing, final commit digest, speech source provenance |
| Accessibility | Local screen readers/IME focus remain OS/UI owned; Labs specifies equivalent semantic projection, not a proven native output adapter. | DESIGN_ONLY | E accessible live-region/focus tests, user choice, no output content leakage |
| Automation | Labs describes machine/scheduled actor with separate scopes. Existing native event API has no authenticated automation initiator. | DESIGN_ONLY_FOR_SHARED_IO | B/A explicit identity, bounded read/policy, no approval elevation or background effects |
| Predictive source/habits | R001 TrustedSources installed only by native code; Tauri default is empty; accepted Tab selections only ephemeral. | IMPLEMENTED_EXPERIMENT_ONLY | B authoritative catalog/context read source; consent; bounded retention; explicit opt-in |
| Output streams | R001 single joined snapshot; Labs revisioned partial stream/deltas is DESIGN. E TierFlow compiler is pure/inert visual command IR. | IMPLEMENTED_PREVIEW + DESIGN_ONLY | A source-labelled ordered deltas with reconnect fallback; E audience-filtered rendering and screen-reader equivalent |
| Approval | B dispatcher.mjs has plan/digest, current grant/approval and Host-only claim path; A/R001 cannot approve. | B = IMPORTED_ON_B_NOT_G | B exact admitted plan bridge only after native identity proof; no Rust approval writer |
| Receipts / uncertainty | B journal/dispatcher emits receipts; E control-client checks matching digest/plan and treats lost execute response as UNCERTAIN. | B/E = IMPORTED_SEPARATELY | F negative tests and Host-verified receipt references projected as observed; never synthesize applied |
| Graph/DAG | A native/hqkg engine.rs, typed_dag.rs, invalidate.rs present; typed DAG has revision and cooperative cancellation, Scheduler is synchronous. | A = IMPORTED_ON_A_NOT_G | A reuse existing engine only; validate bound, stale epoch, abort/cancel and no new graph runtime |
| Semantic UI command | E TierFlow compile.ts uses exact crosswalk and inert compilation; B's JS intent planner remains current action interpretation source. | SEPARATE_PRE_G_SOURCES | F diff parity; B catalog single authority; semantic candidates never implicit effects |

## Non-negotiable architecture

- One verified PC-native ingress with bounded immutable event fan-out; **NO CENTRAL SEMANTIC ROUTER**, no chosen first interpreter. Independent subscriber declarations + existing Rust graph/DAG dependencies handle pure work.
- A draft has only an ephemeral per-session privacy budget; preserve original text, local IME/caret/undo and Unicode spans.
- Source permissions must be enforced **before** a reactor receives source bytes. Actor, approval, grant, Host claim, or receipt must never be trusted from client input.
- Each derivation pins session/input revision, graph epoch, source owner/ref, policy and engine version, audience, evidence, TTL and cancellation. Conflicts are explicit ambiguities.
- Host B alone controls active catalog, committed plan admission, approval, exclusive effects, journal and receipt. Claimed-without-observed-result remains UNCERTAIN, no resend.
- F owns generic interop/Chronos differential through P2-C. N0-IO only stages a specialized native-I/O envelope and synthetic proof. No competing schema authority is installed.

**Negative source result:** the 19-entry Host action catalog (v6) has no canonical full-project-update action. Harness R001 fixture.full.update is synthetic and unavailable as a real Host action. No user data or external source collection was ingested.
