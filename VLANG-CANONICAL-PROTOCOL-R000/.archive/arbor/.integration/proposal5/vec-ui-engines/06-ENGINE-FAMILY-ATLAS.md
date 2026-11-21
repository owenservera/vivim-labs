# Master engine-family atlas

37 independent conceptual engine families and 461 typed named capabilities.
These capabilities are DESCRIBED_NOT_INSTALLED until owner implementations, renderer adapters and Host/device gates pass.

| Engine ID | Typed capabilities | Representative options | Implementation state |
|---|---:|---|---|
| vivim.ui-engine.color@1 | 17 | background, foreground, accent, surface, text | SPECIFIED |
| vivim.ui-engine.paint@1 | 14 | solid, gradient, image, pattern, texture | SPECIFIED |
| vivim.ui-engine.border@1 | 13 | width, style, color, top, right | SPECIFIED |
| vivim.ui-engine.corners@1 | 11 | radius, top-left, top-right, bottom-left, bottom-right | SPECIFIED |
| vivim.ui-engine.shadow@1 | 11 | box, text, drop, ambient, inset | SPECIFIED |
| vivim.ui-engine.typography@1 | 13 | family, weight, size, line-height, letter-spacing | SPECIFIED |
| vivim.ui-engine.text@1 | 13 | color, align, transform, decoration, overflow | SPECIFIED |
| vivim.ui-engine.spacing@1 | 12 | padding, margin, gap, row-gap, column-gap | SPECIFIED |
| vivim.ui-engine.sizing@1 | 13 | width, height, min-width, max-width, min-height | SPECIFIED |
| vivim.ui-engine.layout@1 | 14 | display, flex-direction, flex-wrap, justify, align | SPECIFIED |
| vivim.ui-engine.position@1 | 13 | position, top, right, bottom, left | SPECIFIED |
| vivim.ui-engine.responsive@1 | 12 | breakpoints, container-queries, viewport-class, phone, tablet | SPECIFIED |
| vivim.ui-engine.scroll@1 | 13 | axis, overflow, snap, snap-align, momentum | SPECIFIED |
| vivim.ui-engine.transforms@1 | 11 | translate, scale, rotate, skew, perspective | SPECIFIED |
| vivim.ui-engine.effects@1 | 12 | opacity, blur, brightness, contrast, saturate | SPECIFIED |
| vivim.ui-engine.motion@1 | 13 | duration, delay, easing, transition, keyframes | SPECIFIED |
| vivim.ui-engine.assets@1 | 13 | image, icon, illustration, font, sprite | SPECIFIED |
| vivim.ui-engine.media@1 | 15 | autoplay, loop, mute, volume, speed | SPECIFIED |
| vivim.ui-engine.charts@1 | 13 | type, series, axes, legend, tooltip | SPECIFIED |
| vivim.ui-engine.data@1 | 15 | source, projection, query, filter, sort | SPECIFIED |
| vivim.ui-engine.navigation@1 | 13 | route, href, target, tabs, selection | SPECIFIED |
| vivim.ui-engine.input@1 | 14 | type, min, max, step, value | SPECIFIED |
| vivim.ui-engine.gestures@1 | 12 | tap, long-press, swipe, drag, drop | SPECIFIED |
| vivim.ui-engine.focus@1 | 11 | order, trap, restore, autofocus, ring | SPECIFIED |
| vivim.ui-engine.accessibility@1 | 14 | role, name, description, contrast-target, text-scale | SPECIFIED |
| vivim.ui-engine.states@1 | 14 | loading, empty, error, success, pending | SPECIFIED |
| vivim.ui-engine.forms@1 | 14 | schema, constraints, validation, grouping, required | SPECIFIED |
| vivim.ui-engine.localization@1 | 12 | locale, region, direction, timezone, calendar | SPECIFIED |
| vivim.ui-engine.spatial@1 | 13 | position, rotation, scale, world-anchor, camera | SPECIFIED |
| vivim.ui-engine.theme@1 | 11 | theme, mode, palette, semantic-tokens, component-variants | SPECIFIED |
| vivim.ui-engine.system@1 | 12 | device, connectivity, power, performance, safe-areas | SPECIFIED |
| vivim.ui-engine.workflow@1 | 11 | trigger, condition, data-source, transform, approval | SPECIFIED |
| vivim.ui-engine.security@1 | 11 | scope, visibility, data-class, redaction, access | SPECIFIED |
| vivim.ui-engine.history@1 | 11 | revision, base-revision, undo, redo, rollback | SPECIFIED |
| vivim.ui-engine.templates@1 | 9 | template, variant, slots, tokens, dependencies | SPECIFIED |
| vivim.ui-engine.composition@1 | 9 | priority, layers, conditions, scopes, conflict-policy | SPECIFIED |
| vivim.ui-engine.diagnostics@1 | 9 | trace, metrics, failure, provenance, engine-health | SPECIFIED |

Each engine has a unique domain owner and an agent under agents/A017...A053; the engine family should remain
one canonical source of computation. Theme is a composition engine and must not reimplement Color or Typography.
Every capability has an exact stable major-version ID, value type, no runtime rights and default empty renderer bindings.

Machine-readable full registry: machine/engine-catalog.v1.json.
Programmatic queries: getEngineFamily, getEngineCapability, listEngineFamilies, listEngineCapabilities.
A caller can discover whether a semantic property exists and is conceptually applicable, but cannot infer current device support.
