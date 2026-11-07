# Brainstorm: what is missing — 2026-10-31

## Source of this review
Scanned from my vantage point as the single active repo worker on the `WINDOWS_ROADMAP_2026_10_31` active phase. Evidence below is command output and file contents, not inference.

## 1. Test suite is red (not green)
**Harness** — `bun run test`:
- `tests 290 | pass 281 | fail 8 | skipped 1`
- 6 failures: `test/interface/models-router.test.mjs` — every "auto routing / intelligent model router" test throws `ROUTER_PROFILE_INVALID` at `intelligent-model-router.mjs:7`. The intelligent router (GOALS G2 multi-model review path) is broken.
- 2 failures: `test/core/g3-reply.test.mjs` — `detectTruncation` and `readReply codeBlocks` (GOALS G3 long-replies + code extraction path).
- `npm test` green at root is a stated Phase 1 exit criterion (GOALS.md line 55). It is currently **not met**.

**Studio** — `bun run test` run exceeded the command timeout but output showed:
- `✖ test\unit\p0-windows-probes.test.mjs (98574.67ms)` — the P0 Windows probes suite is failing/extremely slow. GOALS Gate 1 is Windows-first; P0 Windows probes failing is a material gap.

## 2. Grok provider adapter is missing (GOALS lists it)
GOALS.md Gate 1 gap analysis (line 27): "uniform `signedIn` probe for all providers incl. **Grok** (no harness adapter yet)". Product goal line 40 lists ChatGPT, Claude, Gemini, Qwen, Z.ai, DeepSeek — but Gate 1 walkthrough and hard rule mention signing in to AI apps including Grok.

Actual provider inventory:
- `harness/src/providers/`: chatgpt, claude, deepseek, gemini, index, qwen, zai (7 files, **no grok**)
- `studio/src/profiles/sites/`: chatgpt, claude, deepseek, duck-ai, gemini, index, kimi, lmarena, minimax, qwen, zai (**no grok**)

Grok has no selector profile, no auth-wall preflight, no model tier config, no `signedIn` probe, and no studio site profile. This is a concrete missing piece against the owner's stated provider set.

## 3. Capability registry: almost nothing is implemented
From `docs/capabilities/atomic-registry.json` (336 items):
- `decisionState`: `execution_dependent` = 309, `human_review_needed` = 24, `implemented` = **3**
- The 3 implemented are all GUI-05/06/07 (generative UI revision/preview/Undo) under PRD 11.
- `executionWave`: 232 `unassigned`, 104 `unscheduled`. Nothing is scheduled.
- `productPriority`: P0 = 129, P1 = 126, P2 = 53, UNTRIAGED = 28.

So 99% of declared capabilities have no implementation verdict, and none are scheduled into a wave. The registry is a declaration inventory, not a plan.

## 4. Roadmap: 124 tasks, half not started
- `done`: 35 | `in_progress`: 12 | `blocked`: 2 | `queued`: 33 | `planned`: 35 | other terminal-ish: 7
- 44 tasks have owner `unassigned` (or owner contains "unassigned"), including:
  - RG01 Reconcile divergent Personal AI bootstrap, mobile bridge and Build PR with current main (queued)
  - DEV01 Live wh dev E2E (queued)
  - DATA02/DATA03/DATA04/DATA05 (queued/planned)
  - DCB02, ZC01, GB01, AION01, VOMEGA01, IP01, SITE01, FILES01, CANVAS01, PAIR01, TEST01, PACK01, SOURCE01, BOOTX01, THEME01, DCB03, TEST02, MOBUX01, OPSUI01, SKILL02, GUI03
- 24 capabilities carry `human_review_needed` (X-005..X-104), including X-012 Direct Tailscale phone transport, X-013 OpenCode phone sessions, X-014 ZCode phone target, X-015 Grok Bot phone target, X-017 100/200 real webapp scenarios, X-052 DCB ten-layer projection.

## 5. Duplication of work (from the duplication scan)
- 44 source files are claimed by more than one capability; 88 verified symbols are claimed by more than one capability.
- 301 of 336 capabilities share at least one source file or symbol with another capability (19,404 shared-with incidences).
- CAP-01..CAP-15 all cite the same 6 source files (`harness/src/core/models.mjs`, `harness/src/interface/router.mjs`, `studio/bin/commands/relay.mjs`, `studio/bin/lib/relay-chat-studio.mjs`, `apps/mobile-relay/app.js`, `apps/vivim-host/direct-phone.mjs`) — shared foundation, not distinct implementations.
- Documented in `docs/library/DUPLICATION-SCAN-2026-10-31.md`.

## 6. Windows CLI is deliberately limited
`studio/scripts/windows/vivim-cli.ps1` help text:
- `start`: "Unavailable until manifest-selected browser ownership can be verified."
- `status`: read-only health, no start.
- Line 17: "This adapter does not add Host/relay supervision or automatic Windows startup."
GOALS Gate 1 M3/M4 require harness pairing and phone-driven bootstrap from the host; the Windows CLI currently defers `start` and does not supervise Host/relay.

## 7. Phase 2 not started (per GOALS.md)
GOALS.md product goal (lines 47-49): "Phase 2 — agentic harness (not started). Planner/tool loop that uses the Phase 1 base as its 'model call'. Design only after Phase 1 passes a live end-to-end send."
GOALS current state table: "First live send — Not done. Waiting on owner approval."
So Phase 2 (self-build loop G1, multi-model review G2, long replies G3, tab hygiene G4, journal/recovery G5, one-browser-layer G6) is blocked on Phase 1, and Phase 1's first live send has not happened.

## 8. Duplicate provider data models across harness and studio
`harness/src/providers/chatgpt.mjs` and `studio/src/profiles/sites/chatgpt.mjs` define overlapping provider selectore (composer, send, assistant, preflight, authWall) in two different shapes. GOALS G6 "one browser layer" calls for deduplicating studio/harness CDP + MCP layers into one CDP client and one MCP surface, with studio profiles as data only. Not yet done — duplicate modules exist and both suites must be green before this counts.

## 9. No single-package installer
GOALS Gate 1 gap analysis (line 25): Installer exists as `studio/scripts/windows` (VIVIM.cmd, shortcuts; needs Node 22 preinstalled) but missing "single package that bundles Node + harness + studio; macOS/Linux; version/update". M5 (one download, Windows first) is not done.

## Summary of the most concrete, actionable missing items
| # | Missing | Evidence |
|---|---------|----------|
| 1 | Intelligent model router throws ROUTER_PROFILE_INVALID | 6 failing harness tests |
| 2 | reply truncation/code-block detection broken | 2 failing harness g3-reply tests |
| 3 | P0 Windows probes failing | studio p0-windows-probes.test.mjs ✖ |
| 4 | Grok provider adapter absent | RESOLVED: added harness/src/providers/grok.mjs + studio/src/profiles/sites/grok.mjs, registered in both index.mjs files (7 providers / 11 site adapters). Validates cleanly. |
| 5 | First live owner-approved send not done | GOALS.md current state table |
| 6 | Phase 2 agentic harness not started | GOALS.md product goal |
| 7 | 309/336 capabilities execution_dependent | atomic-registry.json decisionState |
| 8 | 44 roadmap tasks unassigned | roadmap.json owner field |
| 9 | 44 source files + 88 symbols duplicated across capabilities | duplication scan |
| 10 | `vivim start` unavailable on Windows | vivim-cli.ps1 help text |
| 11 | No single-package installer (M5) | GOALS.md gap analysis |

## Recommendation order
Fix the red harness tests first (intelligent router + g3-reply), since `npm test` green is a Phase 1 exit criterion and everything downstream (live send, Phase 2) depends on it. Then add the Grok adapter, then address unassigned roadmap tasks and duplication.
