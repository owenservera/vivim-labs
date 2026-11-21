# Source crosswalk — current ARBOR truth

| Source schema | Source file | Distinct closed kinds | Mapping entries | Status |
|---|---|---:|---:|---|
| `vivim.ui-surface/1` | `apps/mobile-relay/generative-ui/catalog.mjs` | 10 | 11 | PWA source-closed, device acceptance separate |
| `vivim.ui-document/1` | `packages/tierflow/src/visual-runtime/document/schema.ts` | 10 | 11 | Inert document/validator, not product publisher |
| `tierflow.component-inspector/1` | `packages/tierflow/src/visual-runtime/components/catalog.ts` | 9 | 9 | Fixture inspector only |
| `tierflow.world-shape/1` | `packages/tierflow/src/ui/types.ts` | 6 | 6 | Spatial geometry only |
| **Total** | | **35 catalog entries** | **37** | **Not 35 ready PWA components** |

Variant rule: source type `text` + `variant=heading` maps to `vivim.ui.foundation.typography.heading@1`; ordinary text maps to `vivim.ui.foundation.typography.text@1`. Never silently map a variantless Heading to plain text if semantics matter.

The runtime registry reports:
- `taxonomy-only`: no matching source binding, e.g. Slider. Requires implementation and tests.
- `source-mapped-not-accepted`: mapping exists in inspected source, but not proof of PWA behavior.
- `source-closed`: one type explicitly in the current closed source vocabulary, **not** an effect grant.
- `fixture-inspector-only`, `world-shape-only`: useful crosswalks, not renderable phone promises.

## Binding truth ownership

Current closed component vocabularies remain authoritative for actual validation. Unit tests parse their source lists and check exact mapping coverage. New taxonomy names cannot evade `compileUiProposal` or dynamically inject `renderSurface` cases. The existing Host `apply-proposal`, private visibility guards, SW shell-only cache and Direct read-only static files remain in force.

## Promotion gates

G0 registry metadata and exact source pins; G1 type/props/a11y/slots review; G2 source renderer + typed UI compiler integration; G3 Host current grants/effects and single publisher; G4 desktop/Android PWA keyboard/touch/assistive parity; G5 independent regression, revocation, low disk and recovery; G6 owner acceptance, then advertise `installed` availability separately. No auto-promotion from metadata.

### Compatibility contracts required in future

A mapping must specify: semantic version, source schema version/native type, property transforms, event transforms, a11y/keyboard equivalence, responsive fallback, data permissions, source revision guard, lossy fields and inverse edit feasibility. Current crosswalk only maps **identity** and variant; full adapters remain future engineering.
