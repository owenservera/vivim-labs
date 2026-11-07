# Platform IP to a small, real mobile bootstrap loop

## Recommendation

Preserve the original platform's boundaries, not its entire implementation stack. Build a thin host-owned core and use the mobile Chat/bootstrap loop to exercise it immediately. The next useful deliverable is a real phone interaction, not a completed general-purpose platform kernel.

This is a source-based integration recommendation, not an implementation, security certification, completeness audit or legal ownership opinion. Historical repositories remain untouched. No model-provider APIs were called.

## Sources and evidence limits

The [source registry](VIVIM-PLATFORM-IP-SOURCE-REGISTRY.md) was read in full at vivim-web-harness main `2db2877`. Twenty-five pinned source files were retrieved read-only from five historical repositories, with exact commit references and SHA-256 hashes recorded locally. Core memory/context documents, the three Omega decisions and kernel/context/SDK source regions were inspected; adjacent rights, privacy, history, collaboration, Sentinel and marketplace PRDs were sampled for their stated concepts. This is not an execution audit of those repositories. The BCP-dev marketplace-future source was not inspected in this pass.

Important distinctions:

- The platform [master index, lines 83–91](https://github.com/owenservera/vivim-platform/blob/18bf523edb887374770a49db61b08204fdd4f141/DEMO_DOCUMENTATION/00-MASTER-INDEX.md#L83-L91) explicitly describes the demos as mock-data, self-contained experiences. Their product claims are not runtime acceptance evidence.
- [SDK memory-node, lines 444–524](https://github.com/owenservera/vivim-sdk/blob/6450c6caf1f0b5413477f2d87d4cfd5dac462a63/src/nodes/memory-node.ts#L444-L524) implements text filtering and word-overlap similarity; its semanticSearch method explicitly falls back to text search. Useful interface evidence, not verified vector retrieval.
- The SDK token type files in vivim-platform and vivim-sdk are byte-identical at the registered snapshots. Do not count them as independent implementations or infer rights-tier enforcement from a token type declaration.
- The historical [memory analysis, lines 85–145](https://github.com/owenservera/vivim-platform/blob/18bf523edb887374770a49db61b08204fdd4f141/server-analysis/memory-system.md#L85-L145) describes extraction/consolidation/search gaps. It is itself an analysis document, not a fresh runtime finding; together with the optimistic deep-dive docs, it is a reason to verify behavior before reuse.
- Omega decisions label themselves ratified and list evidence. This pass read those decisions, not the gates or their historical runtime receipts. Their invariants are architectural guidance, not automatically established properties of the new harness.

## What to carry forward now

### 1. One protected global Personal AI, with owned records and stable references

Keep the current `primary` Personal Agent, memory, Work and continuity services as the owner. Configuration, provider bindings and future specialist agents refer to that primary; none replaces it. A paired phone is a view/controller for the host-owned state, not a second source of identity.

The [Fabric architecture, lines 22–77 and 190–210](https://github.com/owenservera/edge-pwa/blob/b788dd9cdcc88cbc6abb348dbbf22b9541c91ad9/FABRIC_ARCHITECTURE.md#L22-L77) supplies the useful pattern: stable identity, scope, lifecycle, evidence and links. Its [additive integration decision, lines 639–652](https://github.com/owenservera/edge-pwa/blob/b788dd9cdcc88cbc6abb348dbbf22b9541c91ad9/FABRIC_ARCHITECTURE.md#L639-L652) argues against replacing working engines.

**Small implementation:** retain existing IDs and stores; add source/version references where a real bootstrap action needs them. Do not create a universal Node class, migrate every record, invent a graph database, or implement novelty/tension/weave scoring first.

### 2. One host-built context packet per meaningful AI request

The [Context Engine](https://github.com/owenservera/vivim-platform/blob/18bf523edb887374770a49db61b08204fdd4f141/server/docs/CONTEXT-ENGINE.md#L60-L124) separates identity, preferences, topic/entity knowledge, conversation history and the current user request. The later [ContextAssemblyEngine, lines 1–42](https://github.com/owenservera/vivim-final/blob/8a798f91d52574cb3d7db579da516bc1c4aee7b7/src/engines/context-assembly.ts#L1-L42) exposes layers, sources and a budget. [Omega D-433, lines 7–43](https://github.com/owenservera/vivim-omega/blob/79ee558d6c7ea77c20f220db2341adaa57b11d44/docs/decisions/D-433-context-spine.md#L7-L43) makes core-owned, inspectable assembly the stronger boundary.

**Small implementation:** turn the planned priming brief into a versioned ContextPacket: primary identity reference, explicit relevant preferences, current app/configuration reference, actual capability/readiness references, recent accepted decisions and current request. Store ordered source references, budget policy/version, omitted-item reasons and a content digest. Keep timestamps outside the digest's canonical content. This lets us answer what the AI was told and why a suggestion was made.

The provider receives the approved bounded packet; it does not choose arbitrary local memory/files to read. A digest is an integrity aid, not encryption, a permission sandbox, or proof that a provider has forgotten older context. Native provider personalization/history can affect responses independently and must remain qualified as unknown.

Start with deterministic ordering and character limits. Do not implement all L0–L7 infrastructure, embeddings, thermodynamic optimization, distributed context caching or a tokenizer service before the first mobile iteration.

### 3. Memory grows from explicit user decisions first

The [Memory Engine, lines 41–99](https://github.com/owenservera/vivim-platform/blob/18bf523edb887374770a49db61b08204fdd4f141/server/docs/MEMORY-ENGINE.md#L41-L99) distinguishes preferences, goals, facts and episodes. That matters now; a full extraction/consolidation engine does not.

**Small implementation:** use existing Personal Agent memory and accepted configuration receipts. Preferred AI, an activated Chat choice and an explicit correction become source-linked decisions. Keep model-inferred preferences distinct from owner-confirmed choices. When the owner changes a preference, record the replacement and preserve its history rather than keeping contradictory current defaults.

Owner clarification: onboarding first populates a broad, structured personal-context foundation in the primary master workspace from what the selected AI actually knows—owner profile, preferences, working style, roles, projects, goals, routines and environment—not just Chat settings. This intake remains source-linked and provider-reported until confirmed or corrected through use; it does not overwrite confirmed context. The foundation persists locally and is structurally available to the Personal AI beyond its original provider thread.

The next context packet incorporates those decisions. That is an observable personalization loop without claiming autonomous semantic learning. Existing CANON/Immediate/Curated organization need not be migrated to every historic memory taxonomy.

### 4. Capabilities and executable configuration, not a new app per suggestion

The [Fabric feature thread](https://github.com/owenservera/edge-pwa/blob/b788dd9cdcc88cbc6abb348dbbf22b9541c91ad9/FABRIC_ARCHITECTURE.md#L706-L710) connects features to provider status and interface dependencies. The [kernel context code, lines 65–87](https://github.com/owenservera/vivim-final/blob/8a798f91d52574cb3d7db579da516bc1c4aee7b7/src/engines/kernel/kernel-context.ts#L65-L87) gives subsystems shared registry, state and provenance references. Reuse that separation, not the entire kernel runtime.

**Small implementation:** a real Chat capability, its small supported configuration catalog, and validated changes to presentation, placement and exposed controls. Keep three dimensions separate: declared implementation maturity, current runtime readiness, and whether the owner enabled it. A source file, an active toggle and a verified provider are not equivalent facts.

Let the AI propose two or three alternative Chat setups using the same capability ID and different supported configurations. Proposal IDs must be unique; capability IDs need not be unique across alternatives. A selected alternative becomes one checked local change, not an unrestricted code-generation/install job.

Do not migrate to the historic SDK, plugin runtime, Rust/Tauri stack or marketplace to make this possible. The existing plain-JavaScript mobile app, primary agent and versioned blueprint services provide enough working substrate.

### 5. Background work, authority and evidence as small enforceable boundaries

[Omega D-431, lines 7–43](https://github.com/owenservera/vivim-omega/blob/79ee558d6c7ea77c20f220db2341adaa57b11d44/docs/decisions/D-431-watch-substrate.md#L7-L43) separates a headless trigger from authorized action; [D-432, lines 7–43](https://github.com/owenservera/vivim-omega/blob/79ee558d6c7ea77c20f220db2341adaa57b11d44/docs/decisions/D-432-resource-governor.md#L7-L43) separates permission from resource budget. That maps directly to background priming.

**Small implementation:** when the preferred provider becomes ready, enqueue one idempotent preparation intent on the existing worker. Apply the already-authorized installation policy, finite message/deadline budget, exact thread binding and existing effect journal. Keep readiness/queue state alive independently of the custom UI. Reconnect reloads state; it does not resend.

Link the resulting context packet, request, proposal, selected change, configuration revision and local outcome. Use current journals/CAS/receipts; do not start a new event platform, generic watcher language, multi-device governor or observability project.

This is also the first useful rights/Sentinel boundary: selected-provider context disclosure, no secrets in payloads, no automatic public sharing, and truthful local receipts. It is not the historic T0–T5 classifier, TDASS economics, zero-knowledge relay, training-misuse detection or cryptographic proof system.

## The fastest meaningful phone milestone

1. Open the paired dev app and reach the protected Personal AI even if custom app configuration is empty.
2. See one genuinely verified AI connection and select the preferred builder.
3. During pairing, let the host create one fresh onboarding conversation, collect a broad structured baseline and persist it into the existing primary workspace, then assemble the versioned context packet from that foundation.
4. Receive two or three real alternative Chat setups.
5. Preview and apply a supported change to Chat's presentation, placement or exposed controls.
6. Say what to change; use the same prepared conversation and current configuration to propose the next revision.
7. Reopen and undo a local change, preserving the global AI, memory and existing Talk history.
8. Observe that an explicit accepted preference influences the next setup suggestion.

This is the first acceptance loop. Do not wait for a complete historical-IP recovery, all-provider readiness, full permissions editor, generic agent system or final architecture taxonomy. Do not bypass real effect safety or present mocked status as ready to make the loop appear complete.

## Explicitly defer, while retaining extension points

- Vector databases, automatic extraction/consolidation, knowledge-graph scoring and full dynamic-context optimization. Keep memory/source references and replaceable retrieval/assembly boundaries now.
- Complete rights tiers, TDASS, token standards, marketplace/escrow, reputation economics and legal enforcement. Keep owner/scope/purpose/publication boundaries now; do not claim these systems ship.
- Sentinel's claimed thirteen-algorithm training detection and zero-knowledge proofs. Keep local execution evidence now; do not label that as training-misuse detection.
- Peer-to-peer collaboration, CRDT sync, identity federation and cloud memory replication. Keep stable scoped identifiers and a single host authority now.
- Custom agents and modes. Keep the protected primary and capability references now so specialists can later be scoped under it.

The current relay's authentication and local-first storage do not establish end-to-end encryption or zero-knowledge operation. Sending a context packet to a signed-in web AI shares that content with the selected provider. Historic privacy PRDs are not a substitute for the actual data path.

## Recent cross-repository harvest: adopt at the point of use

The [cross-repository opportunity register](../research/CROSS-REPO-CAPABILITY-HARVEST-2026-10-30.md) at main `251286e` complements the historical IP registry; neither replaces `docs/roadmap.json`. Its `H01`–`H11` IDs are namespaced here as **harvest:Hxx**, not the unrelated roadmap H-items.

This final onboarding research pass pinned and retrieved twelve planning/contract documents from four active repositories. README/reality/boundary documents and relevant blueprint sections were inspected; no foreign code was executed or imported. License and implementation dependency reviews remain required before any future source-code transplant; no such transplant is needed for the first loop.

### W1: bounded compatibility and evidence decisions

- **AION `266221e4267f71624556492cc6ab20302e95c254`** — [README](https://github.com/owenservera/aion/blob/266221e4267f71624556492cc6ab20302e95c254/README.md), [state protocol](https://github.com/owenservera/aion/blob/266221e4267f71624556492cc6ab20302e95c254/workspace/aion/STATE-PROTOCOL.md), [Fabric contract index](https://github.com/owenservera/aion/blob/266221e4267f71624556492cc6ab20302e95c254/workspace/aion/agent-fabric/README.md). Source explicitly distinguishes design/fixtures from installed or live behavior. **harvest:H01/H11: adapt** source revision, observation time, evidence references, explicit unknowns and outcome vocabulary into existing receipts. Vivim's primary/World remains owner of private user context. **harvest:H07: defer** external agent runtime/Fabric integration; no AION agent roster, event bus or execution authority is installed here.
- **VOMEGA `12c853259f60cf3a883c7fb1c9f329d696606d64`** — [README](https://github.com/owenservera/VOMEGA/blob/12c853259f60cf3a883c7fb1c9f329d696606d64/README.md), [reality boundary](https://github.com/owenservera/VOMEGA/blob/12c853259f60cf3a883c7fb1c9f329d696606d64/.project/REALITY.md), [seed contract index](https://github.com/owenservera/VOMEGA/blob/12c853259f60cf3a883c7fb1c9f329d696606d64/seed-docs/README.md). Local semantic interpreter/corpus evidence is reported, while live provider/account/prompt.send remains unproven in that source. **harvest:H02: adapt** intent → typed capability/target → validated proposal → authority → execution/result; use the existing harness to execute. Do not import the whole NLCL/runtime/kernel or claim VOMEGA supplies a verified provider driver. **harvest:H10: defer** floating desktop command environment.
- **VIVIM Harness `84a6b0f1bf698fc36e39327d3a82e21cfbbd6c18`** — [README](https://github.com/owenservera/vivim-harness/blob/84a6b0f1bf698fc36e39327d3a82e21cfbbd6c18/README.md), [V1 blueprint](https://github.com/owenservera/vivim-harness/blob/84a6b0f1bf698fc36e39327d3a82e21cfbbd6c18/docs/product/V1-INTEGRATED-APP-BLUEPRINT.md#L144-L205). **harvest:H03/H08: reuse** existing in-repository `studio/` primary, memory, Work, versioned app/configuration and provenance boundaries; compare an exact donor file only if the local copy is missing a necessary behavior. Observed provider content, derived interpretation and owner decisions stay distinct. No second capture process, message database, identity authority or raw-history import is introduced. Deep search/transfer can follow the first loop.
- **Web Harness main `251286e`, dependency PR #36 `e7234d4`, Build scaffold PR #35 `b577714`** — **harvest:H04: reuse** the current router/journal/authority/phone transport and coordinated effect owner. **harvest:H05: adapt** local configuration receipts and one undo using existing CAS; this is a proposed synthesis, not an already verified whole time-travel product. **harvest:H09: defer** public Share/feed integration; private onboarding records must never be published automatically.
- **PC HQ `2e5d70dbde9f127c24c920ba0d09823726b31969`** — [README](https://github.com/owenservera/hq/blob/2e5d70dbde9f127c24c920ba0d09823726b31969/README.md), [scope contract](https://github.com/owenservera/hq/blob/2e5d70dbde9f127c24c920ba0d09823726b31969/contracts/PC-HQ-SCOPE-BOUNDARY.md#L57-L64). Source describes design-v1, not a live-PC health service. **harvest:H06: reference/adapt** timestamped read-only host facts if a real collector is available. For the first loop, report only actual browser ownership, relay reachability and storage checks performed by this host; absent PC HQ evidence is not green and is not an onboarding prerequisite. Never import OS repair or system-remediation authority.

These are initial contract/ownership triage decisions, not completion of every W1 implementation/acceptance requirement. Any later concrete code import needs its own verified row, overlap check, dependency/license review and observed acceptance. All adopted records are private local data unless a separate explicit publication operation says otherwise. Contract-only adaptation adds no runtime dependencies.

### W2: one small bootstrap capability manifest

Extend the existing capability/readiness projection rather than creating another registry. Carry the harvest's fields: `capabilityId`, `source`, `readiness`, `inputs`, `effects`, `approvalScope`, `reversible`, `evidence`, `version`. Preserve the distinctions between implementation maturity, fresh runtime readiness and user-enabled configuration.

The first entries cover only working onboarding/Chat operations: observe readiness, collect baseline through the selected web AI, persist validated primary context, generate/preview supported Chat changes, apply a selected local change and restore its prior configuration. Each has a real host handler or is explicitly unavailable. Schema declarations alone do not make it available.

The global Personal AI can read this inventory and propose semantic operations against it; the installed harness resolves targets and authority and performs the effects. This is the immediate combination of VOMEGA's command discipline, AION's evidence discipline, Studio's local World and the existing Web Harness executor—without installing four competing control planes.

### Adoption timing and the research stop line

**Now, as part of the first mobile loop:** the small W1 ownership map above, W2 manifest, necessary W5 host facts, one W3 configuration revision/undo and the core W6 phone walkthrough. Populate the master Personal AI context workspace during pairing; use that foundation for real Chat design proposals.

**After that loop works:** W4 deeper saved-World integration, richer research recommendations and useful saved-context search. **Later:** custom agent Fabric, multi-mode branching, broader desktop semantics and public Share. None of these gates the initial phone iteration.

Research expansion stops here for the initial slice. Reopen a donor repository only for a specific missing behavior encountered during implementation; do not perform another broad inventory before building. Maintain the authoritative roadmap rather than creating a parallel execution backlog in this note.

## What changes in the existing plan

Treat the detailed implementation plan as a reference and safety checklist, not eight architecture projects that must all finish before using the phone. Execute a narrow vertical loop: existing primary and readiness, one ContextPacket/Chat catalog, one primed session, one persistent Chat change, protected access and one undo. Carry only the schema fields exercised by that loop.

After that loop works, iterate on the phone: improve useful control/layout choices, context selection and remembered preferences one demonstrated need at a time. Historical IP recovery proceeds alongside, with pinned attribution and no cross-repository runtime imports.

No production code, dependencies, provider conversations, deployment or original source files were changed by this research note.
