# Recipe A1 — additive comment parsing, exact attachment and v3 grammar (near-code)
**Owner A. Write targets:** `hq/lib/annotation-v2.mjs` (minimal additive export), `hq/lib/semantic-capability/comment-v3.mjs`, `source-observation.mjs`; **do not modify** existing `hq/lib/extract.mjs` or legacy v1 annotation rows unless independent owner evidence shows a required fix.

## Current behavior to preserve
`parseAnnotationV2(source,{path,contentHash,projectId,symbolId,registry})` accepts nine v2 tags and skips other kinds; its private `scanCommentSpans` correctly avoids string/template/regex false positives; returns 1-based line numbers, record array and diagnostic array. It may return records for unknown registry refs despite an error diagnostic. **Do not treat records as valid if diagnostics include any severity error.** Existing v2 tests must remain byte-for-byte identical.

## Minimal additive export (proposed, not implementation)
```js
// In annotation-v2.mjs: wrapper around its existing, vetted private lexer.
// Keep parseAnnotationV2() unchanged for old callers.
export function listAnnotationCommentSpans(source) {
  if (typeof source !== 'string') return [];
  return scanCommentSpans(source).map(({start,end}) => ({start,end}));
}
```
Do not export live internal mutable buffers or a second comment parser. Check scanCommentSpans returns boundaries excluding/including delimiters in actual code and normalize precisely in wrapper.

## Proposed v3 grammar
```ebnf
tag          ::= "@hq:" key WSP value
key          ::= "unit" | "role" | "relation" | "migration" |
                 "wiki-topic" | "verify" | "observe" | "explain" |
                 "utterance" | "frame" | "slot" | "constraint" |
                 "visual" | "example" | "disclosure"
value        ::= versioned_ref
versioned_ref::= name "@" number ("." number)*
name         ::= [a-z0-9] [a-z0-9._-]*
```
The routine v1/v2 tags remain in their present modules. All new v3 tags carry **only registered data-only refs**; no inline arbitrary predicates, scripts, expressions or action privilege strings. For `unit/role` map to proposed additive semantic registry kinds `unit/role`; `verify/observe/explain` map to `verification-profile/observation-profile/explanation-pack`. `wiki-topic` requires approved type/semantic concept (check current registry kind). `utterance/frame/slot/constraint/visual/example/disclosure` remain unsupported (diagnosed, not silently accepted) until their owner-approved SWC work. `relation/migration` refs require reviewed relation/lineage objects, not free text.

## Algorithm (pseudocode, implementation target)
```ts
function parseScfAnnotations(text, {language,path,blobSha,registry,symbols,semanticProtocolVersion}) {
  assertSafeBoundedSource(path, text, blobSha);
  const v2 = parseAnnotationV2(text, {path,contentHash:verifiedContentHash(text), registry});
  const rawComments = listAnnotationCommentSpans(text);
  const v3 = semanticProtocolVersion === 3
      ? rawComments.flatMap(span => extractKnownV3ReferenceTokens(text, span))
      : [];
  const legacyErrors = v2.diagnostics.filter(d => d.severity === "error");
  const v3Diagnostics = validateRegisteredTypedRefs(v3, registry);
  const attach = attachToFileOrExport(rawComments, [...v2.records,...v3], symbols, text);
  const errors = [...legacyErrors,...v3Diagnostics.filter(d=>d.severity==="error"),...attach.diagnostics.filter(d=>d.blocking)];
  return {
    schema: "arbor.scf.parse/0.1",
    tags: errors.length ? [] : attach.validatedTags,
    attachments: errors.length ? [] : attach.attachments,
    diagnostics: stableSort([...v2.diagnostics,...v3Diagnostics,...attach.diagnostics]),
    coverage: summarizeWithUnknowns(text, symbols, errors)
  };
}
```
**Source hash:** SHA-256 of actual bytes; caller-supplied hash (research SWC) MUST be verified against the source adapter before use. Keep Git blob SHA independently (normally SHA-1, may be SHA-256 object format). Never compare incompatible digest families.

## Scope attachment algorithm
1. Identify the initial source-header region as leading comments (permitted shebang/license) before the first real declaration. Tags here have `scope=file`; repeated capabilities are a set, NOT one implication that every symbol implements every feature.
2. A comment immediately preceding an exported declaration, with only whitespace/decorators and no other declaration between, is a candidate `scope=symbol`. Validate `symbol.line`, `symbol.end_line`, `symbol.name`, export status via `extractSymbols(lang,text)`; source-offset and byte-range proven. TS overloads/JS exports/Rust impl methods require explicit disambiguation.
3. A comment inside a function or above multiple declarations without unique attachment -> `scope=unknown`, blocking if it declares a capability/contract/verify ref; fail closed rather than arbitrarily attach to nearest symbol.
4. If `@hq:unit` appears twice with distinct values on same anchored symbol, `SCF_ID_CONFLICT`. A `@hq:capability` may repeat for different registered features but exact duplicates collapse with warning; no count double.
5. Raw comment tokenizer is untrusted. `@hq:` within string/template/regex, Markdown fixture, example fenced code, inline HTML or vendor/generated file **must not be treated as a first-party declaration**.
6. Annotation source text never becomes an executable instruction; `@hq:effect` descriptive only.

## Required fail cases
- Current v1/v2 differential tests remain identical.
- `const x = "@hq:capability foo@1"` => no annotation.
- `//@hq:capability foo@1` as a legitimate comment => only after scope check.
- Unknown v2 registry ref produces a record but has error => zero publishable facts.
- v3 tag in v2-only mode => `SCF_TAG_UNSUPPORTED_VERSION` diagnostic; no silent successful v3 fact.
- Source comment above two ambiguous exported functions => `SCF_ATTACHMENT_AMBIGUOUS`.
- Reordered comments or CRLF line endings => deterministic scope and hash semantics; preserve source bytes for provenance.
- Invalid `../`, secret-looking paths, giant comments and recursion bombs => hard errors with bounded logs.
