# Current baseline and overlap guard
Observed ARBOR main: 2570b32b47de7efe22e73a38ea4820b83bf29015. Recheck main before every lane. Earlier deep inspection pinned 8b3ffe9e2ab90272ea84c17a9f2c10f185ad46ef; this is not a full re-audit of latest HEAD.

- 2570b32: owner-approved persistent exact command learning and reuse. Inspected commit diff: catalog v13 adds control.rule.remember/revoke; desktop-only R2 learning; dispatcher binds learning preview and owner authorization; personal-rule-store remember is CAS transaction; desktop teach/list/revoke UI, private server rule read, restricted phone history, focused host tests. U5 must preserve these; no duplicate exact-learning implementation. New optional scope is safe parameter binding and receipt-derived proposals with explicit acceptance, dependency-aware reuse, not automatic action execution. If not safely supportable, stop at existing exact reuse with better test coverage.
- e67c4f81: durable native HQKG synchronization and context retrieval improvement observed in commit log; do not redo native transport; inspect actual current code before changing retrieval.
- 8b3ffe9e: shared live interpretation and authorized draft completion; U7 extends, not recreates.
- 6c6da9f3: committed structural Wiki knowledge projected to native HQKG.
- 267bdd88: request-scoped authorized source/Wiki context; U3 extends existing composer.
- fa2d057f / 1a9c92d6: approved Wiki page creation, ordering, links and navigation through existing journal.
- cbea859d / c746f68e: read-only natural-language Wiki reads through paired Host API.
- R5 commits: production journal and demo/roadmap retirement; no resurrection of synthetic product routes.

Existing code boundaries: hq/lib/indexer.mjs indexProject; semantic-registry.mjs stable ID and drift; context.mjs composeContext/verifyContext; search.mjs searchSemanticReferences; apps/vivim-host/control/intent-planner.mjs, command-intent.mjs, dispatcher.mjs and personal-rule-store.mjs; apps/vivim-host/chat/product-journal.mjs, wiki-semantics.mjs, rust-context.mjs; apps/mobile-relay/control-language.mjs, control-chat.mjs, composer-assist.mjs, vec/compiler.mjs; desktop pages/chat.js and wiki.js. Confirm current exact paths before edits. These are existing components, not locations to add competing engines.

Source audit and commit evidence: https://github.com/owenservera/ARBOR/commit/8b3ffe9e2ab90272ea84c17a9f2c10f185ad46ef and https://github.com/owenservera/ARBOR/commit/2570b32b47de7efe22e73a38ea4820b83bf29015 .
