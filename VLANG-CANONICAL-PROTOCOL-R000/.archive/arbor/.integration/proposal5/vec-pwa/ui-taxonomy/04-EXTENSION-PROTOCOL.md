# Adding a UI component without reinventing the system

## Existing standard component
1. Look up stable ID using `getUiComponent` or `findUiComponent`; `findUiComponent` explicitly reports ambiguous names and never chooses from multiple families.
2. Read its inherited role, traits, states, typed property keys and accessibility requirements.
3. Check `resolveUiSourceIdentity` for the intended platform/schema. If none: label `NOT_INSTALLED` and do not make a UI proposal.
4. Extend **the existing closed source component catalog and validator**, or implement a reviewed adapter to a source owner. Do not start a parallel renderer.
5. Add deterministic NL intent grammar and exact component ID/variant binding, then output only the existing `vivim.ui-proposal/1` proposal.
6. Build keyboard/touch/voice/accessibility and effect/state tests, including revocation, disconnect, stale revision, offline SW, PC-private handling, and undo.
7. Update mapping truth and source pins; run generated passport parity tests; obtain real Host and device acceptance before declaring installed.

## New component type (not in catalog)
Submit `vivim.ui-extension-proposal/1` through `inspectUiExtensionProposal` with domain/family/kind/label/properties/traits/events/a11yRole. The result is an inert **candidate** and cannot change runtime registry arrays, Host grants or publisher.

Reviewer must decide whether it belongs to an existing kind as a variant; if truly new, add an explicit stable identity with migration and depreciation policy. Never use arbitrary `html`, `javascript`, `onClick`, dynamically loaded plugins, raw provider URLs or direct filesystem paths as component metadata.

## Workstream ownership
- **VEC taxonomy/SDK:** classification, identity, grammar, compiler contracts and source mapping.
- **TierFlow / PWA UI owner:** accessible renderer and schema conversion, design tokens and device-specific presentation.
- **Host:** source grants, action authority, program revision/publish/journal, pairing/revocation.
- **HQ / Wiki:** indexed development evidence, migration to the VEC Wiki compiler by shadow parity; not private user program storage.
- **Product Rust/HQKG:** pure portable transforms after ABI/parity proof.
- **Independent QA:** synthetic fixtures, real runtime tests, physical PWA and rollback.

The taxonomy is broad from day one; rollout remains incremental and measured. A source-declared component is neither a working interaction nor a secure installed capability.
