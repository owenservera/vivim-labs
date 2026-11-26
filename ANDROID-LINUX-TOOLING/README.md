# Android Linux Tooling — R001
Research/evidence only; Product remains the SDK and capability authority. 2026-11-26.

## Incumbent bridge
VIVIM Android MCP 0.3.0, live Termux development profile, native AArch64 Android. Twelve observed operations: health, probe, capabilities, terminal_start, session_status, session_output, session_input, session_cancel, directory_list/create, file_read/write. The protocol accepts idempotency request IDs, explicit working directory and process duration, exposes byte-cursor output and terminal receipts, and supports bounded stdin (not PTY). Grant is time-bounded; max job 30 minutes. Termux UID/process-group best effort is NOT a sandbox. Do not expand grants, access other app data, or bypass OS security. Bridge source revision not yet pinned; this is an observed tool contract, not a source audit.

## Public-safe device evidence
An authorized physical Android 15 ARM64 device has native Rust/Cargo 1.100.0, Go 1.27.4, Clang 21.1.8, Python 3.14.10, Node 24.22.3, Git 2.56.1, CMake 4.5.1, Tree-sitter 0.27.2 with multiple grammars, SQLite 3.54.1, NumPy and PyTorch packages. Java 17 and ADB are installed; ADB privileges and APK build/installation are NOT established. Memory headroom is constrained; use one bounded job initially. Raw device inventory, identifiers, grant/session tokens, personal files and logs are deliberately excluded from public Labs.

## Actual R001 experiment
Source: [native_smoke.rs](experiments/native_smoke.rs), exact source commit 897485b25af868df5c0e38b772a42d34c0fe115e. Uploaded to a dedicated local workspace and read back after test.
Command: rustc --test native_smoke.rs -o native_smoke; ./native_smoke --test-threads=1
Physical device result: 3 passed, 0 failed; process exit 0. Native target, file roundtrip and child-process execution verified. Python sqlite3 in-memory SELECT 42 returned (42,), SQLite 3.54.1. Node returned android arm64. Temporary test executable removed. No package installs, root, system settings changes or product migration. Not proof of SDK integration, restart recovery, power resilience, or UI parity.

## Research direction
Use cloud/GitHub for canonical source; Windows x64 and Android ARM64 as complementary native workers. A phone is a compute participant, not a separate visual renderer. Keep existing Product SDK, task catalogue, capability registry and Host effect authority. Reuse session receipts and graph-native descriptors; do not build a rival orchestration engine.
Candidate VLANG semantics (not accepted APIs): node discovery and capability lease, resource snapshot, execution placement, exact source pin, bounded run, cursor read, cancellation, checkpoint, resume, evidence attestation. Bind operation identity, task/source pin, worker identity, grant scope, platform, resource budget, exit/output digest. A dropped connection is not success; reconcile prior receipt and never blindly replay effects.

## Next research experiments (not canonical task assignments)
1. Pin bridge source and map every tool to existing SDK contracts; test duplicate request, timeout, cancel, and reconnect with bounded harmless workload.
2. Run exact Product SDK commit on Android; use one Cargo job, scoped checkout, dependency budget and explicit platform blockers. Do not migrate source in Labs.
3. Evaluate local Tree-sitter/HQKG incremental parsing, SQLite persistence and VLANG evaluation on public fixtures.
4. Test resource-sensitive scheduling, checkpoint/restart, interrupted transfer, and evidence comparison with Windows/cloud.
5. Benchmark small CPU-only models/embeddings only after measuring memory; do not download large weights or claim NPU support. Evaluate offline deterministic capability replay first.
6. Evaluate native Android permission-mediated capabilities separately; no UI automation, data access, or device changes outside explicit scope.

## Upstream research
- Rust platform support: https://doc.rust-lang.org/stable/rustc/platform-support.html (Android target is distinct from GNU Linux).
- Termux execution environment: https://github.com/termux/termux-packages/wiki/Termux-execution-environment
- Termux Android process limitations: https://github.com/termux/termux-app#readme
- llama.cpp Android build: https://github.com/ggml-org/llama.cpp/blob/master/docs/android.md
Android can terminate background processes; persistence requires recovery rather than assumptions of daemon uptime. No OS limits changed. Local inference feasibility remains untested.
