# Locked compile-time and runtime seams, dependency graph
## Serial dependency graph
```
S0 exact source/ABI/API lock
   ├─ A typed v3 scanner + source-symbol compiler core  ─┐
   ├─ B registry adapter + additive SCF DB store         ─┤
   └─ F negative fixtures + independent tests (no code) ─┤
                     ↓ A+B present                      │
   ├─ C feature passports + Wiki + synthetic HVI adapter ┤
   ├─ D SV independent proof adapter and explanation     ┤
   └─ E agent protocol + diagnostic CLI + dev service    ┤
                     ↓ all A–F source frozen             │
          Z independent verifier of actual diffs/receipts
                     ↓ (GO_FOR_G1 only)
          G serial integration + combined Node24 tests
                     ↓ GO_FOR_HOST_PRODUCT and accepted main
         H Host actor grant binding || U TierFlow UI
                     ↓ independent Z2 + serial G2 review
```
Source/SV, schema and HVI modules communicate through **injected frozen objects**, not imports of unmerged worker files. After source integration G alone wires imports. No circular imports.

## Declared pure JavaScript interfaces (target implementation)
```ts
export function parseScfAnnotations(sourceText: string, options: {
  language: string; path: string; blobSha: string; semanticProtocolVersion: 2|3;
  registry: {lookup(ref: string): RegistryEntry|null};
  symbols: Array<{name:string; line:number; end_line:number; exported:boolean; kind:string}>;
  maxBytes?: number;
}): {schema:"arbor.scf.parse/0.1"; tags:ScfTag[]; attachments:Attachment[]; diagnostics:ScfDiagnostic[]; coverage:Coverage};

export function observeSourceUnit(input: {pin:Pin; path:string; blobSha:string; text:string; symbols:Symbol[]; imports:Import[]; comments:Attachment[]}): SourceObservation;
export function compileCapabilityFacts(input:{pin:Pin; sources:SourceObservation[]; registry:ReadonlyRegistry; ownerCatalog:ReadonlyOwnerCatalog; approvedSourceManifestDigest:string; swcProjection?:SemanticWikiIR; limits?:Limits}): ScfCompileResult;
export function buildCapabilityPassports(facts:ScfFact[], independentEvidence:EvidenceReadPort, opts:{pin:Pin; coverage:Coverage}): Passport[];
export function compareCapabilityRevisions(pins: Array<{pin:Pin; passports:Passport[]; digest:string}>, opts:{targetPinId:string; hviCompare?:InjectedHviCompare}): SemanticDelta;
export function bindIndependentProofs(passports:Passport[], receipts:ReadonlyArray<SvProofReceipt>, approvedSpecIndex:ReadonlySpecIndex): Passport[];
export function compileExplanation(passport:Passport, clauseRefs:ReadonlyArray<string>, approvedTemplates:ReadonlyTemplatePack): SvExplanationEnvelope;
export function loadScfSnapshot(db:Db, pinId:string): Readonly<ScfSnapshot>|null;
export function persistScfSnapshot(db:Db, snapshot:ScfSnapshot): {ok:boolean;digest:string;status:"unchanged"|"stored"};
export function buildMissionContextPack(input:{pinIds:string[]; capabilityIds:string[]; maxTokens:number; approvedSources:SourceScope[]}): Readonly<ContextPack>;
```
These are **specified proposed signatures**, not currently available exports. G must reconcile accepted HVI API shape through an adapter; do not invent a directly callable HVI native method.

## Source line anchor
A `SourceObservation` contains `scope=file|symbol|unknown`, `symbolAnchor=` actual parsed signature+source ranges, comment start/end offsets, and `attachmentVerdict=EXACT|AMBIGUOUS|UNATTACHED`. A legacy parser can output records with bad refs: a source with any error diagnostic has no publishable semantic facts until corrected.

## Port ownership
- `hq/lib/annotation-v2.mjs` exports a *read-only* comment-span boundary helper via additive API; original `parseAnnotationV2` results and test snapshots MUST remain identical.
- `extract.mjs` provides `extractSymbols` and `extractImports`; do NOT rewrite its v1 annotation rows.
- `semantic-registry.mjs` is the only ID authority; `schema.mjs` only existing DB schema authority.
- `indexer.mjs` remains canonical file/symbol/import scanner; SCF should use its data and a narrow admitted post-index hook rather than duplicate full walks. No edits to indexer until Z confirms need.
- `wiki.mjs` remains one generator with optional normalized passport projection.
- `arbor-service.mjs` is the only HQ development read boundary; E owns addition of opt-in methods after base integration.
- `hvi-adapter.mjs` references HVI service via injection, avoiding duplicated N-way engines.

## Failure semantics
All pure stage inputs invalid: return structured diagnostics and partial/blocked result or throw named caller-data validation errors only at public boundary; **never** swallow a denied security/error into `complete:true`. No graph/metadata mutation from a read; no record marked runtime verified without independent receipt. See `04-ABI-ERROR-SEMANTICS.md`.
