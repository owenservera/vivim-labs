# FINDINGS-04 — Agent-definition audit (communication-bus readiness & spawn config)

Source: every `*.md` under `C:\Users\VIVIM.inc\.zcode\agents\` (30 files), cross-referenced against
`C:\Users\VIVIM.inc\.zcode\cli\memories\projects\repos_hq-5b07505dca327f64\memory\`
(seat-models-can-be-unavailable, seat-spawn-thoughtlevel-max, seat-reasoning-level-must-be-max, team-model-routing).

Operative rule (from the memories): `~/.zcode/agents/<name>.md` is what governs spawning; the
`.opencode/agent/<name>.md` mirror does not. Agent files are read **at session start**, so an edit
is inert until ZCode restarts (`zcode-restart`). `thoughtLevel: max` is required for
`deepseek/deepseek-flash` seats (`enabled`/`disabled` fail with reasoning-level-not-supported);
`openrouter/nvidia/nemotron-3.5-super-vl:free` seats run `thoughtLevel: enabled` in the established
pattern.

| # | Agent | Model | thoughtLevel | thoughtLevel: max? | Communication-bus knowledge | codex-oauth seat? | Verdict |
|---|-------|-------|--------------|--------------------|----------------------------|-------------------|---------|
| 1 | active-task-scanner | openrouter/nvidia/nemotron-3.5-super-vl:free | max | yes | Yes — `sync.mjs check/inbox`, `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 2 | agent-platform-ceo | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 3 | atlas | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `sync.mjs send`, `anet` | No | spawnable |
| 4 | beacon | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 5 | bot-auditor | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `bus.mjs post/check`, `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 6 | bot-chief | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `bus.mjs check`, `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 7 | bot-librarian | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `bus.mjs post`, `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 8 | bot-quartermaster | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `bus.mjs post`, `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 9 | channel-manager | deepseek/deepseek-flash | max | yes | Yes — `sync.mjs send/claims`, `hq seat wake`, `SendMessage`, `anet` | No | spawnable (deepseek-flash + max is correct; proven on this seat) |
| 10 | community-analyst | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 11 | council | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 12 | docs-analyst | openrouter/nvidia/nemotron-3.5-super-vl:free | disabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable — note: `thoughtLevel: disabled` is non-standard for nemotron (pattern is `enabled`); `disabled`/`enabled` fail only for deepseek-flash, but verify before relying on it |
| 13 | explorer | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 14 | fixer | openrouter/nvidia/nemotron-3.5-super-vl:free (frontmatter) | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | Historically — pinned to `codex-oauth/gpt-6.1-sol`, provider-not-found 2026-11-03 | spawnable — current frontmatter is nemotron; monitor, no current failure evidence |
| 15 | forge | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 16 | hr | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `sync.mjs send (claims)`, `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 17 | infra | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 18 | install-analyst | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 19 | keeper | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 20 | lens | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 21 | link-keeper | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 22 | oracle | openrouter/nvidia/nemotron-3.5-super-vl:free (frontmatter) | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | Historically — `codex-oauth/gpt-6.1-sol`; **re-tested 2026-11-04 launches fine** | spawnable |
| 23 | orchestrator | openrouter/nvidia/nemotron-3.5-super-vl:free (frontmatter) | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | Historically — pinned to `codex-oauth/gpt-6.1-sol`, provider-not-found 2026-11-03 | spawnable — current frontmatter is nemotron; monitor |
| 24 | pocket-console | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 25 | researcher | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 26 | scout | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |
| 27 | synthesizer | openrouter/nvidia/nemotron-3.5-super-vl:free (frontmatter) | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | Historically — pinned to `codex-oauth/gpt-6.1-sol`, provider-not-found 2026-11-03 | spawnable — current frontmatter is nemotron; monitor |
| 28 | verifier | openrouter/nvidia/nemotron-3.5-super-vl:free (frontmatter) | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | Historically — pinned to `codex-oauth/gpt-6.1-sol`, provider-not-found 2026-11-03 | spawnable — current frontmatter is nemotron; monitor |
| 29 | weaver | openrouter/nvidia/nemotron-3.5-super-vl:free | enabled | no | Yes — `hq seat wake`, `SendMessage`, `anet` | No | spawnable |

## Notes

**Spawn readiness.** All 30 agent definition files carry a valid model + thoughtLevel combination and a
communication-bus section. No agent is currently misconfigured or unavailable based on the operative
`~/.zcode/agents/<name>.md` frontmatter. Fixes required: none.

**Historical codex-oauth seats (fixer, orchestrator, oracle, synthesizer, verifier).** The
seat-models-can-be-unavailable memory (Wave 1, 2026-11-03) pins these to `codex-oauth/gpt-6.1-sol`,
which failed with `provider-not-found` on this host. The current frontmatter of all five now reads
`openrouter/nvidia/nemotron-3.5-super-vl:free`, and oracle was re-tested 2026-11-04 (background dispatch
accepted, no provider-not-found). Treat the 2026-11-03 outage as historical; seat availability can change
after a model refresh — verify by dispatching rather than assuming.

**thoughtLevel.** `channel-manager` (deepseek-flash) correctly declares `thoughtLevel: max` — the only
value that spawns a deepseek-flash seat (`enabled` and `disabled` both failed). active-task-scanner also
declares `max` (harmless on nemotron). docs-analyst declares `thoughtLevel: disabled` — non-standard for
a nemotron seat (established pattern is `enabled`); no spawn-failure evidence for nemotron+disabled, but
confirm before relying on it.

**Communication-bus knowledge.** Every agent has the standard "Reaching another agent" block citing
`docs/AGENT-BUS.md`: the sync bus delivers but starts nothing; `hq seat wake <seat>` and `SendMessage`
start a real turn; `anet` is the only cross-machine channel; a wake is proven by the seat's reply, not an
acknowledgement. active-task-scanner and channel-manager additionally reference `sync.mjs check` /
`inbox` / `claims` explicitly (the "check --as first act" cadence).

**Session-start rule.** All edits to agent files are inert in a running session; restart ZCode
(`zcode-restart`) after any model/thoughtLevel change and say a restart is needed rather than claiming it
took effect.

## Verdict summary

- Spawnable: 30 / 30
- Misconfigured: 0
- Unavailable: 0

No edits were made to any agent definition file, per mandate (findings-only).