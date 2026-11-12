# Legacy UI + command work recovery map

Status: source-inspected on 2026-11-11, not claimed device-tested or merged. Use live repository SHAs during integration, rather than assuming historical text is current.

| Source | Exact file | Recovery intent |
|---|---|---|
| VIVIM Omega | `owenservera/vivim-omega/plugins/vivim-nlcl-pure/src/symbols.ts` | Original 17 command symbol families: `/ @ # ! ? + - & $ * % ∆ ✓ = ^ ~ →`. These are *language concepts*, not permission overrides. |
| VIVIM Omega | `plugins/vivim-nlcl-pure/src/project.ts` | Recover token notes, role/family annotations and effect preview semantics. |
| VIVIM Omega | `plugins/vivim-nlcl-pure/src/ground.ts`, `grammar.ts`, `interpret.ts`, `types.ts` | Entity grounding, slots, canonical symbolic reading, staged deterministic interpretation, suggestions and ambiguity. |
| VIVIM Omega | `docs/NCLL-AND-SELF-KNOWLEDGE.md` | Owner directive: immediate symbolic response while typing, longform rereading, `/send @this → @peter` and contact alternatives. |
| VIVIM Final | `frontend/src/components/canvas/UnifiedEntry.tsx` | One persistent chat/composer, layer-aware dispatch and autosizing text. |
| VIVIM Final | `frontend/src/components/canvas/CommandPalette.tsx` | Search, keyboard navigation, previews and categorized results. |
| VIVIM Final | `src/engines/nlcl/{nlcl-engine.ts,catalog.ts,command-registry.ts,fuzzy-resolver.ts,layered-resolver.ts}` | Earlier extensive NLU and consumer category inventory: historical candidates, **not** production dispatch. |
| Current Harness | `apps/mobile-relay/control-language.mjs`, `apps/vivim-host/control/{intent-planner.mjs,command-intent.mjs,action-catalog.v1.json}` | Canonical current deterministic interpret-only path and versioned Host action universe. |
| Current HQ | `hq/lib/{action-registry.mjs,action-router.mjs}`; `hq/live-graph/wave1/lane-c/{registry-adapter.mjs,commands.mjs}` | Live source-backed discovery; distinct statuses `DECLARED_ONLY`, `LIBRARY_ONLY`, `DISPATCH_SOURCE_PRESENT`, etc. |
| TierFlow | `src/ui/{types.ts,commands.ts}` and prior `VIVIM-VISUAL-RUNTIME/WAVE-1/LANE-G-HANDOFF.md` | Future publication/preview adapter, only after UI command authorization and revision-guard acceptance. |
| Labs historical | `Architecture/10-THREE-WAVE-RUST-LIVE-GRAPH/COMPILER-LANGUAGE-AND-PLACEMENT-REVIEW.md`; `documentation-consolidation/.../GUI02-...` | Existing architecture and audit notes; never a separate parser authority. |

The uploaded screenshot is the product-visual reference for icon-first command chips and contextual menus. A precise original screenshot-to-component match is **not yet established**. VIVIM Final's `UnifiedEntry` and Omega's visual projection are relevant source material, but not conclusive identification of the pictured screen.

Original source links:
- https://github.com/owenservera/vivim-omega/blob/main/plugins/vivim-nlcl-pure/src/project.ts
- https://github.com/owenservera/vivim-omega/blob/main/docs/NCLL-AND-SELF-KNOWLEDGE.md
- https://github.com/owenservera/vivim-final/blob/master/frontend/src/components/canvas/UnifiedEntry.tsx
- https://github.com/owenservera/vivim-final/blob/master/frontend/src/components/canvas/CommandPalette.tsx