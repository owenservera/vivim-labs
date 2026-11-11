# Native User I/O — Acceptance Gates & Evidence

Status: Proposed tests only. NONE of these gates is marked as passing by this document.

Every gate records exact source SHA, test command, OS/device, runtime logs, expected vs observed values, and an owner-approved GO / BLOCKED verdict. Source-created != built != wired != physically verified.

## Gate 1 — One semantic ingress
For each desktop text/voice/touch/GUI entry, CLI, and phone path, demonstrate that one authenticated Rust interaction entrypoint receives the event BEFORE any semantic planner, context selector, mission coordinator or model dispatcher. Thin JS delivery/adapters allowed; alternative semantic brains not allowed.

## Gate 2 — Input and output completeness
Golden matrix covers: draft; submit; command; free-form dialogue; selection; edit; correction; attachment ref; voice partial/final; gesture; keyboard shortcut; screen-reader command; approve/reject; cancel; auth expiration; reconnect; offline phone draft; CLI; scheduled input. For each, check native acknowledgement and correct output projection.

## Gate 3 — Interpretation parity
Run same inputs against existing Host control-language + planner and Rust candidate compiler. Confirm identical action IDs, risk classes, arguments, ambiguity, negation, refusals, personal rule pins and catalogue references. Any mismatch is logged and blocks retirement of the original path.

## Gate 4 — Authority isolation
No typing/preview creates a durable World effect. No AI, visual glyph, semantic token, local phone draft or Rust DAG can mint approvals. Only exact Host admission with current authenticated actor, plan digest, grants and revision may lead to effects.

## Gate 5 — Single effect once
Duplicate submitted event, repeated approval, concurrent phone/desktop commands, crash after claim, uncertain provider send, reconnect and stale retry do not produce duplicate effect. Receipt state distinguishes claimed/attempted/observed/verified/inconclusive.

## Gate 6 — Source and audience privacy
Sensitive personal data never enters HQ development index. Context bundles are source-backed, destination-specific, least disclosed, versioned, expiry-checked and revalidated at provider-send time. Preview data is not sent to an AI; browser send requires explicit existing Host authorization.

## Gate 7 — Revisioned streaming
Ordered session sequence; per-event revision; snapshot catch-up; cancellable pure work; stale result drop; throttled draft updates; backpressure; output audience filtering; idempotency tests. No stale source evidence is rendered as current.

## Gate 8 — Cross-surface parity
For the same supported semantic request, Windows UI, PWA, CLI and TierFlow receive compatible candidate identity, explanations, plan digest, approval state and authoritative receipts. Mobile display is not mistaken for mobile native authority. Physical phone test required.

## Gate 9 — Usability, latency and accessibility
Local caret/IME/selection operates without Rust round trip. Semantic hints progressively update, remain stable under slow retrieval, expose text alternatives, support keyboard navigation and reduced motion. Benchmark latency budgets after empirical device measurements rather than assert fictional timings.

## Gate 10 — Packaging and recoverability
Product installer and bundled Rust library build on target Windows environment. Stable host startup, version negotiation, offline PWA handling, watchdog/health, lifecycle close and restart; effect journal and World rehydrate; no unobserved replay. Test actual artifact, not just cargo unit tests.

## Required acceptance artifacts
- Verified entrypoint graph, SHA-pinned and classified.
- Golden event/projection fixtures, with old/new differential results.
- Privacy and authority threat-model negatives.
- Host approval and effect crash/uncertain-output fixture logs.
- Windows packaged build and actual paired phone test evidence.
- Explicit roll-forward/rollback and legacy-path retirement plan.

The first product milestone is a read-only native direct-input demonstration with source-backed contextual projections. Do not represent it as a completed full-stack cutover.
