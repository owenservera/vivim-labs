# B-LIVE-INVENTORY-PUBLIC.md — Lane B aggregate (sanitized)

**scanId:** autq01-20261107T2203Z · **lane:** B (Windows & live automation archaeology, READ-ONLY)
**collected:** 2026-11-07, UTC window 22:07–22:16 · **evidence:** real command output (schtasks /query, reg query, Get-Service, Get-Process, Get-CimInstance, Get-ScheduledTask, directory listings)

Aggregate counts and generic categories only. Task names with arguments, command
lines, secret values, and paths outside the project root / user dot-dirs are withheld
(full detail in gitignored `.private-local/B-LIVE-INVENTORY.private.json`).

## 1. Scheduled tasks (Windows Task Scheduler)

| Bucket | Rows |
| --- | --- |
| Total rows observed | 297 (wc counts 296; final CSV row lacks trailing newline) |
| Microsoft OS maintenance noise | 256 |
| Vendor/OEM (laptop maker, browser vendor, Xbox, Windows feature) | 22 |
| Root-level custom/vendor non-project (shell helper, consumer-app updater) | 2 |
| **Project-related automation tasks** | **17 rows / 16 unique names** |

Project-related breakdown: 12 disabled (stale), 2 ready and actively firing (a
git-autosync scheduler on a ~5-minute cadence with last result 0, and a model-catalog
refresh scheduler with last result 0), 1 ready on-demand tool-reinstall task,
1 currently running third-party session agent. Failure signals observed on disabled
tasks: one browser-capture task last terminated (0x41306), one provider-drive task
last exited access-denied (5), one server-launcher task last exited 1.

## 2. Startup entries

| Location | Count |
| --- | --- |
| HKCU Run | 8 values |
| HKLM Run | 3 values |
| HKLM WOW6432Node Run | 1 value |
| User Startup folder | 9 entries |
| Common (ProgramData) Startup folder | 2 entries |

Of these, 8 are project/agent-related (agent-network autostart, agent tray, guard
script, gateway script, chief-of-staff script, OAuth-proxy script [existence only,
never read], disabled gateway variant, mesh-VPN shortcut). 1 entry is an
unknown-purpose scrambled-name `.ini` (metadata only).

## 3. Services

| Measure | Count |
| --- | --- |
| Total services | 317 |
| Non-disabled | 303 |
| Non-Microsoft/Windows filtered | 253 |
| Project-relevant (database, scheduling-agent, mesh-VPN, WSL, remote-control, HTTP-listener, AI-fabric) | 12 |

Notable: one database server running; its companion scheduler agent and connection
pooler are installed but stopped. Two unknown-origin services (HTTP listener,
"AI fabric") running; two "isolation session" services stopped.

## 4. Running processes

| Name family | Count |
| --- | --- |
| Total process rows | 326 |
| ZCode CLI instances | 38 |
| node | 24 |
| bun | 3 |
| chrome | 10 (10 separate top-level instance trees — consistent with dedicated automation profiles, not one browser) |
| powershell | 1 (this scan's own query) |
| opencode / python / pwsh / msedge / code / git / ssh / claude / gemini / codex / cursor | 0 each |

**Project-related observed-running process count: 65** (ZCode 38 + node 24 + bun 3).

## 5. Config surfaces (keys/names only — zero values)

| Surface | Highlights |
| --- | --- |
| `~/.zcode/agents` | 550 files = 53 live agent definitions + 497 `.bak-*` backups (+21 in a sibling backup dir); dominant model line: zai-start-plan/GLM-5.3-Flash, one nvidia free-route |
| `~/.zcode/skills` | 400 entries |
| `~/.zcode/cli/config.json` | keys: plugins, hooks, mcp; hook events: UserPromptSubmit, SessionStart; 10 MCP server names; providers key absent (0 in this file) |
| `~/.zcode/provider-depot` | catalog with 7 source names; 8 backup rotations of the catalog (7 dated scan-day) |
| `~/.zcode/comms` | 4 files (bus, delivered, projects, receipts) |
| `~/.zcode/evolution`, `feedback`, `machine-steward`, `memories` | 3 / 1 / 1 / 1 entries |
| `~/.opencode` | opencode.json keys: $schema, plugin; config.json keys: $schema, plugin, mcp (3 MCP server names); 1 skill; bin empty; memory dir contains 1 secret-named file (never read) |
| `~/.agents` | 97 skill entries (incl. quarantine + backup dirs); model-profile keys: note, entries, roles; 1 diagram |

## 6. VIVIM live workspace (`C:\VIVIM`)

- 24 top-level dirs + 4 files. Largest: chrome-attach 7,069 files, chrome-profile
  7,863 files (browser profiles — counted only, never opened; CREDENTIAL_DENY),
  sessions 1,013 files, conversations 119, channel 160.
- Channel bus (file-based): dispatch 74, outcomes 35, receipts 35, decisions 11,
  tasks 1, revocations 1, pending 0, sprints 0; plus seats/team-state/threads JSON
  state files. Newest activity 2026-11-05.
- Intent/result dispatch pairs observed for at least two seats (names only).

## 7. Watchers and launchers (presence only — nothing executed)

- repos_hq `.opencode/plugins`: 2 entries, incl. a git session-sync plugin (5-minute loop per project docs).
- repos_hq `scripts`: 30 entries (sync bus, canonical git sync, fleet, quality gate, smart tests, legacy autosync).
- SOFTWARE_HQ: 30 top entries, 10 automation-related (automation, central-models, operations, integrations, gpm-coordination, …).
- Project-root launchers: 2 PowerShell scripts (one is a browser launcher — recorded, never executed); CLIs folder: 4 entries (repomix exploration).
- ZCode hooks configured on: UserPromptSubmit, SessionStart.

## 8. Unknown roots

10 items logged in `.private-local/B-UNKNOWN-ROOTS.private.md` (1 secret-ref-only,
8 unknown-review, 1 redacted third-party task payload). Highlights: a literal `~`
directory inside `~/.opencode`, a scrambled-name `.ini` in the user Startup folder,
and one un-inventoried project subdirectory referenced by a failing disabled task.

## Gates (Lane B view)

| Gate | Status | Note |
| --- | --- | --- |
| G0 BASELINE | PASS_WITH_NOTES | Scan set pinned; private outputs in gitignored `.private-local/` (verified); active running jobs identified (65 project processes; 2 active schedulers). |
| G1 COVERAGE | PASS_WITH_LIMITS | All 8 assigned sections collected from real output; config contents held at keys/names by design. |
| G2 SECURITY | PASS | No secret values transcribed; secret-named files and OAuth script existence-only; browser profiles counted, never opened. |
| G9 NO DISRUPTION | PASS | Read-only commands only; nothing started, stopped, killed, configured, deleted, or launched; no browser opened. |

Private detail (task names + redacted command shapes, full service list, process
rows, config keys): `.private-local/B-LIVE-INVENTORY.private.json`
Unknown-root register: `.private-local/B-UNKNOWN-ROOTS.private.md`
