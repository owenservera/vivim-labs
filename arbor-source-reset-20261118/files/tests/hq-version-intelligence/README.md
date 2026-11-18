# HVI F source-only conformance and adversarial suite

These tests cover T01–T24 from `05-ACCEPTANCE-TEST-MATRIX.md` with deterministic schemas, pinned S0 fixtures and **real existing C4 catalogue-delta source**. The `reference-oracle.mjs` is intentionally a fixture oracle, **not** an HVI production implementation or an authorization source. A passed oracle test does **not** prove A–E are integrated.

From repository root, Node 24:
```sh
node --test tests/hq-version-intelligence/*.test.mjs
node --test hq/test/catalogue-delta.test.mjs tests/hq-version-intelligence/*.test.mjs
```
Additional schema validation with a standards-compliant JSON Schema 2020-12 validator is required at G1 (the bundled subset validator is dependency-free and verifies tested schema keywords only). G1 must import actual A–E modules, verify SHA pin drift and offline replay, and execute at least 10k-entry bounds; G2 must verify Host live grant/revoke and TierFlow inert UI. Windows/Android beta checks remain separate and UNRUN until actual device/runner evidence.

`fixtures/hq-missing-361.json` preserves all 361 **original-path absent** HQ rows (path + Git blob SHA) extracted from S0 at exact blob `ec8bea5a5d81510cc2f60ce7201336c0bcced9f7`. No personal source content included. Their semantic state is UNKNOWN, not lost. `fixed-prefix-census.json` freezes the 13 published path-prefix measurements, not capability equivalence.

Expected gate after isolated worker commit: **PREPARED_NOT_PROMOTED**. Do not claim `TEST_EXECUTED` without command, OS/version and observed exit code.
