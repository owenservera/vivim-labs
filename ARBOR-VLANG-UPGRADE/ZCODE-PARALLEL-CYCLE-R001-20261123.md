# ARBOR VLANG / NL-to-Command — ZCode parallel iteration R001
Date: 2026-11-23 (Europe/Madrid). Owner: ChatGPT primary source integration; ZCode independent observers.
Status: **deterministic parallel cycle PASS / Windows scheduler installed**. **ZCode model workflow launch is a separate acceptance gate; do not label it running without an accepted run ID.**

## ERA 1 — Existing product and what was reused

There is **one** GPUI/PWA composer and one Host control/effect authority. Text is interpreted by the existing VLANG/NLCL compiler, then mapped to a catalog-bound typed action candidate with exact risk, source/revision, preconditions and appropriate Host approval/receipt. A typed suggestion, template, Wiki match or ZCode model opinion is **not execution authorization**.

Canonical source seams:
- `apps/vivim-host/control/intent-planner.mjs`, `command-intent.mjs`, `dispatcher.mjs`: deterministic language -> candidate/clarification/refusal -> authority.
- `apps/vivim-host/control/action-catalog.v1.json`: verified version **17** on November 23, with text/mobile/desktop semantics and R0–R3 risk.
- `apps/vivim-host/control/heldout.v1.json`: frozen before initial scoring; evaluate but never train/tune against this set.
- `core/programmability/src/command_intent_projection.rs`, `native/gpui-desktop/src/control.rs`: Rust projection and inert native user-facing preview.
- `apps/mobile-relay/composer-assist.mjs`, `app.js`, `index.html`: same PWA composer, typed slots, real source Wiki and visual feedback.
- Existing Wiki read revision/high-water contract and Host Control approval path; no new browser commander or effect daemon.

Prior research already specified **Z-VLANG, Z-SAFETY, Z-AUTOWIKI, Z-VISUAL**. The earlier ZCode pilot reached upstream HTTP 429: no model-generated finding was verified. Existing `arbor-life` and `arbor-product` mix legacy routes and source-writing stages and must NOT be run unmodified against the personal Host.

## ERA 2 — R001 built and source-pinned verification

**ARBOR commit:** `f79788daa280fa651930a087290c54bb0ec49276` on `main`.
**New tracked ZCode DWF:** `.zcode/workflows/arbor-vlang-iterative-observers.dwf.ts` (explicitly tracked despite `.zcode/` default ignore).
The DWF requires clean `HEAD == origin/main == args.sha`. It runs the 4-file existing deterministic baseline first, then dispatches **four independent read-only ZCode agents concurrently** to inspect non-overlapping product concerns. It collects bounded structured findings, then asks an adversarial independent verifier to reject unsupported, overlapping or hypothetical suggestions. It reports only one possible next source fix and makes no Host execution call or source patch itself. Actual DWF run acceptance / provider identity must be checked; a saved file is not a live run.

**Operator runner:** `ZCODE-DESKTOP-COMMANDER-OPERATIONS/scripts/vlang-parallel-cycle.ts`, TypeScript executed with the installed Node 24 `--experimental-strip-types` (the `bun` executable on this PC currently reports an invalid/corrupted bin mapping). It guards exact current `origin/main`, dirty tracked source and a duplicate already-PASS SHA. It executes four existing Node test groups **in parallel**, writes only bounded counts, SHA and exit code to private `local-logs/vlang-iterations`, and exits nonzero on any failure. This is real execution, not LLM-generated scoring.

First cycle at `d19e388c` found a genuine test-maintenance gap: Z-VLANG 22/22 PASS, Z-SAFETY 159/160 FAIL on the outdated `remote-root` invalid-source expectation, Z-AUTOWIKI 12/12 PASS, Z-VISUAL 0/unknown (the first-customer test carried obsolete service-worker v41, pre-feedback mount order and composer toolbar selectors). The product source had intentionally changed: invalid source now *throws* `CONTROL_SOURCE_INVALID`, First Contact initializes the receipt observer before Wiki, tool mode has a dedicated safe Send visibility class and SW cache is v47. Fixing the tests to assert current safety semantics (without altering runtime grammar) was the narrowest source improvement.

Second cycle at exact committed `f79788da`:
| Lane | Real tests | Result |
|---|---:|---|
| Z-VLANG | 22 / 22 | PASS |
| Z-SAFETY | 160 / 160 | PASS |
| Z-AUTOWIKI | 12 / 12 | PASS |
| Z-VISUAL | 21 / 21 | PASS |
| **TOTAL** | **215 / 215** | **PASS** |

All four exit codes were 0. Physical GPUI typing, Windows native menu painting, actual Android source parity and a settled model-driven ZCode DWF remain **UNRUN** by these unit suites. No account data or raw private user text is placed in public receipts.

**Automated recurrence:** Windows Task Scheduler `VIVIM-VLANG-Observer` is installed, status `Ready`, interactive current-user principal, single instance and four-minute ceiling. Next observed trigger: November 23 at 19:35 Europe/Madrid. At each hourly trigger it runs exactly ONE read-only four-lane deterministic cycle, only for a new pinned SHA; unchanged passing revisions print `UNCHANGED_VERIFIED`. It does not call ZCode models or an API without a separately accepted provider pilot.

## ERA 3 — Actual model pilot / owner handoff

One bounded headless ZCode request was dispatched at pinned `f79788da`, with `--workflow-mode alwaysOn --mode plan`, instructing the official `CreateWorkflow` interface to run the existing DWF by project path and exact SHA. A first `.cmd`/Start-Process argument-quoting attempt yielded only a generic greeting and launched nothing; a corrected direct PowerShell `& zcode --prompt $prompt` invocation **did launch** DWF run **`dwfrun-43fc3e3e-fbb4-460f-91e1-c74ecd59b39e`**, with a logged pinned-SHA admission. The ZCode provider concurrently returned an upstream **rate_limit_error** from the Baidu/OpenRouter shared pool; the accepted run is NOT evidence of successful model-scout results or verifier completion. The scoped headless pilot produced repeated HTTP 429 diagnostics (about 446 KB stderr) and was **explicitly stopped at PID 17552** on November 23 to prevent further retries. No model-scout PASS or terminal DWF success is verified; never automatically restart this run ID. Original prompts contained no private chat/corpus material. Do not retry the same pilot blindly or silently switch to paid providers; require actual terminal run evidence to upgrade model-scout status from BLOCKED.

Operator commands (PowerShell, current Windows PC):
```powershell
$root = 'C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS'
node --experimental-strip-types "$root\scripts\vlang-parallel-cycle.ts" --once
powershell -NoProfile -ExecutionPolicy Bypass -File "$root\scripts\vlang-schedule.ps1" -Action status
powershell -NoProfile -ExecutionPolicy Bypass -File "$root\scripts\vlang-zcode-pilot.ps1" -Action status
# To disable the hourly observer:
powershell -NoProfile -ExecutionPolicy Bypass -File "$root\scripts\vlang-schedule.ps1" -Action remove
```

Evidence: `$root/local-logs/vlang-iterations/latest.json`, per-run JSON, `vlang-model/<sha>.json` model-admission fence and the scoped ZCode pilot stdout/stderr logs. Those are local-only, not durable public training material. Added `scripts/vlang-zcode-pilot.ps1`: a one-attempt-per-SHA model pilot wrapper that rejects an unverified deterministic baseline, watches for a 429/timeout, stops only its own spawned process tree and records BLOCKED rather than automatically retrying.

Operational improvement loop: **(1)** source SHA and canonical tracker task, **(2)** deterministic regression lanes, **(3)** only after a healthy provider, parallel ZCode read-only scouts, **(4)** independent verifier with precise reproduction and one held-out neighbor, **(5)** ChatGPT source owner chooses one minimal Rust-first correction and pushes `main`, **(6)** deterministic replay and physical UI verification, **(7)** keeper updates original task ledger. Never train on heldout results; incorporate only user-approved miss evidence. Two no-defect cycles or quota 429 => stop AI calls and keep cheap deterministic observer.

Boundaries: no model/API required for a supported VLANG command; no free-text shell execution; no unapproved edits/approvals; no source rewriting by ZCode observers; no resurrected legacy /chat or /wiki pages; no second GPUI frontend; no false "delivered" provider badge; no automatic seven-question personal corpus extraction. Future work is **real physical UI acceptance** and a model-scout launch receipt—not another architecture.
