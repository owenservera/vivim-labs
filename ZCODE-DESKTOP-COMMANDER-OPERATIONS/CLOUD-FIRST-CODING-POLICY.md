# Cloud-first coding contract — mandatory for ChatGPT agents (2026-11-21)

**Scope:** Every ChatGPT session that reads this ZCODE-DESKTOP-COMMANDER-OPERATIONS sitrep, AGENTS file, ownership tracker, or descendant runbook. This is the authoritative tool-routing rule for engineering work. Nested files may narrow this contract, never silently relax it. Follow higher-priority instructions and explicit task-specific user overrides.

## Non-negotiable default
**ChatGPT codes in ChatGPT's own cloud execution environment, then commits/pushes to the canonical GitHub repository, normally directly to `main`.** The local Windows computer is NOT the coding workspace. Reading a local SITREP does NOT authorize doing the engineering task locally. The existence of Remote Desktop Commander, ZCode, or a local checkout does not change the execution venue.

## Routing decision — BEFORE any tool call
1. **Orientation / local SITREP only:** Remote Desktop Commander may read this folder, inspect local environment and run specifically requested, read-only health queries. Never run default deep probes, model pings, agent fanout, or edit product code just to answer a status question.
2. **Any application/source implementation, migration, refactor, tests, source generation, Rust parity work, SDK work or bug fix:** use ChatGPT's hosted workspace/container and direct GitHub tools. Refresh canonical repository HEAD; inspect the real source; write and test in the cloud where feasible; commit and push exact owned files on `main`; verify the resulting remote commit SHA. ChatGPT is primary engineer and integration owner.
3. **Physical device / Windows / existing local signed-in profile / PWA live launch or packaging requiring this workstation:** only after the relevant cloud code is pushed, use Desktop Commander or a separately scoped ZCode operator for explicit local verification, launch, deploy, hardware tests or runtime inspection. The local operator may pull/fast-forward safely if authorized; may NOT rewrite source, fix failing tests locally, or push replacement code without a fresh explicit owner override. Report local findings to ChatGPT for cloud fixes.
4. **Independent parallel support:** ZCode may handle explicitly assigned secondary research, nonoverlapping validation or local-only operations; it is never the default coder, integrator, or substitute for ChatGPT's cloud toolchain.
5. **Cloud access missing:** stop with `BLOCKED: CLOUD_GITHUB_UNAVAILABLE` (or another precise blocker), identify the missing permission/tool, and provide the exact unpushed patch if useful. **Never quietly fall back to Desktop Commander, ZCode, a local worktree, or local Git commits.**

## Tool permissions
| Tool / venue | Allowed by default | Forbidden as an implicit fallback |
|---|---|---|
| ChatGPT cloud/container and GitHub connector | Source read/edit, Rust-first implementation, available cloud tests, commit/push directly to `main`, verify remote SHA | Force push, destructive resets, overwriting concurrent work, invented test results |
| Remote Desktop Commander | Read this SITREP, view local state and read-only diagnostics; task-authorized launch, packaging, on-device tests | Source write/patch/refactor, agent-coded local builds as substitute for cloud coding, local `git add/commit/push`, unrequested ZCode sessions |
| ZCode | Explicitly bounded auxiliary lanes and workstation-specific operations | Main user-directed implementation, autonomous edit/commit, silent foreground delegation |
| No suitable tool | Honest BLOCKED/UNRUN report | Claiming work shipped or inventing proof |

**Exception gate:** A *current, explicit* instruction such as “edit this local file,” “ZCode owns coding,” or “build on my Windows PC” may alter the corresponding permission for that task. A prior historical practice, nested AGENTS file, or the phrase “SITREP” is NOT an exception. A request to “build and deploy PWA” authorizes appropriate deployment operations, not local source coding instead of cloud coding.

## Required cloud-first execution receipt
For each coding task, state: (a) canonical GitHub owner/repo and refreshed base SHA; (b) ChatGPT cloud implementation and changed paths; (c) exact cloud test commands and PASS/FAIL/UNRUN; (d) commit/push SHA verified on GitHub `main`; (e) if local proof was needed, separate local commands/results and device evidence; (f) remaining blockers. **`CODED` is not `PUSHED`; `PUSHED` is not `DEPLOYED`; `DEPLOYED` is not `PHONE VERIFIED`.**

## Product defaults
- Rust for all sensible product/core/CLI/integration work; TypeScript 7+ only where Rust is impractical for that component.
- One unified Rust fabric and existing Host effect authority; PWA first. Do not create competing engines.
- Direct `main`, no routine PR/branch/worktree; non-force fast-forward coordination; preserve all dirty local/remote work, other agents' paths, stashes and sensitive files.
- The Labs operations folder is for navigation, ownership and evidence, **not** a substitute repository for ARBOR production code.

**First question every agent must answer internally:** “Am I about to use Desktop Commander or ZCode for an operation ChatGPT can do in its own cloud environment?” If yes, stop and route that operation to cloud/GitHub instead.
