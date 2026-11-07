# Build attribution — who builds VIVIM to V1

**Schema:** `vivim.build-attribution/1` · **Status:** initiative open, baseline mostly UNKNOWN
**Question it answers:** from now to V1 production, what share of the product is built by the owner
(with the PC, or working directly with ChatGPT), by the VIVIM runtime itself, and by coding agents
(ZCode / OpenCode / fleet across ChatGPT, Claude, Qwen).

This is a measurement initiative, not a reporting flourish. Its purpose is to know whether the
"own your AI" claim is being demonstrated on its own construction.

---

## 1. The unit

**One V1 roadmap item**, because that is the only ledger with a definition of done
(`vivim-web-harness/docs/roadmap.json`). Commits, files and lines are *evidence*, never the unit —
a hundred commits can close one item and one commit can close none.

An item may be split across actors. Then it is recorded once per actor with a share, and the shares
must sum to 1.0. No silent rounding.

## 2. The three actors

| Actor | Means | Evidence shape |
|---|---|---|
| `owner` | the owner working on the PC or directly with ChatGPT in a browser, no agent in the loop | prompt id + ChatGPT thread + commit |
| `runtime` | the VIVIM product itself doing the work (Host task, personal agent, Build surface) | task id + receipt + commit |
| `agent` | a coding agent session (ZCode, OpenCode, …) with named providers | session id + providers + commit |

`agent` is further split by provider set, because "ChatGPT + Claude + Qwen" is not the same claim as
"one model".

## 3. Capture — one trailer, zero overhead

Every commit carries exactly one trailer:

```
VIVIM-Build: actor=<owner|runtime|agent> providers=<comma list|none> lane=<id> session=<id|none>
```

Rules:

- `providers` names the models actually used for the work, not the ones installed.
- `session` is the agent session or VIVIM task id, so the record can be audited.
- A commit with no trailer is **UNKNOWN**, never inferred. Guessing by commit style is fabricating.
- Backfill is allowed only where a real artifact proves it (an owner prompt entry, a channel
  receipt, a task record). Anything else stays UNKNOWN and is reported as such.

A pre-commit hook can add the trailer from environment variables; that is the only acceptable way
to keep this at zero overhead. If a human has to type it, it will not be typed.

## 4. The report

```json
{
  "schema": "vivim.build-attribution/1",
  "asOf": "2026-11-03",
  "v1Items": { "total": 0, "done": 0, "open": 0 },
  "byActor": {
    "owner":   { "items": 0, "share": 0.0 },
    "runtime": { "items": 0, "share": 0.0 },
    "agent":   { "items": 0, "share": 0.0, "byProviders": { "chatgpt": 0, "claude": 0, "qwen": 0 } },
    "unknown": { "items": 0, "share": 0.0 }
  },
  "trend": [{ "week": "2026-W45", "owner": 0, "runtime": 0, "agent": 0, "unknown": 0 }],
  "evidenceGaps": ["commit without trailer", "item closed without a receipt"]
}
```

`unknown.share` is the most important number on the first run. A report that shows 100% agent and
0% unknown when nothing was recorded is a lie; the honest first run will be mostly unknown.

## 5. What counts as proof for each actor

- **owner**: a prompt entry in `docs/OWNER-PROMPTS.md` that corresponds to the change, plus the
  commit. A ChatGPT thread link is stronger.
- **runtime**: a VIVIM task id with a receipt whose outcome is `observed` or `verified`. An
  `uncertain` receipt never counts as done.
- **agent**: a session id, the provider list, and a green acceptance command in the same session.

An item closed by an agent that the owner then corrected is still `agent`, with a correction count.
Corrections are a quality signal, not a different actor.

## 6. Boundaries

- This ledger never writes into `vivim-web-harness` or `vivim-ui`. It reads.
- It does not re-open or re-verify another lane's work; it attributes what is already recorded.
- A lane's own session owns its receipts. This ledger cites them.

## 7. V1 scope definition (to be frozen)

"V1 production" is the Gate 1 user journey in `vivim-web-harness/docs/USER-JOURNEY.md:67-79`:
paired phone, one verified AI, Build tab, blank canvas, suggestion, preview, activate, persists,
Auto Chat reply, undo, Talk intact. Items outside that journey are V1-adjacent and are reported in a
separate bucket so the headline number stays honest.

## 8. Current baseline

**UNKNOWN-dominant, by construction.** No commit in either repo carries a `VIVIM-Build` trailer, and
the roadmap does not record an actor per item. The first computed report will therefore report the
UNKNOWN share as the headline and a small attributed subset drawn from artifacts that exist:
`docs/OWNER-PROMPTS.md`, the VC-1 channel receipts, and this session's checker and ledger commits.

The next action is the trailer convention plus a reader — not a backfill guess.
