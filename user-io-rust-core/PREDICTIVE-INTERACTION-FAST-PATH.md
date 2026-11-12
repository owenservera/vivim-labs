# Predictive Native Interaction — User-Learned Fast Path

Date: 2026-11-12  
Status: DESIGN / ENGINEERING CONTRACT. Not implemented, not compiled, not a claim of runtime verification.

**Architecture decision:** Predictive interaction is an adaptive, reversible set of **native Rust reactor capabilities inside the shared parallel event fabric**. It does not create a central semantic router, a second learning service, a second command catalog, an AI execution authority, or a new graph store.

## 0. Goal and invariant

A user who habitually requests "Full update on [project] — next steps" can type **full**, accept two grounded suggestions with **two Tab presses**, and obtain a fully formed **interpreted** intent without manually typing the rest.

The preview can be **structurally validated** and **source/revision-grounded** in real time. It is NOT verified that the user intended the guess until the user selects it, and NOT proof that a future command will execute successfully. Exact authorization, freshness and effect validation happen again at submission and effect admission.

Fundamental distinction:
- **Prediction:** likely user meaning derived from prior accepted interaction patterns and current context; may be wrong.
- **Grounding:** project ID, command ID, current catalog revision, source evidence, valid requested slots, verified local permission scope (not an effect grant).
- **User confirmation:** each Tab commits an exact suggested atom/template or slot value in a revision-checked way.
- **Execution authorization:** **separate** Host action approval when required; predictions or Tab selections alone never constitute it.
- **Outcome verification:** evidence of completed work, only after actual observation and independent verification.

No remote API, provider prompt, or external effect is invoked merely because the user typed \`full\`.

## 1. "full" -> Tab -> Tab: exact state machine

Assume user is in the verified workspace/project context \`XYZ\`. A locally learned template is "Full update on {project} — {section}", with historical explicitly accepted section \`Next steps\`. Both candidates are valid *for that workspace* against current Host-catalog and source revisions.

| Step | Input / UI state | Rust parallel fabric | Consent/effect state |
| --- | --- | --- | --- |
| 0 | User types \`full\` | \`DraftChanged(revision=41)\` fanned out. Language recognizes prefix; prediction reactor ranks full-update template; entity reactor grounds \`XYZ\`; knowledge reactor identifies available sources; command reactor checks current action ID/readiness | Nothing committed or executed |
| 1 | UI immediately projects \`Full update · [XYZ ▾] · [Next steps ▾]\` in a compact, reversible preview. Labels distinguish **predicted** from **grounded** | Results from independent reactors join under input revision 41. Candidate ID + pins identify the actual template and values. Incomplete evidence renders neutral/unavailable, never "verified" | No intent confirmation |
| 2 | **Tab 1** selects the highest-ranked *exact grounded template with project \`XYZ\`*, unless ambiguous. Focus moves to the final output/section slot. | Native \`AcceptSuggestion\` event pins chosen template + project ID. CAS rejects stale suggestion or wrong session. The section remains predicted, not accepted. | User confirmed template+target, not final section or execution |
| 3 | **Tab 2** accepts \`Next steps\` as the section. | Native \`AcceptSlot\` pins exact section variant and source revision. Recompile and validate action shape, slot schema and real source refs. | **Fully specified / syntax+grounding validated** intent. No task run |
| 4 | **Enter/Send** submits a committed request (separate from draft preview). | \`Submit\` with idempotency ID, exact digest and current version pins; recompute/recheck authoritative context as required. Read-only action may run under defined Host R0 behavior; effectful actions go through Host approval and claim/receipt path | Executed only if currently allowed; do not infer outcome from prediction |

**Failure/ambiguity behavior:** If several projects are plausible, if active project is not trusted, if no verified command exists, or if "next steps" is unsupported, there is no forced two-Tab completion. The first Tab offers an accessible menu and may require extra explicit selection. Better one truthful extra keystroke than a confidently incorrect system.

**Alternate layout:** Where an active project is definitely grounded, Tab 1 could commit the command template, and Tab 2 accept a one-step \`project+section\` composite only if the composite is an exact, displayed, versioned user-selectable object. The two presses must each have inspectable atomic meanings.

## 2. The prediction model is not the routing architecture

All logical subscribers retain access to authorized input events. The **fast path only changes priority and precomputation** of safely permitted pure work.

- Language/command recognition observes the same draft even if prediction misses.
- The prediction reactor independently subscribes to relevant \`DraftChanged\`, context changes and correction/confirmation events.
- It emits **PredictionCandidate** events, not executable commands or trusted policies.
- Verified source-linked candidates stimulate graph queries, source selectors, command-slot preparation and UI projection **in parallel**.
- The native scheduler can allocate bounded compute budget to likely candidates, keep hot caches, and cancel stale speculative tasks.
- Reactive completion/projection joins results by exact source input revision, candidate ID, graph revision, command-catalog version, and policy pins.
- New output can trigger other appropriate subscribers; no top-level "if full then run these subsystems" coordinator.
- Prediction cannot suppress other subscribers, forge capability, bypass grant checks, or prioritize effects.
- Event fan-out is the declared subscription/DAG wiring. There is a mechanical ingress and bounded executor, not a central intent router.

**Predictive execution policy:** Speculate only on **pure, read-only, locally authorized** computation (tokenization, catalog lookup, graph query, semantic projection, cached context preparation, source availability). Never speculate on provider send, disk writes to authoritative stores, changes to policy, approvals, background notifications, scheduled work, or any external side effect.

## 3. Three horizons of anticipation

### Horizon A — Hot, under-keystroke time scale
In-memory normalized prefix index / candidate cache; curated command vocabulary and accepted personal aliases; per-session ranking; no model invocation or disk query necessary to show top available candidate. A visual hint can remain tentative while evidence finishes.

### Horizon B — Active context, incremental
When focus/project/workspace or selected provider changes, use the graph's dependency/invalidation machinery to precompute likely entity IDs, relevant source revision summaries, required argument schemas, display-safe context hints and permission availability. Do not prefetch inaccessible personal data.

### Horizon C — Habit and workflow shape
From **explicitly accepted selections, successful completions, user corrections, rejection signals and current project context**, learn:
- habitual command templates
- slot value ranking by project/session and time window
- common command-sequence transitions
- co-occurring requested sections (updates, issues, risks, next steps)
- visual-compression preferences (when to show one icon vs words)
- opt-in suggested reusable macros with explicit preview/approval; never silently invent a privileged macro.

The model proposes faster UI paths, not privileges or automatically scheduled actions. A repeatedly rejected suggestion is demoted. Sparse history falls back to current command syntax and context, not hallucinated memory.

## 4. Concrete model design — start deterministic and local

Version 0:
- Read the canonical Host action catalog and current personal semantics rather than copying commands into a new registry.
- Maintain a local, owner-private, bounded personalized **pattern statistics projection**: intent/template ID, slot key, opaque entity reference, workspace scope, explicit accept/correct/reject/outcome counts, recency decay, last verified catalog version. No raw prompt, contact, email body or private source quotation required.
- Use a prefix trie / finite-state template grammar and context-conditioned count scoring with source-validity gating. Score may rank candidates but must never become a permission bit.
- Feature inputs: prefix length/word boundary, active workspace, pinned focused project, prior *confirmed* template/slots, current schema and capability availability, temporal context (explicit, not inferred as fact), interaction modality.
- Re-rank on user feedback with monotonic revision control. An exposed suggestion is **not** accepted feedback; absence of a click is not necessarily rejection.
- Use deterministic bounded exploration only in the suggestion UI when users explicitly opt in, not in effect selection.

Later:
- A small on-device learned ranker if benchmarks justify it, with versioned opt-in training corpus and rollback.
- Sequence prediction and proactive pure-cache warming across typed graph dependencies.
- Rich multimodal prediction when voice, accessible UI and gestures share common interaction events.
- Calibration/evaluation per user/workspace; never claim numerical confidence as "truth" without calibration.

**Privacy:** Per-owner storage, never committed to public Labs/HQ, no sharing with other users, no remote training, clear controls to inspect/reset learned shortcuts. Historical content can provide patterns only under the owner's ingestion/retention permissions. An idle/bystander/unauthorized session cannot access personalized predictions.

## 5. Typed contracts (illustrative; not source-implemented)

\`\`\`rust
struct PredictionCandidate {
    source_event: EventRef,
    input_revision: u64,
    candidate_id: CandidateId,
    template: ActionTemplateRef,   // canonical catalog ref + version
    slot_candidates: Vec<SlotCandidate>,
    context_pins: ContextPins,     // workspace, project, graph, policy
    provenance: Vec<EvidenceRef>,  // why this guess exists
    expires_at: Expiry,
    state: PredictionState,        // Suggested | PartiallyGrounded | Grounded | Stale
}

struct AcceptSuggestion {
    session: SessionRef,
    expected_input_revision: u64,
    candidate_id: CandidateId,
    exact_selection: SelectionRef,
    idempotency_key: IdempotencyKey,
}

struct AcceptSlot {
    session: SessionRef,
    expected_interaction_revision: u64,
    slot_id: SlotId,
    exact_value_ref: ValueRef,
    idempotency_key: IdempotencyKey,
}
\`\`\`

Actor/authority comes from an authenticated transport; user-provided IDs/digests are never trusted authentication. Derived suggestions must be scoped to authenticated actor and allowed audience. Accepted selection changes must atomically match the latest reachable view revision.

### Output meaning
- Ghost text: **predicted** and easily reversible; never silently inserted as confirmed text.
- Solid grounded chip: verified *entity/source/schema reference* and current revision.
- Filled command: complete *syntax* and missing-slot check.
- Available marker: action supported by current catalog/build.
- Approval marker: effect requires current Host authorization.
- Verified receipt: **only** after authoritative observed postcondition; not a predictor output.
- Screen readers announce "suggestion, not selected", selectable slot/value and available shortcut.

## 6. Latency design (targets, NOT benchmarks)

- Local glyph/caret/IME paint: no Rust round trip, frame-budget responsive.
- Hot in-memory prefix prediction: target ~15–30 ms after native event ingestion on realistic Windows hardware, benchmark before release.
- First useful semantic projection: target ~50–100 ms after input stabilization without waiting for slow graph reads.
- Graph/context-derived signals: asynchronous; may take longer, never overwrite newer revision.
- Time-consuming retrieval: cancel/coalesce on changing prefix, project or focus; never block user text or Tab navigation.
- Tab acceptance: deterministic, revision checked. A stale suggestion becomes a clear clarification instead of a guessed selection.
- Phone PWA: account for network latency and offline mode; local ghost preview may be cached and marked *unverified* until fresh Host evidence returns. No unsupported claim of guaranteed sub-100ms roundtrip.

Use bounded work queues, per-subscriber budgets, p50/p95/p99 latency telemetry, drop/coalesce only speculative drafts, and maintain fairness so favored predictions do not starve other graph computations.

## 7. Differential learning and safe feedback

Events that can update personal ranker:
- Exact suggestion selected; template/slot committed
- User corrects a misgrounded project or section
- User explicitly dismisses a suggestion
- A submitted request succeeds, fails, or is inconclusive, attributed to correct authoritative evidence
- User resets/pauses prediction, revokes relevant source, or changes focus context

Events that must NOT teach by themselves:
- Mere keystroke, view impression, hover, AI's own assertion, incorrect source pointer, unobserved presumed success, silent private-context scraping

Model state must be revisioned with safeguards against poisoning from remote untrusted text and stale feedback. Prediction features are derived from owner-approved interaction metadata.

## 8. Acceptance tests (must be real product tests)

P01. Given focused, authenticated project XYZ and confirmed pattern, type \`full\`; obtain versioned full-update candidate + grounded XYZ + predicted Next steps from independent native reactors (no central semantic router).
P02. First Tab commits the exact command+target at the displayed revision; second Tab commits the exact section; native intent is fully specified. Enter separately submits.
P03. First Tab without a grounded project asks for selection rather than asserting XYZ.
P04. Typed \`full\` with unsupported Host catalog action cannot be labeled available, irrespective of learned frequency.
P05. Learned phrase \`full\` does not make an effect executable or grant provider disclosure.
P06. Four reactor results finish out of order across draft revisions; stale predictions and graph hints never replace current state.
P07. A source/project revision changes between preview and Tab; CAS refuses or offers a fresh candidate.
P08. Source permission revoked between prediction and submit/admit; submission/effect blocked.
P09. Corrections and explicit rejections demote incorrect rankings; raw private prompt bodies are never required in ranker records.
P10. Cold start/no history uses deterministic catalog fallback and does not fabricate a "known habit".
P11. Interrupted/reconnected PWA retains editable draft but downgrades offline speculative suggestions; no phantom verification or send.
P12. Screen reader, keyboard-only, reduced-motion and IME users can discover, inspect, accept and undo the same semantics.
P13. Predictive prewarming is confined to pure read-only work and cannot generate unrequested writes or provider sends.
P14. Model and hot cache are bounded; cancellation and fairness prevent long-running prediction from affecting ordinary typing.
P15. Predictions are scoped per authenticated owner/workspace and do not cross accounts or private graph namespaces.

Each test requires exact source SHA, fixture, observed output, Windows/phone environment, latency and pass/block verdict. No test is marked passed here.

## 9. Integration and ownership

This is a **subsystem within** \`user-io-rust-core/\`, not a new top-level project and not a standalone AI service.

Production: augment the shared Rust engine's existing graph-node, scheduler, typed DAG, invalidate/change-set, and product-native session machinery after a source-level dependency audit. The personalized model should be an owner-local projection anchored to existing canonical Host preferences/corrections and proven acceptance/outcome receipts, not an additional global knowledge database.

Composer/TierFlow: display predictions, argument chips, Tab focus behavior, source revision and uncertainty indicators. No independent ranker with different truths. Keep a tiny local speculative display cache for responsiveness with explicit stale status. Existing \`nlp-command-ui-lab\` is a visual experiment consumer.

Host: canonical catalog, risk, effect approval, grants, send-once journal, World revision, true evidence receipts.

HQ: read-only development evidence and structural source intelligence; **not** a destination for private user habits.

## 10. First implementation slice

One native Rust session emits a \`DraftChanged(full)\` event. Three or more independent read-only reactors run: prefix/template matching, project/source grounding, and predictive ranking. The projection combines them into a compact command+slot visual. Two native \`Accept*\` events bind the exact displayed candidate/slots. Revisions are checked, no effect occurs, and a subsequent \`Submit\` must pass the existing Host authority. Then compare with a no-history account, stale source, wrong project, and slow phone connection.

**Success metric:** a meaningful reduction in confirmed keystrokes and median time to fully specified user intent **without increased mis-selection, unauthorized effects, fabricated grounding, or confusing accessibility behavior**.
