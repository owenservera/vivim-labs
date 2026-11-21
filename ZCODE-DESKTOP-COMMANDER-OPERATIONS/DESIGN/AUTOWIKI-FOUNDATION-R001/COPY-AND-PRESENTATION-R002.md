# R002 human language and progressive bundle examples
All examples below are *illustrative copy*, not claims about live feature readiness.

## Levels are reader intent, not simply more tokens
Overview answer: "AutoWiki helps you explore what the system currently knows. It connects topics, capabilities and their supporting evidence."
Explain answer: "A page is assembled from graph facts associated with the current topic. The semantic layer checks identities and relationships; the page layer converts supported facts into explanations and related links."
Evidence/Trace answer: "Claim wiki.fact.X: subject=wiki.page.Y; predicate=represents; object=graph.entity.Z; origin=source digest [...]; graph epoch [..]; truth status=[..]."
These three must refer to one pinned identity and source revision; a later stage may not add new facts without a source update.

## Intent-to-card taxonomy
- definition / 'what is X': intro + 2 related concepts + where-used.
- mechanics / 'how X works': flow + input/output contract + source-backed dependencies.
- reasoning / 'why': evidence chains, contrasted options, what is still unknown.
- status / 'what changed': event timeline + applied receipts + latest verified tests.
- troubleshooting / 'why not working': real error state + affected capability + next safe diagnostic.
- action / 'can you do X': discoverable typed operation, readiness, approval, nonexecuting preview.
- navigation / 'where is X': exact matching page(s), breadcrumbs, linked entities.
- comparison / 'X vs Y': only mutually evidenced assertions; if one side lacks coverage, show gaps.

## Suggested card skeleton
Title; one useful sentence; reason this surfaced (exact subject match / related concept / recent accepted change); verified/candidate/stale status; source revision; 1-3 links. No unsupported similarity confidence. Expanded card: explanation + relevant related entities + action readiness + evidence/trace, all scope checked.

## Fallback prose
- No match: "I don't have enough information to connect that phrase to a wiki topic yet. You can browse the main pages or choose a more specific term."
- Multiple exact candidates: "That could refer to several items. Choose the one you mean."
- Stale source: "This topic changed since the last verified view. Refresh to see the current source."
- No source: "This page has not yet been grounded in an available source."
- Unauthorized: "This information isn't available in the current session."
- Command unwired: "The capability is described, but it is not verified as available for execution."

## Avoid
Do not claim a code annotation means a capability works. Do not turn syntax fragments into elegant but unsupported explanations. Do not call one past applied receipt proof of current runtime health. Do not fabricate page relationship, owner or history from keyword overlap. Do not offer an edit by using a read-only wiki card as authorization.
