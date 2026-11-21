# VEC deterministic intent compiler and program pipeline

## Draft pipeline
```text
User's free text / voice transcript
    -> lexical normalization (locale, tense, pronouns from explicit context only)
    -> exact phrase / registered aliases / finite grammar and slot recognition
    -> typed intent frame + confidence/ambiguity class
    -> capability registry and current grant/version eligibility
    -> composition planner (typed trigger/condition/data/action/UI edges)
    -> canonical VEC Program IR (data only; source revision, schema & capability versions)
    -> preview + scope approvals in existing Host
    -> versioned personal program in existing Host World/journal
    -> existing timer/event/action owners and precompiled Rust SDK capsules
    -> receipts -> observable TierFlow UI + pause/edit/undo
```

## Important correction to the 'compile Rust comments' idea
Rust capsules are pre-reviewed reusable **implementation units**. Many standard user-programming requests should compile to **data-only program graphs** and bind existing precompiled SDK functions, instead of invoking rustc for every utterance. A developer can author a vetted capsule in comment-bundled Rust, but its extraction/build/registration is a separate controlled developer pipeline. Runtime user programs reference only signed/registered capsule IDs and versioned argument types. No arbitrary Rust from user speech is executed.

## Deterministic parser stages
- N0 utterance normalization: per-locale tokenizer, synonym tables, bounded grammar, intent fingerprints. Never substitute unknown entities or capabilities.
- N1 frame extraction: actor (current authenticated user), verb/intent, event/condition, subject, target, location/source, frequency, thresholds, constraints and output. Preserve original text and unresolved slots; do not invent values.
- N2 typed resolver: stable capability IDs + version constraints, schema compatibility, source permissions and availability status (installed/enabled/unsupported/denied/revoked).
- N3 graph compiler: compute safe order and typed wiring for explicit event/query/filter/transform/proposal/action/view edges; reject cycles except approved bounded feedback and reject unbounded polling.
- N4 verification: schema/type/ACL/owner/safety budgets, reversible action plan, resource bounds, replay/dry-run if feasible.
- N5 install: Host-owned revisioned program with explicit actor/session/approval receipts. Execution rechecks live grant and scope, not only install-time authorization.
- N6 update/undo: natural language 'change that to Friday' resolves revision identity with deterministic conversation context or asks which program; versioned diff and rollback.

## Ambiguity, unsupported tasks and refusal
Resolve by explicit slot-filling, not invented 'confidence'. If two interpretations materially differ, ask one contextual question. If required SDK capability is absent, label `CAPABILITY_UNAVAILABLE` and offer safe creation/request; do not fake implementation. If prohibited, deny. If model escalation is authorized, require it to return *proposals* conforming to the same grammar/ABI with unresolved items explicitly marked.

## Versioned IR states
DRAFT -> PREVIEWED -> AWAITING_PERMISSION -> INSTALLED_DISABLED / ENABLED -> PAUSED -> SUPERSEDED / REVOKED / RETIRED. Failed execution yields RECEIPT_FAILURE or RECOVERABLE/BLOCKED, not successful state. A program may have some valid edges and others unsupported; no partial activation of risky actions without user approval.

## Interaction with existing ARBOR ownership
Use existing Host command/intent, journal, grant, action catalog and owner-controlled scheduler/clock mechanisms. Use TierFlow to render views. Use Rust HQKG or precompiled VEC units only for deterministic computation; personal World data and code-development HQ data remain separated, with explicit authorized cross-domain read when applicable. Studio remains the signed-in browser send owner, never an embedded VEC side channel.
