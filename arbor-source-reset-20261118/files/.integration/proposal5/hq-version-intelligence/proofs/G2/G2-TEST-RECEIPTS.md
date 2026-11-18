# G2 test execution receipts — synthetic-only / not runtime accepted

**2026-11-12. Status:** `REPAIR_REQUIRED`, `PREPARED_NOT_PROMOTED`. The environment diagnostic exits below are NOT G2 test process exits.

| Command or check | Environment / observation | Actual exit | Acceptance |
|---|---|---:|---|
| `node --version` | Isolated Linux container, `v22.16.0` | **0** | Infrastructure check only; required Node24 unavailable |
| `git ls-remote https://github.com/owenservera/ARBOR.git HEAD` | Isolated container DNS `Could not resolve host: github.com` | **128** | Connectivity check only; no clone/read of repository content |
| GitHub Actions run [43315512755](https://github.com/owenservera/ARBOR/actions/runs/43315512755) on `bc644e2e3314ed80a3f9891695d8ad4798cee6b9` | `startup_failure`, jobs=0 | **null** | Test runner never started |
| Connected GitHub six-source V8 transformed module parsing | Six fetched G2/Host/TierFlow source/test blobs accepted by JS parser, not Node ESM loader | **null** | Static transformation/syntax-only |
| Synthetic injected V8 bridge + TierFlow projection | **12 assertions executed / 12 passed**: grant checks, revocation before return, digest mismatch, redaction, bounds, U display projection, disabled live UI. Noncryptographic test-double digest for deterministic smoke only | **null** | Isolated V8 smoke, NOT Node24, AJV or cryptographic proof |
| `node --test apps/vivim-host/test/hvi-capability.test.mjs` | **UNRUN** (no checked-out ARBOR source/Node24) | **null** | No runtime receipt |
| `node --experimental-strip-types --test packages/tierflow/tests/hvi-inspector.test.mjs` | **UNRUN** | **null** | No runtime receipt |
| `node --test tests/hq-version-intelligence/g2-host-ui.test.mjs` | **UNRUN**, authored 3-SHA + 5 invented-repo tests only | **null** | No runtime receipt |
| Combined G1 `tests/hq-version-intelligence/*.test.mjs`, strict AJV Draft2020-12, offline synthetic CLI replay | **UNRUN** | **null** | No runtime receipt |
| Canonical live Host NL action dispatch, real owner/session/revocable source grant, UiCore revision/CAS | **UNRUN**, disabled for safety | **null** | No live security acceptance |
| Windows, Bun/Cargo, physical PWA/Android, rollback drill | **UNRUN** | **null** | Beta/device scope unchanged |

Source blob identities for isolated V8 checks:
- `hq/version-intelligence/integration/host-inspector-projection.mjs` = `2a2eaf7e9fd23de6f2dc721b24a0ac585464ef00`
- `packages/tierflow/src/visual-runtime/knowledge/hvi-inspector.mjs` = `122105e966ffc4574402b09da62ea669d38d4ca8`
- `apps/vivim-host/control/hvi-capability.mjs` = `e4deec0c3b39e827fc5d519ffdee99337bf16848`
- `packages/tierflow/src/visual-runtime/render/hvi-inspector.mjs` = `afbe2a55e531e4de3379f536f4a0d91fb0e66ab9`
- `tests/hq-version-intelligence/g2-host-ui.test.mjs` = `f0866be4cf1c5ed546f54849dce095db5dbc9998`
- `packages/tierflow/tests/hvi-inspector.test.mjs` = `f8a1707d154bb9d94824e03f5153b1131521ba60`

No actual ARBOR/VIVIM/Labs or donor-repository comparison was performed. Next owner must run checked-out Node24 tests with injected **invented** Git fixtures on a functioning approved runner and record SHA, OS, commands and literal exits. No main merge or readiness gate before independent evidence review.

## 2026-11-12 subsequent security-repair checks and CI diagnosis

- **Committed G2 source fixes:** `77bec293a7f0144d1c0bae01a1102a26bf1a7e9f` (native Git sensitive-path read denial), `eec8a16f00e1e26459c6ad6a7e4c4c9851300d23` (C comparator reuses the same fail-closed classifier).
- **Committed G2 synthetic tests:** `06858bf389338695969cff12e55ea1f1138bb20e`, `33b6059615aab0ec5571a3acf5674725b67fcd9f`. Cases include direct credential-family blob denial, excluded metadata removal, repository authorization/cancellation, and direct C metadata with a recomputed digest. **Authored, Node24 UNRUN, exit=null.**
- **Executed isolated V8 check:** fetched the real `sensitivePath` implementation from `hq/version-intelligence/sources/common.mjs`; 11 synthetic credential paths correctly denied and 3 harmless paths accepted, **14/14 assertions passed**. A separately fetched G2 test file was parsed by V8 without syntax exception. **No OS test process exit, no Node24 module runner, no C comparator execution, no AJV.**
- **Latest GitHub Actions:** [run 43319733234](https://github.com/owenservera/ARBOR/actions/runs/43319733234) on `33b6059615aab0ec5571a3acf5674725b67fcd9f`: `startup_failure`, `jobs=[]`, **no test process exit**.
- **Attempt to retry an earlier failed run:** GitHub API returned **403 FORBIDDEN** with `This workflow run cannot be retried`. Retrying cannot supply a synthetic test receipt in this environment.
- **Tracking issue:** [#24 — Actions startup_failure / Node24 receipts](https://github.com/owenservera/ARBOR/issues/24). Draft integration [PR #21](https://github.com/owenservera/ARBOR/pull/21) still unreviewed and unmerged.

**Gate remains `REPAIR_REQUIRED`.** G1 accepted-main SHA remains null; Z2 independent approval remains absent. New G2 tests are not a substitute for executed Node24+AJV, live Host grants, canonical dispatch or UiCore revision attestation. R prerequisites unmet; no R branch/false certificate created.

- **CI workflow now authored:** `.github/workflows/hvi-native-synthetic.yml` at `d2411bae45964f533b5c48f0faac727c31140932`, Node24, AJV 8.17.1, explicit synthetic G1/G2/Host/TierFlow tests, read-only checkout/no credentials. Latest [run 43320303173](https://github.com/owenservera/ARBOR/actions/runs/43320303173): `startup_failure`, **0 jobs**, **exit=null**. A workflow file is not an executed test receipt.
