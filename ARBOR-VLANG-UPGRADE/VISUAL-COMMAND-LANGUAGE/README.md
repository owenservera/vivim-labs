# Visual Command Language

Central design home for the intelligent composer, user-verification feedback, and visual semantics. Runtime remains ARBOR/main, one native GPUI renderer, Host execution authority.

Start with [Master Visual Inventory R001](MASTER-VISUAL-INVENTORY.md): 241 design candidates in 12 families, source crosswalk, implementation priorities and acceptance requirements. This is a design catalogue, not a claim of implementation.

Existing engineering tracking: [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33), especially U09/U08/U10 and G02–G05. Do not duplicate the backlog. Future subdirectories: COMPOSER-DESIGN, VISUAL-EXPERIMENTS, RUST-GPUI-IMPLEMENTATION, USER-ACCEPTANCE. Add the semantic visual grammar after reviewing the design vocabulary.

## Compositional SDK R001

- [Semantic visual grammar and SDK](SEMANTIC-VISUAL-GRAMMAR.md) — semantic graph, visual expression IR, geometry, external anchors and interaction engine.
- [Machine-readable axis registry](VISUAL-AXIS-REGISTRY-R001.json) — 47 factorized configurable axes, constraints and budgets.
- [Email scene example](VISUAL-SCENE-EMAIL-EXAMPLE-R001.json) — illustrative nested/disjoint wrappers, relations and account selector.

Design candidates only; production Rust and GPUI integration remain unimplemented in this round.
