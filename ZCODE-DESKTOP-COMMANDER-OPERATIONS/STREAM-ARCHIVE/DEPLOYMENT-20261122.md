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
- The Scheduled Task reported Running (`0x00041301`) after being triggered; an overlap attempt was recorded as `0x800710E0`. **Do not claim the schedule accepted a completed run until a newer cycle receipt and a successful task result are both observed.**
- Scheduler diagnostic beacons for future invocations were added to the runner in GitHub commit `cdbd1903` and copied locally.
- Estimated local private archive size at one point was **13,978,863 bytes** across **51 files**; inspect disk usage and avoid copying the entire bounded source on every cycle.

## Evidence quality and remaining gates
- Raw logs are confined to the existing Git-ignored `.private` tree; logs may contain tokens, paths, personal data, and commands. Never publish them or infer permission for training.
- Upstream history is trimmed; outputs over 4 KiB are replaced with omission markers. This archive preserves what remains and records missing evidence explicitly, not a reconstructed lossless historical stream.
- The scheduled-task execution path is not yet fully accepted; inspect `state/cycle-entered.json`, `state/cycle-latest.json`, and private scheduler errors. A created/Running task is not a verified successful periodic capture.
- A separate explicit output-tee at the original test runner would be required for full-fidelity stdout/stderr beyond Desktop Commander's omission boundary.
- Keep the ARBOR product task ledger authoritative. Improvements derived from this index must be reproduced against exact GitHub SHAs and validated separately on GPUI, PWA and physical phone.

## GitHub source used
- `scripts/archive-desktop-commander-history.ps1`: incremental JSONL capture, metadata index, receipts.
- `scripts/archive-desktop-commander-arguments.ps1`: argument-log byte deltas and rotated logs.
- `scripts/archive-desktop-commander-cycle.ps1`: guarded single-cycle runner and scheduler beacons.
- `STREAM-ARCHIVE/LESSONS-20261122.md`: initial seven reproducible operational lessons.

This is a **local observability** workstream; no ARBOR application source was edited or committed from Desktop Commander.
