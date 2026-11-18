# PGZ — Independent negative acceptance and actual-versus-required

Every case is **specified** unless an explicitly scoped source-content/V8 check or the **worker-reported PG02 synthetic Node fixture** is noted. No fixture PASS may be reinterpreted as a live Host effect/security acceptance.

| ID | Threat/condition | Required observed refusal and owner | Test/command when G exists | PGZ evidence |
|---|---|---|---|---|
| REF-G01 | G absent, worker import from isolated PG proof | No product adoption; original G sole merge | `git merge-base`, G source lock/compare and W01-GATE | **ABSENT_404**, all four worker diff scopes checked |
| REF-A01 | Foreign 43187 listener replies healthy | No durable owner bearer or UI code to attacker, only verified spawned child | A/B forged-listener integration + Tauri child tests, actual Windows | **BLOCKED/UNRUN**; A literal URL read |
| REF-A02 | Wrong PID/launch ID/nonce/HMAC/remote IPC origin | Fail closed, no UI/Host effect session | A/B native-attest tests, stale/replay/collision | **BLOCKED/UNRUN**; wildcard Tauri remote read |
| REF-A03 | Unknown engine manifest or changed in-repo digest | No install or capabilities despite syntactically valid source hash | `cargo test --manifest-path native/hqkg/Cargo.toml --test pg01_engine_abi` plus attestation test | Fixture **UNRUN**; admission absent |
| REF-A04 | Stale expected generation, downgraded capability, old pinned lease | Atomic refuse; old lease still v1; rollback installs v1 as generation 3 not time rewind | Same actual Cargo tests with exit | Fixture **UNRUN** |
| REF-A05 | Malicious blocking ComputeEngine ignores cancel, source revoked during work | Never claim force-stop; discard late output, no source leak or effect receipt | Rust simulated blocker + sourceEpoch checks | **BLOCKED** by source API and actor binding |
| REF-F01 | A 2-domain vs F 4-domain / N0 engineVersion mismatch | Reject unknown mappings, no forged actor/owner | Rust/TS/JS structural interop conformance | **UNRUN** |
| REF-S01 | @hq comment/semantic alias asserts runnable Host effect | No Host handler or grant; source/link read only | `node --test .integration/proposal5/phase2/pre-g/S-semantic/inert-adapter.test.mjs` + D/B integrated test | Source inspected, **UNRUN** by PGZ |
| REF-S02 | Source bytes/digest stale or D source root revoked | Deny HQ/wiki/personal link, hide denied identity | D verified walker tests (symlink/junction/escape/replace) | **UNRUN** |
| REF-S03 | Unknown/stale Host v6 ID, R3, unsupported full update | No action; original text B replan, current risk/source/World | B dispatcher/journal tests + PG02 test | PGZ catalog parity **V8 PASS**, live **UNRUN** |
| REF-V01 | Unicode scalar→UTF-16 surrogate cut, IME composing, negation 'don't send' | No preview action, preserve original caret and text | E visual command/UI tests, Android IME | PG02 isolated **worker Node synthetic 20/20**; real **UNRUN** |
| REF-V02 | Stale source/catalog/policy/graph/World revision, slow producer | Drop stale result, surface `skipped_budget/failed/unavailable` without private identifiers | A/E/F N0/V revisions and cancellation tests | Synthetic-only |
| REF-W01 | ScreenContext trusted-looking but no B read scope; personal DCB in dev HQ | No source read, no automatic AutoWiki write; immediate revoke/flip-back hide | W mirror tests + D/B scoped read negatives | W source inspected; integrated **UNRUN** |
| REF-B01 | E imports browser-device-key.mjs but B allowlist omits it | No successful enrollment until B serves exact source; 404 test must fail prepatch | `node --test apps/mobile-relay/test/host-static-allowlist-contract.test.mjs apps/mobile-relay/test/browser-enrollment-flow.test.mjs` | **V8 independently confirmed missing import route** |
| REF-B02 | Stolen grant+active session reused on a fresh privileged operation | Signature bound to method/path/query/body/nonce/actor and server atomic replay fence; no new plan effect | B `enrollment-security.test.mjs` and fresh-plan replay adversarial test | **BLOCKED/UNRUN**; B bearer-only source read |
| REF-B03 | Revoked signer/key, wrong Origin/trusted proxy, SW offline POST API | Fail closed, re-pair, no offline effect replay or privileged cache | B/E Host HTTP + Android HTTPS tests | SW source only, **DEVICE UNRUN** |
| REF-C01 | Studio unknown result after provider send | C one sender, journal uncertain; no auto resend | C exact Chrome thread-turn-journal tests and signed-in browser | **UNRUN** |
| REF-R01 | Disk low / WAL/-shm/temp or VACUUM INTO fails mid-commit | No loss of original/notes/receipts/World/journal, recover/rollback | D/B synthetic ENOSPC, crash and backup restore | **UNRUN; actual bytes null** |
| REF-R02 | Rebuildable flag but source absent/revoked/pinned/leased | Deny eviction/rehydrate, no private source leakage | HQ Rock pure suite + integrated file system harness | Research-only, **UNRUN** |
| REF-P01 | Windows installer requires vivim.apk or absent prepare-host-runtime.mjs | Build beta without APK; retain B apk/ grant source; Android PWA over real HTTPS | G/A/F Windows packaging and real phone install | APK required in frozen Tauri; **BLOCKED/UNRUN** |

## Test accounting
- **Actual independent PGZ**: GitHub refs/compare, source fetch and **8/8 content/contract V8 assertions** in connector execution. No Node/Cargo process, no exit code for V8.
- **Inherited but source-reviewed PG02**: Node.js v22.16.0 `node --test test/semantic-visual-crosswalk.test.mjs`, worker reports **exit 0/20 pass**, module blob `8d773c67888634c52e9ea2ad80e42efa5eeac8dd`, test blob `0862a2eeab19afc72008b2df86dcde60d5581916`. **Not rerun by PGZ.**
- **PG00** Node 12 metadata tests, **PG01** Cargo two files, **PG03** Node 5 metadata tests are source-authored but **UNRUN**. Full S/V/W/N0 original suites, real Chrome, no-donor combined Rust/Node/Bun, Windows, Android, storage ENOSPC **UNRUN**.
- Test record for any future PASS: exact accepted G SHA, source file/fixture blob, command, runtime/OS/device, exit code, log, before/after authority + receipt, observed negative behavior and rollback.
