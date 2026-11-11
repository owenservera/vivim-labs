# FreeLLMAPI → ZCode integration pack (2026-11-11)

**Status:** Research-backed integration recipe and read-only diagnostics. Not installed on owner's Windows machine. **Scope:** local developer tooling only; not VIVIM Host runtime. Do not commit credentials or generated ZCode registry.

## ERA 1 — Set up and preserve the ZCode configuration

Source: [FreeLLMAPI](https://github.com/tashfeenahmed/freellmapi), [ZCode agent instructions](https://github.com/tashfeenahmed/freellmapi/blob/main/docs/en/clients/01-agent-clients.md), [CLI documentation](https://github.com/tashfeenahmed/freellmapi/blob/main/cli/README.md). Context7 queried: `/tashfeenahmed/freellmapi`, `/anomalyco/opencode`. FreeLLMAPI upstream baseline: `4c8c2f834b587b6acc3f00698ad8f6468bea1aa2`; VIVIM Labs baseline `6cd098f7b1bf5602438488cb5c0ce1c2d02c3da4` (both observed 2026-11-11).

1. Install [Windows FreeLLMAPI release](https://github.com/tashfeenahmed/freellmapi/releases/latest) (preferred for an existing Windows workstation), or build using upstream [Windows instructions](https://github.com/tashfeenahmed/freellmapi/blob/main/docs/en/install/01-install.md). Review upstream binary provenance; avoid unaudited pipe-to-shell installers.
2. Open http://127.0.0.1:3001; enter provider keys in **Keys** page; copy **unified** API key (not a dashboard token or raw upstream provider key). Do not confuse a model advertised as free with available quota in the user's account.
3. **Completely quit ZCode Desktop and CLI**. Its live registry may otherwise overwrite changes.
4. Run the provided PowerShell diagnostic `./doctor.ps1`, then use `npx freellmapi setup-zcode --url http://localhost:3001 --dry-run` and inspect the planned diff. For real write: `npx freellmapi setup-zcode --url http://localhost:3001`. Beforehand, set `$env:FREELLMAPI_API_KEY = Read-Host "Unified API key"` in this PowerShell session. No API key belongs on command-line arguments.
5. Relaunch ZCode; select `freellmapi/auto`, run a short prompt, then a tool-call test. To use a specific published available model: `npx freellmapi setup-zcode --url http://localhost:3001 --model <actual-model-id>`. Use `--profile <name>` for a separate provider entry; do not assume profile changes the default.
6. Clear session key after setup: `Remove-Item Env:FREELLMAPI_API_KEY`. Keep the generated registry and timestamped backups private. If switching providers, use ZCode's model picker, rather than overwriting its configuration from two tools.

**Native registry:** `~/.zcode/v2/provider_config.json` or path set by `ZCODE_PERSONAL_PROVIDER_CONFIG_FILE`. The upstream generator adds an `api-key` access entry, `openai-chat-completions` endpoint at `http://localhost:3001/v1`, current models and `freellmapi/auto` default, and claims to merge and timestamp-backup the existing registry. ZCode does **not** support secret environment interpolation in this registry. Never commit this file. Old `~/.zcode/v2/config.json` and `~/.zcode/cli/config.json` are migration-only.

## ERA 2 — Model governance and boundaries

- FreeLLMAPI is a **separate alternative** to the existing [ZFree experimental gateway](../zcode-free-model-toolkit/README.md), not a layer that should be stacked over it. Use only **one local gateway** in the active ZCode provider to avoid double fallback, conflicting quotas, unclear billing evidence and unnecessary latency.
- FreeLLMAPI advertises 34 providers/635 free endpoints/7.4B theoretical monthly tokens; **none** is an entitlement of the user's account. The published model catalog is refreshed via a signed feed; free catalog is delayed compared with premium. Use live `GET /v1/models`, then provider-specific price and quota evidence.
- Configure NVIDIA NIM, OpenRouter and OpenCode Zen **only with their valid provider credentials, eligible endpoints and applicable terms**. Distinguish strictly free, promotional/trial-credit and paid-access models. Do not assert OpenCode's app-only free tier grants arbitrary API usage.
- `auto` can select different capabilities, token windows and tool-calling characteristics between requests. For reliable coding, validate tool calls and streaming on chosen model(s), pin stable IDs when needed, and treat provider failover as conditional rather than feature-equivalent.
- No provider APIs or this local coding router should be wired into VIVIM Host runtime by this pack. Nothing here modifies Host, HQ, TierFlow, or existing local ZCode configuration.

## ERA 3 — Acceptance and recovery

Run `./doctor.ps1` and read [ACCEPTANCE.md](./ACCEPTANCE.md). A live model listing is not evidence of a successful model request, tool round-trip, free billing or actual ZCode app integration. Record successful API responses, tool-use tests, HTTP errors, fallback provider and **redacted** quota/billing receipts. The authoring session did not execute owner-account or native Windows tests.

If setup fails: check that FreeLLMAPI is running; ensure this is the *unified* key; check port conflicts (ZFree normally uses 8765, FreeLLMAPI 3001); verify `npx freellmapi --help` exposes `setup-zcode`; confirm registry path and file permissions; fully exit ZCode before writing; validate ZCode reload; restore the timestamped backup if the merge was incompatible. For HTTP 429, inspect upstream quota and routing, don't silently fall back to paid providers.

**Sources:** [upstream client docs](https://github.com/tashfeenahmed/freellmapi/blob/main/docs/en/clients/01-agent-clients.md), [supported agents](https://github.com/tashfeenahmed/freellmapi/blob/main/docs/en/clients/02-supported-agents.md), [install](https://github.com/tashfeenahmed/freellmapi/blob/main/docs/en/install/01-install.md), [CLI](https://github.com/tashfeenahmed/freellmapi/blob/main/cli/README.md), [ZCode support request #1523](https://github.com/tashfeenahmed/freellmapi/issues/1523). Upstream version-dependent; revalidate after upgrades.
