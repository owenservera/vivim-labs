# Sprint 02 — Required Reading & Write Boundaries

## Era 1 — Command
Repo `owenservera/vivim-web-harness`:
- `apps/mobile-relay/control-language.mjs`
- `apps/mobile-relay/nlcl/{index,interpret,grammar,lexer}.mjs` (vendored, **read only**)
- `apps/vivim-host/control/{intent-planner,dispatcher,action-catalog.v1.json,lexicon}.mjs`
- `apps/vivim-host/test/{control-intent-planner,control-language}.test.mjs`
- `apps/mobile-relay/test/control-tab-navigation.test.mjs`
- [intent front-end audit](https://github.com/owenservera/repos-hq/blob/main/docs/audit-2026-11-04/01-intent-front-end.md): known bug: negation dropped, missing routing contract.
Owned changes: control-language, intent-planner, new command IR adapters, corresponding focused tests. Don't change dispatcher without explicit coordination.

## Era 2 — Rust comment compiler
Repo `owenservera/repos-hq`:
- `hq/lib/{protocol,annotation-v2,wiki-program}.mjs` + `hq/test/{annotation-v2,wiki-program}.test.mjs` (**read only**)
- `docs/research/semantic-language-kernel/{ARCHITECTURE,CONTRACT-AND-ROADMAP}.md` (**read only**)
Owned changes: NEW `research/comment-program-rust/**` including Cargo manifest, Rust parser/compiler CLI, tests and fixtures. Don't modify existing Rust HQKG graph.

## Era 3 — Wiki bridge
Repo `owenservera/repos-hq`:
- `hq/lib/{wiki,wiki-program,semantic-registry,action-registry,action-router}.mjs`
- `hq/test/wiki-program.test.mjs`, `hq/bin/hq.mjs` (read-only cross-cutting entrypoint)
- Host dispatcher code from web harness (**read only**).
Owned changes: NEW `hq/lib/comment-wiki-bridge.mjs`, `hq/test/comment-wiki-bridge.test.mjs`, optional NEW `hq/lib/wiki-runtime/**`.

All: re-fetch current main, inspect actual function signatures, preserve existing source authority and unrelated local-only work.
