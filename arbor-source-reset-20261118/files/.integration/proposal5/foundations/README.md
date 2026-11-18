# ARBOR Proposal 5 / Foundational VIVIM Architecture — Research and Wave-Preparation Hub

**Date:** 2026-11-12 · **Program owner:** ARBOR foundational architecture/design workstream.
**Branch:** `work/p5-foundational-architecture-prep-r001` from `work/p5-pre-g-convergence-r001@8b23c73bf450738a76b256bf0fa17ecbf86db591`.
**Status:** **PREPARED_NOT_PROMOTED — SOURCE-BASED DESIGN + FOUR READY PRE-G WORKER PROMPTS (NOT EXECUTED)**.
**Write scope:** this directory only. No changes to A–F frozen workers, G, main, runtime, root locks, Host World, source vault or device releases.

## North star

VIVIM becomes an owner-controlled, reprogrammable local intelligence platform: **one natural-language/visual interaction**, compiled semantic wiki knowledge, bounded native Rust pure computation, source-linked explanation and actual Host-owned approval/receipt truth—on Windows with a **fast HTTPS PWA-first mobile** companion. Full APK delivery is **deferred**.

The current working code already has much of the foundation: native generational engine registry + typed DAG, pure programmability, Host NLCL/planner/dispatcher, HQ semantic registry/wiki/comment bridge, TierFlow visual command and direct PWA, Studio browser/agent/recipe/plugin scaffolds. But **those sources exist on isolated frozen branches**, not an accepted merged/runtime tested G product. The work here maps gaps and owns future architecture, not a replacement second platform.

## Start here

| Artifact | Purpose | Grounded scope |
|---|---|---|
| [FOUNDATIONAL-ARCHITECTURE.md](./FOUNDATIONAL-ARCHITECTURE.md) | Overall three-plane product architecture, authority boundaries, source-linked semantic fabric, native I/O, reprogrammability, personal knowledge, app builder, voice/media/automation, storage | Core architecture |
| [FOUNDATIONAL-PRD.md](./FOUNDATIONAL-PRD.md) | 20 concrete user journeys, 24 functional requirements, product and release gates | UX and product specification |
| [SOURCE-ATLAS.v1.json](./SOURCE-ATLAS.v1.json) | **43 actual selected code/research source blobs** pinned across frozen ARBOR A–F, Labs, HQ and pre-G slices, with observed vs unverified distinction | Source evidence, not exhaustive scan |
| [GAP-REGISTER.v1.json](./GAP-REGISTER.v1.json) | **43 current gaps** with owning lane, phase, priority, closure and negative test requirement | Closure register |
| [IMPLEMENTATION-WAVES.md](./IMPLEMENTATION-WAVES.md) | Readable **14 implementation waves / 100 atomic tasks** across pre-G, original sole G, post-G and later extensions | Human execution plan |
| [WAVE-MANIFEST.v1.json](./WAVE-MANIFEST.v1.json) | Machine-readable tasks, dependency and owner/promotion metadata for subsequent prompt generation | Not yet launched |
| [NATIVE-MACHINERY-AND-CONTRACTS.md](./NATIVE-MACHINERY-AND-CONTRACTS.md) | Real existing Rust EngineRegistry/typed DAG interfaces, proposed typed capability ABI, hot-swap/rollback and bidirectional event fabric with concrete Cargo proof | Native engineering |
| [PWA-FIRST-DELIVERY.md](./PWA-FIRST-DELIVERY.md) | Actual PWA/Host/static/grant/IPC/APK packaging code audit, owner fixes, physical Android browser gates | **PWA only, APK future** |
| [DECISIONS-AND-DEFERRED.md](./DECISIONS-AND-DEFERRED.md) | 22 architecture decisions, open questions and explicit deferrals | Govern against architectural drift |
| [VERIFICATION-AND-ACCEPTANCE.md](./VERIFICATION-AND-ACCEPTANCE.md) | Native/Host/HQ/Studio/TierFlow/PWA hard tests and negative corpus; real-vs-source evidence ladder | F/G acceptance |
| [PROMPT-FACTORY-SCAFFOLD.md](./PROMPT-FACTORY-SCAFFOLD.md) | How to generate future isolated focused A–F prompts from current G HEAD and actual source; copy template **not executed** | Next round prompt creation |
| **[Pre-G Launch Desk](./prompts/README.md)** | Four source-pinned, executable but **not started** worker prompts (PG00–PG03) and independent PGZ referee; unique branch/prefix for each | First focused cohort |
| [Prompt cohort registry](./prompts/LANE-REGISTRY.v1.json) | Machine-readable branch, owner, source-head and task coverage with 16 V8 structural checks | Prevent duplicate/later unsafe launches |
| [VALIDATION-REPORT.v1.json](./VALIDATION-REPORT.v1.json) | GitHub scope audit, 11/11 V8 program checks, 14 declared but UNRUN Node tests, real build/device status | Current evidence receipt |
| [contracts/capability-manifest.v0.schema.json](./contracts/capability-manifest.v0.schema.json) | Strict **proposed** machine-readable source-pinned pure capability manifest with no authority | Schema DESIGN_ONLY |
| [validate-program.mjs](./validate-program.mjs) / [validate-program.test.mjs](./validate-program.test.mjs) | Read-only source/wave/gap structural validator and 14 Node tests (declared; Node **UNRUN** here); 11 V8 structural checks observed | Program QA |

## Stage map

```text
PG-00 freeze and source truth           PRE-G — evidence only, optional
PG-01 Rust ABI / pure engines           PRE-G — isolated prototype preparation
PG-02 semantic wiki / trace / mirror    PRE-G — isolated prototype preparation
PG-03 PWA / security / Rock / privacy   PRE-G — isolated prototype preparation
                 │
          ORIGINAL G-01
     sole A–F serial merge and root workspace
                 │ (requires actual G GO + exact integrated HEAD)
                 ▼
W2-F0 secure coherent source/runtime + PWA no-APK packaging
W2-F1 versioned native hot-pure reprogrammability
W3-F2 semantic wiki compiler + deterministic language
W3-F3 VIEW ALL / AutoWiki / DCB context and source confidence
W3-F4 PWA UX / optional app-builder UI draft / voice / missions
W4-F5 Rock physical quotas / pure extensions / automation pilot
W5-F6 independent security, perf, Windows/phone/Chrome
W6-F7 PWA-first beta candidate + explicit owner GO
                 │
FB-01 future marketplace / full personal corpus / learning / automation / media / optional APK
```

**No implicit G advancement.** All post-G tasks require the **real integrated merged SHA** and accepted owner path. Beta-critical W2 trust/packaging fixes take precedence over optional semantic/product innovation. A high-value project must not delay or bypass the original P5 security and release gates.

## Four architectural principles

1. **Meaning is not authority.** Source comments compile into verified/diagnosed semantic read facts. No wiki, LLM, native pure processor or UI color creates a Host action.
2. **Reprogrammability means tested, source-attested evolution.** Existing native generational engine slots are reused; strict manifest/ABI and real Rust swap/rollback, actor and resource proofs are required.
3. **One runtime owner per side effect and durable store.** Host World/effect journal, HQ development wiki, TierFlow UI publisher, Studio signed-in Chrome sender, Rust graph store remain separate accountable owners, without duplicate services.
4. **User-facing truth and resource bounds are product features.** PWA-first source/revision/approval/receipt status; private data consent/revocation; physical storage reserve; offline source limitations.

## Related already-prepared ARBOR source branches (read-only inputs)

- `work/p5-preg-semantic-wiki-r001` → S-semantic exact HQ pure compiler/inert adapter, NOT promoted.
- `work/p5-preg-visual-language-r001` → V-visual synthetic multi-producer projection, NOT promoted.
- `work/p5-preg-autowiki-r001` → W-autowiki pure screen-state mirror, NOT promoted.
- `work/p5-preg-n0-io-r001` → N0-io proposed native event ABI, NOT promoted.
- `work/p5-p2-preg-{tests,knowledge,interop}-r001`, `work/p5-p2-highvalue-labs` → supporting Phase 2 findings, NOT promoted.
- `owenservera/repos-hq` research semantic wiki/Rock branches and `owenservera/vivim-labs` R005–R010 remain **evidence donors**; no runtime dependency on them after ARBOR source selection.

## What did NOT happen

No G integration, Cargo/Bun/Node/Windows/PWA live build or device acceptance was performed by this architecture round. No source features were promoted. No permissions/receipts/model or browser actions performed. A future worker must run actual test commands and show source/Host/device evidence before marking `VERIFIED`.

**Machine validation:** 43 source entries, 43 gaps, 14 waves, 100 tasks; 11/11 V8 checks of actual committed manifests/validator succeeded after one bug fix. Actual Node test file, Cargo and device commands **UNRUN** in this round.

**Next execution choice:** four dedicated **pre-G** worker prompts now exist at [prompts/README.md](./prompts/README.md). To actually begin the separate workers, run the four starter messages in isolated GitHub-connected conversations. PGZ is an independent referee after their committed handoffs. **No workers were launched or merged merely by authoring these prompts.** Post-G owner prompts still require the actual accepted original G HEAD; this branch remains a preparation candidate.
