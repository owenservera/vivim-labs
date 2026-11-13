# Provider cohort: top ten candidates, all LIVE_UNVERIFIED

| # | Provider/product | Family | Mock streams | Mock action | Dedicated lane |
|---|---|---|---|---|---|
| 1 | Facebook | social | posts/comments/reactions/groups/media; export | post/reaction proposal | W3-A |
| 2 | Facebook Messenger | messaging | threads/messages/participants/attachments | draft→approval→receipt mock | W3-B |
| 3 | Instagram | social+messaging | posts/reels/stories/comments/DMs | DM/comment proposal | W3-C |
| 4 | WhatsApp | messaging | linked-web threads/groups/media; export | message proposal and uncertainty | W3-D |
| 5 | X | social+messaging | feed/replies/bookmarks/DMs | DM/reply proposal | W4-A |
| 6 | YouTube | media+social | history/playlists/channels/comments | comment proposal | W4-B |
| 7 | Reddit | social+messaging | posts/replies/saved/inbox | reply/message proposal | W4-C |
| 8 | LinkedIn | social+messaging | feed/connections/messages | message/comment proposal | W4-D |
| 9 | Telegram | messaging | chats/channels/groups/media | message proposal | W4-E |
| 10 | Gmail | email | mail threads/labels/attachments/drafts | reply/send proposal | W4-F |

Facebook and Messenger are distinct product packs even if both relate to one Meta account. Instagram has separate account and DM restrictions; WhatsApp web requires actual user-controlled linking. The top-ten set is a purposeful initial mock cohort, not a ranking of providers by popularity.

Each pack declares productId, candidate access modes, account identity, stream kinds, permissions, supported mock operations, unsupported operations, read freshness, pagination, failure states and evidence pointers. No shared flag may convert a browser tab into an account grant.

Completeness definition per provider: descriptor + synthetic account(s) + 2 or more scenario fixtures + read/list/detail projected via actual Hub + permission denied/partial/stale + one explicit action proposal or UNSUPPORTED result + tests. Live status stays unverified regardless of mock success.
