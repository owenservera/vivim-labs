# Gesture as dialogue — integration architecture R002

Prepared design, not an implemented ABI. Base ARBOR cf2e5c8. Product: one First Contact PWA, no second UI, engine, graph, journal, permission router or command compiler.

## Two linked loops
PREVIEW (inert): browser pointer / keyboard / optional voice clarification -> current source-backed Wiki target -> pure Rust interaction reducer + existing TierFlow trusted bindings -> supported VLANG/VEC candidate or explicit unsupported -> transient TierFlow/LiveSurfaceDocument preview -> revise, cancel, or explicitly request Apply.

COMMIT (Host-only): explicit Apply -> revision-fenced inert existing action admission -> Host control policy, grants, approvals, claim and effect -> real receipt -> newly observed Wiki/source view -> user-visible confirmation or denial.

Preview never grants permission. Pointerdown, movement, dwell and ordinary release never execute an effect.

## Interaction states
Idle -> Armed (contact/target) -> Revealing (hold) -> Exploring (movement, candidate or language refinement) -> AwaitingExplicitAction (release, preview remains) -> CommitRequested (separate Apply) -> HostPending -> Observed/Refused -> Idle.

Any active state -> Cancel / Stale -> Idle on pointercancel, scroll takeover, second pointer, target removal, focus/visibility loss, stale revision/digest, revoked grant or user Escape. Cancellation has no Host effect and clears transient preview.

## Typed bounded session sketch (NOT a new database)
- GestureStart: sessionNonce, pointerId, pointerType, actorRef, surfaceId, stable targetId, Wiki sourceId/revision/digest, viewRevision, monotonic start.
- GestureSample: session, strictly monotonic input sequence, phase (down/dwell/move/up/cancel), normalized local point, optional hardware-supported pressure. No raw screen image or secret text.
- GesturePreview: same session and source fences, candidateIds, explanation, visualDiff if supported, provisional status (revealing/exploring/stale/unsupported), effectInvoked=false, authorized=false.
- GestureDecision: separate user apply/cancel, exact target/source/view revision, candidateId. Host revalidates all authority and is allowed to refuse.
- ObservedOutcome: Host correlation, actual receipt or refusal, old/new source and view revisions, visual readback. No synthetic success.

## Necessary semantics
1. Stable target identity: admitted Wiki page/section, source digest, and revision; raw visible label/coordinates never sufficient.
2. Freshness: reject any late preview against a newer Wiki identity, session grant, or UI projection; LiveSystemState global publication sequence is not an atomic cross-source transaction.
3. Responsive UI: local inert visual response can happen immediately while async semantic enrichment has loading/stale/unknown status. Do not hold the pointer waiting for Host network.
4. Discoverability: begin with an anchored contextual strip or sheet; compare radial/inline variants only after one measured phone PWA success.
5. Voice/text interaction: may refine the same pending semantic intent after release; never demand users speak while holding. Agent-proposed composer edits have different actor and revision ownership.
6. Accessibility: tap menu, keyboard Enter/Escape, explicit Apply, non-drag alternatives, reduced motion, screen-reader labels and browser pan/zoom retention.
7. Privacy: transient move samples stay in memory; only consented bounded episode summary enters existing Activity Fabric. No default full-screen/audio capture or durable raw coordinates.
8. Pressure: time-based long press is always supported as a possible gesture; reported PointerEvent.pressure is not assumed to be calibrated hardware force.

## Exact future code seams (Dev-owned ARBOR)
A. Minimal TS7+ PointerEvent adapter near First Contact Wiki renderer, preserving existing toggle semantics and 180ms search.
B. Extend existing TierFlow wave4 normalizer for inert gesture phases, reconciling A039 capability taxonomy.
C. Rust-first deterministic guard/reducer adjacent to live_surface.rs / LiveSystemState; no new Host authorization.
D. Existing VEC supported proposals plus TierFlow render projection; FIRST CONTACT mounting gap must be closed rather than assuming Build inspector already applies.
E. Explicit Apply uses existing Host Control path, revision fencing and receipt. Readback reuses Wiki's admitted source revision/digest.
F. Consented activity episode through existing activity_fabric event.rs, without second journal.

All of the above are PLANNED and UNRUN. See SOURCE-ATLAS-R002, WORK-BREAKDOWN-R002 and ACCEPTANCE-R002.
