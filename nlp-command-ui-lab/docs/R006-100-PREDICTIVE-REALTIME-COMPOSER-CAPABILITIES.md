# R006 — 100 Predictive, Two-Tab, Real-Time Composer Capabilities

**Scope:** the VIVIM chat composer, its semantic model, its local contextual menus and the already-existing Rust/HQ/Host read ports. No new router, execution owner, graph store, data authority or full-screen UI.

**Status:** 100 design/architecture proposals, **not implemented or verified**. Prior Labs R005 semantic-lens and source-backed slash suggestions are implementation foundations, not evidence of this prediction engine.

## Reference interaction — one word, two Tabs

1. **Type** `full`. Lightweight local and existing Rust computations concurrently propose: `Full update [Project?] · [Focus?]`.
2. **Tab 1** commits a **grounded** project, or invokes a choice when multiple projects are plausible.
3. **Tab 2** commits a preferred focus like `next steps`, `issues` or `decisions`. Result: a typed, *nonexecuted* command-frame with source anchors and remaining uncertainty.
4. Existing read-only HQ indexes may prewarm evidence as the intent becomes likely; a separate Host capability check can report permissions when applicable. **Never equate intent parsing, data freshness or permission with one another.**
5. The user may inspect/edit/cancel; execution still goes through existing Host approval and receipts. Two Tabs is an efficiency *target* for familiar, unambiguous commands, not a universal guarantee.

## ERA 1 — Compress intent from sparse keystrokes (1–34)

1. **One-word mission expansion.** `full` predicts a verified-read project update with slots `[project] [focus]`; it remains a ghost until selected.
2. **Two-Tab mission closure.** Tab 1 chooses the project, Tab 2 chooses `next steps / issues / decisions`; completion yields a typed, inspectable intent.
3. **Last-project inversion.** `full` while working on TierFlow offers TierFlow first, but exposes the scope so a guess cannot silently become fact.
4. **Verb-fragment runway.** `ful` previews the complete `full update` grammar before the user finishes spelling the verb.
5. **Adaptive slot order.** If the project is already explicitly selected, Tab 1 goes to focus and Tab 2 to a time window rather than asking for the same project again.
6. **One-key batch accept.** A deliberate shortcut commits all low-risk, already-grounded default slots while leaving uncertain ones open.
7. **Prefix-to-parameter transformation.** `issues` proposes `[project] [open/closed] [severity]` instead of merely completing the word.
8. **Editable ghost tail.** `full` displays `update [TierFlow?] · [next steps?]` as low-ink predicted text that disappears cleanly on rejection.
9. **Clause-aware continuation.** `full TierFlow and` predicts a second compatible clause instead of repeating the project selector.
10. **Next-slot predictive type.** After `[project]` is grounded, only compatible `[focus]` choices appear; unrelated tools cannot enter the menu.
11. **Predictive conjunction parser.** `full ... plus` opens an additive step selector and preserves separate action boundaries.
12. **Predictive negation brake.** `full ... but don't` freezes any positive-action inference until the negated scope is known.
13. **Temporal shorthand expansion.** `since` attaches `[last check-in?] [last commit?] [yesterday?]` as mutually exclusive source-defined anchors.
14. **Question-to-operation disambiguation.** `what changed` offers compare revisions versus summarize discussion, not one false universal command.
15. **Pronoun-safe continuation.** `what about that one` offers candidate referents from the active composer context with an explicit ambiguity marker.
16. **Ellipsis recovery.** `full ... steps` retains skipped language as unresolved constraints rather than fabricating a complete sentence.
17. **Project alias grounding.** `TF` resolves to TierFlow only when the chosen workspace or user-approved alias supports it.
18. **Symbolic workflow shorthand.** `f → tf → !` expands to full update / TierFlow / issues only if the symbols were taught and confirmed.
19. **Inline scoped entity choice.** `full V` opens just the registered V-prefixed projects and shows source identities on focus.
20. **Reversible natural-language projection.** Clicking the predicted `[issues]` chip rewrites the English preview, not the other way around alone.
21. **Shape-preserving semantic edit.** Replacing `issues` with `next steps` retains the same project and time window without a full reparse reset.
22. **Multi-project predictive comparator.** `full TierFlow vs Host` proposes a side-by-side comparison command rather than two independent summaries.
23. **Output-format completion.** `full brief` offers concise, executive or code-linked summaries as output settings, not new semantic actions.
24. **Granularity slider in one token.** `full deep` switches depth from quick status to source-linked technical audit without multiplying menus.
25. **Threshold-aware quick commands.** `only blockers` automatically adds an explicit filtering clause, never silently discarding other issues.
26. **Follow-up action runway.** After showing an update preview, the composer predicts `next steps`, `fix issues` or `export` as selectable continuations.
27. **Cursor-local task switch.** Editing `[project]` changes project suggestions but leaves the user's explicitly selected `[focus]` untouched.
28. **Semantic backspace.** Backspace at a selected chip removes one grounded choice rather than corrupting underlying text.
29. **Command family breadcrumb.** A three-glyph rail `status → project → focus` indicates structure without adding a second toolbar.
30. **Inline command split.** Typing `and send` reveals that retrieval and communication are separate steps with different authority requirements.
31. **Typo-resistant intent stem.** `ful updat` finds the intended grammar through bounded fuzzy matching and labels it as a candidate.
32. **Meaning-delta flash.** If one keystroke changes `report` into `repair`, only the altered risk-bearing command segment flashes.
33. **Noisy-input stabilization.** During rapid typing or IME composition, maintain the last stable interpretation and do not bounce menus.
34. **Minimal confirmation sentence.** At readiness, compress the interpretation to `Update TierFlow · next steps · sources current` with a one-tap longform expansion.

## ERA 2 — Learn responsibly from actual user choices (35–67)

35. **Personal habit template mining.** Repeated user-confirmed `full [project] + next steps` sequences become a ranked, editable mission template.
36. **Context-conditioned ranking.** Candidate order depends on explicit workspace, time and recent selections—not just a global frequency count.
37. **Prediction by task phase.** During release verification, `full` prefers blockers and acceptance; during design, it prefers decisions and open questions.
38. **Event-conditioned anticipation.** After a real new commit is observed, `update` can offer `since last commit` rather than a generic date range.
39. **Correction-driven learning.** Changing a predicted `issues` to `next steps` records a bounded local preference correction.
40. **Undo as negative evidence.** Immediately undoing a suggestion reduces its future rank instead of counting the initial selection as success.
41. **Explicit alias teaching.** `When I type FU here, mean full update` stores an owner-approved alias with project scope and version.
42. **No cross-project contamination.** A frequent Host workflow does not automatically become TierFlow's default just because its wording matches.
43. **Personal semantic compression ladder.** New users see `Full project update`; experienced users may see a familiar `◈FU` with expandable labeling.
44. **Choice-frequency half-life.** Old habits gradually lose priority when recent explicit selections demonstrate a new working pattern.
45. **Session-mode sensitivity.** A focused debugging session ranks files and blockers; a planning session ranks tasks and decisions.
46. **Stable preferred slot order.** Learn whether the user usually chooses project→focus or focus→project, preserving keyboard muscle memory.
47. **Prediction confidence calibration.** Measure how often top suggestions are accepted and reduce visual certainty when hit rates fall.
48. **Privacy-scoped learning switch.** Allow learning from command selections while excluding raw message bodies and sensitive entity values.
49. **Temporary preference sandbox.** Offer `use this pattern for this session only` instead of forcing durable personalization.
50. **User-owned predictor reset.** One action clears learned UI habits without deleting operational logs or source evidence.
51. **Task-interruption resumption.** After returning to the composer, offer the last unfinished, non-executed mission skeleton—not an automatic rerun.
52. **Device-aware menu arrangement.** On phone, likely actions appear near the thumb; desktop preserves text+Tab workflow with identical semantics.
53. **Cross-device preference synchronization.** Sync approved ranking preferences with bounded revision rules while leaving private content in Host authority.
54. **Cold-start population priors.** New users receive opt-in, generic task templates; the UI clearly labels these as examples, not inferred habits.
55. **Rare-command promotion.** A seldom-used but highly context-relevant operation can outrank habitual choices when its source evidence is strong.
56. **Negative preference expressions.** `never suggest export on this project` changes local suggestion ranking while leaving command registration unchanged.
57. **Semantic shortcut collision alerts.** If `FU` means two different things across workspaces, expose that collision before selecting an alias.
58. **Adaptation explanation.** `Why this first?` reveals `selected 7 of last 9 times in this workspace`, not an opaque confidence boast.
59. **Outcome-informed preference.** A suggested workflow is preferred only after its explicitly observed, permitted result proves useful—not merely because it was clicked.
60. **Longitudinal intent clusters.** Cluster recurring `status + issues + next steps` requests into one user-named template, without auto-merging commands.
61. **Personal omission recovery.** If the user repeatedly adds `include risks`, the next update preview may offer that clause as an editable default.
62. **Reordering from explicit drag.** Dragging `issues` ahead of `next steps` updates the preferred focus sequence without changing intent semantics.
63. **Energy- and attention-aware compactness.** Offer quiet-mode summaries during rapid typing and richer feedback when the user focuses a token for inspection.
64. **Per-project vocabulary evolution.** A project-specific term like `Gate 5` becomes a grounded suggestion only after it appears in indexed evidence.
65. **User-selected source priorities.** If the user prefers verified code over design docs for engineering status, retrieval ranking reflects that preference transparently.
66. **Prediction drift alarm.** When learned shortcuts no longer match current command schemas, mark them stale and require re-grounding.
67. **Human-in-the-loop grammar graduation.** Promote an observed habit into an approved personal shorthand only after repeated clean matches and explicit confirmation.

## ERA 3 — Unlock anticipatory real-time capability (68–100)

68. **Parallel Rust-core hypothesis fanout.** Typing `full` simultaneously triggers cheap grammar, project-context, capability and knowledge candidate calculations; no new central router.
69. **Read-only speculative source prewarm.** As project probability rises, warm permitted HQ index pages before Tab selection, without fetching unauthorized personal data.
70. **Per-keystroke prefix cache.** Keep a revision-keyed prefix trie of command grammars so common starts respond locally before database work.
71. **Graph-neighbor anticipation.** A grounded project can prefetch adjacent tasks, issues, decisions and dependency summaries through existing Rust graph read ports.
72. **Incremental wiki evidence binding.** New words refine the same evidence query rather than replacing a whole page and causing layout jumps.
73. **Diff-sensitive status preparation.** If `since yesterday` appears, begin a bounded commit delta query before the full request is accepted.
74. **Live issue-set preparation.** A likely `blockers` focus precomputes a read-only issue projection, clearly stale if the ledger updates.
75. **Decision-timeline preparation.** A likely `decisions` focus prepares current versus superseded decisions and maintains both provenance and timestamps.
76. **Dependency-aware next-step ranking.** A potential `next steps` focus requests topologically ready tasks rather than a flat to-do list.
77. **Multi-source speculative race.** Issue ledger, wiki and source graph can respond independently; the composer shows partial grounded results while stragglers remain pending.
78. **Snapshot watermark binding.** Every predicted result displays the exact index/graph/world revision used, so readiness means a specific source state.
79. **Two-phase proof display.** Separate `intent understood` and `data freshly checked`; neither is confused with permission to execute.
80. **Permission-aware preflight.** When a route may require action, ask the existing Host read-only availability port early; never pre-authorize or perform the action.
81. **Speculative DAG assembly.** Build an inert dependency graph for likely multi-step requests, without submitting any Host/TierFlow commands.
82. **Selective cancellation.** When the user changes `issues` to `next steps`, cancel only the irrelevant issue query and reuse valid shared project context.
83. **Out-of-order result rejection.** A slow result from text revision 41 cannot overwrite the visible intent for revision 46.
84. **Latency tier scheduler.** Give local tokens a ~16-ms interaction budget; defer source lookups and optional model synthesis to progressively later tiers.
85. **Interest threshold gating.** Avoid expensive graph or source retrieval until a topic survives a dwell interval or is explicitly grounded.
86. **Zero-network offline fallback.** Continue local grammar and cached presentation while labeling source-dependent claims stale or unavailable.
87. **Local dynamic-context preview.** Prepare a source-linked context bundle candidate for the selected task without sending it to an AI provider.
88. **Naked-model payload forecast.** Estimate likely minimal context for an AI with memory disabled, but disclose only after destination-specific Host authorization.
89. **Prediction-plus-authorization firewall.** Never let speculative command IDs, user habits or low latency bypass approval, receipts, recovery or revision guards.
90. **Privacy-preserving personal recall port.** A probable reference to old conversations invokes only an owner-authorized Host personal read projection, not HQ's development DB.
91. **Source revocation interrupt.** If a personal source is revoked mid-type, cancel its pending retrieval and invalidate derived suggestions immediately.
92. **Staleness-aware visual decay.** An indexed fact changing under the user's fingers degrades its visual grounded status without altering typed text.
93. **Concurrent surface continuity.** Desktop and phone may display the same selected semantic frame through a versioned read-only projection without broadcasting keystrokes by default.
94. **Cost-budgeted speculative branches.** Bound speculative CPU, index reads, battery and sensitive-source queries per keystroke, suppressing low-value branches.
95. **Provenance-carrying completion.** The second Tab commits an intent whose slots each retain identity, source digest, revision and reason for selection.
96. **Latency-proof instrumentation.** Measure input-to-first-symbol, first-menu, grounded-Tab and source-verification times separately; never claim unmeasured real-time behavior.
97. **Two-Tab acceptance benchmark.** For previously learned frequent commands, test whether two Tabs reach an inspectable fully grounded intent at a measured error target.
98. **Adversarial intent collision corpus.** Test `full` inside quotes, negated commands, private-source references and ambiguous project names against unsafe auto-completion.
99. **Predictive repair before failure.** When the preview predicts a schema mismatch or stale alias, offer a corrected alternative before a user submits anything.
100. **Self-evolving prediction contract.** Allow the Rust-owned computational projection to recommend new keyboard grammars, but promote them through user approval and versioned compatibility tests.

## Convergent architecture: no central router

```text
Keystroke / text revision / caret context
   ├─ existing deterministic lexical+intent reader → grammar candidates
   ├─ existing Rust graph compute → entity and dependency candidates
   ├─ existing HQ development index/wiki → source-linked read candidates
   ├─ Host authorized personal read port → only permitted personal candidates
   └─ source metadata + learned local preferences → ranking candidates
                      ↓
        PURE projection / evidence join / revision guard
                      ↓
        In-composer ghost sentence + slots + Tab menu + minimal status
                      ↓
      explicit user selection → typed, inert semantic frame
                      ↓
        Host effect authority / approvals / receipts (only if invoked)
```

This is **not** a newly authoritative router: separate owners compute in parallel; the lens assembles read-only candidates and keeps conflicts explicit. Rust is the intended common computation kernel; owner ports must be verified before implementation, and no prediction can grant access or change governance.

## Progressive implementation acceptance

- **Wave P1 — Instrument:** input-to-symbol, Tab accuracy, correction/undo count, IME/caret stability. Keep all data local and bounded.
- **Wave P2 — Pure grammar:** ghost completion, 2-slot navigation, negation/quotes and lossless source spans; no knowledge reads needed.
- **Wave P3 — Grounding:** project/focus menus from observed HQ+Host+TierFlow inventory with revision-bound evidence and explicit fallback.
- **Wave P4 — Parallel read-only prewarming:** existing HQ/wiki and Rust kernel projection; cancel stale work, cap IO, never read restricted sources speculatively.
- **Wave P5 — Adaptive user model:** session-only selection ranking first, later opted-in persistent per-owner habits, correction learning, alias collisions and forgetting.
- **Wave P6 — Acceptance:** real Windows/phone, keyboard and screen-reader operation; false-positive risk gates; verified source receipts.

### Suggested product-level experimental objectives (hypotheses, not current metrics)
- Familiar, fully grounded 2-slot mission: at most 2 Tab presses in >=80% of eligible attempts with no silent semantic errors.
- Initial purely local visual response: median under 30 ms on target device, with an explicit resource budget.
- No unauthorized effects from speculative computation or inert completion; zero permitted false authority labels.
- Predictive menu stability: no cursor displacement under IME, typing bursts or stale responses.
- Read-only data freshness and runtime permission displayed independently with provenance.

### Ethical and architectural constraints
- No silent provider send or browser automation; no provider APIs; signed-in local-browser path only through Host where permitted.
- Never commit personal history, user-specific learned ranks, contact values or private sources to public Labs.
- The learner may influence candidate **ranking**, not command semantics, ownership, policy, authorization or audit receipts.
- No opaque full auto-accept on negative, conflicting or high-risk commands; keep manual alternatives accessible.
- Reuse HQ's index/wiki and Rust graph kernel; use a Host-authorized personal graph read adapter for private data.
- Tab cycles choices in a predictable keyboard model; avoid stealing Tab from accessibility navigation without a documented mode.
