# Proposal 5 / 03 — Capability Spaces, Internal Source Placement and Labs Graduation

**Architectural unit:** one source-owned capability space with typed operations, views, configuration and test proof. A space is an in-repo *organization and interoperability boundary*, not its own service, database, permission owner, independent app or runtime.

## Primary mapping of donor sources to ARBOR

| Donor source | Internal ARBOR location | First master integration transformation | Beta disposition |
| --- | --- | --- | --- |
| Harness src-tauri | src-tauri/ | Import source preserving layout; coordinated Tauri v2 security update, origin hardening, process identity/credential transfer fix, internal native bridge | MUST RUN |
| Harness apps/vivim-host | apps/vivim-host/ | Source-preserving import; keep ONE action catalog/dispatcher/World effect journal and current endpoints; publish typed read/commit contracts | MUST RUN |
| Harness apps/mobile-relay | apps/mobile-relay/ | Source-preserving PWA, auth, pairing and device limits; ensure native semantic projection forwarded via internal Host bridge; no second phone application | MUST RUN |
| Harness studio/ and needed harness/ | studio/, harness/ | Preserve signed-in Chrome profile/webapp provider controls and exact-send leases; only internal process/bridge, not external repo server | MUST RUN for supported provider |
| Harness core/programmability | core/programmability/ | Keep current typed Rust validator and tests; bridge to canonical native protocol, no second policy writer | MUST BUILD |
| HQ research/rust-knowledge-graph | native/hqkg/ | Import current crate, tests, algorithms, typed DAG, engine registry without rewrite; one canonical engine in ARBOR | MUST BUILD |
| HQ hq/bin, hq/lib and dependency closure | hq/ | Bring source indexer, registry, search, context, wiki and capability introspection **inside ARBOR**; preserve operational tests | MUST RUN for dev-intelligence beta |
| TierFlow renderer, command/compiler, tests | packages/tierflow/ | Reviewed source snapshot, compatible Bun/TS package, one visual publisher, internal view adapters | MUST BUILD; beta views validated |
| Labs Node 0 Prime/R001 | native/node0/, contracts/, tests/ | Ratified architecture and acceptance corpus; replace blocking Mutex + per-edit thread spike, retain only tested pure routines | MUST BUILD a narrow safe native slice |
| Labs UI R005/R006/R007 | packages/tierflow/, apps/vivim-host/public/ or apps/mobile-relay/ view adapters | Port verified semantics/visual controls selectively; R007 full interpretation hypergraph is a design target and may be beta preview | PARTIAL/VIEW PREVIEW |
| Labs personal-knowledge lifecycle and DCB | apps/vivim-host/ owner ingestion + hq/ pure algorithms + capability-space context-dcb/ | Consent/evidence/candidate/verified capture and source selection in existing Host; source-backed DCB reads only | PARTIAL if proven |
| Labs voice-to-tasks | capability-spaces/voice-to-tasks/ + existing voice source | Import verified capture/structured task contract and tests; effectful task write through Host, not new task store | Verify day-0 feasibility |
| Labs Chronos/automation | capability-spaces/chronos-automation/ + existing Host mission scheduling | Import deterministic time logic and contracts only after source proof; no second clock daemon | SOURCE-SCOPED |
| Labs AutoWiki/Rock Core | internal hq/ wiki algorithm + personal view projection + config/storage quota | Personal knowledge views use Host World source, not copy of HQ dev Wiki; bounded materialization | BETA PREVIEW or verified read path |
| Labs app builder | capability-spaces/app-builder/, TierFlow view, current Host authorization | Composer over verified typed ports; no arbitrary code executor or new unreviewed write grant | PREVIEW until actual build proof |
| Labs extension-points / capability observatory | capability-spaces/ metadata + integration capability-index | Import verified interface seams and build/test status taxonomy; no generic third-party plugin marketplace | DECLARATIVE ONLY |
| Labs personal graph/context/zero intent graph | native/hqkg/, Host personal World and context views | Import accepted graph algorithms/metadata after source and data scope proof; no duplicate personal graph store | PARTIAL |
| Legacy private vivim and vivim-platform | import-manifest references, selectively chosen UI assets | Review only; old cloud API assumptions and tracked node_modules/build data prohibited | OPTIONAL DONORS |

## A real capability space entry

Example `capability-spaces/host-control/`: contains a **generated projection** of currently registered Host actions, canonical refs to `apps/vivim-host/control/action-catalog.v1.json`, parsed schema, safe UI projection bindings and test status. It does **not** have another independently maintained action registry.

Required fields:
`capabilityId, canonicalOwner, internalImplementationPath, inputSchemaRef, outputSchemaRef, effectClass, authorizationDomain, sourceScopes, currentVersion, dependencies, configPorts, viewPorts, releaseStatus, proofRef`.

Status taxonomy:
`DESIGNED`, `SOURCE_PRESENT`, `BUILD_VERIFIED`, `RUNTIME_WIRED`, `DEVICE_ACCEPTED`, `AVAILABLE_NOW`, `BLOCKED`, `DEFERRED`, `RETIRED`. A source's self-declared `source_wired` is not a live availability proof.

## Keep current capability identities

- Existing Host catalog v6 includes `vivim.status.show` and its other registered actions; migrate all observed actions with identical risk/approval semantics. Do not invent `full.project.update` because UI research uses it as a demonstration.
- The HQ semantic registry has namespaced IDs and versioned statuses; import its *logic* and adapt descriptions for development-intelligence capability spaces; do not promote HQ registry entries into Host effect permissions.
- TierFlow visual commands are inert candidate/view operations. They reference the corresponding typed capability and Host authority.
- Personal DCBs are source-linked read/assembly transformations. Outbound provider disclosure remains an explicit Host-authorized action.
- Voice, media, missions and app-builder share core capability refs; feature UI cannot invent alternate completion or approval status.

## Labs promotion gates

Every Labs input must have a `sourceSha, sourcePath, ownerDecision, implementationStatus, targetPath, requiredTransform, testEvidence, privacyClass, retirementCondition`. Promoting a PRD or UI concept means **the contract is implemented when it reaches its own gate**, not that documentation magically becomes runnable.

Source tree selection happens once and moves **into** these in-repo owners. No `labs/...` runtime import, no external Labs test fixture loading, and no Labs server.

## Internal self-consistency requirement

One release artifact knows every capability owner and internal implementation path. The same canonical IDs and source state appear in desktop UI, phone, CLI/diagnostics, HQ internal code index, TierFlow view and Host effect receipts. Unsupported features show accurately `preview/blocked`; no UI label claims `verified` merely because a file is present.

**No external HQ/TierFlow/Harness runtime bridge is permitted.** "Adapter" here means a code boundary between two modules *inside the same ARBOR product and release*, not a link to another repository.
