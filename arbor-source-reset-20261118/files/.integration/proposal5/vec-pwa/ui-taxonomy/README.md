# VEC — Universal UI taxonomy backbone

**Status:** A full **descriptive** registry implemented on the draft VEC paired-PWA branch; **not** 417 installed widgets. Existing PWA/TierFlow renderers, Host grants, writer and device security gates remain authoritative.

## Taxonomy coverage

- **9 domains**, **34 families**, **417 versioned component identities**, **37 explicit source mappings**.
- Canonical JS source: [`catalog.mjs`](../../../../apps/mobile-relay/vec/ui-taxonomy/catalog.mjs) — pure declarative data.
- Runtime lookup and proposal validation: `apps/mobile-relay/vec/ui-taxonomy/registry.mjs`.
- Full [machine taxonomy](../machine/ui-taxonomy.v1.json), [individual component passports](../machine/ui-component-passports.v1.json).
- [Family matrix](02-FAMILY-MATRIX.md), [component contract](01-COMPONENT-CONTRACT.md), [crosswalk/implementation gates](03-CROSSWALK-AND-GATES.md), [component adoption protocol](04-EXTENSION-PROTOCOL.md), [web standards references](05-STANDARDS.md).

## Five layers

1. **Domain**: foundation, navigation, input, feedback, data, media, application, spatial, accessibility.
2. **Family**: specific broad function (numeric controls, charts, workflow, overlays, responsive layout).
3. **Kind**: stable versioned semantic component ID, e.g. `vivim.ui.input.numeric.slider@1`. Every kind belongs to exactly one family; aliases and crosswalks must be explicit.
4. **Capabilities/contract**: properties with types, events, states, source-data shape, interaction/effect class, semantic accessibility role, keyboard behavior and required alternative input.
5. **Implementation binding**: exact source schema and native type, with source-only maturity. Unmapped kinds are `taxonomy-only`; source-mapped kinds are **not** auto-installed nor security-accepted.

## Example — a slider

The registry identifies slider as `input.numeric.slider@1`, with `min/max/value/step/disabled/orientation` properties, `input/change/commit` events, keyboard arrow control and multiple states. **Current paired PWA has no native slider in its closed component catalog**, so the registry correctly returns `SOURCE_COMPONENT_UNMAPPED`; it does not invent HTML or bypass the existing renderer.

## Extension model

`inspectUiExtensionProposal` validates new component metadata, emits only an **inert** candidate, and requires reviewed semantic/renderer/a11y/Host approval. It does not change the global catalog, mount a widget, authorize a source, or install a plugin. Approved additions advance through existing renderer owners and real device acceptance.

## VEC migration

The same taxonomy is intended to support:
- **Wiki compilation**: document and inspect component/source capabilities via future HQ→VEC shadow parity.
- **Command compilation**: distinguish commands executed *on* a component from requests to change the component tree; Host dispatch remains sole effect owner.
- **UI reprogramming**: compile prompts/gestures to typed component operations using the same versioned IDs and Host revision guards.
- **SDK/Rust**: share typed descriptor contracts; optimize pure transforms where proven useful; do not run Rust from comments without reviewed isolation.

Source truth: the PWA currently has ten closed render types; TierFlow Visual/1 has ten; the separate TierFlow fixture inspector has nine; World geometry has six. Overlapping mappings account for **37 entries including heading variants**, not 37 independent ready widgets.

## Test truth

A pure registry smoke run with synthetic inputs executed in the connected JS tooling and returned all 417 identities, a mapped heading, a slider contract and an inert extension candidate. Node24 unit suites, actual Android paired PWA integration, accessibility audit and publisher acceptance remain **UNRUN**. No source migration cutover or main merge.
