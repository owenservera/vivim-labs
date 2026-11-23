# VLANG / ZCode operations — one live GPUI/PWA command system

The single source of truth for the design and real execution receipt is
[ARBOR-VLANG-UPGRADE/ZCODE-PARALLEL-CYCLE-R001-20261123.md](../ARBOR-VLANG-UPGRADE/ZCODE-PARALLEL-CYCLE-R001-20261123.md).

**Source workflow:** ARBOR `.zcode/workflows/arbor-vlang-iterative-observers.dwf.ts` at `f79788da`.
**Local observer:** `scripts/vlang-parallel-cycle.ts`, Node 24+ TypeScript.
**Task control:** `scripts/vlang-schedule.ps1`, Windows task `VIVIM-VLANG-Observer`. **Model circuit breaker:** `scripts/vlang-zcode-pilot.ps1`; each SHA gets at most one admitted ZCode pilot, and a 429/timeout stops its scoped process tree.

Four disjoint lanes: Z-VLANG, Z-SAFETY, Z-AUTOWIKI, Z-VISUAL. Tests use existing source/runtime, not a second grammar. One independent agent verifier rejects unsupported proposed improvements.

**2026-11-23 reliability update:** The four *independent test lanes* are now executed **sequentially on this Windows verifier** (`maxConcurrentLanes=1`; Labs `78472401`), because 4-lane and 2-lane simultaneous runs caused transient 1200ms native-child timing failures although each implicated test file passed in isolation. The fully serialized exact-SHA `0cd77a21` run returned **215/215 PASS**, four green lanes, Rust SHA256-verified, zero Host/provider/source effects; [receipt and lineage](DEVOPS-PLANNING/PACKS/EXECUTION-20261123-R001/VERIFICATION-RECEIPT.md). This does **not** disable separate read-only agent fanout or prove model availability/phone rendering.

### Execute / inspect / stop

```powershell
$ops='C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS'
node --experimental-strip-types "$ops\scripts\vlang-parallel-cycle.ts" --once
powershell -NoProfile -ExecutionPolicy Bypass -File "$ops\scripts\vlang-schedule.ps1" -Action status
powershell -NoProfile -ExecutionPolicy Bypass -File "$ops\scripts\vlang-zcode-pilot.ps1" -Action status
Get-Content "$ops\local-logs\vlang-iterations\latest.json"
# Stop automatic recurring test observations:
powershell -NoProfile -ExecutionPolicy Bypass -File "$ops\scripts\vlang-schedule.ps1" -Action remove
```

Initial source-pinned run `d19e388c`: 2 pass, 2 fail due to outdated test assumptions; exact regression tests adjusted on canonical ARBOR. Final source-pinned run `f79788da`: 215/215 across the four parallel lanes.

**Agent workflow:** Run in ZCode with `--mode plan --workflow-mode alwaysOn`, after loading official dynamic-workflows skill. Execute `CreateWorkflow` by **existing** saved project path with full current `sha` argument; require a genuine accepted workflow run ID and independent verifier output. Only source findings; no browser sends, Host effects, local product source patches, repeated 429, or paid model fallback.

**Model readiness** is not implied by the installed task. Source checks continue independently; dynamic model jobs require a proven quota and a genuinely accepted ZCode workflow run. All detailed logs and run records remain local in `local-logs/` (ignored by git); do not commit private prompts, browser titles or personal corpus.
