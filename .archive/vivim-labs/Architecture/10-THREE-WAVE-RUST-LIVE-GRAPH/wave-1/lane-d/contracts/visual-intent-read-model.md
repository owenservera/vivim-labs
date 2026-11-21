# Preserved Wave 3 visual-intent fields — design contract only

Lane D records *viewable* semantic fields without implementing a new NL parser, glyph tokenizer, UI renderer, Host executor, WikiProgram configuration authorizer or Wave 3 feature.

Required future typed IR fields (may remain `UNSUPPORTED` in Wave 1): `commandSemanticId`, `sourceSpans`, `selectedEntityId`, `candidateIntent`, `typedSlots`, `missingSlots`, `negationStructure`, `quoteStructure`, `clauseStructure`, `alternatives`, `risk`, `requiredScopes`, `sourceEvidence`, `worldRevision`, `compilerVersion`, `inputRevision`, `status`, `effectPreview`, `uncertainty`, `ownerApprovalRequired`.

Contract requirements: preserve UTF-16 or documented offset units, source/world/input revision pins and stale-render cancellation, separate source declaration grammar from user NL grammar, report quoted payloads/negations/multiple clauses explicitly, and keep candidate interpretation inert. A user selection/gesture may propose the same typed operation as CLI/NL; it does not authorize the effect. Existing TierFlow/Host owners retain publication and permission boundaries. All missing fields reported honestly, not synthesized as confident defaults.
