# ZCode free-model toolkit (experimental, local-only)

**Status:** Source-backed research implementation; not validated on the owner's Windows PC or ZCode sessions. Lives in Labs as a self-contained tool, **not** the VIVIM Host runtime. No provider credentials or local files are committed.

## Verified compatibility baseline (2026-11-11)

- ZCode official source: [zai-org/ZCode](https://github.com/zai-org/ZCode), release **v3.17.2** (2026-11-03). [ZCode installation](https://zcode.z.ai/en/docs/install).
- Context7 library: `/zai-org/zcode`. Source schema [provider-data-schema.ts](https://github.com/zai-org/ZCode/blob/main/packages/provider/src/config/provider-data-schema.ts): `openai-chat-completions`, `openai-responses`, `anthropic-messages`. ZCode supports custom OpenAI-compatible providers via its **Model Settings > Add Provider** UI; no unsupported direct edits to version-sensitive ZCode config.
- [OpenRouter live models](https://openrouter.ai/api/v1/models) exposes pricing; candidate must have `:free` suffix and numeric zero prompt **and** completion prices, otherwise rejected. [Official free-model rules](https://openrouter.ai/collections/free-models) include throttling/quotas; zero model pricing is not unlimited availability.
- [NVIDIA NIM developer access](https://docs.api.nvidia.com/nim/docs/product): free prototyping access can be subject to trial allowance and terms; a listed model is **not** proof of perpetual zero-price usage. Therefore NVIDIA is disabled unless explicitly opted into trial usage.
- [OpenCode Console models](https://opencode.ai/docs/console/models): free-tier chat models **require the OpenCode client**; the same free chat access is **not** a generic ZCode API. The toolkit reports that boundary instead of misleadingly forwarding ZCode to paid Console endpoints. A future supported, explicitly free third-party access mechanism can be added only after independent verification.

## Era 1 — Install & connect once

Requires Python 3.11+ standard library. From this directory on Windows PowerShell:

```powershell
$env:OPENROUTER_API_KEY = "your-openrouter-key"
# Set optional local authentication shared secret (recommended):
$env:ZFREE_LOCAL_TOKEN = "choose-a-long-random-local-secret"
python .\zfree.py doctor
python .\zfree.py serve
```

Then in ZCode **Settings > Model Providers > Add Provider**, configure:
- Name: `ZFree Local`
- API type: **OpenAI Chat Completions** / OpenAI-compatible
- Base URL: `http://127.0.0.1:8765/v1`
- API key: the `ZFREE_LOCAL_TOKEN` chosen above (or any dummy string if token is unset)
- Add model ID: **`free-auto`** and enable it.

ZCode's config UI remains the sole writer of ZCode settings. `free-auto` resolves at request time so there is **no daily manual model editing**. Restart ZFree after changing environment variables. Model availability refreshes automatically every hour (configurable). Run the gateway as your own Windows scheduled task at login or manually; do not publish its local port.

## Era 2 — Automated provider policy & routing

`python zfree.py scan` performs immediate catalog discovery and writes a cache report to `~/.zfree/catalog.json` (or `ZFREE_HOME`). `GET /health`, `GET /v1/models`, and `POST /v1/chat/completions` are available on loopback. Supports transparent SSE forwarding, ordinary JSON, and tools payload forwarding (model compatibility varies). Routes are reevaluated on TTL expiration. Up to four candidates may be tried if an upstream fails **before** it starts streaming, without falling back to paid models.

| Source | Discovery | Free eligibility | ZCode integration |
|---|---|---|---|
| OpenRouter | Public live model catalog | `:free` + zero prompt and completion catalog price | Automatic, needs API key |
| NVIDIA NIM | Authenticated `/v1/models` | **Trial allowance only**, opt-in required | Automatic with `NVIDIA_API_KEY` and `ZFREE_ALLOW_NIM_TRIAL=1` |
| OpenCode free | Native OpenCode app catalog | Free **inside OpenCode**; not portable API usage | **Not routed** in ZCode; explicit compatibility limit |

Opting into NIM trial: `$env:NVIDIA_API_KEY = "..."; $env:ZFREE_ALLOW_NIM_TRIAL = "1"`. This opt-in means you accept that provider's quota, trial conditions, and possible future changes. To enforce a strict no-cost posture, leave it **off**. Never assume a `/models` API certifies price.

Default preference is price-evidenced OpenRouter free models, then opted-in NIM trial. Pin a known eligible ID with `ZFREE_PREFER` (comma-separated), but it remains subject to live eligibility. The gateway never accepts an arbitrary paid model or URL from the incoming request.

For unattended startup, create a Windows Task Scheduler task **in the user's session**, invoking `python.exe` with `zfree.py serve`. Store credentials in the user's environment or a machine-approved secret store, not in Task Scheduler command arguments, repository files, screenshots, or logs. Windows restart / sleep behavior must be tested locally.

## Era 3 — Acceptance, gaps and safety

Acceptance matrix:
1. `python zfree.py doctor` lists catalog status without credentials or prompts in logs.
2. With no keys, `GET /health` reports no eligible models, and a completion returns 503 rather than silently using a paid model.
3. With an OpenRouter key, catalog returns `:free`, zero-priced models; model `free-auto` works for a real ZCode session.
4. Force HTTP 429 and verify retry stays inside eligible models, with no paid fallback.
5. Verify ZCode tool-calling, SSE, context windows, system prompts, and long-running sessions on installed v3.17.2.
6. Verify a Windows login-started gateway continues through network loss/reconnect and does not expose auth tokens.

**Known limitations:** No successful owner-account request or device validation has been performed here. Provider catalogs may be stale for up to refresh TTL, and catalog zero price doesn't promise availability, throughput, tool-calling, or quality. NVIDIA trial credits can be exhausted; the tool cannot inspect actual billing or stop a provider that independently bills beyond a trial. SSE disconnects cannot safely be retried after delivery starts. ZCode's custom-provider behavior is source/docs-supported but needs local acceptance. OpenCode free models are excluded from the ZCode route by the provider's stated client restriction. No remote exposure, no key collection, no proxy traffic analytics, and no dynamic config write into ZCode internals.

**Possible future enhancement:** Add explicit budget/quota evidence for NVIDIA, managed Windows service installation, authenticated route health probes, safe periodic checks for model deprecations, and a companion *OpenCode-native* free model selector separate from ZCode. Keep the zero-cost guarantee fail-closed and do not conflate a free trial with permanent $0 pricing.

## References / provenance

Context7 `/zai-org/zcode` queried on 2026-11-11; relevant source paths: `packages/provider/src/config/provider-data-schema.ts`, `packages/provider-node/src/provider-config-file-codec.ts` in `zai-org/ZCode`. Source inspection confirms protocol types and a schema-versioned personal provider configuration, but does **not** justify writing that file externally. Official pages linked above are mutable; the live installed-app version remains unverified.
