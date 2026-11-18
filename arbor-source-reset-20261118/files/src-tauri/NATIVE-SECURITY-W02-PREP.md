# Proposal 5 — Lane A W02 Security/Build Preparation (G integration gate BLOCKED)

**Status:** READ-ONLY DESIGN / EXACT PATCH INSTRUCTIONS ONLY. **No executable Rust/Tauri source modified.**
**Prepared from:** `work/p5-w01-a-native` at `a48d7eee05498d280b57c5fbeb2795049f001bd9`.
**Integration check:** `integrate/proposal5-beta-r001` not present (GitHub branch search, commits and compare all failed/not found). Therefore **do not create W02 branch** or modify source, root manifests/locks, or merge concurrent work.
**Cross-owner donor:** Lane B `work/p5-w01-b-host` at `9c52c35d473e15c86939c786f0d5e3819da5c302`. This branch is NOT merged into this Lane A branch; inspect only.
**Ownership:** A writes `src-tauri/`, `core/programmability/`, `native/`; G owns root `Cargo.toml`, root `Cargo.lock`, release manifest and integration branch; B owns Host `apps/vivim-host/`.

## A. Exact native source findings

- `src-tauri/Cargo.toml` pins `tauri`, `tauri-runtime`, `tauri-runtime-wry`, `tauri-utils`, `tauri-build` to `=2.0.0`. Present exact versions are vulnerable/outdated.
- As of 2026-11-12 the upstream Tauri release overview reports `tauri 2.12.3` (released 2026-10-10), `tauri-cli 2.12.2`, `@tauri-apps/api 2.12.3`, and `tauri-build 2.7.2`. Source: https://tauri.app/release/ . **Candidate**, not validated here for resolved dependency compatibility on Windows.
- GHSA-w28w-mhc8-qvjv affects `tauri >=2.0.0, <=2.11.5`; patched at `>=2.11.6`: https://github.com/tauri-apps/tauri/security/advisories/GHSA-w28w-mhc8-qvjv . This is a minimum security floor, NOT a claim that newer versions have passed this product's build.
- `src-tauri/capabilities/default.json`: `windows=["main"]`, `remote.urls=["http://127.0.0.1:*"]`; `core:default` and window permissions. This exposes a broad origin pattern. The Tauri capabilities reference confirms `remote` allows remote pages to access permitted APIs; omission defaults to bundled local app resources. https://tauri.app/security/capabilities/ ; https://v2.tauri.app/reference/config/ .
- `src-tauri/src/main.rs`: spawned Node child is recorded; `wait_healthy` accepts any successful response from fixed `http://127.0.0.1:43187/api/host/status`, without connection to child's identity. `desktop_launch_url` subsequently reads long-lived owner capability and POSTs it as bearer to the fixed port. **A preoccupied/forged listener can receive the owner token.**
- `src-tauri/tauri.conf.json` uses external-dev Host URL `http://127.0.0.1:43187`, `withGlobalTauri=true`, bundles `resources/vivim.apk` (not imported/generated) and a placeholder release manifest; MSI cannot be asserted buildable without G/B/E packaging.
- Original `src-tauri/src/main.rs` `repo_root()` uses executable-relative ascent calibrated to standalone `src-tauri/target/debug/`; the G-root Cargo workspace may move target output.
- `native/hqkg/Cargo.toml` has no external repository `path`/Git dependencies, and `core/programmability` local Tauri dependency resolves inside the future ARBOR workspace. HQKG includes tests/importer/typed DAG/engine registry already—**do not duplicate**.

## B. Exact A-owned W02 changes to implement ONLY after G gate

### B1. Tauri dependency patch: `src-tauri/Cargo.toml`

1. Replace `tauri = { version = "=2.0.0", ... }` with reviewed pinned candidate `=2.12.3` **after resolving/advisory check on Windows**; never use <2.11.6.
2. Replace directly pinned `tauri-runtime`, `tauri-runtime-wry`, and `tauri-utils` version 2.0.0 requirements with **mutually compatible versions reported by Cargo's resolution for the selected `tauri` release**; delete *unneeded direct dependencies only after checking imported source uses*. Never assume all Tauri ecosystem crates have identical version numbers.
3. Pin `tauri-build` to a confirmed compatible build-time release (upstream listed `2.7.2` at investigation time; verify in Cargo resolution). Preserve existing Windows WebView2, Tokio, Reqwest functionality.
4. Ask G to regenerate the **single root `Cargo.lock`** with `cargo update` / `cargo generate-lockfile` and inspect `cargo tree -p vivim-host -d`; run `cargo audit` against resolved lock. Do not edit root manifests or locks in A.
5. Fail W02 security if `cargo tree` resolves `tauri <2.11.6`, duplicates unreviewed Tauri majors, or a security advisory remains unmitigated. Record full resolved versions (not a guessed version matrix).

### B2. Native IPC/capability: `src-tauri/capabilities/default.json` + `src-tauri/src/main.rs`

Do NOT simply change wildcard `127.0.0.1:*` to fixed port and call the issue closed. Remote Host pages are still lower-trust than bundled static Tauri app resources, and port fixation is not identity proof.

- Preferred: a small **bundled local-only launcher webview** with a minimal capability limited to `main`; remove the `remote` stanza from that capability. Keep Host web UI at localhost **without native IPC** (remote commands denied). If an existing Host webview requires one pure validation command, expose it only via an explicitly designed broker/controlled local shell after a trusted session, not via generic `core:default` on remote origin.
- Set `withGlobalTauri=false` if validated against UI behavior; do not assume the change preserves Host without testing. Keep existing local keyboard, navigational and window behavior. Use named `webviews` over broad `windows` if multiwebview is introduced.
- Keep `validate_native_program` pure, size-/schema-limited; no owner token or data access in an unauthenticated invoke. `host_is_running` should test `try_wait` rather than just `Some(child)` if exposed to trusted internal window.
- Add static test that parses capabilities and rejects `remote.urls` wildcard/local origin grants, unexpected privileged permissions and `withGlobalTauri=true` on an untrusted remote origin. Add runtime adversarial test: a foreign localhost window cannot call any privileged native command; an authorized bundled local view can use only minimal pure commands.

### B3. Secure bootstrap A↔B contract: `src-tauri/src/main.rs` AND B-owned Host

**Do not transfer the persistent owner capability over HTTP.** A may not unilaterally change Host APIs.

Require B to implement a **private process-bound boot channel**:
1. A starts the bundled Node executable from a verified in-ARBOR resource path and creates a fresh per-launch 256-bit random handshake secret using OS CSPRNG; transport it via a protected inherited pipe/handle rather than command-line argument or HTTP request. Separate child control channel from stdout log. Child must prove possession on a challenge bound to launch ID, OS child PID, listener endpoint, expiry and protocol version. PID-only and ordinary HTTP health are insufficient; check `child.try_wait` throughout.
2. B accepts the secret only from its spawn-time inherited channel, binds the expected Host listener, proves child identity over the authenticated challenge response, and mints a **short-lived single-use desktop session** directly over that private channel. In particular, the long-lived owner token stays in the Host's protected local store and never traverses unverified localhost. Do not put handshake secret/session in logs, URL query or environment dumps.
3. A navigates the desktop webview only to the verified child listener and a short-lived one-time browser redemption code (existing `#control-session=...` shape may be retained after B asserts ownership). On handshake failure, port collision, child exit, timeout, malformed reply or unexpected restart, A fails closed: **no token**, no owner session, no navigation to untrusted listener.
4. Restart means **new child, new launch ID, new secret, new proof, new session**, never reusing stale capability; shutdown revokes pending grants. Socket/pipe security descriptor should limit same-user unintended access on Windows. The Host remains sole World/effect authority.
5. If B cannot implement inherited pipe/Windows handle safely for Node packaged mode, design a protected named pipe with explicit user/process ACL and peer proof; **no fallback to public HTTP bearer exchange**. Dynamic port reduces accidental collision but never substitutes for authentication.

**Required adversarial synthetic tests (A + B cooperative, before GO):**
- Fake process occupies :43187, returns HTTP 200 for `/api/host/status`, logs request bodies: A must never deliver bearer/capability or navigate to it.
- Child crashes just after spawn; attacker later takes port: no session issuance.
- Forged challenge, wrong PID/launch ID, reused token, stale expiry, cross-session message: all rejected with no owner-token disclosure.
- Real child warm start, clean stop/restart and re-pair: each new boot proves identity, and existing Host one-writer semantics remain.
- Simultaneous two desktop launches cannot both become authoritative effect writers; B owns lock/host-single-instance admission.

### B4. Workspace/build closure (A checks, G owns shared writes)

- G root workspace `members = ["native/hqkg", "core/programmability", "src-tauri"]` plus future `native/protocol`, `native/node0` **only once source really exists**; `resolver = "2"`. Move HQKG `[profile.dev] debug = 0, incremental = false` to root (member profile ignored).
- Verify `cargo metadata --locked --format-version 1` has no `source = git+...` internal-repo dependency, no donor-specific `path` outside ARBOR, and all workspace members resolve within ARBOR.
- Inspect `tauri.conf.json` execution working directory: `beforeDevCommand` and `beforeBuildCommand` currently reference root-relative scripts; ensure G product build runs them from root or configure command paths accordingly. Test spawned Host resource layout and `repo_root` under root Cargo target.
- Replace required APK bundle entry only with approved **PWA-first** package contract (no new APK/app). G/E/B decide whether optional APK support stays off until actual signed artifact exists; remove missing mandatory resource path only after this decision. Do not ship placeholder `vivim-release.manifest.json` as proof of signed release.
- Preserve HQKG crate API and regression corpus. No new scheduler/graph/datastore. Minimal Node0 future seam: trusted actor-bound `InputSnapshot { schema_version, session_ref, editor_revision, input_kind, bounded_text_ref, source_scope, expected_revision }` -> inert `DerivedTrace { revision, evidence_refs, availability, status }`; no effects or broad source reads.
- R001 source on Harness `work/native-user-io-r001-20261112` is research-only and uncompiled according to its README; do not copy `thread::scope` per-keystroke over whole-session mutex to production.

## C. Exact unrun verification commands (execution environment needed)

```powershell
# G integration root, after lockfile and source merge:
cargo metadata --locked --format-version 1
cargo tree -p vivim-host -d
cargo test -p hq-knowledge-graph
cargo test -p hq-knowledge-graph --features importer
cargo test -p vivim-programmability-core
cargo test --workspace
cargo check -p vivim-host --target x86_64-pc-windows-msvc
cargo audit
# Windows production path:
bunx tauri build --bundles msi
```

Run native origin rejection, fake-port, forged-health, wrong-child, restart/replay and Host effect-journal uncertainty security tests. Validate MSI launch from a fresh user profile, signed-in browser bridge, Windows 11 WebView2, limited disk, offline source load and paired PWA. Record command, OS, exit code, log/evidence path. **Not executed in current GitHub source-only environment.**

## D. Fixed vs remaining blockers and G/B handoff

**Fixed by this work:** no application security defects; this preparation is intentionally non-executable because G has not incorporated Wave 1A. Verified exact source facts and a candidate upstream release/advisory floor; documented safe implementation contract.

**Blocking G prerequisites:** (1) integrate Wave 1A and other worker branches under serial G gate, (2) publish exact `integrate/proposal5-beta-r001` HEAD and workspace/lock ownership, (3) schedule B authenticated Host handshake implementation, (4) select PWA-first package/optional APK policy, (5) provide runnable Windows MSVC/Cargo environment and security test harness.

**Execution trigger:** only after G incorporation, create `work/p5-w02-a-native` from G exact HEAD. Implement B1–B4 in native-owned files, coordinate B's Host implementation, and report actual tests. Never merge or rebase onto concurrent work from this preparatory branch.

## Build resource decision

**Date:** 2026-11-13

**Decision:** Removed missing `bundle.resources` mapping `"resources/runtime/": "runtime/"` from `src-tauri/tauri.conf.json` (no real runtime artifact exists under `src-tauri/resources/runtime/`). Bundled only real existing resource: `"resources/vivim-release.manifest.json": "vivim-release.manifest.json"`. Honest: runtime is **not** bundled; `src-tauri/src/main.rs` continues to fall back to repo-relative `node`/server when bundled runtime is absent.

**Commands (repo root, `CARGO_TARGET_DIR=$env:TEMP\arbor-cargo-target-check`):**
- `cargo check --workspace` — exit 0
- `cargo test --workspace --no-run` — exit 0
