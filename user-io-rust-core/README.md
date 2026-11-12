# VIVIM — Native User I/O ↔ Rust Core

Status: Architecture / integration workstream. Created 2026-11-12. NOT a production deployment, merged cutover, or evidence of device acceptance.

## Decision — revised 2026-11-12: no semantic router

**One authenticated ingress is not one decision-making router.** Incoming immutable draft and committed events are published once into a native **parallel event fabric**. Independent graph, language, command, personal-context, privacy and projection reactors observe matching input/derived revisions concurrently. Deterministic subscriptions and typed graph dependencies perform the effective routing. No central semantic classifier chooses the one handler. Effects remain exclusively Host-authorized. See [Parallel Event Fabric Architecture](PARALLEL-EVENT-FABRIC-ARCHITECTURE.md), which controls over any earlier sequential interpretation below.

This is an independent Labs folder, a **sibling** of the existing visual-command composer lab, not a new repository, Rust graph implementation, service, authority, or product identity.

**Product invariant:** Every committed human interaction reaches a single Rust-owned application interaction contract before interpretation, mission coordination, or effect planning. Every user-facing output is derived from the same revisioned interaction and domain-owned authoritative state. Unavoidable platform transport and local view rendering are not considered competing orchestration hops.

The same invariant applies to desktop, PWA, future native Android, CLI, scheduled actions, assistive input, voice input, and visual controls.

### Scope: FULL user-facing I/O
INPUT: text, draft edits, keyboard, touch, gestures, selections, multi-step commands, voice transcripts and voice finalization, file/clipboard references, accessibility commands, device-level actions, approvals, rejection, cancellation, correction, personalization, offline draft reconciliation, automation-generated actions.

OUTPUT: text and streaming responses; semantic tokens; inline contextual menus; entity resolution and ambiguity prompts; source-backed context/wiki suggestions; visual state patches; status/mission cards; review/approval surfaces; durable receipts; errors and uncertainty; accessible semantic labels; speech/haptics/notifications through OS adapters; offline/stale indicators.

Out of scope: moving every paint, caret motion, animation, microphone decoder, keyboard driver, browser API, or network transport into Rust. UI remains responsive locally; the Rust core computes semantic meaning, state and effect candidates. Unsubmitted drafts never perform an effect and must not leak to provider AIs.

## Predictive interaction capability

Personalized **anticipatory computation** is a reactor and graph-driven scheduling capability within this native I/O workstream—not a central semantic router or a new service. Accepted patterns, source-grounded entity references, and current context allow the composer to offer reversible multi-slot completions such as `full → Tab → Tab`. Prediction can prewarm pure computations but never provider sends, approvals, authoritative writes or any other effects. See [Predictive Interaction Fast Path](PREDICTIVE-INTERACTION-FAST-PATH.md) and [Parallel Event Fabric](PARALLEL-EVENT-FABRIC-ARCHITECTURE.md).

## NODE 0 PRIME — Architectural reset (2026-11-12)

**The strategic architecture now takes priority over the initial R001 native experiment.** [Read the complete Node 0 Prime package](node-0-prime/README.md): 30 failure modes, a language/runtime architecture decision set, security findings, exact boundaries and seven gates. R001 draft [Harness PR #55](https://github.com/owenservera/vivim-web-harness/pull/55) remains a source-only experimental spike; it must not be merged as the production user I/O runtime until native security, concurrency, actor/source binding, semantic parity, compatibility and real Windows/PWA tests pass.

## Round 1 PRDs and actual native code (2026-11-12)

The [R001 PRD suite](rounds/README.md) is now published in this workstream, covering the native event fabric, predictive completion, Rust↔UI contract, acceptance and the verified Host-catalog gap for `full update`. The first source implementation is on [Harness branch `work/native-user-io-r001-20261112`](https://github.com/owenservera/vivim-web-harness/tree/work/native-user-io-r001-20261112), extending the existing product `core/programmability` Rust crate with parallel read-only reactors, Tab CAS, an inert Tauri command, synthetic example and unit tests. This is **source-implemented but not compiled, source-grounded or E2E-accepted**. UI remains the higher-level presentation owner.

## Locations and ownership
- **Labs:** this folder owns research, integration specifications, evidence, mapping matrices and test plans. It owns NO product runtime and no canonical registry.
- **Harness native core:** production implementation lives in the existing Rust foundation in owenservera/vivim-web-harness. Extend/converge its existing core, rather than create a duplicate sidecar kernel.
- **Shared graph computation:** reuse the existing Rust graph-node and typed DAG research in owenservera/repos-hq after deliberate packaging, integration, and namespace/provenance verification. Do not copy it wholesale or treat HQ development data as user data.
- **Host/Studio:** preserves exact authority over browser sends, grants, approvals, durable effect attempts, World writes, receipts and recovery until equivalent Rust-owned handling is security-verified and cut over. The Rust interaction core can own the user-facing control plane without prematurely owning all effects.
- **TierFlow:** projection and interaction rendering; command appearance never constitutes executable authority.
- **Visual lab:** owenservera/vivim-labs, branch lab/nlp-visual-command-ui-20261111, folder nlp-command-ui-lab. Its 100 composer innovations are a dependent *view experiment*; not the owner of interaction ingress or policies.

## Principle of one hop
- Windows: UI -> Tauri IPC -> native Rust interaction entrypoint. No JavaScript planning hop.
- Paired phone PWA: UI -> authenticated encrypted transport -> same resident Rust Host interaction entrypoint. No separate phone command brain; offline local drafts only.
- CLI / automation: native in-process or authenticated local IPC -> same interaction contract.
- Future APK: one selected native binding for pure portable computation, but PC Host remains authority for PC World/effects.
- External provider browser: already authorized native request -> existing Host/Studio browser adapter -> observed reply -> native state projection. Do not use provider APIs; do not automatically resend uncertain attempts.

## High-priority integration questions
1. Where are all currently live user ingress entrypoints and duplicate interpreters?
2. Which Rust graph engine functions are sufficiently verified for packaging into the product?
3. How do we pass authenticated actors, precise session/revision fences, data-class/audience and effect provenance across the boundary?
4. What can be interpreted on draft updates without side effects or permanent writes?
5. How does one native event result in exactly one consistent projection stream for desktop and PWA?
6. What moves to Rust and what remains a faithful device, browser or presentation adapter?
7. What are the rollback and differential-parity rules for existing JavaScript paths?

## Required deliverables
- USER-IO-CONTRACT-AND-MIGRATION.md — first-hop contract, full modality coverage, authority map, migration path.
- ACCEPTANCE-GATES.md — release gates and negative-path tests.
- Source-backed entrypoint map with file + commit SHA + runtime/proof classification.
- Differential corpus of existing JS versus native command interpretation and context selection.
- Cross-surface demonstrator using the real Host authority, not mocked write success.
- Evidence ledger distinguishing DESIGN / SOURCE_IMPLEMENTED / BUILD_PASSED / END_TO_END_ACCEPTED.

## Explicit non-goals
No independent chat runtime; no new graph database or journal; no second command catalogue; no new approval authority; no model/AI as execution authority; no default upload of private data to HQ; no multi-platform false claim; no duplicated mobile app; no speculative auto-resend after unknown effects.
