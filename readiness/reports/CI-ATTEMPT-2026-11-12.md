# First-pass readiness execution evidence — 2026-11-12

## Completed: actual GitHub tracked-source scan

The source audit fetched three immutable tree revisions using connected GitHub access and classified paths and source file counts. This is a real repository metadata run, not a synthetic inventory.

- Labs main: 022608f5797e7b4a14ccc4c393bd3c5a4704a4be — 885 tracked blobs.
- Labs Node 0 branch: 61d9fc15c55144c4f0b79afa248981c2b0ca9bd6 — 902 tracked blobs.
- Labs Composer branch: 56bb145ea1eb479e4d8ec629163a84322797367a — 888 tracked blobs.
- 44 mapped components: 44 source-path matches, no missing exact paths at these refs.
- Source-verified status: SOURCE_TREE_PRESENT only. Not a compilation or runtime product GO.

Details: [human report](FIRST-PASS-2026-11-12.md) and [snapshot JSON](FIRST-PASS-2026-11-12.json).

## Attempted: GitHub Actions Node first-pass and isolated test suites

Workflow committed: .github/workflows/p5-labs-readiness.yml.

[GitHub Actions run 43150144128](https://github.com/owenservera/vivim-labs/actions/runs/43150144128) triggered for commit 3e3a7c42f18ca06c690fe755c0a3cba1e7b79a84.

**Observed result:** status COMPLETED, conclusion STARTUP_FAILURE, zero jobs, no scanner or Node-suite output. The reason is not established by the exposed workflow-run data; do not attribute it to a code/test failure or a specific account setting without evidence.

Therefore all newly automated suite statuses remain UNRUN, including scanner unit tests, T=0 archive tests, Capability Observatory unit tests, Voice workbench, Chronos JS and R005 UI tests/bench. Prior Labs source-authored test claims are historical evidence, not refreshed receipts.

## Exact path to a proper test receipt

1. Resolve the workflow startup issue or run commands in readiness/README.md in a real Node 24+ Git checkout with pinned commit objects present.
2. Save command, exit code, environment, test count, captured output and exact source SHA.
3. Upgrade readiness tests only from fresh evidence; never promote tracked path presence to product integration.
4. Native Cargo, Host/World, Windows installer, signed-in Chrome and physical PWA remain separate acceptance gates.

## Data safety

This run inspected Git tree paths and selected source/test documentation, not user data, browser sessions, actual ChatGPT archives, credentials, private local files or provider websites. It did not migrate Labs code to ARBOR or modify current migration workers.
