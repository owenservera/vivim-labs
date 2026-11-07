# Model Health Audit — 2026-11-04

**Window:** last 24h = `started_at > (SELECT MAX(started_at) FROM model_usage) - 86400000`
(≈ 2026-11-03 15:54 → 2026-11-04 15:54 UTC; `MAX(started_at)` = 1793804033141).
**Sources (read-only):**
- `C:\Users\VIVIM.inc\.zcode\cli\db\db.sqlite` — table `model_usage` (29,099 rows total; ~8,360 rows / 27 distinct provider/model in window) + `session`.
- `C:\Users\VIVIM.inc\.zcode\provider-depot\catalog.json` — refreshed 2026-11-04T05:04:35.585Z; sources hf-router (125), openrouter (455), cpa-bridge (58), zcode-local (15), models-dev (8,926 aggregate T2); free-proxy stale (unable to connect).
- `C:\Users\VIVIM.inc\.zcode\v2\provider_config.json` — providerOrder: deepseek, opencode-zen-chat-2, openrouter, new-provider, minimax-m-plan, new-provider-2, minimax, kilocode-free, …; enabled providers: opencode-zen-chat-2, kilocode-free, grok-build-oauth, new-provider-2, minimax-m-plan, new-provider. openrouter and minimax-m-plan rules carry no `enabled` flag (treated enabled); minimax / minimax-m-plan-cn / openrouter-zen-responses / zai / codex / ccr disabled.
- `C:\0-BlackBoxProject-0\repos_hq\hq\lib\model-router.mjs` — read only. Health rules (`classifyHealth`):
  - `unproven`: no observed usage / no completed call.
  - `budget-blocked` (checked first): `auth_failed`, `authentication failed`, `insufficient balance`, status=402, `payment required`, `purchase credits`, `upgrade your token plan`, `token plan usage limit`, `usage limit reached`, `(2056)`.
  - `capped`: rate limit, daily cap, quota, cooling down, `free-models-per-day`, `\(2056\)`.
  - `broken`: request-shape rejections (`invalid_request`, `unsupported parameter`, `validation:`) = all errors and ≥25% of total.
  - `healthy`: completed > 0 and reliability ≥ 0.5 (low-reliability annotation below 0.5).
  - Router eligibility rejects capped / budget-blocked / broken / unproven.

---

## 1. Every provider/model in model_usage, last 24h

| Provider/model | Completed | Errors | Cancelled | Total | Avg latency (ms) | Last error | Health (router rules) |
|---|---:|---:|---:|---:|---:|---|---|
| deepseek/deepseek-flash | 5,764 | 3 | 87 | 5,854 | 8,922 | ECONNABORTED network blip (×1) + unknown connect failure (×1) | **healthy** |
| openrouter/nvidia/nemotron-3.5-super-vl:free | 1,351 | 24 | 53 | 1,428 | 15,516 | rate_limited: free-models-per-day-high-balance (×20); ENOTFOUND openrouter.ai (×4) | **capped** |
| minimax-m-plan/MiniMax-M3 | 741 | 8 | 14 | 763 | 11,537 | Token Plan usage limit reached / (2056) (×7); unknown (×1) | **budget-blocked** |
| grok-build-oauth/grok-4.7 | 144 | 0 | 4 | 148 | 10,257 | — | **healthy** |
| new-provider/nvidia/nemotron-3.5-super-vl-120b-a12b | 68 | 0 | 0 | 68 | 10,202 | — | **healthy** |
| new-provider/nvidia/nemotron-3.5-lightning-30b-a3b | 53 | 1 | 1 | 55 | 4,160 | invalid_request: unsupported `enable_thinking` (×1) | **healthy** (1 shape error < 25%) |
| minimax-m-plan/MiniMax-M3.1-Flash | 5 | 2 | 0 | 7 | 3,976 | unknown "Model request failed" (×2) | **healthy by rule** but MiniMax credit is dead elsewhere — avoid |
| grok-build-oauth/grok-4.7-build-fast | 4 | 0 | 0 | 4 | 15,452 | — | **healthy** (small sample) |
| minimax/MiniMax-M3 | 0 | 4 | 0 | 4 | — | auth_failed ×4 | **budget-blocked** (dead) |
| new-provider/mistralai/mistral-large-4-1t-a52b | 0 | 3 | 1 | 4 | — | invalid_request: unsupported `reasoning`/`enable_thinking` (×3) | **broken** |
| openrouter/thinkingmachines/inkling:free | 0 | 4 | 0 | 4 | — | auth_failed ×4 | **budget-blocked** (dead — free but rejected) |
| new-provider-2/deepseek-ai/DeepSeek-V4.1-Flash | 0 | 3 | 0 | 3 | — | unknown "Model request failed" (×3) | **unproven** |
| minimax/MiniMax-M3.1-Flash | 0 | 2 | 0 | 2 | — | auth_failed ×2 | **budget-blocked** (dead) |
| minimax-m-plan-cn/MiniMax-M3.1-Flash | 0 | 2 | 0 | 2 | — | auth_failed ×2 | **budget-blocked** (dead) |
| opencode-zen-chat-2/mimo-v2.5-free | 0 | 2 | 0 | 2 | — | invalid_request: provider rejected the model request (×2) | **broken** |
| openrouter/google/gemma-4-31b-it:free | 0 | 0 | 2 | 2 | — | — | **unproven** (cancelled only) |
| account:zai-start-plan/GLM-5.3-Flash | 1 | 0 | 0 | 1 | 2,017 | — | **healthy** (sample of 1) |
| codex-oauth/gpt-6.1-sol | 0 | 1 | 0 | 1 | — | rate_limited: credentials cooling down / usage_limit_reached | **capped** |
| deepseek/deepseek-v4.1-flash-1017 | 0 | 1 | 0 | 1 | — | invalid_request: unsupported model name | **broken** |
| new-provider/nvidia/nemotron-3-ultra-550b-a55b | 1 | 0 | 0 | 1 | 24,434 | — | **healthy** (sample of 1) |
| opencode-zen-chat-2/deepseek-v4.1-flash-free | 0 | 0 | 1 | 1 | — | — | **unproven** |
| opencode-zen-chat-2/glm-5.3-flash-free | 0 | 0 | 1 | 1 | — | — | **unproven** |
| opencode-zen-chat-2/nemotron-3-ultra-free | 0 | 1 | 0 | 1 | — | invalid_request: provider rejected (×1) | **broken** |
| opencode-zen-responses/opencode/muse-spark-1.3-contributor-free | 0 | 1 | 0 | 1 | — | invalid_request: provider rejected (×1) | **broken** (provider + model disabled in config) |
| openrouter/nvidia/nemotron-3.5-lightning:free | 1 | 0 | 0 | 1 | 34,013 | — | **healthy** (sample of 1) |
| openrouter/openai/gpt-6.1-sol | 0 | 1 | 0 | 1 | — | unknown "Model request failed" (×1) | **unproven** |
| openrouter/recursive/flint-v1.0:free | 0 | 1 | 0 | 1 | — | invalid_request: provider rejected (×1) | **broken** |

Summary of states: healthy 9, capped 2, budget-blocked 5, broken 6, unproven 5 (27 distinct provider/model).
**The MiniMax credit is dead:** minimax-m-plan/MiniMax-M3 hit the 2056 token-plan limit (×7); minimax/MiniMax-M3, minimax/MiniMax-M3.1-Flash and minimax-m-plan-cn/MiniMax-M3.1-Flash all returned `auth_failed`. All MiniMax-pinned lanes must be routed off.

---

## 2. FREE models (priceIn = 0, priceOut = 0) — are they completable now?

Catalog lists **577 free rows** (sources: openrouter, models-dev; free-proxy source is stale/unreachable). Of the free models observed in the window:

| Free model | Observed result | Completable now? |
|---|---|---|
| new-provider/nvidia/nemotron-3.5-super-vl-120b-a12b | 68/68 completed, 0 errors | **YES** (free, zero errors) |
| new-provider/nvidia/nemotron-3.5-lightning-30b-a3b | 53 completed, 1 avoidable invalid_request | **YES** (free; don't send `enable_thinking`) |
| openrouter/nvidia/nemotron-3.5-lightning:free | 1 completed | **YES** (sample of 1) |
| account:zai-start-plan/GLM-5.3-Flash (zai) | 1 completed | **YES** (sample of 1) |
| openrouter/nvidia/nemotron-3.5-super-vl:free | 1,351 completed but rate_limited ×20 | **NO — capped** (free-models-per-day-high-balance) |
| openrouter/thinkingmachines/inkling:free | 0 completed, auth_failed ×4 | **NO — dead** |
| opencode-zen-chat-2/mimo-v2.5-free | 0 completed, invalid_request ×2 | **NO — broken** |
| opencode-zen-chat-2/nemotron-3-ultra-free | 0 completed, invalid_request ×1 | **NO — broken** |
| openrouter/recursive/flint-v1.0:free | 0 completed, invalid_request ×1 | **NO — broken** |
| openrouter/google/gemma-4-31b-it:free | 0 completed, 2 cancelled | **unproven** |
| opencode-zen-chat-2/deepseek-v4.1-flash-free / glm-5.3-flash-free | 0 completed, cancelled only | **unproven** |

Note: free ≠ completable — 5 of 11 observed free models are capped/broken/dead right now.

---

## 3. PAID models that still have credit (no 402, no auth_failed)

| Paid model | Price (per 1M in/out) | Evidence | Credit OK? |
|---|---|---|---|
| deepseek/deepseek-flash | $0.15 / $0.60 | 5,764 completed, reliability 99.94%, no auth_failed/402/2056 | **YES** |
| deepseek/deepseek-v4.1-flash-1017 | $0.50 / $1.50 | 0 completed; invalid_request (wrong model name) | Has credit but **broken as configured** |
| grok-build-oauth/grok-4.7 | unpriced (cpa-bridge OAuth; priceKnown=false) | 144 completed, 0 errors | OAuth, no payment signal |
| minimax-m-plan/MiniMax-M3 | $0.72 / $2.88 | 741 completed but 2056 token-plan limit → budget-blocked | **NO** |
| minimax/MiniMax-M3 / M3.1-Flash; minimax-m-plan-cn/M3.1-Flash | $0.72/$2.88 / $0.1494/$0.5976 | auth_failed on all | **NO — dead** |

Only **deepseek/deepseek-flash** is paid, funded, and demonstrably completing at volume right now.

---

## 4. Ranked shortlist — 3 funded, healthy models to route the 22 MiniMax-pinned agents

Router policy (DEFAULT_POLICY): preferFree, tiers free(0) < priced-known(1) < unpriced(2); score = effectiveCost×1e6 + latency×0.05 + (1−reliability)×1000; maxUsdPerCall $0.50.

**Rank 1 — new-provider/nvidia/nemotron-3.5-lightning-30b-a3b (FREE)**
- Catalog: priceIn 0, priceOut 0, context 1,000,000 (models-dev).
- Evidence: 53 completed / 0 budget/cap/auth failures in 24h; avg latency **4,160 ms** (fastest healthy model observed); single invalid_request on `enable_thinking` (avoidable, <25% → healthy).
- Router tier 0, lowest score of free candidates.

**Rank 2 — new-provider/nvidia/nemotron-3.5-super-vl-120b-a12b (FREE)**
- Catalog: priceIn 0, priceOut 0, context 1,048,576 (models-dev); this machine's current session model.
- Evidence: **68/68 completed, zero errors** in 24h; avg latency 10,202 ms; no auth_failed / rate_limited / 2056 ever observed for it.
- Router tier 0; largest proven free volume on NVIDIA integrate API.

**Rank 3 — deepseek/deepseek-flash (PAID, $0.15 in / $0.60 out per 1M)**
- Evidence: **5,764 completed** (dominant volume of the window), reliability **99.94%**, avg latency 8,922 ms; credit intact — no auth_failed, no 402, no 2056; single ECONNABORTED network blip (transient, not dead).
- Router tier 1; fallback when both NVIDIA free models are capped/unavailable.

Reserve (not in top 3): grok-build-oauth/grok-4.7 — 144 completed, 0 errors, avg 10,257 ms, healthy, but unpriced in the catalog (priceKnown=false → tier 2, ranks behind priced/free); OAuth credential, no payment exposure. Use if the top 3 are unavailable.

**Do not route to:** any minimax model (budget-blocked/dead), openrouter/nvidia/nemotron-3.5-super-vl:free (capped, ENOTFOUND openrouter.ai), openrouter/thinkingmachines/inkling:free (auth_failed).

---

## 5. Models that look funded but are actually dead

| Model | Price/free status | Why dead |
|---|---|---|
| openrouter/thinkingmachines/inkling:free | FREE | 4 × auth_failed, 0 completed → budget-blocked |
| minimax/MiniMax-M3 | PAID ($0.72/$2.88) | 4 × auth_failed, 0 completed → budget-blocked |
| minimax/MiniMax-M3.1-Flash | PAID ($0.1494/$0.5976) | 2 × auth_failed → budget-blocked |
| minimax-m-plan-cn/MiniMax-M3.1-Flash | PAID ($0.1494/$0.5976) | 2 × auth_failed → budget-blocked |

Not dead (transient only): deepseek/deepseek-flash (1 × ECONNABORTED, 5,764 completed — healthy); openrouter/nvidia/nemotron-3.5-super-vl:free (4 × ENOTFOUND openrouter.ai but 1,351 completed — currently capped, not dead).
