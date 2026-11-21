# Sprint 02 — V0.1 Acceptance and Stretch

## Era 1 — Minimum
- C1: Deterministic JS/TS command-intent/0.1 with refusal-preserving negation, conjunction, unknown IDs and ambiguity; old Host planner unaffected.
- C2: Same valid command on mobile/desktop resolves to same typed action; text alone never bypasses permission.
- M1: Rust compiler parses *real comments*, not tags hidden in strings; produces stable `hq.comment-program/0.1` JSON, line/digest and bounded diagnostics.
- M2: Program compiles into a config accepted by existing `hq.wiki-program/1`; unregistered interaction ID, unsafe command, missing registry or secret is refused.
- W1: JS/TS bridge composes with existing preview, diff, explicit activation, version and rollback APIs; comment change updates the wiki view.
- W2: Wiki user gesture and natural-language command yield same typed intent (fixture conformance), no additional effect authority.

## Era 2 — Stretch
100+ command fixtures (including bounded English/Spanish cases if actually supported); 100+ comment lexer/type/registry/secret fixtures; multi-file incremental comment-program build with source digests; deterministic wiki diff and undo; measured compiler performance; at least one complete source-comment → Rust compile → wiki preview → approved activation → undo workflow.

## Era 3 — Evidence rules
Code commits are not a live-device result. Report actual test commands, results and unrun gates. No arbitrary execution from comments. No provider APIs, Playwright/Puppeteer, duplicate browser actor or unnecessary language rewrite. Prioritize code and tests over more documents.
