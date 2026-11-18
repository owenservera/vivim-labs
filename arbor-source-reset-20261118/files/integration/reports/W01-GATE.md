# ARBOR Proposal 5 — Original W01-G Serial Integration Gate

**Date:** 2026-11-12  
**Decision:** **NO_GO / REPAIR_REQUIRED** — SOURCE_MERGED; BUILD_UNRUN; SECURITY_BLOCKED; DEVICE_UNRUN.  
**Branch:** `integrate/proposal5-beta-r001`  
**Initial architecture baseline:** `6ab7e6976c02cac2ee71f030deda3f6541f8102f` (8 documentation changes ahead of common worker ancestor `dd245d78ff3c110ba8630cad557ea0479c51a635`).  
**Sixth source merge:** `0b3474a5cb2053d607453b18d319d225f1d3ca57`.  
**Gate report commit:** the Git commit introducing this document. Read the final branch ref after committing; a commit cannot contain its own SHA.

## 1. Actual serial GitHub integration

| Lane | Final frozen worker SHA | Merge PR | GitHub merge SHA | Accepted scope |
|---|---|---:|---|---|
| A | `dda546a11294920d6521a0f64ab7ee62ec993c70` | #2 | `57ae0bae83b47fadeea2cc6038365e7d3f577fe3` | `native/hqkg/`, `core/programmability/`, `src-tauri/` and worker reports |
| B | `78a5105a9c1f1880d658f350e130b1e2632d3532` | #3 | `0d0d39e9debe4ebe3121f387334a47e0be42be90` | `apps/vivim-host/` and reports |
| C | `f950b91685927f37b48d020433aa8d2d3c271419` | #4 | `362b6e376a90999c35ba5a803971c83a241ed00e` | `studio/`, `harness/` and reports |
| D | `ad55fe28c2c331841675f308f9199ee4c2add86a` | #5 | `8fd4b1abb16f2d5ab418fdba42a03e2b918231c8` | `hq/` and reports |
| E | `1a5cf26ce14ebce4e31d9a4373060ace61852645` | #6 | `ad07a0dd937fe02235f0a27c5d40c4f0bb4caada` | `apps/mobile-relay/`, `packages/tierflow/` and reports |
| F | `be458f3c4cdb7eb38d47b1de2a00d441b7cdc704` | #7 | `0b3474a5cb2053d607453b18d319d225f1d3ca57` | `contracts/`, `tests/`, `capability-spaces/` and reports |

**Post-merge ancestry verification:** GitHub compare from each frozen A–F worker SHA to the integration branch returned `status=ahead` and `behind_by=0` for all six. This establishes worker commit ancestry, not standalone combined runtime correctness. No PG00–PG03, PGZ, Phase 2 or Labs research proof branch merged. Main not modified.

**Source donor inventories from frozen PREG reports (worker-reported, not newly recomputed):** A 99 donor blobs, B 135, C 501, D 315, E 270, F 7 selected Labs files; these totals cover different scopes and do not substitute for a fresh recursively enumerated full integration tree or secret scan. Reports and source provenance were retained. GitHub merge success is an API result, not a shell exit code.

## 2. Root workspace, source closure, and packaging truth

Created root `Cargo.toml` workspace members `native/hqkg`, `core/programmability`, `src-tauri` (default members limited to pure Rust crates), with `resolver=2` and low-disk dev profile; root `package.json` Bun workspaces for Host, mobile, Studio, harness, HQ and TierFlow; root `rust-toolchain.toml`; `integration/source-lock.json`; and `integration/release-manifest.json`.

**Not generated:** root `Cargo.lock`, root `bun.lock`. No dependency resolution occurred. The root manifests are **source configuration only**; reproducible compilation has NOT been established. Toolchain compatibility and full transitive source closure remain unverified. `src-tauri/Cargo.toml` still pins Tauri family `=2.0.0`; `src-tauri/tauri.conf.json` still requires `resources/vivim.apk`, runtime resources and `tools/packaging/prepare-host-runtime.mjs` prebuild hook. PWA FIRST: full APK is DEFERRED; this packaging configuration must be reconciled before a Windows release. Do not invent an APK, signed installer, generated lockfile, installed Node runtime or acceptance receipt.

Canonical runtime owners retained: B Host v6 catalog (19 IDs), approval, claim/receipt/uncertain-effect and protected personal World writer; A native pure Rust/HQKG and Tauri; C one signed-in Chrome sender; D development HQ/wiki/index; E one TierFlow renderer/PWA signer; F test-only contract mirrors; G root integration. No provider API, Playwright/Puppeteer or second Host effect writer authorized.

## 3. Command / evidence ledger

| Check | Actual invocation / evidence | Result | Exit |
|---|---|---|---|
| Six exact branch pins | GitHub fetch commit by branch name, SHA equality | **PASS** (6/6) | N/A API |
| Integrator branch initial presence | GitHub branch search + commit lookup | **ABSENT**, then created from pinned architecture baseline | N/A API |
| A → F serial integration | GitHub create PR / merge PR, #2 through #7; GitHub `merged=true` each | **PASS** six merges | N/A API |
| Ancestry completeness | GitHub compare `A..G` through `F..G` | **PASS** six `behind_by=0` | N/A API |
| Manifest read and write | GitHub fetch root/subtree manifests; GitHub create file | **PASS** source authoring | N/A API |
| `cargo metadata --locked --format-version 1` | NOT EXECUTED; root Cargo.lock absent | **UNRUN / BLOCKED** | N/A |
| `cargo test --workspace`, `cargo check -p vivim-host` | NOT EXECUTED | **UNRUN** | N/A |
| `cargo audit`, `cargo tree`, Tauri MSI | NOT EXECUTED | **UNRUN** | N/A |
| `bun install --frozen-lockfile`, `bun test`, `bun run typecheck` | NOT EXECUTED; root bun.lock absent | **UNRUN / BLOCKED** | N/A |
| Host/HQ/Studio/F Node tests; privilege negatives | NOT EXECUTED on combined checkout | **UNRUN** | N/A |
| Source-path lint / secret scan / no donor process build | NOT EXECUTED | **UNRUN** | N/A |
| Windows MSI / physical HTTPS Android PWA / signed-in browser | NOT EXECUTED | **UNRUN** | N/A |
| Owner-World backup/restore / no-resend uncertain / rollback drill | NOT EXECUTED | **UNRUN** | N/A |

**Execution environment:** connected GitHub API for source/commit/ref operations; no checked-out ARBOR local workspace available to execute Node/Bun/Cargo, no Windows desktop/Chrome sign-in or physical paired Android device. GitHub API `merged=true` is not an OS process exit 0. There is **no valid combined test PASS claim**.

## 4. Explicit blockers and exact owner assignments

**P0 S1 — Native launcher / A+B; F adversarial tests.** Fixed localhost `127.0.0.1:43187` readiness may be impersonated; native sends durable owner Bearer before authenticated child attestation. Implement source-verified OS child identity and nonce challenge / single-use Host UI code under the *existing* B authority, deny foreign listener, occupied-port race and replay. Remove wildcard remote Tauri IPC, update patched compatible Tauri v2 family; preserve native/Host actor origins and strict ACL. **NOT FIXED.**

**P0 S2 — PWA signer / B+E; F tests.** B static Host route omits E `browser-device-key.mjs` and active grant/session bearer lacks request-bound proof/replay defense. Implement strict route allowlist, nonextractable P-256 signer, fresh method/path/body/nonce proof on privileged operations and fail-closed revoke/offline behavior. Real HTTPS Chrome pairing/IME/reconnect acceptance required. **NOT FIXED.**

**P0 Packaging and source closure — G+A+B+E+F.** Generate correct `Cargo.lock`, `bun.lock` with real toolchains; resolve Windows linker/build.rs and staged Host Node >=24 runtime, source closure, absent `tools/packaging/prepare-host-runtime.mjs` if verified, and remove mandatory APK packaging for PWA-first. Prove no donor sibling checkout/process/database needed. **NOT VERIFIED.**

**P0 HQ source authorization — B+D+F.** Bind D development index/source reads to Host-approved actor/session/source revisions, handle allowedRoots, revoke, symlink/junction and World namespace separation. **NOT ACCEPTED.**

**P0 Signed-in provider and single effects — B+C+F.** Real sender thread identity, uncertain send/no auto retry, Host journal and R2 approvals must be exercised with owner-authorized signed-in Chrome. **UNRUN.**

**P1 Rock Core / D+B+F.** Protected originals, disk reserves, WAL/-shm and ENOSPC safety require actual byte measurements and crash/recovery acceptance. PGZ research is not production proof. **UNRUN.**

## 5. PGZ classification

PGZ `f0b0ce2522eef0929af498a6d0cc31c3294f61b3` and PG00–PG03 remain **PREPARED_NOT_PROMOTED**. Their independent findings inform the blockers above, but **none of their synthetic tests, fixtures or proof branches were integrated or elevated to production acceptance**. No new semantic compiler, native executor, graph, scheduler, browser sender, UI publisher or APK introduced.

## 6. Capability continuity and gate decision

| Surface | Source state | Actual combined acceptance |
|---|---|---|
| Windows Tauri + HQKG | SOURCE_MERGED | BUILD_UNRUN / SECURITY_BLOCKED |
| Host World/approval/journal | SOURCE_MERGED | EFFECT_RECOVERY_UNRUN |
| Studio / signed-in Chrome | SOURCE_MERGED | PROVIDER_E2E_UNRUN |
| Internal HQ wiki/index/context | SOURCE_MERGED | NODE24 / AUTH_UNRUN |
| Android browser PWA | SOURCE_MERGED | SECURITY_BLOCKED / DEVICE_UNRUN |
| TierFlow visual UI | SOURCE_MERGED | BUN_BUILD / UI_UNRUN |
| F contracts + selected Labs tests | SOURCE_MERGED | COMBINED_TEST_UNRUN |
| Full Android APK | DEFERRED | NOT REQUIRED FOR PWA-FIRST |

**Decision: NO_GO / REPAIR_REQUIRED.** Source-only serial integration is committed; the acceptance condition of *minimal reproducible Cargo/Bun workspace and no-donor source closure* is not met. **W2 must not be labeled GO** or launched as if G were approved. Original G source merges are preserved; W2 owner repair is conditional on gate approval. There is no verified beta.

**Rollback:** frozen six worker refs remain unchanged; GitHub merge history #2–#7 is traceable. Revert gate configuration commits and six merges in reverse order through separate reviewed commits if necessary, never force-push main or overwrite frozen refs. Actual rollback drill UNRUN. No owner data or live Host effects migrated. **Next-wave accepted base SHA: NONE** until reproducible build/source-closure proof and gate re-evaluation. Use current integration branch HEAD for *repair work only*, not as a GO baseline.


---

## W01-G R001 focused continuation (2026-11-12)

**Starting G HEAD verified:** 6a9debe6cb7b43b2c77a18b6fd9c0f0125911c0a. The original A–F serial merges and source ownership remain intact. G created a complete-local-checkout source-closure scanner, fail-closed Windows Node24 staging helper and evidence-capturing PowerShell test runner. **All combined Cargo/Bun/Node24/Windows/no-donor commands remain UNRUN**; actual root Cargo.lock and bun.lock do not exist, and no reproducible-build PASS is claimed. Draft **A-owned** [PR #8](https://github.com/owenservera/ARBOR/pull/8) removes the mandatory APK packaging resources, but is unmerged pending A review. **PWA FIRST; APK DEFERRED**. Security P0 native impersonation/IPC and PWA signer/replay, HQ authorization, browser/effect recovery remain unchanged.

Canonical scoped evidence and exact command/exit handoff: [W01-G-R001-SOURCE-CLOSURE.md](W01-G-R001-SOURCE-CLOSURE.md). This amendment **does not override original NO_GO**. Decision remains **REPAIR_REQUIRED**; W2 accepted base **NONE**, beta **BLOCKED**, rollback UNRUN. The G R001 authoring commits and A draft PR are listed in the linked report.

## Owner-directed one-main source consolidation (2026-11-12)

The owner explicitly requested a single ARBOR main branch, superseding the earlier `no main merge` staging constraint for **source consolidation only**, not beta acceptance, publication, user-data cutover or effect execution. GitHub compare reports ARBOR `main` is a direct ancestor of `integrate/proposal5-beta-r001` (`behind_by=0` from main). The original six frozen A–F donor branches were already serially merged into integration. Reviewed isolated A packaging PR #8 merged at `23e17145201828bfde2508ae09fbdc0fc1c70f1b`; B PWA browser-device static route PR #9 merged at `bed77e7e8141f71412067d20d08477600d8ea3f0`. These are narrow source fixes, not security or device acceptance. The source-lock keeps original worker SHAs.

Root `.github/workflows/w01g-reproducibility.yml` was committed as an attempted real Cargo/Bun/Windows runner. **Observed GitHub Actions run 43233248631** reported `status=completed, conclusion=startup_failure`, and its jobs list was empty. No real Cargo/Bun/Node/Windows command, version or exit receipt was produced; cause of startup failure is **not diagnosed** by an empty jobs list. Both root Cargo.lock and bun.lock remain **absent**, confirmed by GitHub file lookups. CI, no-donor source closure, MSI, security, phone, browser, rollback and World isolation remain UNRUN/BLOCKED.

Main consolidation PR #10 is source merge only; it must not be described as GO. Status remains **REPAIR_REQUIRED**, `betaAuthorized=false`, `nextWaveAcceptedSha=null`. Deployment/cutover requires a separate real verified decision. Main may contain unverified implementation but must not release an installer or activate privileged effects. Further repairs should branch from and merge into main, preserving one canonical development tip. Do not merge PGZ/PG/Phase2 preparation branches as production code.
