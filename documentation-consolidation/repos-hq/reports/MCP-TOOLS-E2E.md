# MCP Team Tools — End-to-End Verification

**Target machine:** `C:\0-BlackBoxProject-0\repos_hq`
**Server:** real stdio MCP server (`bun hq/bin/hq.mjs mcp`), JSON-RPC 2.0 over stdio
**Database:** `.hq/index.db` (real, read through `openWorkspace`)
**Date:** 2026-11-04

## Method

Opened the MCP server over stdio and sent four JSON-RPC messages in one session:

1. `initialize` (protocolVersion `2025-11-25`)
2. `tools/list`
3. `tools/call` `team_state` with `{}`
4. `tools/call` `route_model` with `{ apply: false }`

No `wake_seat` call was made (read-only lane; wake was confirmed by code reading only). No files were edited.

---

## 1. `team_state` — called through the real stdio server

**Result: PASS**

The call returned through `tools/call` id=3 and rendered a full team panel:

- **AGENTS (32)** — 32 agent definitions listed with model pin, think level, tools, protocol, ctx7. Example rows: `agent-platform-ceo model=minimax-m-plan/MiniMax-M3.1-Flash`, `channel-manager model=deepseek/deepseek-flash`, `nvidia-probe model=new-provider/nvidia/nemotron-3.5-lightning-30b-a3b`, plus 9 agents on `model=inherit`.
- **SEATS (3)** — `chatgpt ACTIVE 3m`, `gemini IDLE`, `claude IDLE` (from `C:/VIVIM/channel/seats.json`).
- **SESSIONS (16)** — live session list with age, message count, title.
- **TASKS** — sessions=23 open=74 blocked=28 learnings=21 defects=0.
- **PROVIDERS (last 5h)** — provider health rows **including capped and budget-blocked rows**:
  - `minimax-m-plan/MiniMax-M3 budget-blocked ok=741 err=7 11537ms`
  - `openrouter/nvidia/nemotron-3.5-super-vl:free capped ok=213 err=21 17223ms`
  - `openrouter/google/gemma-4-31b-it:free budget-blocked ok=0 err=4 0ms`
  - `openrouter/recursive/flint-v1.0:free broken ok=0 err=1 0ms`
  - `new-provider/mistralai/mistral-large-4-1t-a52b broken ok=0 err=3 0ms`
  - plus healthy rows (deepseek, grok-build-oauth/grok-4.7, new-provider/nvidia/nemotron-3.5-super-vl-120b-a12b, ...).
- **DEFECTS (21)** — thought-level defects, 12 shown + "9 more (use --verbose)".
- **NEXT MOVES (26)** — fix-grant moves, 12 shown + "14 more (use --verbose)".

**Read-only confirmation:** `team_state` only reads the agent directory, seats registry, session store, and usage DB. No file is written. The handler `handleTeamState` (mcp-team-tools.mjs:459-465) calls `buildTeamSnapshot` (read-only) and `renderTeamState`. Nothing on disk changes.

---

## 2. `route_model` with `apply:false` — called through the real stdio server

**Result: PASS**

The call returned through `tools/call` id=4 and rendered:

> ROUTE MODEL 8 route(s) in 1 group(s), 24 skipped, window 5h
>
> -> new-provider/nvidia/nemotron-3.5-lightning-30b-a3b (8 agents)
>    from: inherit
>    why: inherits the session model — ... free, healthy, $0.0000 per call, 4.2s avg
>    beat: openrouter/thinkingmachines/inkling:free (budget blocked ...), openrouter/nvidia/nemotron-3.5-super-vl:free (capped ...), ...
>    WARNING: 8 agents on one model is a single point of failure ...
>    agents: active-task-scanner, atlas, forge, hr, keeper, lens, scout, weaver
>
> SKIPPED
>   22 x healthy pin kept (minimax-m-plan/MiniMax-M3.1-Flash) — set optimizeHealthy to revisit
>   1 x healthy pin kept (deepseek/deepseek-flash) — channel-manager
>   1 x already on the best route (new-provider/nvidia/nemotron-3.5-lightning-30b-a3b) — nvidia-probe
>
> PROPOSED ONLY — nothing was written. Call route_model again with apply: true to write these pins.

- **Route groups:** 1 group (destination `new-provider/nvidia/nemotron-3.5-lightning-30b-a3b`).
- **Skipped agents:** 24 (22 healthy pins kept, 1 channel-manager, 1 nvidia-probe already on best route).
- **Nothing written confirmation:** the rendered text explicitly says "PROPOSED ONLY — nothing was written." The `apply` write branch in `defaultPlanRoutes` (mcp-team-tools.mjs:608-622) is gated on `input?.apply === true`, which was false, so no agent file was rewritten and no `.bak-route-<stamp>` backup was created.

**Post-call disk check:** `find /c/Users/VIVIM.inc/.zcode/agents -name '*.bak-route-*'` returned **no files**. Zero `.bak-route` files exist before or after the call.

---

## 3. `wake_seat` — confirmed by code reading only (NOT invoked)

**Result: PASS (code-confirmed)**

No `wake_seat` call was made, per the read-only mandate. Confirmed from source:

- **Refuses an ACTIVE seat unless `force: true`:** `handleWakeSeat` (mcp-team-tools.mjs:474-500) resolves the seat state; `if (state === 'ACTIVE' && !force)` returns a `renderToolError` refusing the wake ("seat "…" is ACTIVE, so the wake is deferred … or call wake_seat again with force: true if you have decided it is stuck."). The same guard exists in `guardWake` (channel-seats.mjs:152-166): `if (state === 'ACTIVE' && !force)` returns `{ decision: 'DEFERRED', code: 'SEAT_ACTIVE', … }`. An IDLE seat proceeds to `wakeSeatChannel`; an unknown seat is denied with `SEAT_UNKNOWN` and the roster printed.
- **WAKE_PROOF_RULE attached:** `WAKE_PROOF_RULE` is defined (mcp-team-tools.mjs:58-59): `'A wake is proven by the seat\'s own reply, not by an acknowledgement.'` It is appended to the ACTIVE-refusal error (line 485), to other non-ACCEPTED error paths (line 497), and to every rendered wake result via `renderWakeResult` (line 450). So the rule is attached in every wake code path.

**Not invoked:** no intent journal, no transport send, no seat turn was started.

---

## 4. All 3 tool names appear in `tools/list`

**Result: PASS**

`tools/list` (id=2) returns the full `TOOLS` array which spreads `...TEAM_TOOLS` at mcp.mjs:161. The three names are present with full input schemas:

- `team_state` — `sections` (array of agents/seats/sessions/tasks/providers/defects/moves), `verbose` (boolean).
- `route_model` — `apply` (boolean, default false), `hours` (number, default 5), `agent` (string).
- `wake_seat` — `seat` (string, required), `prompt` (string, required), `wait` (number, default 200), `force` (boolean, default false).

All three are registered in `HANDLERS` (mcp-team-tools.mjs:519-523) and routed in `callTool` (mcp.mjs:287-291).

---

## 5. Validation gaps

**Result: one gap remains**

The `route_model` **`apply:true` path was NOT exercised**, because it writes to disk: it rewrites the `model:` line of each affected agent definition and leaves a timestamped `.bak-route-<stamp>` backup beside it (mcp-team-tools.mjs:608-622). Exercising it would mutate agent definitions, which is outside this read-only verification lane's scope and was explicitly not done.

All other behavior was exercised through the real stdio server against the real machine (real agent definitions, real seats registry `C:/VIVIM/channel/seats.json`, real `.hq/index.db`, real usage rows). No validation gap for `team_state` or `wake_seat` beyond `wake_seat` not being invoked (by design).

---

## Summary

| Tool | How verified | Result |
|------|--------------|--------|
| `team_state` | Real stdio `tools/call` against real DB/seats | PASS — 32 agents, 3 seats (chatgpt ACTIVE, gemini IDLE, claude IDLE), capped/budget-blocked provider rows printed, read-only |
| `route_model` (apply:false) | Real stdio `tools/call`; no `.bak-route` files before/after | PASS — 8 routes in 1 group, 24 skipped, "PROPOSED ONLY — nothing was written" |
| `wake_seat` | Code reading only; not invoked | PASS — ACTIVE seat refused unless force:true; WAKE_PROOF_RULE attached in all paths |
| `tools/list` contains all 3 names | Real stdio `tools/list` | PASS |

**Validation gap remaining:** `route_model apply:true` (write path) not exercised, by design.
