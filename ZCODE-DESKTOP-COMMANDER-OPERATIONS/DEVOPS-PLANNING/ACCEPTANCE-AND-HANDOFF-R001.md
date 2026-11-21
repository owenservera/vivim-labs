# R001 executable verification and handoff scaffold

**State: source/contract plan, not product implementation.** No ARBOR source edits, Rust compilation or runtime/device pass were performed in this planning task. Baselines: ARBOR `e2ef16ec221fb752fd50a1e5808d7e9a7a8e3e42`; Labs `b0eb56e41d76ba785eff91858eb82584b08a1162` before planning commits.

## Gates in actual sequence

| Gate | Command/probe or owner question | Receipt required | Observed |
|---|---|---|---|
| G0 Source | Current ARBOR/Labs main, latest TASK-CLAIMS, source blob pins and Dev file ownership | Commit/hash, collision table, source symbols and owner | Source-read only, implementation repin required |
| G1 Staged R003 Rust | `cargo test --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --all-targets` | rustc/cargo version, command exit, passed/failed/skipped counts | **UNRUN**: cargo/rustc not present in this ChatGPT cloud container |
| G2 Style and static | `cargo fmt --manifest-path .../Cargo.toml --check`; `cargo clippy --manifest-path .../Cargo.toml --all-targets -- -D warnings` | exact exit, lint diagnostics | **UNRUN** |
| G3 Core registration | After Dev accepts, `cargo test --manifest-path core/programmability/Cargo.toml --lib autowiki` | compile+test after `pub mod autowiki` in existing Rust crate | **UNRUN / not merged** |
| G4 HQKG bridge | After Dev accepts, `cargo test --manifest-path native/hqkg/Cargo.toml --features native-program --lib` | existing query_scoped adapted without crate cycle | **UNRUN / not merged** |
| G5 IDs and grants | Rust negative tests: allowed exact seed, forbidden neighbor, ambiguous alias, bad source, revoked actor | exact same-origin/actor grant and graph scope, denial cases | **UNKNOWN / UNRUN** |
| G6 Source and claim truth | SentenceClaim fixtures for independent Verified vs indexed/candidate/unknown, negation/units/actor/time | claim-to-English evidence, deterministic output and genuine unknown | **UNRUN** |
| G7 Privacy/effects | Unauthorized trace, redacted title, draft log, stale revision, human-agent race, no Send | fail-closed result, no leak or auto write/command | **UNRUN** |
| G8 Host read transport | Scoped existing read port, source revision/digest, graph epoch and latest request sequence | correct 401/403, stale suppression and bounded read | **UNRUN** |
| G9 Native/browser/phone | Separately Rust core, actual GPUI screen if wired, PWA browser/Host and optional physical Android | actual screenshot/source revision/build/Host receipt, no mock promotion | **UNRUN** |

Official Cargo command/reference: https://doc.rust-lang.org/cargo/commands/cargo-test.html. Use verified current manifest paths and package/features when executing, not hard-coded assertions about unavailable commands.

## Next three useful packets

**A. Dependency and compile readiness — PL-003, AW-001, AW-002:** refresh exact code, run standalone R003 tests with actual cloud cargo if available, check Cargo feature/dependency graph. A standalone test does *not* prove an installed ARBOR module.

**B. One scoped read seam — AW-003, AW-004, AW-005, AW-010, AW-013:** derive stable aliases to existing node IDs, trusted actor grants and approved source items; bind wiki revision/digest + graph epoch + request sequence; choose existing Host read port; write comprehensive denial tests. No owner-private HQ endpoint on phone.

**C. Verified narrative and hubs — AW-006, AW-007, AW-008, AW-009, AW-011, AW-012:** crosswalk semantic verification receipts, grounded sentences, seven navigation hubs, ambient/focus behavior and inert reviewed quality diagnostics. Unknowns remain unknown, not hallucinated commands.

## Exact Dev handoff fields

```text
Canonical owner ID: [only when found in OWNER-WORK; otherwise proposal ID]
Planning scope and recipient: [Planning preparation -> ChatGPT Dev]
Inspected ARBOR / Labs HEAD: [full commit hashes]
Paths/symbols and disposition: [REUSE / REPAIR / NEW / UNKNOWN]
Files proposed / touched: [Labs source links; ARBOR left unchanged]
File collision: [Dev owner, active claims, no concurrent overwrite]
Trust model: [actor/grant, graph scope, revision, epoch, source digest, redaction]
Rust commands and actual results: [exit + environment] or UNRUN(reason)
Negative and human/agent same-lens acceptance: [test list]
Migration/rollback and source freshness gates: [exact actions]
Release status: [SOURCE, TEST, PACKAGE, SERVE, AUTH-ACK, HTTPS, GPUI, PWA, PHONE]
Sender claim: OFFERED_TO:CHATGPT-DEV; Receiver ACK: NOT_OBSERVED
```

## Never infer

- 37 test cases statically present does not mean 37 tests passed.
- A GitHub code commit or HTTP 200 does not prove visual AutoWiki in GPUI, browser or physical Android.
- An approved phone composer draft does not grant an agent permission to type into native GPUI.
- A Wiki claim retrieved from code does not imply the described product behavior works.
- A GitHub file cannot itself wake a ChatGPT session; until supported dispatch exists, mark the bridge MANUAL.
- A browser-only test is not proof of a physical Android screen; Android is not a prerequisite for Rust/GPUI core validation.
