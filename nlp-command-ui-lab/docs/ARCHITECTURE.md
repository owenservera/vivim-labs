# Visual command compiler architecture

## The one-way pipeline

```
User keystrokes
  ├─> Host `intent-planner.planText` (pure, existing NLCL) ─> candidate / refusal / clarification
  ├─> HQ `registry-adapter.inventory()` (source backed) ─> action + CLI evidence
  └─> optional local contacts read projection (bounded; never an identity grant)
        ↓
Visual projection (`visualCompile`) ──> symbols / slots / menus / English / trace / gaps
        ↓
Browser experiment (no execution path)

Actual effects (future production only) remain with Host dispatcher + approvals + journal + receipts.
Published UI documents (future production only) remain with TierFlow guarded publisher.
```

The lab **must not** create a second command registry, change existing parse byte semantics, independently map different risk systems, or introduce a new Rust/SQLite graph/governance layer. It is a *visual and symbolic compiler* and research measurement surface. This is intentionally different from `vivim-core`'s general command definitions or the Rust comment compiler.

## Source and identity rules

1. Host catalog is authoritative for Host action IDs, input parameter schemas, readiness vocabulary and effects. `source_wired` is NOT proof of production availability.
2. HQ `inventory()` yields *observed* actions + CLI roots; it includes source hashes and admits unknown runtime proof. It is NOT automatically mapped to Host action identities.
3. Omega 17 symbols form a **visual vocabulary and UX research baseline**; they cannot bypass policy (`!` does not skip approval).
4. Contact examples stay fake until a scoped, consent-appropriate Host contact index read port is available. Never infer authority from a name match.
5. Candidate interpretation is effect-free; `planText` outputs remain candidates. A clickable icon never means permission.
6. UI revisions/caret guard: browser aborts and ignores earlier requests; source revision digest travels with each result. Any *real* future action would need pinned World/catalog revisions and Host revalidation.

## Proposed future production contract (NOT active)

```
VisualCommandProjection {
  schema: "vivim.visual-command-projection/1",
  sourceText, sourceCaret, sourceRevision,
  authority: {hostCatalogRevision, hqGraphRevision, worldRevision},
  command: {actionId, namespace, candidateOrUnknown, readiness},
  tokens: [{span, role, family, note}],
  symbols: [{iconRef, semanticRole, canonicalId}],
  slots: [{slotId, entityQuery, groundedCandidates, uncertainty}],
  englishReading, ambiguity, riskPreview, provenance, evidence,
  executionPerformed: false
}
```

**Only a typed Host/TierFlow adapter may promote this projection** to interactive product UX. The Lab version deliberately uses a narrower prototype shape.

## Trust boundaries

- Browser → loopback lab server only (`127.0.0.1`, strict Host header, no permissive CORS, same-origin POST, JSON body cap). No external browser automation.
- Server → local repo **read-only** imports. Never imports Host dispatcher, never spawns shells, never writes to HQ repositories.
- Browser prints untrusted strings with escaped HTML or textContent; nothing in the UI executes code from catalogs.
- No persisted chat transcripts or real contact exports in the repo.

## Acceptance criteria

- Deterministic exact symbol/slot output for the same pinned source; no old keystroke response may overwrite a newer result.
- `email` opens Contacts before typing a name, `email Pet` narrows, Tab inserts selected contact into exact span.
- Longform English interpretation **directly under chat composer** on every keystroke, visible by default, togglable for power users.
- A catalog that is missing, stale, unverified or rejected is explicitly shown as such.
- Negation/compound phrasing must never be reduced to an executable candidate by speculative UI heuristics.
- Real outcome requires Host effects/receipts, entirely outside the Lab; tests prove there is no execution route.