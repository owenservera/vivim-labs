# ZFree Deep Research & Refinement Record — 2026-11-11

**Disposition:** Locally exercised research prototype, not verified on the owner's Windows/ZCode instance. Code lives only at `vivim-labs/research-intake/zcode-free-model-toolkit`. No VIVIM Host, HQ, TierFlow or owner browser/profile changes. This report supersedes the earlier snapshot [AUDIT-2026-11-11.md](AUDIT-2026-11-11.md) for current test status.

## Era 1 — Pinned-source research

| Surface | Specific evidence | Engineering consequence |
|---|---|---|
| ZCode release | [v3.17.2](https://github.com/zai-org/ZCode/releases/tag/v3.17.2) (published Nov 4, 2026; release source `e6b675b40abb79e1d38d98d4e8750d0859cfedd9`) | Do not assume moving `main` is the owner's installed build |
| ZCode provider schema | [tagged `provider-data-schema.ts`](https://github.com/zai-org/ZCode/blob/v3.17.2/packages/provider/src/config/provider-data-schema.ts), GitHub blob `273bd346ecfc8c4329cd53efcad0f477204fa094` | `openai-chat-completions` is supported; `openai-responses` and `anthropic-messages` are **not implemented** by ZFree |
| ZCode streaming boundary | [tagged stream contract](https://github.com/zai-org/ZCode/blob/v3.17.2/docs/model-tool-call-stream-commit-boundary.md), blob `aa2e79bd3f7f651a0b8f8affeb325de788b2f9c3` | Preserve raw SSE tool deltas; allow ZCode to assemble final function calls, don't synthesize them in proxy |
| Context7 | `/zai-org/zcode`, queried Nov 11 | Confirms versioned provider config and model option-to-request mapping; no supported automated ZCode config writer identified |
| OpenRouter catalog | [live GET /api/v1/models](https://openrouter.ai/api/v1/models) Nov 11; contains `pricing`, `architecture.input_modalities`, `supported_parameters`, `top_provider.max_completion_tokens` | Price/capability check model-by-model and refresh; `max_completion_tokens` alias is not always advertised |
| OpenRouter rates | [free collection](https://openrouter.ai/collections/free-models/) | Conservative unfunded account: 20 requests/minute, 50/day; failed attempts count. Funded accounts may receive 1000/day. Local ledger is not provider-account truth |
| NVIDIA | [Developer Program terms](https://docs.api.nvidia.com/nim/docs/product), [NIM API ref](https://docs.nvidia.com/nim/large-language-models/latest/reference/api-reference.html) | Trial/prototype access only; `GET /v1/models` is not proof of chat tools/vision features nor quota/billing state |
| OpenCode | [native server provider endpoint](https://opencode.ai/docs/server/), [free-model client restriction](https://opencode.ai/docs/console/models) | Read-only native free-model inventory added; free OpenCode chat requests cannot be relayed from ZCode without an authorized provider path |

### Known provider/agent issue corpus

- [#703](https://github.com/zai-org/feedback/issues/703) and [#1296](https://github.com/zai-org/feedback/issues/1296): conflicting ZCode wildcard reasoning fields. **Mitigated** by normalizing `reasoning`/`reasoning_effort` and removing `thinking` aliases; no live installation replay.
- [#1454](https://github.com/zai-org/feedback/issues/1454): malformed streamed tool-call function names on ZCode 3.17.2. **Not repaired** by transparent proxy; requires provider response fixture/owner acceptance.
- [#623](https://github.com/zai-org/feedback/issues/623): custom-model picker/registry mismatch. **Cannot be fixed by a gateway** when ZCode rejects selection before HTTP.
- [#1446](https://github.com/zai-org/feedback/issues/1446): Windows ZCode/TLS stream crash. Gateway prevents double HTTP replies after stream starts; doesn't eliminate ZCode's own process crash.

## Era 2 — Implemented corrections

| Component | Previous risk | Current defense |
|---|---|---|
| Model eligibility | `:free` label could hide request/image/web-search fees | Require actual numeric zero for every published price field; require text output |
| Cost leakage | Provider plugins and server-side tools could add charges to free model calls | Reject plugins, web-search options and non-client function tools before any upstream request |
| Tool support | A chat-capable model might silently ignore ZCode function calls | Require catalog-declared `tools`; resumed tool messages also enforce tooling |
| Image/structured output | Text model might receive images or JSON constraints it cannot honor | Filter by `input_modalities` and advertised response-format support |
| Reasoning | `reasoning` object + `reasoning_effort` string collision | Emit one supported canonical OpenRouter field, or omit unsupported effort |
| Output limit | ZCode emits `max_completion_tokens` even when model advertises only `max_tokens` | One matching token field and top-provider cap |
| Streaming | Attempting another HTTP response after headers | Close incomplete stream; never write a second response |
| Local security | Unauthenticated loopback gateway | Require local bearer token; optional Windows DPAPI user-bound secret storage |
| Model churn | Switching free model during active tool histories | Best-effort session fingerprint and in-memory model pin, explicit 409 if pinned model unavailable |
| Retry exhaustion | Repeated 429/403/5xx on same free endpoint | Bounded attempts, HTTP error classification, per-model cooldown; local conservative quota ledger |
| NVIDIA NIM | Every `/models` entry treated as chat/tool-capable | Exclude known non-chat IDs; explicit tool/vision allowlists, trial opt-in |
| OpenCode | Unsupported third-party free chat route risk | Read-only native `GET /provider` zero-cost inventory, never ZCode proxy |

**Caveat:** No dynamic trusted tokenizer, model-quality benchmark, verified prompt persistence, or per-account billing/entitlement meter is implemented. Even when pricing=0, free capacity and endpoint correctness are subject to upstream outages/region restrictions.

## Era 3 — Executable evidence and open release gates

### Local tests

Command used:

```bash
cd /mnt/data/zfree_audit
python -m compileall -q .
python -m unittest discover -s . -p 'test_*.py' -v
```

Observed on Python 3.13.5 in a Linux container: **22 passed / 0 failures**, around five seconds; includes actual loopback HTTP gateway→simulated upstream requests with status 200, SSE `[DONE]`, tool history, normalized parameters, key rejection, paid add-on rejection, 429/quota persistence and provider-specific errors. This exercises a source-equivalent local reconstruction of the committed gateway plus tested policy and native inventory modules. The GitHub test files are committed; an independent runner against the just-committed remote bytes has **not** been executed, and GitHub Actions are intentionally not enabled.

### Gates remaining explicitly UNRUN

1. Owner PC Python/PowerShell and Task Scheduler/DPAPI acceptance, including startup after login and recovery after sleep.
2. Installed ZCode 3.17.2 custom provider picker, request body, reasoning setting, SSE tool-call completion and persisted selection.
3. Live authenticated OpenRouter free request, actual model availability, 429 behavior and usage receipt; no secrets collected in this audit.
4. Live NVIDIA trial model-specific chat and tool probes and quota/account terms. No zero-billing assertion.
5. Local OpenCode server running with provider auth; native scanner only tested with fake catalog fixtures.
6. Large contexts, multi-modal messages, tool-call argument fragments, malformed SSE and actual agent session compaction.

**Acceptance rule:** These gates must pass before calling the toolkit fully wired, unattended or device-verified. If any gate fails, preserve a redacted reproduction fixture and exact local ZCode version. Do not publish private keys, raw prompts, session profiles or transcripts to Labs.

## Commit provenance / execution

GitHub connector updates committed the independent `policy.py`, `zfree.py`, `opencode_native.py`, test suites, `setup-windows.ps1`, `start-windows.ps1`, and `.gitignore`. No branch force update, history rewrite or unrelated file mutation. The Labs repository retains evidence-only scope and may not be treated as production runtime authority.
