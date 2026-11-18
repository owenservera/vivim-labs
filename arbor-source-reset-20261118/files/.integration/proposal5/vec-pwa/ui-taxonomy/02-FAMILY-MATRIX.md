# Standard UI families — 34 families / 417 kinds

The broad component families are purpose-based, not bespoke VIVIM widgets. Every kind is represented by a versioned semantic ID with inherited trait and property contracts.

| Domain | Family | Kinds | Examples | Source mapping rows |
|---|---|---:|---|---:|
| foundation | layout | 18 | stack, grid, flex, row | 6 |
| foundation | surface | 16 | card, panel, section, accordion | 3 |
| foundation | typography | 17 | text, heading, paragraph, label | 6 |
| foundation | decoration | 12 | divider, separator, icon, avatar | 1 |
| foundation | responsive | 8 | breakpoint, visibility-gate, responsive-switch, safe-area-container | 0 |
| navigation | global | 16 | nav-bar, bottom-nav, top-nav, side-nav | 0 |
| navigation | content | 12 | pagination, stepper, tree-nav, tree-item | 0 |
| input | actions | 12 | button, icon-button, toggle-button, split-button | 3 |
| input | text-entry | 14 | text-field, textarea, search-field, password-field | 1 |
| input | numeric | 14 | slider, range-slider, dial, knob | 0 |
| input | choice | 15 | checkbox, checkbox-group, radio, radio-group | 0 |
| input | date-time | 10 | date-picker, time-picker, datetime-picker, date-range-picker | 0 |
| input | files | 10 | file-picker, file-dropzone, attachment-picker, folder-picker | 0 |
| input | gestures | 10 | swipe-zone, drag-handle, drop-target, reorder-handle | 0 |
| feedback | status | 15 | status, toast, banner, alert | 1 |
| feedback | progress | 10 | progress-bar, progress-ring, spinner-indicator, loading-indicator | 0 |
| feedback | overlays | 12 | modal, dialog, alert-dialog, popover | 0 |
| feedback | validation | 9 | field-error, field-hint, validation-summary, required-indicator | 0 |
| data | collections | 14 | collection, list, list-item, ordered-list | 2 |
| data | tables | 11 | table, data-grid, pivot-table, property-grid | 0 |
| data | charts | 19 | chart, line-chart, bar-chart, area-chart | 2 |
| data | graphs | 10 | node-graph, dependency-graph, knowledge-graph, force-graph | 0 |
| data | temporal | 10 | timeline, event-timeline, calendar-view, agenda-view | 2 |
| media | players | 11 | audio-player, video-player, media-player, livestream-view | 0 |
| media | assets | 12 | image-view, image-gallery, avatar-group, thumbnail | 0 |
| application | composites | 12 | profile-card, project-card, task-card, conversation-card | 4 |
| application | workspaces | 13 | workspace, dashboard, home-screen, project-board | 0 |
| application | workflows | 14 | workflow-canvas, workflow-step, trigger-node, condition-node | 0 |
| application | editing | 11 | property-inspector, layers-panel, component-tree, visual-canvas | 0 |
| application | collaboration | 9 | comment-thread, comment-editor, presence-list, collaborator-avatar | 0 |
| application | system | 12 | pairing-status, device-status, network-indicator, storage-meter | 0 |
| spatial | primitives | 8 | shape-box, shape-sphere, shape-cylinder, shape-plane | 6 |
| spatial | interactions | 9 | spatial-canvas, orbit-control, spatial-pointer, world-layer | 0 |
| accessibility | assistive | 12 | skip-link, focus-ring, focus-trap, live-announcer | 0 |

Domain totals:

| Domain | Component kinds |
|---|---:|
| foundation | 71 |
| navigation | 28 |
| input | 85 |
| feedback | 46 |
| data | 64 |
| media | 23 |
| application | 71 |
| spatial | 17 |
| accessibility | 12 |

**Source mapping rows are historical/inspected code support declarations, not installed widget count.** Many future kinds, including sliders, color pickers, complex tables, rich editors, audio/video players, Kanban boards and spatial controls, currently have no paired-PWA renderer. They are intentionally catalogued for staged implementation.

## Why traits matter

A slider, dial and range-slider share `input.numeric`, `bindable`, `numeric` and `keyboard-operable` but have different states/value shapes. A button and workflow approval node both emit **intent proposals**, not direct effects. A chart and table can both read typed data but require different accessible alternative representations. A World sphere may be mapped as a spatial primitive without becoming a PWA slider or an executable 3D renderer.

The catalog is intentionally extensible, but consumers must use exact IDs and installed adapter mappings rather than guess equivalent components from matching labels.
