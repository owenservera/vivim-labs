# ARBOR W01-G R001 — Source Closure, Packaging and Reproducibility

Date: 2026-11-12
Decision: REPAIR_REQUIRED — BUILD_UNRUN / LOCKS_UNRUN / PWA_PACKAGING_PENDING_OWNER_REVIEW
Starting integration HEAD verified: 6a9debe6cb7b43b2c77a18b6fd9c0f0125911c0a.
This is a focused original G continuation, not W2. No main, frozen worker or PG changes. No approved next-wave baseline.

## G-owned committed repair ledger

| Commit | Path | Result |
|---|---|---|
| 68a27342019161f7c961a215068c50100d996856 | integration/scripts/w01g-r001-source-closure.mjs | Authored full local tree scanner; execution UNRUN |
| 0d17aadec96ba09c0008bf308b3c59757c05fa2c | tools/packaging/prepare-host-runtime.mjs | Authored fail-closed Node24 Windows staging; execution UNRUN |
| 6fdbe89f6592b7f9e128b3c87f019ed73af658ac | integration/scripts/w01g-r001-windows.ps1 | Authored actual lock/test/no-donor runner; execution UNRUN |

**A-owned isolated repair:** branch work/p5-w01g-r001-a-pwa-packaging-review at fb5591c4f19544c6afd7dd3a2d899062f6e8bd5b, draft PR #8 https://github.com/owenservera/ARBOR/pull/8. Exactly one changed A-owned path: src-tauri/tauri.conf.json. Removes mandatory native APK and placeholder APK release manifest resources, keeps MSI and runtime directory, removes implicit packaging hook (G explicit root staging instead). **UNMERGED and requires A owner review.** Does not alter native launch security or Host PWA grants.

## Actual source findings

Root Cargo workspace imports native/hqkg, core/programmability and src-tauri. Root Bun workspace imports Host, mobile relay, Studio, harness, HQ and TierFlow. HQ requires Node >=24. Existing root manifest declares bun@1.4.4 and rust-toolchain 1.99.0, but this does **not** prove compatible Tauri resolution. src-tauri/Cargo.toml still pins exact 2.0.0 Tauri runtime/build family. No reviewed patched compatible family selected; Cargo must resolve real graph after A review.

Packaging hook tools/packaging/prepare-host-runtime.mjs was missing (GitHub fetch 404) at starting HEAD, now source-added. Tauri config at starting HEAD still requires resources/vivim.apk, resources/vivim-release.manifest.json and resources/runtime/. A branch removes first two but is not merged. Existing src-tauri/src/main.rs packaged Host mode expects runtime/node.exe and runtime/apps/vivim-host/server.mjs; development repo_root ascends three directories from executable directory, incorrect for the root-workspace target/debug placement (A owner repair). B Host direct-phone static allowlist omits E mobile/browser-device-key.mjs, a separate P0 Host/PWA security blocker.

The G packaging helper --check refuses required missing lock inputs and remaining APK config. The --stage path requires **actual Windows x64**, absolute Windows Node>=24 executable, independently supplied SHA256, valid Authenticode signature, frozen Bun install with ignore-scripts, real Host entry syntax check and atomic stage/restore. It refuses symlinks and secret-shaped inputs. It neither fabricates a binary nor grants effects. This source is not Windows-verified.

The G closure scanner recursively traverses all shipped product source directories in a **real checkout**, not a 300-file GitHub compare. It checks workspace manifests, literal JS imports, Rust includes and Cargo paths, symlinks, outside donor references and mandatory Tauri resources. Computed imports, third-party package resolution, browser assets, MSI and private data boundaries require additional execution/review. **Full recursive integrated checkout scanner UNRUN**; do not claim complete closure. Static V8 parse of both committed JS files after ESM import stripping passed; not Node runtime/test proof.

## Toolchain / command receipts — actual vs unavailable

**Only shell environment available:** an ephemeral Linux container *without the ARBOR repository checkout*.

| Actual command | Exit | Observed |
|---|---:|---|
| git --version | 0 | git version 2.47.3 |
| node --version | 0 | v22.16.0, below project Node24 requirement |
| command -v cargo | 1 | not installed |
| cargo --version | 127 | command not found |
| command -v bun | 1 | not installed |
| bun --version | 127 | command not found |
| command -v pwsh | 1 | not installed |
| git ls-remote https://github.com/owenservera/ARBOR.git refs/heads/integrate/proposal5-beta-r001 | 128 | fatal: Could not resolve host: github.com |

GitHub connector source inspection and writes are API operations, not shell exits. GitHub refs are available through the connector, but the runner environment cannot materialize a network checkout. No actual root Cargo.lock or bun.lock generated. **UNRUN**: cargo generate-lockfile, cargo metadata --locked/tree/test/check/audit, bun install --frozen-lockfile, Node>=24 test suites, full source closure, staged Windows runtime, Windows MSI, physical PWA, browser provider, no-donor isolated checkout and rollback. No exit code is invented for those.

## Exact executable Windows/CI handoff

In a disposable **Windows x64 ARBOR checkout**, with Node>=24, Bun, Cargo/Rust MSVC, PowerShell>=7.2 and no active owner data, use the committed runner. It stores real command args, versions, process exits and captured output in a temporary evidence JSON, without committing or publishing:

1. Run: pwsh -File integration/scripts/w01g-r001-windows.ps1 -Mode GenerateLocks
   - Actual cargo generate-lockfile; bun install --ignore-scripts; cargo metadata --locked and bun install --frozen-lockfile. Inspect generated Cargo.lock and bun.lock with exact dependency graph and security/license review; commit *only real outputs* through G.
2. After A review of PR #8 and G merge, run: pwsh -File integration/scripts/w01g-r001-windows.ps1 -Mode Verify
   - Actual recursive closure, packaging preflight, Cargo metadata/tree/tests/MSVC check, Bun frozen and Host/HQ/F/mobile/TierFlow tests. Scrub local evidence before committing; failure is blocker.
3. Once both locks are committed and tree is clean, run: pwsh -File integration/scripts/w01g-r001-windows.ps1 -Mode NoDonor
   - Clone *only ARBOR* into a temporary detached checkout at exact HEAD and run the closure and combined tests there. Independently verify no donor processes or databases at OS level.
4. Supply independently verified ARBOR_NODE24_EXE and ARBOR_NODE24_SHA256, and on actual Windows invoke: node tools/packaging/prepare-host-runtime.mjs --stage
   - Verify staged source/dependency closure and MSI packaging explicitly. No APK. No installer publication.

No Windows tests may be relabeled PASS without their actual outputs and exits. G must reconcile any discovered package versions or packaging failures with the original A/B/C/D/E owner, not modify product files directly.

## Remaining blockers and handoffs

**P0 A+B+F native security:** fixed loopback listener impersonation, durable owner bearer transfer, wildcard native IPC origin, patched compatible Tauri, child identity and A dev root path. All UNFIXED.

**P0 B+E+F PWA:** missing B static browser-device-key route, active bearer replay/per-privileged-request proof; real HTTPS phone tests. UNFIXED.

**P0 G+A+F packaging:** root Cargo.lock/bun.lock absent; actual graph/Node24/Windows MSI/no-donor build UNRUN; A draft packaging PR #8 unmerged.

**P0 B+D+F:** HQ source actor/permission/allowed roots/revocation and private World isolation UNRUN.

**P0 B+C+F:** source-grounded signed-in Chrome sender and Host claim/receipt/uncertain-send recovery UNRUN.

**P1 D+B+F storage:** Rock protected originals, WAL/ENOSPC and rollback UNRUN.

PG00–PG03/PGZ remain PREPARED_NOT_PROMOTED and unmerged. The single Host effect authority, A Rust graph, C browser sender, D HQ and E TierFlow publisher are unchanged. APK deferred. No user-data ingestion/cutover or release.

## Disposition and rollback

**REPAIR_REQUIRED; W2 GO=false, beta GO=false, next accepted W2 base=null.** No complete reproducible workspace, no build PASS, no donor independence or device/security acceptance. G script additions can be reverted as individual commits after verifying current branch lease; no force push. A draft PR #8 can remain unmerged or closed after review. No runtime/owner data rollback was executed. **STOP until real Windows/CI receipts and A review.**
