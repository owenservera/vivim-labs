# ARBOR Proposal 5 — PRE-G Lane A Final Handoff

**Status: FREEZE_READY — SOURCE-ONLY. Native security remains BLOCKED / UNRUN.**

**Canonical frozen branch:** `work/p5-w01-a-native`  
**Final freeze SHA:** the Git commit containing this `PREG-A.md` report (the branch tip at final readback, reported to G as an exact SHA; the report necessarily cannot self-embed its own commit SHA).  
**Last source/preparation HEAD before final report:** `1acce99e75d300208715f68b6db7ab11cc0cd93c`  
**Common worker ancestor:** `dd245d78ff3c110ba8630cad557ea0479c51a635`  
**Latest observed proposal architecture head:** `6ab7e6976c02cac2ee71f030deda3f6541f8102f`.  
**G integration branch:** `integrate/proposal5-beta-r001` **ABSENT** at pre-G inspection. Do not treat W0-G or W1-G as accepted.  
**Final action:** commit this report; stop all further Lane A writes. G should review immutable source and integrate serially. This is a worker freeze, NOT beta/security acceptance.

## 1. Source pinning and physically imported inventory

| Imported component | Donor path, ref, commit | Original Git tree | Files | Status |
|---|---|---|---:|---|
| HQKG engine and tests | `owenservera/repos-hq`, `research/rust-knowledge-graph/`, `main@9b922ea049dbadc94e504b5b9432743995c020ca` | `81b797948692b8a1e34ff13450be8779711f0e4d` | 79 | Source present and hash-verified |
| Windows Tauri shell | `owenservera/vivim-web-harness`, `src-tauri/`, `main@cae97afe848d79957c9d6c698a90996a758bfd56` | `a50ad8e0bca8ee8986be17e88aac2e1998e31105` | 13 | Source present and hash-verified |
| Pure Rust programmability | Same Harness SHA, `core/programmability/` | `3ce6d17623bc04593ae5e45125daaa8318e468e1` | 7 | Source present and hash-verified |
| **Total** | | | **99** | **99/99 unchanged donor Git blobs** |

**Integrity recheck during PRE-G:** fetched source and ARBOR import Git trees recursively; all **99 expected blob SHA-1s matched**, **0 mismatches** and **all four trees reported `truncated=false`**. Exact imported file list and individual hash provenance are preserved in `integration/worker-reports/W01-A.md`; import tree `09759f65fbbe65a6a7dd632c136d86ba45c3e5f1`. This proves source-byte equivalence, not build correctness.

Original user wording `repos-hq — research/rust-knowledge-graph` is a **source directory**, not a branch. Actual pinned donor is repos-hq main at the above SHA. R001 native I/O research is intentionally **not** production-imported; observed Harness research branch `work/native-user-io-r001-20261112` at `fa6dea9576c83496d95562b98b7a815fda4aec88`.

**Excluded:** generated `src-tauri/resources/vivim.apk` binary, HQKG research-only integrations/large evidence artifacts, target caches, private DB/profile/owner secrets, and donor runtime state. Existing Tauri config still refers to missing APK; this remains an unresolved packaging blocker.

## 2. Lane A commit and follow-up ledger

| Commit | Role | Files and truth |
|---|---|---|
| `718fd5b62964733c258675089c495ee2c4687ff6` | Physical import | 99 source blobs under `native/hqkg/`, `src-tauri/`, `core/programmability/`; no Rust rewrite |
| `af19b2eee8099d86488f54c4aa1a9135a719a46b` | Import report | `integration/worker-reports/W01-A.md` |
| `a48d7eee05498d280b57c5fbeb2795049f001bd9` | Baseline-drift report | Same W01-A report corrected to reflect concurrent architecture movement |
| `1acce99e75d300208715f68b6db7ab11cc0cd93c` | **Second focused follow-up completed** | Added `src-tauri/NATIVE-SECURITY-W02-PREP.md` (precise patch/security plan, **not** an implemented security fix) |
| **this report's commit** | PRE-G final freeze | **Only** `integration/worker-reports/PREG-A.md`; immutable frozen HEAD must be captured from Git after commit |

No executable source changes after the original 99-file import. No G root workspace file, shared lockfile, other lane source file, main, or integration branch modified. There are no unfinished Lane A edits after this report.

## 3. A ⇄ B native launcher contract reconciliation (P0 S1)

B's own `integration/worker-reports/W01-B-FOLLOWUP.md` on `work/p5-w01-b-host` at observed HEAD `c12f4a27a8c700c9df4cba17a9ff96efd5897748` proposes **`arbor.host-launch/1`**. A's second follow-up packet `src-tauri/NATIVE-SECURITY-W02-PREP.md` is compatible **at the contract level**. A and B must implement in W2 **only after G integrates**, with a single agreed authenticated launch protocol, never two session authorities:

- **A native owner:** generate high-entropy per-spawn secret and launch identifier; start bundled Host process through protected Windows inherited pipe/handle (or demonstrably equivalent ACL-constrained authenticated OS channel); retain/process-check spawned child handle and listener identity; generate fresh attestation challenge; validate purpose-separated HMAC-SHA256 in constant time with exact protocol version, launchId, nonce, child PID and endpoint binding; fail closed on occupied :43187, child crash, forged health, stale/replayed attestation and launch-code request. Keep secret off CLI args, HTTP headers/URLs, browser renderer and logs. An environment variable containing a secret alone is not adequate against same-user local inspection.
- **B Host owner:** listen only after authenticated native launch mode initialization; implement strict `/api/host/native/attest` and single-use `/api/host/native/ui-launch` against verified per-spawn secret and consumed state. Reuse existing `uiSessions.issue()` (short-lived single-use code and HttpOnly SameSite cookie exchange), keep persistent `bootstrap/owner-token` in protected Host store, preserve existing legacy authenticated CLI pathway as separately authorized; do not mint alternate World grants or effect tokens.
- **Joint A/B invariant:** no persistent owner capability transmitted over unverified loopback. No fallback to `GET /api/host/status` as authenticity or native Bearer POST to `/api/host/control/ui-session`. Require nonce/domain separation and strict origin/session/actor checks. Port collision must fail closed rather than attach blindly to any listener. Resolve race between attestation and UI code issuance on same authenticated child/connection; do not mistake a claimed PID or unauthenticated health JSON for child identity.
- **G:** reconcile A and B implementations only after both are committed on W2 branches, preserving B's sole Host claim/receipt/uncertain-send authority.

**Negative security acceptance tests, all UNRUN:** foreign port-43187 listener responding HTTP 200; fake health JSON; forged HMAC/challenge/PID; nonce replay and expired challenge; wrong protocol version; child crash/restart/race; UI code redemption twice; native session after child death; multiple launches causing duplicate Host writer; foreign origin invoking native IPC; legacy CLI owner-token behavior regression. No source-only inspection satisfies P0 S1.

## 4. Tauri/native security and build blockers, with exact owners

| ID | Status | Required owner/action |
|---|---|---|
| A-S1a Tauri pinned 2.0.0 family | **UNFIXED** | A W2 proposes reviewed patched mutually compatible Tauri 2.x `Cargo.toml` family; G resolves and commits root `Cargo.lock`, Cargo audit/Windows builds; do not claim current 2.0.0 safe |
| A-S1b remote IPC wildcard | **UNFIXED** | A W2 remove `remote.urls = ["http://127.0.0.1:*"]` capability grant; broker trusted local-only IPC where essential; test foreign loopback webview denial; assess `withGlobalTauri=true` before relaxing |
| A-S1c durable owner Bearer leak | **UNFIXED** | A/B W2 implement `arbor.host-launch/1` jointly; no native Owner Bearer through forged localhost |
| A-BUILD1 Cargo workspace | **G-OWNED, UNRUN** | G root `Cargo.toml`, `Cargo.lock` and `rust-toolchain.toml`; members `native/hqkg`, `core/programmability`, `src-tauri`; `resolver="2"`; preserve `[profile.dev] debug=0; incremental=false` at workspace root |
| A-BUILD2 dependencies | **STATIC ONLY** | Original Tauri local `../core/programmability` resolves inside ARBOR. HQKG Rust/typed DAG/engine/API/importer preserved. Resolve `cargo metadata --locked`, `cargo tree`, `cargo audit`, edition/features and optional SQLite/Postgres on one Windows MSVC build |
| A-BUILD3 paths | **UNVERIFIED** | G/A confirm root Cargo target modifies `repo_root()` executable-relative ascent; staged Node runtime, `../apps/vivim-host/public`, `beforeDevCommand` and `beforeBuildCommand` working directory; B owns Host process service |
| A-BUILD4 APK | **UNRESOLVED** | G/E/B/A remove mandatory `resources/vivim.apk` from PWA-first release package if not shipping signed APK; no fabricated release APK/manifest, no new Android app |
| A-NODE0 | **PREP ONLY** | In W2 secure, source-scoped inert event/projection adapter, built atop existing HQKG if proven; no per-keystroke thread spawn / whole-session mutex from R001, no new engine/World writer/Host command catalog |
| A-PRODUCT | **UNRUN** | Windows MSI, browser/PWA handshake, device, restart/crash, donor-removal and Host one-writer checks require combined product and real runtimes |

No dependency on external donor repositories was introduced in the imported Rust Cargo manifests. This is manifest-level inspection only; runtime/browser/packaging independence is not yet proven.

## 5. Actual verification vs planned tests

| Verification | Result / exit code |
|---|---|
| GitHub source tree and blob comparison (rechecked PRE-G) | **PASS, 99/99 matching**; static Git object equality; no process exit code |
| GitHub original import branch and report/head readback | **PASS**, source files and W01-A/prep reports present |
| Cargo build/check/test — HQKG default and `--features importer` | **UNRUN; exit code N/A** |
| Cargo test — programmability core | **UNRUN; exit code N/A** |
| Cargo check — Tauri Windows target | **UNRUN; exit code N/A** |
| Cargo metadata/tree/audit with G root lock | **UNRUN; exit code N/A** |
| Adversarial native launch/attestation security tests | **UNRUN; exit code N/A** |
| Windows MSI packaging, clean install and real WebView2 | **UNRUN; exit code N/A** |
| Physical phone browser PWA, provider and reboot/resume | **UNRUN; exit code N/A** |
| No-donor removal, private World/effects recovery | **UNRUN; exit code N/A** |

Suggested **post-integration Windows/MSVC commands** for G and A W2 (not execution results):

```powershell
cargo metadata --locked --format-version 1
cargo tree -p vivim-host -d
cargo test -p hq-knowledge-graph
cargo test -p hq-knowledge-graph --features importer
cargo test -p vivim-programmability-core
cargo test --workspace
cargo check -p vivim-host --target x86_64-pc-windows-msvc
cargo audit
bunx tauri build --bundles msi
```

## 6. Cross-owner requirements and freeze handoff

- **B:** implement `arbor.host-launch/1` Host verifier and one-time UI code bound to actual native child; preserve protected owner-auth/CLI and sole Host effect journal.
- **C:** Studio in-repo Chrome/CDP and signer remains one send-owner; preserve exact thread selection/uncertain send; no new provider API.
- **D:** internal HQ dev-source read must be Host-approved and namespace isolated from personal World. No external HQ process.
- **E:** ordinary PWA browser keys and pairing/renewal need B/E/F Host verifier agreement and physical tests; no effect authority in PWA.
- **F:** versioned Rust/TS identity/status contracts and adversarial security/fault-injection tests; no second action catalog.
- **G:** read this report and `W01-A.md`; honor source pin/worker root, reconcile worker-base drift, integrate six frozen branches serially, make root Cargo/Bun and packaging edits, run combined tests, publish exact W1-GATE and W2 baseline.

**Freeze statement:** the second follow-up is complete; the source import is unchanged; this report is the last planned worker write. **FREEZE_READY** means branch content is committed and immutable for G's review, NOT security-patched, compiled, runtime-verified, or accepted as a beta. After this commit, no more work on `work/p5-w01-a-native` without G reopening the lane.
