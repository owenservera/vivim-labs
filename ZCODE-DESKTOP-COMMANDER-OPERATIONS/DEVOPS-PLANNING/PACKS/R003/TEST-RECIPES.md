# R003/R004 test matrix for DevAgent

All commands below are **proposed**; none were executed in the current Planning session. GitHub file read and static counting verified **55 authored standalone Rust `#[test]` markers** (previously 37 + 18 R004), plus **2** authored but uncompiled native HQKG integration tests.

## Standalone Labs code — pre-integration

From a checkout of vivim-labs:

```sh
cargo --version
rustc --version
cargo test --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --all-targets
cargo fmt --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --check
cargo clippy --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --all-targets -- -D warnings
```

Expected: zero exit on actual runnable tests, no duplicates/cycles, no unverified assertion promoted. **Current: UNRUN — cargo/rustc not installed in this ChatGPT cloud container.** Do not claim compile or test PASS from static source.

## Native ARBOR after explicit Dev acceptance

```sh
cargo test --manifest-path core/programmability/Cargo.toml --lib autowiki
cargo test --manifest-path native/hqkg/Cargo.toml --features native-program --lib
cargo test --manifest-path native/hqkg/Cargo.toml --features native-program --test autowiki_authorized
```

Only run after copying the exact modules and registering core/HQKG feature, and after moving `native_hqkg_authorized.rs` to the tested crate. Running with an unavailable feature/test target is a **FAIL/UNRUN** condition, not a passing result.

## Negative cases (source/actor fences)

| ID | Scenario | Required behavior | Current evidence |
|---|---|---|---|
| ID-01 | Exact Host `wiki:page:` id and HQKG entity id | Only source-attested binding accepted | Six authored page alias tests, UNRUN |
| ID-02 | Nonexistent/ambiguous private Wiki page | No guessed link, no private-name oracle | Staged, UNRUN |
| AUTH-01 | Prefix contains private neighbor, complete exclusion set attested | HQKG never traverses/returns excluded node; one read | Native integration test authored, UNRUN |
| AUTH-02 | No complete exclusions for current graph epoch | Refuse; never treat prefix as grant | Six staged query admission tests plus native test, UNRUN |
| AUTH-03 | Cross-user, revoked scope or untrusted source reference | Entire projection refuses, no partial private metadata | Existing R003 tests, UNRUN |
| RACE-01 | Human edit after slow agent proposal | Old input sequence never paints newer Wiki | Six authored readback tests, UNRUN |
| RACE-02 | Graph/Wiki epoch changed midread | Refuse stale, discard result and re-read source | Staged guard plus independent Host revision required, UNRUN |
| RACE-03 | Changed selection/pin/depth or source grant | Discard superseded/unauthorized context | Staged, UNRUN |
| PROSE-01 | Indexed occurrence misclassified as verified behavior | No factual English until SCF verifier evidence | Existing narrative tests, UNRUN |
| UI-01 | Same source projected into Dev-owned human and agent typing | Same latest source Wiki lens; no Submit/Search/Send effect | Dev integration required, UNRUN |
| LIVE-01 | Actual Host/PWA/GPUI visual update | Actual frame, source revision/receipt; Android separately optional | UNRUN |

## Exact passing receipt schema

```text
Source SHA: <full ARBOR commit> / <full Labs commit>
Environment: <OS, rustc version, cargo version>
Command: <verbatim>
Exit code: <integer>
Tests: <passed, failed, ignored and exact target>
Artifact: <build binary hash or test log path>
Observed authorized scope/revocation: <source-backed result>
GPUI screenshot: <actual native frame OR UNRUN>
PWA browser screenshot: <actual frame OR UNRUN>
Physical phone: <actual frame OR UNRUN, optional>
Outcome: PASS | FAIL | UNRUN, gate-specific
```

Never count a source commit as a device pass, or a mocked Host response as physical phone acceptance.
