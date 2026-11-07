# VIVIM Mobile UX — Research, Semantic Mapping & Schema Contract

**Date:** 2026-10-30  
**Status:** FORMAL PROPOSAL / research bindings. **Not production-ready, not a claim of on-device support.**  
**Owner:** Mobile Experience research track.  
**Boundary:** Independent of the historical IP registry, cross-repo harvest and authoritative `docs/roadmap.json`.

**Machine-readable artifacts**
- [JSON Schema 2020-12](./schema/mobile-interaction.schema.json) — five top-level document kinds.
- [28 binding reference catalog](./schema/mobile-interaction.catalog.json) — research-stage proposed mappings.
- [Mobile Interaction Lab](./MOBILE-INTERACTION-LAB-2026-10-30.md) — preceding UX hypothesis list.
- [Product-level journey](../USER-JOURNEY.md) — user-facing intent and Build/Talk coexistence.
- [Gate 1 goals](../../GOALS.md) — host, provider, pairing and send authority.

## 0. Decision

**Gesture is an input technique, never an authorization channel.** Normalize touch, keyboard, visible controls, and assistive actions into the *same semantic action*. Before an external effect, validate the exact target and current capability, show a preview, and use the **existing harness approval and send-once journal**. The formal schema describes UI-side intent; the harness must revalidate and authorize everything it actually executes.

**Do not create a parallel browser controller, separate send endpoint, new approval authority or hidden model API.** The phone remains a client of the paired host; AI provider calls are through the signed-in web UI only.

## 1. Surfaces, interaction roles and user goals

| Surface (`surface`) | Primary targets | Fast interactions | Visible fallback | Primary user promise |
|---|---|---|---|---|
| `global` | tabs, status, bottom sheet | tap tab, pull status, dismiss sheet | tabs, Refresh and Close buttons | Know where I am and whether my PC is connected |
| `talk` | message, conversation, composer | hold message, swipe conversation, attach | message overflow, action sheet, file chooser | Talk without losing or accidentally sending anything |
| `build` | capability, composer, canvas | hold to inspect, tap configure, optional pinch | Inspect/Configure buttons, Zoom slider | Build my personal app without technical setup |
| `agents` | agent | hold to inspect | Details button | Understand what an agent is doing and what it can access |
| `world` | world_object | hold, explicit Arrange drag, attach context | overflow, Move Before/After, Add context | Own and rearrange my saved knowledge |
| `history` | revision | timeline scrub / preview | labeled slider, Preview and Restore buttons | See and restore only reversible settings |
| `self` | mode | tap switch mode | mode picker | Carry several personal app configurations |
| `share` | share_preview | hold preview, tap Review Publication | Preview and Review buttons | Publish only what I deliberately choose |

**Critical terminology:** User-facing **Build** is the visual bootstrap; **Talk** remains a working, separate chat surface. **World** is local saved material; **History** means configuration revisions, not remote-provider history. The UI may combine screens later, but semantic IDs stay stable.

### Mapping: recognition -> semantic action -> effect

| Binding | Input | Semantic action | Side-effect class | Safety / fallback |
|---|---|---|---|---|
| `mx.talk.message.hold` | 480ms hold, 12px slop *hypothesis* | `message.actions.open` | read-only | Don't swallow text selection; overflow menu |
| `mx.talk.message.save` | Save button from sheet | `message.save` | local reversible | Source provenance and undo local creation |
| `mx.talk.conversation.swipe` | horizontal swipe (64px *hypothesis*) | `conversation.actions.open` | read-only | Reveals actions, does not archive/delete by swipe |
| `mx.talk.composer.send-review` | Send button | `composer.send.review` | read-only preview | Final provider send is separately approved/journaled |
| `mx.build.capability.hold` | long-press proposal | `capability.inspect` | read-only | Show readiness, effects, permission, undo; visible Inspect |
| `mx.build.capability.propose` | Request proposals | `capability.propose` | remote write | **Explicit owner approval before webapp send**; no auto retry |
| `mx.build.capability.activation-review` | Activate entry | `capability.activate.review` | read-only preview | Execution effect class determined by manifest and approved at host |
| `mx.world.object.drag` | drag handle, Arrange mode | `object.reorder` | local reversible | Move Before/After fallback; undo config revision |
| `mx.history.revision.scrub` | scrub a version | `revision.preview` | read-only | Slider; no restore until explicit confirmation |
| `mx.history.revision.restore-review` | Restore control | `revision.restore.review` | read-only preview | Name unundoable remote side effects before local restore |
| `mx.self.mode.switch` | mode button | `mode.switch` | local reversible | Persistent local revision; remote history unchanged |
| `mx.global.status.pull` | pull to refresh status | `status.refresh` | remote read | Refresh button; no command replay |
| `mx.share.publish.review` | Review publication | `share.publish.review` | read-only preview | Publication needs separate publication authority + sanitization |

All 28 catalog entries have an explicit `fallback`; the table highlights the consequential paths.

## 2. Five formal document kinds

Every object carries `kind` and `schemaVersion: "1.0.0"`; the schema is strict about unknown properties.

1. **`interaction.catalog`**: stable `binding.id`; `surface`, `targetRole`, `trigger`, `semanticAction`, `risk`, `guard`, `fallback`, `reversibility`, `feedback`, `stage`, `testCaseIds`.
2. **`interaction.intent`**: a user-sourced action with opaque target reference, origin, timestamp, binding ID and expected config/capability revision. **Not** an approved command. Intentionally excludes arbitrary prompt text, credentials, raw screen coordinates and file bytes.
3. **`interaction.guard`**: evaluated allow / preview / owner-approval-needed / blocked decision and a reason. Any reported approval-request reference is descriptive; it does **not** convey authorization.
4. **`interaction.outcome`**: observed result (`previewed`, `cancelled`, `committed`, `blocked`, `failed`, `uncertain`) with authoritative receipt link for committed harness, host and publication effects.
5. **`interaction.device_report`**: per-device, timestamped feature support with *how support was determined*. `documentation_only` does not count as live proof.

### Example: inspect a Build proposal (valid `interaction.intent`)

```json
{
  "kind": "interaction.intent",
  "schemaVersion": "1.0.0",
  "intentId": "ui-1a2b3c4d5e6f7081",
  "bindingId": "mx.build.capability.hold",
  "semanticAction": "capability.inspect",
  "surface": "build",
  "target": { "role": "capability", "id": "feature.reading_assistant" },
  "origin": "pointer",
  "issuedAt": "2026-10-30T21:00:00Z",
  "expected": { "configRevision": "rev-8" }
}
```

Proposed reply (valid `interaction.guard`) for read-only inspection:

```json
{
  "kind": "interaction.guard",
  "schemaVersion": "1.0.0",
  "intentId": "ui-1a2b3c4d5e6f7081",
  "decision": "allow_local",
  "reasonCode": "none",
  "evaluatedAt": "2026-10-30T21:00:01Z"
}
```

No external provider sends occur in this inspect sequence. A subsequent activation request requires a **different reviewed action**; do not inherit approval from the inspection.

## 3. Interaction recognizer: proposed deterministic state machine

```text
IDLE
  pointerdown on eligible target -> PRESS_PENDING (record pointerId, target, origin)
  key/menu/assistive action  -> dispatch semantic intent immediately
PRESS_PENDING
  movement beyond slop      -> SCROLL_YIELD or SWIPE_TRACK / DRAG_TRACK (if eligible)
  hold threshold elapsed    -> CONTEXT_OPEN (only if no selection/scroll)
  pointerup before threshold -> TAP_COMMIT
  pointercancel, new pointer, focus loss, native selection -> CANCELLED
SWIPE_TRACK / DRAG_TRACK
  scroll or OS gesture wins  -> CANCELLED
  pointerup with valid threshold + allowed mode -> INTENT_DISPATCHED
  pointercancel             -> CANCELLED
CONTEXT_OPEN
  release -> no second tap/click; choose action from sheet
ANY TERMINAL STATE -> IDLE, clear timers/captures and suppress duplicate synthetic click
```

**Initial tuning values are research hypotheses**, not standards: long hold 480 ms, movement slop 12 CSS px, swipe 64 CSS px, status pull 72 CSS px, drag slop 10 CSS px. Test with real thumbs, screen readers, selections and low-end Android devices before making them default.

Precedence: OS/browser navigation and page scroll > native text selection/composer input > assistive tech > explicit Arrange handle > long-press context > swipe reveal > tap. Never capture edge-back gestures or apply `touch-action: none` to the whole page. Normal lists should keep vertical scroll; the canvas may own gestures **only within its bounded interactive region**. Handle `pointercancel` and `lostpointercapture` explicitly.

## 4. Authorization and replay state machine

```text
INTENT_CREATED (mobile)
  -> VALIDATE_SCHEMA + LOOKUP_BINDING (client advisory only)
  -> CHECK_SCOPE/CAPABILITY/PAIRING/REVISION (host authoritative)
  -> BLOCKED | PREVIEW_REQUIRED | OWNER_APPROVAL_REQUIRED | ALLOW_LOCAL/READ
  -> OWNER_REVIEWS_EXACT_TARGET + EFFECT + REQUEST (when required)
  -> HOST APPROVAL / EXISTING JOURNAL (not gesture permission)
  -> QUEUED -> CLAIMED -> COMMITTED | FAILED | UNCERTAIN
```

For read-only presentation, render a local preview without asking host permission where safe. For local reversible edits, persist a versioned config and allow undo. For **any external write** (provider send, publication, host change, agent execution), force checked preview and exact owner authorization. Re-check idempotency at the host; a matching UI intent ID cannot substitute for a harness send journal key.

On timeout or lost relay acknowledgement: **UNKNOWN IS NOT FAILED**. Mark `uncertain`, query status by the existing request/receipt ID, and do not replay or create a second send. Browser background sync may independently retry tasks, so it must **never** be used to replay non-idempotent provider actions.

For `public_publish`, require a scrubbed content preview, specific destination/audience, explicit approval, publication receipt and eventual revoke controls where supported; revocation cannot promise removal of copies already received.

## 5. Capability detection / browser support strategy

**Three states are different:** documented API existence, feature probe result and verified real-device journey. Unsupported or unknown means visible fallback, not fabricated success.

| Capability | Delivery posture | Detection/verification | Fallback |
|---|---|---|---|
| Pointer events + `touch-action` | Base interaction engine | `PointerEvent` / CSS support and real gesture arbitration | Native click/tap menus |
| `VisualViewport` for keyboard | Progressive baseline | Runtime presence, OSK open/close test | Responsive layout, scrollIntoView |
| Native `contextmenu` event | Opportunistic | Event/device test | Explicit message overflow |
| Web Share Target | Progressive feature; installed PWA support varies | Manifest registration + actual OS Share sheet | Paste/link/file importer |
| File/camera picker | Permissioned native control | File input accept/capture test on real device | Standard file chooser |
| Clipboard | User-initiated only | Feature probe + paste permission test | Native paste into text field |
| Voice input / microphone | Optional experiment | Microphone permissions + end-to-end speech path | Typed composer |
| Vibration / haptics | Optional enhancement | `navigator.vibrate` plus physical feedback test | Visual cues |
| Notifications/push | Opt-in and redacted | Permission and delivery test per installation | In-app status/readiness |
| Offline shell | Cached UI, drafts and saved World only | Offline reopen test + host status reconciliation | Offline explanation; no fake AI |
| Manifest home-screen shortcuts | Progressive installed feature | Actual launcher menu test | In-app tabs |
| Passkey/WebAuthn | Separate secure-design spike | Device credentials + binding and recovery study | Paired device re-auth; no auto-approval |
| Screen reader / switch / keyboard | Core acceptance criterion | TalkBack, VoiceOver, keyboard and switch testing | Semantic buttons and labeled actions |

**Do not promise browser-specific support without measured device results.** In particular, Web Share Target, vibration, OS shortcut menus and the VirtualKeyboard API are not uniform Web Platform Baseline capabilities.

## 6. Thumb, focus and accessibility contract

- Target 44–48 CSS px for primary touch controls as a *VIVIM design goal*. WCAG 2.2 AA 2.5.8 defines a 24 × 24 CSS px minimum with documented exceptions; its criterion is not 44 or 48.
- Every swipe, hold, drag, pinch and pull has a single-tap equivalent. For drag, provide **Move Before/After** buttons; for history, labeled slider/buttons.
- Don't rely on color, haptics or motion alone. Announce `queued`, `blocked`, `connected`, `uncertain` as status text, and respect `prefers-reduced-motion`.
- Modal bottom sheets require accessible name, logical focus entry, focus containment, Escape/back handling, focus return and no dismissal of unsaved drafts/approval without warning.
- Keep the composer above the *visual* keyboard and safe-area inset; avoid auto-scroll jumps when the keyboard opens. Measure visual viewport on Android and iOS.
- Do not override selection, copy/paste, browser pinch-to-zoom or OS edge-back. Offer optional built-in canvas zoom only inside the canvas.
- A tap on a proposal means inspect/preview by default; activation always uses a named explicit button and checked authority.

## 7. Validation obligations beyond JSON Schema

JSON Schema validates **shape**. Application policy still must enforce:

1. Unique `binding.id` values in each catalog and globally stable semantic-action names.
2. Runtime `intent.bindingId` exists; `semanticAction`, `surface`, and `target.role` match binding; target belongs to the user/session.
3. Trigger/gesture collisions resolved for the same target. A hold cannot also generate a click/send.
4. Intent IDs single-use for effects; guard and outcome IDs correlate; no approval granted by an AI output or user-supplied JSON.
5. **Dynamic effect risk can only increase**, never downgrade, the catalog's declared risk (e.g., activation preview becoming a host mutation).
6. `requiresVerifiedCapability` means read-only host evidence newer than agreed TTL and matching provider/model; never rely on static provider claims.
7. Device report entries unique by feature; reported availability is not authorization.
8. No raw message text, raw touch coordinates, credentials, cookies, file blobs or clipboard contents in interaction protocol logs.
9. External effect receipt and uncertain-state classification come from harness journal, not optimistic frontend state.
10. Config revision compare-and-swap before local restore/reorder; retain prior snapshots and disallow silent overwrite on multi-phone races.
11. Preserve current working Talk, pairing and cookie auth; no unscoped event interception.
12. Unknown schema major version or unknown action -> refuse with visible accessible error, not a best-effort host action.

## 8. Test mapping and field evidence

| Case | Setup | Exercise | Must observe |
|---|---|---|---|
| MX01 | Android mobile portrait; tabs & menus | Tap-only journey, open/close sheet, back | No gesture-only critical action; no back hijack |
| MX02 | Real provider message containing selectable text | Hold + overflow; scroll during hold; select text | One context sheet or native selection; zero accidental sends |
| MX03 | Available/unavailable proposal cards | Hold / tap Inspect; Configure; Activate review | Correct readiness, permissions, reversible scope; no auto activation |
| MX04 | Small phone, keyboard visible and hidden | Focus/compose/send review, rotate and change browser chrome | Composer remains operable, no lost draft, explicit send gate |
| MX05 | World object list, Arrange mode | Drag reorder, cancel, undo; Move Before/After | Reversible, same result by taps, scroll preserved outside handle |
| MX06 | Two named modes, >=3 config revisions | Scrub, inspect, restore earlier config | No remote resend; local state restored and journal preserved |
| MX07 | Installed PWA and normal browser tab | OS share-to-app, file attach, camera permission | Proper fallback when unsupported; private preview prior to ingestion |
| MX08 | Pair then cut host connection | Pull refresh, offline open, queue local draft, reconnect | Offline truth, no implicit background send or false success |
| MX09 | TalkBack/VoiceOver and hardware keyboard | Full critical path, focus in sheets, reduced motion | Labels, focus order, target spacing, status announcements |
| MX10 | Canvas with many objects | Optional pinch/pan/haptics, OS back and zoom | No app-wide gesture capture; real benefit over basic controls |

**Evidence packet per run (PROPOSED):** `testCaseId`, device OS/browser/installation mode, app revision, schema revision, time, expected behavior, observed behavior, screenshots or video location, sanitized intent/receipt refs, accessibility notes, issues, pass/fail. Keep videos/screenshots private unless explicitly sanitized and approved for publication.

### First implementation experiment (without breaking existing Talk)

**Spike A:** message hold + overflow with cancellation and selection.  
**Spike B:** Build proposal hold + bottom sheet with real readiness evidence.  
**Spike C:** thumb-first composer that survives mobile keyboard resizing.  
**Spike D:** tap-only parity and failure/reconnection acceptance.

Only after on-device proof should the chosen catalog bindings be promoted from `research` to `verified_on_device`, and only scheduled implementation tasks move into `docs/roadmap.json`.

## 9. Source research and standards

Primary standards and browser documentation, checked 2026-10-30:

- W3C Pointer Events Level 3, Recommendation 2026-06-30: https://www.w3.org/TR/pointerevents3/
- MDN `touch-action` and pointer cancellation: https://developer.mozilla.org/en-US/docs/Web/CSS/touch-action
- MDN Pointer Events and pointer capture: https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events
- W3C WCAG 2.2 (2.5.1, 2.5.2, 2.5.7, 2.5.8): https://www.w3.org/TR/WCAG22/
- W3C dragging-movements understanding: https://www.w3.org/WAI/WCAG22/Understanding/dragging-movements
- MDN `VisualViewport`: https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport
- MDN VirtualKeyboard API (limited availability): https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard_API
- MDN PWA `share_target` (limited availability): https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Manifest/Reference/share_target
- MDN `Navigator.vibrate()` (limited availability): https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate
- MDN accessible dialog/focus: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/dialog_role
- MDN PWA manifest shortcuts (limited availability): https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Manifest/Reference/shortcuts
- MDN offline/background operation: https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Guides/Offline_and_background_operation

## 10. Concrete handoff and boundary

Formal definitions and 28 mapping records are present on main as **research artifacts**, not wired UI behavior. A structural internal check has confirmed binding ID uniqueness, recognized action/surface/risk enums and safety rules for high-impact bindings. A full independent Draft 2020-12 schema-validator run, platform compatibility evidence and live on-device interaction testing have **not** been completed. The model of UI intent must not be promoted into an authoritative server protocol without revalidation, typed handlers and threat review.

**No roadmap, historical-IP registry or running mobile app files were modified in this UX research pass.**
