# L05 — Source, planning packages, tools and handoff links

Do not use this index as evidence of capabilities that were only drafted. **Re-fetch current source and check paths before relying on them.** The curated links intentionally include shared authority and lane-specific packages so the folder can remain an agent's primary landing page for the entire remaining campaign.

## Canonical shared sources
- [Master deliverables #33](https://github.com/owenservera/ARBOR/issues/33)
- [Signed-in AI delivery #32](https://github.com/owenservera/ARBOR/issues/32)
- [Canonical same-source release and device proof #31](https://github.com/owenservera/ARBOR/issues/31)
- [CI runner #24](https://github.com/owenservera/ARBOR/issues/24)
- [Current Labs task claims](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md)
- [Current Planning SITREP](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/SITREP.md)
- [Planning task catalogue (37 candidate bundles, not canonical backlog)](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/MASTER-PLAN-R001.md)
- [Planning source reuse map](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/SOURCE-REUSE-MAP-R001.md)
- [Planning acceptance handoff](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/ACCEPTANCE-AND-HANDOFF-R001.md)
- [Planning bundle directory](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/BUNDLE-INDEX.md)
- [Oracle mission and authority](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/ORACLE/AGENTS.md)
- [ChatGPT Dev mission](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/CHATGPT-DEV/AGENTS.md)
- [Current canonical UI closure](https://github.com/owenservera/ARBOR/blob/main/docs/product/CANONICAL-UI-SURFACE-CLOSURE.md)
- [100-step physical acceptance ledger](https://github.com/owenservera/ARBOR/blob/main/docs/product/CANONICAL-SURFACE-100-STEP-LEDGER.md)
- [Five-lane strategic audit](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/FIVE-LANE-OPERATIONS-R001/STRATEGIC-AUDIT-R002.md)
- [Shared five-lane directive](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/FIVE-LANE-OPERATIONS-R001/DIRECTIVE.md)
- [Current five-lane manifest](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/FIVE-LANE-OPERATIONS-R001/manifest.json)
- [Workflow status / safe pickup](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/PACKS/EXECUTION-20261123-R001/ZCODE-WORKFLOW-REALITY-R005.md)

## Lane-specific source and evidence
- [Current run auditor](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/scripts/zcode-workflow-run-status.ts)
- [Current model workflow reality](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/PACKS/EXECUTION-20261123-R001/ZCODE-WORKFLOW-REALITY-R005.md)
- [Existing source-pinned regression DWF](https://github.com/owenservera/ARBOR/blob/main/.zcode/workflows/arbor-product-verification-parallel.dwf.ts)
- [Existing model registry/config (inspect current saved config locally)](https://github.com/owenservera/ARBOR/tree/main/.zcode)
- [CI issue #24](https://github.com/owenservera/ARBOR/issues/24)
- [Actions runs](https://github.com/owenservera/ARBOR/actions)
- [Current source-to-device evidence pack](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/PACKS/EVIDENCE-CLOSURE-R003/README.md)
- [Local tests and observed receipts](https://github.com/owenservera/vivim-labs/blob/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/STREAM-ARCHIVE/DEPLOYMENT-20261122.md)
- [Original 100-step acceptance tracker](https://github.com/owenservera/ARBOR/blob/main/docs/product/CANONICAL-SURFACE-100-STEP-LEDGER.md)

## Tool stack and permissions

- **GitHub (cloud primary source writer):** current ARBOR and Labs branches, compare SHA, claim conflict check and source tests, direct main commit with file-owner scope and no force push
- **Existing ZCode 0.18 and native workflows:** inspect current .dwf.ts bodies and runtime admission. A/E historical component workflows are reference-only; `arbor-life` can restart production Host, and `arbor-chatgpt-bridge` can actually send — never auto-launch them
- **Desktop Commander:** local Windows/Host/Studio/ZCode process inspection, exact-source test/release/device evidence, not default product-source editing
- **Planning/Oracle:** no second task ledger; use grants/provenance/claim handoffs and only authorized personal content
- **Automations:** one scoped hourly recurring cycle per dedicated ChatGPT lane when available; verify actual enabled state, next run and environment before claiming continuity
- **Source freshness:** the URLs above resolve to moving main, so record the exact SHA you actually tested; otherwise mark historical

## Handoff protocol

`task ID | current ARBOR SHA | Labs claims SHA | active executor & exact paths | implemented commit | test command/exit | observed Host/provider/GPUI/phone evidence | blocking dependency | next atomic step | recipient acceptance`.

Read [full lane direction](AGENTS.md), [launch prompt](CHATGPT-LAUNCH.md), [the shared policy](../DIRECTIVE.md), [the R002 safety audit](../STRATEGIC-AUDIT-R002.md), and [receipt template](HANDOFF.md) before taking any file claim.
