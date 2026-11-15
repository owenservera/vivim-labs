# OpenRouter free-first model routing — current recommendations, never static authority

As checked November 15, 2026, free variants include:

| Role | Preferred zero-price exact OpenRouter ID | Fallback | Caution |
|---|---|---|---|
| Planning / independent reasoning | `nvidia/nemotron-3-ultra:free` | `baidu/ernie-5.5-flash-preview:free` | long context ≠ guaranteed coding quality |
| Coding implementation | `particle-ai/particle-1:free` | `cohere/north-mini-code:free` | test actual ZCode tools/tool-call quality |
| Reviewer / second opinion | `baidu/ernie-5.5-flash-preview:free` | `nvidia/nemotron-3-ultra:free` | avoid reviewing solely with author model |
| Fast triage / short tasks | `nvidia/nemotron-3.5-lightning:free` | `cohere/north-mini-code:free` | lower cost doesn't imply reliable refactors |

`nvidia/nemotron-3.5-super-vl:free` can be candidate for GUI/image reviews if user permits screenshot analysis and local ZCode supports images. Do not send private UI/content to OpenRouter without explicit authorization. Current free roster and descriptions: https://openrouter.ai/collections/free-models ; exact model pages: https://openrouter.ai/baidu/ernie-5.5-flash-preview%3Afree , https://openrouter.ai/particle-ai/particle-1%3Afree , https://openrouter.ai/cohere/north-mini-code%3Afree .

## Mandatory catalog gates

1. `node scripts/discover-models.mjs --out .local-models.json` fetches OpenRouter public model metadata. The router accepts **only** models whose ID ends with `:free` AND whose prompt/completion pricing fields are both numeric zero. A zero-priced paid ID does not count under strict policy.
2. Required tool calling and text-in/text-out: reject incompatible modality and tool metadata when explicitly declared; uncertain metadata is reported, and must be confirmed by a live ZCode compilation/smoke run.
3. Candidate selection is deterministic with a documented priority order. No auto-switch to `openrouter/free` roulette for a production agent: it cannot pin identity/provenance, and its match may have different tool semantics.
4. ZCode requires a configured provider catalog. Provider ID is **not** the OpenRouter model ID. The renderer uses `--provider-id openrouter` by default; confirm the actual ID in installed ZCode before workflow authoring.
5. Model strings are **fixed at workflow compile time**. No runtime arbitrary model ID selection is valid in ZCode's compiled facade; the renderer inserts only validated literal choices.
6. Quotas: OpenRouter currently states free `:free` variants capped at ~20/min and 50/day unless the account has qualified for a higher daily cap; failed attempts may count. The local runtime must monitor usage, stop on sustained rate limiting, and never assume uninterrupted free service.
7. Free requests still leave the local device. Do not upload personal chats, secrets, private corpora, tokens or sensitive repository content without authorization. Keep a redaction/scope rule in asks and avoid exposing tool outputs beyond the authorized repo.

## Model-quality evaluation

Before setting a permanent route, benchmark every candidate on exact task/fixture test pass rate, ZCode tool success, structured-result failure, speed, requests/repair, and refusal/retry rate. OpenRouter usage ranking is **not** accuracy ranking. Never interpret an advertised model benchmark as proven performance for this project's environment.