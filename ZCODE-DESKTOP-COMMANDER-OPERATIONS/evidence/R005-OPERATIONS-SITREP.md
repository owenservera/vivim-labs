# R005 Operations-mode baseline — 2026-11-21

Operator: ChatGPT via authorized Desktop Commander on DESKTOP-9KB03LU. Model/provider engineering is owned by a separate ChatGPT session; no model changes were made during this operations baseline.

## Exact command and exit
```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS\CONTINUOUS-IMPROVEMENT-FLEET\scripts\sitrep-r004.ps1" -Deep -V4 -Team -BaselineTests
```
Remote process PID 16764, script exit 0, `R004_SITREP_COMPLETE`, measured runtime 32.36s.

## Observed actual state
- ZCode CLI 0.18.0; SQLite read-only 3,353 stored sessions, 2,721 child sessions, 1,495 historical task links; 0 workflow run records; 15 saved workflow files.
- ZCode native protocol queries 3/3+ returned: sessions (5), saved workflows (15), subagent projection (1 historical child result), plugin list (67); protocol probe exit 0 with zero failed requests.
- V4 read-only sessions-index subscription: ACK snapshot PASS, one frame received, zero errors; exit 0. Initial snapshot only; no live delta/reconnect proof.
- HQ team JSON command PASS: 93 agent definitions, 4 seats, 1 task, 157 defects, 131 next moves. Roster snapshot is NOT live authorized fanout.
- ARBOR local HEAD `9889082`; four tracked modified entries, untouched by this round. Do not assume local tree is synchronized with remote main.
- Rust fleet-validator PASS: 10 lanes, 4 dependency waves, `auto_launch=false`; no DWF workflow launched, no agent child spawned.
- Focused actual Host baseline tests: **15 passed / 0 failed**, runtime 1684ms. Includes real NL command intents, inert AI proposal, VEC typed plan refusal, Host HTTP VEC approval/wiki readback/restart, stale N→N+1 Wiki projection and structural fallback. Native Rust parity checks that explicitly reported no packaged binary remain **UNRUN**, not PASS.
- Existing ZCode model fanout remains blocked/partial, handed to model owner; not addressed by operations.

## Operator next execution
Highest-value no-model operations gates: source-grounded V4 revision/stream integrity; Host/GPUI/core native regression checks with real binaries where available; workflow and HQ delivery only after explicit grants/seat/budget proof. Do not start recurring scheduling, alter account credentials, or launch pre-existing ARBOR production workflows. For actual ARBOR edits, inspect active ownership and preserve dirty state/worktrees.

This report is independent of the separate model-owner handoff at `evidence/R005-OPERATIONS-HANDOFF.md`.
