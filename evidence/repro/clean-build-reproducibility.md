# Labs evidence — clean-checkout build reproducibility (TierFlow)

**Record:** 2026-11-06 · **Factory:** 06 Production Scaffold Factory / test method 5 (clean-checkout reproducibility)
**Status:** MEASURED LOCALLY — Labs-owned evidence. Not device, Host, browser or production evidence.

## Source pin

| Item | Value |
|---|---|
| Repository | `owenservera/tierflow` |
| Local checkout used | `C:\0-BlackBoxProject-0\_scratch_l3\clean-build-repro\tierflow-clean` (**isolated**) |
| Clone method | `git clone --no-local --depth 1 --branch main file:///C:/0-BlackBoxProject-0/vivim-ui` |
| Source SHA | `ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7` |
| Baseline checkout SHA (for comparison) | `C:\0-BlackBoxProject-0\vivim-ui` @ `ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7` (identical) |
| Machine | 8 logical cores · 21.90 GiB RAM · C: 31.62 GiB free |
| Toolchain | bun 1.4.2 · node v24.22.1 · TypeScript 7.0.2 · cargo 1.99.1 · git 2.51.2.windows.1 |

The isolated checkout started with **no `node_modules/`** and **no `core/target/release/tierflow-core.exe`** —
verified before running anything. This is the discriminating condition: the documented `ts-fallback`
failure mode only appears when the Rust sidecar is absent.

## Commands, exit codes, results (all run in the isolated checkout)

| # | Command | Exit | Result |
|---|---|---|---|
| 1 | `bun install` | 0 | `Checked 9 installs across 34 packages (no changes)` — 112 ms |
| 2 | `bun run core:build` | 0 | Rust sidecar built from scratch, `Finished release profile in 13.19s`, 2 `dead_code` warnings only; `core/target/release/tierflow-core.exe` produced (256,512 bytes) |
| 3 | `bun run typecheck` | 0 | `tsc -p tsconfig.json` — 0 errors |
| 4 | `bun run core:test` | 0 | `10 passed; 0 failed; 1 ignored; 0 measured` |
| 5 | `bun run verify` | 0 | typecheck → core:test → Bun suite: **449 pass / 0 fail**, 1120 expect() calls, 35 files, 8.84 s |
| 6 | `bun test tests/ui-commands.test.ts tests/ui-store.test.ts` | 0 | **52 pass / 0 fail**, 250 expect() calls (Lane C `bun:sqlite`) |
| 7 | `bun test tests/visual-runtime/integration-2/real-a-h.test.ts` | 0 | **25 pass / 0 fail** (real A–H composition + adversarial negatives) |

## Reproducibility verdict

The TierFlow baseline at `ec832f8` reproduces **exactly** from a clean, isolated checkout:
449/449 Bun tests, 10/10 Rust tests (1 ignored), 52/52 SQLite, 25/25 real A–H — identical to the
in-place working-tree run measured in the readiness handoff. The `ts-fallback` failure mode did
**not** appear, because the sidecar was built before the suite ran.

**Build time from scratch:** 13.19 s for the Rust sidecar; total clean-checkout wall time well under
one minute for the full gate chain.

**Depth gained:** this is a *depth* result, not breadth — one canonical repo (TierFlow) proven to
build and verify identically from zero, on this machine, with no cache, no sidecar and no
machine-local config. It does **not** prove:
- reproducibility of the other four repos (repos_hq, vivim-web-harness, vivim-labs, vivim-workstation) — not yet run;
- reproducibility on a second machine or a clean Windows image;
- native Tauri / desktop / Android / browser / live-Host / live-KB behaviour — none were exercised;
- production-release readiness (no installer, channel or cutover was attempted).

## Reproduce

```sh
mkdir -p C:/0-BlackBoxProject-0/_scratch_l3/clean-build-repro
cd C:/0-BlackBoxProject-0/_scratch_l3/clean-build-repro
git clone --no-local --depth 1 --branch main file:///C:/0-BlackBoxProject-0/vivim-ui tierflow-clean
cd tierflow-clean
bun install && bun run core:build && bun run typecheck && bun run core:test && bun run verify
bun test tests/ui-commands.test.ts tests/ui-store.test.ts
bun test tests/visual-runtime/integration-2/real-a-h.test.ts
```

**Maturity: integrated** (real neighbouring modules interoperate; a clean environment was
actually executed). **Not** environment-verified for device/browser, and **not** production-verified.
