# Visual Command Language — Master Visual Inventory R001

Date: 2026-11-25. Design research and implementation candidate inventory, not proof of runtime completion.

## Authority and scope
This is the centralized design inventory under the existing ARBOR-VLANG-UPGRADE program. ARBOR/main remains runtime authority; U09 (real-time visual feedback), U08 (completion/VIEW ALL), U06/U07 (interpretation/planning), U10 (TierFlow/VEC migration), and G02–G05 (gestures) remain canonical tracking in https://github.com/owenservera/ARBOR/issues/33. Do not create a second parser, renderer, or effect authority. GPUI is the single native renderer; phone is its authenticated viewport. Existing source must be reconciled before implementation.

## Source crosswalk
- https://github.com/owenservera/ARBOR/blob/main/native/gpui-desktop/src/control.rs — inert native interpretation and review
- https://github.com/owenservera/ARBOR/blob/main/core/programmability/src/command_intent_projection.rs — typed command projection
- https://github.com/owenservera/ARBOR/blob/main/apps/mobile-relay/composer-assist.mjs — semantic slots/spans, corrections and suggestions; migrate useful behavior to native GPUI
- https://github.com/owenservera/ARBOR/blob/main/native/gpui-desktop/src/first_contact.rs — draft observation and wiki refresh
- https://github.com/owenservera/ARBOR/tree/main/.integration/proposal5/phase2/pre-g/V-visual — historical inert multi-producer visual projection
- https://github.com/owenservera/vivim-labs/tree/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/FIVE-LANE-OPERATIONS-R001/04-vlang-visual-gestures — existing engineering lane
- https://github.com/owenservera/vivim-labs/tree/main/ZCODE-DESKTOP-COMMANDER-OPERATIONS/DESIGN/AUTOWIKI-FOUNDATION-R001 — existing knowledge feedback design

## Design principles
Separate semantic role (action/entity/argument/constraint/literal/relationship) from interpretation status, capability availability, approval and execution outcome. Use redundant accessible visual cues; no style alone asserts authorization or execution. No invented confidence percentages. Preserve original text, Unicode spans, caret, IME, selection, clipboard semantics and revision identity. Wrappers are presentation, not hidden executable characters. Decorations may be inert or editable but never execute on typing/hover/Tab. User correction is reinterpreted and validated before it is eligible for scoped learning. No simulated production capability evidence. Use design experiments to compare combinations, not all primitives simultaneously.

Total: 241 distinct candidates across 12 families (some are state projections/interaction affordances rather than literal styling properties).

## 1. Typography and glyphs

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-001 | Font family | Candidate; reconcile existing code |
| VCL-002 | Font size | Candidate; reconcile existing code |
| VCL-003 | Font weight (light/normal/medium/semibold/bold) | Candidate; reconcile existing code |
| VCL-004 | Italic/upright | Candidate; reconcile existing code |
| VCL-005 | Font width (condensed/normal/expanded) | Candidate; reconcile existing code |
| VCL-006 | Variable-font axes | Candidate; reconcile existing code |
| VCL-007 | Letter spacing | Candidate; reconcile existing code |
| VCL-008 | Word spacing | Candidate; reconcile existing code |
| VCL-009 | Line height | Candidate; reconcile existing code |
| VCL-010 | Capitalization/small caps | Candidate; reconcile existing code |
| VCL-011 | Monospace for literals | Candidate; reconcile existing code |
| VCL-012 | Tabular numerals | Candidate; reconcile existing code |
| VCL-013 | Superscript/subscript | Candidate; reconcile existing code |
| VCL-014 | Ligatures | Candidate; reconcile existing code |
| VCL-015 | Text direction and bidirectional isolation | Candidate; reconcile existing code |

## 2. Text color and decoration

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-016 | Foreground hue | Candidate; reconcile existing code |
| VCL-017 | Saturation | Candidate; reconcile existing code |
| VCL-018 | Lightness | Candidate; reconcile existing code |
| VCL-019 | Opacity | Candidate; reconcile existing code |
| VCL-020 | Solid underline | Candidate; reconcile existing code |
| VCL-021 | Dotted underline | Candidate; reconcile existing code |
| VCL-022 | Dashed underline | Candidate; reconcile existing code |
| VCL-023 | Wavy underline | Candidate; reconcile existing code |
| VCL-024 | Double underline | Candidate; reconcile existing code |
| VCL-025 | Underline thickness/offset | Candidate; reconcile existing code |
| VCL-026 | Strikethrough | Candidate; reconcile existing code |
| VCL-027 | Overline | Candidate; reconcile existing code |
| VCL-028 | Text highlight | Candidate; reconcile existing code |
| VCL-029 | Text outline | Candidate; reconcile existing code |
| VCL-030 | Gradient text (experimental only) | Candidate; reconcile existing code |

## 3. Inline semantic wrappers

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-031 | Inline background fill | Candidate; reconcile existing code |
| VCL-032 | Tinted background | Candidate; reconcile existing code |
| VCL-033 | Outline-only wrapper | Candidate; reconcile existing code |
| VCL-034 | Border hue | Candidate; reconcile existing code |
| VCL-035 | Border thickness | Candidate; reconcile existing code |
| VCL-036 | Border style | Candidate; reconcile existing code |
| VCL-037 | Corner radius | Candidate; reconcile existing code |
| VCL-038 | Horizontal/vertical padding | Candidate; reconcile existing code |
| VCL-039 | Capsule/pill | Candidate; reconcile existing code |
| VCL-040 | Rectangular token | Candidate; reconcile existing code |
| VCL-041 | Bracket enclosure | Candidate; reconcile existing code |
| VCL-042 | Quotation enclosure | Candidate; reconcile existing code |
| VCL-043 | Nested wrappers | Candidate; reconcile existing code |
| VCL-044 | Left/right edge markers | Candidate; reconcile existing code |
| VCL-045 | Multi-line continuous wrapper | Candidate; reconcile existing code |
| VCL-046 | Per-line wrapped fragments | Candidate; reconcile existing code |
| VCL-047 | Collapsible token | Candidate; reconcile existing code |
| VCL-048 | Inline token label | Candidate; reconcile existing code |
| VCL-049 | Leading/trailing icon | Candidate; reconcile existing code |

## 4. Command grouping and layout

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-050 | Clause wrapper | Candidate; reconcile existing code |
| VCL-051 | Full-command bubble | Candidate; reconcile existing code |
| VCL-052 | Paragraph container | Candidate; reconcile existing code |
| VCL-053 | Nested command groups | Candidate; reconcile existing code |
| VCL-054 | Multi-command stack | Candidate; reconcile existing code |
| VCL-055 | Side-by-side groups | Candidate; reconcile existing code |
| VCL-056 | Indentation | Candidate; reconcile existing code |
| VCL-057 | Hanging indent | Candidate; reconcile existing code |
| VCL-058 | Numbered steps | Candidate; reconcile existing code |
| VCL-059 | Group headers | Candidate; reconcile existing code |
| VCL-060 | Separators | Candidate; reconcile existing code |
| VCL-061 | Vertical rails | Candidate; reconcile existing code |
| VCL-062 | Scope brackets | Candidate; reconcile existing code |
| VCL-063 | Connecting arrows | Candidate; reconcile existing code |
| VCL-064 | Branch/fork markers | Candidate; reconcile existing code |
| VCL-065 | Dependency lines | Candidate; reconcile existing code |
| VCL-066 | Conditional blocks | Candidate; reconcile existing code |
| VCL-067 | Loop/repetition marker | Candidate; reconcile existing code |
| VCL-068 | Negation scope boundary | Candidate; reconcile existing code |
| VCL-069 | Selection boundary | Candidate; reconcile existing code |

## 5. Composer canvas and surrounding surfaces

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-070 | Editor background hue | Candidate; reconcile existing code |
| VCL-071 | Background tint | Candidate; reconcile existing code |
| VCL-072 | Surface opacity | Candidate; reconcile existing code |
| VCL-073 | Editor border | Candidate; reconcile existing code |
| VCL-074 | Focus ring | Candidate; reconcile existing code |
| VCL-075 | Corner radius | Candidate; reconcile existing code |
| VCL-076 | Shadow/elevation | Candidate; reconcile existing code |
| VCL-077 | Inset shadow | Candidate; reconcile existing code |
| VCL-078 | Inner padding | Candidate; reconcile existing code |
| VCL-079 | Maximum text width | Candidate; reconcile existing code |
| VCL-080 | Wrapping behavior | Candidate; reconcile existing code |
| VCL-081 | Auto-growing editor height | Candidate; reconcile existing code |
| VCL-082 | Scroll indicators | Candidate; reconcile existing code |
| VCL-083 | Gutter | Candidate; reconcile existing code |
| VCL-084 | Active-line highlight | Candidate; reconcile existing code |
| VCL-085 | Context header | Candidate; reconcile existing code |
| VCL-086 | Interpretation strip | Candidate; reconcile existing code |
| VCL-087 | Suggestion tray | Candidate; reconcile existing code |
| VCL-088 | Toolbar | Candidate; reconcile existing code |
| VCL-089 | Footer status | Candidate; reconcile existing code |
| VCL-090 | Expandable command inspector | Candidate; reconcile existing code |
| VCL-091 | Docked AutoWiki panel | Candidate; reconcile existing code |
| VCL-092 | Responsive density | Candidate; reconcile existing code |

## 6. Icons and symbolic cues

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-093 | Action icon | Candidate; reconcile existing code |
| VCL-094 | Entity type icon | Candidate; reconcile existing code |
| VCL-095 | Application/provider icon | Candidate; reconcile existing code |
| VCL-096 | File type icon | Candidate; reconcile existing code |
| VCL-097 | Search icon | Candidate; reconcile existing code |
| VCL-098 | Location/execution target icon | Candidate; reconcile existing code |
| VCL-099 | Read/write indicator | Candidate; reconcile existing code |
| VCL-100 | Risk shield | Candidate; reconcile existing code |
| VCL-101 | Lock/unlock | Candidate; reconcile existing code |
| VCL-102 | Missing-slot question | Candidate; reconcile existing code |
| VCL-103 | Ambiguity fork | Candidate; reconcile existing code |
| VCL-104 | Checkmark | Candidate; reconcile existing code |
| VCL-105 | Warning triangle | Candidate; reconcile existing code |
| VCL-106 | Error mark | Candidate; reconcile existing code |
| VCL-107 | Clock | Candidate; reconcile existing code |
| VCL-108 | Progress spinner | Candidate; reconcile existing code |
| VCL-109 | Link/source icon | Candidate; reconcile existing code |
| VCL-110 | Offline/disconnected icon | Candidate; reconcile existing code |
| VCL-111 | Stale/revision marker | Candidate; reconcile existing code |
| VCL-112 | Undo/redo | Candidate; reconcile existing code |
| VCL-113 | Cancel/stop | Candidate; reconcile existing code |
| VCL-114 | Attachment marker | Candidate; reconcile existing code |
| VCL-115 | Voice input indicator | Candidate; reconcile existing code |
| VCL-116 | Human/agent actor marker | Candidate; reconcile existing code |

## 7. Suggestions and corrections

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-117 | Ghost text | Candidate; reconcile existing code |
| VCL-118 | Prefix completion | Candidate; reconcile existing code |
| VCL-119 | Inline completion | Candidate; reconcile existing code |
| VCL-120 | Ranked suggestion list | Candidate; reconcile existing code |
| VCL-121 | Two-TAB stages | Candidate; reconcile existing code |
| VCL-122 | VIEW ALL drawer | Candidate; reconcile existing code |
| VCL-123 | Keyboard shortcut hint | Candidate; reconcile existing code |
| VCL-124 | Autocomplete dropdown | Candidate; reconcile existing code |
| VCL-125 | Parameter chip | Candidate; reconcile existing code |
| VCL-126 | Editable slot | Candidate; reconcile existing code |
| VCL-127 | Enumerated choice menu | Candidate; reconcile existing code |
| VCL-128 | Entity picker | Candidate; reconcile existing code |
| VCL-129 | Date/time picker | Candidate; reconcile existing code |
| VCL-130 | File/folder picker | Candidate; reconcile existing code |
| VCL-131 | Scope picker | Candidate; reconcile existing code |
| VCL-132 | Missing-parameter placeholder | Candidate; reconcile existing code |
| VCL-133 | Ambiguity selector | Candidate; reconcile existing code |
| VCL-134 | Alternative interpretation | Candidate; reconcile existing code |
| VCL-135 | Tap-to-reveal tooltip | Candidate; reconcile existing code |
| VCL-136 | Correction preview | Candidate; reconcile existing code |
| VCL-137 | Accept/reject controls | Candidate; reconcile existing code |
| VCL-138 | Reparse result | Candidate; reconcile existing code |

## 8. Meaning, context and provenance

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-139 | Action label | Candidate; reconcile existing code |
| VCL-140 | Entity label | Candidate; reconcile existing code |
| VCL-141 | Argument type | Candidate; reconcile existing code |
| VCL-142 | Resolved value | Candidate; reconcile existing code |
| VCL-143 | Unresolved value | Candidate; reconcile existing code |
| VCL-144 | Constraint | Candidate; reconcile existing code |
| VCL-145 | Negation | Candidate; reconcile existing code |
| VCL-146 | Quoted literal | Candidate; reconcile existing code |
| VCL-147 | Reference/pronoun resolution | Candidate; reconcile existing code |
| VCL-148 | Active workspace | Candidate; reconcile existing code |
| VCL-149 | Selected application/account | Candidate; reconcile existing code |
| VCL-150 | Local/cloud destination | Candidate; reconcile existing code |
| VCL-151 | Capability availability | Candidate; reconcile existing code |
| VCL-152 | Context breadcrumb | Candidate; reconcile existing code |
| VCL-153 | Source anchor | Candidate; reconcile existing code |
| VCL-154 | Revision identity | Candidate; reconcile existing code |
| VCL-155 | Evidence indicator | Candidate; reconcile existing code |
| VCL-156 | Personal alias marker | Candidate; reconcile existing code |
| VCL-157 | Deterministic/AI-assisted route | Candidate; reconcile existing code |
| VCL-158 | Unknown/unsupported span | Candidate; reconcile existing code |
| VCL-159 | Conflicting evidence marker | Candidate; reconcile existing code |
| VCL-160 | AutoWiki contextual link | Candidate; reconcile existing code |

## 9. Execution and verification

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-161 | Draft state | Candidate; reconcile existing code |
| VCL-162 | Interpreting state | Candidate; reconcile existing code |
| VCL-163 | Incomplete state | Candidate; reconcile existing code |
| VCL-164 | Ambiguous state | Candidate; reconcile existing code |
| VCL-165 | Resolved state | Candidate; reconcile existing code |
| VCL-166 | Unavailable state | Candidate; reconcile existing code |
| VCL-167 | Refused state | Candidate; reconcile existing code |
| VCL-168 | Stale state | Candidate; reconcile existing code |
| VCL-169 | Plan preview | Candidate; reconcile existing code |
| VCL-170 | Affected-object count | Candidate; reconcile existing code |
| VCL-171 | Scope preview | Candidate; reconcile existing code |
| VCL-172 | Before/after diff | Candidate; reconcile existing code |
| VCL-173 | Approval required | Candidate; reconcile existing code |
| VCL-174 | Approved state | Candidate; reconcile existing code |
| VCL-175 | Queued state | Candidate; reconcile existing code |
| VCL-176 | Running state | Candidate; reconcile existing code |
| VCL-177 | Step progress | Candidate; reconcile existing code |
| VCL-178 | Partial success | Candidate; reconcile existing code |
| VCL-179 | Failed state | Candidate; reconcile existing code |
| VCL-180 | Cancelled state | Candidate; reconcile existing code |
| VCL-181 | Completed-unverified state | Candidate; reconcile existing code |
| VCL-182 | Verified receipt | Candidate; reconcile existing code |
| VCL-183 | Retry affordance | Candidate; reconcile existing code |
| VCL-184 | Recovery/undo availability | Candidate; reconcile existing code |

## 10. Motion and temporal feedback

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-185 | Caret blink | Candidate; reconcile existing code |
| VCL-186 | Focus transition | Candidate; reconcile existing code |
| VCL-187 | Semantic highlight transition | Candidate; reconcile existing code |
| VCL-188 | Wrapper growth/shrink | Candidate; reconcile existing code |
| VCL-189 | Suggestion reveal | Candidate; reconcile existing code |
| VCL-190 | Chip insertion/removal | Candidate; reconcile existing code |
| VCL-191 | Disambiguation expansion | Candidate; reconcile existing code |
| VCL-192 | Command regrouping animation | Candidate; reconcile existing code |
| VCL-193 | Progress animation | Candidate; reconcile existing code |
| VCL-194 | Pending pulse | Candidate; reconcile existing code |
| VCL-195 | Receipt confirmation animation | Candidate; reconcile existing code |
| VCL-196 | Error transition | Candidate; reconcile existing code |
| VCL-197 | Long-press progress ring | Candidate; reconcile existing code |
| VCL-198 | Stale-result fade | Candidate; reconcile existing code |
| VCL-199 | Reduced-motion alternative | Candidate; reconcile existing code |

## 11. Interaction, gestures and sensory feedback

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-200 | Caret positioning | Candidate; reconcile existing code |
| VCL-201 | Text selection | Candidate; reconcile existing code |
| VCL-202 | Click/tap semantic span | Candidate; reconcile existing code |
| VCL-203 | Hover inspection | Candidate; reconcile existing code |
| VCL-204 | Long press | Candidate; reconcile existing code |
| VCL-205 | Double tap | Candidate; reconcile existing code |
| VCL-206 | Keyboard navigation | Candidate; reconcile existing code |
| VCL-207 | Escape dismiss | Candidate; reconcile existing code |
| VCL-208 | Tab completion | Candidate; reconcile existing code |
| VCL-209 | Enter submission | Candidate; reconcile existing code |
| VCL-210 | Drag selection | Candidate; reconcile existing code |
| VCL-211 | Drag reorder (plan only) | Candidate; reconcile existing code |
| VCL-212 | Swipe dismiss | Candidate; reconcile existing code |
| VCL-213 | Pinch/zoom inspector | Candidate; reconcile existing code |
| VCL-214 | Copy semantic value | Candidate; reconcile existing code |
| VCL-215 | Undo correction | Candidate; reconcile existing code |
| VCL-216 | Touch target enlargement | Candidate; reconcile existing code |
| VCL-217 | Haptic confirmation | Candidate; reconcile existing code |
| VCL-218 | Haptic warning | Candidate; reconcile existing code |
| VCL-219 | Optional sound feedback | Candidate; reconcile existing code |
| VCL-220 | Screen reader announcement | Candidate; reconcile existing code |
| VCL-221 | IME composition preservation | Candidate; reconcile existing code |

## 12. Accessibility, personalization and presentation modes

| ID | Primitive / capability | Status |
|---|---|---|
| VCL-222 | Quiet/assisted/inspect densities | Candidate; reconcile existing code |
| VCL-223 | Light/dark themes | Candidate; reconcile existing code |
| VCL-224 | High contrast | Candidate; reconcile existing code |
| VCL-225 | Color-vision-safe palettes | Candidate; reconcile existing code |
| VCL-226 | Non-color redundant cues | Candidate; reconcile existing code |
| VCL-227 | User font size | Candidate; reconcile existing code |
| VCL-228 | User font weight | Candidate; reconcile existing code |
| VCL-229 | Zoom/text scaling | Candidate; reconcile existing code |
| VCL-230 | Reduced motion | Candidate; reconcile existing code |
| VCL-231 | Accessible names | Candidate; reconcile existing code |
| VCL-232 | Logical focus order | Candidate; reconcile existing code |
| VCL-233 | Screen reader semantic span descriptions | Candidate; reconcile existing code |
| VCL-234 | Keyboard-only interaction | Candidate; reconcile existing code |
| VCL-235 | Touch-only interaction | Candidate; reconcile existing code |
| VCL-236 | RTL/multilingual layout | Candidate; reconcile existing code |
| VCL-237 | User-customizable decoration intensity | Candidate; reconcile existing code |
| VCL-238 | Explicit learning opt-in | Candidate; reconcile existing code |
| VCL-239 | Personal alias correction | Candidate; reconcile existing code |
| VCL-240 | Reset learned mappings | Candidate; reconcile existing code |
| VCL-241 | Developer raw IR/provenance inspector | Candidate; reconcile existing code |

## Implementation order
P0: live revision-fenced spans, basic typography/color/underline, non-color status, inline and full-command wrapper, interpretation strip, missing/ambiguous/unavailable states, editable arguments, exact Host receipt and keyboard/touch/accessibility. P1: nested clause groups, sequencing/negation, context anchors, AutoWiki, suggestions/2-TAB/VIEW ALL, scope/diff and plan progress. P2: motion, user density/theme, haptics and learning controls. All priority assignments are proposals pending native audit.

## Verification
Every candidate needs a source path, semantic input contract, renderer mapping, interaction rule, accessibility alternative, current-source test and actual GPUI/phone evidence where applicable. Current inventory is not a complete code census. No visual mockup is evidence of a working capability.
