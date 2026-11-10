# VIVIM Automation Queen — system archaeology and safe central intake

**Status:** DISCOVERY / COPY-ONLY handoff; no runtime migration, no new automation executor, no authority transfer.
**Primary owner:** local VIVIM main agent. **Source-of-truth for current execution:** existing Host/HQ/Studio/SOFTWARE_HQ contracts and verified live state, not this research folder.
**Critical:** This VIVIM Labs repository is PUBLIC. Repos HQ, SOFTWARE_HQ and VIVIM Web Harness are PRIVATE. Never copy private code, internal topology, credentials or unreviewed local artifacts into a tracked path in this public repository.

## Outcome
Create a complete, provenance-backed central view of ALL automation and agentic machinery developed across repository history and the Windows machine. Copy eligible source into a Labs corpus without moving or deleting original files or stopping an existing runtime. Produce an evidence-driven design for the future "Automation Queen": a meta-orchestrator able to discover, design, schedule, supervise, verify, repair and improve automations through existing VIVIM permissions.

Read [LOCAL-AGENT-MISSION.md](LOCAL-AGENT-MISSION.md) for the execution order and non-negotiable safety gates. The [SCHEMA-v1.json](SCHEMA-v1.json) defines required evidence fields.

## Deliberately separated locations
- **Tracked, PUBLIC-safe:** automation-queen/README.md, mission/contracts, redacted/aggregated PUBLIC-ATLAS.md, PUBLIC-MANIFEST.jsonl, sanitized graph/ownership map, approved public-source excerpts/fixtures.
- **Local ONLY, never staged:** automation-queen/.private-local/ (source snapshots, raw private manifests, exact paths, raw dependencies, private code, local operational maps); use restricted permissions and copy immutable blobs only after classification. No credentials/session secrets copied even there.
- **Existing source repositories:** stay unmodified. Their runtime authority stays where it is. This is a mirror, not a new owner.

## Scope and search anchors
Scan *all relevant automations* and their dependencies, not just files with "automation" in the name:
- Current known repos: vivim-web-harness, repos-hq, SOFTWARE_HQ, vivim-labs, tierflow, hq; also other checked-out user repos under C:\0-BlackBoxProject-0 and earlier relevant remote histories.
- Known leads: HQ supervisor, Work/Plan/Step/Attempt substrate, agent bus/agent protocols, agentic taxonomies/frontmatter, Host bootstrap and Studio relays, provider effect gates, ZCode/OpenCode orchestration, SOFTWARE_HQ automation/engines, Labs orphaned-work automation shelves.
- Historical hypotheses to locate and verify, not assume deployed: Automation Design Architect (ADA), Architecture Steward / Agent Commons / CFA, agentctl, autonomous execution kernel, Windows launchers/watchers, workflow/temporal dispatcher, context-aware agent teams, self-organization.
- Windows: installed OpenCode/ZCode manifests, checked-out repo configs, tool/plugin/skill folders, user-scoped scheduled tasks/services/startup definitions, scripts, MCP config metadata, launchers, local execution logs **metadata only**. No whole-disk scan, no browser profile/cookie scraping, no keychain/credential exfiltration.

## Output topology (created by local agent after scanning)
- `01-discovery/` — scan scope, source index, evidence of inaccessible locations, completeness estimates.
- `02-taxonomy/` — canonical automation/agentic type hierarchy and 217-format crosswalk.
- `03-corpus-public/` — approved public, attributed and hash-verified source copies only.
- `04-lineage/` — source→capability→workflow→agent→runtime→proof map and duplicate families.
- `05-architecture/` — current-state atlas, proposed Queen end-state, authority and interface decisions.
- `06-gaps-and-decisions/` — conflicts, missing components, portability risks, KEEP/ADAPT/RETIRE/UNKNOWN decisions.
- `07-evidence/` — non-sensitive validation commands/results, copy checksums and public-safe evidence.
- `.private-local/` — **gitignored** private staging, complete authorized local-only source copies and raw manifests.

Never publish source paths, branch names, logs, personal metadata or internal diagrams from private repos unless separately reviewed for public release. Public summaries should be aggregated and nonspecific.

## Completion is not a file count
The deliverable must answer: What automation capabilities exist? Which are actually running? Which overlap? Who has effect authority? What is prototype vs verified? What is missing on the PC? What must be reused/retired to make a single Queen without a second scheduler, graph, browser sender or memory authority?

This work is complementary to the separate Repos HQ PR #9 intelligence integration effort: PR #9 wires existing runtime; this PR gathers the full landscape for later owner-reviewed consolidation. Do not modify PR #9 code or claim its implementation finished.
