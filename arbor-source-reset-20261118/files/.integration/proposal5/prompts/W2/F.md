# W02-F — Generated protocol, capability space index and interop tests

Wave 2 of ARBOR Proposal 5. Start from **W1-G GO HEAD** on `integrate/proposal5-beta-r001`, create `work/p5-w02-f-verify`. Write only: **contracts/, capability-spaces/, tests/ (root locks G only)** and disjoint `integration/worker-reports/W02-F.md`. No pushes to integration/main. Coordinate cross-owner source/protocol changes via your report, not by editing another lane's files.

## Mission — Internal Build, Native Security and Contract Wiring
Read all Proposal 5 architecture and current W1-G gate. Treat source-preserving import as the baseline: fully internal ARBOR runtime, no donor checkouts or servers after import, no wholesale JS/Rust rewrite, no new UI feature. Host JS is sole effects/World writer; native and UI proposals inert until Host admission. No secrets/user data. Only G merges and owns root Cargo/Bun files.

1. Define the smallest common source-owned wire contract for capability ID, actor/device/source revision, candidate status, view projection, request/effect/receipt states. Use existing Host catalog identifiers, not a second action registry.
2. Generate or validate TS/Rust representations and runtime validators; unknown effect states must fail closed, source_wired must not imply available_now.
3. Build capability-space projections from in-repo Host action catalog, HQ semantic IDs and TierFlow descriptors; mark unverified Labs ideas as design/preview.
4. Add synthetic interop tests for Host status read, HQKG graph source, TierFlow source projection, principal denial and stale revision. Hand off exact schema version to A–E.
5. Check import and dependency path violations, including ../repos-hq, ../tierflow and donor service URLs, without hiding legitimate user source-path access.

## Required proof
One generated/validated contract version and demonstrably nonauthoritative capability inventory with cross-language tests.

Commit source/tests/report with base/head SHA, exact import/adapter paths, test commands and exit codes, Rust/Bun/Windows caveats, source and authorization scopes, unavailable status and blockers. Report HEAD_SHA to the serial W2-G integrator; do not merge.
