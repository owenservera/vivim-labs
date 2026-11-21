# R006 native HQKG permission coverage — DevAgent code prep

**Purpose:** close a real blocker in R004: the query admission layer requires an attested *complete* in-prefix exclusion set but cannot derive it itself. R006 computes that set using **existing native HQKG `Graph::nodes()`** and a fresh Host-owned actor/source grant. No extra graph, journal, ACL database, scheduler, browser route or index is created.

**State:** actual Rust source + 8 authored native integration tests in Labs; static source audit only, **UNRUN** Rust compilation/integration, **NOT** installed in ARBOR. Dev product ownership remains unchanged; GPUI owner-paused.

## Code pack

- [native_grant_coverage.rs](native_grant_coverage.rs): derives `CompleteGraphCoverage` from the current actor grant and bounded native graph scan. Creates `TrustedQueryScope` only after full coverage, delegates to the **existing** R004 `read_authorized_context` and finally rechecks Host grant revision. Missing/revoked/changed grant, unknown nodes, insufficient source evidence and oversized graph all refuse.
- [native_grant_coverage_test.rs](native_grant_coverage_test.rs): 8 native integration tests for private-neighbor exclusion, missing/mismatched grant, ghost node, unauthorized source, preflight revocation, one bounded HQKG query and mid-read revocation.
- [DEVAGENT-APPLY.md](DEVAGENT-APPLY.md): source-pinned Rust installation, existing Host grant bridge requirement, feature gates, rollback.
- [VERIFICATION.md](VERIFICATION.md): commands, test/permission/scope boundaries and performance notes.

## Source basis

ARBOR `main` inspected at `7ae24ff78a797581b510fe7a78105bbe301c0de4`. `native/hqkg/src/lib.rs` owns `Graph::nodes()`, `Graph::epoch()`, and each native `Node.sources`. `native/hqkg/src/context_retrieval.rs` already owns `query_scoped`; R003/R004 Labs stage `autowiki_context_bridge.rs` and `query_admission.rs`.

The **actual Host actor-grant producer remains unverified**. `CurrentHostGrantSource` is an interface, not authentication; only the existing owner Control/permission source may implement it. A client-supplied grant or a fake `CurrentHostGrantSource` is not valid runtime authority.

## Conservative performance bound

The initial design scans no more than 65,536 **total** graph nodes and permits at most 2,048 authorized IDs/sources. It rejects a larger graph instead of asserting partial completeness. This is a safe fallback, not an optimization target: where an incumbent HQKG scoped index can authoritatively enumerate the same permitted/denied IDs, Dev may replace the scan with that EXISTING index only after equivalent test/evidence. No permanent second index.

## Acceptance status

**8 authored native tests, 0 executed in this round.** The earlier 82 standalone R003–R005 Rust cases and 2 R004 native cases are distinct authored test collections. Run native suite only after Dev receives and applies both R004 bridge + R006 module on a genuine Cargo toolchain. No ARBOR edits, runtime grant, browser screenshot or phone acceptance are claimed.
