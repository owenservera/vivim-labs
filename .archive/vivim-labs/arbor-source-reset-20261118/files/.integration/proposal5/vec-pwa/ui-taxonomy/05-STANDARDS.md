# External standard vocabulary — build on established patterns

We are classifying **existing UI concepts**, not inventing a proprietary vocabulary for buttons, sliders or tabs. These references inform semantics and tests; this catalog does **not** claim compliance or adoption by current ARBOR renderers.

- **WHATWG HTML Living Standard:** [content model and interactive elements](https://html.spec.whatwg.org/multipage/), [content categories](https://html.spec.whatwg.org/dev/dom.html). Prefer native controls and semantics where suitable.
- **W3C WAI-ARIA Authoring Practices Guide:** [pattern index](https://www.w3.org/WAI/ARIA/apg/patterns/) and [keyboard/accessible name practices](https://www.w3.org/WAI/ARIA/apg/practices/). Role metadata alone is never sufficient; implement interaction, focus order, semantics and tested assistive behavior.
- **Open UI:** [component names, parts and states research](https://github.com/openui/open-ui). Use it as a vocabulary reference; don't assume every suggested component is standardized HTML.
- **Design Tokens Community Group 2025.10:** [stable format specification](https://www.w3.org/community/reports/design-tokens/CG-FINAL-format-20251028/). Keep design tokens separate from structural component identity and user data, with existing VIVIM theme renderer owning published values.

Recommended future separate registers: component semantic IDs, accessibility patterns and keyboard behavior, typed state/event interfaces, design-token IDs, platform adapters, templates/recipes and explicit Host effect permissions. **Do not collapse these into a single untyped property bag.**

Verification source consulted November 2026; update links and tests during implementation acceptance.
