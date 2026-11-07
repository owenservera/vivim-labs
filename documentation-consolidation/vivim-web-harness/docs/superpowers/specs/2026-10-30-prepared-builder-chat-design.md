# Prepared builder: Chat-first personal bootstrap

Status: owner-approved scope, including the global Personal AI clarification; implementation planning only.

## Shared generative UI requirement — 2026-10-31

The first user-facing **Bootstrap V1** must feed the same versioned, validated UI components/surface as the Build design session: [Generative UI PRD 11](../../../dev/prd/11-realtime-generative-ui-bootstrap-first.md) and [adoption contract](../../design/GENERATIVE-UI-BOOTSTRAP-V1.md). Keep this spec's fresh prepared Build conversation, protected `primary` identity, verified selected AI, early baseline trigger and single Studio owner. Do not replace the existing F02/F01/D03–D07 prompt schemas: their validated outputs enter the shared renderer via post-parse adapters. This note is a product gate, not execution permission.

## Intent

Continue the working dev version toward autobootstrap, rather than creating a parallel product. The user should see which AI services Vivim can actually use, choose a preferred builder, and find useful setup suggestions waiting in a fresh, prepared Build conversation.

The user is first building their **global Personal AI**: the persistent, maximally capable assistant for everything the harness can actually do with the owner's authorization. It is a protected harness primitive, not a removable app feature, a custom specialist agent, or a provider conversation.

The user designs their Vivim through conversation: its visual presentation, app structure, and how capabilities are exposed and selected. **Chat is the first working capability to shape**, not a permanently fixed interface or a reason to remove Talk. Build and Chat are interfaces to the same global Personal AI; specialized agents follow later.

References:

- `docs/USER-JOURNEY.md`: product-experience reference.
- `docs/roadmap.json`: execution authority.
- `docs/ROADMAP.md`: human-readable tracker.
- PR #35: additive Build scaffold; inspect before reusing.
- PR #36: Windows/Linux compatibility dependency; review separately, do not merge implicitly.

## Owner decisions

1. Availability must be compact and understandable, including whether an AI is really signed in.
2. Recommend the user's preferred AI because it is the service they already use; never claim access to personal history or memory that has not been established.
3. Start a fresh Build conversation, not an existing personal conversation.
4. The harness prepares and primes that AI in the background for the work to come.
5. Priming also requests initial setup suggestions before the user types in Build.
6. For the dev version, assume installation authorization has already been granted. No additional priming approval prompt. Configurable authorization rules are deferred.
7. Let the user shape visuals, app structure, feature exposure, and feature selection; begin with Chat.
8. Build remains separate from Talk. Existing pairing and operational chat must remain usable.
9. The global Personal AI has maximum authorized visibility and capability across the harness; later specialist agents do not replace it.
10. Access to it survives layout changes, mode changes, feature removal, configuration undo and provider-thread loss. An independent protected recovery entrance remains available if the custom interface breaks.
11. Begin baseline collection during onboarding as soon as phone pairing, the selected provider's verified sign-in, and installation authorization are established. Do not wait for the user to open Build or submit a bootstrap request.
12. Baseline collection means populating as much grounded personal context as the connected AI can actually supply into the owner's persistent global Personal AI workspace/identity structure. It is not just a Chat setup questionnaire, a transient prompt, or a few preferences. Personalized setup proposals are a downstream use of this foundation.

## Protected global Personal AI

Reuse the existing local `primary` Personal Agent and its versioned memory, Work and continuity foundations. Do not create another master identity or pretend the current data-directory fingerprint is a verified cross-device account identity. A paired phone reaches the same host-owned primary record.

The global identity and recovery entrance are not members of the user-removable capability list. A proposal, layout, reset, rollback, or future custom-agent configuration cannot delete, replace, hide, or narrow that protected global. Personal preferences and the presentation of its normal Chat entry remain customizable.

Inventory all registered harness capabilities with availability and authorization qualifications, rather than falsely promising that every discovered tool works. The global may coordinate supported authorized tools; credentials stay inside the harness. The first implementation delivers Chat/bootstrap operations, not a claim that every inventoried capability has been made executable.

Provide a stable harness-owned recovery route independent of custom app configuration and its JavaScript renderer, using the existing Studio/Chrome effect owner. If custom configuration is invalid, preserve it, explain the problem and offer a last-known-good configuration; do not wipe state or launch another controller.

"Always accessible" means layout/configuration changes cannot remove access. A powered-off host, missing installation, damaged storage, revoked access or unavailable provider still limits operation. Report those limits honestly; cached phone state is not proof that the host or AI is available.

Durable local identity, memory, preferences and work survive a provider-thread loss. A new provider thread is a replaceable binding with an explicit recovery record, not a new Personal AI or a reason to repeat an uncertain send. Switching provider is a deliberate user choice, never a silent fallback.

## First development slice

### Verified availability

Publish read-only, timestamped provider readiness: ready, sign-in needed, unavailable, or unknown/not checked. An open tab, installed driver, or cached model entry is not proof of readiness. The current relay manifest's unconditional `signedIn: true` is not sufficient.

Track supported provider/model/effort capabilities separately from sign-in readiness. The first slice requires one genuinely ready provider; unknown must never become ready by default.

### Preferred builder

Persist the owner's preferred provider and any explicitly selected model/effort. Recommend it when ready. If it becomes unavailable, show that condition and let the user select another ready provider; do not silently switch the Build conversation to another service.

### Durable background preparation

Create one owner-scoped bootstrap session, with a dedicated fresh provider conversation, a preparation revision, request identifiers, and durable state. Continue that prepared conversation when the user starts building. Reconnect and process restart must not duplicate initialization or resend a possibly delivered brief.

Trigger this session from onboarding readiness, not a Build-screen event: paired phone + selected AI + fresh verified sign-in + existing installation authorization. If several providers are available and none is selected, ask for the builder choice rather than contacting all of them. Pairing without a ready provider remains waiting; returning to the app does not restart baseline collection.

The initial bounded exchange introduces Vivim and asks the selected AI for the personal context it already has access to, organized into the owner's profile, preferences, communication/working style, roles and responsibilities, active projects, goals, routines, tools/environment and useful constraints. Cover these areas broadly; do not restrict intake to interface preferences. Missing facts stay explicitly unknown. A fresh conversation may have limited provider-managed memory: do not imply that it can enumerate all past conversations or export the provider's complete private memory.

Compile validated intake into the existing primary Personal Agent's versioned workspace: distinguish the Personal AI's own identity/role, its owner's profile, durable preferences, active project/goal context, working context and a source index. Each accepted field carries source/request references, collection time, and an assertion basis such as provider-reported or owner-confirmed. This is a persistent context foundation used by subsequent agents and app design, not merely text appended to the priming conversation. Keep the existing primary/store as authority; any human-readable identity-folder documents are projections of that authority, not a competing database.

Do not request credentials or treat model claims as account entitlements. Never infer missing personal facts or silently overwrite owner-confirmed context with provider-reported statements. Preserve disagreements and unknowns for later refinement. Provider capabilities and signed-in state come from host observations. Keep private context under the configured local data root, out of Git, public shares and status snapshots; the phone receives a bounded progress/coverage summary rather than the entire personal dossier.

The brief contains:

- Vivim's purpose and the Chat-first development scope.
- A versioned catalog of supported capabilities and configuration operations.
- Current local configuration and explicitly saved preferences relevant to this task.
- Verified provider capabilities and their observation time.
- Instructions to return schema-valid proposals, state missing prerequisites, and distinguish suggestions from activated changes.

Do not scrape unrelated conversations to infer personal tastes. A fresh provider conversation may benefit from provider-managed personalization, but Vivim cannot promise that it does.

### Initial proposals

Ask the chosen web AI for a small set of starting proposals while preparation runs. Validate capability IDs, prerequisites, and configuration operations against the local catalog before presenting them. Retain the actual provider and request provenance.

Workspace population comes before setup suggestions. A first dev exchange may return both structured context and 2–3 suggestions, but the host must validate, source and save the context foundation before using it as the basis for proposals. Do not optimize for a one-message demonstration by reducing onboarding to a narrow Chat questionnaire. Unknown baseline information must not block honestly labeled generic proposals, and the experience need not wait for every context section to be complete. Any later targeted intake follow-up needs a separate finite budget and request identity, not an unbounded background interview.

Suggestions alone must not mutate the personal app. A user's deliberate selection or activation is a product interaction, not an extra permission popup. The model cannot label its own proposed changes as already applied.

## Chat as the first configurable capability

The initial proposal/configuration contract must cover three dimensions together:

- **Presentation:** how the Chat capability is visually presented, within supported design parameters.
- **Structure:** where Chat belongs in the personal app and how the user reaches it.
- **Exposure and selection:** which supported Chat controls/capabilities are exposed, and how the user chooses among them.

Implementation must define and expose a finite supported vocabulary for these dimensions. Unsupported requests remain proposals with a clear limitation, not invented working features. The concrete visual system and control treatment are not chosen by this brief.

Keep capability identity and functionality separate from presentation/navigation configuration, so a visual change does not recreate conversations, change the selected provider silently, or discard history. Validate proposed changes and make them previewable and reversible before activation.

Initial decisions are constrained configuration changes, not arbitrary AI-generated HTML, JavaScript, dependency installation, or unrestricted filesystem edits. More powerful construction can follow when the necessary execution and recovery capabilities exist.

## Boundaries and invariants

- AI providers are used only through authorized web sessions and the existing harness; no model-provider APIs.
- Installation authorization covers bounded background priming in this dev flow. This explicitly amends B01's current blanket wording, "no automatic writes or sends," for recorded priming requests only; it does not authorize an unbounded background loop.
- Preserve exact-thread binding, draft protection, provider attribution, and no automatic resend after uncertain effects.
- Report preparing, ready, blocked, failed, and uncertain states honestly; never imply priming succeeded from a browser connection alone.
- Cancel/supersede stale preparation when its provider or brief revision changes. Do not turn cancellation into an automatic resend of an uncertain request.
- Keep secrets, cookies, pairing material, and raw private runtime data out of Git and proposal payloads.
- Existing Talk is not gated on bootstrap completion and is not reset by Build configuration changes.
- Full time travel, modes, arbitrary code installation, and full multi-provider coverage are not first-slice requirements.

## Proposed implementation order

1. Inspect existing readiness probes, relay contracts, Build scaffold, persistence, and Chat configuration seams.
2. Protect the global primary identity and recovery entrance, then define versioned contracts for readiness, preferred builder, bootstrap preparation, and Chat configuration proposals.
3. Replace optimistic readiness with conservative real observations.
4. Implement durable preferred-builder selection and fresh-conversation preparation through the journaled router.
5. Validate initial Chat proposals and connect them to the existing Build experience without replacing Talk.
6. Complete the smallest supported Chat preview/activate/undo path and test it from the paired phone.

This is a design sequence, not a claim that these capabilities already exist. The implementation plan must map reuse and missing pieces to B01/B02/B04/B05 and H01/H02/H03/H04/H05/H08, with no wholesale roadmap status changes from offline work.

## Evidence required

- Deterministic checks distinguish signed-out and unknown providers from verified readiness.
- Removing every optional capability, corrupting a custom layout or restoring an old app configuration cannot remove the global primary identity or protected recovery route.
- Losing a provider tab/thread changes only the provider binding; existing primary identity, memory and Work survive, and unavailable AI is not represented as a successful response.
- A fake router establishes priming idempotency, reconnect recovery, stale revision handling, and no resend after uncertain sends without calling providers.
- Configuration validation rejects unsupported capabilities and operations; changing Chat presentation preserves conversation identity.
- Local state round-trips preferred builder, preparation state, and the selected Chat configuration without secrets in exported receipts.
- Provider-reported baseline survives reconnect/restart in the existing primary workspace, populates the correct owner/profile/preference/project/goal sections, preserves confirmed facts, and exposes truthful coverage/unknowns without publishing the private content in relay status.
- A real paired phone shows verified availability, a preferred ready builder, initial suggestions from that provider, continued conversation in the prepared thread, and a supported Chat configuration change that survives reconnect and can be undone.
- Existing Talk still sends and receives normally. A passing fixture or prepared thread alone is not a live end-to-end pass.

## Builder decisions to resolve during implementation planning

- The smallest catalog of Chat configuration operations genuinely supported by current code.
- Exact existing storage/bridge seams to extend rather than duplicate.
- A finite priming budget and readiness freshness policy, recorded as implementation choices.
- Visual treatment and responsive behavior, without interpreting "user-designed" as unrestricted code execution.

Recording this approved design does not itself execute product code, send provider messages, activate configuration, deploy, or merge a dependency PR. The implementation plan must honor the current instruction to use bounded runtime checks rather than unit-suite sweeps, and distinguish those checks from real-phone acceptance.
