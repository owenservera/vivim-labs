# Complete minimal programmable PWA — design
## Meaning of minimal
Minimal default screen, maximal reachable real functionality. Do not show every component type simultaneously. No synthetic dashboard, fake progress, bogus success or generic empty canvas as replacement for existing product. Start with a curated working configuration; reveal depth on demand. All current visible controls become real registered instances. The entire catalog remains discoverable with truthful availability.

## Default proposed arrangement
- Neutral graphite/light equivalent, one restrained accent; readable contrast; existing fonts and assets; consistent spacing, radius and hierarchy. A design preference, not a fixed hardcoded theme.
- Header: compact VIVIM brand, workspace title, connection/authority status, navigation/menu and settings. Keep pairing and recovery accessible.
- Content: Today (live connection and needs-you, activity, shortcuts, capture); Talk (default conversation entry, real messages/provider/Host context, conversation drawer); Wiki (actual pages/sections and knowledge sources); Build (current visual canvas, preview, activation, restoration). World, Me and Control reachable via All, optionally promoted. Retain seven real destinations; existing first-visit Talk behavior until product explicitly changes it.
- Shared composer: natural-language draft, context-aware semantic/typed hints, attachments, microphone, target/model/effort, send; preserve input state, keyboard/IME, click and touch, no duplicate dispatch. Default collapsed options where safe.
- Bottom or responsive navigation: Today, Talk, Wiki, Build, All. All exposes World/Me/Control, settings, Pocket and live components. Presentation, order, grouping configurable; destinations stable.
- VIEW ALL: overlay or side panel with selection mode, current tree, component search, installed engine controls, data/action bindings, history. No permanent debug clutter or artificial tasks. Desktop uses same responsive document with side rail or expanded inspector; no different logic.

## Existing visible product inventory
| Area | Real controls and content that must survive |
|---|---|
| Shell | header/logo, tablist/tab/tabpanel, connection badge, menu, theme panel, settings, install, pairing/recovery |
| Composer | text field, semantic suggestions, target/model/effort, send, attachments/file input, voice, context, focus, recent conversation and provider refresh |
| Talk | conversations, actual messages, reply, status, target selectors, native sessions, typed preview and approval/receipt cards |
| Today | connection, shortcuts, live needs-you and activity, launcher, quick capture |
| Build | composer, live canvas, typed candidate, actual visual preview, activate, undo, uncertain/readback states |
| Wiki | pages, section text, canonical revisions, semantic context, provenance and source links |
| World | real projects/collections, built surface, activity, media list/player/pagination |
| Me | identity, paired phones, permissions, memory, installation, readiness/preferences |
| Control | typed action input, preview, decisions and actual results, recovery |
| Pocket | real local notes and offline voice/quick capture |
| System | pairing, dialogs, error/status, permission/revocation and recovery |

A registered instance is not a new implementation: preserve its existing source handler, data source, accessibility and authorization. Missing data = truthful empty/unavailable state. Do not copy data from fixtures.

## Programmable component model
Stable instance ID (e.g. pwa.composer.input, pwa.shell.nav.wiki; illustrative only), existing taxonomy ID and native/source identity, current properties, parent/order, applicable capabilities, installed adapters, typed data and action refs, visibility, scope, revision. Dynamic content uses existing stable domain IDs; repeated instances must be distinct; privacy-sensitive labels must not become global identifiers. User-generated views are bounded reusable documents, not one unbounded tree. Initial default cannot create new user data. User can change labels where safe, colors, text sizes, spacing, layout, visibility of optional controls, supported component properties and arrangement. Cannot alter action semantics, permission checks, canonical identity, required accessibility/approval/recovery paths.

## Engine family coverage / priority
P0 appearance: color, paint, border, corners, shadow, typography, text, theme.
P0 geometry: spacing, sizing, layout, position, responsive, scroll.
P0 usability: navigation, input, focus, gestures, accessibility, states.
P0 authority and history: security, history, system (constraints, not free styling).
P1 rich presentation: transforms, effects, motion, assets, media, charts, data.
P1 interaction: forms, localization, workflow.
P1 reuse: templates, composition, diagnostics.
P2 spatial: only if real existing source/renderer supported, otherwise truthful unsupported.
All 37 families represented in searchable library. Installed status per component/capability/runtime, not per family by blanket claim. Prefer existing tokens and adapter mappings. One visual setting may configure many components; scope opt-in, contrast and reduced motion mandatory.

## Three authoring modes, one typed meaning
NL via VLANG/NLCL/VEC; direct controls/selection (tap, keyboard, drag if source closed); typed settings. All create the same bounded change candidate. Preview affected real components and before/after properties, revision, scope and exact identity. No effect during preview. Explicit Activate/approve uses incumbent Host publisher; authenticate and validate again; only show success after persisted expected readback and real repaint. Undo/restoration is a validated later revision or existing safe rollback proof; do not weaken guards. VIEW ALL and power-user interaction consume current typed meaning, not new parser. Existing provider conversation remains separate from local UI compilation. Known edits do not need an LLM. 
Example intents: 'Make all cards flat with square corners'; 'Move Wiki first'; 'Make the chat box translucent'; 'Show my projects as a timeline'; 'Put the microphone on the left'; 'Restore yesterday's layout'. Each must be supported only once real applicable adapters/data/authority are wired; otherwise identify limits or clarify. 
## Architecture
Existing shared NLCL/VLANG/VEC -> canonical component identities + UI engine registry -> TierFlow document/render computation -> exact preview -> explicit approval -> existing Host UI revision persistence -> authenticated projection -> actual PWA and desktop readback. Rust used through existing real native interfaces only; don't invent a second native visual authority.
## Definition of success
First-launch working baseline, all actual visible components registered, library complete and truthful, no parallel render authority, safe programmable installed settings, all existing functionality retained. Shared appearance persists across app surfaces, owner controls view structure safely, approval cannot be removed, real persisted reload and verified undo. Design choices are proposals pending owner adjustment, not current runtime claims.

## R002 — source advance (7c1b01630a80)
Register actual Host-bound meaning panel, exact args, optional verified sentence, semantic/catalog/target revision and receipt data. Retain desktop-only teach/revoke UI; never expose private rules on phone. Visual approval should obtain equivalent exact evidence via existing primitives; never claim sentence verification unless compiler round-trips. Ordinary selection/inspect is inert; direct mutations go through same preview/approval as NL. No generic visual learning feature in this round.
