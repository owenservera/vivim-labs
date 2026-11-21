# R002 UX, trust, latency and link acceptance contract
Status: specification, not executed checks.

## Non-negotiable observable user scenarios
A1 Empty draft: stable human-written/verified Home with active hubs and no invented summaries. If no source, show useful empty state.
A2 As user types 'What does VLANG do': 1-3 topic cards in ambient rail, *without* changing current page or sending a command.
A3 User clicks VLANG: page becomes focused, breadcrumb records route; switching from Overview to Explain then Trace reveals increasing detail from the same authorized graph revision; does not substitute unrelated text.
A4 User pins VLANG, types about permissions: VLANG remains open, permissions card appears as suggestion; pin persists during scoped graph revision change but never across authorization revocation.
A5 Input is rapidly deleted/replaced: earlier responses are discarded by sequence and epoch, even if network callbacks arrive late; no flicker onto old topic.
A6 During Android IME composition: no speculative summary until compositionend; user text is not mutated, no lost caret/scroll.
A7 Page source changes: revision stamped; stale/inconsistent bundle cannot be shown as current, user receives non-disruptive update / re-evaluate option.
A8 Ambiguous noun ('it', 'this', multiple project matches): exact typed command reference remains clarification-required; wiki can show candidate navigation cards but not claim the target is verified.
A9 Read-only path: no Host effect plan, draft submission, provider call, wiki edit, or graph write from typing, opening a link, pin, depth or read of evidence.
A10 Permission downgraded: zero private content, titles, snippets, counts, backlinks or source paths survive on screen, in pending response, or in client cache.
A11 Unsupported or missing claims: readable 'Not established' with source caveat instead of nonsense prose.
A12 Links: only same-scope, exact semantic targets; no dangling links or cross-project leakage.
A13 Persisted/committed wiki sections remain attributable to source and revision; automatic preview is ephemeral, never silently committed.
A14 Enter and 2-TAB are unchanged; no conflict with current composer assistance.
A15 Navigating back: preserve depth and scroll if still valid in current scope/revision; else clearly rebase.

## Performance budgets to test (design targets, not measurements)
- local input reaction visual <= 100ms; no blocking graph work on input thread.
- preview card target <= 400ms after debounce for a warm bounded local source.
- focus page target <= 800ms for warm local sources.
- strict max 3 ambient cards, per-focus item/token bounds documented in Dynamic Lens spec.
- failure timeout produces a stable, helpful unavailable state; no background infinite retry.
- test with large graphs, slow Host, dropped connection, stale source and phone reconnect.

## Rust unit contracts
- Higher input sequence wins; stale reply refused.
- Same input sequence cannot replace a different candidate.
- Typing never changes selected page; explicit navigation does.
- Pin cannot auto-navigate and unpin cannot auto-commit.
- Unknown/unauthorized target refused and cannot be suggested or selected.
- Source revision mismatch and new authority epoch clear previews and retire stale content.
- Detail profile monotonically adds bounded read allowance; scope stays fixed and never grants access.
- Inert outputs always authority none / executionAuthorized false.

## Publication/English acceptance
Each hub has: defined subject; coherent summary; explicit scope; at least one relevant link if evidence exists; current/revision status; own sources. Each factual sentence must map to supported claims, retaining negation, temporal status and owner attribution. Human-level prose never displays raw relation key:value as the only explanation. Unknown 'how' question elicits a bounded explanation of missing evidence rather than invented causal logic.

## Evidence ledger template
G0 exact ARBOR HEAD + source maps: UNRUN
G1 isolated pre-code rustc/cargo: UNRUN
G2 native/Rust HQKG+Wiki adapter tests: UNRUN
G3 browser PWA tab/typing/links/IME: UNRUN
G4 privacy/authorization + Host no-effects: UNRUN
G5 unchanged Android URL observational proof: UNRUN
Do not substitute structural test results for device evidence.
