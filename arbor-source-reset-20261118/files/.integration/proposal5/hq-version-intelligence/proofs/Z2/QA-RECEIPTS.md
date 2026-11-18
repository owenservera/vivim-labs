# Z2 QA receipts

**No executable Node/Bun/Cargo/Windows/Android tests were run by Z2.** This GitHub connector provides source and commit access, not a checked-out executable runtime. An attempt to reach GitHub from an available remote container failed DNS; container has Node 22, not the required G1 Node24; no real corpus was accessed. CI `startup_failure` means **zero OS test exit receipts**, not failed source tests.

| Test | Observed truth | Test process exit |
|---|---|---|
| G1 GitHub Actions | startup_failure; runs 43307067218, 43307026813 | NONE |
| H GitHub Actions | startup_failure; run 43310564183; jobs=0 | NONE |
| U GitHub Actions | startup_failure; run 43311616409 | NONE |
| H Node test | UNRUN: node --test apps/vivim-host/test/hvi-capability.test.mjs | NONE |
| U Node test | UNRUN: node --experimental-strip-types --test packages/tierflow/tests/hvi-inspector.test.mjs | NONE |
| G1 combined Node24 | UNRUN: node --test tests/hq-version-intelligence/*.test.mjs | NONE |
| G1 strict AJV Draft 2020-12 | UNRUN | NONE |
| Synthetic 2/3/5-pin offline replay | UNRUN | NONE |
| H Host live grant and revocation | UNRUN | NONE |
| H→U typed report and TierFlow publisher | UNRUN | NONE |
| U isolated V8 smoke | LANE-REPORTED: 21 assertions, mocked renderer/validator; not Z2 rerun | NONE |
| Bun typecheck, Windows/Android/PWA | UNRUN | NONE |

## Immutable inspected source blobs
- G1 `hq/version-intelligence/integration/index.mjs` = `0aa6a2723d322af78e80ded34541f9093749c663`; `comparison-report-adapter.mjs` = `8e464df1b46b7e99120769a40b7a509eb42d840d`.
- H `control/hvi-capability.mjs` = `8e6260ca3e925a7171aa600c147b2c98fb626146`; `action-catalog.v1.json` = `770feb4f66497fb8c5d0a9396d1e24fb14162a96`; `test/hvi-capability.test.mjs` = `ceede5e6888762a38b5ba6a7c8ecebc7077aa79a`.
- U `knowledge/hvi-inspector.mjs` = `122105e966ffc4574402b09da62ea669d38d4ca8`; `render/hvi-inspector.mjs` = `a4ecec46c81dab83adfd1882e2939ef9fb007027`; `tests/hvi-inspector.test.mjs` = `100e83cb3a06feaa97ff4407934d9bf1d17407c4`.
- Existing TierFlow `src/ui/commands.ts` = `9ad3c6cf8864dc8f8728ade68088dea3ad406f4e`; `src/ui/store.ts` = `90aeae09c8c765fb385e7eeac5715d3d3577a57a`.
- Beta gate `integration/beta-gates.json` = `81255176eaec6728043777ec7d2c62d41cf16c66`.

## Next executable remote commands — synthetic-only
```sh
node --version
node --test apps/vivim-host/test/hvi-capability.test.mjs
node --experimental-strip-types --test packages/tierflow/tests/hvi-inspector.test.mjs
node --test tests/hq-version-intelligence/*.test.mjs
node --test hq/test/catalogue-delta.test.mjs tests/hq-version-intelligence/*.test.mjs
```
Include strict AJV Draft 2020-12, offline synthetic replay, H live grant/revoke and H→U integration tests. Capture SHA, command, OS/Node version, stdout and literal exit. No actual source-comparison dogfood.

**Gate REPAIR_REQUIRED; all executable test exits UNOBSERVED.**
