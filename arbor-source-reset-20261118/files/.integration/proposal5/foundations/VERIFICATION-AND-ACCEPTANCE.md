# FA-QA-001 — Verification Ladder, Native/PWA Test Matrix, and Evidence Truth

**Date:** 2026-11-12. **This foundation branch did not execute Rust, Tauri, Host, PWA, signed-in browser or personal-data migration.** Source inspections and data-only planning are not release testing. See [GAP-REGISTER.v1.json](./GAP-REGISTER.v1.json) for owner and negative proof for each source gap.

## 1. Status semantics

| Status | Required evidence | Cannot claim |
|---|---|---|
| `DESIGN_ONLY` | Architecture/PRD and identified source owners | Module implemented |
| `SOURCE_PRESENT` | Immutable code SHA/path/blob present | Runnable or integrated |
| `PREPARED_NOT_PROMOTED` | Isolated source prototype/adapter/fixture and actual test ledger | Production wiring |
| `INTEGRATED_SOURCE` | G exact merged HEAD, full dependency closure, root lock | Runtime passed |
| `TEST_EXECUTED` | Real command+exit code in accepted environment, source digest bound | Windows/device acceptance |
| `RUNTIME_VERIFIED` | Trusted actual Host/HQ/Rust source path, actor/permission and receipt | Cross-device release |
| `DEVICE_ACCEPTED` | Real installed Windows/Android HTTPS PWA/profiles and regression evidence | Full untested capability space |
| `DEFERRED` | Owner-approved exclusion, impact and future gate | Fake PASS or hidden fallback |
| `BLOCKED` | Exact failed expectation, source and owner | GO |

## 2. Test matrix for later G/owner work

| Program gate | Required tests | Current foundation status |
|---|---|---|
| G root | Cargo/Bun/Node workspace resolution; no-donor source build; source-lock/licensing | **UNRUN**: G not run |
| A pure native | Real `cargo test` HQKG/programmability with generation lease, bounded DAG, cancel, rollback | **UNRUN** in this round |
| A/B native security | Foreign port Host impersonation, challenge/nonce replay, Tauri remote IPC privilege, durable token proof | **UNRUN**, P0 |
| B Host command | Real Node Host plan/execute/journal tests on v6 catalog; R3/unknown deny; uncertain-send restart | **UNRUN**, P0 |
| D HQ runtime | Node24 `node:sqlite`, FTS5, root traversal/revoke checks, `index→search→wiki→context` | **UNRUN** |
| F portability | Validate exact JSON Schema 2020-12 and Rust/TS/JS fixture equality; deny malformed/unknown | **UNRUN** |
| S Semantic wiki | Full actual Node ESM `node --test` compiled HQ v2 source tests and negative adapter | **UNRUN** here; previous staged reported Node adapter 16/16 and V8 synthetic 12 |
| N0 native I/O | Actual Rust actor/producer fanout, cancellation/revision parity; no mutex-held UI freeze | **UNRUN** |
| V interpretation | Actual browser/TierFlow render with exact Unicode spans, privacy masking, zoom/IME/screen-reader | **UNRUN**; staged V reports V8 synthetic 39 |
| W AutoWiki | Source-authorized read from actual Host screen, revocation/late response, flip-back no lost draft | **UNRUN**; staged synthetic code exists |
| Rock storage | Measured per-volume bytes, original protection, dry-run, ENOSPC, crash compaction, rehydrate | **UNRUN**, P0 baseline |
| Studio/browser | Signed-in Chrome exact-thread read+send, unexpected result/uncertain replay | **UNRUN**, P0 |
| PWA device | Actual Android HTTPS pair/approve/revoke, WebCrypto key, lost-key refusal, offline shell, keyboard | **UNRUN**, P0 |
| PWA build | Windows Tauri installer builds without a `vivim.apk`; PWA served and installable | **UNRUN**, P0 |
| Personal T=0 | Synthetic import, bounded ZIP/JSON stream, checkpoint/idempotency/consent/erasure | **UNRUN** in this round |
| App builder | One typed UI draft preview/review/publish/rollback on PC+PWA | **UNRUN** |
| Voice | Real native audio/ASR pipeline and Host task accept; no partial auto-commit | **UNRUN** |
| Automation | Native pure due evaluation + Host journal once + crash resume; no second scheduler | **UNRUN** |
| Extensions | Real pure engine install/deny/revoke/swap/rollback with bounded provenance | **UNRUN** |

## 3. Minimum "truly reprogrammable" proof

Reject any future report claiming programmable platform implementation unless all are observed:
1. Source-attested manifest installed and owner-admitted against actual G HEAD.
2. Pure registered Rust engine generation v1 is pinned and running under resource bounds; v2 installs, v1 lease still returns v1 output.
3. A stale source or policy event cancels/removes old dependent outputs without confusing user text revision or Host actor.
4. Rollback to a reviewed known manifest works; history/revisions remain visible, and no effect journal is touched by pure engine machinery.
5. Rust/TS/JS ABI and typed source status passes actual toolchain tests; a real UI reflects revision changes on PC/PWA with accessibility parity.
6. Effectful action *only* uses current B Host typed catalog/risk/approval/journal and explicit user confirmation as required.

## 4. Highest-risk negative corpus

- Fake native Host listener returns HTTP 200, tries to steal persistent owner bearer — must refuse.
- Stolen phone grant/session request without bound key proof — must refuse; revoked phone session cannot read source or execute.
- Invalid/quoted/negated prompt `do not delete`, `don't send`, mixed high-risk intent — zero unwanted effect.
- Wrong semantic ID, source hash, stale Host catalog, expired source permission — no candidate wrongly marked executable.
- Comment `@hq:executor`, untrusted wiki page or LLM output attempts to install new action — refuse.
- Source reindex/revocation while user composes — stale results do not overwrite or leak denied nodes.
- Old Tab/undo selection applied after an intervening edit or IME composition — reject stale transform.
- User disk <reserve> — read-only graceful mode, no data loss; unknown WAL/graph source must not be auto-evicted.
- Service worker offline and phone reconnect — no cached authenticated POST, no silent submit/replay.
- Real signed-in provider send times out after click — uncertain result with no automatic retry.
- Engine generation swap, cancellation and exception — no second effect handler and no revived revoked knowledge.
- Personal memory/ DCB flavor hint — not a saved definition or durable grant until actual Host owner receipt.

## 5. Owner acceptance protocol

Every wave must emit `BASE_SHA`, `HEAD_SHA`, `changed_paths`, `source_import_blobs`, `actual_commands`, `exit_codes`, `negative_tests`, `runtime/device status`, `privacy effects`, `blockers`, `G handoff`. A V8 synthetic check is described as V8, not Node CLI. A source diff is source inspection, not executable acceptance.

**No fallback from a FAIL to user content synthesis.** When a registered command is unavailable, the truthful UI answers from safe source if permitted and labels the command path unavailable. G alone can promote source into merged release.
