# ChatGPT architecture-question queue (oracle-owned; entry-point protocol)

Lanes: drop questions here, one block each. Do NOT contact ChatGPT directly.
Oracle batches + forwards via manager → chatgpt-keeper → `bun harness/bin/wh.mjs ask chatgpt` (PROVEN 2026-11-14 bfe2b9; studio broken, do not use). Read-only; owner-gated writes; no asks while owner sleeps — batching stands for wake-time.

Format per block:
- from: <lane>
- question: <one paragraph>
- why-strategic: <one line>
- status: queued | forwarded | answered (answer below the block)

---

## 2026-11-17 — arbor-life strategy check (entry: manager, via oracle; owner-gated)

Context: first ARBOR LIFE lineage complete — autonomous loop (USE→OBSERVE→DIAGNOSE→FIX→VERIFY→EVOLVE) verified A00/A01/A02 green (gate 0, evidence static+live 0, fresh-eyes replays, keeper handoffs). Models: zai GLM-5.3 retired (quota 1005); ernie-5.5 leads; NIM mistral-large healthy-but-slow (37s avg gen); minimax/step-5/inclusion untested; browser journeys via native control-browser work in-subagent.

Questions for ChatGPT:
1. Loop pacing: per-round wall time is ~20-40 min (NIM generation-bound + rate-limit stalls). Better topology for ≤5-min cycles: smaller per-round fixer fan-out, journey caching, or a different phase split?
2. Verifier design: is "fresh-eyes retest journey must reproduce the fix" sufficient as independent verification for queue flips, or what additional adversarial check would you require?
3. Feeder: dedup currently reads queue+receipts+journal. What dedup key would you trust for autonomous continuous operation across restarts?
4. With ernie as lead and NIM mistral as workhorse, what model-role split do you recommend for journey vs diagnose vs fix vs verify stages?

Status: QUEUED for owner approval — not sent. Route: manager → oracle → chatgpt-keeper → ChatGPT.
