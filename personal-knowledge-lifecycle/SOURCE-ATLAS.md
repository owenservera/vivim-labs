# Personal knowledge source atlas — provider and local intake opportunities

Date: 2026-11-11. This is a **candidate catalog**, not a list of functioning VIVIM connectors.

## Access-mode taxonomy
- **L-FILE** user-selected files/folders, OS permitted; **L-APP** user-selected app exports/local data with schema verification; **B-WEB** existing signed-in browser app with user permission, provider restrictions and explicit bounded capture; **P-EXPORT** official export/data-portability archive; **S-FEED** RSS/ICS/CalDAV/WebDAV/other documented user-authorized standard feed; **P-API-RESEARCH** official service API, requires deliberate policy exception if conflicting with no-provider-API product invariant; **MANUAL** user paste/import; **UNVERIFIED** no assured safe method.
- Availability and technical legality must be checked per provider, account tier, region, platform, terms and current export schema. Authentication is never copied from Chrome to an unrelated network client. Do not implement broad invasive screen/keylogging or surveillance.

## Source candidates, prioritized for knowledge yield
| Family | Candidate examples | Initial import | Continuous updates | Notes / priority |
| --- | --- | --- | --- | --- |
| Local documents | PDF, DOCX, TXT, MD, HTML, CSV, JSON, EPUB (where supported) | L-FILE | approved folder watcher | P0: provenance offsets, attachments, filesystem rename |
| Local developer work | Git repos, commits, issues snapshots, code, readmes, tests | L-FILE | Git change observation | P0: separate personal vs development domain; no unbounded private repo uploads |
| Local conversations | exported chat JSON, saved transcripts, browser downloads | L-FILE / MANUAL | new approved exports | P0: thread IDs, edits, branching, roles |
| ChatGPT | eligible account export ZIP (JSON, other data), signed-in existing web app | P-EXPORT / B-WEB | bounded approved session capture or re-export | P0; Business/Enterprise export differs |
| Claude | Settings > Privacy data export for eligible users; native files may not be included | P-EXPORT / B-WEB | bounded session capture / re-export | P0, account/workspace restrictions |
| Gemini | Google Takeout Gemini Apps activity and Gemini data categories | P-EXPORT / B-WEB | selected export or session | P0; verify category formats |
| Other model chats | Qwen, DeepSeek, Kimi, Z.ai, MiniMax, Perplexity, Poe, Grok, Mistral Le Chat, OpenCode | P-EXPORT if official, B-WEB if permitted, MANUAL | adapter-dependent | P1; **do not assume** exports or documented interfaces |
| Coding assistants | OpenCode, ZCode, Claude Code, Codex, VS Code assistants, Cursor | L-APP / export | local session watcher | P0/P1; discover schemas and privacy boundaries |
| Browser artifacts | Chrome/Vivaldi/Edge bookmarks, user-approved history, reading lists, downloads | L-APP / selected export | bounded browser events | P1; browsing history especially sensitive; not implicit consent |
| Email | Gmail, Outlook, Thunderbird, MBOX, EML, PST | approved exports / L-APP | local mail snapshots or explicitly sanctioned connector | P1; redact contacts, third-party private data |
| Calendar / contacts | ICS, VCF, CalDAV, Google/Outlook exports | P-EXPORT / L-FILE / S-FEED | user-authorized feed | P1; contacts are not unlimited consent for other people's data |
| Cloud files | Google Drive, OneDrive, Dropbox, Box, iCloud Drive, Nextcloud | synced local folder / export / sanctioned connector | local sync delta / approved account route | P1; avoid assuming API access |
| Collaboration | Slack, Discord, Teams, Notion, Obsidian, Joplin, Evernote, Confluence | exports, local vaults | watch/export if supported | P1; workspace admin restrictions and third-party privacy |
| Project management | GitHub, GitLab, Jira, Linear, Trello, Asana | exports / local clones | selected event snapshots | P1; authority vs inferred state |
| Social networks | Instagram, Facebook, X, LinkedIn, Reddit, Bluesky, Mastodon | official takeout/download where offered | re-export / documented feed | P2; social graph is sensitive and provider-specific |
| Messaging | WhatsApp, Telegram, Signal, Messenger, iMessage, SMS | user-approved native exports, where available | export cadence / carefully scoped local adapters | P2; E2EE, media and other-party consent complexities |
| Media & interests | YouTube history/subscriptions/playlists, Spotify/Apple Music listening exports, podcasts, RSS, Kindle highlights | account export / OPML / local files | periodic user-approved refresh | P2; avoid equating recommendation signals with explicit preferences |
| Photos / locations | EXIF photo libraries, Google Photos, Apple Photos exports, Maps timeline | L-FILE / P-EXPORT | selected album/folder watchers | P2; sensitive precise location/face information, opt-in only |
| Personal finance | bank exports CSV/OFX/QIF, invoices, receipts | L-FILE | user-managed export only | P3 opt-in vault, no default extraction/disclosure |
| Medical and biometric | FHIR files, medical documents, wearable exports | manually selected export | only with explicit specialized consent | P3 restricted vault, no default AI external delivery |
| Mobile / device | Android shared storage, user-selected app exports, photos, notes, notifications | Android SAF / explicit sharing | scoped events permitted by OS | P2; permissions and device limits, no covert background collection |
| Web knowledge | bookmarks, saved articles, user-selected sites, public pages | direct files / permitted fetch | RSS/sitemaps where allowed | P1; public facts are not always personal facts |

## Provider-specific verified public references
- ChatGPT export: https://help.openai.com/en/articles/7260999-exporting-your-chatgpt-history-and-data — eligible accounts, ZIP, privacy restrictions; links may expire.
- Claude export: https://support.claude.com/en/articles/9450526-export-your-claude-data — individual data export, mobile initiation limitations, uploaded original files omitted.
- Gemini export: https://support.google.com/gemini/answer/16920332 — Google Takeout with Gemini Apps categories.
- Google Data Portability: https://developers.google.com/data-portability/user-guide/overview — official authorization-based API for specified data classes, **research only** unless no-provider-API invariant formally revised.

## Adapter capability matrix (must be evidence-filled per provider)
For every concrete adapter, record: provider and product; user account/workspace scope; supported platform; artifact classes; discovery mode; official docs and verification date; export UI; file format; file/media coverage; delta ability; rate limits; consent UX; terms/legal review; authentication model; error handling; revocation behavior; idempotency key; last-tested fixture; owner and maturity (CANDIDATE / RESEARCHED / IMPLEMENTED / WIRED / TESTED / DEVICE-VERIFIED).

## Sequencing
Start with **local files + actual AI exports + local developer sessions**: high knowledge value, stable user-selected inputs, no new provider APIs. Then signed-in browser incremental conversations, Drive/mail/calendar via approved policy, followed by social/messaging. Sensitive finance/health/location sources live in opt-in protected scopes and are not silently included in generic DCBs.

## Research questions
Which cloud providers support full original file recovery vs transcript-only metadata? Which account tiers restrict export? Can provider export be repeated idempotently? How to preserve branching edits, deleted messages and attachments? What restrictions apply to browser DOM capture? How do mobile OS permissions limit ongoing observation? Document answers with primary sources and dated verification, not guesses.
