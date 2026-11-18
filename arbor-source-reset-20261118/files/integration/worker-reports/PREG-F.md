# ARBOR Proposal 5 — PRE-G Lane F FINAL FREEZE

**Verdict: FREEZE_READY for G source/evidence integration only. NOT beta GO, BUILD_PASS, HOST_WIRED, SECURITY_ACCEPTED or DEVICE_ACCEPTED.**

- Repository: `owenservera/ARBOR`
- Frozen branch: `work/p5-w01-f-verify`
- **Frozen HEAD:** the exact Git commit introducing **this** `PREG-F.md`; master G/referee MUST resolve `refs/heads/work/p5-w01-f-verify` and pin its immutable SHA after this commit. A self-referential commit SHA cannot be embedded inside the committed file.
- **Last code/evidence HEAD before report:** `7538ebc706f98c15530fe39ba3a40b479448b37f`
- **Original common base:** `dd245d78ff3c110ba8630cad557ea0479c51a635`, not an approved W0-G baseline.
- **Integration branch:** `integrate/proposal5-beta-r001` absent in pre-G review; G exclusively creates/merges it.
- Freeze work order: `work/p5-pre-g-convergence-r001:.integration/proposal5/pre-g/prompts/R2-F-FINAL-FREEZE.md`. Refreshed decision and original convergence read. **No additional feature/code work in this final freeze.**

## 1 — Exact source pins and physical source inventory

| Donor | Pinned source commit | Material physically imported on F | Exact proof |
| --- | --- | --- | --- |
| `owenservera/vivim-labs` | `022608f5797e7b4a14ccc4c393bd3c5a4704a4be` | **7 / 7 exact donor files**, byte-for-byte verified | `tests/migration/W01-F-ledger.v1.json`, `W01-F-proof.v1.json` |
| `owenservera/vivim-web-harness` | `cae97afe848d79957c9d6c698a90996a758bfd56` | **0 executable Host donor files.** Derived **test-only** catalog baseline from `control/action-catalog.v1.json` v6 + journal read semantics | B is the exclusive real Host importer |
| `owenservera/repos-hq` | `9b922ea049dbadc94e504b5b9432743995c020ca` | **0 original HQ files**; source-derived identity/evidence contract only | D alone imports HQ source |
| `owenservera/tierflow` | `50822f11ee63c3d80f631f8fea7dcb9b19bffc38` | **0 original TierFlow files**; inert-view typed mirror only | E alone imports renderer |

**Exact 7 source-preserving donor files → ARBOR destination:**

1. `vivim/voice-to-tasks/workbench/fixtures.json` → `tests/fixtures/labs/voice-to-tasks/fixtures.json`
2. `vivim/voice-to-tasks/workbench/trace-check.mjs` → `tests/fixtures/labs/voice-to-tasks/trace-check.mjs`
3. `capability-observatory/data/capabilities.json` → `tests/fixtures/labs/capability-observatory/seed.json`
4. `capability-observatory/schemas/capabilities.schema.json` → `tests/fixtures/labs/capability-observatory/schema.json`
5. `chronos-toolkit/wave2/repos-hq/contracts/chronos/contract.v1.json` → `contracts/chronos/staged-contract.v1.json`
6. `personal-knowledge-lifecycle/runtime/src/chatgpt-normalize.mjs` → `tests/fixtures/labs/personal-knowledge/src/chatgpt-normalize.mjs`
7. `personal-knowledge-lifecycle/runtime/tests/chatgpt-normalize.test.mjs` → `tests/fixtures/labs/personal-knowledge/tests/chatgpt-normalize.test.mjs`

Original donor **blob SHA for each source and target**, implementation status, privacy class and retirement condition: `tests/migration/W01-F-ledger.v1.json`. These are synthetic/pure/test artifacts, not a second personal knowledge ingestion owner or runtime.

**ARBOR-authored files added beyond the 7 exact donor files:**
- `contracts/interop/v1/{README.md,protocol.schema.json,types.ts,types.rs,validate.mjs}`: minimal source/ref, revision, capability, evidence, event, status and inert view mirrors. Rust type file is not yet Cargo-linked.
- `contracts/extension-ports/source-hooks.v1.json`: **descriptive, non-installing** extension seam census.
- `capability-spaces/source-truth.v1.json`: read-only classification inventory; all `releaseReady=false`.
- `tests/compatibility/{host-actions.baseline.v1.json,synthetic-projections.v1.json,wave1f.test.mjs}`: Host catalog drift baseline and synthetic source/receipt cases.
- `tests/privacy/{w01-f-source-lint.mjs,w01-f-source-lint.test.mjs}`: scoped donor-path/secret-shaped-file guard; **not** a complete security scan.
- `tests/migration/{W01-F-ledger.v1.json,W01-F-proof.v1.json}`: exact provenance and observed test ledger.
- `integration/worker-reports/W01-F.md`: original handoff.
- **This file**: `integration/worker-reports/PREG-F.md` (report only).

**Deliberately excluded:** Labs Node 0 R001 mutex/thread-per-edit implementation (not present at referenced Labs main path, separate uncompiled draft); full R007 visual lab; private ChatGPT exports, transcripts, audio, device profiles, credentials, World databases; Labs T0 coordinator/write store and ingestion adapters; Chronos daemon/scheduler; third-party extension loader/marketplace; AutoWiki/Rock Core new implementation; second Host catalog, journal, native graph engine or UI publisher.

**Status taxonomy here:** `IMPLEMENTED` = code observed at source, `VERIFIED` = synthetic/source-side test material, `PREVIEW` = conditional/non-effectful, `DESIGN_ONLY` = specification, `DEFERRED` = intentionally not shipped, `BLOCKED` = missing proof/dependency. No source-status label asserts availability or device acceptance.

## 2 — Complete Lane F commit chain, including prior focused work

| SHA | Change type | Exact change |
| --- | --- | --- |
| `106d7cfd230294d50241524401bdab5b2edb3244` | **SOURCE_IMPORTED + CONTRACTS/TESTS** | Initial 21 files: 7 exact Labs donor copies + 14 ARBOR-authored contract/evidence/test files (includes `W01-F.md`), no other lane files |
| `43cf1edaf87d7b6d52c1e64ccdee597656700beb` | **SOURCE_PATCHED** | Fix over-escaped source-ID control-character validator in `contracts/interop/v1/validate.mjs` after smoke caught false refusal |
| `7538ebc706f98c15530fe39ba3a40b479448b37f` | **EVIDENCE ONLY** | Updated `W01-F.md` + added `tests/migration/W01-F-proof.v1.json`, 25 V8 pure assertions and 7/7 donor equality recorded |
| **This report's new Git commit (resolve branch tip)** | **DOCUMENT ONLY** | `PREG-F.md`; nothing executable or other lane changed |

The already-completed first focused follow-up and actual verification are fully accounted for by `43cf1eda...` and `7538ebc7...`. **No unreported second runtime implementation or new test execution.** The branch's common parent is exactly `dd245d78...`; G must reconcile the later architecture changes during serial merge. No root workspace, lockfile, other worker branch or `main` edits.

## 3 — Canonical authority/contract owners and exact Host regression baseline

- **B Host** owns action catalog v6, 19 source actions, World, owner grants, planning, approvals, exclusive journal effect fence, claim/receipt/recovery and single effect writer. **F's JSON is test-only**, never a dispatcher or alternate availability catalog.
- **A Rust/Tauri** owns native origin/actor/process attestation and HQKG computation, with **no duplicate Host effect authority**.
- **C Studio** owns signed-in Chrome/CDP, exact provider/thread targeting, local send-once/uncertain outcome fencing; must flow through B Host effect/admission.
- **D HQ** owns in-ARBOR Development-domain index/search/context/wiki with source identity, allowlisted roots and grant-derived read scope, never Personal World data.
- **E PWA/TierFlow** owns mobile presentation, browser signer and inert visual projections; Host remains only device/grant/effect/publisher authority.
- **F** owns interop/schema baselines, evidence classifications, differential and negative tests.
- **G** alone owns serial source merger, root Bun/Cargo and lockfiles, release/CI integration, final combined acceptance.

**Host v6 exact action ID → risk/readiness, in test baseline** (`source_wired` except explicitly noted):
`control.catalog.list R0`; `vivim.status.show R0`; `devices.summary R0`; `devices.approve R2`; `devices.revoke R2`; `personalization.context.summary R0`; `personalization.preference.list R0`; `personalization.preference.add R1`; `personalization.preference.remove R1`; `ui.theme.show R0`; `ui.theme.set_preset R1`; `ui.theme.undo R1`; `privacy.hard_no.list R0`; `privacy.hard_no.add R2`; `privacy.hard_no.clear R2`; `control.lexicon.teach R1`; `control.lexicon.forget R1`; `vivim.network.pause_egress R2 (enforcement_missing)`; `machine.network.disable_all R3 (unsupported)`.

**Invariants to test after G:** original ID, owner, risk, readiness and exact catalog digest; R3 never auto-admits; R2 requires actual desktop owner approval; plan digest/expiry/stale policy; a Host journal `claimed` without receipt projects **`uncertain`**, no automatic resend. TierFlow read statuses `available/permitted` or `reported_applied` do **not** certify a verified Host effect or grant. HQ `name@version` semantic registry is not a Host permission registry; synthetic Labs seed `integrated_test` is not `AVAILABLE_NOW`.

## 4 — Executed test evidence versus UNRUN (no inflated exit codes)

| Check / concrete command | Observed status | Exit code |
| --- | --- | --- |
| Connected GitHub source/commit/branch/ref and donor file reads | **PASSED SOURCE INSPECTION**: source pins, import paths and final pre-report head observed | **N/A** (API calls, not process) |
| Exact 7 Labs source files vs corresponding donor GitHub blobs/content | **PASSED**: **7/7 identical**, no changed donor bytes | **N/A** (API comparison) |
| Authored pure contract assertions run in connector V8 against committed source | **PASSED** **25/25**: source owner/revision, stale/refusal, Host uncertain claims, synthetic voice traces, Chronos staged, taxonomy and catalog count; after validator correction | **N/A** (in-process V8 checks, not shell) |
| JSON.parse and source count during commit construction | **PASSED** static authoring check | **N/A** (in-process check) |
| `node --test tests/compatibility/wave1f.test.mjs tests/privacy/w01-f-source-lint.test.mjs tests/fixtures/labs/personal-knowledge/tests/chatgpt-normalize.test.mjs` | **UNRUN**: first full on-repo Node regression; must run after G | **N/A** |
| `rustc --test contracts/interop/v1/types.rs`; root `cargo test --workspace` | **UNRUN**: Rust mirror not linked/compiled | **N/A** |
| `bun test` + TierFlow/Host/Studio full suite | **UNRUN** | **N/A** |
| Native adversarial child spoof / owner-token/IPC cases | **UNRUN** | **N/A** |
| Browser HTTPS PWA enrollment/WebCrypto DER/Host static route, active bearer replay | **UNRUN** | **N/A** |
| HQ Node24 `node:sqlite`/FTS5/dev-root auth/revoke + leakage tests | **UNRUN** | **N/A** |
| Signed-in ChatGPT/Claude/Gemini, Windows MSI, physical Android/PWA, backup/rollback, no-donor installation | **UNRUN** | **N/A** |
| Full credential/path/privacy scan of all six combined imports | **UNRUN** | **N/A** |

Do **not** convert GitHub source inspection, V8 test results, local worker self-report or test-file existence into a Node/Bun/Cargo/Windows/device exit 0. E's first-follow-up report described **8 synthetic browser tests passed locally on Node 22.16.0**, but its freeze explicitly says no raw CI/per-command exit codes independently verified; **SOURCE_ONLY / SELF_REPORTED_LOCAL_PASS**, not ARBOR integrated acceptance.

## 5 — G-owned combined-source test matrix (run only after all six frozen tips merged)

| ID / priority | G combined check and negative case | Source precondition / owner | Current truth |
| --- | --- | --- | --- |
| **F-G01 P0** | Run Host catalog v6 **19 exact IDs/risk/readiness** differential; unknown or `R3` refuse; R1/R2 exact approval, plan digest, claimed/no receipt = uncertain and never resend | B Host + F baseline | **SOURCE_ONLY; NODE UNRUN** |
| **F-G02 P0** | Studio complete import and dependency closure: `studio/src/config.mjs`, workspace manifest, `studio/bin/studio.mjs`, bridge commands, Host bootstrap adapter, relay correlation, browser/driver lease, provider thread/sendOnce; explicitly address **305 omitted Studio test files** | C 501 exact imported blobs, 589 exclusions; B/G own integration | **SOURCE_IMPORTED; BUILD/PROVIDER UNRUN** |
| **F-G03 P0 (S1)** | Fake loopback :43187 HTTP-200 or fake health; wrong child/PID/launch secret/version, forged HMAC, nonce replay, double UI code, crash/race and foreign IPC origin **must fail closed without durable owner bearer disclosure** | A native + B Host design, root security upgrade G | **SECURITY_BLOCKED; TEST UNRUN** |
| **F-G04 P0 (S2)** | Browser PWA WebCrypto P-256 nonextractable IndexedDB key, SPKI/DER vs Node verifier, literal `\\n` signed transcript; lost key refusal; host `GET /mobile/browser-device-key.mjs` **requires B-owned allowlist patch** | E browser signer SOURCE_PATCHED, B Host unchanged static list; E patch `W02-E-B-HOST-STATIC.patch` | **SOURCE_ONLY; HOST ROUTE/DEVICE UNRUN** |
| **F-G05 P0 (S2)** | Stolen active `x-vivim-grant-id` + `x-vivim-device-session` replay on privileged request must fail under agreed request-bound device proof/nonce/method/path/body; isolated stolen grant ID insufficient to mint session | B existing validator lacks per-request proof; E must match contract, F negative test | **SECURITY_BLOCKED; UNRUN** |
| **F-G06 P0** | HQ `allowedRoots` and trusted `actorId/sessionId/sourceId` from Host; traversal/symlink/junction race refusal; forbidden source, stale source, revoked source and `source.revoke` cleanup; Personal World cannot be read or written via Development HQ index | D 315 source imports + Host-mode adapter SOURCE_PATCHED; B provides real grant verifier | **SOURCE_ONLY; NODE/Windows UNRUN** |
| **F-G07 P0** | Build/install/run ARBOR with donor sibling checkouts renamed or removed: no donor path imports, processes, sockets, DBs, symlink, external HQ CLI, cloud fallback, second scheduler or second UI publisher | A–F imports, G Cargo/Bun lock and installer | **UNRUN** |
| **F-G08 P0** | One Windows installer, patched Tauri/capability/origin, paired real HTTPS phone PWA, approve/revoke/session expiry/reconnect/offline, same Host identity/status, uncertain receipts and restart recovery | A+B+C+D+E+F+G | **BUILD_UNRUN / DEVICE_UNRUN** |
| **F-G09 P1** | TS/Rust JSON source ID, owner scope, revisions, evidence digest, IME and effect-status parity; TierFlow inert UI shows no false approval; clock due never grants action; voice proposal requires Host receipt; source truth/observatory never reports false production availability | A HQKG, D HQ IDs, E TierFlow, B Host, F schema/fixtures | **25 V8 PURE PASSED; CROSS-LANGUAGE/DEVICE UNRUN** |
| **F-G10 P1** | Synthetic personal ChatGPT export normalizer negative/malformed/size/omitted media cases; consent/revoke/source binding; no unapproved private data imported or sent | F selected fixture only; B/D actual World/graph port absent | **SOURCE_ONLY; NODE + HOST INTEGRATION UNRUN** |

**G execution order:** (1) pin all six final worker HEADs from refs and reconcile common base; (2) source-lock and inspect combined complete recursive tree (not 300-file capped compare); (3) root dependency/lock/package closure and source tests; (4) negative Host/Studio/HQ/PWA cases; (5) security S1/S2 W2 fixes and signed-in provider; (6) Windows installer/physical phone/no-donor/recovery acceptance. Do not start Phase2 T/K/C implementation here or block source integration on a proposed feature.

## 6 — Cross-owner blockers and final classification

- **SOURCE_IMPORTED:** seven exact Labs files + F test-only baseline/schema/status/fixtures committed; **7/7 donor byte equality**. B real Host actions imported on B branch; C Studio 501; D HQ 315; E mobile/TierFlow 270 original selected; A native 99; **not combined in F branch**.
- **SOURCE_PATCHED:** one F pure validator correction (`43cf1ed...`). D and E each have separate owner-side patches; none were applied by F. No second authority introduced.
- **BUILD_UNRUN:** F Node/Bun/Cargo/TS and whole ARBOR release unexecuted; native Tauri family/privileged origin and A↔B launch protocol P0 S1 still unsafe/unimplemented. G/W2 must fix.
- **DEVICE_UNRUN:** browser PWA Host static module missing, signer DER/WebCrypto physical behavior and privileged stolen-bearer replay P0 S2 unresolved; Windows and signed-in Chrome/phone acceptance not run.
- **Cross-owner integration blockers:** B/G apply E Host allowlist patch only through B ownership; B/E/F agree active bearer replay protection; A/B secure native process-bound launch; D/B exact dev-source authorizer and revocation; C/B signed-in send-once + dependency closure; A/D/F revision/graph identity, E visual inert projection. G alone resolves combined root manifests, importer lockfiles, source/fixture exclusions and actual CI gates.
- No new Labs runtime, external repository wiring, personal data import, generic plugin marketplace or independent effect writer approved.

**Final stop condition:** once this file is committed, report the exact immutable branch-tip SHA in the worker conversation, declare **FREEZE_READY**, and STOP. No other Lane F writes unless master G deliberately reopens the lane.
