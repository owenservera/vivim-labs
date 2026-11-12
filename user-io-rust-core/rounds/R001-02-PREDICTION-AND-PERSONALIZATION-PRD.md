# R001-02 — PRD: Predictive Fast Path, Two-Tab Grounding & Responsible Learning

Status: R001 pure native source slice + larger product contract in design. R006 UI catalog is design, not operational measurement.

## Goal
Familiar user intents should become fully **specified and source-grounded** with minimal deliberate selection. Example:
\`full\` -> \`Full update on [XYZ?] — [Next steps?]\` -> Tab chooses project -> Tab chooses focus -> fully specified inert frame.

Two tabs are a *conditional usability target*, not an unconditional promise. Untrusted project, conflicting aliases, staleness, negation, unsupported action, or ambiguous slot produces a choice/refusal.

## Prediction boundaries
1. **Candidate**: matched typed command pattern and rank.
2. **Grounded**: exact source-backed ID/revision for the project, command and selected focus.
3. **Confirmed**: an explicit user selection event pins each candidate to the current interaction revision.
4. **Operational availability**: checked by Host owner, not by the predictor.
5. **Authorized**: separately admitted by Host with current grants and approval.
6. **Verified effect**: only after Host receipt plus independent postcondition; never from autocomplete.

## Multi-horizon compute
- A0 immediate: prefix matching and local candidate generation, no provider API or network calls.
- A1 context: source-pinned current focus/project and relevant semantic graph neighbors; read-only.
- A2 prewarm: probability-weighted read-only cached HQ/Host source projections if the principal currently has access, with CPU/IO budget and cancellation.
- A3 habit learning: owner-local compact stats from explicitly accepted choices/corrections, scoped by workspace; only owner-approved persistent storage in later rounds.
- A4 future: optional on-device ranker if calibrated evaluation shows benefit, never required to parse short commands.

## Versioned data model
\`\`\`
PatternChoice {
 principalRef; workspaceRef; templateId; templateVersion;
 slotName; opaqueEntityRef?; chosenValueRef;
 explicitAcceptCount; explicitRejectCount; correctionCount;
 lastAcceptedAt; lastSourceRevision; policyRevision;
 decayPolicy; learningConsentVersion;
}
\`\`\`
No raw email bodies, conversation dumps, addresses, passwords or untrusted injected page text in ranking state. The actual R001 Rust slice only accepts a trusted \`AcceptedPattern { template_id, project_id, focus, selections }\` snapshot and temporarily increments a session-local count on the second Tab. It does **not** store data durably and is not a learning-model deployment.

## Deterministic ranking v0
- Filter to exact current canonical template/source scope.
- Prefix match and intended slot type compatibility are hard filters.
- Grounded workspace/project identity is mandatory before auto-project Tab.
- Rank focus values with explicit accepted counts; stable declared order is a deterministic tie break.
- Corrections/undo should demote patterns in later rounds; do not count suggestion exposures as acceptance.
- Retire/reset on catalog version change or source revocation.
- No statistical rank or "confidence" implies authorization; scores are not calibrated probabilities.

## Two Tab semantics
\`\`\`
DraftChanged("full", seq=1)
    -> candidate template / grounded project XYZ / predicted focus next_steps
Tab(seq=2, expected_revision=1)
    -> selects displayed project XYZ and pins matched template
Tab(seq=3, expected_revision=2)
    -> selects displayed focus next_steps
Submit(seq=4, expected_revision=3)
    -> R001 inert marker; future Host-authorized read/effect path
\`\`\`
The displayed exact revision, IDs and source provenance must match what Tab commits. Tab should not steal focus during IME, while the accessibility framework is traversing other controls, or when a menu is absent. UI manages focus mechanics; Rust processes a deliberate Tab acceptance event.

## Real-time verification language
- "Recognized" = deterministic parser matching a registered grammar.
- "Grounded reference" = owner-sourced candidate identity with pinned revision, but may still be stale.
- "Current/available" = explicit present-time source owner preflight, not merely \`readiness:"verified"\` in a source catalog.
- "Fully specified" = complete typed frame, not yet run.
- "Verified done" = independently supported effect receipt, impossible during draft prediction.

## Prewarming constraints
Speculate on existing pure engine operations only: cache prefix, inspect catalog snapshot, derive candidate graph neighbors, compute read-only issue/status summary, estimate context size. Protect user privacy and skip work without current grants. Do not send browser messages, write World, schedule notifications, make remote model calls, approve grants or pre-commit commands.

## Risks and mitigations
- "Habit lock-in": rerank from corrections, display alternatives, owner reset.
- Wrong project: explicit grounded identity, menu on ambiguity, project revision pins.
- Command collision: preserve alternate frames with source evidence; never pick a privileged action by rank.
- Model poisoning: only accept trusted explicit user-selection outcome metadata, not text in imported documents.
- Cost: bounded speculative budget, cancellation and fair scheduling, low-power mode.
- Privacy: per-owner and per-workspace isolation, session-only by default, no HQ developer DB upload.

## Acceptance corpus
- Exact frequent \`full\` behavior and two distinct Tab commits.
- No-history cold start offers registered templates but no invented "personal habit".
- Multiple projects, revoked sources, stale aliases and unsupported actions cannot be auto-grounded.
- Edited/negated/quoted \`full\` does not silently trigger an action.
- Explicit correction changes future focus ranking; unseen suggestions are not feedback.
- Authoritative status, permission and execution outcome are distinct from prediction.
- Unverified lab contacts and source inventory IDs do not become executable Host action IDs.
- Input/out-of-order/reactor delays cannot return a stale candidate as current.
- Private data no cross-account/cross-workspace disclosure; accessibility/IME two-Tab parity.

R001 initial code is only a subset. Future goal from UI R006: eligible familiar two-slot cases complete in <=2 Tabs at an experimentally measured correct selection rate. No measurable target is declared achieved.
