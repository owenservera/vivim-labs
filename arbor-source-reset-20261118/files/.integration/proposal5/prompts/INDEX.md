# ARBOR Proposal 5 — Copyable Agent Prompt Index

**Work orders ready, execution NOT started.** All prompt links are relative within the same ARBOR `architecture/proposal5-beta-first-centralization` branch.

## One paste to execute everything

**[00-AUTO-RUN-ALL.md — Copy the full self-running master prompt](00-AUTO-RUN-ALL.md)**

Paste this into the local orchestration agent that can create branches/spawn workers. It should launch each wave's six independent agents A–F, wait, run the serial integration G, then automatically proceed until a hard blocker or owner approval boundary.

## Wave launches and per-agent copies

| Wave | One-paste wave launcher | Parallel lane prompts | Serial integrator |
| --- | --- | --- | --- |
| **W0 Source truth** | [Copy/launch one prompt](W0/LAUNCH.md) | [A](W0/A.md) · [B](W0/B.md) · [C](W0/C.md) · [D](W0/D.md) · [E](W0/E.md) · [F](W0/F.md) | [G](W0/G.md) |
| **W1 Source landing** | [Copy/launch one prompt](W1/LAUNCH.md) | [A](W1/A.md) · [B](W1/B.md) · [C](W1/C.md) · [D](W1/D.md) · [E](W1/E.md) · [F](W1/F.md) | [G](W1/G.md) |
| **W2 Internal wiring** | [Copy/launch one prompt](W2/LAUNCH.md) | [A](W2/A.md) · [B](W2/B.md) · [C](W2/C.md) · [D](W2/D.md) · [E](W2/E.md) · [F](W2/F.md) | [G](W2/G.md) |
| **W3 Beta journeys** | [Copy/launch one prompt](W3/LAUNCH.md) | [A](W3/A.md) · [B](W3/B.md) · [C](W3/C.md) · [D](W3/D.md) · [E](W3/E.md) · [F](W3/F.md) | [G](W3/G.md) |
| **W4 Red-team proof** | [Copy/launch one prompt](W4/LAUNCH.md) | [A](W4/A.md) · [B](W4/B.md) · [C](W4/C.md) · [D](W4/D.md) · [E](W4/E.md) · [F](W4/F.md) | [G](W4/G.md) |
| **W5 Blocker repair** | [Copy/launch one prompt](W5/LAUNCH.md) | [A](W5/A.md) · [B](W5/B.md) · [C](W5/C.md) · [D](W5/D.md) · [E](W5/E.md) · [F](W5/F.md) | [G](W5/G.md) |
| **W6 Beta handoff** | [Copy/launch one prompt](W6/LAUNCH.md) | [A](W6/A.md) · [B](W6/B.md) · [C](W6/C.md) · [D](W6/D.md) · [E](W6/E.md) · [F](W6/F.md) | [G](W6/G.md) |

### Discipline

- Each wave has **six parallel lane prompts** plus **one serial G gate**. Never run G while lanes are changing code.
- Stable lane ownership: **A Native Rust/Tauri; B Host World/control; C Studio/Chrome; D HQ JS intelligence; E phone/TierFlow; F contracts/QA.**
- Branch `integrate/proposal5-beta-r001` is the integration destination. All workers branch from its exact current wave baseline; G alone merges/root-locks.
- Proposal 5 contains **no outside wiring**: HQKG, HQ JS operational index/context/wiki, Host, Studio, PWA, TierFlow and selected Labs code **physically inside ARBOR**.
- **First integration is source-preserving migration**, not an excuse for a feature sprint. Preserve the current UI and provider workflows; fix only security/liveness/pairing/release blockers.
- W4-G may issue a finite `REPAIR_HANDOFF` into W5; W5-G must return `GO_FOR_W6`.
- W6 final private-data cutover, publication and replacing the running owner's Host requires **explicit owner approval** and verified backup/rollback.

Read the [complete execution playbook](../09-EXECUTION-PLAYBOOK.md), [beta gates](../08-BETA-GATES.json) and [beta journeys](../05-FULL-BETA-RELEASE-CONTRACT.md) before invoking workers. If the orchestration agent cannot spawn subagents, launch A–F individually using these links, then G, then next wave.
