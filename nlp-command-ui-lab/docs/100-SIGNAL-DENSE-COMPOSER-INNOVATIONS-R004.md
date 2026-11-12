# 100 Signal-Dense Chat Composer Innovations — R004

**Scope:** the chat box itself; no side panels, external wiki pages, full-screen canvases, or execution engine replacement.
**Design law:** maximum useful meaning per pixel; visible uncertainty; input stability; accessibility; explicit authority; no accidental actions.
**Status:** experimental idea catalog. These are design proposals, not implemented or accepted features.

## ERA 1 — The atomic visual language (1–34)

1. **Unique glyph per registered command** — render an unmistakable command icon keyed to the canonical action ID, never a guessed executor.
2. **Composable glyphs** — combine action, target and modifier mini-icons into one readable emblem.
3. **Semantic role ink** — use a small color vocabulary for actions, entities, destinations, dates and conditions.
4. **Outline truth grammar** — solid for grounded, dashed for candidate, double for locked; text labels back up every signal.
5. **Risk edge accents** — put operational risk on the chip boundary, not across the whole input.
6. **Typeface switching** — prose, identifiers, quoted literals and executable syntax use visibly different typographic voices.
7. **Uncertainty notch** — one tiny corner cut indicates a missing or ambiguous parameter.
8. **Icon-to-language reversal** — click any icon to expand it into its exact plain-English meaning.
9. **Stable entity monograms** — known projects and people retain visually recognizable identity marks.
10. **Semantic ligatures** — compact common structures like 'send to' into a legible action→recipient symbol.
11. **Role-aware cursor highlight** — illuminate only the active verb, parameter, or referenced object.
12. **Literal quote shields** — visibly protect quoted text from accidental command parsing.
13. **Negation brackets** — draw an unmistakable brace over the clause that 'don't' negates.
14. **Operator punctuation** — visually distinguish questions, conditions, direction, urgency and confirmations.
15. **Required-slot dots** — use a tiny count of missing arguments instead of a bulky warning panel.
16. **Inline completeness ring** — show whether a command frame has all its necessary fields.
17. **Temporal mini-signals** — represent a date, range or duration as a small timeline mark.
18. **Typed-value shapes** — contacts, files, projects, URLs and numeric values get different chip geometry.
19. **Source-origin corner marks** — show whether a meaning came from Host, HQ, TierFlow or a local exploratory rule.
20. **Freshness tick** — display the age or revision of the evidence behind a resolved object.
21. **Ambiguity fork** — one icon splits into two branches when interpretations compete.
22. **Modifier glyph stacking** — represent priority, approximation and conditions without adding words.
23. **Progressive symbolization** — keep unfamiliar commands as words and compress them after the user learns their icon.
24. **Locale-sensitive glyph labels** — preserve canonical IDs while presenting localized names and aliases.
25. **Accessibility twin rendering** — every symbol exposes a persistent short label or screen-reader name.
26. **Micro-animation as explanation** — animate only real semantic transitions such as candidate→grounded.
27. **Reduced-motion parity** — replace animation with equally informative static marks.
28. **Contrast-adaptive semantics** — preserve meaning on dark, light, high-contrast and color-blind palettes.
29. **Negative space codes** — use whitespace grouping to distinguish separate commands from tightly bound arguments.
30. **Semantic superscripts** — show tiny optional role letters only in advanced mode.
31. **Inline proof flags** — a miniature proof mark appears only when runtime evidence exists, not merely source registration.
32. **Command-chain numerals** — show how many actions are inferred without expanding the entire plan.
33. **Density budget** — automatically pick text, icon+label or icon-only rendering based on available width and ambiguity.
34. **Revision fingerprint** — a compact hash mark identifies the exact interpretation and catalog version.

## ERA 2 — Contextual menus and compositional editing (35–67)

35. **Verb-triggered smart menus** — typing 'email' immediately exposes eligible recipient selection.
36. **Caret-anchored contextual menus** — menus follow the currently edited semantic slot rather than the screen edge.
37. **Personalized menu ordering** — reorder suggestions from explicitly observed selections without changing permissions.
38. **Workspace-scoped choices** — rank projects and tools relevant to the active workspace first.
39. **Contact identity picker** — never substitute a guessed name for a verified contact record.
40. **Tab-to-ground** — commit the highlighted suggestion with Tab and preserve cursor location.
41. **Shift-Tab rewind** — revisit the previous semantic slot without selecting the entire sentence.
42. **Arrow-key navigation** — move between suggestions while keeping the composer focused.
43. **Escape-to-dismiss** — decline a suggestion without destroying typed text.
44. **Type-to-narrow entities** — immediately filter live scoped results as the name grows.
45. **Explainable fuzzy matching** — suggestions show why a close match appeared, including typo ambiguity.
46. **Two-way disambiguation** — competing meanings can be compared inside a single compact menu.
47. **One-question repair** — ask for the highest-impact missing fact instead of opening a form.
48. **Menu-on-need** — surface no autocomplete until it improves the next likely action.
49. **Slot trail** — show one thin inline breadcrumb of already filled command arguments.
50. **Next-slot anticipation** — suggest the next required argument after each committed selection.
51. **Micro completion meter** — small dots show missing versus grounded slots without a separate status panel.
52. **Exact-target drilldown** — selecting a contact or file opens a bounded identity detail in place.
53. **One-hop disclosure** — hover or focus to inspect provenance, confidence and aliases, then collapse.
54. **Nested menu folding** — deeper options slide into the same small anchor instead of launching dialogs.
55. **Semantic command discovery** — search by what an operation does, then insert the actual registered command.
56. **Menu grouping by task** — show contacts, projects, tools and files only when their slot type is compatible.
57. **Partial-chain editing** — select and repair one step of a compound command without rewriting the rest.
58. **Chip-to-text toggle** — turn any structured segment back into editable natural language.
59. **Local consequence peek** — a brief preview appears beside the token whose effect is changing.
60. **Structured paste intake** — detect pasted lists, links, commands or quoted text and offer appropriate safe wrapping.
61. **Voice-partial visuals** — render provisional transcription as provisional meaning, never as auto-execution.
62. **Why-this-suggestion key** — one shortcut explains the active menu recommendation.
63. **Provider-aware affordances** — show possible service targets but do not infer that an account is connected.
64. **Duplicate-recipient brake** — visually distinguish two identically named contacts before commitment.
65. **Source-permission marker** — a contact or file menu indicates whether the source is readable and authorized.
66. **Keyboard-first semantic travel** — move between action, recipient, topic and time using consistent shortcuts.
67. **Stable layout under typing** — reserve menu space or overlay it so interpretation updates never move the caret.

## ERA 3 — Strange but testable interactions (68–100)

68. **Semantic gravity** — recognized phrase fragments gently condense into one meaningful chip when that reduces clutter.
69. **Ghost interpretations** — show speculative meaning as translucent, clearly uncommitted UI.
70. **Counterfactual peel** — press a modifier to reveal 'if this were the command, then these effects would follow'.
71. **Contradiction spark** — a tiny mark appears where two clauses give incompatible instructions.
72. **Scope bubble** — surface the current project/account scope as one removable contextual token.
73. **Permission is not knowledge** — a visually separate authority gate makes that distinction unavoidable.
74. **Provenance filaments** — thin links connect tokens to source IDs on demand without opening a side panel.
75. **Reversible semantic edits** — change an argument using a tiny local control and show the exact original-text diff.
76. **Ephemeral thought capsules** — preserve a temporary parsed idea in the composer without saving it to durable memory.
77. **Temporal ink** — fade or restyle derived references when the referenced world revision changes.
78. **Freeze understanding** — pause live reinterpretation while the user reviews an unstable complex phrase.
79. **Live correction diff** — highlight precisely which meaning changed after each keystroke.
80. **Two-axis readiness** — distinguish 'I understand the request' from 'I can perform the action' in two compact marks.
81. **Ambiguity debt counter** — show unresolved assumptions as a numeric indicator with a one-tap repair list.
82. **Permission friction proportionality** — low-risk previews are frictionless; consequential changes expose clear approvals.
83. **Proof-before-perform watermark** — no action symbol can look fully actionable without execution-authority evidence.
84. **Optimistic local semantics** — tokenize immediately, then reconcile with authoritative source interpretations.
85. **Latency-aware degradation** — when external context is slow, retain the last stable view and mark it stale.
86. **Attention budget governor** — render only the few highest-value signals until the user expands details.
87. **Semantic cursor shadow** — a tiny hint follows the caret showing the role currently being filled.
88. **Clause microscope** — temporarily enlarge only the selected clause while the rest compresses into crumbs.
89. **Hidden semantic footnotes** — attach evidence, exclusions and caveats to a tiny focusable annotation marker.
90. **One-line branching accordion** — fold multiple possible intentions into one row of alternatives.
91. **Missing-context silhouettes** — show the type of absent information without pretending to know its value.
92. **User-owned icon dialects** — allow approved alternate symbols for frequently used operations.
93. **Visual macro stamps** — condense confirmed multi-step command patterns into an inspectable composite icon.
94. **Haptic semantic feedback** — on supported mobile hardware, optional patterns distinguish grounded, ambiguous and blocked.
95. **Offline integrity mode** — retain the visual grammar while visibly downgrading unverifiable source claims.
96. **Micro trace ribbon** — one compact strip shows lex→frame→ground→risk stages for power users.
97. **Shape-shifting command spine** — the composer rearranges its internal controls for the task, not into a separate application.
98. **Confidence shadow instead of numbers** — a subtle visual texture represents uncertainty without implying fake precision.
99. **Intent collision detector** — show when a user phrase matches unrelated command families, with instant disambiguation.
100. **Self-proposed UI mutations** — the composer suggests a better affordance based on recurring friction, but applies only after user approval.

## Test requirements

- Measure: recognition accuracy, false confidence, keystrokes to correct, layout shift near cursor, characters per rendered semantic unit, time to comprehension, keyboard-only flow, WCAG semantics, mobile thumb reach, and source freshness.
- Compare: quiet (one line), assisted (chips + menu), X-ray (full explainable interpretation). Every layer is optional.
- Reuse: Host typed catalog and planner; HQ source inventory and wiki/context evidence as **inputs only**; TierFlow renderer when promoted; Labs for disposable UI experiments.
- Never: execute from preview, fake grounded contacts, silently merge action IDs, grant permission from semantics, store personal chat text in Labs, or convert visual confidence into operational authority.