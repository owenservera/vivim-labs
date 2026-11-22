# Gestures — VIVIM bidirectional command space

**R003 · 2026-11-22 · shared GPUI/PWA core precode staged, not product-integrated**

Principle: A gesture is a live negotiation, not a one-shot trigger. Hold, movement, focus, language and real UI feedback shape a changing proposed command. The person can inspect, revise, release, cancel or explicitly Apply; only existing Host authority can commit an effect.

## R003 first: canonical GPUI + mobile PWA

[Canonical shared interaction contract](R003-CANONICAL-CONTRACT.md) · [Pure Rust precode](src/gesture_interaction.rs) · [Dev integration](R003-DEV-INTEGRATION.md) · [Cross-viewport gates](R003-ACCEPTANCE.md) · [Truthful evidence](R003-EVIDENCE.md).

**One semantic gesture reducer, two native input adapters.** Windows GPUI is the master native authoring/rendering viewport; mobile PWA renders the same semantic surface. Existing Host remains the only authority for commands/effects. The R002 PWA-first text below is historical and must be interpreted through ARBOR's 2026-11-22 ONE-CANONICAL-LIVE-SURFACE rule.

## R002 background entry (read selectively)
1. [SITREP.md](SITREP.md): observed source pin, status and next safe step.
2. [SOURCE-ATLAS-R002.md](SOURCE-ATLAS-R002.md): exact ARBOR source reuse, verified gaps, collision zones.
3. [ARCHITECTURE-R002.md](ARCHITECTURE-R002.md): preview/commit separation, reducer states and invariant fences.
4. [WORK-BREAKDOWN-R002.md](WORK-BREAKDOWN-R002.md): 12 bounded PWA-facing subtasks, dependencies and proposed owners.
5. [manifest.v1.json](manifest.v1.json): machine-readable projection **not** a new authoritative task queue.
6. [ACCEPTANCE-R002.md](ACCEPTANCE-R002.md): G0–G9 visible acceptance and adversarial tests.
7. [DEVAGENT-HANDOFF-R002.md](DEVAGENT-HANDOFF-R002.md): exact code seams and release gates for ChatGPT Dev.
8. [TRACKER-CROSSWALK-R002.md](TRACKER-CROSSWALK-R002.md): local OW-018, proposed PV-15, A039 and existing claims.
9. [EVIDENCE-R002.md](EVIDENCE-R002.md): observed source lines, external references, UNRUN limits.

Original conceptual notes remain in [PRINCIPLES.md](PRINCIPLES.md) and [RESEARCH.md](RESEARCH.md). This folder extends the existing DESIGN/DevOps Planning system. It is not a second Wiki, gesture engine, renderer, command parser, journal, permission router or customer interface.

## The first PWA-visible deliverable
On the unchanged First Contact URL: type -> real Wiki card -> hold, see live contextual options -> move to revise preview -> cancel without effect. **Read-only first.** Explicit Host-approved Apply and Wiki readback are a separately gated extension, never an automatic effect from pointer movement or release.

## Architecture ownership
- Rust-first: LiveSystemState/LiveSurfaceDocument, gesture reducer, revision guards, deterministic candidates.
- Minimal TypeScript 7+ only for DOM PointerEvent, accessible controls and browser lifecycle.
- Reuse TierFlow/VEC visual projection, A039 work order identity, actual Wiki source and Activity Fabric.
- Host alone owns authorization/effects/receipts. ChatGPT Dev codes ARBOR via cloud GitHub main; Planning may prepare Labs files only.
- Local owner registry is not fully published to GitHub; remote PWA review tracker + TASK-CLAIMS are separate sources, not a new merged task database.

**Status:** R003 Rust PRECODE STAGED (17 authored tests, compiler UNRUN); R002 docs retained. ARBOR integration, real GPUI/PWA adapter wiring, host effects, device parity, Dev ACK and deployment remain UNRUN.
