# Sprint 02 — Frozen V0.1 Contracts

## Era 1 — Command language (JS/TS)
Canonical IR: `vivim.command-intent/0.1`: input `{text,worldRevision,catalogRevision,audience,scope}`; output `{schema,outcome,actionId?,args?,risk?,requiredScopes?,alternatives?,refusal?,provenance?}`. Outcomes: candidate, clarification_required, interpretation_required, refused. Negation and conjunction **must not be silently erased**. No parser/model suggestion grants permission; fixed Host dispatcher and its journal own effects. Preserve backward compatibility with `vivim.control-plan/1`. Use current NLCL and Host action catalog; no new Rust NL implementation.

## Era 2 — Comment language (Rust)
Canonical compiler input: source text, language identifier, project-relative path, source digest, registry manifest. Output: `hq.comment-program/0.1` JSON containing source mapping, validated declarative `hq.wiki-program/1` configuration, inert interaction bindings to registered action IDs, diagnostics and exact compiler version. V0.1 candidate syntax (new, not yet supported):

```ts
/**
 * @hq:wiki recent-visits@0.1
 * @hq:source docs
 * @hq:filter kind=browser-visit
 * @hq:layout columns=3 density=compact
 * @hq:select mode=recent limit=20
 * @hq:bind open=web.visit.read@1
 * @hq:requires owner-read@1
 * @hq:evidence test:wiki-recent-visits
 */
```

Validate concrete values **against existing** `hq/lib/wiki-program.mjs` (enum and numeric bounds); the example is a proposal, not a guarantee that those values are accepted. Existing `@hq:` v1 and v2 annotations must retain their original behavior. Parse actual source comments, not strings. Reject shell/eval/network execution, secrets, unsupported tags/refs, duplicate definitions, invalid file paths, unsafe effects and unregistered handlers. Comment source cannot authorize a mutation.

## Era 3 — Wiki bridge (JS/TS)
Flow: Rust comment compiler → JSON `hq.comment-program/0.1` → JS validation → existing `hq.wiki-program/1` preview/diff → explicit owner approval → versioned activation/rollback receipt. Wiki interaction and NL command both map to `vivim.command-intent/0.1`-compatible typed IR; only Host/Studio/TierFlow effect owners perform authorized effects. The comment compiler is project-neutral; cross-repo integration later consumes its stable output, not its internal graph state.
