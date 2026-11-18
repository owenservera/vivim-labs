# W02-C — Studio Chrome/browser internal runtime

Wave 2 of ARBOR Proposal 5. Start from **W1-G GO HEAD** on `integrate/proposal5-beta-r001`, create `work/p5-w02-c-studio`. Write only: **studio/, harness/** and disjoint `integration/worker-reports/W02-C.md`. No pushes to integration/main. Coordinate cross-owner source/protocol changes via your report, not by editing another lane's files.

## Mission — Internal Build, Native Security and Contract Wiring
Read all Proposal 5 architecture and current W1-G gate. Treat source-preserving import as the baseline: fully internal ARBOR runtime, no donor checkouts or servers after import, no wholesale JS/Rust rewrite, no new UI feature. Host JS is sole effects/World writer; native and UI proposals inert until Host admission. No secrets/user data. Only G merges and owns root Cargo/Bun files.

1. Resolve Studio/Host imported relative-path and package dependencies entirely within ARBOR; no child process loads donor source paths.
2. Keep the supported signed-in dedicated Chrome profile path configurable outside Git; reuse current CDP session, receive/capture and exact-thread/send-once logic rather than rewriting browser automation.
3. Expose/validate one narrow internal Studio bridge to current Host effect owner with correlation, provider/thread, body digest, attempt/unknown/observed statuses; no direct UI authority.
4. Run synthetic Studio/provider fixtures and start/health tests where safe; mark real provider login-dependent cases pending W3.

## Required proof
Internal Studio import/run tests and Host bridge contract; no provider API or extra effect writer.

Commit source/tests/report with base/head SHA, exact import/adapter paths, test commands and exit codes, Rust/Bun/Windows caveats, source and authorization scopes, unavailable status and blockers. Report HEAD_SHA to the serial W2-G integrator; do not merge.
