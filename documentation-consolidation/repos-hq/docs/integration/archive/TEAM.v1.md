# The team

Six seats. One loop. Three invariants.

This page replaces the earlier governance draft. `lanes.v1.json` still defines what may be
written; this page defines who does the work and how a task moves. No gate matrix, no evidence
taxonomy, no validator that grades the team.

---

## The seats

| Seat | Agent | Job | First assignment |
| --- | --- | --- | --- |
| **Atlas** | `atlas` | Lead. Plans, dispatches one builder per lane, merges, reports to the owner. | Review F0 and dispatch Wave 1. |
| **Scout** | `scout` | Read-only recon. Maps the files, contracts and traps before anyone writes. | Map the kernel/intake seam for L1/L2. |
| **Forge** | `forge` | Builder. Takes one lane and one frozen contract; writes only inside it. | L1 durable kernel store. |
| **Lens** | `lens` | Independent falsifier. Re-runs the acceptance and checks every claim. | Audit the four F0 contracts against source. |
| **Keeper** | `keeper` | Operator. Runs the gates and machine checks; reports, never repairs. | Baseline the five gate commands and separate noise. |
| **Weaver** | `weaver` | Synthesizer. Turns reports and verdicts into one brief. | Wave 1 decision brief. |
| **HR** | `hr` | Team HR manager. Roster, seat assignments, onboarding, seat conflicts, escalations. Read-only toward product code. | Bootstrap the HR seat itself. |

Standing: `infra` for machine blockers, `oracle` for install truth, `fixer` for post-gate repair, `hr` for roster and conflicts.

**Models.** Master session: `deepseek/deepseek-flash`. All six seats:
`openrouter/nvidia/nemotron-3.5-super-vl:free`. This host has no direct `nvidia/` provider — the
Nemotron IDs are all OpenRouter. If that free tier hits its budget, switch a seat to
`ccr-router/ccr-economy` or another configured provider; the role contract does not change.

---

## The lanes and their seats

| Lane | Work | Seat | Wave |
| --- | --- | --- | --- |
| **L0** | Freeze, scaffold and integrate | Atlas | 0 |
| **L1** | Durable kernel store outside `.hq/` | Forge | 1 |
| **L2** | Nine-section model intake | Forge | 2 |
| **L3** | KB corpus parity and migration receipts | Forge | 2 |
| **L4** | Read-only Host connector | Forge | 2 |
| **L5** | Prototype acceptance receipts | Forge (Keeper on call) | 1 |
| **L6** | UI timing assessment | Weaver (Lens audits) | 1 |
| **L7** | Mount and adopt the UI core (conditional) | Forge | 3 |

A lane is one unit of work, defined in `docs/integration/lanes.v1.json` with a repo, a write scope
and an acceptance command. Forge takes one lane at a time; Atlas may run up to three Forges in
parallel when their scopes are disjoint.

---

## The loop

```
owner
  │ task
  ▼
Atlas ──work order──▶ Scout (map, optional)
  │                      │ map
  │                      ▼
  └──work order──▶ Forge (build one lane)
                     │ build report
                     ▼
                   Lens (re-run, falsify)
                     │ verdict
                     ▼
                   Keeper (run the gate, machine checks)
                     │ gate report
                     ▼
                   Atlas (commit path-scoped, update the ledger, report)
                     │
                     ▼
                   Weaver (wave brief)
```

---

## One work order

Atlas writes this to start a lane; it is the only kickoff artifact.

```
LANE: L1
TARGET: hq/lib/kernel/**          (from lanes.v1.json)
CONTRACT: hq.kernel-store/1
ACCEPT: bun test hq/test/kernel
RETURN: changed paths, commands, exact output, open items, undo
```

If the work order needs more than five lines, the lane is too big.

---

## Handoffs

**Forge → Atlas / Lens**

```
CHANGED: path:line ...
RAN: <exact command>
RESULT: <exit code, pass/fail counts, the lines that matter>
OPEN: ...
UNDO: ...
```

**Lens → Atlas**

```
CLAIM: <one line>
VERDICT: upheld | weakened | demoted | retracted
EVIDENCE: path:line or command output
```

**Keeper → Atlas**

```
GATE: <id>   STATE: green | red | noise
COMMAND: <exact>
OUTPUT: <exit code and the lines that matter>
NOISE: <why this is an exclusion, not a failure>
```

**Weaver → owner**

```
DECISION / COST / RISK / NEXT
```

---

## Three invariants

1. **Forge never grades Forge.** Lens re-runs the acceptance; Atlas records the verdict.

2. **Parallel lanes need disjoint scopes and path-scoped commits.** `scripts/git-main-sync.mjs`
   stages with `git add -A` (`:216`) and commits the whole tree (`:223`). L0 therefore stages only
   the lane's scope and commits it. An auto-sync checkpoint commit is not gate evidence; a mixed
   commit is repaired by a follow-up path-scoped commit, never by reset or force-push. The ruling
   is recorded in `lanes.v1.json` `commitPolicy`.

3. **No provider effects.** Nobody sends, approves, executes, or writes to the Host, Studio
   journal, Android broker or Tier 3. Evidence is not permission.

---

## How to summon

```text
Agent(subagent_type="atlas", prompt="<owner task>")
Agent(subagent_type="forge", prompt="<work order>")
Agent(subagent_type="lens",  prompt="<lane id and acceptance command>")
Agent(subagent_type="hr",    prompt="<roster or conflict concern>")
```

- **Today**, until the definitions load: use the existing types — `orchestrator` for Atlas,
  `explorer` for Scout, `fixer` for Forge, `verifier` for Lens, `oracle` for Keeper,
  `synthesizer` for Weaver, `hr` for team HR.
- **After the restart**: the seven call signs above are available.
- The owner talks to Atlas, not to Forge. HR handles roster, not product.

---

## Current board

| Seat | Now |
| --- | --- |
| Atlas | F0 review; dispatch Wave 1 (L1, L5, L6 — three repos) |
| Scout | map `hq/lib/{schema,walk,kb-intake}.mjs` and the intake insertion points |
| Forge | L1: durable kernel store outside `.hq/` (Wave 1, repos_hq) |
| Lens | audit the four F0 contracts against the files they cite |
| Keeper | run `check`, `status`, `verify`, `wiki --check`, `integration/check`; classify noise |
| Weaver | hold until Wave 1 closes |
| HR | bootstrap the HR seat (live after restart) |

---

## What this team does not do

No gate matrix, no evidence taxonomy, no per-role JSON schema, no validator that grades the team.
The lane file is the contract; this page is the roster; the agent files are the prompt.

**Owner decisions still open:** T2 approve the roster · T3 routine cadence · T4 bus identities ·
T5 model tiers.

**Decided:** T1 — parallel writes with serialized, path-scoped commits by L0 (`lanes.v1.json`
`commitPolicy`).
