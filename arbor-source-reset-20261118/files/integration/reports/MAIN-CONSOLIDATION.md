# ARBOR Proposal 5 — Canonical Main Source Consolidation
Date: 2026-11-12
Status: SOURCE_CONSOLIDATED_ON_MAIN / RELEASE_BLOCKED / REPAIR_REQUIRED

## Provenance and single source of development truth
- Main before owner-directed consolidation: `f573e090a603d99c4be45f3c30f896e29458e79a`.
- Integrated A–F and R001 baseline: `0e5ef1b599047eeff151eb37ee485e8d4397da4b`.
- Narrow A owner packaging PR #8 merged into integration at `23e17145201828bfde2508ae09fbdc0fc1c70f1b`.
- Narrow B owner PWA static route PR #9 merged at `bed77e7e8141f71412067d20d08477600d8ea3f0`.
- GitHub compare main -> integration before consolidation: ahead_by=213, behind_by=0.
- Owner-directed consolidated source PR #10 merged into **main** at `dc2ae81103925b18d962ff6c53d14c6a470b9f79`.
- Main branch subsequent G-owned changes: CI trigger/lock output switched to main; source lock, beta gates and release manifest explicitly mark source-only consolidation. Read current branch tip for final SHA.

The one canonical development destination is now `owenservera/ARBOR:main`. Original frozen A–F branches remain historical proof; they are **not** separate active release authorities. PG00–PG03/PGZ and Phase2 preparation branches are **NOT promoted**. No attempt was made to merge full research proofs as runtime features.

## Build/acceptance truth
- `Cargo.lock`: **ABSENT** on main at final source review; genuine Cargo resolution has NOT run.
- `bun.lock`: **ABSENT** on main; genuine Bun resolution has NOT run.
- GitHub Actions workflow `.github/workflows/w01g-reproducibility.yml` exists and now targets main, but observed earlier run `43233248631` ended `startup_failure` with zero jobs, so it produced **no** Cargo/Bun/Windows build receipts. Whether subsequent main push runs execute must be independently checked before assigning PASS.
- Windows MSI and bundled signed Node 24, no-donor closure, physical Android HTTPS PWA, Host native process attestation, PWA replay refusal, HQ authorization, signed-in browser provider and World/effect recovery are **UNRUN/BLOCKED**.
- The A packaging change removes the mandatory APK resource from Tauri config; the B Host change serves the existing PWA browser signer module. Neither change implements per-request grant proof or fixes native owner-token transfer.

## Security and owner repair responsibilities
1. A+B+F: real authenticated native child/Host handshake, remove owner-token-to-unauthenticated-loopback, narrow Tauri IPC origin, patch compatible Tauri graph.
2. B+E+F: binding every privileged PWA request to signed fresh nonce, actor/permission and replay ledger; physical phone acceptance.
3. G+A+F: resolve real Cargo/Bun lockfiles and compatible Cargo/MSVC build, stage genuine Node 24 runtime, complete no-donor reproducibility verification.
4. B+D+F: trusted HQ source actor/grant/revoke and personal World exclusion.
5. B+C+F: browser send-once and Host uncertain-effect recovery acceptance.
6. D+B+F: protected original storage, ENOSPC and WAL crash/recovery tests.

## Decision and rollback
**MAIN SOURCE CONSOLIDATION: COMPLETE. BETA/INSTALLER RELEASE: NO_GO. W2 ACCEPTED: NONE.**
Do not publish installer, mutate user data, grant privileged effects or call CI-only configuration acceptance.
Rollback source consolidation via reviewed GitHub revert of PR #10 merge and subsequent G commits, never force-push or delete frozen evidence. No actual rollback drill performed. The previous integration branch is an archival source reference; new scoped repairs should target branches from **main** and merge into **main**.
