# Proposal 5 / 06 — One-Time Master Integration: Execution Order, Work Ownership, Cutover

**Optimize for minimum elapsed integration time** by doing source-preserving work in parallel after one architecture/contract checkpoint, not by skipping verification or inventing new infrastructures.

## Stage 0 — Freeze donor evidence (read-only)

Record exact Git SHAs (including work branches with source not yet on main), source trees, current built entrypoints and tests for:
- vivim-web-harness native Tauri, Host control, mobile, Studio, media, onboarding, World;
- repos-hq HQKG Rust + operational `hq/` JS dependencies and tests;
- tierflow renderer/typed interaction and tests;
- vivim-labs ratified architecture/source, Node 0 R001, UI R005/R007, personal knowledge, Chronos, media and voice;
- old private vivim and vivim-platform only if an identified, licensed, useful component is needed.

Output: ARBOR `integration/source-lock.json`, manifest of **exact files to physically import**, branch resolution conflicts and a source redaction check. Protect historical secrets: no Git history bulk merge or `node_modules`, browser profiles, user DBs or source chats.

## Stage 1 — Source-preserving import (maximum fan-out)

Parallel workstreams, all targeting one fixed internal tree:

**A. Native:** import HQKG + programmability + Tauri under preserved paths, establish Cargo workspace, source hash checks, patched Tauri and its native binary tests.

**B. Host:** import current Host and required dependency closure under existing `apps/vivim-host/`, keep its action catalog/World/journal semantics; test read/plan/approval/receipt.

**C. Studio:** import signed-in browser worker + harness helpers under `studio/`, `harness/`; preserve Chrome profile linkage but never commit profile; test exact-thread read/send with synthetic and live authorized tests.

**D. Mobile/UI:** import PWA `apps/mobile-relay/`, one in-repo `packages/tierflow/` source snapshot and a thin inert visual adapter; test desktop/PWA UI mounts and current grants.

**E. HQ intelligence:** import selected HQ `hq/bin/`, `hq/lib/` and full checked transitive dependency into **ARBOR/hq/**. Wire HQ indexer/search/context/wiki to ARBOR-owned local data and selected source roots. No outside source/service calls.

**F. Contracts/Labs:** generate shared IDs/protocol and capability inventory from **internal** source owners; promote only ratified Lab types, tests and confirmed code into their owners. Build no generic plugin system.

Independently managed branches may cherry-pick into one ARBOR integration branch, but **one integrator owns the combined Cargo.lock/bun.lock, deployment topology, data writer and release manifest**. No concurrent unreviewed reorganization.

## Stage 2 — The narrow native backbone

- One Cargo/Bun build with internal artifacts and no external Git/path dependencies.
- HQKG native graph/typed DAG with scoped namespace, generation and source revisions.
- Source-preserving JS Host, one actual authority, no native duplicate journal.
- Small Node 0 read-only event/trace adapter with short session lock, bounded scheduler and revisioned evidence; existing JS language planner remains command execution baseline.
- Unified identity/effect/status schema and generated TS/Rust validators.
- Tauri/Host bootstrap origin and listener identity security patched before any private source exposure.
- Phone bridge binds authenticated PWA events to the same typed input and Host effect state.

## Stage 3 — Full internal capability coherence

Import currently operational capabilities, not only source files. Prove:
- actions from Host catalog, user-visible permission/receipt states;
- HQ internal dev search, graph and wiki/context on selected local repo;
- signed-in Chrome conversation and response projection;
- paired PWA read/submit/grant rejection;
- TierFlow visual projection and current status;
- personal knowledge/DCB/mission/media/voice where already operational;
- limitations and blocked Labs features surfaced through generated capability status.

The **one system truth** lives in typed IDs, source ownership, source revisions and effect transitions. There may still be JS and Rust modules; they must have no conflicting meaning or duplicate writer.

## Stage 4 — Build-test matrix and isolated donor removal

Mandatory test ordering:
1. `cargo test` and Rust graph equality/parity against frozen HQ source;
2. `bun test` for imported HQ intelligence, Host, mobile/Studio and TierFlow packages;
3. cross-language golden schema tests and NLCL/Host semantic parity for core command set;
4. Node 0 event scheduling, 500-edit/IME/stale/sensitive input adversarial tests;
5. security: foreign Tauri origin, localhost service impersonation, actor/phone revoke, untrusted provider text as command;
6. Host effect crash/uncertain-send recovery, no duplicate effects or writers;
7. Windows packaged and physical phone PWA acceptance;
8. **rename/move every donor repo and stop its background services**; rerun self-contained build/install/test/smoke with just ARBOR and user-selected test content;
9. low-storage install, index quota/compaction and rollback; published release manifest matches actual installed modules.

Do not claim this works until tests execute on Windows and real provider sessions as needed.

## Stage 5 — One controlled product cutover

1. Back up/verify current Host World, grants, journal, source identities, consented user memory, local configuration and source links. Do not include the user's personal data in Git or public CI.
2. Freeze or drain the old Host effect writer, inspect in-flight claims and unknown/uncertain external sends.
3. Install ARBOR single release identity using its internally shipped Tauri/Host/HQKG/HQ/TierFlow/Studio/PWA packages.
4. Adopt/transform data only through source-verified migrations with matching owner/version, integrity checks and reversible rollback.
5. Verify old product background processes/ports don't act as a second Host.
6. Run owner smoke B01–B12 and exact source/effect reconciliation.
7. Retire prior product launchers from delivered release and mark old checkouts as donor archives; preserve source history for audit, not runtime wiring.

## One-time integration status ledger

Every source item must be in exactly one actionable status:
`NOT_IMPORTED`, `IMPORTED`, `COMPILES`, `CONTRACT_WIRED`, `RUNTIME_VERIFIED`, `DEVICE_ACCEPTED`, `DEFERRED_APPROVED`, `RETIRED`, or `BLOCKED`.

Every row includes `sourceRepo, sourceSha, sourcePath, importedPath, capabilityOwner, existingUsers, portContract, dataClass, testCommand, observedResult, rollback, ownerSignOff`.

**Success means real old product capabilities are reachable in ARBOR, not merely that code directories exist.**

## Independent security release blocker

Current source-pinned Tauri v2.0.0 family and localhost bootstrap owner-token-to-health-port behavior need threat model and actual Windows repair. This is not optional "later optimization". Those exact issues were documented in Labs Node 0 Prime, but no fix is claimed implemented here.

## Why beta-first works

This strategy eliminates external repo wiring **and** avoids unnecessary rewrites:
- same code, moved once under one build and release;
- canonical graph crate imported once;
- one Host writer retained;
- internal HQ dev intelligence integrated now, not deferred to a future HQ API project;
- in-repo TierFlow;
- one PWA;
- one typed semantics/effect/view contract;
- only mission-critical security/liveness refactoring before beta.

Later performance refactoring is local code substitution, not another full-stack integration.
