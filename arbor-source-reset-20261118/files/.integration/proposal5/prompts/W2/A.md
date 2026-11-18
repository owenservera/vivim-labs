# W02-A — Native Rust graph, Tauri and trusted IPC

Wave 2 of ARBOR Proposal 5. Start from **W1-G GO HEAD** on `integrate/proposal5-beta-r001`, create `work/p5-w02-a-native`. Write only: **src-tauri/, native/, core/programmability/** and disjoint `integration/worker-reports/W02-A.md`. No pushes to integration/main. Coordinate cross-owner source/protocol changes via your report, not by editing another lane's files.

## Mission — Internal Build, Native Security and Contract Wiring
Read all Proposal 5 architecture and current W1-G gate. Treat source-preserving import as the baseline: fully internal ARBOR runtime, no donor checkouts or servers after import, no wholesale JS/Rust rewrite, no new UI feature. Host JS is sole effects/World writer; native and UI proposals inert until Host admission. No secrets/user data. Only G merges and owns root Cargo/Bun files.

1. Build imported HQKG and programmability as real in-repo dependencies of Tauri, without an external graph process. Preserve HQKG algorithms, source-domain namespaces and existing validator functions.
2. Upgrade coherent pinned Tauri v2 dependency family to a currently patched supported version, audit native capabilities/origin/withGlobalTauri and prepare the real Windows build; ask G to regenerate root lockfile.
3. Implement the Tauri-side process identity/owner-token-safe bootstrap handshake with B-owned Host; never disclose owner credential on an unauthenticated health HTTP success. Use a synthetic token adversarial test.
4. Provide a minimal typed native read-only graph/health command, actor-bound from trusted invocation context; avoid implementing any new effect or general native command router.
5. Report exact Rust/Tauri build commands and link results, including unknowns on non-Windows host.

## Required proof
Internal Cargo native compile/tests and constrained ingress mechanism; source evidence of patched Tauri/origin with adversarial tests pending W4.

Commit source/tests/report with base/head SHA, exact import/adapter paths, test commands and exit codes, Rust/Bun/Windows caveats, source and authorization scopes, unavailable status and blockers. Report HEAD_SHA to the serial W2-G integrator; do not merge.
