# Frozen interfaces, contracts and exclusive ownership

## Identity
Engine family: vivim.ui-engine.<domain>@1
Named capability: vivim.ui-engine.<domain>.<feature>@1
Component kind: vivim.ui.<domain>.<family>.<kind>@1
Proposal schema: vivim.ui-config-proposal/1; output executed=false, applied=false, persisted=false, authorized=false.
Token package: vivim.ui-token-pack/1 with explicit typed DTCG-style groups and alias syntax.

## Shared type vocabulary
color; color-alpha; gradient; shadow; length; number; integer; duration; vector2; vector3; transform;
range; boolean; enum; token; ref; map. Values are inert data only. No CSS code or event handlers.
Typed color permits strict hex, structured sRGB, Display P3 and OKLCH; renderer support is separately gated.
Enums are fixed by source-reviewed enum-choices.mjs, NEVER supplied by user.

## Pure core APIs
listEngineFamilies, listEngineCapabilities, getEngineFamily, getEngineCapability,
inspectUiEngineApplicability, validateEngineSetting, inspectEngineExtension,
compileUiConfiguration, validateUiValue, resolveUiTokenPackage, resolveUiTokenReference.

## Runtime authority
VEC owns pure configuration semantics and IDs; Host owns personal program data/revisions/grants, pairing, effects and receipts.
TierFlow/PWA own existing source validators, projection and reviewed renderer adapters; no second UI publisher.
HQ owns developer source evidence and Wiki indexing; planned VEC Wiki compiler migration is staged shadow parity.
Rust may provide measured pure computation through reviewed ABI; its current inert graphs do not confer grants.

## 100-agent rule
Each worker owns exactly its unique contribution/implementation/adapter/test/proof folder. Shared src/ catalog and ABI
can only be edited by the serial engine integration lead after independently verified proof.
No worker modifies root package manager manifests, Host dispatcher, TierFlow publisher, or accepted HVI gates.
All workers must report exact SHA, observed command exits, remaining UNRUN, source pins and rollback path.

## Standard source-owner ABI for implementation contribution
Module descriptor: id/version, consumes/produces types, dependencies, applicable taxonomy traits, sideEffects:none,
unsupported features, test receipt and reviewer requirements. Optional exports inspect, normalize, validate, plan.
A source comment, component node, model proposal, type descriptor or token alias is NOT a runtime permission.
