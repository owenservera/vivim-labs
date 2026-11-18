# NIM Model Routing Guidebook — ARBOR workflow fleet

Date: 2026-11-16. Scope: the 10 models this install lists under the NVIDIA NIM provider (`new-provider/…`), plus measured local behavior. Purpose: pick the right model per task type in the dynamic-workflow fleet (`.zcode/workflows/*.dwf.ts`), and know the failure modes before they cost a run.

Sources: 22 raw pulls saved in [`raw/`](raw/index.json) (context7 + WebFetch, each with libraryId/query/URLs; no credentials — provider config pull is masked). Local measurements: `~/.zcode/cli/db/db.sqlite` table `model_usage` (readonly; `model_id` stored WITHOUT provider prefix; `started_at` epoch ms). Claim tiers: **[official-doc]**, **[model-card]**, **[benchmark]**, **[local-measured]**, [unverified].

Owner canon note: context7 pulls are destined for `github.com/owenservera/SOFTWARE_HQ` `context7/`; no local SOFTWARE_HQ checkout exists here, so they live in `raw/` until then.

---

## 1. Per-model profiles

| Model id (`new-provider/…`) | Params | Context | Modalities | License | Local 24h (calls / avg TTFT / errors) | Verdict |
|---|---|---|---|---|---|---|
| `nvidia/nemotron-3.5-super-vl-120b-a12b` | 120B total / 12B active, hybrid Mamba-Transformer LatentMoE [model-card] | 1M [model-card] | text+image+video in, text out (no audio) [official-doc] | NVIDIA Nemotron Open Model License [model-card] | **1344 / 2.36s / 6** [local-measured] | **WORKHORSE** — fast, huge throughput, healthy |
| `nvidia/nemotron-3-ultra-550b-a55b` | 550B total / 55B active MoE [model-card] | 1M [model-card] | text (agentic/tool-optimized) [model-card] | Nemotron Open Model License [model-card] | not yet used at scale | **LEAD** — best agentic/SWE/long-context (MMLU-Pro 83.73, AIME25 90.21, strong SWE-Bench, RULER@1M) [benchmark] |
| `nvidia/nemotron-3.5-lightning-30b-a3b` | 30B-A3B class, speed-optimized for long-running agents [official-doc] | model-family 1M [model-card] | text | OpenMDW 1.1 [official-doc] | not yet used | fast workhorse alt |
| `nvidia/nemotron-3-nano-omni-30b-a3b-reasoning` | 30B-A3B omni, reasoning model [model-card] | family 1M | text+image (omni) [official-doc] | Nemotron Open Model License [model-card] | not yet used | reasoning-heavy alt; toggle via `chat_template_kwargs {"enable_thinking": false}` (kwarg name mandatory — `thinking` is silently ignored) or `/no_think` [official-doc] |
| `nvidia/nemotron-3.5-bolt-e4b` | E4B small/fast [unverified — repo search found no card] | — | text | — | not yet used | cheap router alt, verify before trusting |
| `poolside/laguna-xs-2.1` | xs coding model [official-doc] | — | text | Poolside terms [official-doc] | not yet used | cheap coding-specific parcel |
| `minimaxai/minimax-m3.1-flash` | flash text chat [official-doc] | — | text | MiniMax terms | **14 / 11.3s / 8 of 14 errored** [local-measured] | **avoid in fleet** — slow + flaky locally; probes only |
| `stepfun-ai/step-5-preview` | Step-5 preview, tool-calling supported [official-doc] | — | text | StepFun terms [official-doc] | not yet used | backup general |
| `mistralai/mistral-large-4-1t-a52b` | MoE, multimodal, open-weight preview [model-card] | **1M** [model-card] | text+image, reasoning + `json_schema` guaranteed-shape output [model-card] | open weights (preview) [model-card] | **159 / 3.9s / 4** [local-measured] | **main session model + strategic/synthesis** — healthy |
| `meta/muse-glimmer-30b` | 30B [official-doc page exists] | — | text (VL family page) | — | not yet used | [unverified] cheap general alt |

NIM API facts that shape routing [official-doc]:
- Hosted endpoint `integrate.api.nvidia.com/v1`, per-model reference pages carry RPM/TPM rate limits per key [official-doc].
- Reasoning on Nemotron is ON by default; control via `reasoning_effort` (`low|medium|high`) or `chat_template_kwargs`; too-small `max_tokens` with reasoning → `finish_reason: "length"` + empty content [official-doc].
- **`json_object` only guarantees valid JSON, not your shape** — the model can return `{}` or drop fields; use `json_schema` with `required` for guaranteed structure [official-doc]. This is the upstream explanation for our typed-ask failures (see §4).
- Forced `tool_choice: "required"` on a no-parameter tool can invent argument keys (vLLM quirk) — use `"auto"` for parameterless tools [official-doc].

---

## 2. Routing table per task type (what the fleet should use)

| Task type | Primary | Fallback | Rationale |
|---|---|---|---|
| LEAD / judgment (designer, brief, adjudicator, batch-exit gate) | `nvidia/nemotron-3-ultra-550b-a55b` | `nemotron-3.5-super-vl-120b-a12b` | Ultra is the agentic/tool/SWE-strongest Nemotron [benchmark]; Super-VL proven locally at 2.4s TTFT [local-measured] |
| WORKHORSE scoped build/edit (builders, fixers, surveyors) | `nemotron-3.5-super-vl-120b-a12b` | `nemotron-3.5-lightning-30b-a3b` → `nemotron-3.5-bolt-e4b` | 1344 calls at 2.36s TTFT, 6 errors/24h [local-measured]; Lightning/Bolt for speed when Super-VL is hot |
| Read-only survey / investigate (no writes) | `nemotron-3.5-super-vl-120b-a12b` | `nemotron-3-nano-omni-30b-a3b-reasoning` | cheap enough to fan out; Nano-Omni when reasoning needed |
| Evidence-runner verifier (honest pass/fail on receipts) | `nemotron-3.5-super-vl-120b-a12b` | `nemotron-3.5-lightning-30b-a3b` | determinism matters more than size; the runner script decides, the model judges |
| Cheap router / classifier / name-picker | `nemotron-3.5-bolt-e4b` | `poolside/laguna-xs-2.1` | smallest models; verify Bolt first [unverified] |
| Strategic / long-context / synthesis (bridge, PDR, roadmap synthesis) | `mistralai/mistral-large-4-1t-a52b` (session model) | `nemotron-3-ultra-550b-a55b` | 1M context + guaranteed `json_schema` [model-card]; 3.9s TTFT, 4 errors/24h [local-measured] |
| Main session (manager) model | `mistralai/mistral-large-4-1t-a52b` | — | current session model, healthy [local-measured] |
| Image/vision judgment | **none on NIM subagents** | — | NIM subagent Read omits image media ("model does not support image input"); visual checks must be programmatic (PNG validity, IHDR dims, pixel stats) and reported `notCovered` visually |
| Provider verification probes only | `minimaxai/minimax-m3.1-flash` | — | flaky locally (8/14 errors) [local-measured]; never route fleet work here |

Contrast data point [local-measured]: the same Super-VL weights on the OpenRouter `:free` lane run 17s TTFT / 73 errors vs 2.4s / 6 on NIM — **NIM is the healthy lane for Nemotron today**; OpenRouter free is the rate-limited one.

---

## 3. Live ladder rules (as implemented in `.zcode/workflows/*.dwf.ts`)

- Health probe at run start: `model_usage` errors (`rate_limited|provider_overloaded|server_error|timeout`) per model in the last 10 min; **≥5 rows → skip that model** (bun:sqlite readonly).
- `model()` takes **string literals only** (compile-checked); runtime picks index a pre-declared `ModelRef` table (`MODEL_IDS` + `WORKHORSE_LADDER` / `LEAD_LADDER`).
- Role aliases: `ERNIE_HIGH = LEAD`, `NEMOTRON = WORKHORSE` (legacy names kept so existing ask sites compile).
- `Date.now()`, `fetch`, `fs`, `Math.random()` are banned in workflow scripts (replay safety). `world.run` command names are compile-time literals; values go in the args array.

## 4. Measured failure modes (2026-11-16) and workarounds

| Failure | Cause | Workaround (applied in saved defs) |
|---|---|---|
| Typed `ask<T>` fails: `submit_result` "expected object, provided string" | Harness ingress stringifies payloads on NIM subagents; upstream, `json_object` doesn't guarantee shape [official-doc] | Untyped `ask<string>` + `JSON.parse` of the braces span, with ONE strict retry ("return ONLY the JSON"), via AmendWorkflow |
| `DriverError: typed ask ended without submit_result` | Oversized interpolated prompts on throttled models | Trim interpolated payloads (`.slice(0, 9000)` etc.) at mapper/finalizer asks |
| Duplicate `agent("Repair builder N")` kills the run | Names are cache identities; two repair loops reused the same names | Unique names per loop (`Gap repair builder N` / `Failure repair builder N`) |
| `bun -e CODE -- args` flip reads wrong argv | `process.argv` = `[bun.exe, ...args]` — ONE leading entry, `--` stripped | `process.argv.slice(1)` |
| Subagent pinned to `new-provider-3/*` dies `ECONNREFUSED 127.0.0.1:31415` | Local FREEllmAPI proxy down | Never pin subagents there; session model or NIM-pinned defs only |
| Screenshot pre-screeners can't see PNGs | NIM subagent models have no image input | Programmatic checks only; verdict `UNVERIFIED-VISUAL`; run reports visual as `notCovered` |
| AmendWorkflow drops saved args | Amended runs lose `args.*` (e.g. verify items list) | Pass args at CreateWorkflow; make verify-mode logic not depend on args (skip dependsOn validation in verify mode) |

## 5. Adding / evaluating a new NIM model

1. `ListModels` for the exact id; check the NIM support matrix [official-doc] for the profile.
2. Bounded probe run on a scratch workflow; read `model_usage` (last 10 min errors, TTFT).
3. If healthy (<5 errors/10 min, sane TTFT): add a `model("new-provider/…")` literal to the def's table, insert into `WORKHORSE_LADDER`/`LEAD_LADDER`, re-save via SaveWorkflow, launch a small run.
4. Record the pull in `raw/` + `index.json` (owner canon).

## Appendix: raw pulls

See [`raw/index.json`](raw/index.json) — 22 pulls: NIM API/auth/rate-limits, tool-calling & structured-output known issues, reasoning control, Nemotron family specs/benchmarks/licenses, per-vendor model cards (Mistral, Poolside, MiniMax, StepFun), dead ends, and local `model_usage` aggregates + masked provider config.
