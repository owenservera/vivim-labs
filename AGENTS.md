# VIVIM Labs — agent and contributor rules
**Cloud-first agent routing:** For ARBOR and related engineering, ChatGPT writes/tests in its own cloud workspace and pushes through GitHub; Desktop Commander only reads local context or performs specifically authorized post-push device/runtime work. See [mandatory Cloud-First Coding Policy](ZCODE-DESKTOP-COMMANDER-OPERATIONS/CLOUD-FIRST-CODING-POLICY.md). ZCode does not automatically inherit primary coding ownership.


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
