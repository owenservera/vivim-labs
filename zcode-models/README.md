# zcode-models — provider status & budget capture

One-stop capture of the four active ZCode providers: **live status**, **budget/plan
state**, and **per-model pricing/context** — all in the ZCode `providerRules` shape so
anything here can be pasted straight into config, routing ladders, or workflow model
pins. Built 2026-11-20.

## Files

| File | What it is |
| --- | --- |
| `zcode-models-capture-20261120.json` | THE deliverable — 4 providers × 22 models, status + budget + pricing, keys redacted |
| `build-capture.mjs` | Re-runnable builder: `node build-capture.mjs --probe <manifest.json>` |
| `capture-20261120/manifest.json` | Probe agent's live-check manifest (HTTP-verified 2026-11-20) |
| `capture-20261120/raw/*.status.json` | Verbatim probe responses (keys redacted, ≤50KB per body) |

## Verdicts at capture time (2026-11-20 ~18:10–18:25Z)

| Provider | Live status | Budget state |
| --- | --- | --- |
| `new-provider` (NVIDIA NIM) | **OK** — `/v1/models` 200, 33 models served, **9/9 configured present**, no rate-limit headers surfaced | No quota endpoint answered; key auth valid. Treat as free-tier credit semantics, unverified limit |
| `openrouter-2` | **OK** — `/key`, `/credits`, `/models` all 200; 4 `:free` ids live in 442-model catalog | **Daily free quota exhausted**: `free_model_daily_requests` used **1012/limit 1000**, remaining 0. Credits balance **26** vs usage **26.191**; key limit null → paid calls draw on credits |
| `minimax-m-plan` (International) | **FAIL to serve** — auth accepted; `chatcompletion_v2` in-band `base_resp.status_code=2056 "Token Plan usage limit reached"`, anthropic `/v1/messages` 429; catalog still lists 9 models | **Plan quota EXHAUSTED** (verbatim in raw capture) — needs plan upgrade, credit purchase, or window reset |
| `account:zai-start-plan` | **Live in-session** (this capture was produced on it) but REST **UNVERIFIED** — stored coding-plan api-key expired (401 "token expired or incorrect"), oauth token rejected on REST paths (401/500/404) | No quota signal obtainable via REST; plan gateway is `https://zcode.z.ai/api/v1/zcode-plan/anthropic` |

## How to leverage

- **Pin a model**: `providers[].models[].zcodeModelId` (e.g. `account:zai-start-plan/GLM-5.3-Flash`) pastes verbatim into `CreateSession.model` or a workflow's `subagent_model`.
- **Re-wire a provider**: `providers[].config` mirrors `~/.zcode/v2/provider_config.json` providerRules (`access`/`api`/`modelOrder`); key values are `[REDACTED len=N]` — read the real key from the live config at apply time.
- **Cost a route**: each model carries `priceInPerM`/`priceOutPerM`/`contextLength` with `priceSource` (provider-depot source + `fetchedAt`). Depot canon: a price without a source is not a price; rows marked `unmatched in depot catalog` are unpriced, do not invent a number.
- **Schedule around quota**: OpenRouter `:free` lanes die at ~1000 req/day; MiniMax serving is plan-gated; NIM has no surfaced quota. Check `providers[].budget` before pinning a lane onto a provider.
- **Refresh**: `node build-capture.mjs --probe <fresh manifest>` after any re-probe; the depot must pass `depot.mjs check` first (run `depot.mjs refresh` if stale).

## Provenance chain

1. `ListModels` (session ground truth, 2026-11-20) — configured provider/model ids + reasoning levels.
2. `~/.zcode/v2/provider_config.json` `$.config.providerConfigRules.providerRules` — the ZCode providerRules shape (redacted).
3. `provider-depot catalog.json` refreshed `2026-11-20T18:06:02.998Z` (gate green, 7 sources) — pricing/context/capabilities with per-source URLs.
4. Live HTTP probes 2026-11-20 — status + budget signals, raw bodies in `capture-20261120/raw/`.

## Known stale data (do not trust)

- `~/.zcode/v2/coding-plan-cache.json`: says `zai-start-plan` not entitled — timestamp ≈2026-06-22, contradicted by runtime. Ignored.
- `~/.zcode/v2/bots-model-cache.v2.json`: builtin zai entries say disabled/not-authenticated — same stale vintage. Provenance only.

## Gaps (honest)

- Zai Start Plan REST auth needs a fresh api-key (stored one expired) — quota state unverifiable until then.
- MiniMax exposes no usage endpoint that answers (GET `/v1/usage` → 404); quota state is from the in-band 2056 error only.
- NIM: no rate-limit/credit headers or endpoint surfaced; budget assessment is "free tier, unverified limit".
