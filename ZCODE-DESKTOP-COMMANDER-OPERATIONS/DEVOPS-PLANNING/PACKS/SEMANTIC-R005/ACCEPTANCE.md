# R005 acceptance matrix — semantic-to-human AutoWiki

**Observed source QA:** 11 Rust modules exported under existing R003 staging crate, 82 authored `#[test]` markers total in standalone crate (37 R003 +20 R004 +25 R005). R004 pack additionally contains 2 native HQKG tests. Source SHA and module entrypoint checks were reviewed, but **Rust build/test, Host integration, live browser, GPUI and phone are UNRUN** in this round: cargo/rustc unavailable in ChatGPT cloud container.

## Commands (execute only with a genuine Rust toolchain)

```bash
cargo --version
rustc --version
cargo test --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --all-targets
cargo fmt --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --check
cargo clippy --manifest-path ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001/Cargo.toml --all-targets -- -D warnings
```

After accepted Dev product integration:

```bash
cargo test --manifest-path core/programmability/Cargo.toml --lib autowiki
cargo test --manifest-path native/hqkg/Cargo.toml --features native-program --lib
cargo test --manifest-path native/hqkg/Cargo.toml --features native-program --test autowiki_authorized
```

## R005 test scenarios authored (25 tests; NOT passed)

| Scenario | Expected result |
|---|---|
| No verifier installed (`IndependentProofReader=None`) | No verified capability sentence; truthful unavailable state |
| Independent authenticated `PROVED_UNDER_MODEL` | Only qualified claim + checked language/scope may be shown |
| Test-only `SUPPORTED_BY_TESTS` | Candidate, never formal proof |
| Missing checker authentication, incomplete scope or unchecked explanation | Unknown, no verified prose |
| Refutation with independently checked counterexample | Disputed, never assert positive behavior |
| Indexed source, user declaration, runtime action without Host receipt | Unknown regardless copied SV fields |
| Changing native semantic source revision without Wiki revision change | No stale semantic fact becomes current |
| Private source/evidence | Omitted with no private diagnostics/links |
| Missing canonical Wiki hub | Unavailable, never invent a page |
| Same Wiki revision but different digest | Suppress delayed result |
| Human/agent concurrent typing | Superseded input sequence never paints |
| Forged paragraph different from verified sentences | Refuse all |
| Trace UI without owner grant | Refuse |
| Actual production verified-proof end-to-end | **UNRUN / BLOCKED** until independently implemented SV |
| Host complete grants/revocation, actual PWA rendering, GPUI/phone | **UNRUN**; downstream Dev runtime work |

**Receipt fields for any PASS:** source SHA, Rust/Cargo version, exact test command, exit code, actual counts, source/actor grant and epoch, reason for refusal where safe, artifact hash, browser/GPUI screenshot and explicit optional physical-phone evidence. SOURCE IMPLEMENTED never substitutes for TESTED or visually VERIFIED.

## Failure handling

- A compiler/test failure is FAIL, not PREPARED/READY_FOR_DEV; Dev/Planning must fix owned staging or product code.
- If no real checker/grant adapter is installed, the correct product response is an explicit unavailable state with no private content or fake certainty.
- Do not use any mock `IndependentlyCheckedProof` in production. Test doubles in Rust `#[cfg(test)]` only exercise contract plumbing.
