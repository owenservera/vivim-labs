# UI identity and component contract /1

## Classification (not a render implementation)

```text
vivim.ui.<domain>.<family>.<kind>@<major>
                         |
                         +- semantic role + composition (leaf/container)
                         +- traits (bindable, interactive, continuous, responsive...)
                         +- properties: typed keys, e.g. number/string/enum/ref
                         +- events: typed *candidate* events, no direct callbacks
                         +- states: idle, disabled, pending, invalid, dragging etc.
                         +- data: none/scalar/collection/series/graph/media/World
                         +- effects: none / host-intent-proposal
                         +- accessibility: role, keyboard, accessible name, alternate input
                         +- bindings: exact renderer schema/native kind/variant/maturity
```

**Identity must be stable even if the implementation changes.** The semantic ID is *not* a DOM tag, CSS selector, React component name, Rust function path, schema version or permission. Source-specific aliases and variant projections are explicit, versioned crosswalks.

Example: `vivim.ui.input.numeric.slider@1` is a component *kind*. A particular instance has a separate document ID, revision, actual value, bounded state and source grants; the value is personal data in Host/World, never the global taxonomy. Its implementation must provide keyboard arrow interactions and an accessible value representation, not merely declare it.

## Six orthogonal contracts

1. **Identity/classification** — unique versioned full ID, name, domain/family and semantic role.
2. **Property/value** — typed fields (including refs), constraints, defaults and allowed edit operations; raw function/callback/URL execution disallowed.
3. **Composition/layout** — leaf vs container, children constraints, slots, responsive viewport conditions and rendering fallback.
4. **State/interaction** — input/commit vs change, keyboard/touch/voice parity, focus order, disabled/invalid/uncertain states.
5. **Data/effects** — scoped source refs, availability, redaction, idempotent command *proposals*, independent Host authorization. An `onClick` string is never an executable action.
6. **Accessibility/evidence/lifecycle** — semantic role, keyboard model, status announcements, revision/provenance, compiler/renderer tests, rollback compatibility.

## Independent type systems are allowed

The registry describes a **semantic kind**. Existing `vivim.ui-surface/1`, `vivim.ui-document/1`, TierFlow inspector and spatial World each have their own closed, safe field grammar. A mapping is a proposal for compatibility, *not* a right to ignore source-specific validators. Explicitly report properties/behaviors lost in conversion.

## Typed intent distinction

- `command: set slider value` → user input/value-change candidate; executing any bound action needs Host approval and policy.
- `ui-program: add slider to settings screen` → proposed interface revision; first requires a reviewed slider adapter because PWA does not currently support it.
- `wiki: document slider capability` → source-derived knowledge/Wiki projection, no user runtime effect.

## Machine representation

The canonical JS module exports data definitions and stable lookup APIs; the accompanying JSON has all domain/family/kind rows plus 417 flattened, deterministic **component passports**. A contract is descriptive until a renderer adapter, Host policy, integration tests, accessibility and physical-device acceptance establish support.
