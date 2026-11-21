# Cross-Session Messaging and ChatGPT Thread Bridge — R002

## Three independent concepts
1. **ZCode native sessions** (persisted `sess_...`): agent turns, child sessions, workflow runs, V4 conversation revisions. Parent-child relation is NOT automatically an inter-agent inbox.
2. **HQ team seats / messages**: existing local `repos_hq` team controller and ZCode `/team` plugin. The `team.md` contract explicitly says "bus delivery is not movement": a bus send delivers text; `hq seat wake` or `SendMessage` starts a turn. Never wake an ACTIVE seat, because it may interrupt its work.
3. **ChatGPT conversation threads**: each ChatGPT thread is its own conversation. A model cannot spontaneously communicate across them. Cross-thread coordination needs an authorized shared transport (remote files, HQ broker, or Studio channel), a wake mechanism, and delivery verification.

## What exists on this machine
- Legacy `~/.zcode/comms/bus.jsonl`, `delivered.jsonl`, `receipts.jsonl`, `projects.json`. Last observed writes: 2026-10-07; NOT proof of current live traffic.
- Observed bus record keys: `id,ts,from,priority,subject,body,prev,hash`. This is NOT the proposed envelope previously in playbook. Do not write into these files unless reader/writer and hash-chain contracts are understood.
- `C:\0-BlackBoxProject-0\repos_hq\hq\bin\hq.mjs` exists; its `/team` ZCode plugin documents `bun hq/bin/hq.mjs team` and `--json`, plus `seat wake`. These have not yet been executed via this bridge.
- `chatgpt-keeper.md` documents VIVIM Capture Studio CLI (`studio.mjs chat chatgpt`), thread keys, `requestId` replay receipts, explicit `--execute --accept-sends` send authorization. This is a separate web-app transport requiring browser-owner discipline; NOT permission to send to ChatGPT in this task.
- ZCode's internal `SendMessage` and `TaskOutput` tools are listed in deployed agent manifest grants. Their actual delivery semantics need an integration test.

## Proposed durable bridge (R003 implementation target)
```
ChatGPT thread A ─┐                            ┌── ZCode parent session
ChatGPT thread B ─┼─ Desktop Commander ─→ HQ broker ─→ wake/SendMessage ─→ child sessions
ChatGPT thread C ─┘      ↑             └── receipts / journal / status
                         └── read-only SITREP and correlated replies
```
Messaging envelope v1 (PROPOSAL, not existing ZCode format):
`message_id`, `correlation_id`, `from_thread`, `to_seat_or_session`, `workspace`, `kind`, `payload_ref`, `created_at`, `expires_at`, `requires_ack`, `approval_scope`.
Keep payloads in file-backed content refs, not inside unbounded bus records. Record `accepted`, `delivered`, `woken`, `executing`, `done|failed|cancelled` separately. Only the target's authenticated response counts as a wake receipt. Replays must be keyed by `message_id`.

## Concurrency and safety
- Lease per target seat; check liveness before wake. TTL + monotonic generation prevent duplicate sends after ambiguous timeouts.
- One authoritative journal, append-only receipts, idempotent consume, bounded retry; no optimistic ACK equals completed.
- Host owns effects/approvals; agents do not mint their own permissions or write directly into production bus as a convenience.
- Never relay provider secrets, session cookies, credentials or entire raw transcripts.
- Local operator may read only folders authorized for the current task. No blind cross-session transcript export.
- Bounded fanout; nonoverlapping file ownership; tests with synthetic **communication envelopes only** and never fake product evidence.

## R003 test ladder
T1 read HQ team panel in JSON and validate keys; T2 read session status through native protocol; T3 create an isolated test message/receipt in a temporary location; T4 validate two independent polling processes, no duplicate delivery; T5 authorized true ZCode two-seat wake/resume; T6 bridge two ChatGPT conversations through a user-authorized shared channel with verifiable receipts. T1-T2 read-only; T3-T4 isolated; T5-T6 side effects requiring project-specific authorization.
