# Research baseline — continuous, bidirectional gestures
2026-11-21 · First external research pass + internal VIVIM mapping

## External evidence
1. **Direct manipulation / immediate feedback.** Nielsen Norman Group describes direct manipulation as acting on visible objects with immediate feedback. This supports live previews, but also warns that gestures may be less efficient than explicit commands for complex operations. VIVIM should combine direct manipulation with its semantic command surface rather than replace typing. https://www.nngroup.com/articles/direct-manipulation/
2. **Early or partial recognition.** Nielsen's historical 'Noncommand User Interfaces' discussion highlights late feedback as a gesture-recognition problem, and the potential of hybrid gesture/direct-manipulation approaches that provide feedback mid-gesture. VIVIM should show candidate meaning while the input is still happening. https://www.nngroup.com/articles/noncommand/
3. **Platform consistency, discoverability and progress.** Apple's current gesture guidance emphasizes familiar gesture behavior, meaningful feedback while the gesture is in progress, and touch-and-hold as a contextual affordance. VIVIM's novelty must not turn every press into a surprise command. https://developer.apple.com/design/human-interface-guidelines/gestures
4. **Pointer-event semantics.** W3C Pointer Events unifies mouse, touch, pen; includes down/move/up/cancel and optional pressure. Browser touch can be captured and tracked but must handle cancellation and device differences. https://www.w3.org/TR/pointerevents/
5. **Pressure is not always true pressure.** MDN notes fallback `PointerEvent.pressure=0.5` during active contact when hardware lacks pressure sensing. A long press (time) is NOT the same thing as a hard press (force). Do not infer degrees of force from generic touch events. https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pressure
6. **Do not break browser touch behavior.** CSS `touch-action` influences whether the browser owns scrolling/zooming and dispatches `pointercancel`; avoid global `touch-action:none`. Explicit zones, safe touch-action and pointer-capture behavior must be tested on the actual PWA. https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/touch-action
7. **Cancellation and equivalent operation.** WCAG 2.2 2.5.1/2.5.2/2.5.7/2.5.8 require attention to path/multipoint alternatives, pointer cancellation, drag-free alternatives and target sizes. The interactive gesture must not be the exclusive route to a function. https://www.w3.org/TR/WCAG22/
8. **Reduced motion.** Respect user OS preference via `prefers-reduced-motion`: the same progress/meaning must be perceivable with minimal animation. https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion
9. **Haptics only as enhancement.** Android guidance uses haptics for selection/feedback while respecting user settings. A PWA should not assume the same API or hardware response across devices. https://developer.android.com/develop/ui/views/haptics/haptic-feedback

## VIVIM/ARBOR evidence and mapping
- Existing product design: `vivim-labs/ARBOR-PWA-BASELINE-UPGRADE/README.md` and lanes B04 (TierFlow), B06 (programming), B07 (VIEW ALL), B08 (Host approval), B09 (product proof). These are historical planning lanes; runtime integration must be inspected before claiming complete.
- Existing system mirror: `vivim-labs/ARBOR-LIVE-SURFACE-DOCUMENT/HANDOFF-PWA.md` defines actual mounted screen observation, focus/overlay/revisions, existing Rust source paths and Host readback proof.
- Existing knowledge/learning: `OWNER-WORK/BEHAVIOR-TO-DETERMINISM.md` explicitly identifies touch/hold/movement, semantic targets, correction, privacy, safe causal episode capture and deterministic Rust proposals.
- Current ops receipt: `OWNER-WORK/NOW.md` and `OWNER-WORK/OW-20261121-014-DEPLOYMENT.md` report deployed one-composer PWA and focused tests; phone gesture acceptance remains UNRUN.
- Product authority remains Host; Wiki is a source-backed live projection, not permission to write to its underlying state.

## Synthesis / first research hypothesis
The product differentiator is **semantic direct manipulation**: the finger acts as a continuous question about the currently grounded object; VLANG/VEC supplies intelligible candidate actions; TierFlow makes consequences perceptible as a preview; Host governs execution; Wiki reflects the actual result. This is a design hypothesis, not an observed capability.

## Required next-source study
Read current GitHub ARBOR main pointer/touch handler, mounted mobile composer, Wiki item component, TierFlow interaction contract, VEC preview/approval wiring and LiveSurfaceDocument projection; build a verified source-to-target matrix. Do not assume a historical module exists or is mounted merely because a Labs design names it.

## Next research comparison
Compare three low-cost UI idioms using one Wiki card and actual phone observations: (A) inline expanding command strip, (B) radial lens, (C) anchored preview sheet. Score discoverability, thumb reach, cancellation, text-selection interference, accessibility and measured responsiveness. No implementation is authorized by this document.
