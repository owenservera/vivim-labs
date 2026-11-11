# Kilo Gateway + Free Model Explorer (Labs research)

## ERA 1 — Kilo's actual public model discovery

Kilo documents a no-auth **GET** `https://ai-gateway.kilo.ai/api/v1/models`, an OpenAI-compatible **POST** `https://ai-gateway.kilo.ai/api/v1/chat/completions`, and free variants typically suffixed `:free`. Its documented `kilo-auto/free` is dynamically routed with no credits required; `kilo-auto/frontier`, `kilo-auto/efficient` and `kilo-auto/small` are **not** automatically considered free. Free availability and model IDs change. Do not ship a hard-coded model list.

```powershell
cd research-intake/zcode-free-model-toolkit
python .\kilo_gateway.py --query nemotron
python .\kilo_gateway.py --json > $env:TEMP\kilo-free-models.json
python .\kilo_gateway.py --all --query glm     # read-only inventory, not free routing
```

Eligibility is deliberately narrow: an explicit `:free` ID **plus** all supplied numeric price fields confirmed zero (including auxiliary fields), or the literal **`kilo-auto/free`**, listed in the live catalogue with no contradictory price. Missing catalog, missing price for other `:free` models, a paid model, or unknown advertised extra charges fail closed. Kilo API IDs are kept exact and receive a distinct **`kilo/` prefix only in ZFree's public route ID**, e.g. Kilo API model `z-ai/glm-5:free` becomes ZCode ID `kilo/z-ai/glm-5:free` *if that model appears in the live verified eligible catalog*. That example is not a current availability claim.

By default Kilo's documented anonymous free endpoint can be used without an API key, subject to limits. Set `KILO_API_KEY` in your private process environment for account-scoped calls; no key is stored in this repository. The local ZFree gateway still requires its own `ZFREE_LOCAL_TOKEN` for ZCode. `kilo-auto/free` and free NVIDIA-backed endpoints may log and use user data: **do not send personal or confidential prompts**. Live account entitlements, operational capability, geography, stream behaviour and billing must be confirmed on the owner's machine.

## ERA 2 — Desktop GUI (Windows; Python 3.11+)

```powershell
cd research-intake/zcode-free-model-toolkit
python .\model_explorer.py
# or
.\launch-model-explorer.ps1
```

Tkinter is included with most python.org Windows installations. No npm, provider SDK, browser automation, or dependencies are required. The GUI provides: asynchronous refresh of the existing OpenRouter/NVIDIA/Kilo catalogues; optional read-only discovery from a **locally running** `opencode serve --port 4096`; text search; provider filter; sortable model table; detailed native ID, route ID, context, max tokens, declared capabilities, pricing, evidence and privacy notes; **Copy upstream ID**, **Copy ZCode route ID**, **Copy ZCode base URL**, **Copy model details**; JSON/CSV export of the current filtered list; loading the prior cached inventory with an explicit stale warning.

To use the proxy in ZCode, configure its custom **OpenAI Chat Completions** provider using local base URL `http://127.0.0.1:8765/v1`, local bearer token from `setup-windows.ps1`, and `free-auto` (or a specific model's **ZCode route ID**). The full upstream model ID is for calling Kilo directly; do not paste it into ZCode if duplicates are possible. The GUI **never sends chat completions**.

An OpenCode-native free listing is for **native OpenCode usage only**, not ZCode routing. NVIDIA developer trial is not a perpetually guaranteed free API and is opt-in via existing `ZFREE_ALLOW_NIM_TRIAL` controls.

## ERA 3 — Regression and device gates

```powershell
python -m unittest discover -s . -p 'test_*.py' -v
python .\zfree.py doctor
```

Tests added here use isolated synthetic model catalogues and assert rejection of paid/nonzero/missing prices, malicious auxiliary charges, image-only entries, model alias preservation, and native OpenCode non-routing. This is **not** equivalent to live Kilo free calls or Windows GUI acceptance. Validate the live catalog, one real `$0` Kilo completion (with only harmless synthetic text), SSE streaming, ZCode ID selection, function-call support only for models that actually declare it, and quota/429 handling on the owner's machine before claiming production readiness. In particular, if a Kilo model's catalogue omits a `tools` capability, the proxy intentionally rejects tool-enabled agent requests instead of silently sending incompatible payloads.

Source references (consult again when updating):
- https://kilo.ai/docs/gateway/models-and-providers
- https://kilo.ai/docs/gateway/api-reference
- https://kilo.ai/docs/gateway/authentication
- https://kilo.ai/docs/getting-started/using-kilo-for-free

**Scope:** This stays under `vivim-labs/research-intake/zcode-free-model-toolkit` only, without touching VIVIM Host, Studio, World, HQ or TierFlow.