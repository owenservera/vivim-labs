# HVI W2 U inert TierFlow inspector — source-only preparation

W2 entry pin: bbe7e4896eea02bcdec6f8fedb66eab885250161, under explicit GO_FOR_W2_TOOLING owner waiver for development only.

Implemented packages/tierflow/src/visual-runtime/knowledge/hvi-inspector.mjs. This is a pure, bounded read model to be consumed by the existing TierFlow publisher; it creates no UiStore, DOM, network source reader or Host effect. Empty/denied/revoked/partial states, 2/3/5 synthetic pins, provenance, pagination, unknown capability and runtime UNRUN are represented explicitly. No real ARBOR/VIVIM/Labs migration results are generated.

Runnable synthetic unit tests: node --test packages/tierflow/tests/hvi-inspector.test.mjs. **UNRUN**, not a pass.

Remaining W2 U: wire the read model to existing TierFlow renderer and revision guard, demonstrate accessible phone view, and verify Host authorization integration with H. All compare actions remain disabled. Status PREPARED_NOT_PROMOTED.
