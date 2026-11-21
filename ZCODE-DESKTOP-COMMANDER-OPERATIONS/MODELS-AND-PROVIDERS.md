# Provider and Model Routing — R002

## Inventory from safe local metadata (NOT authenticated availability)
- `opencode-zen-chat-2`: enabled; 16 model rules. Examples `kimi-k3`, `minimax-m3`, `deepseek-v4-pro`, `glm-5.2`, `glm-5.3-flash-free`, `nemotron-3.5-lightning-free`.
- `new-provider` named NVIDIA NIM: enabled; 73 model rules. Examples `nvidia/nemotron-3-ultra-550b-a55b`, `nvidia/nemotron-3.5-lightning-30b-a3b`, `mistralai/mistral-large-4-1t-a52b`, `deepseek/deepseek-v4.1-flash`.
- `minimax-m-plan`: enabled; one explicit rule `MiniMax-M3.1-Flash`; six ordered model names, including `MiniMax-M3`, `MiniMax-M2.7` and `MiniMax-M2.5`.
- `account:zai-start-plan`: account-backed, two model rules `glm-5.3-flash`, `GLM-5.3`, plus alias/order `GLM-5.3-Flash`.
- `openrouter-2` is separate from `opencode-zen-chat-2`; its status did not have explicit `enabled:true` in the examined config. Default was `openrouter/nvidia/nemotron-3.5-super-vl:free`; do not assume it is still usable.

## Zero-surprise routing contract
Routing input: task type, repo, required tools, context/window constraints, cost ceiling, user-approved provider set, model provenance, desired mode. Routing output: exact `{providerId,modelId,options?}`, why chosen, capability prerequisites, fallback policy, tests.
Never infer free from a model name, or assume `:free` means no quota. Validate provider/model `available`, credentials, availability, quota and actual billing before letting automatic switching spend credits.
The default model is not a permission grant. Do not modify process provider rules or user settings merely to run a test.

## Model selection control
1. `zcode --help` does NOT advertise `--model`; its interactive command `/model [id]` exists.
2. Legacy native protocol defines `session/setModel` (state changing; requires current session ID and model selection schema). V4 `switchModelConfig` command uses a revision-aware command envelope. Inspect exact payload schema before sending.
3. Session creation `v4/command` `createSession` supports `firstInput.modelSelection`; the caller must know the authoritative provider ID and model ID.
4. Built-in subagent model overrides are persisted separately; never edit those silently. User custom agent frontmatter can pin `model: provider/model`, but stale IDs may make spawn fail.
5. Prefer role-based model binding per session over changing global default.

## Proposed policy, unbenchmarked
- Fast metadata/search: proven free or plan-backed OpenCode model with strict tool policy.
- Regular code and source migrations: MiniMax M Plan International if healthy and credits permitted.
- High-level architectural analysis: StartPlan GLM model if healthy and credits permitted.
- Cross-checks and multimodal: NVIDIA NIM where available.
Policy is advisory. No live connectivity tests of all four providers have been performed in R002.

## Probe strategy
P0: read config projection only, exact IDs. P1: native provider list/catalog, no model invocation. P2: one-token/short prompt per approved provider with explicitly chosen model, gather latency and request/receipt ID; stop on billing/quota/permission error. P3: paired coding microbenchmarks, deterministic test evaluation and context limits. P4: failure routing with no silent paid fallback.
Do not use raw API keys, provider configuration dumps, authorization headers or logs containing tokens.
