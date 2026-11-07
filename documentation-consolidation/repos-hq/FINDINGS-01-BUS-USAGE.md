# FINDINGS-01: How to use the HQ communication bus

Source of truth: `docs/AGENT-BUS.md`, `docs/SYNC-BUS.md`, `docs/SYNC-BUS-CONTRACT.md`,
`scripts/sync.mjs`, `AGENTS.md` (sections "Comms and agents — start here" and
"Coordination with the VIVIM agent"). Run all commands from the `repos_hq` root:

```
bun scripts/sync.mjs <command> …        # same as: bun run sync <command> …
```

The bus is one append-only JSONL ledger (`sync/ledger.jsonl`) plus a spaces
registry (`sync/spaces.json`), per-agent cursors (`sync/cursors/`) and an
append-only receipt ledger (`sync/receipts.jsonl`). No daemon, no broker, no
network. Every record carries `authority: "coordination_only"`.

## 1. The four channels — when to use each, and the proof

| Channel | Use it for | Starts a turn? | Crosses machines? | Proof it happened |
|---|---|---|---|---|
| **sync bus** (`scripts/sync.mjs send`) | Deliver text to an inbox (running agent or text-only drop) | **No** | No | `check --as <seat>` exits 3 |
| **seat wake** (`hq seat wake`) | Start a real turn in a live ZCode session | **Yes** | No | `reply` in `C:/VIVIM/channel/dispatch/<wake-id>.result.json` + `seat verify` |
| **SendMessage** | Resume a subagent you spawned, with full history intact | **Yes** | No | the subagent answers, in its notification |
| **anet** (`anet delegate`) | A2A task to an agent on another machine | via auto-reply | **Yes** | `anet task get <id>` |

Rule of thumb: **deliver → bus**. **Move → seat wake (session) or SendMessage (subagent). Another machine → anet.**

Command examples:

```sh
# bus delivery (text lands in an inbox; nothing is woken)
bun scripts/sync.mjs send --from hq-agent --to vivim-orchestrator --kind status --topic vivim-feed --body "lane done: rows landed"
bun scripts/sync.mjs log --topic vivim-feed

# seat wake (starts a turn; answer is the proof)
bun hq/bin/hq.mjs seat list                                      # IDLE / ACTIVE + age
bun hq/bin/hq.mjs seat wake <seat> --prompt "<slice>" --wait 200
bun hq/bin/hq.mjs seat wake <seat> --force --prompt "<slice>"     # seat is ACTIVE
bun hq/bin/hq.mjs seat verify <seat>

# SendMessage (resumes a subagent with history)
Agent(subagent_type: "<seat>", prompt: "<self-contained order>")   → agentId
SendMessage(to: agentId, message: "<follow-up>", summary: "…")     → resumes

# anet (another machine)
anet delegate <aid> "<goal>"                                       → anet task get <id>
```

Who may wake whom: `channel-manager → any seat`; a seat → `channel-manager` only.

## 2. Delivery is not movement

A bus message lands in `sync/messages/<id>.json` (mirrored to `sync/ledger.jsonl`)
and wakes nobody. In the recorded case, three orders landed unread and all three
seats stayed `IDLE`. Use the bus when the seat is already running or you only
need the text to be there. To actually move something, use `seat wake` or
`SendMessage` — never expect the bus itself to start work.

## 3. Verified vs claimed

| Fact | Where it is proven |
|---|---|
| a message was sent | `sync/messages/<id>.json` + `log` |
| a seat ran a turn and answered | `channel/dispatch/<wake-id>.result.json` (`reply`), `seat verify` |
| a provider send happened | `channel/receipts/<id>.json` + its outcome |
| an agent *said* it did something | the agent's own text — a claim, not evidence |

A wake's `accepted: true` is an acknowledgement, never a receipt. If `reply` is
`null`, the turn outlived `--wait` and was never back-filled — that is not a
refusal by the seat.

## 4. Cadence commands

**First act of every turn:** `check --as <your-id>`. Exit 3 means unread — read
the inbox before searching or re-deriving anything.

```sh
bun scripts/sync.mjs check  --as hq-agent      # exit 3 = unread; exit 0 = in sync
bun scripts/sync.mjs inbox  --as hq-agent       # read unread (advances cursor = read receipt)
bun scripts/sync.mjs claims                   # before editing a shared file
bun scripts/sync.mjs send --from hq-agent --to vivim-orchestrator --kind status --topic vivim-feed --body "…"
```

Full per-agent cadence:

1. Turn start — `check --as <me>`; exit 3 → `inbox --as <me>` before editing or searching.
2. Before touching a shared ref — `claims`; if another agent holds it, send a `question` instead of editing.
3. Taking work — `send --kind claim --refs <files> [--ttl MIN]` (default TTL 120 min). A claim intersecting a ref held by another agent is **refused by the bus itself** (exit 1, names holder and expiry).
4. Lane done — `send --kind status` quoting the exact commands run and their results, then `send --kind release` for the claim.
5. Handing a lane over — `handoff --changed … --next … --open … --evidence …`.
6. Confirming receipt — `ack --as <me> --message <id>`. Reading is not acknowledging; only an `ack` record acknowledges.
7. Blocked on another agent — `question`, then `pending` on your next turn.
8. Before a commit — `verify` (must report `N/N valid`).

Kinds: `note`, `handoff`, `claim`, `release`, `question`, `answer`, `status`, `decision`, `ack`.
Routing: `agent` = the agent in the sender's space; `space:agent` = cross-space; `*`
= broadcast to every registered space except the sender. A bare address never
leaves the sender's space.

## 5. Standing rules: bus first, always

A standing rule delivered by wake must **also** go to `*` on the bus with
`topic: agent-bus`. A wake moves only the seats that are running; the bus is
where the rule survives for every session that was idle at that moment.

```sh
bun scripts/sync.mjs send --from <sender> --to '*' --kind status --topic agent-bus --body "<the rule>"
bun scripts/sync.mjs log --topic agent-bus           # durable record of every rule
bun scripts/sync.mjs check --as <identity>           # exit 3 = unread; read before you search
```

Learned the hard way on 2026-11-04: nine sessions were woken with a standing rule
and answered in their own transcripts, and `grep -c 'BUS-ACK' sync/ledger.jsonl`
returned `0`. A tenth session, not running at the time, searched the filesystem
and concluded the rule did not exist — it was never written to the bus.

## 6. No secrets in bodies; coordination is never authority

Bodies, refs and payloads are scanned for keys, tokens, cookies, passwords,
bearer headers and private keys; a match **refuses the send** (never redacts in
place). Do not paraphrase a secret to get past the check. Envelope keys implying
permission (`approve`, `authorized`, `permission`, `grant`, `token`, `secret`,
`credential`, `execute`, `effect`, `browser`, `click`, …) are refused on write and
reported by `verify`. A bus message records what an agent said; it never
authorizes an effect, a send, an approval or a browser action. A receipt is
evidence of an action, never authorization of it.

## 7. Manager steer cadence (background-first execution)

Any work that is not a single read or a bounded edit runs in a background
subagent; the manager never builds while a lane runs — it steers and reconciles
evidence.

- Size every dispatch so it yields a checkpoint inside **3 minutes**; if it
  cannot, split it until it can. Three minutes with nothing steerable is a
  mandate defect — re-scope, do not extend the wait.
- Dispatch in the background, then wake at most every **180 s** (`TaskOutput`
  with `timeout: 180000`, or `bus.mjs watch --seconds 180`).
- Send something on every wake: a direction ("do X next", "stop, Y is wrong") or
  an explicit ack ("on track, continue"). Silence is not a check-in; silence for
  3 minutes means the brief was too vague or too large.
