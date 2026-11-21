# ARBOR local implementation handoff — native computer control

**Use this only when executing actual ARBOR product work.** Research is in Labs, but all executable code and production tests belong to `owenservera/ARBOR@main`. Preserve the current user preference: Rust by default, TypeScript 7+ for necessary browser/PWA glue, Bun not npm, source changes on main, no PRs or feature branches, no force pushes.

## First task (copy/paste)

> You are integrating native natural-language computer control directly into ARBOR, inspired by Desktop Commander's *capabilities* but independent of its SDK, proprietary relay and runtime. Refresh latest main, inspect real Host command catalog, NLCL parser, command projector, dispatcher/journal, Rust product kernel, GPUI supervisor, PWA and any existing filesystem helpers. Read `vivim-labs/vivim/chatgpt-plugin-ecosystem/native-command-space/` for the **research** proposal and candidate registry; do NOT import its 279 `status=proposed` commands as executable actions. Implement one end-to-end Rust-native, user-scoped, read-only filesystem **largest-files** capability through the incumbent Host plan/receipt path. Validate a real selected folder, bounded traversal, source-based file sizes, exact sorting/limit, denied paths/junction/symlink cases, stale/replaced file handling, cancellation and output limits; no raw shell or arbitrary filesystem grants. Preserve existing 38-action catalog compatibility and R3 nonexecutable rule. Wire PWA/GPUI to the existing Host plan/inspect surface as needed, no extra Host/writer/permission/journal. Use true commands and exits in results, commit on main with fast-forward preservation. If live Windows execution is unavailable, say UNRUN explicitly and implement/test only what can really be run. Report source commit, files, tests PASS/FAIL/UNRUN and next candidate action.

## Thin-slice acceptance protocol

1. `git rev-parse HEAD`, `git status --porcelain` before any edits; preserve all stashes and other changes.
2. Inspect current `action-catalog.v1.json`, `dispatcher.mjs`, `intent-planner.mjs`, `command_intent_projection.rs`, `product_kernel.rs` and actual Host endpoint/auth/receipt definitions. Record SHAs.
3. Build/execute a scoped Rust-native adapter in the **Host execution layer**, not the existing pure kernel.
4. Introduce versioned typed file target projection without granting a raw path outside approved scope.
5. Create action registration only if it has an actual callable executor; keep readiness `adapter_pending` until source wired, and `verified` only after live accepted.
6. Test positive: folder with real files, nested files, size sorting, unicode, repeated names, inaccessible entries, actual correct limited output.
7. Test negative: no grant, other user/device, traversal, symlink/junction escape, stale file identity, malformed argument, oversized output, cancelled operation.
8. Inspect actual Host journal and verified read receipt. Do not fabricate a result or bypass the Host to exercise a helper.
9. Run existing command regression tests. If not possible, report exact command and reason UNRUN.
10. Commit and push only the feature's source files and tests to main without overwriting concurrent changes.

## Required discipline on breadth

Prioritize real primitives and interoperability; every command added to production must cite a real adapter owner, Host authorization/effect path, schema, risk, reversible status, source revision, tests and surface parity. The JSON catalog here is **research backlog**, not an executable runtime feature catalog. No bulk-generated fake executors, arbitrary shell pass-through or redundant audits.

## Follow-on after first slice

- File name/content search and storage breakdown.
- Scoped reversible create/rename/move, with digest preview and receipt-backed compensation.
- App/process inspection, then approved process spawn/session.
- Typed developer cargo commands.
- Multi-step selection → preview → claim → effect DAG.
- Studio browser operations using existing Chrome lease; connected application surfaces only after authorization.
- Evaluation corpus expansion and measurement of genuine 90% completion.
