# G1 test truth — executable acceptance is UNRUN

Candidate source SHA: 0dfb08406c50df2e385c0ff924694e0aeaa33565
Repository: owenservera/ARBOR
Integration branch: integrate/p5-hq-version-intelligence-r001
Synthetic test source blob: e3ae47ce12b8ef4e7bc471d2a26e5573ac4a2a7b
AJV 2020-12 test source blob: ed11580d18bf0d32c784f94b891e5c6445188f6a
Workflow blob: f42c72757e2e2cca4c91777f153e7812b1707c66

## Observed evidence and exits

1. The available isolated container was Linux x86_64 with Node v22.16.0; node --version executed successfully (exit 0). There is no Node24 binary, no ARBOR checkout and no external GitHub network access in that container. Node22 environment information is NOT integrated test acceptance.
2. GitHub Actions push run 43303858096, SHA 0d2a5eea8ad89be9719f5b9b36e6f8e41a102b3e: conclusion startup_failure; jobs=0. URL: https://github.com/owenservera/ARBOR/actions/runs/43303858096. No command or test exit exists.
3. Connected GitHub static source import inspection at candidate code SHA: 13 modules inspected; unresolved relative imports=0. This is NOT executable module resolution.
4. Local Code Mode V8 transformed syntax parser (ESM import/export stripped, Node shebang omitted) accepted six G1 source/test modules plus the CLI. This is NOT an actual node --check invocation, not Node24, and NOT runtime acceptance.
5. All five G1 schema JSON sources were read from GitHub and JSON.parse returned successfully; Draft 2020-12 standards-compliant AJV validation UNRUN.
6. Existing isolated worker receipts include C Node22 9/9 and E Node22 13/13 reported by Z1X, but no G1 cross-lane test exit is inherited from them.

## Required command matrix — deliberately UNRUN, exit=null

- node --version on Node24 integrated checkout: UNRUN; exit=null.
- node --check hq/version-intelligence/sources/snapshots.mjs: UNRUN; exit=null.
- node --check hq/version-intelligence/compare/index.mjs: UNRUN; exit=null.
- node --check hq/version-intelligence/parity/atlas.mjs: UNRUN; exit=null.
- node --check hq/version-intelligence/report/index.mjs: UNRUN; exit=null.
- node --check hq/version-intelligence/integration/index.mjs: UNRUN; exit=null.
- node --test hq/version-intelligence/sources/snapshots.test.mjs hq/version-intelligence/identity/test/identity.test.mjs hq/catalogue-identity/test/crosswalk.test.mjs hq/version-intelligence/compare/index.test.mjs hq/version-intelligence/parity/atlas.test.mjs hq/version-intelligence/report/test/report.test.mjs tests/hq-version-intelligence/*.test.mjs: UNRUN; exit=null.
- node --test hq/test/catalogue-delta.test.mjs: UNRUN; exit=null.
- Real AJV 8.17.1 strict Draft 2020-12 schema negative suite: UNRUN; exit=null.
- Integrated 3-revision, 2/5 invented repository, T01-T24 negative, no-donor offline replay: test source committed, UNRUN; exit=null.
- Windows, PWA/Android, Host grant, device, native binary and beta gate tests: UNRUN; exit=null.

## Explicit next run requirement

Use an actual checked-out integration HEAD and Node24, install the isolated tests/hq-version-intelligence AJV 8.17.1 package from its package.json, run the committed workflow commands and collect real stdout/stderr/exits. Do not infer a pass from startup_failure, from static code compilation, from a test file existing, or from synthetic fixtures authored but not executed. No acceptance merge to main until test exits and review pass.
