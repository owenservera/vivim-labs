# ZCode Free Model Toolkit — source-backed prototype

**2026-11-11 deep-refinement revision.** A standalone local coding-assistant tool in VIVIM Labs, **not** a VIVIM Host/product provider integration. No secrets are committed. Python 3.11+ standard library only.

**Verified baseline:** [ZCode v3.17.2 release](https://github.com/zai-org/ZCode/releases/tag/v3.17.2), tagged [custom provider schema](https://github.com/zai-org/ZCode/blob/v3.17.2/packages/provider/src/config/provider-data-schema.ts), tagged [tool stream boundary](https://github.com/zai-org/ZCode/blob/v3.17.2/docs/model-tool-call-stream-commit-boundary.md), and Context7 `/zai-org/zcode`. ZCode supports custom `openai-chat-completions` providers; the gateway exposes **only** that protocol. **Not** OpenAI Responses or Anthropic Messages.

## November 11 extension — Kilo Gateway and Free Model Explorer

- Kilo public catalog query: `python kilo_gateway.py --query <term>` or `python kilo_gateway.py --json`. Includes only verified `:free` zero-priced entries and the documented live `kilo-auto/free` special tier. `--all` only inspects paid models; it never unlocks paid routing.
- Native desktop GUI: `python model_explorer.py` or `./launch-model-explorer.ps1` on Windows. Search/filter, details, copy upstream vs ZCode route IDs, export JSON/CSV, refresh OpenRouter/Kilo/NIM and optional local OpenCode (read-only).
- Kilo uses a distinct **ZCode** ID prefix `kilo/` to avoid model-name collisions. Example: upstream `vendor/name:free` routes as `kilo/vendor/name:free` *only if currently eligible*. Copy the ZCode route ID from the GUI.
- Optional Kilo key is stored under user-bound DPAPI by updated `setup-windows.ps1`; anonymous free endpoint works without it, subject to per-IP limits. Kilo Auto Free may use data-logging upstreams; do not submit confidential information.
- See [KILO-AND-GUI.md](KILO-AND-GUI.md) for implementation, source links, runnable commands, tests and explicit real-device acceptance gaps. No VIVIM product-provider API or Host authority is modified.

## Era 1 — Windows setup (one time)

Clone/download this toolkit locally. In a Windows PowerShell window opened **in this directory**, run:

```powershell
.\setup-windows.ps1
```

The setup script prompts for an OpenRouter key, optionally a NVIDIA developer key and explicit trial approval; stores secrets using Windows **user-bound DPAPI** under `%LOCALAPPDATA%\ZFreeZCode`; creates a random local gateway bearer token; and attempts to register and start the **ZFree ZCode Gateway** Task Scheduler job at the signed-in user's logon. It prints the local token **once per setup invocation** so you can paste it into ZCode. The script is an **untested Windows automation candidate**; Task Scheduler/DPAPI behavior requires owner-PC acceptance. `-SkipTask` creates the credentials without task registration. Re-running with NVIDIA opt-out disables the NIM trial route.

In ZCode **Model Settings > Add Provider**, configure exactly:
- Name: `ZFree Local`
- API type: **OpenAI Chat Completions**
- Base URL: `http://127.0.0.1:8765/v1`
- API key: the **local bearer token printed by setup**, not your OpenRouter/NVIDIA key.
- Model ID: **`free-auto`** (add/enable it). **Do not** choose OpenAI Responses.

No unsupported edits to ZCode's versioned `provider_config.json`. ZCode custom-model registration is a one-time **manual UI step**; no supported external provider-configuration RPC was verified.

To start the Windows process manually after setup: `.\start-windows.ps1`. It decrypts user-bound secrets, sets the child process environment, and runs `zfree.py serve` on loopback only.

Alternative manual developer setup (cross-platform): set `OPENROUTER_API_KEY` and **required** `ZFREE_LOCAL_TOKEN` as environment variables, then `python zfree.py serve`. Without a local token, `serve` refuses to start. To inspect source catalogs without a key: `python zfree.py scan` or `doctor`. The latter may report zero eligible routes; no live user-account requests were run here.

## Era 2 — Automatic free model routing

| Source | Catalog and evidence | ZCode route |
|---|---|---|
| **OpenRouter** | [Live `/models`](https://openrouter.ai/api/v1/models), must end in `:free`, report numeric **zero across all published price fields**, support text output | **Yes**, automatic every hour, subject to key and quotas |
| **NVIDIA NIM** | Authenticated `/v1/models`, deny known non-chat/safety/embedding IDs; [Developer Program prototype terms](https://docs.api.nvidia.com/nim/docs/product) | **Opt-in only**; catalog cannot prove model tooling, account credits or permanent zero-cost |
| **OpenCode native free** | Local OpenCode `GET /provider` [documented server API](https://opencode.ai/docs/server/), explicit zero input/output/cache cost for native `opencode` or `opencode-go` IDs | **Native OpenCode only**, not ZCode proxy; official [free-client restriction](https://opencode.ai/docs/console/models) |

The synthetic ZCode model `free-auto` chooses current compatible free models and keeps a model pinned to a best-effort session fingerprint, preventing silent mid-tool-turn model switching. An explicit `X-ZFree-Session` header can supply a stronger per-session identity for clients that support it. ZCode's UI may not expose such dynamic headers; inference from the initial messages can fail after history compaction, so this is **not** guaranteed session identity. The model is selected using provider metadata; actual tokenization and agent behavior cannot be guaranteed by a catalog alone.

**Capability guardrails:** Tools require a provider/model advertising `tools`; resumed messages containing tool history also require tools. Image input requires explicitly advertised image support. JSON-formatted requests require advertised format support. NVIDIA tooling/vision is excluded until the user explicitly approves model IDs in `ZFREE_NIM_TOOL_MODELS` and `ZFREE_NIM_VISION_MODELS` after external validation, since NVIDIA's `/models` metadata is not a dependable capability contract.

**Schema normalization:** ZCode's historically conflicting `thinking`, `enable_thinking`, `reasoning_effort` and `reasoning` request fields are reduced to a supported OpenRouter `reasoning.effort` or omitted. For OpenRouter, token-limit aliases `max_tokens` and `max_completion_tokens` are consolidated according to `supported_parameters`; `top_provider.max_completion_tokens` is respected when known. Message/tool-result history is passed through without reformatting. Streaming SSE, including incremental tool-call deltas, is passed through without inventing final function arguments (ZCode performs that assembly).

**Cost and quota protections:** Disallow add-ons/plugins, web search requests and provider-executed tools that might trigger nonzero charges. No arbitrary model or upstream URL from a request. Local OpenRouter quota ledger persists successful and failed attempts under `ZFREE_HOME`, default **20/minute** and **50/day**, matching the conservative unfunded tier; this does **not** track requests through other apps or API keys. `ZFREE_OR_DAILY_LIMIT` may be raised to at most 1000 only when the account's [funding-based higher allowance](https://openrouter.ai/collections/free-models/) is known to apply. The toolkit never automatically uses a paid model. An opted-in NVIDIA prototype is **not** a strict zero-billing guarantee. Avoid treating that trial as cost-certified.

If provider HTTP errors are region-specific (403), model-specific tool capability (404), rate-limited (429), or transient (5xx), the gateway distinguishes retryable cases from invalid credentials/request schema errors, applies a model cooldown, and may try another **eligible free model** before returning an explicit error. Default at most two candidates per request (`ZFREE_MAX_ATTEMPTS`), never paid fallback. It never issues another HTTP response after streaming has started.

**OpenCode-native inventory:**
```powershell
opencode serve --port 4096
# in another terminal:
python .\opencode_native.py
```
This is a **read-only** inventory from the local OpenCode server; it never calls the OpenCode free inference endpoint from ZCode. Native OpenCode free models can be used through the official OpenCode CLI/app, but no unsupported client impersonation or side-effecting OpenCode agent bridge is included.

## Era 3 — Tests, acceptance and known gaps

Run offline tests in the toolkit directory:

```powershell
python -m unittest discover -s . -p "test_*.py" -v
python zfree.py doctor
```

**Evidence from this session:** 22 local Python tests passed on Python 3.13 (Linux container; reconstructed matching gateway/policy code) including a real loopback HTTP proxy to an entirely simulated upstream. This tests auth, free-only filtering, plugin rejection, session identity, tool history, output tokens, reasoning normalization, nonstream/SSE proxying, failures and persistent quotas. The corresponding implementation and tests are committed here; **no tests were run against your Windows machine or paid/free provider accounts**. Exact installed ZCode GUI integration remains unverified.

Required owner-machine gates:
1. Verify installed ZCode version matches v3.17.2 (or re-check new release schemas first) and custom provider has Chat Completions, `/v1` and `free-auto`.
2. Confirm real OpenRouter `free-auto` nonstream/SSE session and a successful multi-turn **tool call**, including response `[DONE]` and tool-call history.
3. Check region-sensitive models, 429 quota exhaustion, IPv4 loopback auth, Task Scheduler restart after login/sleep/network loss, and Windows DPAPI decryption under the correct user.
4. If enabled, verify each NVIDIA model's chat/tool capabilities and actual prototype entitlement/remaining allowance before treating it as an approved agent route. No account-level balance API was validated.
5. Check long-context behavior, actual tokenizer/context length, tool-schema fidelity, stream truncation, reasoning-content continuity, and model-specific 400 errors. The proxy cannot fix malformed provider tool-call deltas once streamed.

**Security:** no provider keys in GitHub; no plaintext local secrets stored by the Windows setup script, loopback-only HTTP with required bearer token, no prompt logging, and no unreviewed server-tool add-ons. Verify your own Windows Task Scheduler permissions. Different gateway instances/ports and other clients can still consume account quotas outside this process's local ledger.

**Known ZCode issue reports** (not all established as still reproducible): [conflicting reasoning fields #703](https://github.com/zai-org/feedback/issues/703), [Windows custom-provider wildcard schema #1296](https://github.com/zai-org/feedback/issues/1296), [stream tool-call abort #1454](https://github.com/zai-org/feedback/issues/1454), [provider-model selection #623](https://github.com/zai-org/feedback/issues/623), and [network stream crash #1446](https://github.com/zai-org/feedback/issues/1446). Further evidence and the chronological prior audit: [AUDIT-2026-11-11.md](AUDIT-2026-11-11.md).

**Overall status:** locally verified research prototype. **Not production/device accepted** without the real ZCode + provider + Windows tests above.
