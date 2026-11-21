# Gestures R002 — visible acceptance & adversarial contract

State: SPECIFIED / UNRUN. Proposed PWA outcome PV-15. No phone gesture test or runtime code implemented here.

## Visible finish line
On the SAME First Contact PWA URL and a verified deployed SHA: type a non-sensitive query -> a real Rust-returned Wiki card appears -> hold on card and observe context affordances BEFORE releasing -> adjust/slide to see another preview -> release and cancel without mutation -> optionally tap explicit Apply for an existing low-risk Host-authorized action -> inspect receipt and Wiki readback.

Important scope split: The current First Contact composer is read-only search, with Send disabled. The initial read-only hold/preview/cancel milestone is independent of the later Host effect milestone. If no safely authorized action is present, Apply remains BLOCKED rather than creating an unauthorized Wiki writer.

| Gate | Visible behavior | Must also prove | Evidence | State |
| --- | --- | --- | --- | --- |
| G0 | Wiki item appears from actual typed query | same URL, source revision/digest | deployed SHA, actual visible item | UNRUN for gestures |
| G1 | holding progressively reveals contextual actions | browser scroll and text selection still work | frames during down/dwell, timestamps | UNRUN |
| G2 | finger movement changes focus/preview | move out, second pointer, pointercancel safely cancel | candidate IDs, frames, pointer chronology | UNRUN |
| G3 | ordinary release/cancel changes no persisted state | Escape, background, unmount and lost focus cancel | no effect receipt; before/after source identity | UNRUN |
| G4 | gesture target is stable and revision-aware | stale Wiki/source/grant prevents old proposal | source watermark, visible stale/refusal | UNRUN |
| G5 | explicit Apply goes through existing Host approval | wrong actor/grant, double submit, disconnect refuse | action ID/grant/receipt correlation | UNRUN |
| G6 | approved outcome is truly read back by same PWA | delayed/revoked/missing receipt not success | view revision, source readback, receipt | UNRUN |
| G7 | keyboard/tap/a11y reaches equivalent meaning | drag-free path, reduced motion, focus return | screen-reader/keyboard PWA proof | UNRUN |
| G8 | actual phone-browser proof; unchanged URL | orientation, scroll and low-resource device | device/URL/SHA, screenshots, visual trace | UNRUN |
| G9 | privacy and bounded episode observation | no raw text/touch/audio default logging | opt-in status and redacted metadata | UNRUN |

## Proposed focused test inventory
- Pure Rust: valid state transitions, pointer/session mismatch, monotonically ordered samples, stale revisions, rapid target replacement, cancellation idempotence, unknown candidate, expired TTL, no implicit authorization, at most one explicit commit intent.
- TierFlow: baseline UNSUPPORTED_GESTURE present; after change test exact trusted source bindings, wrong document versions, unsupported pinch/swipe, cancel and effects-none result.
- Browser: PointerEvent down/move/up/cancel, scrolling competition, native details toggle, touch-action scope, focus/IME, overscroll, rebuilt Wiki results, keyboard equivalent.
- Host: explicit Apply permission, approval, same-action replay/double tap, stale grant, wrong target, denial, actual receipt.
- Device: independent real Windows PWA proof and optional phone-browser PWA proof; native Android APK not required for PWA visibility.

## Test receipt schema (reuse current task/claim receipt)
Task ID; gate ID; actor; initial Wiki target+revision; actual input; PWA URL; ARBOR source+deployed SHA; expected preview; actual visible frame; cancellation/Host receipt; screenshot/trace ref; tester identity; tested time; PASS/FAIL/BLOCKED/UNRUN; fresh regression result.

A unit test source file, GitHub commit, static screenshot mock, HTTP 200 or prepared packet alone does NOT satisfy these gates. Do not mutate personal production Wiki in tests.
