# Local Desktop Commander stream deployment — 2026-11-22

## Scope
On-device archive and metadata-only learning index for the owner's existing Desktop Commander stream on `DESKTOP-9KB03LU`. This is **not** an ARBOR Host journal, model training corpus, or a replacement application source authority.

## Verified local recovery and archive
- Desktop Commander Windows shell command preflight recovered: a harmless `Write-Output DC_SHELL_READY` completed exit 0. Unrelated shell-profile PSReadLine warning persists; use `pwsh -NoProfile` inside runners.
- Private initial backup created successfully under `C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS\.private\desktop-commander-stream\raw\2026-11-22`.
- Initial backup contained **two files, 4,599,238 bytes** total; `tool-history.jsonl` 2,768,972 bytes, `claude_tool_call.log` 1,830,266 bytes. Both hashes and the source-path manifest are kept in the same private directory.
- Source history first indexed at **2,453 records**, **0 malformed**, **331 large outputs omitted upstream**; process exited 0, receipt `capture-20261122T1716577454428Z`.
- Second capture was incremental: **2,524 observed, 71 new, 2,453 duplicate, 0 malformed, 6 additional omitted outputs**; cycle exit 0 at 2026-11-22 17:19 UTC.
- Argument capture initially emitted a false rollover and duplicated the active file because PowerShell `ConvertFrom-Json` materialized an ISO timestamp into a date object. Fix committed `01b944d4`, deployed locally. Follow-up direct run captured **16,066 new bytes**, **no rollover warning**, exit 0.
- Latest on-PC aggregate in this audit: **2,723 indexed records**, **345 upstream large-output omission markers**, **41 indexed records with nonzero process exits**, **290 with exit 0**, across 2 indexed day files. A process output line is not a unique job; repeated reads can repeat an outcome. No raw command arguments or content were uploaded to GitHub.

## Top observed tool categories in indexed records
| Tool | Indexed events |
|---|---:|
| read_process_output | 1,578 |
| start_process | 497 |
| read_file | 254 |
| list_directory | 129 |
| write_file | 99 |
| edit_block | 63 |
| read_multiple_files | 37 |
| get_file_info | 18 |
| list_sessions | 12 |
| list_processes | 10 |

## Local task deployment
- Windows Scheduled Task `VIVIM-DesktopCommander-Archive` was **created** with a **two-minute** trigger, existing interactive user principal, limited run level, no privilege escalation, no model calls and no network effect.
- Task runs `pwsh.exe -NoProfile -NonInteractive` against the private copy of `archive-desktop-commander-cycle.ps1`, which holds a named mutex, calls the two existing Labs archivers and records local receipts.
- Manually invoking the cycle script completed `CAPTURE_CYCLE=SUCCESS`, `CYCLE_EXIT=0`; the latest verified manual receipt finished 2026-11-22 17:19:04 UTC.
- Initial scheduled Task Scheduler states included Running (`0x00041301`) and one overlap refusal (`0x800710E0`); these are retained as diagnostic observations, not asserted task failures.
- **Scheduled execution VERIFIED:** at **2026-11-22 17:26:29 UTC**, Task Scheduler started the job. The local `state/cycle-entered.json` was written at 17:26:30 UTC and `state/cycle-latest.json` reported `SUCCESS` at **17:26:34.4089822 UTC**. A later independent query returned `Task Status=Ready`, **`LastTaskResult=0x00000000`**, next scheduled run **17:28:28 UTC**. The unique record index grew to **2,759 hashes**. This proves an unattended scheduled tick, not just manual invocation.
- Scheduler diagnostic beacons for future invocations were added to the runner in GitHub commit `cdbd1903` and copied locally.
- Estimated local private archive size at one point was **13,978,863 bytes** across **51 files**; inspect disk usage and avoid copying the entire bounded source on every cycle.

## Evidence quality and remaining gates
- Raw logs are confined to the existing Git-ignored `.private` tree; logs may contain tokens, paths, personal data, and commands. Never publish them or infer permission for training.
- Upstream history is trimmed; outputs over 4 KiB are replaced with omission markers. This archive preserves what remains and records missing evidence explicitly, not a reconstructed lossless historical stream.
- The scheduled task has one verified successful automatic tick. Continue checking receipt freshness and task last result; an unattended success today is not proof of indefinite uptime, historical completeness, or future operation while the user is logged out.
- A separate explicit output-tee at the original test runner would be required for full-fidelity stdout/stderr beyond Desktop Commander's omission boundary.
- Keep the ARBOR product task ledger authoritative. Improvements derived from this index must be reproduced against exact GitHub SHAs and validated separately on GPUI, PWA and physical phone.

## GitHub source used
- `scripts/archive-desktop-commander-history.ps1`: incremental JSONL capture, metadata index, receipts.
- `scripts/archive-desktop-commander-arguments.ps1`: argument-log byte deltas and rotated logs.
- `scripts/archive-desktop-commander-cycle.ps1`: guarded single-cycle runner and scheduler beacons.
- `STREAM-ARCHIVE/LESSONS-20261122.md`: initial seven reproducible operational lessons.

This is a **local observability** workstream; no ARBOR application source was edited or committed from Desktop Commander.


## R008 continuation — live verification after 17:46 UTC

- **Archive remains continuous:** at 2026-11-22 17:46:32 UTC, two-minute scheduled capture was Ready, result `0x00000000`, index had **2,819 unique rows**; 17,638,782 bytes private archive across 92 files; C: had 32.63 GB free. The collector was still running after the initial 17:26 proof.
- **Separate full-output acceptance wrapper:** committed `50c3e757` and installed under private `scripts/capture-validation-output.ps1`. A real read-only ZCode `sitrep.ps1` execution finished exit 0, SHA-256-stamped private stdout (785 bytes) and stderr (0 bytes), with receipt under `validation-runs/20261122T1749349111290Z-ZCODE-READONLY-SITREP-e4ac12cd/`. The wrapper uses raw stream copies but a live >4 KiB test has NOT been performed; product acceptance remains NOT_DETERMINED.
- **Learning report hardening:** commit `9c9034e2`; private `learning-latest.{json,md}` stable handles, daily UTC coverage and delta since previous report. Direct PC run exited 0 with 2,847 index rows / 351 upstream omissions / 43 nonzero-exit observations. A nonzero exit observation is not a unique failed job.
- **Daily review is scheduled locally:** `VIVIM-DesktopCommander-DailyLearning` runs at 19:05 PC local time under limited interactive user, no network or provider calls. Manually triggered via the actual Scheduler; at 17:52:12 UTC Windows returned final result **`0x00000000`**, and `learning-latest.json` was generated at 17:52:19 UTC. This proves Scheduled Task execution, not indefinite future uptime.
- **Read-only health guard:** committed `d4426768`. A real PC execution at 17:53:23 UTC returned `HEALTH_RESULT=PASS`, zero failures/warnings and exit 0. Both tasks were Ready and result `0x00000000`; collector receipt freshness 0.85 minutes; daily learning age 0.02h; unique archive hashes 2,867; free C: reported 35.77 GB at that later instant.
- **Privacy and authority:** local raw outputs and arguments in the existing Git-ignored `.private` tree only. Cloud GitHub has scripts and reviewed counts, no private logs. No ARBOR product source changes or competing task tracker. These are PC observability successes, not product behavior validations.

To inspect anytime from the authorized PC, run the GitHub-versioned `scripts/desktop-commander-health.ps1` with explicit `-ArchiveRoot` pointing into the private local archive. Read `reports/learning-latest.md` for aggregate lessons and the existing Labs `TEST-LEDGER.md` for acceptance status.
