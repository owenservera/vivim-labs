# VIVIM Mobile-First Interaction Lab — Gesture and Native-Capability Opportunities

**Created:** 2026-10-30  
**Status:** Separate UX research track / proposed interaction contract, not a build authorization.  
**Scope:** Phone PWA first, potential Android wrapper later. Covers Talk, Bootstrap, Agents, World/Self and Share.  
**Separation:** Independent of `docs/ip/VIVIM-PLATFORM-IP-SOURCE-REGISTRY.md`, the October cross-repo harvest, and the authoritative `docs/roadmap.json`.

## North star

VIVIM should feel like **a personal app that the user builds with their hands and their words**, not a desktop administration console squeezed onto a phone.

- Start with a blank canvas, a composer and context-aware proposed actions.
- Touch directly manipulates the user's own objects, modes, agents and timelines.
- The user can always see **what an action will do**, **which host/provider it affects**, **whether it needs approval**, and **what can be undone**.
- Tap is sufficient for every task. Gestures are optional accelerators, not secret essential commands.
- Treat the system/browser's back gesture, text selection, scrolling and accessibility actions as first-class constraints.

## Era A — Interaction vocabulary and candidate actions

| Gesture / affordance | Context | Proposed behavior | Priority | Guardrail |
|---|---|---|---|---|
| **Tap** | Everywhere | Open object, choose proposal, enter mode | P0 | Visible default for every action |
| **Long-press** | Chat bubble | Context sheet: copy, select, save to World, cite source, share preview, branch from here | P0 | Context actions also on overflow menu; never sends automatically |
| **Long-press** | Bootstrap capability card | Preview: required provider, permissions, local changes, reversibility, evidence | P0 | Activation needs distinct explicit tap/approval |
| **Long-press** | World object | Preview, multi-select, rename, move, pin, history, private Share draft | P1 | Sensitive items marked; public publish separate |
| **Long-press** | Agent card | Inspect current task, provenance, permissions, pause controls | P1 | No execution/approval merely from holding |
| **Swipe horizontal on chat row** | Conversation list | Reveal archive/pin/mark for later actions | P1 | Do not auto-delete, preserve scrolling/back |
| **Swipe horizontal on proposal card** | Bootstrap | Explicitly reveal Later / Inspect options | P1 | No swipe-to-approve or swipe-to-run |
| **Swipe between internal views** | Full-screen canvas only | Change user-owned modes, when gesture does not conflict with OS back | P2 | Visible tab/mode switcher always works |
| **Drag object** | Bootstrap canvas | Reorder feature blocks or group them into Spaces | P1 | Dedicated drag handle, auto-scroll, cancel/undo |
| **Drag conversation or memory chip** | World or composer | Attach selected local context to draft | P1 | Preview what will be sent; no silent provider transfer |
| **Pinch-to-zoom and pan** | World map/canvas | Zoom the spatial arrangement of Spaces and agents | P2 | Preserve normal page zoom outside canvas |
| **Two-finger gesture** | Timeline | Optional explore/scrub through configuration versions | P2 | Slider/timeline buttons remain primary |
| **Press-hold + slide** | Context menu | Optional quick radial or linear action selection | P2 | Prototype only after standard sheets work |
| **Pull-to-refresh** | Read-only statuses | Recheck host / relay / provider readiness | P1 | Do not intercept OS/browser refresh unless needed; never rerun commands |
| **Bottom sheet drag** | Proposals, model choice, permissions | Peek / half / full / dismiss | P0 | Unsaved draft and pending approvals protected |
| **Edge gesture** | Navigation | Prefer platform/browser back; use visible navigation | P0 | No custom conflicting edge swipe |
| **Double tap** | Canvas cards | Optional zoom into object | P2 | Not needed for essential operations |

### Touch feedback

- Distinct pressed, selected, busy, queued, offline, blocked, failed and completed states.
- Use subtle animation and optional vibration where supported, but never require haptics; web vibration support is uneven and user/device preferences win.
- Immediate feedback on touch, optimistic **local-only** UI updates with eventual authoritative state from host. Never claim a provider send succeeded before the harness journal confirms.
- Provide undo snackbar only when a genuine reversible local operation has occurred.

## Era B — Missing mobile-native capabilities to explore

| Candidate | User value | First-slice approach | Caveat |
|---|---|---|---|
| **System Share Target** | Save a URL/text into the World from other apps | PWA share-target feasibility experiment; local review before save | Browser/platform support and installation dependent |
| **Clipboard import** | Paste links, prompts and snippets directly into a feature | User-triggered paste and provenance annotation | Never silently read clipboard |
| **Camera and photo attach** | Show agent a document/screen/object | Permissioned file/camera picker, local preview, explicit destination | Provider attachment support is capability-gated |
| **Audio note / voice composer** | Describe desired features hands-free | Optional microphone capture and speech-to-text design spike | Permissions, browser support and privacy need validation |
| **Push notifications** | Relay online/offline, approvals, agent completion | Opt-in, redacted notification contents, notification destination | Requires platform delivery architecture and permission; never expose private prompts |
| **Installable/offline shell** | Open World and drafts even when host disconnected | Cached app shell, local pending drafts, clear sync state | Offline shell ≠ offline remote AI |
| **Home-screen shortcuts** | Open Talk, World, Bootstrap directly | Evaluate manifest shortcuts and deep-link handling | Platform support varies |
| **File imports / drag-drop** | Bring saved documents, screenshots and exported chats | File chooser as baseline; drag-drop enhancement | Keep processing and storage bounded |
| **Biometric/passkey unlock** | Protect sensitive local views | Evaluate WebAuthn/passkey use against host trust model | Never equate device unlock with remote action approval |
| **Accessibility and alternate inputs** | VoiceOver/TalkBack, switch control, keyboard, reduced motion | Semantic buttons, menu alternatives, focus/announcements | Test real mobile screen readers |

## Era C — Five distinctive VIVIM interaction prototypes

### 1. Hold-to-inspect, tap-to-activate
A suggested capability appears on the blank bootstrap canvas. Long-press reveals its real effects, required permissions, source, cost/availability and whether it is reversible. Tapping **Configure** opens an explicit approval flow. No gesture directly authorizes external work.

### 2. Thumb-first conversation actions
Long-press a bubble to get **Save**, **Quote**, **Fork**, **Copy**, **Share preview**. Selecting **Save** creates a local World item with provider/thread/source metadata. Selecting **Quote** attaches only the chosen text to a new draft, with an explicit provider destination before sending.

### 3. Living World objects
Drag feature cards to organize a personal Space. Long-press to reveal **Why this exists**, **What it can access** and **Past versions**. A visible **Arrange** mode prevents accidental drag while scrolling.

### 4. Time-travel scrubber
A timeline shows config checkpoints, approval receipts and mode versions. User drags a thumb to preview past arrangements, then selects **Restore this configuration**. This restores only local settings and supported reversible changes; it never recalls already-sent prompts, public content already copied, or remote operations without compensating actions.

### 5. Pocket command surface
A persistent, thumb-reachable composer accepts “I want an AI that organizes my reading.” It returns actionable cards: **Connect provider**, **Create Space**, **Enable capture**, **Test connection**. Each card has status, consequence and inspection controls. The text box remains the fallback when gestures are unfamiliar.

## Gesture-state contract (PROPOSED)

A single touch recognizer should map each target+gesture to a **semantic intent**, never directly to an unguarded host command.

```text
pointer / keyboard / screen-reader input
  -> gesture recognizer (thresholds, cancel, scroll arbitration)
  -> semantic intent {targetId, action, origin, timestamp}
  -> capability/readiness check
  -> preview / explicit consent when required
  -> existing harness approval + send-once journal
  -> observed result -> UI state + local evidence
```

- Prefer standard Pointer Events and targeted `touch-action`; handle `pointercancel` and lost capture. Do not disable normal page scrolling or pinch zoom globally.
- Tunable hold-delay and movement threshold should be tested on physical devices; do not freeze magic numbers before usability tests.
- Prevent menu activation during scroll, text selection, OS long-press or assistive touch.
- Standard tap, overflow menu, keyboard and assistive-technology actions must dispatch the same semantic intents.
- Distinguish **preview**, **local reversible mutation**, **browser action**, **provider send**, **public publish**, and **host/OS mutation** risk classes.
- Confirm high-impact actions explicitly; no swipe, long-press or radial gesture is an approval shortcut.
- Persist idempotent intent IDs and authoritative outcomes for flaky relay/reconnection.
- Respect reduced-motion and accessible target sizes; test on small phones in portrait/landscape and with keyboard visible.

## Focused usability acceptance plan

1. **Talk:** user can save a message via hold and overflow, open source context, and compose without fighting text selection.
2. **Bootstrap:** user can inspect a card, tell whether it is installed, approve explicitly, and see a distinct success/failure state.
3. **World:** user enters Arrange mode, drags an item, cancels, and undoes a reversible reordering without losing content.
4. **Time travel:** preview older settings and restore local config without re-sending or rewriting provider history.
5. **Offline:** user sees cached World/draft locally while the host is offline, without fake “connected” badges.
6. **Accessibility:** complete each path using only taps/visible menus and with a mobile screen reader.
7. **Browser behavior:** verify scroll, text selection, zoom, edge-back, soft-keyboard and pointer cancellation on real Android Chrome and iOS Safari/PWA as available.

## Separate backlog (not added to roadmap yet)

| ID | Priority | Experiment | Pass criterion |
|---|---|---|---|
| MX01 | P0 | Gesture vocabulary + tap/overflow parity | Every critical gesture action has an accessible tap path |
| MX02 | P0 | Chat long-press context sheet | No accidental sends or selection/scroll breakage |
| MX03 | P0 | Capability hold-to-inspect sheet | Real readiness/permission/reversibility data displayed |
| MX04 | P0 | Thumb-reachable composer and bottom sheets | Usable with keyboard open on small phone |
| MX05 | P1 | Arrange mode with drag/reorder | Cancel/undo and auto-scroll proven |
| MX06 | P1 | Time-travel preview/scrubber | Does not roll back remote side effects |
| MX07 | P1 | Installed PWA share-target + file inputs spike | Supported-platform matrix and privacy checks |
| MX08 | P1 | Offline/sync-state and rejoin | No false action success on disconnect |
| MX09 | P1 | Accessibility / reduced-motion / touch-target audit | Mobile keyboard + screen reader complete core journeys |
| MX10 | P2 | Haptics, radial menus, pinch World canvas | Measured value over ordinary menus, safe fallback |

## Research references

- Apple context-menu guidance: https://developer.apple.com/design/human-interface-guidelines/context-menus
- Apple long-press gesture handling: https://developer.apple.com/documentation/uikit/handling-long-press-gestures
- MDN Pointer Events: https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events
- MDN pointercancel: https://developer.mozilla.org/en-US/docs/Web/API/Element/pointercancel_event
- GitHub Primer mobile accessibility checklist: https://primer.style/accessibility/tools-and-resources/checklists/mobile-checklist/

## Decision to make next

Prototype **MX02 + MX03 + MX04** in an isolated mobile UX demonstrator using fake local data first, then connect to **existing** harness read-only capability and journal state. Do not disrupt the current pairing/Talk implementation while exploring gestures. Record videos/screenshots, platform behavior, accessibility findings and the user's preferred interaction semantics before promoting chosen items to `docs/roadmap.json`.
