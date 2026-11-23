# VLANG / ZCode operations — one live GPUI/PWA command system

The single source of truth for the design and real execution receipt is
[ARBOR-VLANG-UPGRADE/ZCODE-PARALLEL-CYCLE-R001-20261123.md](../ARBOR-VLANG-UPGRADE/ZCODE-PARALLEL-CYCLE-R001-20261123.md).

**Source workflow:** ARBOR `.zcode/workflows/arbor-vlang-iterative-observers.dwf.ts` at `f79788da`.
**Local observer:** `scripts/vlang-parallel-cycle.ts`, Node 24+ TypeScript.
**Task control:** `scripts/vlang-schedule.ps1`, Windows task `VIVIM-VLANG-Observer`.

Four disjoint lanes: Z-VLANG, Z-SAFETY, Z-AUTOWIKI, Z-VISUAL. Tests use existing source/runtime, not a second grammar. One independent agent verifier rejects unsupported proposed improvements.

### Execute / inspect / stop

```powershell
$ops='C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS'
node --experimental-strip-types "$ops\scripts\vlang-parallel-cycle.ts" --once
powershell -NoProfile -ExecutionPolicy Bypass -File "$ops\scripts\vlang-schedule.ps1" -Action status
Get-Content "$ops\local-logs\vlang-iterations\latest.json"
# Stop automatic recurring test observations:
powershell -NoProfile -ExecutionPolicy Bypass -File "$ops\scripts\vlang-schedule.ps1" -Action remove
```

Initial source-pinned run `d19e388c`: 2 pass, 2 fail due to outdated test assumptions; exact regression tests adjusted on canonical ARBOR. Final source-pinned run `f79788da`: 215/215 across the four parallel lanes.

**Agent workflow:** Run in ZCode with `--mode plan --workflow-mode alwaysOn`, after loading official dynamic-workflows skill. Execute `CreateWorkflow` by **existing** saved project path with full current `sha` argument; require a genuine accepted workflow run ID and independent verifier output. Only source findings; no browser sends, Host effects, local product source patches, repeated 429, or paid model fallback.

**Model readiness** is not implied by the installed task. Source checks continue independently; dynamic model jobs require a proven quota and a genuinely accepted ZCode workflow run. All detailed logs and run records remain local in `local-logs/` (ignored by git); do not commit private prompts, browser titles or personal corpus.
