# Desktop Commander stream archive — R001

Status: **live on Windows, 2026-11-22**. Initial private backup, incremental capture, and one Windows scheduled cycle were verified. Last observed scheduled run at 17:26:29 UTC returned Task Scheduler result `0x00000000` and an on-PC `SUCCESS` receipt. Continued availability still requires monitoring.

## Purpose
Turn Desktop Commander activity into a durable, local-only chronological record for debugging, failure clustering, task receipts, and lessons learned. Keep raw evidence private on the PC; only reviewed, redacted lessons or receipts belong in GitHub. This does not create a second ARBOR Host journal, browser recorder, process supervisor, or agent that writes product source.

## Actual source / known limitations (22 Nov 2026)
- Live device: DESKTOP-9KB03LU, Desktop Commander 0.3.4.
- Desktop Commander reports 3,805 total tool calls, 3,786 successful and 19 failed, 5 sessions; 1,000 recent calls in in-memory history.
- Its `get_recent_tool_calls` reads local `tool-history.jsonl`, with history trimming and outputs larger than 4 KiB omitted. These are **not full-fidelity stdout/stderr or all past sessions**.
- The upstream Desktop Commander implementation also records raw input arguments in `%USERPROFILE%\\.claude-server-commander\\claude_tool_call.log`, rotating near 10 MiB into timestamped files. This covers commands/paths but **not tool outputs** and may contain secrets. Its historical rotated files should be preserved privately. <https://github.com/wonderwhy-er/DesktopCommanderMCP>
- A tool-call success is not a product/test success. Multiple recent PWA tests failed despite successful tool invocations.
- Continuous **two-minute on-PC collection is verified** by Windows Task Scheduler and a recent `SUCCESS` receipt; continued uptime should be checked. This is **not lossless historical recovery**: upstream discarded records and oversized outputs remain unavailable.

## Verified local execution (22 Nov 2026)

The private runtime archive is located at `C:\\0-BlackBoxProject-0\\vivim-labs\\ZCODE-DESKTOP-COMMANDER-OPERATIONS\\.private\\desktop-commander-stream`; it is explicitly excluded from Git. The same archiver scripts are versioned in Labs and their copies reside in this private directory's `scripts/` folder.

Windows Task Scheduler task: `VIVIM-DesktopCommander-Archive`, every **2 minutes** while the interactive user session is available, limited privileges. Its last independently confirmed scheduled cycle succeeded at 17:26:34 UTC; `state/cycle-latest.json` is the truth source for finished work. No provider calls, browser recording, extra ARBOR Host or product source edits.

To inspect without modifying anything: `Get-ScheduledTaskInfo -TaskName 'VIVIM-DesktopCommander-Archive'` and read `state/cycle-latest.json`. If a receipt is stale, inspect `state/cycle-entered.json` and `state/cycle-last-error.json` before restarting anything.

## Local-only data layout

```text
ZCODE-DESKTOP-COMMANDER-OPERATIONS/
  STREAM-ARCHIVE/               # Versioned design and reviewed SITREPs only
    README.md
    SITREP-20261122.md
  .private/desktop-commander-stream/ # LIVE LOCAL PRIVATE: excluded from Git
    raw/YYYY-MM-DD/             # Incremental unique JSONL raw segments; sensitive
    raw/arguments/              # Argument-log byte deltas + rotated logs; sensitive
    index/days/YYYY-MM-DD.jsonl # One row per observed tool call
    index/tools/TOOL/YYYY-MM-DD.jsonl
    index/seen.sha256           # Deduplication hashes
    receipts/capture-*.json    # Observed capture runs and evidence gaps
    state/cycle-latest.json     # Success receipt used by health check
    reports/learning-latest.md # Daily aggregate; NO raw command text
    reports/learning-latest.json
    validation-runs/RUN_ID/     # Full stdout.bin/stderr.bin + SHA256 metadata
    scripts/                    # Local copies of versioned Labs runner scripts
```

## Capture
First discover and verify the real local `tool-history.jsonl` path with read-only device inspection; **do not assume where it lives**. From the project workspace, run:

```powershell
$dc = Join-Path $env:USERPROFILE '.claude-server-commander'
$archive = 'C:\\0-BlackBoxProject-0\\vivim-labs\\ZCODE-DESKTOP-COMMANDER-OPERATIONS\\.private\\desktop-commander-stream'
pwsh -NoProfile -File .\scripts\archive-desktop-commander-history.ps1 -Source (Join-Path $dc 'tool-history.jsonl') -ArchiveRoot $archive
pwsh -NoProfile -File .\scripts\archive-desktop-commander-arguments.ps1 -SourceDirectory $dc -ArchiveRoot $archive
```

The script snapshots the rolling source temporarily, stores only previously unseen raw records as private increment segments, and emits minimal metadata indexes. It does not invoke ZCode, scrape the browser, modify ARBOR, or contact a network. No raw command arguments, typed text or outputs enter the index. Source snapshots are SHA-256 stamped in receipts and the temporary copy is deleted after indexing, limiting disk growth. Raw delta segments must stay private.

**Continuity:** The owner-authorized Windows Scheduled Task `VIVIM-DesktopCommander-Archive` already runs both scripts **every two minutes**, without elevated privileges, while the interactive user is available. A single execution preserves only history still present in the source; verify the task and receipt freshness regularly. Do not claim that historical trimmed calls can be reconstructed. A true complete stdout/stderr stream also requires explicit tee capture at the runner before Desktop Commander's 4-KiB omission boundary.

## Learning workflow
1. Raw evidence stays on PC. Index actions by date, tool, source hash, and status without raw parameters.
2. Join explicit test receipts to ARBOR Git SHA, device, command, exit code, and observed UI result; distinguish source build/test from live GPUI/PWA and phone verification.
3. Detect repeat failure fingerprints, timeline clusters and missing handoffs; issue an actionable improvement with a reproducible test, named owner and validation gate.
4. Publish only reviewed, redacted SITREPs/lessons to Labs; apply product patches cloud-first to ARBOR `main` through ChatGPT's GitHub authority.

## Safety
Treat raw history as sensitive: CLI arguments and outputs may contain tokens, personal data, source paths and provider information. No raw logs in Git, public issues, agent prompts, remote uploads or training corpora without separate opt-in and sanitization. Do not extend this into keystroke, screen or browser-session recording. Prefer short retention for raw data and preserve only needed diagnostic receipts.
## Daily learning and verification — deployed 22 November 2026

Daily analysis is run by Windows Scheduled Task `VIVIM-DesktopCommander-DailyLearning` at **19:05 PC local time**; a real scheduled-process invocation ended with Windows result `0x00000000` on 22 Nov and updated `reports/learning-latest.json`. The deterministic script `scripts/summarize-desktop-commander-index.ps1` only computes aggregates, pattern hit counts and daily timeline counts, with stable `learning-latest.{json,md}` outputs. No model is invoked, no token expense is incurred, and no raw text is uploaded.

On-demand health acceptance for both jobs is `scripts/desktop-commander-health.ps1 -ArchiveRoot $archive`. Verified on 22 Nov at 17:53:23 UTC with `HEALTH_RESULT=PASS`, 0 failures and 0 warnings. The check separately verifies schedule results, successful capture receipt freshness, latest learning report, unique hash index and disk headroom. It's a point-in-time check, not proof of indefinite uptime.

To preserve **complete raw stdout and stderr for one explicitly selected validation command**, use `scripts/capture-validation-output.ps1` with `-ArchiveRoot $archive`, `-EvidenceId`, `-Surface`, `-SourceSha`, `-Executable` and `-CommandArguments`. Product tests require exact 40-character Git source SHA; `UNPINNED-OPS` is permitted only for local operational diagnostics. The runner captures the two streams directly into separate byte-oriented local files and records hashes in a private receipt; it never replays archived commands or attempts provider calls by itself. First real read-only ZCode SITREP capture passed (exit 0, 785 stdout bytes, 0 stderr bytes). **A live controlled >4 KiB stream test passed**: 8,192 stdout bytes and 2,048 stderr bytes preserved, exact byte values and SHA-256 hashes independently checked. It tests recorder fidelity, not product behavior. Don't confuse process exit zero with product acceptance.

The native Desktop Commander history record schema exposes `timestamp`, `toolName`, `arguments`, `output` and `duration`, but **no session ID** on the inspected record. Day and tool indexes are grounded; per-session grouping cannot be truthfully reconstructed from missing identifiers.


## R009 — source-pinned PWA/Host acceptance evidence

Read [ARBOR-PWA-VERIFICATION-20261122.md](ARBOR-PWA-VERIFICATION-20261122.md) before interpreting old Desktop Commander failures as current regressions. The canonical existing ARBOR PWA runner was executed from a clean isolated checkout at `10006454b433efd53affc74e5e7a3d6518f96432`: 25/25 syntax-and-suite command groups, 196/196 Node TAP tests, 0 failures. Targeted Wiki/PWA/Host runs separately passed 29/29 and 14/14 assertions; they overlap canonical coverage. All stdout and byte hashes remain under private `validation-runs/`.

`scripts/summarize-pwa-verification-receipt.ps1` is the lightweight, no-model extractor for the existing canonical runner. It reads its exact-SHA private capture receipt, matches the stdout SHA-256 hash and parses the runner's native JSON and TAP summaries into a **local** `code-verification-summary.json`. It refuses to infer native GPUI, phone or cloud CI acceptance.

GitHub Actions had a `startup_failure` and zero created jobs for the checked `3798469` main commit. During the local pass, new cloud source advanced to `87143c6`, 12 commits beyond the tested SHA, including Wiki/Host/GPUI code changes. **R009 source PASS is valid only for `1000645`; refresh current HEAD before any release or customer-facing PASS claim.**
