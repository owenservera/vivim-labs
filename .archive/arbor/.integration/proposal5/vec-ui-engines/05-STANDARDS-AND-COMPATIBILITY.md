# External standards and source adoption

Use established standards and open component research as contract vocabulary; don't invent proprietary analogues.

1. DTCG Design Tokens Community Group Format 2025.10: https://www.designtokens.org/TR/2025.10/format/
   Typed values, groups, aliases, composites, color spaces, cross-tool formats. It is a Community Group Final Report, NOT a W3C Recommendation.
   The current token resolver supports a guarded SUBSET: explicit types, nested groups, alias traversal, type/cycle refusal.
   Full DTCG semantics, modes, composites and export are dedicated workers; not claimed supported today.
2. WAI-ARIA APG patterns: https://www.w3.org/WAI/ARIA/apg/patterns/
   Focus, role, keyboard interaction, range and slider behavior; ARIA metadata alone is not accessibility.
3. CSS Color Level 4: https://www.w3.org/TR/css-color-4/
   Structured color-space representations; no arbitrary CSS text is permitted as an engine value.
4. CSS Typed OM: https://www.w3.org/TR/css-typed-om-1/
   Typed DOM style concepts; never assume browser feature presence without adapter support.
5. Open UI: https://open-ui.org/
   Existing UI component patterns, semantics, states, anatomy and consistent vocabulary.

## Existing ARBOR donors (NOT design limits)
- apps/mobile-relay/visual-theme.mjs — deterministic palettes, contrast/first-paint, reduced motion.
- apps/mobile-relay/bootstrap-f02.mjs — bounded theme parse/render plan and font stacks.
- apps/mobile-relay/generative-ui/catalog.mjs — ten closed types, fixed PWA theme, source-ref guard.
- packages/tierflow/src/visual-runtime/components/layout.ts — pure bounded 2D layout checks.
- packages/tierflow/src/visual-runtime/document/schema.ts — closed Visual/1 document and a11y contracts.
- apps/mobile-relay/vec/ui-taxonomy/registry.mjs — 417 declarative component identities; most lack renderer bindings.

Any useful algorithm must be independently compared, adopted with typed versioned interfaces, and retired only after parity.
Do not constrain the new engine catalog to the narrow original background/spacing/rounded-corner token list.
