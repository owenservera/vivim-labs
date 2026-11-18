# W02-D — Operational HQ intelligence wired internally

Wave 2 of ARBOR Proposal 5. Start from **W1-G GO HEAD** on `integrate/proposal5-beta-r001`, create `work/p5-w02-d-hq`. Write only: **hq/** and disjoint `integration/worker-reports/W02-D.md`. No pushes to integration/main. Coordinate cross-owner source/protocol changes via your report, not by editing another lane's files.

## Mission — Internal Build, Native Security and Contract Wiring
Read all Proposal 5 architecture and current W1-G gate. Treat source-preserving import as the baseline: fully internal ARBOR runtime, no donor checkouts or servers after import, no wholesale JS/Rust rewrite, no new UI feature. Host JS is sole effects/World writer; native and UI proposals inert until Host admission. No secrets/user data. Only G merges and owns root Cargo/Bun files.

1. Make the imported hq/bin and hq/lib dependency closure runnable entirely from ARBOR with its own local development source index, schema, registry, search, context and generated wiki, without external HQ process/data.
2. Preserve Node runtime compatibility (node:sqlite and supported source imports), source ignore/secret filters and index watermark/reference semantics.
3. Expose a minimal internal read-only source/index/context/semantic-ID API or CLI export, versioned and actor-scoped, callable by Host/native from ARBOR package paths; do not write personal World data.
4. Prove index→search→context→wiki against a synthetic repo under ARBOR tests, report actual output/version and missing capabilities, and honor disk-size limits.

## Required proof
In-repo HQ CLI/index/context/wiki work using synthetic source and documented stable read schema.

Commit source/tests/report with base/head SHA, exact import/adapter paths, test commands and exit codes, Rust/Bun/Windows caveats, source and authorization scopes, unavailable status and blockers. Report HEAD_SHA to the serial W2-G integrator; do not merge.
