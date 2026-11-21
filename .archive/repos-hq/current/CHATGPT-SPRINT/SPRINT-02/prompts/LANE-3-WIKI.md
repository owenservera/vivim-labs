# Paste into ChatGPT High: Lane 3 — Intelligent Wiki Runtime Bridge V0.1

You own **Lane 3** of `owenservera/repos-hq/CHATGPT-SPRINT/SPRINT-02`. Work now with High reasoning effort; ship executable JS/TS bridge, tests and direct-to-main commits. Read README, CONTRACT, SOURCE-MAP, ACCEPTANCE; inspect actual HQ wiki/program, annotation, action registry and semantic registry APIs.

**Mission:** Make the **Rust-compiled inline-comment program run as a safe intelligent interactive wiki configuration**: preview/diff, explicit owner-confirmed activation, revision and rollback. Bridge wiki interactive gestures and the natural-language commands from Lane 1 to equivalent typed intent. Use **JS/TS** as practical integration, not a new language and not a Rust rewrite of NLCL.

**Write ONLY** NEW `owenservera/repos-hq/hq/lib/comment-wiki-bridge.mjs`, `hq/test/comment-wiki-bridge.test.mjs`, and optional NEW `hq/lib/wiki-runtime/**`. Read-only: `hq/lib/{wiki-program,wiki,annotation-v2,semantic-registry,action-registry,action-router}.mjs` and `hq/bin/hq.mjs`. Never touch Host, browser/CDP, or Lane 2's Rust files.

**Must ship:** a strict `hq.comment-program/0.1` validator/loader; compatibility with existing `validateProgramConfig`, `previewProgram`, `activateProgram`, `rollbackProgram`; source maps, diff, registry-validated inert bindings; no action without separate owner approval and native effect handler. Deterministic fixture: comment compiled program → wiki preview → activation → undo. Another fixture: wiki click and natural-language command resolve to equivalent typed intent. You may use a frozen Rust output fixture while Lane 2 builds, then verify the real compiler output when available.

**Stretch:** interactive wiki controls, incremental comment recompile/diff, per-file provenance and 100+ integration tests. Keep Repos HQ a knowledge/interactive wiki environment, **not** another Host/browser effect owner. Provide stable contracts for later cross-repository integration.

Small commits on main, re-fetch SHAs, no PRs/branches/Actions/force push. Report exact tests and commits, what is working, what remains unverified on owner devices.
