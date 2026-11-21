# R006 test/permission/scale matrix

**State:** Rust source authored; 8 test cases authored and staged. All Cargo/Host/Windows/GPUI/browser/phone checks **UNRUN** in this ChatGPT cloud environment, where cargo/rustc are absent. Counts are static `#[test]` markers.

## Commands after Dev accepts source

```sh
cargo test --manifest-path native/hqkg/Cargo.toml --features native-program --test autowiki_grant_coverage
cargo test --manifest-path native/hqkg/Cargo.toml --features native-program --test autowiki_authorized
cargo test --manifest-path core/programmability/Cargo.toml --lib autowiki
cargo fmt --manifest-path native/hqkg/Cargo.toml --check
cargo clippy --manifest-path native/hqkg/Cargo.toml --features native-program --all-targets -- -D warnings
```

## Refusal/coverage

| Test | Condition | Expected |
|---|---|---|
| R6-01 | Native graph has public and private connected nodes | All private in-prefix IDs excluded **before** existing HQKG BFS |
| R6-02 | No Host actor grant | Refuse, no source or title disclosure |
| R6-03 | Granted user differs from source actor | Refuse before read |
| R6-04 | Grant names nonexistent graph entity | Refuse rather than synthesizing node |
| R6-05 | Allowed entity carries unapproved source ref | Refuse rather than partial redaction or title leak |
| R6-06 | Grant revoked during complete scan | Refuse even without graph epoch change |
| R6-07 | Current grant and graph | Existing `query_scoped` only; allowed cards, no private nodes |
| R6-08 | Grant changed between query and postflight | Refuse, no stale projection |
| R6-09 | Over scan bound (not unit-tested by default to save PC disk/time) | Refuse with ScopeTooLarge; add bounded integration test on appropriate runner |
| R6-10 | Permission revoked between return and render | Must be denied by existing Host current-grant readback gate (integration still needed) |

## Evidence

A PASS requires exact ARBOR source SHA, Rust version, Cargo command + exit, test count/log, current source-owned actor/grant revision, current HQKG graph epoch, Host read receipt, and denial behavior; it does **not** require Android. Actual GPUI/PWA visual proof must remain separately labeled and is NOT implied by native tests.

## Efficiency constraints

No synthetic product-state fallback, no new graph index, no provider API, no detached worker, no second permissions service. The graph scan is capped and correctness-first. Future optimization must reuse HQKG's current indexing/namespace capabilities where they can prove identical complete exclusions.
