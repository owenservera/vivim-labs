# VIVIM Labs — agent and contributor rules

## Owner directive — VIVIM-first workstation operations (2026-11-25, R030)

**Mandatory for every ChatGPT development session:** use the installed VIVIM ARBOR MCP connector for authorized workstation operations. Direct general terminal execution is now live, not recipe-only. Desktop Commander is retired as a routine operations dependency; use it only for an explicitly authorized, specific missing capability or recovery, and record the gap. Do not route around authorization failures. Cloud/GitHub remains the primary source engineering and canonical main-commit venue; local validation, build, launch, packaging and deployment go through VIVIM first. ZCode is subordinate, direct CLI only for explicitly scoped work; no ZCode workflows.

At session start, discover actual VIVIM tools and read tunnel health and grant summary. Prefer PowerShell 7+ (program ID `pwsh`), not Windows PowerShell 5.1. Use existing terminal/session/file operations within the current grant, preserve claims and concurrent work, and record exit/output/receipt evidence. Never print credentials. If a grant expires or is missing, use the supported owner-approved renewal path. Do not automatically expand permissions. If catalogue is stale, use ChatGPT Settings → Plugins → VIVIM ARBOR → scroll to Tools → Refresh tools; recheck tool discovery and run a harmless direct smoke test. No disconnect/reinstall needed. Browser automation of this refresh was demonstrated through a temporary bridge; native Studio-driven refresh and self-upgrade still require completion.

Canonical operating detail: [ARBOR MCP handoff](https://github.com/owenservera/ARBOR/blob/main/tools/mcp/vivim-arbor/HANDOFF.md) and [cloud-first / VIVIM-first policy](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/CLOUD-FIRST-CODING-POLICY.md). This directive supersedes historical Desktop Commander defaults, recipe-only/health-only status and legacy ZCode workflow instructions, without changing task ownership, source venue or approval boundaries.

**Cloud-first agent routing:** For ARBOR and related engineering, ChatGPT writes/tests in its own cloud workspace and pushes through GitHub; VIVIM ARBOR is the default local operations connector; Desktop Commander is exceptional recovery only. See [mandatory Cloud-First Coding Policy](ZCODE-DESKTOP-COMMANDER-OPERATIONS/CLOUD-FIRST-CODING-POLICY.md). ZCode does not automatically inherit primary coding ownership.


**Labs is public, source-linked research/evidence only.** It is not a product roadmap, production Host, UI publisher, cross-repo index, graph engine or private archive.

## Read first

- [README.md](README.md) for repository role and research/archive entrypoints.
- [Z.ai campaign](zai/README.md), [charter](zai/CAMPAIGN-CHARTER.md), [decision protocol](zai/governance/DECISION-PROTOCOL.md), [decision records](zai/governance/DECISIONS.md), [pending decisions](zai/governance/OPEN-ITEMS.md) and [round template](zai/rounds/ROUND-TEMPLATE.md).
- [Historical document relocation inventory](documentation-consolidation/INVENTORY.json) for prior archive copies and provenance.

## Cross-repository authority

VIVIM has exactly four primary GitHub repos: `repos-hq` (existing shared intelligence/index/wiki), `vivim-web-harness` (product Host/mobile, effects, current roadmap), `tierflow` (independent visual runtime), and `vivim-labs` (evidence). The local `vivim-ui` folder means GitHub `tierflow`; it is not another project. SOFTWARE_HQ is a **separate private repository**, not part of VIVIM and never a permitted data input to public Labs.

## Work rules

1. Keep public-safe tests, experiments, archival metadata, evaluations and candidate evidence with exact repo/commit/blob/consumer provenance. Record limitations and never promote research to accepted product behavior without source-owner verification.
2. Use HQ's existing indexing and documentation generation for shared development views. Do not maintain another authoritative roadmap, capability status store, project register, graph engine or documentation generator in Labs.
3. For Z.ai, identify conceptual overlaps, unique components and orphaned value; log proposed best-of-breed decisions, explicit owner approvals and unresolved evidence in existing `zai/governance/` files. These are campaign *evidence*, not status authority.
4. Review material before archiving or removing it. A matching hash alone proves identity, not safe source deletion. Preserve dependency consumers, build/test hooks and rollback, and repair links after any approved retirement.
5. **Never commit credentials, private owner messages, provider sessions, browser profiles, raw private logs, sensitive Git bundle history or confidential recovered-source material.** Never copy SOFTWARE_HQ quarantine artifacts here. If existing public Labs content appears sensitive, report a bounded publication blocker without exposing values.
6. Cloud tests cannot prove physical Android, actual owner Chrome sessions or Windows signed builds. Report actual command exits and missing verification explicitly.
7. No force-push, history rewrite, concurrent destructive cleanup or source-owner effect changes without explicit authorization.

Each campaign round must identify four pinned repos and deliver a manifest, conceptual overlap/unique-value dispositions, decision and open-item deltas, tests, missing gates and bundle-restoration evidence. Git bundles are **not** public-safe by default and must not be committed into this repository.

## Current 2026-11-23 engineering intake

Read [ARBOR now→future wiring audit](https://github.com/owenservera/ARBOR/blob/main/docs/product/ARBOR-NOW-FUTURE-WIRING-20261123.md) and [R002 delta to the existing execution pack](ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/PACKS/EXECUTION-20261123-R001/WIRING-DELTA-R002.md) before touching any current VIVIM/ARBOR plan. #33 is the authoritative oversight checklist and Labs `TASK-CLAIMS.md` is the atomic ownership overlay. Publication is not cross-session acknowledgment or an active agent claim. Keep Labs public-safe and do not copy private desktop/provider logs here.
