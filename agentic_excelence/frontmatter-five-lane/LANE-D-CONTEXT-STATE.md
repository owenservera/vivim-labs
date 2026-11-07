# Lane D — Context, living status and handoff frontmatter

**Owns:** 78 files: family 04 (46), family 12 (20), family 19 (12). Sidecar-first; no new memory owner, transcript store, context composer or automatic sender.

## Type-specific design
- `README.md` 45: path-scope, folder purpose, source provenance, navigation hierarchy, auto-exposure eligibility, canon-versus-context label, content freshness; do not treat folder README as owner policy.
- `CONTEXT.md` 1: context domain, sourceRef/revision, projection authority, trust/sensitivity, dependency neighborhood, policy-scoped delivery. Context is a *read-only projection*.
- `SITREP.md` 20: observed time, exact HEAD/test receipt, reported/observed/verified status, blockers, expiry/invalidation on new events; not evergreen instructions.
- `HANDOFF.md` 12: task owner/next owner, original task identity, source revisions, active context digest, finished work, open scope, in-flight effects and no-resend guard; recovery must be deterministic.

## Connection to continuous DCB + LRU
For every file, specify event triggers and invalidation: source changed, test failed, goal updated, handoff accepted, permission revoked. Dynamic fields live in a versioned **context projection** and must not overwrite the original file's provenance. The next model call can receive a freshly composed projection; never rewrite an in-flight prompt or send private context without the existing Host boundary. Cache keys include project/principal/disclosure/graph epoch/file hash/schema revision.

## Shared vs unique
A README's "context" maps to navigation and discoverability; a SITREP's "context" is dated operational evidence; a HANDOFF's "context" is resumable task state. Preserve unique headings and references to source spans. Give 168 unclassified supplement headings a proposed type only when source body proves it.

## Gate
**78/78** file IDs assigned and specified. Demonstrate three per-file provenance examples plus stale-status, mid-task revision, ambiguous handoff, and cross-principal disclosure denial fixtures. Zero stale-current assertions; no auto-send or owner-policy promotion; Lane E validates.
