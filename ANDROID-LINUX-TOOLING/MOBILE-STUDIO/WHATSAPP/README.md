# WhatsApp — first Mobile Studio integration (R001)
Owner objective: enable VIVIM to read the owner's WhatsApp conversations and send messages natively through the installed Android WhatsApp app. Personal account, native phone, no separate business service required. Full access is an aspiration, not a supported capability claim.

## Actual phone evidence
The existing authorized Termux bridge discovered com.whatsapp. A same-user read-only Android package-manager resolution on API 35 returned exit 0:
- MAIN + LAUNCHER: com.whatsapp/.Main
- SEND + text/plain: com.whatsapp/.contact.ui.picker.ExternalShareAlias
- VIEW + https://wa.me/: com.whatsapp/.TextAndDirectChatDeepLink
The initial unspecified-user query failed with INTERACT_ACROSS_USERS permission denial. The second query explicitly targeted the existing same Android user (0); no grant elevation, pairing or root. No app launched, no chat read, no message sent. These are discovered entry points, not verified runtime behaviors; internal class names are evidence only, never stable contracts.

## Capability paths and limits
1. Outbound draft: documented click-to-chat URL or native ACTION_SEND; route to official app, user verifies recipient and text and sends. Draft launch is not delivery. No blind retries.
2. Historical read: WhatsApp Export chat, user selects a chat and shares the text to a companion importer. Supports local parsing and indexing, incomplete per-chat snapshots, not a live account-wide history API. Respect disappearing/view-once content and deletions; no recovery attempts.
3. Incoming event: optional user-authorized Android notification listener restricted in application logic to WhatsApp and opted-in conversations; previews may be absent, grouped or redacted, cannot reconstruct all history. Keep local, avoid OTP/sensitive content, allow revocation and deletion; no notification access granted now.
4. Native reply: evaluate app-provided notification reply affordance with explicit user confirmation, exact conversation and fresh action validation. Do not claim arbitrary programmatic send or delivery; user-facing WhatsApp composer is baseline.
5. In-app user-selected screen observation: official MediaProjection, user consent each session, no protected capture; no unattended surveillance. Review Accessibility API restrictions before any user-directed UI experiments; no autonomous accessibility agent or security bypass.
6. Official WhatsApp Business API is business-account infrastructure, not a general personal-account API. Official linked clients may sync only part of history; not a replacement for native integration. No unofficial protocol, extraction of app-private databases, backup keys, root, token access, or account conversion.

## Work sequence
WA01 verified: same-user app discovery and intent resolution.
WA02 candidate: draft via official deep link with a self-chat test and user confirmation; verify actual native composer and visible send result.
WA03 candidate: manually export a self-chat fixture; receive via user-selected share/SAF; parse and deduplicate in Rust, record coverage and provenance.
WA04 candidate: build minimal native Android companion with no broad permissions by default; test on owned app and separately authorize notification scope if wanted.
WA05 candidate: opted-in incoming test notification and explicit reply; verify recipient and actual outcome; reject stale/ambiguous notifications.
WA06 candidate: Product SDK/VLANG adapter with typed read/draft/send intent, grant scope, fresh observation, user approval, idempotent request and distinct submitted/delivered/unknown states. Never equate local command exit with message sent.
WA07 candidate: user-selected app observation, gap analysis against full history and full send objective. No production migration from Labs.

## Sources
https://faq.whatsapp.com/5913398998672934
https://faq.whatsapp.com/1180414079177245/
https://faq.whatsapp.com/653480766448040/
https://developers.facebook.com/documentation/business-messaging/whatsapp/overview/
https://developer.android.com/reference/android/app/RemoteInput
https://developer.android.com/reference/android/service/notification/NotificationListenerService
https://developer.android.com/media/grow/media-projection
https://support.google.com/googleplay/android-developer/answer/10964491
https://source.android.com/docs/compatibility/15/android-15-cdd
