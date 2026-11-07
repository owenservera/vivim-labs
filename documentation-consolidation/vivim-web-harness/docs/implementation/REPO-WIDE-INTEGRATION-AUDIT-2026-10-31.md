# VIVIM repo-wide integration checkpoint — 2026-10-31

**Scope:** GitHub `main` at `76a4b672` and subsequent targeted repair `2437f834`. Source inspection + isolated unit tests, NOT an end-to-end live acceptance certificate.

## Observed checks

- Queried the GitHub `main` tree (1,246 entries at the initial head), recent commits, Linux Grokbot Direct runner, Host bootstrap/Direct routes, Studio/relay and PWA entry points.
- Ran original committed new-engine/GUI tests from the available isolated source package on Node 22.16: **162 passed, 0 failed**.
- Corrected the Node-only browser imports; reran the expanded isolated unit suite including five new portable-digest regression tests: **167 passed, 0 failed**.
- Observed `git ls-remote` / clone is unavailable in the execution container (`Could not resolve host: github.com`). The **existing original repo-wide Host/mobile/harness/Studio suites were NOT executed** here; do not label them passing.
- No live Chrome owner, real signed-in AI webapp, Linux native Grokbot, cloud deployment or paired Android was used.

## Findings

| Priority | Finding | Status |
| --- | --- | --- |
| P0 | GUI `compiler.mjs` and `semantic-projection.mjs` imported `node:crypto`; semantic layer used `Buffer` although the no-build mobile renderer needs portable source | **Fixed:** shared dependency-free SHA-256/UTF-8 implementation, digest equivalence regression tests |
| P0 | Direct Host's closed asset map lacked `generative-ui/*` paths | **Fixed:** exactly six explicit static module mappings and an assertion for each in original Direct-phone tests (original Host suite execution pending) |
| P1 | Root `npm test` omits Host, mobile and newly added core/GUI tests | **Resolved:** repo-root `test/run.mjs` (`npm run test:fast`) performs git-diff affected selection across Host/mobile/harness/Studio/mobile-relay suites with Bun parallel execution and content-hash caching; `npm run test:all` runs all suites |
| P0 | New engine and GUI modules have no actual Host/Studio or Android Build runtime entrypoint wiring | **Pending:** GUI04 owner-authorized Host revision/CAS/Undo, GUI05 shared renderer mount, GUI06 authenticated Direct projection and verified Studio approval seam |
| P0 | Grokbot Linux runner requires an explicitly configured, verified native adapter; available repo source does not establish a real adapter execution | **Pending:** preserve fail-closed preflight; no native invocations or retry of uncertain claimed jobs |
| P1 | Integration data-root, worker/process leases, Node SQLite, original Studio baseline, real-browser sends and phone repaint not proven by synthetic unit suites | **Pending:** verify on the owner's Linux/Windows environment with exact owner, process, journal and Android receipts |

## Run on the actual checkout

```bash
npm run check:integration
npm run verify:integration
```

The integration runner is local-only: static import graph, browser-global check, Direct asset check, all test groups and existing docs checker. It never launches provider processes, opens Chrome or issues a model send. Its exit status and per-suite failures should be captured on the actual repository checkout. For existing Studio baseline failures, keep baseline attribution separate from regression triage.

**Live acceptance chain remains unverified:** paired Android → confirmed Studio-owned approved F02/F01 send → Host typed ledger → shared Build/Bootstrap renderer → revision approval → Undo → reconnect, with no uncertain resend or private D06 projection.

Only `docs/roadmap.json` is authorized to track formal milestone status; no completion status is changed by this checkpoint.
