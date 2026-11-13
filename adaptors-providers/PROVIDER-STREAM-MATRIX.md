# Provider streams, interfaces and priority — R001
All rows are CANDIDATES, not connected services. Machine-readable enumeration lives in [providers.json](catalog/providers.json). No default permission, no auto-send.

## Stream families
| Surface | Data units | Proposed UI | Ingest/control route | Safety boundary |
|---|---|---|---|---|
| Messages | thread, participant, message, attachment, read-state, reaction, draft | unified inbox, thread list, conversation, composer, send confirmation and receipt | Host-approved browser read or user export; outbound only via Host | account+recipient+thread binding, E2EE visibility, no silent send |
| Social feed | post, comment, reply, like, share, bookmark, group, event | timeline, filter, saved items, discussion detail | official owner export or permitted bounded read | avoid other-user mass collection |
| Media | photo, video, story, reel, playlist, watch history | library, album grid, player, metadata drawer | user-select/download/export | licensing, disappearing content, large binary storage |
| Contacts/graph | friends, followers, connections, participants | relationship explorer, origin/source badge | explicit source-scoped grant | sensitive social associations, identity merging review |
| Email | mail thread, message, draft, attachment | inbox, thread, composer (later) | user export, approved Host browser | send effects separate; third-party confidentiality |
| Files/knowledge | file, folder, page, comment, revision | library, preview, document provenance | local/official export, local sync | no second document store |
| Developer/AI | repo, issue, chat turn, task, decision, artifacts | activity stream, thread, graph link | existing readers/importers | source evidence, no inference as truth |
| Device/calendar | file share, event, notification | intake sheet, calendar, review | OS-granted local file/feed | Android permissions and off-device disclosures |

## First-round exemplars
- **P0 Facebook:** profile/posts/comments/media/history via selected account export and scoped browser observation, separately from message-send.
- **P0 Facebook Messenger:** 2026 desktop route `facebook.com/messages`. The former standalone `messenger.com` website ended in April 2026. Browser session and web availability differ from Messenger-only mobile accounts. Discovery must verify the actual signed-in account and site origin; no hardcoded DOM selectors.
- **P0 Instagram:** posts, Reels/Stories and Direct are different stream families. Direct messaging is a **candidate**, not assumed live-browser functionality.
- **P0 WhatsApp:** official linked-web route requires owner-controlled device linking; preserve E2EE/attachment limitations. No QR interception or cookie transfer.
- **P0 YouTube:** history, playlists, subscriptions and comments; a video feed is not a direct messaging channel.
- **P0 ChatGPT:** reuse existing Labs import work and Studio capture; do not add a parallel AI web actor.
- **P1:** X, Reddit, LinkedIn, Telegram, Discord, Slack, Gmail, Outlook, Google Drive, TikTok, GitHub and knowledge services.
- **P2:** Signal, Pinterest, Mastodon, Spotify and constrained/mobile-specific variants; feasibility unknown.

## Capability states (never collapse)
`CATALOGED` → `ACCOUNT_OBSERVED` → `PERMISSION_GRANTED` → `READ_READY` → `OUTBOUND_CAPABILITY_VERIFIED` → `SEND_APPROVED` → `SENT_PROVIDER_CONFIRMED`.
A live session can be observed but missing permission. A thread can be read but not sent to. "Sent" is not the same as delivered/read; show provider semantics and `UNKNOWN` when uncertain. Failed/denied/revoked/partial/out-of-date states remain explicit.

## Source considerations / unresolved
2026 reporting on Messenger web route: https://techcrunch.com/2026/02/19/meta-is-shutting-down-messengers-standalone-website/ . WhatsApp linked devices: https://www.whatsapp.com/download/desktop . Existing VIVIM source atlas: `personal-knowledge-lifecycle/SOURCE-ATLAS.md`. These references do **not** prove VIVIM adapter support or that any browser automation is allowed under a service's current terms. Account/region/terms/format and provider UI must be re-checked before adapter implementation.