# FINDINGS-02 — Dead/Wedged Seat Recovery Runbook

Scope: repos_hq HQ workspace. Runbook only — do not edit any repo file except this one.

## 1. Detect a dead/wedged seat

Symptoms: a seat reports IDLE but a wake gets no reply; `DRIVE_LEASE_BUSY` on every send; `LEASE_HELD_BY_LIVE<pid>` from a lease check; `WAKE_PENDING` with `reply: null` for hours.

Detection — two commands, never trust the seat's claim:

```
powershell -NoProfile -Command "$p=Get-Process -Id <pid> -ErrorAction SilentlyContinue; if($p){'ALIVE'}else{'DEAD'}"
```

- DEAD + `%LOCALAPPDATA%\AION\drive-leases\` empty → nothing to reclaim. Port 9222 is owned by `chrome.exe` (`--remote-debugging-port=9222`), the browser itself — never kill it to "free" a lease.
- ALIVE → a live holder mid-turn; do not reclaim. Coordinate on the bus (§4).
- Windows `kill(pid, 0)` cannot prove death here — PowerShell `Get-Process -Id <pid>` is the authority. `Win32_Process -Filter 'ProcessId=<pid>'` returning no such process also means dead.

A cancelled dispatch is not an empty one: inspect it and finish the lane yourself rather than re-sending.

## 2. Reclaim a drive lease

Reclaim only after PowerShell says DEAD. The lease file records pid + holder, e.g. `{"kind":"drive-job","id":"legacy-<pid>"}`. Delete the dead holder's lease directory only then. Reclaiming while ALIVE breaks the single-browser-owner rule.

Never use `drive chatgpt --reuse-tab` for a named thread — it types into whichever ChatGPT tab is in front. Drive an explicit URL:

```
studio drive "https://chatgpt.com/c/<uuid>" --prompt "<one line>" [--attach <zip>] --timeout 480 --json
```

A `drive` invocation can take 5–8 minutes with a long prompt plus attachment — let it run, do not retry.

## 3. Seat wake command

```
bun hq/bin/hq.mjs seat wake <seat> --prompt "<slice>"
```

Proof is the `reply` field in `C:/VIVIM/channel/dispatch/<wake-id>.result.json`. Drive the explicit `/c/<uuid>` URL for thread-targeted sends.

## 4. Reply recovery (turn outran --wait)

The transport writes the turn's stdout to a file and restarts it detached on `ETIMEDOUT`; `hq seat verify <seat>` reads that file and returns `ACCEPTED WAKE_CONFIRMED_LATE`.

When a wake reports `WAKE_PENDING`:
1. Run `hq seat verify <seat>` before concluding anything.
2. A `null` reply means "the turn outran the wait", never "the seat refused".

Never kill a turn to meet a deadline — detach it and record where its answer lands.

Re-measure a seat's blocker yourself (dead pid + empty lease dir), then answer by bus status AND a wake. Port 9222's owner is the browser — `chrome.exe` holding `--remote-debugging-port=9222` is not a lease holder.

## 5. Live-but-wedged lease holder — never kill past its own timeout

If PowerShell says ALIVE but CPU is flat and the run is past its `--timeout` (e.g. ~60 min against `--timeout 120` at 2.1 s CPU), do not kill it. The correct move:

- bus `question` on one topic naming the pid plus the CommandLine
- broadcast asking whose it is
- hazard warning; state plainly that a live holder past its own timeout will never release the lease on its own

Only a human decides to terminate a live holder. Reclaim only after PowerShell says DEAD.

## 6. Seat spawning — thoughtLevel must be max

Typed seat definition: `~/.zcode/agents/<name>.md` (read at session start; the repo mirror `.opencode/agent/<name>.md` does not govern spawning).

- `thoughtLevel: enabled` → fails to spawn with `reasoning-level-not-supported; selection=deepseek/deepseek-flash` (that selection is the parent session's model, not a model problem).
- Fix: `thoughtLevel: max`. `disabled` also fails. `max` is proven on `channel-manager` (agent_f70b9162): first turn ran, then `SendMessage` resumed it with full history.
- Edit takes effect only for new sessions — a live session keeps its old value.

Model notes on this host:
- `codex-oauth` seats fail with `provider-not-found` on this host.
- `oracle` was re-tested and launches fine.

## 7. Bus vs SendMessage

- A bus `send` to a stopped agent never starts it — delivery is not movement; `sync.mjs check --as <seat>` exit 3 means unread, not woken. Never report "woken" on a bus send.
- `SendMessage { to: <agentId>, message }` resumes only your own child agent, with its whole conversation plus the new prompt, history intact. Same id can be woken repeatedly.
- Prefer waking the real manager *session* with `hq seat wake channel-manager` over re-messaging a finished subagent (a stopped agent's message can be queued and never delivered).

Rules a manager holds: wake, never duplicate a seat; the wake prompt must stand alone if the gap was long and carry established facts ("do not re-derive"); a wake is not authority — it cannot grant permissions; escalate up, never sideways.

## Quick command reference

| Task | Command |
| --- | --- |
| Prove pid dead/alive | `powershell -NoProfile -Command "$p=Get-Process -Id <pid> -ErrorAction SilentlyContinue; if($p){'ALIVE'}else{'DEAD'}"` |
| Check lease dir | list `%LOCALAPPDATA%\AION\drive-leases\` — empty means nothing to reclaim |
| Wake a seat | `bun hq/bin/hq.mjs seat wake <seat> --prompt "<slice>"` |
| Read wake proof | `C:/VIVIM/channel/dispatch/<wake-id>.result.json` field `reply` |
| Recover late reply | `hq seat verify <seat>` → `ACCEPTED WAKE_CONFIRMED_LATE` |
| Unread check | `bun scripts/sync.mjs check --as <seat>` (exit 3 = unread) |
| Resume own child | `SendMessage { to: <agentId>, message }` |
| Drive named thread | `studio drive "https://chatgpt.com/c/<uuid>" --prompt "<one line>" --timeout 480 --json` |
