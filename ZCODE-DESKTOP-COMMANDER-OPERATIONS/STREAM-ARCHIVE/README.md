# Desktop Commander stream archive — R001

Status: archival specification and script committed to VIVIM Labs. **Not evidence of a PC capture until the actual local source and archive paths are verified.**

## Purpose
Turn Desktop Commander activity into a durable, local-only chronological record for debugging, failure clustering, task receipts, and lessons learned. Keep raw evidence private on the PC; only reviewed, redacted lessons or receipts belong in GitHub. This does not create a second ARBOR Host journal, browser recorder, process supervisor, or agent that writes product source.

## Actual source / known limitations (22 Nov 2026)
- Live device: DESKTOP-9KB03LU, Desktop Commander 0.3.4.
- Desktop Commander reports 3,805 total tool calls, 3,786 successful and 19 failed, 5 sessions; 1,000 recent calls in in-memory history.
- Its `get_recent_tool_calls` reads local `tool-history.jsonl`, with history trimming and outputs larger than 4 KiB omitted. These are **not full-fidelity stdout/stderr or all past sessions**.
- A tool-call success is not a product/test success. Multiple recent PWA tests failed despite successful tool invocations.
- No claims of continuous collection, lossless history or local installation until verified.

## Local-only data layout

```text
ZCODE-DESKTOP-COMMANDER-OPERATIONS/
  STREAM-ARCHIVE/               # Versioned design and reviewed SITREPs only
    README.md
    SITREP-20261122.md
  local-logs/                   # LOCAL PRIVATE: excluded from Git
    raw/YYYY-MM-DD/             # Atomic snapshots of source JSONL; sensitive
    index/days/YYYY-MM-DD.jsonl # One row per observed tool call
    index/tools/TOOL/YYYY-MM-DD.jsonl
    index/seen.sha256           # Deduplication hashes
    receipts/capture-*.json    # Observed capture runs and evidence gaps
```

## Capture
First discover and verify the real local `tool-history.jsonl` path with read-only device inspection; **do not assume where it lives**. From the project workspace, run:

```powershell
pwsh -NoProfile -File .\scripts\archive-desktop-commander-history.ps1 -Source 'ACTUAL_PATH\tool-history.jsonl'
```

The script copies the source to a local immutable snapshot and emits minimal metadata indexes. It does not invoke ZCode, scrape the browser, modify ARBOR, or contact a network. No raw command arguments, typed text or outputs enter the index. Snapshot filenames incorporate SHA-256 to avoid overwriting identical archives; the raw snapshots must stay private.

**Continuity:** A single execution preserves only the history still present in the source. To prevent future trimming gaps, arrange frequent owner-approved capture (e.g., once per minute) and verify missed intervals. Do not claim that historical trimmed calls can be reconstructed. A true complete stdout/stderr stream also requires explicit tee capture at the runner before Desktop Commander's 4-KiB omission boundary.

## Learning workflow
1. Raw evidence stays on PC. Index actions by date, tool, source hash, and status without raw parameters.
2. Join explicit test receipts to ARBOR Git SHA, device, command, exit code, and observed UI result; distinguish source build/test from live GPUI/PWA and phone verification.
3. Detect repeat failure fingerprints, timeline clusters and missing handoffs; issue an actionable improvement with a reproducible test, named owner and validation gate.
4. Publish only reviewed, redacted SITREPs/lessons to Labs; apply product patches cloud-first to ARBOR `main` through ChatGPT's GitHub authority.

## Safety
Treat raw history as sensitive: CLI arguments and outputs may contain tokens, personal data, source paths and provider information. No raw logs in Git, public issues, agent prompts, remote uploads or training corpora without separate opt-in and sanitization. Do not extend this into keystroke, screen or browser-session recording. Prefer short retention for raw data and preserve only needed diagnostic receipts.