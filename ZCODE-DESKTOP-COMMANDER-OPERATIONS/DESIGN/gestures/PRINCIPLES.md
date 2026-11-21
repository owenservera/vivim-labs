# Principles — gestures as an interactive command space
R001 · Concept baseline, not an implementation specification

## Core proposition
The interface should be a responsive partner during an unfinished gesture. A finger touching an object is not an unconditional command: it establishes a situated dialogue between user, target, state, available actions and visible feedback. Speech/typing can refine the same draft intent. A preview is not an effect.

## Twelve design principles
1. **Continuous, not binary:** visible feedback begins on contact and evolves during hold, direction, speed, proximity, focus, language and target changes. Do not force the user to finish the gesture to discover its meaning.
2. **Meaning is grounded:** every candidate is tied to a stable semantic target, its role, allowed operations and observed revision. Coordinates alone are insufficient.
3. **Exploration is safe:** no side-effectful Host action during touch/hover/hold/previews. Reserve effectful operations for an explicit commit path with existing permissions, approvals and receipts.
4. **Reversible by default:** movement can change a candidate; moving out, pressing Escape, system pointer cancellation, backgrounding and network loss can abort safely.
5. **Responsive without pretending success:** the UI responds locally to interaction immediately; asynchronous context from Wiki/VLANG/VEC is labeled loading/stale/error and never shown as a verified effect.
6. **Feedback uses more than animation:** visible state, explanatory text, selection outline, optional accessible sound/haptic and semantic inspection. Never depend solely on vibration or color.
7. **Multiple input modalities, one meaning:** touch, mouse, keyboard, stylus, accessibility action and voice should converge on the same typed candidate and Host authorization—not independent implementations.
8. **Contextual, not surprising:** long press reveals commands for the touched object; familiar tap/scroll behavior stays intact; do not steal OS/browser gestures or text selection without an explicit interaction region.
9. **Adaptive but inspectable:** available options may shift with permissions, selection, context and conversation; explain why, and preserve stable targets while the user acts.
10. **Revision aware:** if Wiki item identity or source revision changes mid-gesture, refresh and require reconsideration; never commit against an obsolete target.
11. **Privacy and learning discipline:** consent-scoped interaction episodes can inform deterministic capability design; no always-on raw gesture/camera/transcript capture, no learning from a cancellation as success.
12. **One system:** Rust-first semantic state machine and existing VLANG/VEC/LiveSystemState/TierFlow/Host; tiny TS7+ browser input binding only. No new runtime authority, graph, journal or command router.

## Interactive hold — proposed reference behavior
| Phase | User action | Interface response | Authority |
|---|---|---|---|
| Contact | Touch a Wiki card | Identify card and highlight its semantic bounds | Local projection |
| Dwell | Continue holding | Reveal contextual affordances and their meanings | Read-only candidate |
| Explore | Drag, pause, or speak/enter a qualifier | Selection and explanatory preview update continuously | Inert candidate |
| Inspect | Hold on a candidate | Show exactly which content, command and permissions would be involved | VLANG/VEC read-only plan |
| Cancel | Move away, press Escape, pointercancel, background or stale target | Dismiss preview; persisted state unchanged | None |
| Commit | Choose explicit Apply / approve action | Existing Host plans, authorizes, executes, records receipt | Host only |
| Reflect | Host result arrives | Live Wiki/UI reflects exact source revision and error/success | Observed receipt |

**Important:** Simply releasing a long press must not trigger consequential mutation. A deliberate tap-on-Apply or equivalent accessible explicit action is required for a real effect. Exact release-to-choose behavior for harmless navigation can be evaluated separately.

## The bidirectional interaction loop
User input -> stable target + evolving intent -> contextual semantic candidates -> progressive visual preview -> user correction (touch/voice/text) -> new preview -> explicit approve/cancel -> Host receipt -> source-backed UI readback. The UI itself may expose available commands; commands may modify the proposed UI; neither has execution authority until Host admission.

## Runtime connections to inspect, not recreate
- **PWA first:** existing one-text-box + Wiki surface and stable link; start with ONE real Wiki item, not another screen.
- **TierFlow:** candidate visualization, live layout and touch affordances; verify exact existing render/interaction APIs on current GitHub main before proposing hooks.
- **VLANG/VEC:** semantic target interpretation, contextual plan/proposed visual structure, VIEW ALL mode. Show candidate and uncertainty without executing.
- **LiveSystemState / LiveSurfaceDocument:** observed inputs, focus, overlay, candidate/revision projection, renderer reconciliation; respect separate source revision/publication sequence.
- **Host:** sole side-effect/grant/approval/receipt authority; independent UI preview never bypasses it.
- **Activity/behavior compiler:** opt-in causal episode and correction tracking; no permanent storage of raw pointer coordinates by default.

## First acceptance experiment
On a real phone PWA: hold a Wiki card; observe visual affordances during the hold; slide between two choices and see the preview change; cancel and verify no mutation; repeat and explicitly approve one low-risk existing Wiki command; verify Host receipt and Wiki revision; repeat with no-pressure device, scrolling competition, pointercancel, stale revision, keyboard alternative, reduced motion and offline/reconnect. Record screenshots or a user-authorized trace and measured latency only when actually tested.

## Open research decisions
- Whether hold opens a radial fan, inline morph, command strip or small lens; user feedback and PWA visibility should decide, not novelty.
- What semantic data is ready without network delay, and when a candidate becomes reliable enough to reveal.
- Collision handling between native text selection/context menus, page scrolling and custom gestures.
- What opt-in data to retain from touch and voice, for how long, and how cancellations are represented.
- Whether device/browser exposes calibrated pressure; fallback always remains duration-and-motion-based.
- Definition of minimum smoothness, feedback latency, accessibility parity and battery cost based on measured real-device tests.
