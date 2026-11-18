# W02-B — Host internal World/effect and bootstrap bridge

Wave 2 of ARBOR Proposal 5. Start from **W1-G GO HEAD** on `integrate/proposal5-beta-r001`, create `work/p5-w02-b-host`. Write only: **apps/vivim-host/** and disjoint `integration/worker-reports/W02-B.md`. No pushes to integration/main. Coordinate cross-owner source/protocol changes via your report, not by editing another lane's files.

## Mission — Internal Build, Native Security and Contract Wiring
Read all Proposal 5 architecture and current W1-G gate. Treat source-preserving import as the baseline: fully internal ARBOR runtime, no donor checkouts or servers after import, no wholesale JS/Rust rewrite, no new UI feature. Host JS is sole effects/World writer; native and UI proposals inert until Host admission. No secrets/user data. Only G merges and owns root Cargo/Bun files.

1. Make imported Host start from ARBOR's release paths only, retaining current single control dispatcher/action catalog/journal/World writer and source data storage layout.
2. Implement Host-side matching process-instance/nonce handshake for A's Tauri bootstrap; refuse unrecognized callers and avoid sending persistent owner token over unknown localhost listener.
3. Expose the smallest authenticated internal read/intent/status port needed by native Node0 and PWA, with current action source IDs and readiness (never claim source_wired is runtime_available).
4. Preserve exact approval/claim/receipt/recovery behavior and validate existing source fixtures; ensure no second Host writer is added by compatibility glue.
5. Provide bounded internal request timeout/version policies and report any source/World migration compatibility conflict.

## Required proof
Real internal Host startup and safe status path; one current owner journal, matched native bootstrap contract and source-backed control tests.

Commit source/tests/report with base/head SHA, exact import/adapter paths, test commands and exit codes, Rust/Bun/Windows caveats, source and authorization scopes, unavailable status and blockers. Report HEAD_SHA to the serial W2-G integrator; do not merge.
