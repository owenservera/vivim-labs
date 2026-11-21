# R002.1 multi-surface extension status — 2026-11-21

- Multi-capture source-index import: **implemented**, `src/adapter.mjs` and `src/cli.mjs`.
- Capture index schema: `schemas/craft-artifact-index.schema.json`.
- Route fingerprints and per-artifact counters: **implemented** (metadata only; not executable).
- Multi-capture source matching, mixed/duplicate-ID refusal, CLI overwrite refusal and bounded-output tests: **4 new tests passed**.
- Total Labs local tests: **23 PASS / 0 FAIL** from `node --test test/*.test.mjs` (Node v22.16.0).
- JSON schemas: **5 PASS** (Python Draft 2020-12 schema validation).
- Genuine Craft captures, Rust helper compilation, native parity with a real bundle, live Studio readback and ARBOR integration: **UNRUN** in this environment.
- Existing ARBOR and Craft source: **unchanged** by the Labs work.

Next local action: capture multiple first-party VIVIM MVP routes within the same isolated Craft MCP session, export the bundle, construct the ID-index from actual evidence records and import with `--native-bin` after compiling the Rust example. Then reconcile against a refreshed source baseline and review the read-only atlas in Studio development tooling.
