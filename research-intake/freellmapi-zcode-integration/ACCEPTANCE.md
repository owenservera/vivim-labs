# Integration acceptance matrix — evidence required

| Gate | Local check | Pass evidence |
| --- | --- | --- |
| A1 | `node --version`, `npx freellmapi --help` | Node >=20.18 and `setup-zcode` listed |
| A2 | `./doctor.ps1` with gateway running | Authenticated `/v1/models` OpenAI list, non-empty IDs |
| A3 | Quit ZCode; `setup-zcode --dry-run` | Diff inspected; no write |
| A4 | `setup-zcode` actual write | Timestamped backup, other providers preserved, non-empty registry |
| A5 | Relaunch ZCode, select `freellmapi/auto` | One real reply; served model and routing evidence |
| A6 | ZCode coding agent calls a harmless tool | Tool name, args and result round-trip correctly |
| A7 | Long streaming response | SSE complete without malformed tool delta, timeout or truncation |
| A8 | Artificial rate-limit or exhausted provider | Expected fallback or explicit failure; no paid endpoint |
| A9 | Verify provider billing/quota | Independent portal receipts show $0 spend for tested requests |
| A10 | Re-run setup; revert test | Existing non-FreeLLMAPI providers persist; backup can restore |

All A1–A10 are **UNRUN on the owner's Windows system** as of 2026-11-11. Do not turn these checkboxes green based on source documentation alone. Beware ZCode configuration schema drift and upstream generator changes. This research pack does not replace tests.

## Risks and controls

1. **Credential file:** ZCode registry contains unified key; private permissions and backup handling required. Do not paste config in issues or telemetry.
2. **Claims:** catalog free-tier estimates are not user-specific eligibility; NVIDIA experimental credits can expire and OpenRouter rate limits apply.
3. **Unexpected payment:** disable paid upstream credentials/routes, check account usage and invoices, and fail closed when price evidence absent; the pack cannot independently enforce upstream billing.
4. **Host-only gateway:** keep loopback binding; do not publish port 3001 to LAN or internet without explicit secure design.
5. **Parallel routers:** existing `zcode-free-model-toolkit` uses different port; do not chain it under FreeLLMAPI without an evaluated policy.
6. **Model switching:** model-catalog capabilities differ; only promote proven models to an unattended coding pool.
7. **Sources:** upstream docs, Context7 `/tashfeenahmed/freellmapi`, `/anomalyco/opencode`; do not assume ZCode UI/CLI acceptance from schema support.

## Suggested evidence log (redacted only)

Date/time | ZCode version | FreeLLMAPI commit/release | Model ID | Provider | API status | Tool/SSE | $0 verified? | Notes

Do not store raw prompts, session data or tokens here.
