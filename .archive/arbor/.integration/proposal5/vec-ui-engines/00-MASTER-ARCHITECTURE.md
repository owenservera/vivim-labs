# VEC Universal UI Engine System — master architecture

Foundation, not a retrofit: the historical PWA theme editor is a donor of limited algorithms, not the capability ceiling.
The user target is any UI component/property that the component taxonomy declares and a reviewed adapter actually supports. One Color Engine, one Border Engine, and a unified collection of central engines; never 100 duplicated mini-engines.

## Verified source foundation
- 37 canonical engine families, 461 typed capability descriptors in packages/vec-ui-engines/src/catalog.mjs.
- Core pure modules: validators.mjs, registry.mjs, tokens.mjs, compiler.mjs, enum-choices.mjs, index.mjs.
- 100 independent agent work orders with exclusive paths and machine manifest; no agents launched automatically.
- Current PWA and TierFlow have CLOSED UI component schemas. Engine identities do not grant render support.
- Output is a data-only, source-revision-bound configuration candidate, not CSS, a runtime action, or a UI publication.

## End-to-end pipeline
1. Natural-language/gesture/property inspector selects specific component instance IDs at exact current revision.
2. Existing VEC component taxonomy identifies domain/family/kind and semantic traits.
3. Engine Registry determines semantic applicability and separately reports actual renderer status.
4. Typed value normalization rejects invalid color, dimension, range, enum, refs, executable text and cycles.
5. Typed token resolver resolves versioned tokens and aliases; mode/cascade owner integration is a pending workstream.
6. Engine dependency graph composes shared computations; Color owns color, Border consumes but never clones Color.
7. Atomic intent compiler produces declarative proposed changes, with source/version and no effects.
8. Accessibility/privacy/system constraints independently inspect final proposed presentation.
9. Reviewed renderer adapter lowers only supported typed fields, with explicit loss/fallback report.
10. Existing Host checks actor/session/grants/current base revision and exclusively publishes; paired PWA renders receipt.

## Separate permanent identities
Component kind: vivim.ui.foundation.surface.card@1. Component instance: source-document specific ID.
Engine: vivim.ui-engine.color@1. Named capability: vivim.ui-engine.color.background@1.
Style token: typed token path and mode, not engine nor component ID.
Program: Host-owned personal VEC revision, never an independent engine store.

## Eight platform planes
Identity; Typed Values; Token Graph and Theme/Cascade; Applicability; Dependency DAG; Compilation;
Renderer+Host Authority; Independent Verification and Recovery.

## First proof of value
Request: 'Make these selected cards deep blue with 12px rounded corners and a soft shadow.'
Resolve exact selected IDs. Normalize three typed capabilities using Color, Corners and Shadow.
Compile one atomic preview; reject unavailable renderer rather than generate code. Approve only via current paired Host.

## Status
Core source established, isolated V8 smoke completed. No Node24/Bun/Cargo or real phone tests executed in this research.
Engines beyond primitive validation are NOT complete. Mode/cascade, adapters, Host/PWA integration and security acceptance are pending.
