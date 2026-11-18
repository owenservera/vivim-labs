# VEC PWA D0 — Remote source proof and execution truth

**Observed:** 2026-11-13 (Europe/Madrid). **Branch:** `work/p5-vec-pwa-ui-kernel-r001`. **Source head reviewed:** `4106da40146cbc107e334ee067b0db2c619f14d9`. **Base main:** `9b79d732da8b92713d399040db4a02d505bc7cf1`. **Scope:** initial PWA deterministic compiler; **NOT** the HQ Wiki or Host command compiler cutover.

## Remote static checks actually executed
Using GitHub source fetch and direct structural assertions (these are NOT Node/cargo tests):
- `apps/mobile-relay/vec/compiler.mjs` declares three isolated domains `wiki`, `command`, `ui`; default output explicitly `authorization:none`, `executed:false`, `persisted:false`.
- `vec/ui-adapter.mjs` uses existing `compileUiProposal` and never invokes provider or Host.
- `build-composer.mjs` exposes deterministic `resolveBuildRequest`.
- `build-canvas.mjs` branches on `candidate` before the existing model `send`, and retains `apply(proposal,surface)` on Activate.
- `apps/vivim-host/direct-phone.mjs` static read-only asset allowlist includes both new VEC modules.
- `apps/mobile-relay/sw.js` shell cache v5 includes both new VEC modules; the cache does not gain API or personal data paths.
- Machine `workstreams.json` and `source-pins.json` parse as JSON, six workstreams defined.
- The additional compound-phrase guard was committed after those structural assertions; current source must be reread for exact final source verification.

**Static checks do not establish that tests execute, modules import on Android Chrome, Host actually accepts the proposal or the paired device works.**

## Actual automation outcome, not green
GitHub Actions run [43327237673](https://github.com/owenservera/ARBOR/actions/runs/43327237673) for this PR source lineage returned `completed / startup_failure`, with `0` jobs. Earlier branch pushes showed the same startup failure. **This is a CI infrastructure/activation blocker; no test process was launched and no executable exit code exists.** Do not classify as tests passed, red failing assertion, or device result.

## Tests authored but UNRUN
- `node --test apps/mobile-relay/test/vec-ui-compiler.test.mjs`
- `node --test apps/mobile-relay/test/build-vec-routing.test.mjs`
- `node --test apps/mobile-relay/test/build-composer.test.mjs apps/mobile-relay/test/generative-ui.test.mjs apps/mobile-relay/test/service-worker.test.mjs`
- `node --test apps/mobile-relay/test/*.test.mjs`
- Node24 source syntax, Host Direct static-import closure, Windows Host, actual paired PWA Chrome and privacy/security tests: **UNRUN**.

## Gate
`D0_SOURCE_PREPARED` only. `D1_EXECUTABLE_ACCEPTANCE=BLOCKED_CI_STARTUP`. `WIKI_CUTOVER=NO`, `COMMAND_CUTOVER=NO`, `TIERFLOW_DYNAMIC_CUTOVER=NO`, `PRODUCT_GO=NO_CLAIM`.

Next owner action: execute D1 test matrix from `03-PAIRED-PWA-ACCEPTANCE.md` on Node24 and a real pair; fix any failures on the isolated branch. Re-pin latest HEAD before testing, and record exact commands/exits.
