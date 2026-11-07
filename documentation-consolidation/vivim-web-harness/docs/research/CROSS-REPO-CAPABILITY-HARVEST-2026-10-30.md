# VIVIM — Cross-Repository Capability Harvest and Integration Opportunities

**Recorded:** 2026-10-30  
**Scope:** Work developed across active repositories during October 2026, especially the past month.  
**Status:** Research and candidate-integration backlog; not implementation authorization or proof of live behavior.  
**Owner:** VIVIM product / integration planning.

## Why this is separate

This is a *new* opportunity register for recent active engineering work. It does not replace or modify `docs/ip/VIVIM-PLATFORM-IP-SOURCE-REGISTRY.md`, which is already the historical platform-IP inventory undergoing absorption. This file focuses on *incremental operational and UX value* discovered in recent sibling repositories. `docs/roadmap.json` remains the authoritative execution tracker; do not automatically duplicate its items here.

## Integration rule

**Harvest behaviors and stable contracts, not entire codebases.** Before moving code, compare overlap, live evidence, ownership, tests, security assumptions, license and dependency footprint. Prefer a single canonical owner for each capability. No undocumented browser write, no API-based AI-provider bypass, no unapproved action, no second controller. Preserve local-first ownership, provenance and explicit sharing authority. A README, mock or design is not live proof.

## Candidate opportunity register

| ID | Priority | Candidate | Source / starting files | VIVIM destination and user value | Stage / evidence required |
|---|---|---|---|---|---|
| H01 | **P0** | Research intake, deduplication and evidence evaluation | [AION](https://github.com/owenservera/aion): `README.md`, `workspace/aion/STATE.json`, `workspace/aion/ops/chrome-relay/DEVELOPMENT-INDEX.json` | **Research service**: gather discoveries across repos, turn evidence into actionable, nonduplicate personal-agent upgrades | Inspect implementations and provenance; AION README confirms governance and evidence structure but not a finished service |
| H02 | **P0** | Semantic command grammar and capability dispatch | [VOMEGA](https://github.com/owenservera/VOMEGA): `README.md`, `seed-docs/README.md`, `.project/REALITY.md` | **Bootstrap canvas**: describe a desired behavior in natural language; resolve to an explicit capability proposal, preview, grant and reversible execution | Inspect command schema and live local proof; avoid importing a separate orchestration kernel |
| H03 | **P0** | Capture, local data, provenance and saved personal-world views | [vivim-harness](https://github.com/owenservera/vivim-harness): `README.md`, `ARCHITECTURE.md`, `docs/product/V1-INTEGRATED-APP-BLUEPRINT.md` | **World storage and timeline**: saved provider material, source-linked artifacts, local search, reversible configuration history | Inspect schemas and code vs new harness to avoid parallel captures/stores; source documents only partial integration |
| H04 | **P0** | Provider routing, journal, approvals, mobile relay and fleet | [vivim-web-harness](https://github.com/owenservera/vivim-web-harness): `GOALS.md`, `docs/roadmap.json`, `harness/`, `studio/` | **Execution engine behind the phone**: validated connections, send-once operation, phone-controlled chats and agents | Existing authoritative roadmap owns build/repair; validate relay end-to-end and recovery before enabling bootstrap actions |
| H05 | **P0** | User-visible proof and rollback / time travel | Combine AION evidence protocol, Web Harness revision/journal and VIVIM Harness saved-world concepts | **Bootstrap Time Travel**: record each proposed feature, approval, applied configuration, observed result and reversal; maintain multiple modes | **PROPOSED synthesis**, not an existing verified feature; define append-only event schema and compensating actions, not magic undo for external sends |
| H06 | **P1** | Host health and readiness diagnostics | [hq](https://github.com/owenservera/hq): `README.md`, `architecture/COVERAGE.md`, `contracts/PC-HQ-SCOPE-BOUNDARY.md` | **Pairing/readiness card**: storage, browser ownership, host connectivity, OS constraints, actionable diagnosis | PC HQ README calls current work design-only; consume read-only snapshots, leave hardware/OS remediation under PC HQ authority |
| H07 | **P1** | Agent Fabric and multi-actor collaboration contracts | [AION](https://github.com/owenservera/aion): `workspace/aion/canon/AGENT-FABRIC-ARCHITECTURE-2026-10-20.md`, `workspace/aion/agent-fabric/README.md` | **Agents tab**: roles, task provenance, interrupted-work recovery and coordinated actions | AION README explicitly marks design-only, no live end-to-end I1/I2/I3 proof; prototype contracts before any execution adoption |
| H08 | **P1** | Context packaging, projects, search, transfer and provenance | [vivim-harness](https://github.com/owenservera/vivim-harness): `docs/product/V1-INTEGRATED-APP-BLUEPRINT.md`, `docs/TECHNICAL-README.md` | **Personal memory building blocks**: take chosen saved context across supported providers without pretending to modify their remote histories | Compare existing schema/coverage; preserve observed vs derived vs user-written source labels |
| H09 | **P1** | Shareable, owner-approved execution/artifact records | [vivim-web-harness](https://github.com/owenservera/vivim-web-harness): `docs/roadmap.json` entries P03 and share implementation | **Share**: read-only published snapshots or public feed with explicit publish/revoke controls and redaction | In progress at recorded snapshot; prohibit raw credentials, private chat content, command bodies or secrets in public telemetry |
| H10 | **P2** | Broader desktop semantic UX / command box | [VOMEGA](https://github.com/owenservera/VOMEGA): `README.md`, `seed-docs/README.md` | **Optional advanced desktop surface** after mobile-first Gate 1 | Keep as independent experiment until user journey and command safety are demonstrated |
| H11 | **P2** | Cross-repo operational state and source truth | [AION](https://github.com/owenservera/aion): `workspace/aion/STATE-PROTOCOL.md`, `workspace/aion/STATE.json` | **SITREP and evidence**: distinguish verified capabilities, proposed ideas and blocked integrations in VIVIM | Design mapping only; do not turn GitHub documentation into a fake live agent state |

## Suggested product architecture and ownership

- **Web Harness:** only current signed-in webapp action owner, approval/journal and phone relay; do not create a second CDP owner.
- **Personal World:** adopt a single versioned local data store and provenance vocabulary after a schema comparison with `vivim-harness`.
- **Bootstrap:** thin orchestration/UX layer on top of explicit capabilities. It proposes actions; user approvals remain independent gates.
- **Semantic commands:** adapt VOMEGA grammar as a contract; the harness remains the executor.
- **Research:** consume AION evidence and deduplication practices, without duplicating its research authority.
- **Host readiness:** consume PC HQ diagnostics as read-only external evidence, not host-administration authority.
- **Public sharing:** separate sanitized, explicitly published snapshots from private local objects.

## Highest-value work packages (new proposals, not duplicate roadmap entries)

### W1 — Cross-repo compatibility and evidence matrix (first)
For H01–H09, record source path, commit SHA, implemented surface, acceptance evidence, target owner, overlapping code, license, dependencies, data sensitivity and decision `reuse | adapt | reference | defer`. Reject imports without evidence.

**Acceptance:** one row per capability with a verified source revision, overlap resolution and owner; no duplicated runtime controller.

### W2 — Bootstrap capability manifest
Combine the existing provider capability schema with semantic command metadata: `capabilityId`, `source`, `readiness`, `inputs`, `effects`, `approvalScope`, `reversible`, `evidence`, `version`.

**Acceptance:** the phone can show available/unavailable features as user-readable proposals, never pretending a designed capability is installed.

### W3 — Time-travel configuration journal
Append events for proposal → consent → activation → observed result → deactivation/switch-mode. Snapshot only locally owned configuration; external provider writes are not reversible unless an explicit compensating action is known.

**Acceptance:** restore a prior *configuration* snapshot without resending prompts or altering non-owned browser tabs.

### W4 — Local-world data bridge
Produce a minimal mapping between existing capture/provenance/saved-item structures and the new web harness. Preserve source URL, provider, timestamp, completeness, sensitivity, consent and import lineage.

**Acceptance:** one real saved conversation can be read through the new World view, with no source loss and no second writer.

### W5 — Host readiness during pairing
Expose browser-owner status, relay state, storage headroom and capabilities through a read-only status projection. Do not copy PC HQ's system-remediation code.

**Acceptance:** user sees a precise reason when pairing/activation is blocked and a safe manual remedy.

### W6 — Evidence-driven integration demos
One phone-first demo across the real host: pair → validate a provider → propose a capability → approve → execute once → save evidence to local World → switch mode → restore earlier configuration. Then opt-in sanitized Share.

**Acceptance:** end-to-end real user evidence, error/recovery receipts, privacy audit; no model-provider API calls.

## Integration sequencing

1. **Now:** W1 inventory, then W2 manifest; no broad code port.
2. **Gate 1:** W5 host readiness, W3 reversible configuration, then W6 phone bootstrap acceptance.
3. **Following Gate 1:** W4 local-world bridge and H01 research/evidence-driven recommendations.
4. **Later:** H07 agent fabric, H08 deep context transfer, H09 richer sharing, H10 desktop semantic environment.

## Risks and no-go cases

- Don't merge `vivim-harness` and `vivim-web-harness` blind: capture, model routing, persistence and browser ownership overlap.
- Don't imply past AION research/fabric designs are a live autonomous system.
- Don't treat VOMEGA simulated quick tests as verified provider writes.
- Don't offer “undo” of an already-sent provider message or public share without defined compensating/revocation semantics.
- Don't expose private conversations or local runtime artifacts to `vivim.live` or `share.vivim.live`.
- Don't build a second roadmap: promote selected W-items into `docs/roadmap.json` only when scheduled.

## Evidence inspected for this edition

Read on 2026-10-30: `README.md` in `owenservera/aion`, `owenservera/VOMEGA`, `owenservera/vivim-harness`, `owenservera/hq`; `GOALS.md`, `docs/roadmap.json`, and `docs/ip/VIVIM-PLATFORM-IP-SOURCE-REGISTRY.md` in `owenservera/vivim-web-harness`. Paths listed as follow-up inspections have *not all* been opened individually. This is a high-value triage of identified active repositories, not a complete audit of all repositories or commit activity in the month.
