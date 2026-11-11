# ChatGPT T=0 acquisition race: speed, yield and availability
Research date: 2026-11-11. Status: DESIGN / VERIFIED PUBLIC DOCUMENTATION, not live-account automation acceptance.

## Anchor
T=0 is *verified signed-in ChatGPT account identity in the dedicated Host Chrome profile*, not installation start. Account authentication does not grant content ingestion, automation or external transmission. Host asks for distinct grants. Studio serializes browser effects even if four logical acquisition lanes run concurrently. Source-backed outcomes only.

## Source-backed facts (OpenAI documentation checked 2026-11-11)
- Export: Settings -> Data controls -> Export -> Confirm; eligible Free/Go/Plus/Pro and some Edu. Business/Enterprise/Healthcare do not offer self-service in this route. Export can take up to 7 days, notification email/SMS; settings-export link expires after 24h and should be opened while signed into requesting account. ZIP has index.html viewer, conversation JSON and other included data; older exports may have chat.html. Never promise exact export media coverage. https://help.openai.com/en/articles/7260999-exporting-your-chatgpt-history-and-data
- Only one pending export should be tracked; do not spam/re-request to accelerate. Same-account verification, notification monitoring strictly opt-in.
- Memory summary: Settings -> Personalization -> Memory; can be high-yield, but is not full account history, may omit details and may draw from chats, files, or connected sources. https://help.openai.com/en/articles/8590148-memory-in-chatgpt
- Custom Instructions: Settings -> Personalization -> Custom Instructions; included in official export; may contain valuable explicit owner preferences. https://help.openai.com/en/articles/8096356-chat-preference-for-chatgpt
- Sidebar search: finds title/content including older/archived chats; results and filters depend on account, and locked chats not in normal search. https://help.openai.com/en/articles/10056348-finding-your-chats-projects-and-files-in-chatgpt
- Projects: related chats, project-specific instructions, files/sources and memory boundaries; do not collapse all projects into account-global identity. https://help.openai.com/en/articles/10169521-projects-in-chatgpt
- Deleted chats cannot be recovered; temporary chats stay out of regular history unless saved. https://help.openai.com/en/articles/8983778-chat-and-file-retention-in-chatgpt

## Critical path from login (engineering targets, not guarantees)
T=0–15 s: Host verifies provider account identity and separate source-grant review; capture consent decision and browser action queue.
T=15–60 s: Ask to request official export *first*; only user-approved UI interaction confirms provider status. Immediately mark PENDING; don't repeatedly request.
T=30–120 s: with separate grants, capture visible custom instructions, memory summary and qualified project/instruction metadata where UI supports user-approved reading. Use source snapshots with capture timestamp; do not silently export to other AI.
T=1–4 min: exact approved F02/F01 bootstraps through Host scheduler, Studio owner with send-once and response receipt; do not assert they exhaust history.
T=2–6 min: read visible sidebar metadata via existing read-only ChatGPT index reader. Additional targeted search only via permissible user-directed UI workflows. Build ranked *discovery candidates*, not exhaustive counts.
T=3–10 min: merge confirmed direct preferences, user-reviewed memory candidates, selected visible conversation evidence and bootstrap-derived candidates; first minimal authorized Personal World and DCB possible, independently of export.
T=10+ min: selectively capture user-opened/high-priority threads with bounded per-thread consent and browser action receipts. No hidden private-endpoint HTTP requests, cookie reuse, undocumented bulk enumeration or rate-limit evasion.
Export arrival: provider-variable, up to 7 days; user downloads ZIP and selects local file. Host archive parser processes high-priority conversations and then complete permitted archive in checkpointed batches. Parser processing time is a function of bytes, schema, CPU, disk, extraction method, not a fixed promised duration.

## Acquisition lanes
A EXPORT EARLY: request once at first approval, track pending/ready/downloaded/expired/invalid, notify user of link expiry, optionally user-authorized notification integration; do not assume Gmail belongs to same identity or auto-open mail. Offer local folder watch of user-selected Download location if user permits. Hash and manifest once after local select.
B PROFILE SNAPSHOT: user-approved capture of current custom instructions, memory summary, optionally reviewed Characteristics and project instructions. Record source type and confidence: owner-authored instructions > generated memory synthesis. Memory may reflect connected apps and is not independent corroboration.
C METADATA DISCOVERY: existing Studio read-only index reader can show only currently rendered sidebar. Create prioritized candidates via user-directed search terms and selected project navigation only after validating supported UI. Never claim search index is an authorized programmatic archive API.
D APPROVED BOOTSTRAP: use existing exact prompt approval workflow, no prompt flooding. Propose domain-specific pass requests (identity/preferences, active projects, decisions, timelines) only when product's permission and effect quotas permit. Their answers remain model-generated hypotheses, not verified history.
E SELECTED THREADS: explicitly opened user-selected active threads; capture authorized content through existing Studio adapter if supported and proven; preserve native identifiers and source offsets.
F OFFLINE ARCHIVE: official ZIP is high coverage, highest long-tail yield and no browser throughput competition. Preserve JSON tree/branch/thread/message IDs and timestamps, attachments/omissions and account identity.

## Yield and coverage accounting
Never invent universal messages-per-second, number of recoverable chats, or median export ETA. Count four things separately:
- discovered_objects: visible/UI-derived candidate items (partial).
- captured_objects: content actually read with grant/receipt.
- imported_objects: locally parsed permitted export objects with manifest.
- verified_claims: graph assertions backed by evidence/owner acceptance; not equivalent to message count.
Per-lane ledger: source, account, grant, artifact/time range, discovered, captured, imported, skipped, duplicate, unreadable, contradictions, evidence-linked claims, bytes, duration, throughput measured, last changed, pending/unavailable/unknown.
Before ZIP arrival show 'partial browser-visible coverage / full history pending'; after ZIP compare provider IDs and evidence hashes; report archive coverage only relative to supplied archive, not theoretical all-time history.

## Optimization policy
P0 request export once; P0 parallel owner-visible profile material; P0 seed immediate DCB. Prioritize by expected knowledge gain per wall-clock and risk, not by raw number of browser navigations.
Use cheap deterministic structural extraction first, semantic candidate passes only on deduplicated/salient content, recent/active discussions priority, no excessive model prompts. Cap CPU/memory and disk; speed requires pressure-aware checkpoints and no replay of uncertain browser effects. Support pause.
Use differentiated confidence for explicit instruction, memory summary, model-generated F02/F01, captured message, verified local archive; prevent circular self-corroboration when bootstrap repeats memory-derived synthesis.
Never mass-scrape browser private APIs, alter Chrome protections, bypass access controls or provider rate limits. Product's no-provider-API invariant remains.
No claims of complete, live, or real-account tested ingestion without receipts.

## Milestone states
M0 login accepted and source permissions separate.
M1 export request submitted and provider confirmation captured.
M2 first explicit profile snapshot retained locally.
M3 approved bootstrap reconciled and first candidate profile reviewable.
M4 first task DCB built, explicit destination permission and send receipt.
M5 local official archive selected/validated.
M6 archive manifest, progressive exact-ID reconciliation, graph apply, source-scoped revisions.
M7 steady-state authorized live capture and source-wide revocation/deletion tests.

## Dependencies and unresolved verification
Existing Host F02/F01 flows and Studio read-only index are source-implemented per CHATGPT-FIRST-BOOTSTRAP-INVESTIGATION.md. No proof of supported automated UI reads for Memory or Custom Instructions; treat them as user-guided/manual until verified. Export request UI automation must be verified separately. No live performance benchmarking, provider account behavior, or complete archive import tested. Need account-specific consent and a tested UI matrix before implementing any browser navigation.
